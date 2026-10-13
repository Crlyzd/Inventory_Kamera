//! Game data lookup repository with defensive loading and dictionary indexing.
#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Central game dictionary repository.
#[derive(Debug, Clone, Default)]
pub struct GameDataRepository {
    pub weapons: HashSet<String>,
    pub artifact_sets: HashSet<String>,
    pub characters: HashSet<String>,
}

impl GameDataRepository {
    /// Initializes repository with embedded default standard game dictionaries.
    pub fn new() -> Self {
        let mut repo = Self::default();
        repo.load_defaults();
        repo
    }

    /// Loads custom dictionary lists defensively from disk if present.
    pub fn load_from_directory(&mut self, dir_path: impl AsRef<Path>) {
        let dir = dir_path.as_ref();
        if !dir.exists() {
            return;
        }

        Self::load_list_file(&dir.join("weapons.json"), &mut self.weapons);
        Self::load_list_file(&dir.join("artifacts.json"), &mut self.artifact_sets);
        Self::load_list_file(&dir.join("characters.json"), &mut self.characters);
    }

    fn load_list_file(path: &Path, target_set: &mut HashSet<String>) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(list) = serde_json::from_str::<Vec<String>>(&content) {
                target_set.extend(list);
            }
        }
    }

    fn load_defaults(&mut self) {
        // High-frequency seed entries for zero-dependency baseline validation
        self.weapons.insert("Staff of Homa".to_string());
        self.weapons.insert("Mistsplitter Reforged".to_string());
        self.weapons.insert("Aqua Simulacra".to_string());
        self.weapons.insert("Favonius Sword".to_string());
        self.weapons.insert("Favonius Lance".to_string());

        self.artifact_sets.insert("Gladiator's Finale".to_string());
        self.artifact_sets.insert("Viridescent Venerer".to_string());
        self.artifact_sets.insert("Emblem of Severed Fate".to_string());
        self.artifact_sets.insert("Deepwood Memories".to_string());

        self.characters.insert("Hu Tao".to_string());
        self.characters.insert("Raiden Shogun".to_string());
        self.characters.insert("Furina".to_string());
        self.characters.insert("Kaedehara Kazuha".to_string());
    }
}
