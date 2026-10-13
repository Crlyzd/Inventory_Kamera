//! Dedicated floating settings modal window.

use crate::core::{AppState, SettingsTab};
use crate::ui::theme::Palette;
use crate::ui::views::settings::{render_tab_about, render_tab_capture, render_tab_general};
use egui::{Color32, CornerRadius, Id, LayerId, Order, Rect, RichText, Ui, Vec2};

/// Renders the floating Settings dialog when active.
pub fn render_settings_window(ctx: &egui::Context, state: &mut AppState) {
    if !state.show_settings {
        return;
    }

    let palette = if state.is_dark_mode {
        Palette::DARK
    } else {
        Palette::LIGHT
    };

    let screen_rect = ctx.screen_rect();
    let modal_size = Vec2::new(600.0, 480.0);
    let modal_rect = Rect::from_center_size(screen_rect.center(), modal_size);

    // Semi-transparent backdrop dimming
    let painter = ctx.layer_painter(LayerId::new(Order::Middle, Id::new("modal_backdrop")));
    painter.rect_filled(screen_rect, CornerRadius::ZERO, Color32::from_black_alpha(150));
    painter.rect_filled(modal_rect, CornerRadius::same(8), palette.bg_elevated);

    egui::Area::new(Id::new("settings_modal_area"))
        .order(Order::Foreground)
        .fixed_pos(modal_rect.min + Vec2::new(14.0, 14.0))
        .show(ctx, |ui| {
            ui.set_width(modal_rect.width() - 28.0);
            ui.set_height(modal_rect.height() - 28.0);

            render_modal_header(ui, state, &palette);
            ui.add_space(8.0);
            render_active_tab_content(ui, state, &palette);
        });
}

fn render_modal_header(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.horizontal(|ui| {
        render_tab_pill(ui, "⚙ General", SettingsTab::General, &mut state.active_settings_tab, palette);
        ui.add_space(4.0);
        render_tab_pill(ui, "📷 Capture", SettingsTab::Capture, &mut state.active_settings_tab, palette);
        ui.add_space(4.0);
        render_tab_pill(ui, "♡ About", SettingsTab::About, &mut state.active_settings_tab, palette);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button(RichText::new("✕").strong()).clicked() {
                state.show_settings = false;
            }
        });
    });
}

fn render_tab_pill(ui: &mut Ui, label: &str, tab: SettingsTab, current: &mut SettingsTab, palette: &Palette) {
    let active = *current == tab;
    let text = if active {
        RichText::new(label).strong().color(Color32::WHITE)
    } else {
        RichText::new(label).color(palette.text_muted)
    };

    let fill = if active { palette.accent } else { palette.bg_surface };
    let btn = egui::Button::new(text).fill(fill).corner_radius(CornerRadius::same(12));

    if ui.add(btn).clicked() {
        *current = tab;
    }
}

fn render_active_tab_content(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    egui::ScrollArea::vertical().show(ui, |ui| match state.active_settings_tab {
        SettingsTab::General => render_tab_general(ui, state, palette),
        SettingsTab::Capture => render_tab_capture(ui, state, palette),
        SettingsTab::About => render_tab_about(ui, palette),
    });
}
