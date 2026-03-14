//! Core types for Terminal Colors Architecture (TCA)
//!
//! This crate provides the foundational type definitions used across
//! the TCA ecosystem for theme representation and manipulation.

#![warn(missing_docs)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod builtin;
pub use builtin::BuiltinTheme;

/// Errors that can occur when parsing a hex color string.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HexColorError {
    /// The hex string (excluding a leading `#`) was not exactly 6 characters.
    #[error("hex color must be 6 characters (got {0})")]
    InvalidLength(usize),
    /// A hex digit could not be parsed.
    #[error("invalid hex digit: {0}")]
    InvalidHex(#[from] std::num::ParseIntError),
}

/// A complete TCA theme definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme {
    /// Theme metadata. Serde key is `theme` to match the TOML section name.
    #[serde(rename = "theme")]
    pub meta: Meta,
    /// ANSI 16-color definitions.
    pub ansi: Ansi,
    /// Optional named color palette.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub palette: Option<Palette>,
    /// Optional Base16 color scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base16: Option<Base16>,
    /// Semantic color roles.
    pub semantic: Semantic,
    /// UI element colors.
    pub ui: Ui,
}

/// Theme metadata (TOML section `[theme]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Meta {
    /// Human-readable theme name.
    pub name: String,
    /// URL-safe identifier for the theme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Theme author name or contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Semantic version string (e.g. `"1.0.0"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Short description of the theme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// `true` for dark themes, `false` for light themes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<bool>,
}

/// ANSI 16-color definitions (TOML section `[ansi]`).
///
/// All values must be direct `#RRGGBB` hex strings — no palette references.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ansi {
    /// ANSI color 0 — black.
    pub black: String,
    /// ANSI color 1 — red.
    pub red: String,
    /// ANSI color 2 — green.
    pub green: String,
    /// ANSI color 3 — yellow.
    pub yellow: String,
    /// ANSI color 4 — blue.
    pub blue: String,
    /// ANSI color 5 — magenta.
    pub magenta: String,
    /// ANSI color 6 — cyan.
    pub cyan: String,
    /// ANSI color 7 — white.
    pub white: String,
    /// ANSI color 8 — bright black (dark gray).
    pub bright_black: String,
    /// ANSI color 9 — bright red.
    pub bright_red: String,
    /// ANSI color 10 — bright green.
    pub bright_green: String,
    /// ANSI color 11 — bright yellow.
    pub bright_yellow: String,
    /// ANSI color 12 — bright blue.
    pub bright_blue: String,
    /// ANSI color 13 — bright magenta.
    pub bright_magenta: String,
    /// ANSI color 14 — bright cyan.
    pub bright_cyan: String,
    /// ANSI color 15 — bright white.
    pub bright_white: String,
}

impl Ansi {
    /// Return the hex color string for the given ANSI key name (e.g. `"red"`, `"bright_black"`).
    ///
    /// Returns `None` for unknown key names.
    pub fn get(&self, key: &str) -> Option<&str> {
        match key {
            "black" => Some(&self.black),
            "red" => Some(&self.red),
            "green" => Some(&self.green),
            "yellow" => Some(&self.yellow),
            "blue" => Some(&self.blue),
            "magenta" => Some(&self.magenta),
            "cyan" => Some(&self.cyan),
            "white" => Some(&self.white),
            "bright_black" => Some(&self.bright_black),
            "bright_red" => Some(&self.bright_red),
            "bright_green" => Some(&self.bright_green),
            "bright_yellow" => Some(&self.bright_yellow),
            "bright_blue" => Some(&self.bright_blue),
            "bright_magenta" => Some(&self.bright_magenta),
            "bright_cyan" => Some(&self.bright_cyan),
            "bright_white" => Some(&self.bright_white),
            _ => None,
        }
    }
}

/// Color palette with named hue ramps (TOML section `[palette]`).
///
/// Each ramp is a 0-indexed `Vec<String>` where values are either
/// `#RRGGBB` hex strings or `ansi.<key>` references.
/// Ramps should be ordered darkest (index 0) to lightest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Palette(HashMap<String, Vec<String>>);

impl Palette {
    /// Create a palette from a map of ramp names to color vectors.
    pub fn new(map: HashMap<String, Vec<String>>) -> Self {
        Self(map)
    }

    /// Return the named ramp, or `None` if it doesn't exist.
    pub fn get_ramp(&self, name: &str) -> Option<&[String]> {
        self.0.get(name).map(|v| &**v)
    }

    /// Return all ramp names in sorted order.
    pub fn ramp_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.0.keys().map(String::as_str).collect();
        names.sort();
        names
    }

    /// Iterate over all `(ramp_name, colors)` pairs in arbitrary order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &Vec<String>)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }
}

