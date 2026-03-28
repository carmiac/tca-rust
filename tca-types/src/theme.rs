//! Core types for Terminal Colors Architecture (TCA)
//!
//! This crate provides the foundational type definitions used across
//! the TCA ecosystem for theme representation and manipulation.

#![warn(missing_docs)]
#[cfg(feature = "fs")]
use anyhow::Result;
use serde::Serialize;
#[cfg(feature = "fs")]
use std::path::PathBuf;

use crate::base24::{is_dark, normalize_hex, parse_base24};
#[cfg(feature = "fs")]
use crate::user_themes_path;

/// Errors that can occur when parsing a hex color string.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HexColorError {
    /// The hex string (excluding a leading `#`) was not exactly 6 characters.
    #[error("hex color must be 6 characters (got {0})")]
    InvalidLength(usize),
    /// A hex digit could not be parsed.
    #[error("invalid hex digit: {0}")]
    InvalidHex(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for HexColorError {
    fn from(e: std::num::ParseIntError) -> Self {
        HexColorError::InvalidHex(e)
    }
}

/// The raw base24 color slots (base00–base17) as `#rrggbb` hex strings.
///
/// These are the canonical values from which all TCA semantic fields are derived.
/// base16-only themes leave base10–base17 set to their spec-defined fallbacks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Base24Slots {
    /// Darkest background / ANSI black background.
    pub base00: String,
    /// Secondary background.
    pub base01: String,
    /// Selection / border background.
    pub base02: String,
    /// Comments / bright black (ANSI 8).
    pub base03: String,
    /// Muted / dark foreground.
    pub base04: String,
    /// Primary foreground / white (ANSI 7).
    pub base05: String,
    /// Secondary foreground.
    pub base06: String,
    /// Bright white (ANSI 15).
    pub base07: String,
    /// Red / error (ANSI 1).
    pub base08: String,
    /// Orange / warning.
    pub base09: String,
    /// Yellow (ANSI 3).
    pub base0a: String,
    /// Green / success (ANSI 2).
    pub base0b: String,
    /// Cyan / info (ANSI 6).
    pub base0c: String,
    /// Blue / link (ANSI 4).
    pub base0d: String,
    /// Magenta / highlight (ANSI 5).
    pub base0e: String,
    /// Brown / deprecated.
    pub base0f: String,
    /// True black (ANSI 0 terminal color). Fallback: base00.
    pub base10: String,
    /// Reserved. Fallback: base00.
    pub base11: String,
    /// Bright red (ANSI 9). Fallback: base08.
    pub base12: String,
    /// Bright yellow (ANSI 11). Fallback: base0a.
    pub base13: String,
    /// Bright green (ANSI 10). Fallback: base0b.
    pub base14: String,
    /// Bright cyan (ANSI 14). Fallback: base0c.
    pub base15: String,
    /// Bright blue (ANSI 12). Fallback: base0d.
    pub base16: String,
    /// Bright magenta (ANSI 13). Fallback: base0e.
    pub base17: String,
}

/// A complete TCA theme definition.
///
/// All color fields contain resolved `#rrggbb` hex strings.
#[derive(Debug, Clone, Serialize)]
pub struct Theme {
    /// Theme metadata.
    pub meta: Meta,
    /// ANSI 16-color definitions derived from base24 slots.
    pub ansi: Ansi,
    /// Semantic color roles derived from base24 slots.
    pub semantic: Semantic,
    /// UI element colors derived from base24 slots.
    pub ui: Ui,
    /// Raw base24 color slots for direct interoperability.
    pub base24: Base24Slots,
}

/// Theme metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Meta {
    /// Human-readable theme name.
    pub name: String,
    /// Theme author name or contact. Empty string if not specified.
    pub author: String,
    /// `true` for dark themes, `false` for light themes.
    ///
    /// Derived from the luminance of `bg.primary` (base00 vs base07).
    pub dark: bool,
}

/// ANSI 16-color definitions, derived from base24 slots per the TCA spec.
///
/// All values are resolved `#rrggbb` hex strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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


