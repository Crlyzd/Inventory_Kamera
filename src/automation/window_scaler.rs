//! Aspect ratio validation and normalized coordinate scaling for Genshin Impact.
#![allow(dead_code)]

/// Supported in-game aspect ratio categories.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectRatio {
    SixteenNine,
    SixteenTen,
}

/// A normalized bounding box within the client area (values from 0.0 to 1.0).
#[derive(Debug, Clone, Copy)]
pub struct NormalizedRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl NormalizedRect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Converts normalized bounds to actual physical client pixel coordinates.
    pub fn to_pixel_rect(&self, client_width: u32, client_height: u32) -> (u32, u32, u32, u32) {
        let px_x = (self.x * client_width as f32).round() as u32;
        let px_y = (self.y * client_height as f32).round() as u32;
        let px_w = (self.width * client_width as f32).round() as u32;
        let px_h = (self.height * client_height as f32).round() as u32;
        (px_x, px_y, px_w, px_h)
    }
}

/// Standardized crop regions for weapon and artifact inspection cards.
pub struct GenshinCrops;

impl GenshinCrops {
    /// 16:9 Item title and name card crop region.
    pub const ITEM_NAME_16_9: NormalizedRect = NormalizedRect::new(0.665, 0.105, 0.285, 0.055);

    /// 16:9 Item stats and description panel.
    pub const ITEM_STATS_16_9: NormalizedRect = NormalizedRect::new(0.665, 0.200, 0.285, 0.450);

    /// 16:9 Navigation menu slot crop region.
    pub const GRID_SLOT_16_9: NormalizedRect = NormalizedRect::new(0.080, 0.140, 0.500, 0.750);
}

/// Evaluates client dimensions to verify 16:9 or 16:10 aspect ratio compliance.
pub fn validate_aspect_ratio(width: u32, height: u32) -> Result<AspectRatio, String> {
    if height == 0 {
        return Err("Client height cannot be zero".to_string());
    }

    let ratio = width as f32 / height as f32;

    if (ratio - (16.0 / 9.0)).abs() < 0.05 {
        Ok(AspectRatio::SixteenNine)
    } else if (ratio - (16.0 / 10.0)).abs() < 0.05 {
        Ok(AspectRatio::SixteenTen)
    } else {
        Err(format!(
            "Unsupported aspect ratio: {:.2}:1 (Resolution: {}x{}). Please configure Genshin to 16:9 or 16:10 windowed.",
            ratio, width, height
        ))
    }
}
