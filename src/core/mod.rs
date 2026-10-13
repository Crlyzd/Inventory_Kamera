//! Core domain state, configuration, logging, and updater logic.

pub mod config;
pub mod events;
pub mod logger;
pub mod self_updater;
pub mod state;

#[allow(unused_imports)]
pub use config::AppConfig;
#[allow(unused_imports)]
pub use events::{EventBus, ScanCommand, ScanEvent};
#[allow(unused_imports)]
pub use logger::{LogEntry, LogLevel, LogStore};
#[allow(unused_imports)]
pub use self_updater::{apply_update, check_for_updates, UpdateStatus};
pub use state::{AppState, SettingsTab};
