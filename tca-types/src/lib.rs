//! Core types for Terminal Colors Architecture (TCA)
//!
//! This crate provides the foundational type definitions used across
//! the TCA ecosystem for theme representation and manipulation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete TCA theme definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub meta: Meta,
    pub palette: Palette,
    pub ansi: Ansi,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base16: Option<Base16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<HashMap<String, String>>,
}

/// Theme metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meta {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Color palette with hue ramps
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Palette {
    pub neutral: HueRamp,
    #[serde(flatten)]
    pub other: HashMap<String, HueRamp>,
}

/// A hue ramp containing color values at different shades
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HueRamp(pub HashMap<String, String>);

/// ANSI 16-color definitions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ansi {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    #[serde(rename = "brightBlack")]
    pub bright_black: String,
    #[serde(rename = "brightRed")]
    pub bright_red: String,
    #[serde(rename = "brightGreen")]
    pub bright_green: String,
    #[serde(rename = "brightYellow")]
    pub bright_yellow: String,
    #[serde(rename = "brightBlue")]
    pub bright_blue: String,
    #[serde(rename = "brightMagenta")]
    pub bright_magenta: String,
    #[serde(rename = "brightCyan")]
    pub bright_cyan: String,
    #[serde(rename = "brightWhite")]
    pub bright_white: String,
}

/// Base16 color definitions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Base16(pub HashMap<String, String>);

impl Theme {
    /// Resolve a palette reference to its color value
    ///
    /// Supports the following formats:
    /// - Direct hex color: `#ff0000`
    /// - Palette reference: `palette.red.5`
    pub fn resolve(&self, color_ref: &str) -> Option<String> {
        if color_ref.starts_with('#') {
            return Some(color_ref.to_string());
        }

        // Strip optional $ prefix for backward compatibility
        let reference = color_ref;

        let parts: Vec<&str> = reference.split('.').collect();

        match parts.len() {
            3 if parts[0] == "palette" => {
                let (hue, shade) = (parts[1], parts[2]);
                if hue == "neutral" {
                    self.palette.neutral.0.get(shade).cloned()
                } else {
                    self.palette
                        .other
                        .get(hue)
                        .and_then(|ramp| ramp.0.get(shade).cloned())
                }
            }
            _ => None,
        }
    }
}

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let hex = if hex.starts_with("#") {
        &hex[1..7]
    } else {
        hex
    };
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_theme() -> Theme {
        let mut neutral = HashMap::new();
        neutral.insert("1".to_string(), "#1a1a1a".to_string());
        neutral.insert("2".to_string(), "#666666".to_string());
        neutral.insert("5".to_string(), "#fafafa".to_string());

        let mut red = HashMap::new();
        red.insert("3".to_string(), "#ff0000".to_string());
        red.insert("5".to_string(), "#ff5555".to_string());

        let mut other_hues = HashMap::new();
        other_hues.insert("red".to_string(), HueRamp(red));

        Theme {
            meta: Meta {
                name: "Test Theme".to_string(),
                slug: Some("test".to_string()),
                author: None,
                version: None,
                description: None,
            },
            palette: Palette {
                neutral: HueRamp(neutral),
                other: other_hues,
            },
            ansi: Ansi {
                black: "neutral.1".to_string(),
                red: "red.4".to_string(),
                green: "#00ff00".to_string(),
                yellow: "#ffff00".to_string(),
                blue: "#0000ff".to_string(),
                magenta: "#ff00ff".to_string(),
                cyan: "#00ffff".to_string(),
                white: "neutral.5".to_string(),
                bright_black: "neutral.2".to_string(),
                bright_red: "red.5".to_string(),
                bright_green: "#00ff00".to_string(),
                bright_yellow: "#ffff00".to_string(),
                bright_blue: "#0000ff".to_string(),
                bright_magenta: "#ff00ff".to_string(),
                bright_cyan: "#00ffff".to_string(),
                bright_white: "$neutral.5".to_string(),
            },
            base16: None,
            semantic: None,
            ui: None,
        }
    }

    #[test]
    fn test_resolve_hex_color() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("#ff0000"), Some("#ff0000".to_string()));
    }

    #[test]
    fn test_resolve_simple_reference() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("$red.3"), Some("#ff0000".to_string()));
        assert_eq!(theme.resolve("$neutral.5"), Some("#fafafa".to_string()));
    }

    #[test]
    fn test_resolve_palette_reference() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("palette.red.3"), Some("#ff0000".to_string()));
        assert_eq!(
            theme.resolve("palette.neutral.1"),
            Some("#1a1a1a".to_string())
        );
    }

    #[test]
    fn test_resolve_invalid() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("$nonexistent.3"), None);
        assert_eq!(theme.resolve("invalid"), None);
    }
}
