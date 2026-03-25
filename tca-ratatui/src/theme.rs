#[cfg(feature = "fs")]
use anyhow::{Context, Result};
use ratatui::style::Color;
use std::collections::HashMap;

#[cfg(feature = "fs")]
use tca_types::BuiltinTheme;

/// Theme metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    /// Human-readable theme name.
    pub name: String,
    /// Theme author name or contact.
    pub author: Option<String>,
    /// Semantic version string (e.g. `"1.0.0"`).
    pub version: Option<String>,
    /// Short description of the theme.
    pub description: Option<String>,
    /// `true` for dark themes, `false` for light themes.
    pub dark: Option<bool>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "Unnamed Theme".to_string(),
            author: None,
            version: None,
            description: None,
            dark: None,
        }
    }
}

/// ANSI 16-color definitions mapped to Ratatui colors.
///
/// All colors are resolved to concrete [`Color::Rgb`] values when loaded
/// from a theme file. The [`Default`] impl maps to Ratatui's named colors
/// as a fallback when no theme file is present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ansi {
    /// ANSI color 0 — black.
    pub black: Color,
    /// ANSI color 1 — red.
    pub red: Color,
    /// ANSI color 2 — green.
    pub green: Color,
    /// ANSI color 3 — yellow.
    pub yellow: Color,
    /// ANSI color 4 — blue.
    pub blue: Color,
    /// ANSI color 5 — magenta.
    pub magenta: Color,
    /// ANSI color 6 — cyan.
    pub cyan: Color,
    /// ANSI color 7 — white.
    pub white: Color,
    /// ANSI color 8 — bright black (dark gray).
    pub bright_black: Color,
    /// ANSI color 9 — bright red.
    pub bright_red: Color,
    /// ANSI color 10 — bright green.
    pub bright_green: Color,
    /// ANSI color 11 — bright yellow.
    pub bright_yellow: Color,
    /// ANSI color 12 — bright blue.
    pub bright_blue: Color,
    /// ANSI color 13 — bright magenta.
    pub bright_magenta: Color,
    /// ANSI color 14 — bright cyan.
    pub bright_cyan: Color,
    /// ANSI color 15 — bright white.
    pub bright_white: Color,
}

impl Ansi {
    /// Return the resolved color for the given ANSI key name (e.g. `"red"`, `"bright_black"`).
    ///
    /// Returns `None` for unknown key names.
    pub fn get(&self, key: &str) -> Option<Color> {
        match key {
            "black" => Some(self.black),
            "red" => Some(self.red),
            "green" => Some(self.green),
            "yellow" => Some(self.yellow),
            "blue" => Some(self.blue),
            "magenta" => Some(self.magenta),
            "cyan" => Some(self.cyan),
            "white" => Some(self.white),
            "bright_black" => Some(self.bright_black),
            "bright_red" => Some(self.bright_red),
            "bright_green" => Some(self.bright_green),
            "bright_yellow" => Some(self.bright_yellow),
            "bright_blue" => Some(self.bright_blue),
            "bright_magenta" => Some(self.bright_magenta),
            "bright_cyan" => Some(self.bright_cyan),
            "bright_white" => Some(self.bright_white),
            _ => None,
        }
    }
}

impl Default for Ansi {
    fn default() -> Self {
        Self {
            black: Color::Black,
            red: Color::Red,
            green: Color::Green,
            yellow: Color::Yellow,
            blue: Color::Blue,
            magenta: Color::Magenta,
            cyan: Color::Cyan,
            white: Color::Gray,
            bright_black: Color::DarkGray,
            bright_red: Color::LightRed,
            bright_green: Color::LightGreen,
            bright_yellow: Color::LightYellow,
            bright_blue: Color::LightBlue,
            bright_magenta: Color::LightMagenta,
            bright_cyan: Color::LightCyan,
            bright_white: Color::White,
        }
    }
}

/// A named color ramp, 0-indexed from darkest (index 0) to lightest.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorRamp {
    /// The resolved colors in this ramp, ordered darkest (index 0) to lightest.
    pub colors: Vec<Color>,
}

impl ColorRamp {
    /// Returns the color at the given 0-based index, or `None` if out of range.
    pub fn get(&self, idx: usize) -> Option<Color> {
        self.colors.get(idx).copied()
    }

    /// Returns the number of colors in the ramp.
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Returns `true` if the ramp contains no colors.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
}

/// Color palette of named hue ramps (`[palette]` section).
///
/// All ramps are 0-indexed and ordered darkest → lightest.
/// An absent `[palette]` section deserializes to an empty palette.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Palette(pub(crate) HashMap<String, ColorRamp>);

