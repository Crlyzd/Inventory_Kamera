//! Text normalization and regex sanitization for raw OCR output.
#![allow(dead_code)]

/// Sanitizes raw OCR text strings, stripping noise and standardizing spacing.
pub fn sanitize_ocr_text(raw: &str) -> String {
    let trimmed = raw.trim();
    let cleaned: String = trimmed
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '%' || *c == '+' || *c == '.' || *c == '\'')
        .collect();

    cleaned.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Normalizes numeric stat strings extracted from item cards (e.g., "46. 6 %" -> 46.6).
pub fn parse_stat_value(raw: &str) -> Option<f32> {
    let clean: String = raw.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
    clean.parse::<f32>().ok()
}

/// Normalizes recognized stat header names into standard display labels.
pub fn normalize_stat_label(raw: &str) -> &'static str {
    let lower = raw.to_lowercase();
    if lower.contains("crit rate") {
        "CRIT Rate%"
    } else if lower.contains("crit dmg") || lower.contains("crit damage") {
        "CRIT DMG%"
    } else if lower.contains("energy recharge") {
        "Energy Recharge%"
    } else if lower.contains("elemental mastery") {
        "Elemental Mastery"
    } else if lower.contains("healing") {
        "Healing Bonus%"
    } else if lower.contains("atk") {
        "ATK"
    } else if lower.contains("def") {
        "DEF"
    } else if lower.contains("hp") {
        "HP"
    } else {
        "Unknown Stat"
    }
}
