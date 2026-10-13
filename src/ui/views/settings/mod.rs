//! Settings window views and tabs.

pub mod tab_about;
pub mod tab_capture;
pub mod tab_general;
pub mod window;

pub use tab_about::render_tab_about;
pub use tab_capture::render_tab_capture;
pub use tab_general::render_tab_general;
pub use window::render_settings_window;
