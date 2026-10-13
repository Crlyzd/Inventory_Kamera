//! Scraper abstraction trait and stepping state machine.
#![allow(dead_code)]

use crate::data::models::{GameArtifact, GameWeapon};

/// An item recognized during scanning traversal.
#[derive(Debug, Clone)]
pub enum ScannedItem {
    Weapon(GameWeapon),
    Artifact(GameArtifact),
}

/// Outcome of an atomic scraper traversal step.
#[derive(Debug, Clone)]
pub enum ScraperStepResult {
    /// Scanner is navigating to the next slot or settling.
    Navigating,
    /// An item card was read and recognized.
    ItemFound(ScannedItem),
    /// Current category scanning has concluded.
    CategoryFinished,
    /// An unrecoverable traversal error occurred.
    Failed(String),
}

/// Abstract contract for menu category walkers.
pub trait Scraper {
    /// Display name of the category (e.g. "Weapons", "Artifacts").
    fn category_name(&self) -> &str;

    /// Executes one discrete navigation and scanning step.
    fn step(&mut self) -> ScraperStepResult;

    /// Resets the scraper back to initial slot index 0.
    fn reset(&mut self);
}
