//! Application entrypoint for Inventory Kamera.

mod app;
mod automation;
mod core;
mod data;
mod export;
mod ocr;
mod scraper;
mod ui;

use app::InventoryKameraApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    cleanup_previous_update();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Inventory Kamera")
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_inner_size([800.0, 600.0])
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Inventory Kamera",
        native_options,
        Box::new(|cc| Ok(Box::new(InventoryKameraApp::new(cc)))),
    )
}

/// Removes any leftover .old binary from a prior in-place self-update.
fn cleanup_previous_update() {
    if let Ok(exe_path) = std::env::current_exe() {
        let old_file = exe_path.with_extension("old");
        if old_file.exists() {
            let _ = std::fs::remove_file(old_file);
        }
    }
}
