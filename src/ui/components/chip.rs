//! Interactive category toggle chip widget.

use crate::ui::theme::Palette;
use egui::{CornerRadius, Response, Sense, TextStyle, Ui, Vec2};

/// Renders a toggleable category chip with subtle rounding and no borders.
pub fn category_chip(ui: &mut Ui, label: &str, selected: &mut bool, dark_mode: bool) -> Response {
    let padding = Vec2::new(14.0, 6.0);
    let text = if *selected {
        format!("{} ●", label)
    } else {
        label.to_string()
    };

    let font_id = TextStyle::Button.resolve(ui.style());
    let galley = ui.painter().layout_no_wrap(text, font_id, ui.visuals().text_color());
    let desired_size = galley.size() + padding * 2.0;

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
        *selected = !*selected;
    }

    if ui.is_rect_visible(rect) {
        let palette = if dark_mode { Palette::DARK } else { Palette::LIGHT };
        let hovered = response.hovered();

        let bg_color = match (*selected, hovered) {
            (true, _) => palette.accent,
            (false, true) => palette.bg_surface,
            (false, false) => palette.bg_elevated,
        };

        let text_color = if *selected {
            egui::Color32::WHITE
        } else if hovered {
            palette.text_primary
        } else {
            palette.text_muted
        };

        ui.painter().rect_filled(rect, CornerRadius::same(14), bg_color);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            if *selected { format!("{} ●", label) } else { label.to_string() },
            TextStyle::Button.resolve(ui.style()),
            text_color,
        );
    }

    response
}