/// Semantic color roles (TOML section `[semantic]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UiBg {
    /// Primary application background.
    pub primary: String,
    /// Secondary / sidebar background.
    pub secondary: String,
}

/// Foreground colors (nested under `[ui.fg]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UiFg {
    /// Primary text color.
    pub primary: String,
    /// Secondary text color.
    pub secondary: String,
    /// De-emphasized / placeholder text color.
    pub muted: String,
}

/// Border colors (nested under `[ui.border]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UiBorder {
    /// Active / focused border color.
    pub primary: String,
    /// Inactive / de-emphasized border color.
    pub muted: String,
}

/// Cursor colors (nested under `[ui.cursor]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UiCursor {
    /// Active cursor color.
    pub primary: String,
    /// Inactive cursor color.
    pub muted: String,
}

/// Selection colors (nested under `[ui.selection]`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UiSelection {
    /// Selection background.
    pub bg: String,
    /// Selection foreground.
    pub fg: String,
}

/// UI element colors (TOML section `[ui]` with nested sub-tables).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
    /// Construct a `Theme` by deriving all fields from a set of base24 slots.
    pub fn from_base24_slots(slots: Base24Slots) -> Self {
        let dark = is_dark(&slots.base00, &slots.base07);
        let name = String::new(); // caller must set via from_raw_slots
        Theme {
            meta: Meta {
                name,
                author: String::new(),
                dark,
            },
            ansi: Ansi {
                black: slots.base00.clone(),
                red: slots.base08.clone(),
                green: slots.base0b.clone(),
                yellow: slots.base0a.clone(),
                blue: slots.base0d.clone(),
                magenta: slots.base0e.clone(),
                cyan: slots.base0c.clone(),
                white: slots.base05.clone(),
                bright_black: slots.base03.clone(),
                bright_red: slots.base12.clone(),
                bright_green: slots.base14.clone(),
                bright_yellow: slots.base13.clone(),
                bright_blue: slots.base16.clone(),
                bright_magenta: slots.base17.clone(),
                bright_cyan: slots.base15.clone(),
                bright_white: slots.base07.clone(),
            },
            semantic: Semantic {
                error: slots.base08.clone(),
                warning: slots.base09.clone(),
                info: slots.base0c.clone(),
                success: slots.base0b.clone(),
                highlight: slots.base0e.clone(),
                link: slots.base0d.clone(),
            },
            ui: Ui {
                fg: UiFg {
                    primary: slots.base05.clone(),
                    secondary: slots.base06.clone(),
                    muted: slots.base04.clone(),
                },
                bg: UiBg {
                    primary: slots.base00.clone(),
                    secondary: slots.base01.clone(),
                },
                border: UiBorder {
                    primary: slots.base02.clone(),
                    muted: slots.base01.clone(),
                },
                cursor: UiCursor {
                    primary: slots.base05.clone(),
                    muted: slots.base04.clone(),
                },
                selection: UiSelection {
                    bg: slots.base02.clone(),
                    fg: slots.base05.clone(),
                },
            },
            base24: slots,
        }
    }

    /// Parse a base24 YAML string and construct a `Theme`.
    ///
    /// Accepts the flat `key: "value"` YAML subset used by Base16/24 scheme files.
    /// Theme name is read from the `scheme` key (with `name` as a fallback).
    pub fn from_base24_str(src: &str) -> anyhow::Result<Self> {
        let slots = parse_base24(src.as_bytes())?;
        Self::from_raw_slots(&slots)
    }

    /// Build a `Theme` from a parsed [`RawSlots`] map (e.g. from [`parse_base24`]).
    pub fn from_raw_slots(
        slots: &std::collections::HashMap<String, String>,
    ) -> anyhow::Result<Self> {
        use anyhow::anyhow;

        let b = |key: &str| -> anyhow::Result<String> {
            let raw = slots
                .get(key)
                .ok_or_else(|| anyhow!("missing required slot '{key}'"))?;
            normalize_hex(raw).map_err(|e| anyhow!("slot '{key}': {e}"))
        };

        let is_base24 = slots.contains_key("base10");

        let base00 = b("base00")?;
        let base01 = b("base01")?;
        let base02 = b("base02")?;
        let base03 = b("base03")?;
        let base04 = b("base04")?;
        let base05 = b("base05")?;
        let base06 = b("base06")?;
        let base07 = b("base07")?;
        let base08 = b("base08")?;
        let base09 = b("base09")?;
        let base0a = b("base0a")?;
        let base0b = b("base0b")?;
        let base0c = b("base0c")?;
        let base0d = b("base0d")?;
        let base0e = b("base0e")?;
        let base0f = b("base0f")?;

        // Base24 extended slots — fall back to spec-defined base16 equivalents.
        let base10 = if is_base24 {
            b("base10")?
        } else {
            base00.clone()
        };
        let base11 = if is_base24 {
            b("base11")?
        } else {
            base00.clone()
        };
        let base12 = if is_base24 {
            b("base12")?
        } else {
            base08.clone()
        };
        let base13 = if is_base24 {
            b("base13")?
        } else {
            base0a.clone()
        };
        let base14 = if is_base24 {
            b("base14")?
        } else {
            base0b.clone()
        };
        let base15 = if is_base24 {
            b("base15")?
        } else {
            base0c.clone()
        };
        let base16 = if is_base24 {
            b("base16")?
        } else {
            base0d.clone()
        };
        let base17 = if is_base24 {
            b("base17")?
        } else {
            base0e.clone()
        };

        let name = slots
            .get("scheme")
            .or_else(|| slots.get("name"))
            .cloned()
            .unwrap_or_else(|| "Imported Theme".to_string());
        let author = slots.get("author").cloned().unwrap_or_default();

        let raw = Base24Slots {
            base00,
            base01,
            base02,
            base03,
            base04,
            base05,
            base06,
            base07,
            base08,
            base09,
            base0a,
            base0b,
            base0c,
            base0d,
            base0e,
            base0f,
            base10,
            base11,
            base12,
            base13,
            base14,
            base15,
            base16,
            base17,
        };

        let mut theme = Self::from_base24_slots(raw);
        theme.meta.name = name;
        theme.meta.author = author;
        Ok(theme)
    }

    /// Creates a `Theme` from an optional name, searching in order:
    ///
    /// 1. User theme files (if `fs` feature enabled).
    /// 2. Built-in themes.
    /// 3. User config default (if `fs` feature enabled).
    /// 4. Built-in dark/light default based on terminal mode.
    ///
    /// The name is case-insensitive and accepts any common case format
    /// (`"Nord Dark"`, `"nord-dark"`, `"NordDark"` all work).
    pub fn from_name(name: Option<&str>) -> Theme {
        use crate::BuiltinTheme;
        #[cfg(feature = "fs")]
        {
            use crate::util::load_theme_file;
            use terminal_colorsaurus::{theme_mode, QueryOptions, ThemeMode};
            name.and_then(|n| load_theme_file(n).ok())
                .as_deref()
                .and_then(|s| Theme::from_base24_str(s).ok())
                .or_else(|| {
                    name.and_then(|n| {
                        let slug = heck::AsKebabCase(n).to_string();
                        slug.parse::<BuiltinTheme>().ok().map(|b| b.theme())
                    })
                })
                .or_else(|| {
                    crate::util::mode_aware_theme_name()
                        .and_then(|n| n.parse::<BuiltinTheme>().ok().map(|b| b.theme()))
                })
                .unwrap_or_else(|| match theme_mode(QueryOptions::default()).ok() {
                    Some(ThemeMode::Light) => BuiltinTheme::default_light().theme(),
                    _ => BuiltinTheme::default().theme(),
                })
        }
        #[cfg(not(feature = "fs"))]
        {
            name.and_then(|n| n.parse::<BuiltinTheme>().ok().map(|b| b.theme()))
                .unwrap_or_else(|| BuiltinTheme::default().theme())
        }
    }

    /// Returns the kebab-case slug for the theme name.
    ///
    /// e.g. `"Tokyo Night"` -> `"tokyo-night"`
    pub fn name_slug(&self) -> String {
        heck::AsKebabCase(&self.meta.name).to_string()
    }

    /// Returns the canonical file name for the theme.
    ///
    /// e.g. `"Tokyo Night"` -> `"tokyo-night.yaml"`
    pub fn to_filename(&self) -> String {
        let slug = self.name_slug();
        if slug.ends_with(".yaml") {
            slug
        } else {
            format!("{}.yaml", slug)
        }
    }

    /// Returns the canonical install path for the theme in the user themes directory.
    #[cfg(feature = "fs")]
    pub fn to_pathbuf(&self) -> Result<PathBuf> {
        let mut path = user_themes_path()?;
        path.push(self.to_filename());
        Ok(path)
    }

    /// Serialize this theme to a base24 YAML string.
    ///
    /// The output is a flat `key: "value"` YAML file compatible with the
    /// [Tinted Theming base24](https://github.com/tinted-theming/base24/) format.
    /// Hex values are written as 6-character lowercase strings without a leading `#`.
    pub fn to_base24_str(&self) -> String {
        let h = |s: &str| s.trim_start_matches('#').to_lowercase();
        let variant = if self.meta.dark { "dark" } else { "light" };
        let author = &self.meta.author;
        let s = &self.base24;
        format!(
            "scheme: \"{name}\"\nauthor: \"{author}\"\nvariant: \"{variant}\"\n\
             base00: \"{b00}\"\nbase01: \"{b01}\"\nbase02: \"{b02}\"\nbase03: \"{b03}\"\n\
             base04: \"{b04}\"\nbase05: \"{b05}\"\nbase06: \"{b06}\"\nbase07: \"{b07}\"\n\
             base08: \"{b08}\"\nbase09: \"{b09}\"\nbase0A: \"{b0a}\"\nbase0B: \"{b0b}\"\n\
             base0C: \"{b0c}\"\nbase0D: \"{b0d}\"\nbase0E: \"{b0e}\"\nbase0F: \"{b0f}\"\n\
             base10: \"{b10}\"\nbase11: \"{b11}\"\nbase12: \"{b12}\"\nbase13: \"{b13}\"\n\
             base14: \"{b14}\"\nbase15: \"{b15}\"\nbase16: \"{b16}\"\nbase17: \"{b17}\"\n",
            name = self.meta.name,
            b00 = h(&s.base00),
            b01 = h(&s.base01),
            b02 = h(&s.base02),
            b03 = h(&s.base03),
            b04 = h(&s.base04),
            b05 = h(&s.base05),
            b06 = h(&s.base06),
            b07 = h(&s.base07),
            b08 = h(&s.base08),
            b09 = h(&s.base09),
            b0a = h(&s.base0a),
            b0b = h(&s.base0b),
            b0c = h(&s.base0c),
            b0d = h(&s.base0d),
            b0e = h(&s.base0e),
            b0f = h(&s.base0f),
            b10 = h(&s.base10),
            b11 = h(&s.base11),
            b12 = h(&s.base12),
            b13 = h(&s.base13),
            b14 = h(&s.base14),
            b15 = h(&s.base15),
            b16 = h(&s.base16),
            b17 = h(&s.base17),
        )
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name_slug() == other.name_slug()
    }
}
impl Eq for Theme {}

