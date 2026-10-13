//! Windows automation, window discovery, coordinate scaling, and input simulation.

pub mod input_simulator;
pub mod window_finder;
pub mod window_scaler;

#[allow(unused_imports)]
pub use input_simulator::{send_key_down, send_key_tap, send_key_up, KeyCodes};
#[allow(unused_imports)]
pub use window_finder::{find_genshin_window, GameWindowTarget};
#[allow(unused_imports)]
pub use window_scaler::{validate_aspect_ratio, AspectRatio, GenshinCrops, NormalizedRect};
