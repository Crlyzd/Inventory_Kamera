//! Minimalist titlebar window control buttons.

use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

/// Renders a frameless window control button (minimize or close).
pub fn titlebar_button(ui: &mut Ui, label: &str, is_close: bool) -> Response {
    let size = Vec2::new(32.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let hover = response.hovered();
        let bg_fill = if hover {
            if is_close {
                Color32::from_rgb(232, 17, 35) // Windows close red
            } else {
                Color32::from_white_alpha(32)
            }
        } else {
            Color32::TRANSPARENT
        };

        ui.painter().rect_filled(rect, CornerRadius::same(4), bg_fill);

        let text_color = if hover && is_close {
            Color32::WHITE
        } else {
            ui.visuals().text_color()
        };

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(12.0),
            text_color,
        );
    }

    response
}
