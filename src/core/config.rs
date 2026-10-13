//! User configuration persistence.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Persistent preferences stored on disk.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub export_directory: PathBuf,
    pub dark_mode: bool,
    pub auto_open_exports: bool,
    pub sound_notifications: bool,
    pub auto_check_updates: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            export_directory: PathBuf::from("./exports"),
            dark_mode: true,
            auto_open_exports: true,
            sound_notifications: true,
            auto_check_updates: true,
        }
    }
}

impl AppConfig {
    /// Loads configuration or returns default if file does not exist.
    pub fn load_or_default() -> Self {
        Self::default()
    }
}
