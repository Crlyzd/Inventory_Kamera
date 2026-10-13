//! Game entities and dictionary data structures for validation.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Recognized weapon entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameWeapon {
    pub name: String,
    pub rarity: u8,
    pub weapon_type: String,
}

/// Recognized artifact piece.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameArtifact {
    pub name: String,
    pub set_key: String,
    pub slot_key: String,
    pub rarity: u8,
}

/// Recognized character definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCharacter {
    pub key: String,
    pub name: String,
    pub element: String,
}

/// Normalized in-game main stat and substat keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatKey {
    Hp,
    HpPercent,
    Atk,
    AtkPercent,
    Def,
    DefPercent,
    ElementalMastery,
    EnergyRecharge,
    CritRate,
    CritDmg,
    HealingBonus,
    PhysicalDmg,
    AnemoDmg,
    GeoDmg,
    ElectroDmg,
    HydroDmg,
    PyroDmg,
    CryoDmg,
    DendroDmg,
}

impl StatKey {
    pub fn as_good_key(&self) -> &'static str {
        match self {
            Self::Hp => "hp",
            Self::HpPercent => "hp_",
            Self::Atk => "atk",
            Self::AtkPercent => "atk_",
            Self::Def => "def",
            Self::DefPercent => "def_",
            Self::ElementalMastery => "eleMas",
            Self::EnergyRecharge => "enerRech_",
            Self::CritRate => "critRate_",
            Self::CritDmg => "critDMG_",
            Self::HealingBonus => "heal_",
            Self::PhysicalDmg => "physical_dmg_",
            Self::AnemoDmg => "anemo_dmg_",
            Self::GeoDmg => "geo_dmg_",
            Self::ElectroDmg => "electro_dmg_",
            Self::HydroDmg => "hydro_dmg_",
            Self::PyroDmg => "pyro_dmg_",
            Self::CryoDmg => "cryo_dmg_",
            Self::DendroDmg => "dendro_dmg_",
        }
    }
}