/// Base16 color definitions (TOML section `[base16]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Base16(HashMap<String, String>);

impl Base16 {
    /// Create a Base16 map from a raw key→value map.
    pub fn new(map: HashMap<String, String>) -> Self {
        Self(map)
    }

    /// Return the raw color reference for the given Base16 key, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    /// Iterate over all `(key, value)` pairs in arbitrary order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

/// Semantic color roles (TOML section `[semantic]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Semantic {
    /// Color for error states.
    pub error: String,
    /// Color for warning states.
    pub warning: String,
    /// Color for informational states.
    pub info: String,
    /// Color for success states.
    pub success: String,
    /// Color for highlighted text.
    pub highlight: String,
    /// Color for hyperlinks.
    pub link: String,
}

/// Background colors (nested under `[ui.bg]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiBg {
    /// Primary application background.
    pub primary: String,
    /// Secondary / sidebar background.
    pub secondary: String,
}

/// Foreground colors (nested under `[ui.fg]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiFg {
    /// Primary text color.
    pub primary: String,
    /// Secondary text color.
    pub secondary: String,
    /// De-emphasized / placeholder text color.
    pub muted: String,
}

/// Border colors (nested under `[ui.border]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiBorder {
    /// Active / focused border color.
    pub primary: String,
    /// Inactive / de-emphasized border color.
    pub muted: String,
}

/// Cursor colors (nested under `[ui.cursor]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiCursor {
    /// Active cursor color.
    pub primary: String,
    /// Inactive cursor color.
    pub muted: String,
}

/// Selection colors (nested under `[ui.selection]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiSelection {
    /// Selection background.
    pub bg: String,
    /// Selection foreground.
    pub fg: String,
}

/// UI element colors (TOML section `[ui]` with nested sub-tables).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ui {
    /// Background colors (`[ui.bg]`).
    pub bg: UiBg,
    /// Foreground / text colors (`[ui.fg]`).
    pub fg: UiFg,
    /// Border colors (`[ui.border]`).
    pub border: UiBorder,
    /// Cursor colors (`[ui.cursor]`).
    pub cursor: UiCursor,
    /// Selection colors (`[ui.selection]`).
    pub selection: UiSelection,
}

impl Theme {
    /// Resolve a color reference to its `#RRGGBB` hex value.
    ///
    /// Supported reference formats:
    /// - Direct hex: `#ff0000`
    /// - ANSI reference: `ansi.red`, `ansi.bright_black`
    /// - Palette reference: `palette.neutral.0` (0-based index)
    /// - Base16 reference: `base16.base08` (resolved recursively)
    pub fn resolve(&self, color_ref: &str) -> Option<String> {
        if color_ref.starts_with('#') {
            return Some(color_ref.to_string());
        }

        // Split into at most 3 parts on the first two dots.
        let parts: Vec<&str> = color_ref.splitn(3, '.').collect();

        match parts.as_slice() {
            ["palette", ramp, idx_str] => {
                let idx: usize = idx_str.parse().ok()?;
                let value = self.palette.as_ref()?.get_ramp(ramp)?.get(idx)?;
                // Palette values may themselves be ansi.* refs or hex.
                self.resolve(value)
            }
            ["ansi", key] => Some(self.ansi.get(key)?.to_string()),
            ["base16", key] => {
                let value = self.base16.as_ref()?.get(key)?;
                self.resolve(value)
            }
            _ => None,
        }
    }
}

