use anyhow::{Context, Result};
use serde_path_to_error::deserialize;
use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Write},
    path::Path,
};

use tempfile::NamedTempFile;
use tracing::{error, info, instrument, warn};

use orcid_works_model::OrcidWorkDetailFile;

// Read the existing JSON file; use the empty list if absent.
#[instrument(name = "read_work_details_json", skip_all)]
pub(crate) fn read_work_details_json<P: AsRef<Path>>(path: P) -> Result<OrcidWorkDetailFile> {
    let path = path.as_ref();

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut de = serde_json::Deserializer::from_reader(reader);

            let data: OrcidWorkDetailFile = deserialize(&mut de).map_err(|e| {
                error!(
                    path = path.display().to_string(),
                    err = %e,
                    "JSON parse failure"
                );
                e
            })?;

            Ok(data)
        }

        Err(e) if e.kind() == ErrorKind::NotFound => {
            info!(
                path = path.display().to_string(),
                "file not found; use empty JSON"
            );
            Ok(OrcidWorkDetailFile { records: vec![] })
        }

        Err(e) => {
            error!(path= path.display().to_string(), err = %e, "failed to open work-detail file");
            Err(e).with_context(|| format!("open {}", path.display()))
        }
    }
}

// Write JSON file
#[instrument(name = "write_pretty_json", skip_all)]
pub(crate) fn write_pretty_json<P: AsRef<Path>>(
    path: P,
    value: &OrcidWorkDetailFile,
) -> Result<()> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or(Path::new("."));

    let mut tmp = match NamedTempFile::new_in(parent)
        .with_context(|| format!("create temp file for {}", path.display()))
    {
        Ok(f) => f,
        Err(e) => {
            error!(path = path.display().to_string(), err = %e, "failed to create temp file");
            return Err(e);
        }
    };

    if let Err(e) = serde_json::to_writer_pretty(BufWriter::new(&mut tmp), value)
        .with_context(|| format!("serialize JSON into {}", path.display()))
    {
        error!(path = path.display().to_string(), err = %e, "JSON serialization failure");
        return Err(e);
    }

    if let Err(e) = tmp.as_file_mut().flush() {
        error!(path = path.display().to_string(), err = %e, "flush failure");
        return Err(e).context("flush tmp file");
    }
    if let Err(e) = tmp.as_file_mut().sync_all() {
        error!(path = path.display().to_string(), err = %e, "fsync failure");
        return Err(e).context("fsync tmp file");
    }

    if let Err(e) = tmp
        .persist(path)
        .map_err(|e| e.error)
        .with_context(|| format!("rename temp file into {}", path.display()))
    {
        error!(path = path.display().to_string(), err = %e, "atomic rename failure");
        return Err(e);
    }

    if let Ok(dir) = parent.to_owned().canonicalize() {
        if let Ok(dir_fd) = File::open(&dir) {
            let _ = dir_fd.sync_all();
        }
    }

    Ok(())
}
