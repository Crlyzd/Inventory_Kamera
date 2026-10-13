//! Genshin Open Object Description (GOOD v2) data models.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Standard GOOD v2 database envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodDatabase {
    pub format: String,
    pub version: u32,
    pub source: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub weapons: Vec<GoodWeapon>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub artifacts: Vec<GoodArtifact>,
}

impl Default for GoodDatabase {
    fn default() -> Self {
        Self {
            format: "GOOD".to_string(),
            version: 2,
            source: "InventoryKamera".to_string(),
            weapons: Vec::new(),
            artifacts: Vec::new(),
        }
    }
}

/// GOOD format weapon representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodWeapon {
    pub key: String,
    pub level: u8,
    pub ascension: u8,
    pub refinement: u8,
    pub location: String,
    pub lock: bool,
}

/// GOOD format artifact representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    pub set_key: String,
    pub slot_key: String,
    pub level: u8,
    pub rarity: u8,
    pub main_stat_key: String,
    pub location: String,
    pub lock: bool,
    pub substats: Vec<GoodSubstat>,
}

/// GOOD format artifact substat entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodSubstat {
    pub key: String,
    pub value: f32,
}
