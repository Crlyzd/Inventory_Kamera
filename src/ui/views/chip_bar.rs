//! Target categories chip bar and scan filters.

use crate::core::AppState;
use crate::ui::components::category_chip;
use crate::ui::theme::Palette;
use egui::{RichText, Ui};

/// Renders the category target toggle chips and filter selectors.
pub fn render_chip_bar(ui: &mut Ui, state: &mut AppState) {
    let dark = state.is_dark_mode;
    let palette = if dark { Palette::DARK } else { Palette::LIGHT };

    ui.add_space(8.0);
    render_categories(ui, state, dark);
    ui.add_space(4.0);
    render_filter_chips(ui, state, &palette);
    ui.add_space(8.0);
}

fn render_categories(ui: &mut Ui, state: &mut AppState, dark: bool) {
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        category_chip(ui, "⚔ Weapons", &mut state.targets.weapons, dark);
        category_chip(ui, "👑 Artifacts", &mut state.targets.artifacts, dark);
        category_chip(ui, "👤 Characters", &mut state.targets.characters, dark);
        category_chip(ui, "🎒 Materials", &mut state.targets.materials, dark);
        category_chip(ui, "🏆 Achievements", &mut state.targets.achievements, dark);
    });
}

fn render_filter_chips(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.horizontal(|ui| {
        ui.add_space(14.0);
        ui.label(RichText::new("Filters:").small().color(palette.text_muted));

        ui.label(RichText::new("Min Weapon:").small().color(palette.text_muted));
        egui::ComboBox::from_id_salt("min_weapon_combo")
            .selected_text(format!("{}★", state.filters.min_weapon_rarity))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.filters.min_weapon_rarity, 3, "3★");
                ui.selectable_value(&mut state.filters.min_weapon_rarity, 4, "4★");
                ui.selectable_value(&mut state.filters.min_weapon_rarity, 5, "5★");
            });

        ui.label(RichText::new("Min Artifact:").small().color(palette.text_muted));
        egui::ComboBox::from_id_salt("min_artifact_combo")
            .selected_text(format!("{}★", state.filters.min_artifact_rarity))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.filters.min_artifact_rarity, 4, "4★");
                ui.selectable_value(&mut state.filters.min_artifact_rarity, 5, "5★");
            });

        ui.label(RichText::new("Min Level:").small().color(palette.text_muted));
        egui::ComboBox::from_id_salt("min_level_combo")
            .selected_text(format!("+{}", state.filters.min_artifact_level))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.filters.min_artifact_level, 0, "+0");
                ui.selectable_value(&mut state.filters.min_artifact_level, 12, "+12");
                ui.selectable_value(&mut state.filters.min_artifact_level, 16, "+16");
                ui.selectable_value(&mut state.filters.min_artifact_level, 20, "+20");
            });
    });
}
