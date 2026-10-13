//! Shared application state.
#![allow(dead_code)]

/// Active tab within the dedicated settings window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    General,
    Capture,
    About,
}

/// Capture engine and timing configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureSettings {
    pub use_wgc: bool,
    pub enforce_aspect_ratio: bool,
    pub traversal_delay_ms: u32,
    pub scroll_settle_ms: u32,
    pub ocr_threads: usize,
    pub min_confidence: u8,
}

impl Default for CaptureSettings {
    fn default() -> Self {
        Self {
            use_wgc: true,
            enforce_aspect_ratio: true,
            traversal_delay_ms: 50,
            scroll_settle_ms: 120,
            ocr_threads: 4,
            min_confidence: 85,
        }
    }
}

/// Selected category targets for scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanTargets {
    pub weapons: bool,
    pub artifacts: bool,
    pub characters: bool,
    pub materials: bool,
    pub achievements: bool,
}

impl Default for ScanTargets {
    fn default() -> Self {
        Self {
            weapons: true,
            artifacts: true,
            characters: true,
            materials: false,
            achievements: false,
        }
    }
}

/// Filter thresholds for scanning items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScanFilters {
    pub min_weapon_rarity: u8,
    pub min_artifact_rarity: u8,
    pub min_artifact_level: u8,
}

impl Default for ScanFilters {
    fn default() -> Self {
        Self {
            min_weapon_rarity: 4,
            min_artifact_rarity: 5,
            min_artifact_level: 16,
        }
    }
}

/// Live scanning progress information.
#[derive(Debug, Clone, Default)]
pub struct ScanProgress {
    pub is_scanning: bool,
    pub current_category: String,
    pub current_item_name: String,
    pub items_scanned: usize,
    pub total_estimated: usize,
    pub progress_ratio: f32,
    pub confidence: f32,
}

/// Information about the detected Genshin game window.
#[derive(Debug, Clone)]
pub struct GameWindowInfo {
    pub detected: bool,
    pub process_name: String,
    pub dimensions: (u32, u32),
}

impl Default for GameWindowInfo {
    fn default() -> Self {
        Self {
            detected: false,
            process_name: "GenshinImpact.exe".to_string(),
            dimensions: (1920, 1080),
        }
    }
}

/// Master application state container.
#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub targets: ScanTargets,
    pub filters: ScanFilters,
    pub capture: CaptureSettings,
    pub progress: ScanProgress,
    pub game_window: GameWindowInfo,
    pub is_dark_mode: bool,
    pub show_settings: bool,
    pub show_logs: bool,
    pub active_settings_tab: SettingsTab,
    pub export_path_input: String,
}

impl AppState {
    /// Creates a newly initialized state with dark mode active.
    pub fn new() -> Self {
        Self {
            is_dark_mode: true,
            export_path_input: "./exports".to_string(),
            ..Default::default()
        }
    }

    /// Toggles color theme between Dark and Light.
    pub fn toggle_theme(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
    }
}