/// Convert a hex color string to RGB components.
///
/// Accepts colors in format `#RRGGBB` or `RRGGBB`.
///
/// # Examples
///
/// ```
/// use tca_types::hex_to_rgb;
///
/// let (r, g, b) = hex_to_rgb("#ff5533").unwrap();
/// assert_eq!((r, g, b), (255, 85, 51));
///
/// ```
///
/// # Errors
///
/// Returns [`HexColorError::InvalidLength`] if the hex string is not exactly
/// 6 characters (excluding a leading `#`), or [`HexColorError::InvalidHex`]
/// if any character is not a valid hex digit.
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), HexColorError> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(HexColorError::InvalidLength(hex.len()));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_theme() -> Theme {
        Theme {
            meta: Meta {
                name: "Test Theme".to_string(),
                slug: Some("test".to_string()),
                author: None,
                version: None,
                description: None,
                dark: Some(true),
            },
            palette: Some(Palette(HashMap::from([
                (
                    "neutral".into(),
                    vec![
                        "#1a1a1a".to_string(),
                        "#666666".to_string(),
                        "#fafafa".to_string(),
                    ],
                ),
                (
                    "red".into(),
                    vec!["#cc0000".to_string(), "ansi.bright_red".to_string()],
                ),
            ]))),
            ansi: Ansi {
                black: "#1a1a1a".to_string(),
                red: "#cc0000".to_string(),
                green: "#00ff00".to_string(),
                yellow: "#ffff00".to_string(),
                blue: "#0000ff".to_string(),
                magenta: "#ff00ff".to_string(),
                cyan: "#00ffff".to_string(),
                white: "#fafafa".to_string(),
                bright_black: "#666666".to_string(),
                bright_red: "#ff5555".to_string(),
                bright_green: "#00ff00".to_string(),
                bright_yellow: "#ffff00".to_string(),
                bright_blue: "#0000ff".to_string(),
                bright_magenta: "#ff00ff".to_string(),
                bright_cyan: "#00ffff".to_string(),
                bright_white: "#ffffff".to_string(),
            },
            base16: None,
            semantic: Semantic {
                error: "palette.red.0".to_string(),
                warning: "ansi.yellow".to_string(),
                info: "ansi.blue".to_string(),
                success: "ansi.green".to_string(),
                highlight: "ansi.cyan".to_string(),
                link: "palette.red.1".to_string(),
            },
            ui: Ui {
                bg: UiBg {
                    primary: "palette.neutral.0".to_string(),
                    secondary: "palette.neutral.1".to_string(),
                },
                fg: UiFg {
                    primary: "palette.neutral.2".to_string(),
                    secondary: "palette.neutral.1".to_string(),
                    muted: "palette.neutral.1".to_string(),
                },
                border: UiBorder {
                    primary: "ansi.blue".to_string(),
                    muted: "palette.neutral.1".to_string(),
                },
                cursor: UiCursor {
                    primary: "ansi.white".to_string(),
                    muted: "palette.neutral.1".to_string(),
                },
                selection: UiSelection {
                    bg: "palette.neutral.1".to_string(),
                    fg: "palette.neutral.2".to_string(),
                },
            },
        }
    }

    #[test]
    fn test_resolve_hex_color() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("#ff0000"), Some("#ff0000".to_string()));
    }

    #[test]
    fn test_resolve_palette_reference() {
        let theme = create_test_theme();
        // palette.red.0 → first element of red ramp
        assert_eq!(theme.resolve("palette.red.0"), Some("#cc0000".to_string()));
        assert_eq!(theme.resolve("palette.red.1"), Some("#ff5555".to_string()));
        assert_eq!(
            theme.resolve("palette.neutral.0"),
            Some("#1a1a1a".to_string())
        );
    }

    #[test]
    fn test_resolve_ansi_reference() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("ansi.red"), Some("#cc0000".to_string()));
        assert_eq!(
            theme.resolve("ansi.bright_black"),
            Some("#666666".to_string())
        );
    }

    #[test]
    fn test_resolve_invalid() {
        let theme = create_test_theme();
        assert_eq!(theme.resolve("$nonexistent.3"), None);
        assert_eq!(theme.resolve("invalid"), None);
        assert_eq!(theme.resolve("palette.red.99"), None); // out of bounds
    }

    #[test]
    fn test_hex_to_rgb_with_hash() {
        let (r, g, b) = hex_to_rgb("#ff5533").unwrap();
        assert_eq!((r, g, b), (255, 85, 51));
    }

    #[test]
    fn test_hex_to_rgb_without_hash() {
        let (r, g, b) = hex_to_rgb("ff5533").unwrap();
        assert_eq!((r, g, b), (255, 85, 51));
    }

    #[test]
    fn test_hex_to_rgb_black() {
        let (r, g, b) = hex_to_rgb("#000000").unwrap();
        assert_eq!((r, g, b), (0, 0, 0));
    }

    #[test]
    fn test_hex_to_rgb_white() {
        let (r, g, b) = hex_to_rgb("#ffffff").unwrap();
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn test_hex_to_rgb_too_short() {
        assert!(hex_to_rgb("#fff").is_err());
        assert!(hex_to_rgb("abc").is_err());
    }

    #[test]
    fn test_hex_to_rgb_too_long() {
        assert!(hex_to_rgb("#ff5533aa").is_err());
    }

    #[test]
    fn test_hex_to_rgb_invalid_chars() {
        assert!(hex_to_rgb("#gggggg").is_err());
        assert!(hex_to_rgb("#xyz123").is_err());
    }

    #[test]
    fn test_hex_to_rgb_empty() {
        assert!(hex_to_rgb("").is_err());
        assert!(hex_to_rgb("#").is_err());
    }
}