impl Palette {
    /// Returns the named ramp, or `None` if it doesn't exist.
    pub fn get_ramp(&self, name: &str) -> Option<&ColorRamp> {
        self.0.get(name)
    }

    /// Returns all ramp names in sorted order.
    pub fn ramp_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.0.keys().map(String::as_str).collect();
        names.sort();
        names
    }
}

/// Base16 color mappings (`base00`–`base0F`).
///
/// An absent `[base16]` section deserializes to an empty map.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Base16(pub(crate) HashMap<String, Color>);

impl Base16 {
    /// Returns the resolved color for the given Base16 key, or `None`.
    pub fn get(&self, key: &str) -> Option<Color> {
        self.0.get(key).copied()
    }

    /// Iterates over all `(key, color)` pairs in sorted key order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, Color)> {
        let mut pairs: Vec<(&str, Color)> = self.0.iter().map(|(k, &v)| (k.as_str(), v)).collect();
        pairs.sort_by_key(|(k, _)| *k);
        pairs.into_iter()
    }
}

/// Semantic color roles.
///
/// The [`Default`] impl maps to Ratatui's named colors as a fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semantic {
    /// Color for error states.
    pub error: Color,
    /// Color for warning states.
    pub warning: Color,
    /// Color for informational states.
    pub info: Color,
    /// Color for success states.
    pub success: Color,
    /// Color for highlighted text.
    pub highlight: Color,
    /// Color for hyperlinks.
    pub link: Color,
}

impl Default for Semantic {
    fn default() -> Self {
        Self {
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
            success: Color::Green,
            highlight: Color::Cyan,
            link: Color::Blue,
        }
    }
}

/// UI element colors for application chrome.
///
/// Field names flatten the nested TOML sub-tables (`[ui.bg]`, `[ui.fg]`, etc.)
/// into a single struct for convenient access.
///
/// The [`Default`] impl provides a minimal dark-theme fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ui {
    /// Primary application background.
    pub bg_primary: Color,
    /// Secondary / sidebar background.
    pub bg_secondary: Color,
    /// Primary text color.
    pub fg_primary: Color,
    /// Secondary text color.
    pub fg_secondary: Color,
    /// De-emphasized / placeholder text color.
    pub fg_muted: Color,
    /// Active / focused border color.
    pub border_primary: Color,
    /// Inactive / de-emphasized border color.
    pub border_muted: Color,
    /// Active cursor color.
    pub cursor_primary: Color,
    /// Inactive cursor color.
    pub cursor_muted: Color,
    /// Selection background.
    pub selection_bg: Color,
    /// Selection foreground.
    pub selection_fg: Color,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            bg_primary: Color::Black,
            bg_secondary: Color::Black,
            fg_primary: Color::White,
            fg_secondary: Color::Gray,
            fg_muted: Color::DarkGray,
            border_primary: Color::White,
            border_muted: Color::DarkGray,
            cursor_primary: Color::White,
            cursor_muted: Color::Gray,
            selection_bg: Color::DarkGray,
            selection_fg: Color::White,
        }
    }
}

/// A fully resolved TCA theme with Ratatui-compatible colors.
///
/// All color references have been resolved to concrete [`Color`] values.
/// Construct via [`TcaThemeBuilder`] or the `from_file`/`from_toml` methods.
#[derive(Debug, Clone)]
pub struct TcaTheme {
    /// Theme metadata (name, author, version, etc.).
    pub meta: Meta,
    /// Resolved ANSI 16-color definitions.
    pub ansi: Ansi,
    /// Resolved palette ramps. Empty if the theme has no `[palette]` section.
    pub palette: Palette,
    /// Resolved Base16 mappings. Empty if the theme has no `[base16]` section.
    pub base16: Base16,
    /// Resolved semantic color roles.
    pub semantic: Semantic,
    /// Resolved UI element colors.
    pub ui: Ui,
}

impl TcaTheme {
    /// Creates a new theme, trying to match the passed name to a known
    /// theme name or path and a reasonable default otherwise. The name is internally
    /// converted, so e.g. any of the following would get the same theme:
    /// NordDark
    /// Nord Dark
    /// Nord-Dark
    /// nord-dark
    /// nordDark
    /// etc...
    ///
    /// The search fallback order is:
    /// 1. Local theme files.
    /// 2. Built in themes.
    /// 3. Built in default light/dark mode theme based on current terminal mode.
    #[cfg(feature = "fs")]
    pub fn new(name: Option<&str>) -> Self {
        TcaTheme::try_from(tca_types::Theme::from_name(name)).unwrap_or_else(|_| {
            use terminal_colorsaurus::{theme_mode, QueryOptions, ThemeMode};
            let builtin = match theme_mode(QueryOptions::default()).ok() {
                Some(ThemeMode::Light) => BuiltinTheme::default_light(),
                _ => BuiltinTheme::default(),
            };
            TcaTheme::try_from(builtin.theme()).expect("hardcoded default must be valid")
        })
    }
}

