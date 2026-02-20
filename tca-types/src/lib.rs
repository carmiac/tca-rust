//! Core types for Terminal Colors Architecture (TCA)
//!
//! This crate provides the foundational type definitions used across
//! the TCA ecosystem for theme representation and manipulation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete TCA theme definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme {
    /// Serde key is `theme` to match the TOML section name.
    #[serde(rename = "theme")]
    pub meta: Meta,
    pub ansi: Ansi,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub palette: Option<Palette>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base16: Option<Base16>,
    pub semantic: Semantic,
    pub ui: Ui,
}

/// Theme metadata (TOML section `[theme]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<bool>,
}

/// ANSI 16-color definitions (TOML section `[ansi]`).
///
/// All values must be direct `#RRGGBB` hex strings — no palette references.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ansi {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

/// Color palette with named hue ramps (TOML section `[palette]`).
///
/// Each ramp is a 0-indexed `Vec<String>` where values are either
/// `#RRGGBB` hex strings or `ansi.<key>` references.
/// Ramps should be ordered darkest (index 0) to lightest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Palette(pub HashMap<String, Vec<String>>);

/// Base16 color definitions (TOML section `[base16]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Base16(pub HashMap<String, String>);

/// Semantic color roles (TOML section `[semantic]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Semantic {
    pub error: String,
    pub warning: String,
    pub info: String,
    pub success: String,
    pub highlight: String,
    pub link: String,
}

/// Background colors (nested under `[ui.bg]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiBg {
    pub primary: String,
    pub secondary: String,
}

/// Foreground colors (nested under `[ui.fg]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiFg {
    pub primary: String,
    pub secondary: String,
    pub muted: String,
}

/// Border colors (nested under `[ui.border]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiBorder {
    pub primary: String,
    pub muted: String,
}

/// Cursor colors (nested under `[ui.cursor]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiCursor {
    pub primary: String,
    pub muted: String,
}

/// Selection colors (nested under `[ui.selection]`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UiSelection {
    pub bg: String,
    pub fg: String,
}

/// UI element colors (TOML section `[ui]` with nested sub-tables).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ui {
    pub bg: UiBg,
    pub fg: UiFg,
    pub border: UiBorder,
    pub cursor: UiCursor,
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
                let value = self.palette.as_ref()?.0.get(*ramp)?.get(idx)?;
                // Palette values may themselves be ansi.* refs or hex.
                self.resolve(value)
            }
            ["ansi", key] => {
                let hex = match *key {
                    "black" => &self.ansi.black,
                    "red" => &self.ansi.red,
                    "green" => &self.ansi.green,
                    "yellow" => &self.ansi.yellow,
                    "blue" => &self.ansi.blue,
                    "magenta" => &self.ansi.magenta,
                    "cyan" => &self.ansi.cyan,
                    "white" => &self.ansi.white,
                    "bright_black" => &self.ansi.bright_black,
                    "bright_red" => &self.ansi.bright_red,
                    "bright_green" => &self.ansi.bright_green,
                    "bright_yellow" => &self.ansi.bright_yellow,
                    "bright_blue" => &self.ansi.bright_blue,
                    "bright_magenta" => &self.ansi.bright_magenta,
                    "bright_cyan" => &self.ansi.bright_cyan,
                    "bright_white" => &self.ansi.bright_white,
                    _ => return None,
                };
                Some(hex.clone())
            }
            ["base16", key] => {
                let value = self.base16.as_ref()?.0.get(*key)?;
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
/// let (r, g, b) = hex_to_rgb("ff5533").unwrap();
/// assert_eq!((r, g, b), (255, 85, 51));
/// ```
///
/// # Errors
///
/// Returns error if hex string is not exactly 6 characters (excluding `#`)
/// or contains non-hexadecimal characters.
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        // Return a ParseIntError by trying to parse invalid input
        return u8::from_str_radix("", 16).map(|_| (0, 0, 0));
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
