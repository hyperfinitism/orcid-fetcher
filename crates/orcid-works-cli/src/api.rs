//! Thin async wrapper around ORCID public API v3.0.
//!
//! Only the **read‑public** endpoints are used, so no authentication token is
//! required.  Each JSON response is deserialised into types from
//! `orcid_model`.
//!
//! Public functions:
//! ```
//! fetch_works(&client, orcid)      -> OrcidWorks
//! fetch_work_detail(&client, orcid, putcode) -> OrcidWorkDetail
//! ```
//!
//! A single `reqwest::Client` instance should be reused so underlying TLS
//! connections

use anyhow::Context;
use serde_path_to_error::Segment;
use serde_path_to_error::deserialize;

use orcid_works_model::{OrcidWorkDetail, OrcidWorks};

const BASE: &str = "https://pub.orcid.org/v3.0";
const JSON_ACCEPT: &str = "application/json";

/// GET /v3.0/{orcid}/works
pub async fn fetch_works(client: &reqwest::Client, orcid: &str) -> anyhow::Result<OrcidWorks> {
    let url = format!("{BASE}/{orcid}/works");
    let res = client
        .get(&url)
        .header(reqwest::header::ACCEPT, JSON_ACCEPT)
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;
    let status = res.status();
    if !status.is_success() {
        anyhow::bail!("ORCID /works returned HTTP {status}");
    }
    let bytes = res.bytes().await?;
    // for debugging purpose
    let mut de = serde_json::Deserializer::from_slice(&bytes);
    let works: OrcidWorks = match deserialize(&mut de) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "[orcid-fetch] /works parse error at {}: {}",
                e.path().iter().map(Segment::to_string).collect::<String>(),
                e
            );
            anyhow::bail!(e);
        }
    };
    Ok(works)
}

/// GET /v3.0/{orcid}/work/{putcode}
pub async fn fetch_work_detail(
    client: &reqwest::Client,
    orcid: &str,
    putcode: u64,
) -> anyhow::Result<OrcidWorkDetail> {
    let url = format!("{BASE}/{orcid}/work/{putcode}");
    let res = client
        .get(&url)
        .header(reqwest::header::ACCEPT, JSON_ACCEPT)
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;
    let status = res.status();
    if !status.is_success() {
        anyhow::bail!("ORCID /work/{putcode} returned HTTP {status}");
    }
    let bytes = res.bytes().await?;
    // for debugging purpose
    let mut de = serde_json::Deserializer::from_slice(&bytes);
    let detail: OrcidWorkDetail = match deserialize(&mut de) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "[orcid-fetch] /work/{putcode} parse error at {}: {}",
                e.path().iter().map(Segment::to_string).collect::<String>(),
                e
            );
            anyhow::bail!(e);
        }
    };
    Ok(detail)
}
