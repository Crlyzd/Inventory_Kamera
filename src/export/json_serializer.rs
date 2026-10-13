//! GOOD format JSON disk serializer.
#![allow(dead_code)]

use crate::export::good_format::GoodDatabase;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Serializes and saves a GOOD database to the designated export folder.
pub fn export_good_database(database: &GoodDatabase, export_dir: impl AsRef<Path>) -> Result<PathBuf, String> {
    let dir = export_dir.as_ref();
    fs::create_dir_all(dir).map_err(|e| format!("Failed to create exports folder: {e}"))?;

    let filename = format!("genshin_good_export_{}.json", current_timestamp());
    let target_path = dir.join(filename);

    let json_bytes = serde_json::to_string_pretty(database).map_err(|e| format!("Failed to format JSON: {e}"))?;
    fs::write(&target_path, json_bytes).map_err(|e| format!("Failed to write export file: {e}"))?;

    Ok(target_path)
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
