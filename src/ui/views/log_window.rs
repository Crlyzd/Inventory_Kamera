//! Standalone self-contained diagnostic log viewer window.

use crate::core::logger::{LogLevel, LogStore};
use crate::core::AppState;
use crate::ui::theme::Palette;
use egui::{Color32, CornerRadius, Id, LayerId, Order, Rect, RichText, ScrollArea, Ui, Vec2};

/// Renders the floating diagnostic logs window when active.
pub fn render_log_window(ctx: &egui::Context, state: &mut AppState, log_store: &LogStore) {
    if !state.show_logs {
        return;
    }

    let palette = if state.is_dark_mode { Palette::DARK } else { Palette::LIGHT };
    let screen_rect = ctx.screen_rect();
    let modal_size = Vec2::new(620.0, 480.0);
    let modal_rect = Rect::from_center_size(screen_rect.center(), modal_size);

    let painter = ctx.layer_painter(LayerId::new(Order::Middle, Id::new("log_backdrop")));
    painter.rect_filled(screen_rect, CornerRadius::ZERO, Color32::from_black_alpha(150));
    painter.rect_filled(modal_rect, CornerRadius::same(8), palette.bg_elevated);

    egui::Area::new(Id::new("log_modal_area"))
        .order(Order::Foreground)
        .fixed_pos(modal_rect.min + Vec2::new(14.0, 14.0))
        .show(ctx, |ui| {
            ui.set_width(modal_rect.width() - 28.0);
            ui.set_height(modal_rect.height() - 28.0);

            render_log_header(ui, state, &palette);
            ui.add_space(8.0);
            render_log_entries(ui, log_store, &palette);
            ui.add_space(8.0);
            render_log_actions(ui, log_store);
        });
}

fn render_log_header(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("📄 Diagnostics & Scan Logs").heading().strong().color(palette.text_primary));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button(RichText::new("✕").strong()).clicked() {
                state.show_logs = false;
            }
        });
    });
}

fn render_log_entries(ui: &mut Ui, log_store: &LogStore, palette: &Palette) {
    let entries = log_store.get_entries();

    ScrollArea::vertical()
        .max_height(370.0)
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            if entries.is_empty() {
                ui.add_space(20.0);
                ui.label(RichText::new("No log entries recorded yet.").small().color(palette.text_muted));
                return;
            }

            for entry in entries {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&entry.timestamp).monospace().small().color(palette.text_muted));
                    let tag_color = match entry.level {
                        LogLevel::Info => palette.accent,
                        LogLevel::Warning => Color32::from_rgb(230, 160, 40),
                        LogLevel::Error => palette.danger,
                        LogLevel::Scan => palette.success,
                    };

                    let badge = RichText::new(format!(" {} ", entry.level.as_str())).small().strong().color(tag_color);
                    ui.label(badge);
                    ui.label(RichText::new(&entry.message).small().color(palette.text_primary));
                });
            }
        });
}

fn render_log_actions(ui: &mut Ui, log_store: &LogStore) {
    ui.horizontal(|ui| {
        if ui.button("Clear Logs").clicked() {
            log_store.clear();
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Copy to Clipboard").clicked() {
                let lines: Vec<String> = log_store
                    .get_entries()
                    .into_iter()
                    .map(|e| format!("{} [{}] {}", e.timestamp, e.level.as_str(), e.message))
                    .collect();
                ui.ctx().copy_text(lines.join("\n"));
            }
        });
    });
}
