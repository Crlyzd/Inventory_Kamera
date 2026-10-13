//! Cross-thread communication event channels.
#![allow(dead_code)]

use crate::core::logger::LogLevel;
use crate::core::state::ScanProgress;
use crossbeam_channel::{unbounded, Receiver, Sender};

/// Commands sent from the UI to the background scanner engine.
#[derive(Debug, Clone)]
pub enum ScanCommand {
    Start,
    Stop,
    Pause,
    Resume,
}

/// Events sent from the scanner engine back to the UI.
#[derive(Debug, Clone)]
pub enum ScanEvent {
    WindowDetected { detected: bool, name: String },
    Progress(ScanProgress),
    Log { level: LogLevel, text: String },
    Completed,
    Error(String),
}

/// Bidirectional message bus between UI and background scanner.
pub struct EventBus {
    pub cmd_sender: Sender<ScanCommand>,
    pub cmd_receiver: Receiver<ScanCommand>,
    pub evt_sender: Sender<ScanEvent>,
    pub evt_receiver: Receiver<ScanEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let (cmd_sender, cmd_receiver) = unbounded();
        let (evt_sender, evt_receiver) = unbounded();
        Self {
            cmd_sender,
            cmd_receiver,
            evt_sender,
            evt_receiver,
        }
    }

    /// Dispatches a scan command to the worker thread.
    pub fn send_command(&self, cmd: ScanCommand) {
        let _ = self.cmd_sender.send(cmd);
    }

    /// Dispatches an event to the UI thread.
    pub fn send_event(&self, evt: ScanEvent) {
        let _ = self.evt_sender.send(evt);
    }
}
