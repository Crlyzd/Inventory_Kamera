//! Central full-bleed OCR viewport stage.

use crate::core::AppState;
use crate::ui::theme::Palette;
use egui::{Align2, CornerRadius, FontId, Rect, RichText, Ui, UiBuilder, Vec2};

/// Renders the central OCR viewport, dynamically adapting to idle vs scanning states.
pub fn render_ocr_stage(ui: &mut Ui, state: &AppState) {
    let avail = ui.available_size();
    let (rect, _) = ui.allocate_exact_size(avail, egui::Sense::hover());
    let palette = if state.is_dark_mode {
        Palette::DARK
    } else {
        Palette::LIGHT
    };

    if ui.is_rect_visible(rect) {
        ui.painter().rect_filled(rect, CornerRadius::same(8), palette.bg_elevated);

        if state.progress.is_scanning {
            render_scanning_card(ui, rect, state, &palette);
        } else {
            render_idle_screen(ui, rect, state, &palette);
        }
    }
}

fn render_idle_screen(ui: &mut Ui, rect: Rect, state: &AppState, palette: &Palette) {
    let center = rect.center();

    ui.painter().text(
        center - Vec2::new(0.0, 50.0),
        Align2::CENTER_CENTER,
        "📷",
        FontId::proportional(40.0),
        palette.accent,
    );

    ui.painter().text(
        center - Vec2::new(0.0, 10.0),
        Align2::CENTER_CENTER,
        "READY TO SCAN",
        FontId::proportional(18.0),
        palette.text_primary,
    );

    let info_text = if state.game_window.detected {
        format!("Target: {}", state.game_window.process_name)
    } else {
        "Target: Launch Genshin Impact and open Paimon menu".to_string()
    };

    ui.painter().text(
        center + Vec2::new(0.0, 22.0),
        Align2::CENTER_CENTER,
        info_text,
        FontId::proportional(12.0),
        palette.text_muted,
    );

    ui.painter().text(
        center + Vec2::new(0.0, 48.0),
        Align2::CENTER_CENTER,
        "Click START SCAN below or press Enter to begin",
        FontId::proportional(11.0),
        palette.accent,
    );
}

fn render_scanning_card(ui: &mut Ui, rect: Rect, state: &AppState, palette: &Palette) {
    let card_width = (rect.width() * 0.7).min(420.0);
    let card_height = (rect.height() * 0.75).min(320.0);
    let card_rect = Rect::from_center_size(rect.center(), Vec2::new(card_width, card_height));

    ui.painter().rect_filled(card_rect, CornerRadius::same(6), palette.bg_surface);

    ui.allocate_new_ui(UiBuilder::new().max_rect(card_rect), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(14.0);
            ui.label(RichText::new(&state.progress.current_category).small().color(palette.text_muted));
            ui.label(RichText::new(&state.progress.current_item_name).heading().color(palette.text_primary));
            ui.add_space(10.0);

            let preview_rect = ui.allocate_space(Vec2::new(card_width - 32.0, 140.0)).1;
            ui.painter().rect_filled(preview_rect, CornerRadius::same(4), palette.bg_elevated);
            ui.painter().text(
                preview_rect.center(),
                Align2::CENTER_CENTER,
                "Live Crop Stream",
                FontId::proportional(12.0),
                palette.text_muted,
            );

            ui.add_space(10.0);
            let bar = egui::ProgressBar::new(state.progress.progress_ratio)
                .show_percentage()
                .fill(palette.accent);
            ui.add(bar);
        });
    });
}
