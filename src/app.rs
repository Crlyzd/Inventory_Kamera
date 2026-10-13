//! Top-level eframe application controller.

use crate::core::{AppState, EventBus, LogStore};
use crate::ui::apply_theme;
use crate::ui::views::{
    render_action_dock, render_chip_bar, render_header_bar, render_log_window, render_ocr_stage,
    render_settings_window,
};
use egui::CentralPanel;

/// Root Inventory Kamera eframe application struct.
pub struct InventoryKameraApp {
    pub state: AppState,
    pub log_store: LogStore,
    #[allow(dead_code)]
    pub event_bus: EventBus,
}

impl InventoryKameraApp {
    /// Constructs a new application instance.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::new();
        let log_store = LogStore::new();
        let event_bus = EventBus::new();

        log_store.info("Initialized Inventory Kamera v2.0.0 (Rust x64)");
        log_store.info("Borderless UI shell ready (800x600 fixed viewport)");

        apply_theme(&cc.egui_ctx, state.is_dark_mode);
        Self {
            state,
            log_store,
            event_bus,
        }
    }
}

impl eframe::App for InventoryKameraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.state.is_dark_mode);

        if let Some(target) = crate::automation::find_genshin_window() {
            if !self.state.game_window.detected {
                self.log_store.info(format!(
                    "Attached to Genshin: {} ({}x{}){}",
                    target.title,
                    target.width,
                    target.height,
                    if target.is_minimized { " [Minimized]" } else { "" }
                ));
            }
            self.state.game_window.detected = true;
            self.state.game_window.is_minimized = target.is_minimized;
            self.state.game_window.process_name = target.title;
            self.state.game_window.dimensions = (target.width, target.height);
        } else {
            if self.state.game_window.detected {
                self.log_store.info("Genshin game window disconnected");
            }
            self.state.game_window.detected = false;
            self.state.game_window.is_minimized = false;
        }

        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                render_header_bar(ui, &mut self.state);
                render_chip_bar(ui, &mut self.state);

                let dock_height = 48.0;
                let available_height = ui.available_height() - dock_height;

                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), available_height),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        render_ocr_stage(ui, &self.state);
                    },
                );

                render_action_dock(ui, &mut self.state);
            });

        // Dedicated floating dialogs
        render_settings_window(ctx, &mut self.state);
        render_log_window(ctx, &mut self.state, &self.log_store);
    }
}
