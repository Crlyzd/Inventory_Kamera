//! Modular UI views.

pub mod action_dock;
pub mod chip_bar;
pub mod header_bar;
pub mod log_window;
pub mod ocr_stage;
pub mod settings;

pub use action_dock::render_action_dock;
pub use chip_bar::render_chip_bar;
pub use header_bar::render_header_bar;
pub use log_window::render_log_window;
pub use ocr_stage::render_ocr_stage;
pub use settings::render_settings_window;