impl PartialOrd for Theme {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Theme {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name_slug().cmp(&other.name_slug())
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

    const MINIMAL_BASE24: &str = r#"
scheme: "Test Theme"
author: "Test Author"
base00: "1a1a1a"
base01: "222222"
base02: "333333"
base03: "666666"
base04: "888888"
base05: "fafafa"
base06: "e0e0e0"
base07: "ffffff"
base08: "cc0000"
base09: "ff8800"
base0a: "ffff00"
base0b: "00ff00"
base0c: "00ffff"
base0d: "0000ff"
base0e: "ff00ff"
base0f: "aa5500"
base10: "1a1a1a"
base11: "000000"
base12: "ff5555"
base13: "ffff55"
base14: "55ff55"
base15: "55ffff"
base16: "5555ff"
base17: "ff55ff"
"#;

    fn test_theme() -> Theme {
        Theme::from_base24_str(MINIMAL_BASE24).unwrap()
    }

    #[test]
    fn test_from_base24_str_name_and_author() {
        let t = test_theme();
        assert_eq!(t.meta.name, "Test Theme");
        assert_eq!(t.meta.author, "Test Author");
    }

    #[test]
    fn test_from_base24_str_dark_detection() {
        let t = test_theme();
        assert!(t.meta.dark, "dark bg should be detected as dark theme");
    }

