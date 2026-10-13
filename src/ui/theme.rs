//! Borderless design system and theme definitions.

use egui::{Color32, CornerRadius, Stroke, Visuals};

/// Palette container for cohesive UI rendering.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub bg_root: Color32,
    pub bg_elevated: Color32,
    pub bg_surface: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub danger: Color32,
}

impl Palette {
    /// Dark mode color palette with slate-charcoal tones.
    pub const DARK: Self = Self {
        bg_root: Color32::from_rgb(18, 19, 22),
        bg_elevated: Color32::from_rgb(26, 29, 33),
        bg_surface: Color32::from_rgb(36, 40, 47),
        text_primary: Color32::from_rgb(240, 243, 246),
        text_muted: Color32::from_rgb(139, 148, 158),
        accent: Color32::from_rgb(68, 194, 179),
        accent_hover: Color32::from_rgb(86, 218, 203),
        success: Color32::from_rgb(70, 200, 120),
        warning: Color32::from_rgb(245, 166, 35),
        danger: Color32::from_rgb(235, 87, 87),
    };

    /// Light mode color palette with soft neutral tones.
    pub const LIGHT: Self = Self {
        bg_root: Color32::from_rgb(245, 247, 250),
        bg_elevated: Color32::from_rgb(234, 238, 243),
        bg_surface: Color32::from_rgb(255, 255, 255),
        text_primary: Color32::from_rgb(33, 37, 41),
        text_muted: Color32::from_rgb(108, 117, 125),
        accent: Color32::from_rgb(46, 150, 138),
        accent_hover: Color32::from_rgb(38, 128, 117),
        success: Color32::from_rgb(40, 167, 69),
        warning: Color32::from_rgb(217, 130, 0),
        danger: Color32::from_rgb(220, 53, 69),
    };
}

/// Applies borderless, stroke-free visual styling to egui.
pub fn apply_theme(ctx: &egui::Context, dark_mode: bool) {
    let mut visuals = if dark_mode {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    let palette = if dark_mode {
        Palette::DARK
    } else {
        Palette::LIGHT
    };

    visuals.panel_fill = palette.bg_root;
    visuals.window_fill = palette.bg_elevated;
    visuals.window_stroke = Stroke::NONE;

    // Eliminate strokes on all widget states
    visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);
    visuals.widgets.noninteractive.bg_fill = palette.bg_elevated;
    visuals.widgets.noninteractive.fg_stroke.color = palette.text_primary;

    visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.inactive.bg_fill = palette.bg_surface;
    visuals.widgets.inactive.fg_stroke.color = palette.text_primary;

    visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.bg_fill = palette.accent_hover;

    visuals.widgets.active.bg_stroke = Stroke::NONE;
    visuals.widgets.active.corner_radius = CornerRadius::same(6);
    visuals.widgets.active.bg_fill = palette.accent;

    ctx.set_visuals(visuals);
}
