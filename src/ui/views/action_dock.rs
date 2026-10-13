//! Bottom floating action dock containing scan trigger controls.

use crate::core::AppState;
use crate::ui::components::flat_button;
use egui::Ui;

/// Renders the bottom action dock with primary start/stop buttons.
pub fn render_action_dock(ui: &mut Ui, state: &mut AppState) {
    let dark = state.is_dark_mode;

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        if flat_button(ui, "📁 Open Exports", false, dark).clicked() {
            let export_dir = std::path::Path::new("./exports");
            let _ = std::fs::create_dir_all(export_dir);
            let _ = open::that(export_dir);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(12.0);

            if state.progress.is_scanning {
                if flat_button(ui, "⏹ Stop Scan (Esc)", false, dark).clicked() {
                    state.progress.is_scanning = false;
                }
            } else if flat_button(ui, "▶ START SCAN", true, dark).clicked() {
                state.progress.is_scanning = true;
                state.progress.current_category = "Weapons".to_string();
                state.progress.current_item_name = "Scanning...".to_string();
                state.progress.progress_ratio = 0.05;
            }
        });
    });
    ui.add_space(8.0);
}