    #[test]
    fn test_ansi_mapping() {
        let t = test_theme();
        // Spec: black = base00, red = base08, white = base05
        assert_eq!(t.ansi.black, "#1a1a1a");
        assert_eq!(t.ansi.red, "#cc0000");
        assert_eq!(t.ansi.white, "#fafafa");
        assert_eq!(t.ansi.bright_black, "#666666"); // base03
        assert_eq!(t.ansi.bright_red, "#ff5555"); // base12
    }

    #[test]
    fn test_semantic_mapping() {
        let t = test_theme();
        // Spec: error=base08, warning=base09, info=base0c, success=base0b
        assert_eq!(t.semantic.error, "#cc0000"); // base08
        assert_eq!(t.semantic.warning, "#ff8800"); // base09
        assert_eq!(t.semantic.info, "#00ffff"); // base0c
        assert_eq!(t.semantic.success, "#00ff00"); // base0b
        assert_eq!(t.semantic.link, "#0000ff"); // base0d
    }

    #[test]
    fn test_ui_mapping() {
        let t = test_theme();
        assert_eq!(t.ui.bg.primary, "#1a1a1a"); // base00
        assert_eq!(t.ui.bg.secondary, "#222222"); // base01
        assert_eq!(t.ui.fg.primary, "#fafafa"); // base05
        assert_eq!(t.ui.fg.secondary, "#e0e0e0"); // base06
        assert_eq!(t.ui.fg.muted, "#888888"); // base04
        assert_eq!(t.ui.border.primary, "#333333"); // base02
        assert_eq!(t.ui.border.muted, "#222222"); // base01
        assert_eq!(t.ui.cursor.primary, "#fafafa"); // base05
        assert_eq!(t.ui.cursor.muted, "#888888"); // base04
        assert_eq!(t.ui.selection.bg, "#333333"); // base02
        assert_eq!(t.ui.selection.fg, "#fafafa"); // base05
    }