#[cfg(feature = "fs")]
impl Default for TcaTheme {
    fn default() -> Self {
        TcaTheme::new(None)
    }
}

#[cfg(feature = "fs")]
/// Load a TcaTheme from a TOML string.
impl TryFrom<&str> for TcaTheme {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<TcaTheme, Self::Error> {
        let raw: tca_types::Theme = toml::from_str(value)?;
        TcaTheme::try_from(raw)
    }
}

/// Converts a raw [`tca_types::Theme`] into a fully resolved [`TcaTheme`].
///
/// ANSI hex values are parsed strictly — malformed hex returns an error.
/// Semantic and UI color references that cannot be resolved fall back to
/// the named-color defaults from [`Semantic::default`] and [`Ui::default`].
#[cfg(feature = "fs")]
impl TryFrom<tca_types::Theme> for TcaTheme {
    type Error = anyhow::Error;
    fn try_from(raw: tca_types::Theme) -> Result<TcaTheme, Self::Error> {
        // ANSI is required and hex-only; hard error on bad hex.
        let ansi = parse_ansi(&raw.ansi)?;

        // Palette and Base16 are optional; absent sections -> empty defaults.
        let palette = parse_palette(raw.palette.as_ref(), &raw.ansi);
        let base16 = parse_base16(raw.base16.as_ref(), &raw.ansi, &palette);

        let resolve = |r: &str| resolve_ref(r, &ansi, &palette, &base16);

        let defaults = Semantic::default();
        let semantic = Semantic {
            error: resolve(&raw.semantic.error).unwrap_or(defaults.error),
            warning: resolve(&raw.semantic.warning).unwrap_or(defaults.warning),
            info: resolve(&raw.semantic.info).unwrap_or(defaults.info),
            success: resolve(&raw.semantic.success).unwrap_or(defaults.success),
            highlight: resolve(&raw.semantic.highlight).unwrap_or(defaults.highlight),
            link: resolve(&raw.semantic.link).unwrap_or(defaults.link),
        };

        let defaults = Ui::default();
        let ui = Ui {
            bg_primary: resolve(&raw.ui.bg.primary).unwrap_or(defaults.bg_primary),
            bg_secondary: resolve(&raw.ui.bg.secondary).unwrap_or(defaults.bg_secondary),
            fg_primary: resolve(&raw.ui.fg.primary).unwrap_or(defaults.fg_primary),
            fg_secondary: resolve(&raw.ui.fg.secondary).unwrap_or(defaults.fg_secondary),
            fg_muted: resolve(&raw.ui.fg.muted).unwrap_or(defaults.fg_muted),
            border_primary: resolve(&raw.ui.border.primary).unwrap_or(defaults.border_primary),
            border_muted: resolve(&raw.ui.border.muted).unwrap_or(defaults.border_muted),
            cursor_primary: resolve(&raw.ui.cursor.primary).unwrap_or(defaults.cursor_primary),
            cursor_muted: resolve(&raw.ui.cursor.muted).unwrap_or(defaults.cursor_muted),
            selection_bg: resolve(&raw.ui.selection.bg).unwrap_or(defaults.selection_bg),
            selection_fg: resolve(&raw.ui.selection.fg).unwrap_or(defaults.selection_fg),
        };

        let meta = Meta {
            name: raw.meta.name,
            author: raw.meta.author,
            version: raw.meta.version,
            description: raw.meta.description,
            dark: raw.meta.dark,
        };

        Ok(TcaTheme {
            meta,
            ansi,
            palette,
            base16,
            semantic,
            ui,
        })
    }
}

impl PartialEq for TcaTheme {
    fn eq(&self, other: &Self) -> bool {
        self.meta.name == other.meta.name
    }
}
impl Eq for TcaTheme {}

impl PartialOrd for TcaTheme {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TcaTheme {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.meta.name.cmp(&(other.meta.name))
    }
}

