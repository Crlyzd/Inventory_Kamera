//! Game entities, lookup tables, and fuzzy dictionary validation.

pub mod matcher;
pub mod models;
pub mod repository;

#[allow(unused_imports)]
pub use matcher::{find_best_match, MatchCandidate};
#[allow(unused_imports)]
pub use models::{GameArtifact, GameCharacter, GameWeapon, StatKey};
#[allow(unused_imports)]
pub use repository::GameDataRepository;