    #[test]
    fn test_base16_fallbacks() {
        // A base16-only theme (no base10-17) should use fallbacks
        let src = MINIMAL_BASE24
            .lines()
            .filter(|l| !l.starts_with("base1"))
            .collect::<Vec<_>>()
            .join("\n");
        let t = Theme::from_base24_str(&src).unwrap();
        // bright_red (base12) should fall back to red (base08)
        assert_eq!(t.ansi.bright_red, t.ansi.red);
    }

    #[test]
    fn test_name_slug() {
        let t = test_theme();
        assert_eq!(t.name_slug(), "test-theme");
    }

    #[test]
    fn test_to_filename() {
        let t = test_theme();
        assert_eq!(t.to_filename(), "test-theme.yaml");
    }

    #[test]
    fn test_to_base24_str_round_trip() {
        let original = test_theme();
        let yaml = original.to_base24_str();
        let reloaded = Theme::from_base24_str(&yaml).unwrap();
        assert_eq!(reloaded.meta.name, original.meta.name);
        assert_eq!(reloaded.meta.dark, original.meta.dark);
        assert_eq!(reloaded.base24.base08, original.base24.base08);
        assert_eq!(reloaded.ansi.red, original.ansi.red);
        assert_eq!(reloaded.semantic.error, original.semantic.error);
    }

    #[test]
    fn test_to_base24_str_format() {
        let t = test_theme();
        let yaml = t.to_base24_str();
        // Should be valid flat key:value YAML parseable by our parser
        assert!(yaml.contains("scheme: \"Test Theme\""));
        assert!(yaml.contains("base00:"));
        assert!(yaml.contains("base17:"));
        // Hex values should be without '#'
        assert!(!yaml.contains(": \"#"));
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