/// Builder for constructing a [`TcaTheme`] programmatically.
///
/// All sections default to sensible fallback values so you only need to
/// supply what differs from the defaults.
///
/// # Example
///
/// ```rust
/// use tca_ratatui::{TcaThemeBuilder, Semantic};
/// use ratatui::style::Color;
///
/// let theme = TcaThemeBuilder::new()
///     .semantic(Semantic {
///         error: Color::Rgb(255, 80, 80),
///         ..Default::default()
///     })
///     .build();
///
/// assert_eq!(theme.semantic.error, Color::Rgb(255, 80, 80));
/// assert_eq!(theme.semantic.warning, Color::Yellow); // default
/// ```
#[derive(Debug, Clone, Default)]
pub struct TcaThemeBuilder {
    meta: Meta,
    ansi: Ansi,
    palette: Palette,
    base16: Base16,
    semantic: Semantic,
    ui: Ui,
}

impl TcaThemeBuilder {
    /// Create a new builder with all sections set to their defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the theme metadata.
    pub fn meta(mut self, meta: Meta) -> Self {
        self.meta = meta;
        self
    }

    /// Set the ANSI 16-color definitions.
    pub fn ansi(mut self, ansi: Ansi) -> Self {
        self.ansi = ansi;
        self
    }

    /// Set the color palette ramps.
    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }

    /// Set the Base16 color mappings.
    pub fn base16(mut self, base16: Base16) -> Self {
        self.base16 = base16;
        self
    }

    /// Set the semantic color roles.
    pub fn semantic(mut self, semantic: Semantic) -> Self {
        self.semantic = semantic;
        self
    }

    /// Set the UI element colors.
    pub fn ui(mut self, ui: Ui) -> Self {
        self.ui = ui;
        self
    }

    /// Consume the builder and return the constructed [`TcaTheme`].
    pub fn build(self) -> TcaTheme {
        TcaTheme {
            meta: self.meta,
            ansi: self.ansi,
            palette: self.palette,
            base16: self.base16,
            semantic: self.semantic,
            ui: self.ui,
        }
    }
}

