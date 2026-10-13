//! About and acknowledgments tab.

use crate::ui::theme::Palette;
use egui::{CornerRadius, RichText, Ui, Vec2};

/// Renders the About tab with credits, repository links, and update checker.
pub fn render_tab_about(ui: &mut Ui, palette: &Palette) {
    ui.add_space(6.0);
    render_hero_header(ui, palette);
    ui.add_space(10.0);
    render_authors_card(ui, palette);
    ui.add_space(10.0);
    render_updates_card(ui, palette);
}

fn render_hero_header(ui: &mut Ui, palette: &Palette) {
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("📷").size(32.0).color(palette.accent));
        ui.horizontal_centered(|ui| {
            ui.label(RichText::new("Inventory Kamera").heading().strong().color(palette.text_primary));
            ui.label(RichText::new("v2.0.0 Portable").small().color(palette.accent));
        });
        ui.label(
            RichText::new("Ultra-fast, distraction-free Genshin Impact scanner & GOOD exporter,")
                .small()
                .color(palette.text_muted),
        );
        ui.label(
            RichText::new("crafted in native Windows borderless Rust + egui.")
                .small()
                .color(palette.text_muted),
        );
        ui.add_space(4.0);
        ui.label(RichText::new("Made with ❤️ by Kaleksanan Bagus (@Crlyzd)").strong().color(palette.text_primary));
    });
}

fn render_authors_card(ui: &mut Ui, palette: &Palette) {
    ui.label(RichText::new("AUTHORS & ACKNOWLEDGMENTS").small().color(palette.text_muted).strong());
    ui.add_space(4.0);

    let card_height = 96.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), card_height), egui::Sense::hover());
    ui.painter().rect_filled(rect, CornerRadius::same(6), palette.bg_surface);

    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.add_space(6.0);
        render_author_row(ui, "Author & Maintainer:", "Kaleksanan Bagus (@Crlyzd)", "https://github.com/Crlyzd");
        render_author_row(ui, "Forked From:", "Rogerio Hirooka (@rogerioth)", "https://github.com/rogerioth/Inventory_Kamera");
        render_author_row(ui, "Original Creator:", "Andrewthe13th", "https://github.com/Andrewthe13th/Inventory_Kamera");
    });
}

fn render_author_row(ui: &mut Ui, label: &str, name: &str, url: &'static str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(RichText::new(label).small().color(Palette::DARK.text_muted));
        ui.label(RichText::new(name).small().strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            if ui.small_button("Open ↗").clicked() {
                let _ = open::that(url);
            }
        });
    });
}

fn render_updates_card(ui: &mut Ui, palette: &Palette) {
    ui.label(RichText::new("UPDATES & FEEDBACK").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Software Update: v2.0.0 (Latest)").small());
        if ui.button("Check for Updates").clicked() {
            let _ = open::that("https://github.com/Crlyzd/Inventory_Kamera/releases");
        }
        ui.add_space(12.0);
        ui.label(RichText::new("Report Issues:").small());
        if ui.button("GitHub Issues ↗").clicked() {
            let _ = open::that("https://github.com/Crlyzd/Inventory_Kamera/issues");
        }
    });
}
