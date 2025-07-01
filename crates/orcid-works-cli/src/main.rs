use anyhow::{Context, Result};
use clap::Parser;
use futures::stream::{self, StreamExt};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::InMemoryState, state::NotKeyed};
use orcid_works_model::{OrcidWorkDetail, OrcidWorkDetailFile, OrcidWorkSummary};
use reqwest::Client;
use std::fs;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::sync::Arc;

mod api;
mod compare;
use api::{fetch_work_detail, fetch_works};
use compare::detail_changed;

// Environment Constants
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = env!("CARGO_REPOSITORY_URL");

// Parallel fetch with rate limit
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

async fn guarded_fetch<T, Fut>(limiter: &Option<Arc<Limiter>>, fut: Fut) -> Result<T, anyhow::Error>
where
    Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
{
    if let Some(l) = limiter {
        l.until_ready().await;
    }
    fut.await
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
    #[arg(short = 'i', long, help = "ORCID iD (xxxx-xxxx-xxxx-xxxx)")]
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
        default_value_t = 10,
        help = "Maximum parallel requests"
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
}

fn build_user_agent(note: Option<String>) -> String {
    let base = format!("{APP_NAME}/{APP_VERSION} (+{REPO_URL})");
    match note {
        Some(note) if !note.trim().is_empty() => format!("{base}; {note}"),
        _ => base,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // HTTP client
    let ua = build_user_agent(cli.user_agent_note);
    let client = Arc::new(Client::builder().user_agent(&ua).build()?);
    let id: Arc<str> = cli.id.clone().into();
    println!("[{APP_NAME}] {ua}");
    // Rate limit
    let limiter: Option<Arc<Limiter>> = Some(Arc::new(Limiter::direct(Quota::per_second(
        NonZeroU32::new(cli.rate_limit).unwrap(),
    ))));
    // Fetch works summary list
    println!(
        "[{}] fetch work summaries of ORCID iD {}",
        APP_NAME, &cli.id
    );
    let works = fetch_works(&client, &cli.id)
        .await
        .with_context(|| format!("fetch work summaries of {}", cli.id))?;

    // Flatten summaries
    let summaries: Vec<OrcidWorkSummary> = works
        .group
        .iter()
        .flat_map(|g| g.work_summary.clone())
        .collect();

    let putcodes: Vec<u64> = summaries.iter().map(|s| s.put_code).collect();

    // 3) Parallel fetch work details
    println!("[{}] fetch work details of ORCID iD {}", APP_NAME, cli.id);
    let details_res = stream::iter(putcodes)
        .map(|pc| {
            let limiter = limiter.clone();
            let client = client.clone();
            let id = id.clone();
            async move {
                let fut = fetch_work_detail(&client, &id, pc);
                guarded_fetch(&limiter, fut).await
            }
        })
        .buffer_unordered(cli.concurrency)
        .collect::<Vec<_>>()
        .await;

    let mut details: Vec<OrcidWorkDetail> = Vec::new();
    for res in details_res {
        details.push(res?);
    }
    details.sort_by_key(|d| d.summary.put_code);

    // 4) Write JSON ---------------------------------------------------------
    let detail_file = OrcidWorkDetailFile { records: details };
    write_if_changed(&cli.out, &detail_file, |p, v| detail_changed(p, v))?;

    println!(
        "[{}] finished: {} entries → {}",
        APP_NAME,
        detail_file.records.len(),
        cli.out.display()
    );
    Ok(())
}

fn write_if_changed<P, T, F>(path: P, value: &T, is_changed: F) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq,
    F: Fn(&Path, &T) -> anyhow::Result<bool>,
{
    let path = path.as_ref();
    if is_changed(path, value)? {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(value)?;
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, path)?;
        println!("[{}] wrote {}", APP_NAME, path.display());
    } else {
        println!("[{}] no changes in {}", APP_NAME, path.display());
    }
    Ok(())
}
