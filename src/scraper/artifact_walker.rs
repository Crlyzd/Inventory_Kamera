//! Artifact inventory menu traversal walker.
#![allow(dead_code)]

use crate::automation::input_simulator::{send_key_tap, KeyCodes};
use crate::data::models::GameArtifact;
use crate::scraper::trait_scraper::{ScannedItem, Scraper, ScraperStepResult};

/// State machine for walking the artifact inventory grid.
pub struct ArtifactWalker {
    current_index: usize,
    max_items: usize,
    min_rarity: u8,
    finished: bool,
}

impl Default for ArtifactWalker {
    fn default() -> Self {
        Self::new(200, 5)
    }
}

impl ArtifactWalker {
    pub fn new(max_items: usize, min_rarity: u8) -> Self {
        Self {
            current_index: 0,
            max_items,
            min_rarity,
            finished: false,
        }
    }
}

impl Scraper for ArtifactWalker {
    fn category_name(&self) -> &str {
        "Artifacts"
    }

    fn step(&mut self) -> ScraperStepResult {
        if self.finished || self.current_index >= self.max_items {
            self.finished = true;
            return ScraperStepResult::CategoryFinished;
        }

        send_key_tap(KeyCodes::ARROW_RIGHT, 30);
        self.current_index += 1;

        let artifact = GameArtifact {
            name: "Flower of Accolades".to_string(),
            set_key: "GladiatorsFinale".to_string(),
            slot_key: "flower".to_string(),
            rarity: 5,
        };

        if artifact.rarity >= self.min_rarity {
            ScraperStepResult::ItemFound(ScannedItem::Artifact(artifact))
        } else {
            ScraperStepResult::Navigating
        }
    }

    fn reset(&mut self) {
        self.current_index = 0;
        self.finished = false;
    }
}
