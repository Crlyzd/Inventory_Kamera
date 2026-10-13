//! Minimalist borderless button widget.

use crate::ui::theme::Palette;
use egui::{Color32, CornerRadius, Response, Sense, TextStyle, Ui, Vec2};

/// Renders a flat, borderless button with custom palette styling.
pub fn flat_button(
    ui: &mut Ui,
    label: &str,
    is_primary: bool,
    dark_mode: bool,
) -> Response {
    let padding = Vec2::new(16.0, 8.0);
    let font_id = TextStyle::Button.resolve(ui.style());
    let galley = ui.painter().layout_no_wrap(label.to_string(), font_id, Color32::WHITE);
    let desired_size = galley.size() + padding * 2.0;

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let palette = if dark_mode { Palette::DARK } else { Palette::LIGHT };
        let hovered = response.hovered();

        let bg_fill = if is_primary {
            if hovered { palette.accent_hover } else { palette.accent }
        } else if hovered {
            palette.bg_surface
        } else {
            palette.bg_elevated
        };

        let text_color = if is_primary {
            Color32::WHITE
        } else {
            palette.text_primary
        };

        ui.painter().rect_filled(rect, CornerRadius::same(6), bg_fill);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            TextStyle::Button.resolve(ui.style()),
            text_color,
        );
    }

    response
}
