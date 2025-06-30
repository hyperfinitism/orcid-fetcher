//! Diff / equality helpers for ORCID JSON artefacts.

use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;

use orcid_works_model::OrcidWorkDetailFile;

/// Load a JSON file if it exists; otherwise return `None`.
fn load_optional<P, T>(path: P) -> Result<Option<T>>
where
    P: AsRef<Path>,
    for<'de> T: serde::Deserialize<'de>,
{
    match File::open(&path) {
        Ok(f) => Ok(Some(serde_json::from_reader(f)?)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).with_context(|| format!("open {:?}", path.as_ref().display())),
    }
}

/* -------------------------------------------------------------------------
 * WorkDetail diff
 * --------------------------------------------------------------------- */

/// Compare old vs new WorkDetailFile, ignoring ordering.
/// Returns `true` if contents differ.
pub fn detail_changed<P: AsRef<Path>>(path: P, newest: &OrcidWorkDetailFile) -> Result<bool> {
    let older = load_optional::<_, OrcidWorkDetailFile>(&path)?;
    Ok(older.map(|v| v.into_map()) != Some(newest.clone().into_map()))
}
