//! In-memory structured diagnostic logger for the application.
#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Log severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Scan,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
            Self::Scan => "SCAN",
        }
    }
}

/// A single structured log record.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

/// Thread-safe ring buffer log store.
#[derive(Debug, Clone, Default)]
pub struct LogStore {
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::with_capacity(500))),
        }
    }

    /// Appends a new log message.
    pub fn log(&self, level: LogLevel, message: impl Into<String>) {
        if let Ok(mut lock) = self.entries.lock() {
            if lock.len() >= 1000 {
                lock.remove(0);
            }
            lock.push(LogEntry {
                timestamp: current_timestamp(),
                level,
                message: message.into(),
            });
        }
    }

    pub fn info(&self, msg: impl Into<String>) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&self, msg: impl Into<String>) {
        self.log(LogLevel::Warning, msg);
    }

    pub fn error(&self, msg: impl Into<String>) {
        self.log(LogLevel::Error, msg);
    }

    pub fn scan(&self, msg: impl Into<String>) {
        self.log(LogLevel::Scan, msg);
    }

    /// Retrieves all current log records.
    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().map(|l| l.clone()).unwrap_or_default()
    }

    /// Clears all stored records.
    pub fn clear(&self) {
        if let Ok(mut lock) = self.entries.lock() {
            lock.clear();
        }
    }
}

fn current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let hours = (now / 3600) % 24;
    let mins = (now / 60) % 60;
    let secs = now % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}
