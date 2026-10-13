//! General settings tab content.

use crate::core::AppState;
use crate::ui::theme::Palette;
use egui::{RichText, Ui};

/// Renders the General preferences tab.
pub fn render_tab_general(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.add_space(8.0);
    render_storage_section(ui, state, palette);
    ui.add_space(14.0);
    render_hotkeys_section(ui, palette);
    ui.add_space(14.0);
    render_notifications_section(ui, palette);
}

fn render_storage_section(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.label(RichText::new("STORAGE & EXPORTS").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Export Directory:").small().color(palette.text_primary));
        ui.text_edit_singleline(&mut state.export_path_input);
        if ui.button("Browse...").clicked() {
            let export_dir = std::path::Path::new(&state.export_path_input);
            let _ = std::fs::create_dir_all(export_dir);
            let _ = open::that(export_dir);
        }
    });
}

fn render_hotkeys_section(ui: &mut Ui, palette: &Palette) {
    ui.label(RichText::new("HOTKEYS & CONTROLS").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Emergency Stop Key:").small().color(palette.text_primary));
        ui.label(RichText::new("Escape").strong().color(palette.accent));
        ui.add_space(16.0);
        ui.label(RichText::new("Inventory Key:").small().color(palette.text_primary));
        ui.label(RichText::new("B").strong().color(palette.accent));
    });
}

fn render_notifications_section(ui: &mut Ui, palette: &Palette) {
    ui.label(RichText::new("NOTIFICATIONS & DATA").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.label(RichText::new("Game Data Version: v7.1.0 (AnimeGameData2)").small().color(palette.text_primary));
    ui.label(RichText::new("● Automatic data verification active").small().color(palette.success));
}
