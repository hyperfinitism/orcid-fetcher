use anyhow::{Context, Result, bail};
use reqwest::{
    Client,
    header::{ACCEPT, HeaderValue},
};
use serde::de::DeserializeOwned;
use tracing::{Instrument, error, instrument};

use orcid_works_model::{OrcidWorkDetail, OrcidWorks};

const BASE: &str = "https://pub.orcid.org/v3.0";
const JSON_ACCEPT: &str = "application/json";

// Build HTTP Client
pub(crate) fn build_client(ua: &str) -> Result<Client> {
    let client = Client::builder().user_agent(ua).build()?;
    Ok(client)
}

// Get JSON from URL
#[instrument(name = "get_json", skip_all)]
async fn get_json<T>(client: &Client, url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let res = client
        .get(url)
        .header(ACCEPT, HeaderValue::from_static(JSON_ACCEPT))
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;

    if res.error_for_status_ref().is_err() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();

        error!(%status, %url, response_body = %body, "HTTP error");
        bail!("HTTP {status} while GET {url}: {body}");
    }

    match res.json::<T>().await {
        Ok(parsed) => Ok(parsed),

        Err(e) => {
            if e.is_decode() {
                error!(%url, err = %e, "JSON parse failure");
            } else {
                error!(%url, err = %e, "response body read failure");
            }
            Err(e).with_context(|| format!("parse JSON from {url}"))
        }
    }
}

// GET /{id}/works
#[instrument(name = "fetch_works", skip_all)]
pub async fn fetch_works(client: &reqwest::Client, id: &str) -> Result<OrcidWorks> {
    let url = format!("{BASE}/{id}/works");
    get_json::<OrcidWorks>(client, &url)
        .in_current_span()
        .await
        .with_context(|| format!("fetch work summaries for ORCID iD {id}"))
}

// GET /{id}/work/{putcode}
#[instrument(name = "fetch_work_detail", skip_all)]
pub async fn fetch_work_detail(
    client: &reqwest::Client,
    id: &str,
    putcode: u64,
) -> Result<OrcidWorkDetail> {
    let url = format!("{BASE}/{id}/work/{putcode}");

    get_json::<OrcidWorkDetail>(client, &url)
        .in_current_span()
        .await
        .with_context(|| format!("fetch work detail of putcode {putcode}"))
}
