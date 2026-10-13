//! OCR preprocessing, inference management, and text normalization.

pub mod normalizer;

#[allow(unused_imports)]
pub use normalizer::{normalize_stat_label, parse_stat_value, sanitize_ocr_text};
