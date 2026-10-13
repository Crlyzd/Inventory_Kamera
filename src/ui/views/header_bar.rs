//! Frameless custom header bar with integrated window controls.

use crate::core::AppState;
use crate::ui::components::titlebar_button;
use crate::ui::theme::Palette;
use egui::{CornerRadius, RichText, Sense, Ui, UiBuilder, Vec2, ViewportCommand};

/// Renders the integrated custom header bar.
pub fn render_header_bar(ui: &mut Ui, state: &mut AppState) {
    let height = 36.0;
    let palette = if state.is_dark_mode {
        Palette::DARK
    } else {
        Palette::LIGHT
    };

    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), height),
        Sense::click_and_drag(),
    );

    if response.drag_started() || response.dragged() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    if ui.is_rect_visible(rect) {
        ui.painter().rect_filled(rect, CornerRadius::ZERO, palette.bg_elevated);
    }

    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(10.0);
            render_branding(ui, state, &palette);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                render_window_controls(ui);
                ui.add_space(8.0);
                render_tool_icons(ui, state, &palette);
            });
        });
    });
}

fn render_branding(ui: &mut Ui, state: &AppState, palette: &Palette) {
    ui.label(RichText::new("📷 Inventory Kamera").strong());
    ui.label(RichText::new("v2.0.0").small().color(palette.text_muted));
    ui.add_space(8.0);

    let (status_text, status_color) = if state.game_window.detected {
        if state.game_window.is_minimized {
            (format!("● {} (Minimized)", state.game_window.process_name), palette.warning)
        } else {
            (format!("● {}", state.game_window.process_name), palette.success)
        }
    } else {
        ("○ Waiting for game".to_string(), palette.text_muted)
    };

    ui.label(RichText::new(status_text).small().color(status_color));
}

fn render_tool_icons(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    if ui.button(RichText::new("⚙").color(palette.text_primary)).clicked() {
        state.show_settings = !state.show_settings;
    }

    let theme_glyph = if state.is_dark_mode { "☀" } else { "🌙" };
    if ui.button(RichText::new(theme_glyph).color(palette.text_primary)).clicked() {
        state.toggle_theme();
    }

    if ui.button(RichText::new("📁").color(palette.text_primary)).clicked() {
        let export_dir = std::path::Path::new("./exports");
        let _ = std::fs::create_dir_all(export_dir);
        let _ = open::that(export_dir);
    }

    if ui.button(RichText::new("📄").color(palette.text_primary)).clicked() {
        state.show_logs = !state.show_logs;
    }
}

fn render_window_controls(ui: &mut Ui) {
    if titlebar_button(ui, "✕", true).clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
    }
    if titlebar_button(ui, "—", false).clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
