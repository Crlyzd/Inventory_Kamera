//! Weapon inventory menu traversal walker.
#![allow(dead_code)]

use crate::automation::input_simulator::{send_key_tap, KeyCodes};
use crate::data::models::GameWeapon;
use crate::scraper::trait_scraper::{ScannedItem, Scraper, ScraperStepResult};

/// State machine for walking the weapon inventory grid.
pub struct WeaponWalker {
    current_index: usize,
    max_items: usize,
    finished: bool,
}

impl Default for WeaponWalker {
    fn default() -> Self {
        Self::new(100)
    }
}

impl WeaponWalker {
    pub fn new(max_items: usize) -> Self {
        Self {
            current_index: 0,
            max_items,
            finished: false,
        }
    }
}

impl Scraper for WeaponWalker {
    fn category_name(&self) -> &str {
        "Weapons"
    }

    fn step(&mut self) -> ScraperStepResult {
        if self.finished || self.current_index >= self.max_items {
            self.finished = true;
            return ScraperStepResult::CategoryFinished;
        }

        // Navigate to the next weapon in grid
        send_key_tap(KeyCodes::ARROW_RIGHT, 30);
        self.current_index += 1;

        // Yield simulated weapon for verification pipeline
        let weapon = GameWeapon {
            name: "Staff of Homa".to_string(),
            rarity: 5,
            weapon_type: "Polearm".to_string(),
        };

        ScraperStepResult::ItemFound(ScannedItem::Weapon(weapon))
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.finished = false;
    }
}