/// Parse `#RRGGBB` hex into [`Color::Rgb`]. Returns `None` on malformed input.
#[cfg(feature = "fs")]
fn hex_to_color(hex: &str) -> Option<Color> {
    let (r, g, b) = tca_types::hex_to_rgb(hex).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Resolve a color reference string to a [`Color`].
///
/// Supported formats: `#RRGGBB`, `ansi.<key>`, `palette.<ramp>.<index>`, `base16.<key>`.
/// Returns `None` if the reference cannot be resolved.
#[cfg(feature = "fs")]
fn resolve_ref(r: &str, ansi: &Ansi, palette: &Palette, base16: &Base16) -> Option<Color> {
    if r.starts_with('#') {
        return hex_to_color(r);
    }

    let parts: Vec<&str> = r.splitn(3, '.').collect();
    match parts.as_slice() {
        ["ansi", key] => ansi.get(key),
        ["palette", ramp, idx_str] => {
            let idx: usize = idx_str.parse().ok()?;
            palette.get_ramp(ramp)?.get(idx)
        }
        ["base16", key] => base16.get(key),
        _ => None,
    }
}

/// Parse a raw [`tca_types::Ansi`] into a resolved [`Ansi`].
/// Returns an error if any hex color is malformed (spec requires hex-only in `[ansi]`).
#[cfg(feature = "fs")]
fn parse_ansi(raw: &tca_types::Ansi) -> Result<Ansi> {
    let p = |hex: &str| -> Result<Color> {
        hex_to_color(hex).with_context(|| format!("Invalid hex color in [ansi]: {:?}", hex))
    };
    Ok(Ansi {
        black: p(&raw.black)?,
        red: p(&raw.red)?,
        green: p(&raw.green)?,
        yellow: p(&raw.yellow)?,
        blue: p(&raw.blue)?,
        magenta: p(&raw.magenta)?,
        cyan: p(&raw.cyan)?,
        white: p(&raw.white)?,
        bright_black: p(&raw.bright_black)?,
        bright_red: p(&raw.bright_red)?,
        bright_green: p(&raw.bright_green)?,
        bright_yellow: p(&raw.bright_yellow)?,
        bright_blue: p(&raw.bright_blue)?,
        bright_magenta: p(&raw.bright_magenta)?,
        bright_cyan: p(&raw.bright_cyan)?,
        bright_white: p(&raw.bright_white)?,
    })
}

/// Parse a raw [`tca_types::Palette`] into a resolved [`Palette`].
/// Palette values may be `#RRGGBB` hex or `ansi.<key>` references.
/// Values that cannot be resolved are silently skipped.
#[cfg(feature = "fs")]
fn parse_palette(raw: Option<&tca_types::Palette>, raw_ansi: &tca_types::Ansi) -> Palette {
    let Some(raw_palette) = raw else {
        return Palette::default();
    };

    let ramps = raw_palette
        .entries()
        .map(|(name, values)| {
            let colors = values
                .iter()
                .filter_map(|v| {
                    if v.starts_with('#') {
                        hex_to_color(v)
                    } else if let Some(key) = v.strip_prefix("ansi.") {
                        hex_to_color(raw_ansi.get(key)?)
                    } else {
                        None
                    }
                })
                .collect();
            (name.to_string(), ColorRamp { colors })
        })
        .collect();

    Palette(ramps)
}

/// Parse a raw [`tca_types::Base16`] into a resolved [`Base16`].
/// Values may be `#RRGGBB`, `ansi.<key>`, or `palette.<ramp>.<index>`.
/// Values that cannot be resolved are silently skipped.
#[cfg(feature = "fs")]
fn parse_base16(
    raw: Option<&tca_types::Base16>,
    raw_ansi: &tca_types::Ansi,
    palette: &Palette,
) -> Base16 {
    let Some(raw_b16) = raw else {
        return Base16::default();
    };

    let map = raw_b16
        .entries()
        .filter_map(|(key, value)| {
            let color = if value.starts_with('#') {
                hex_to_color(value)?
            } else if let Some(k) = value.strip_prefix("ansi.") {
                hex_to_color(raw_ansi.get(k)?)?
            } else {
                let parts: Vec<&str> = value.splitn(3, '.').collect();
                match parts.as_slice() {
                    ["palette", ramp, idx_str] => {
                        let idx: usize = idx_str.parse().ok()?;
                        palette.get_ramp(ramp)?.get(idx)?
                    }
                    _ => return None,
                }
            };
            Some((key.to_string(), color))
        })
        .collect();

    Base16(map)
}

/// A cycling cursor over resolved [`TcaTheme`] values.
///
/// Wraps [`tca_types::ThemeCursor<TcaTheme>`] and adds convenience constructors
/// for building a cursor from built-in or user-installed themes.
///
/// All cursor methods (`peek`, `next`, `prev`, `themes`, `len`, `is_empty`)
/// delegate to the inner [`tca_types::ThemeCursor`].
///
/// # Example
///
/// ```rust,no_run
/// use tca_ratatui::TcaThemeCursor;
///
/// let mut cursor = TcaThemeCursor::with_builtins();
/// let first = cursor.peek().unwrap();
/// let second = cursor.next().unwrap();
/// ```
#[cfg(feature = "fs")]
pub struct TcaThemeCursor(tca_types::ThemeCursor<TcaTheme>);

#[cfg(feature = "fs")]
impl TcaThemeCursor {
    /// Create a cursor from an explicit list of resolved themes.
    pub fn new(themes: Vec<TcaTheme>) -> Self {
        Self(tca_types::ThemeCursor::new(themes))
    }

    /// All built-in themes, resolved to [`TcaTheme`].
    /// Themes that fail to resolve are silently skipped.
    pub fn with_builtins() -> Self {
        let themes = tca_types::BuiltinTheme::iter()
            .filter_map(|b| TcaTheme::try_from(b.theme()).ok())
            .collect();
        Self::new(themes)
    }

    /// User-installed themes only, resolved to [`TcaTheme`].
    pub fn with_user_themes() -> Self {
        let themes = tca_types::all_user_themes()
            .into_iter()
            .filter_map(|t| TcaTheme::try_from(t).ok())
            .collect();
        Self::new(themes)
    }

    /// Built-ins + user themes, resolved to [`TcaTheme`].
    /// User themes with matching names override builtins.
    pub fn with_all_themes() -> Self {
        let themes = tca_types::all_themes()
            .into_iter()
            .filter_map(|t| TcaTheme::try_from(t).ok())
            .collect();
        Self::new(themes)
    }

    /// Returns the current theme without moving the cursor.
    pub fn peek(&self) -> Option<&TcaTheme> {
        self.0.peek()
    }

    /// Advances the cursor to the next theme (wrapping) and returns it.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&TcaTheme> {
        self.0.next()
    }

    /// Retreats the cursor to the previous theme (wrapping) and returns it.
    pub fn prev(&mut self) -> Option<&TcaTheme> {
        self.0.prev()
    }

    /// Returns a slice of all themes in the cursor.
    pub fn themes(&self) -> &[TcaTheme] {
        self.0.themes()
    }

    /// Returns the number of themes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the cursor contains no themes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
