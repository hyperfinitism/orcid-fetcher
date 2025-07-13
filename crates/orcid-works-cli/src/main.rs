use anyhow::{Context, Result};
use clap::Parser;
use futures::stream::{self, StreamExt, TryStreamExt};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::InMemoryState, state::NotKeyed};
use std::{collections::HashMap, num::NonZeroU32, path::PathBuf, sync::Arc};

use tracing::{Instrument, info, info_span, warn};

use orcid_works_model::{OrcidWorkDetail, OrcidWorkDetailFile, OrcidWorks};

mod api;
mod compare;
mod io;
use api::{build_client, fetch_work_detail, fetch_works};
use compare::{added_putcodes, deleted_putcodes, diff_putcodes, kept_putcodes, updated_putcodes};
use io::{read_work_details_json, write_pretty_json};

// Environment Constants
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = env!("CARGO_REPOSITORY_URL");

// Parallel fetch with rate limit
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

async fn guarded_fetch<T, F>(l: &Limiter, fut: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    l.until_ready().await;
    fut.await
}

// Build User-Agent string
fn build_user_agent(note: Option<String>) -> String {
    let base = format!("{APP_NAME}/{APP_VERSION} (+{REPO_URL})");
    match note {
        Some(note) if !note.trim().is_empty() => format!("{base}; {note}"),
        _ => base,
    }
}

// usize parser
fn ranged_usize_parser<const MIN: usize, const MAX: usize>(s: &str) -> Result<usize, String> {
    let n: usize = s.parse().map_err(|_| "not an integer".to_string())?;
    if (MIN..=MAX).contains(&n) {
        Ok(n)
    } else {
        Err(format!("{n} is not in {MIN}..={MAX}"))
    }
}

// Flags
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Fetch all WorkDetails for a given ORCID iD (ORCID API v3.0)",
    bin_name = "orcid-works-cli",
    after_help = "Disclaimer: This is a third-party tool and not endorsed by ORCID."
)]
struct Cli {
    #[arg(short = 'i', long, help = "ORCID iD (xxxx-xxxx-xxxx-xxxx).")]
    id: String,

    #[arg(
        short = 'o',
        long,
        default_value = "./output.json",
        help = "Output path to the JSON file; Parent dirs are created if absent."
    )]
    out: PathBuf,

    #[arg(
        long = "concurrency",
        default_value_t = 8,
        value_parser = ranged_usize_parser::<1, 32>,
        help = "Maximum parallel requests (1–32). Should not exceed rate-limit."
    )]
    concurrency: usize,

    #[arg(
        long = "rate-limit",
        default_value_t = 12,
        value_parser = clap::value_parser!(u32).range(1..=40),
        help = "Requests-per-second cap (1–40). See README References for details."
    )]
    rate_limit: u32,

    #[arg(
        long = "user-agent-note",
        help = "Extra text appended to the built-in User-Agent string [default: None]"
    )]
    user_agent_note: Option<String>,

    #[arg(
        long = "force-fetch",
        default_value_t = false,
        help = "Ignore diff and refetch every work-detail entry"
    )]
    force_fetch: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    if let Err(err) = run().await {
        tracing::error!(err = %err, "fatal error; exiting:");
        std::process::exit(1);
    }

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Concurrency & Rate Limit check
    if cli.concurrency > cli.rate_limit.try_into().unwrap() {
        warn!(
            concurrency = &cli.concurrency,
            rate_limit = &cli.rate_limit,
            "concurrency exceeds rate-limit"
        );
    }

    // HTTP client
    let ua = build_user_agent(cli.user_agent_note);
    let client = build_client(&ua)?;
    let id: Arc<str> = cli.id.clone().into();

    // Rate limit
    let limiter: Arc<Limiter> = Arc::new(Limiter::direct(Quota::per_second(
        NonZeroU32::new(cli.rate_limit).unwrap(),
    )));

    // Open the existing work details JSON
    info!(
        path = &cli.out.display().to_string(),
        "opening the existing work-details JSON"
    );
    let existing: OrcidWorkDetailFile = read_work_details_json(&cli.out).with_context(|| {
        format!(
            "open the existing work-details JSON from {}",
            &cli.out.display()
        )
    })?;
    let existing_map: HashMap<u64, OrcidWorkDetail> = existing
        .records
        .into_iter()
        .map(|d| (d.summary.put_code, d))
        .collect();

    // Fetch works summaries
    info!(id = &cli.id, "fetching work summaries");
    let works: OrcidWorks = fetch_works(&client, &cli.id)
        .in_current_span()
        .await
        .with_context(|| "update cache: fetch work summaries")?;

    // Detect changes
    let diff_map = diff_putcodes(&existing_map, &works, cli.force_fetch);
    let added = added_putcodes(&diff_map);
    let updated = updated_putcodes(&diff_map);
    let kept = kept_putcodes(&diff_map);
    let deleted = deleted_putcodes(&diff_map);

    info!(
        added = added.len(),
        updated = updated.len(),
        deleted = deleted.len(),
        "diff stats"
    );

    // Exit if no changes detected
    let to_fetch: Vec<u64> = added.into_iter().chain(updated.into_iter()).collect();

    if to_fetch.len() + deleted.len() == 0 {
        info!(
            to_fetch = to_fetch.len(),
            deleted = deleted.len(),
            "no changes detected - skip fetch & rewrite"
        );
        return Ok(());
    }

    // Parallel fetch work details
    info!(id = &cli.id, "fetching work details");
    let batch_span = info_span!("fetch_work_details_batch",
                            id    = %*id,
                            total = to_fetch.len());
    let fetched: Vec<OrcidWorkDetail> = stream::iter(to_fetch)
        .map(|pc| {
            let task_span = info_span!("work_detail_task", %pc);
            let limiter = limiter.clone();
            let client = client.clone();
            let id = id.clone();

            async move {
                guarded_fetch(
                    &limiter,
                    fetch_work_detail(&client, &id, pc).in_current_span(),
                )
                .await
            }
            .instrument(task_span)
        })
        .buffer_unordered(cli.concurrency)
        .try_collect::<Vec<_>>()
        .instrument(batch_span)
        .await
        .with_context(|| format!("batch fetch for ORCID iD {id}"))?;

    // Merge
    let mut merged: Vec<OrcidWorkDetail> = kept
        .into_iter()
        .filter_map(|pc| existing_map.get(&pc).cloned())
        .chain(fetched.into_iter())
        .collect();

    merged.sort_by_key(|d| d.summary.put_code);

    // Write JSON
    info!(
        path = cli.out.display().to_string(),
        "writing work-details JSON"
    );
    let out_json = OrcidWorkDetailFile { records: merged };
    write_pretty_json(&cli.out, &out_json)
        .with_context(|| format!("write work-details JSON to {}", cli.out.display()))?;

    info!("finished successfully");

    Ok(())
}
