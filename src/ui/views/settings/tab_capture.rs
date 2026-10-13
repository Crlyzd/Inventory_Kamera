//! Capture and OCR engine settings tab.

use crate::core::AppState;
use crate::ui::theme::Palette;
use egui::{RichText, Slider, Ui};

/// Renders the Capture engine and OCR timing configuration tab.
pub fn render_tab_capture(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.add_space(8.0);
    render_backend_section(ui, state, palette);
    ui.add_space(14.0);
    render_delays_section(ui, state, palette);
    ui.add_space(14.0);
    render_ocr_section(ui, state, palette);
}

fn render_backend_section(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.label(RichText::new("WINDOW CAPTURE ENGINE").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.radio_value(&mut state.capture.use_wgc, true, "Windows Graphics Capture (WGC)");
        ui.add_space(8.0);
        ui.radio_value(&mut state.capture.use_wgc, false, "BitBlt Fallback");
    });
    ui.checkbox(&mut state.capture.enforce_aspect_ratio, "Enforce 16:9 / 16:10 aspect ratio verification");
}

fn render_delays_section(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.label(RichText::new("SCAN SPEED & TIMINGS").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Menu Traversal Delay:").small());
        ui.add(Slider::new(&mut state.capture.traversal_delay_ms, 30..=200).suffix(" ms"));
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new("Scroll Settle Duration:").small());
        ui.add(Slider::new(&mut state.capture.scroll_settle_ms, 50..=400).suffix(" ms"));
    });
}

fn render_ocr_section(ui: &mut Ui, state: &mut AppState, palette: &Palette) {
    ui.label(RichText::new("OCR WORKER POOL").small().color(palette.text_muted).strong());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Parallel OCR Workers:").small());
        ui.add(Slider::new(&mut state.capture.ocr_threads, 1..=8).suffix(" Threads"));
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new("Confidence Threshold:").small());
        ui.add(Slider::new(&mut state.capture.min_confidence, 70..=100).suffix(" %"));
    });
}
