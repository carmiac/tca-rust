#[cfg(feature = "loader")]
use anyhow::{Context, Result};
use ratatui::style::Color;
use std::collections::HashMap;

/// Theme metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    pub name: String,
    pub slug: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub dark: Option<bool>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "Unnamed Theme".to_string(),
            slug: None,
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
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
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
    pub colors: Vec<Color>,
}

impl ColorRamp {
    /// Returns the color at the given 0-based index, or `None` if out of range.
    pub fn get(&self, idx: usize) -> Option<Color> {
        self.colors.get(idx).copied()
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Returns a sorted list of all valid indices for this ramp.
    pub fn indices(&self) -> Vec<usize> {
        (0..self.colors.len()).collect()
    }
}

/// Color palette of named hue ramps (`[palette]` section).
///
/// All ramps are 0-indexed and ordered darkest → lightest.
/// An absent `[palette]` section deserializes to an empty palette.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Palette(pub HashMap<String, ColorRamp>);

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
pub struct Base16(pub HashMap<String, Color>);

impl Base16 {
    /// Returns the resolved color for the given Base16 key, or `None`.
    pub fn get(&self, key: &str) -> Option<Color> {
        self.0.get(key).copied()
    }
}

/// Semantic color roles.
///
/// The [`Default`] impl maps to Ratatui's named colors as a fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Semantic {
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub success: Color,
    pub highlight: Color,
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
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_muted: Color,
    pub border_primary: Color,
    pub border_muted: Color,
    pub cursor_primary: Color,
    pub cursor_muted: Color,
    pub selection_bg: Color,
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
/// Construct via [`ThemeLoader`] or [`TcaThemeBuilder`].
#[derive(Debug, Clone, PartialEq)]
pub struct TcaTheme {
    pub meta: Meta,
    pub ansi: Ansi,
    /// Resolved palette ramps. Empty if the theme has no `[palette]` section.
    pub palette: Palette,
    /// Resolved Base16 mappings. Empty if the theme has no `[base16]` section.
    pub base16: Base16,
    pub semantic: Semantic,
    pub ui: Ui,
}

impl TcaTheme {
    /// Returns the theme name.
    pub fn name(&self) -> &str {
        &self.meta.name
    }

    /// Returns the theme author, if provided.
    pub fn author(&self) -> Option<&str> {
        self.meta.author.as_deref()
    }

    /// Returns `true` if the theme is a dark theme. Defaults to `true` when unspecified.
    pub fn is_dark(&self) -> bool {
        self.meta.dark.unwrap_or(true)
    }

    /// Load a TCA theme from a file path or theme name.
    ///
    /// Accepts an explicit path (`"themes/nord-dark.toml"`) or a bare theme
    /// name (`"nord-dark"`) looked up from `$XDG_DATA_HOME/tca/themes/`.
    #[cfg(feature = "loader")]
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<TcaTheme> {
        ThemeLoader::from_file(path)
    }

    /// Parse a TCA theme from a TOML string.
    #[cfg(feature = "loader")]
    pub fn from_toml(toml: &str) -> Result<TcaTheme> {
        ThemeLoader::from_toml(toml)
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn meta(mut self, meta: Meta) -> Self {
        self.meta = meta;
        self
    }

    pub fn ansi(mut self, ansi: Ansi) -> Self {
        self.ansi = ansi;
        self
    }

    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }

    pub fn base16(mut self, base16: Base16) -> Self {
        self.base16 = base16;
        self
    }

    pub fn semantic(mut self, semantic: Semantic) -> Self {
        self.semantic = semantic;
        self
    }

    pub fn ui(mut self, ui: Ui) -> Self {
        self.ui = ui;
        self
    }

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

/// Loads and resolves TCA theme files into [`TcaTheme`].
///
/// Uses [`tca_loader`] for XDG-aware file discovery and [`tca_types`] for
/// raw TOML parsing, then resolves all color references into concrete
/// Ratatui [`Color`] values.
///
/// All color references that cannot be resolved (e.g. a `palette.*` ref in
/// a theme without a `[palette]` section) silently fall back to
/// [`Color::Reset`].
#[cfg(feature = "loader")]
pub struct ThemeLoader;

#[cfg(feature = "loader")]
impl ThemeLoader {
    /// Load a theme from a file path or bare theme name.
    ///
    /// - Absolute/relative path → loaded directly.
    /// - Bare name (e.g. `"nord-dark"`) → searched in `$XDG_DATA_HOME/tca/themes/`.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<TcaTheme> {
        let path_str = path
            .as_ref()
            .to_str()
            .context("Theme path contains non-UTF-8 characters")?;
        let raw: tca_types::Theme =
            tca_loader::load_theme(path_str).context("Failed to load theme file")?;
        Self::resolve(raw)
    }

    /// Parse and resolve a theme from a TOML string.
    pub fn from_toml(toml: &str) -> Result<TcaTheme> {
        let raw: tca_types::Theme = ::toml::from_str(toml).context("Failed to parse theme TOML")?;
        Self::resolve(raw)
    }

    fn resolve(raw: tca_types::Theme) -> Result<TcaTheme> {
        // ANSI is required and hex-only; hard error on bad hex.
        let ansi = parse_ansi(&raw.ansi)?;

        // Palette and Base16 are optional; absent sections -> empty defaults.
        let palette = parse_palette(raw.palette.as_ref(), &ansi);
        let base16 = parse_base16(raw.base16.as_ref(), &ansi, &palette);

        let resolve = |r: &str| resolve_ref(r, &ansi, &palette, &base16);

        let semantic = Semantic {
            error: resolve(&raw.semantic.error),
            warning: resolve(&raw.semantic.warning),
            info: resolve(&raw.semantic.info),
            success: resolve(&raw.semantic.success),
            highlight: resolve(&raw.semantic.highlight),
            link: resolve(&raw.semantic.link),
        };

        let ui = Ui {
            bg_primary: resolve(&raw.ui.bg.primary),
            bg_secondary: resolve(&raw.ui.bg.secondary),
            fg_primary: resolve(&raw.ui.fg.primary),
            fg_secondary: resolve(&raw.ui.fg.secondary),
            fg_muted: resolve(&raw.ui.fg.muted),
            border_primary: resolve(&raw.ui.border.primary),
            border_muted: resolve(&raw.ui.border.muted),
            cursor_primary: resolve(&raw.ui.cursor.primary),
            cursor_muted: resolve(&raw.ui.cursor.muted),
            selection_bg: resolve(&raw.ui.selection.bg),
            selection_fg: resolve(&raw.ui.selection.fg),
        };

        let meta = Meta {
            name: raw.meta.name,
            slug: raw.meta.slug,
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

/// Parse `#RRGGBB` hex into [`Color::Rgb`]. Returns `None` on malformed input.
fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Look up an `ansi.<key>` name against a resolved [`Ansi`] struct.
fn ansi_key(key: &str, ansi: &Ansi) -> Option<Color> {
    match key {
        "black" => Some(ansi.black),
        "red" => Some(ansi.red),
        "green" => Some(ansi.green),
        "yellow" => Some(ansi.yellow),
        "blue" => Some(ansi.blue),
        "magenta" => Some(ansi.magenta),
        "cyan" => Some(ansi.cyan),
        "white" => Some(ansi.white),
        "bright_black" => Some(ansi.bright_black),
        "bright_red" => Some(ansi.bright_red),
        "bright_green" => Some(ansi.bright_green),
        "bright_yellow" => Some(ansi.bright_yellow),
        "bright_blue" => Some(ansi.bright_blue),
        "bright_magenta" => Some(ansi.bright_magenta),
        "bright_cyan" => Some(ansi.bright_cyan),
        "bright_white" => Some(ansi.bright_white),
        _ => None,
    }
}

/// Resolve a color reference string to a [`Color`].
///
/// Supported formats: `#RRGGBB`, `ansi.<key>`, `palette.<ramp>.<index>`, `base16.<key>`.
/// Unresolvable references return [`Color::Reset`].
#[cfg(feature = "loader")]
fn resolve_ref(r: &str, ansi: &Ansi, palette: &Palette, base16: &Base16) -> Color {
    if r.starts_with('#') {
        return hex_to_color(r).unwrap_or(Color::Reset);
    }

    let parts: Vec<&str> = r.splitn(3, '.').collect();
    match parts.as_slice() {
        ["ansi", key] => ansi_key(key, ansi).unwrap_or(Color::Reset),
        ["palette", ramp, idx_str] => {
            let idx: usize = idx_str.parse().unwrap_or(usize::MAX);
            palette
                .get_ramp(ramp)
                .and_then(|r| r.get(idx))
                .unwrap_or(Color::Reset)
        }
        ["base16", key] => base16.get(key).unwrap_or(Color::Reset),
        _ => Color::Reset,
    }
}

/// Parse a raw [`tca_types::Ansi`] into a resolved [`Ansi`].
/// Returns an error if any hex color is malformed (spec requires hex-only in `[ansi]`).
#[cfg(feature = "loader")]
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
#[cfg(feature = "loader")]
fn parse_palette(raw: Option<&tca_types::Palette>, ansi: &Ansi) -> Palette {
    let Some(raw_palette) = raw else {
        return Palette::default();
    };

    let ramps = raw_palette
        .0
        .iter()
        .map(|(name, values)| {
            let colors = values
                .iter()
                .filter_map(|v| {
                    if v.starts_with('#') {
                        hex_to_color(v)
                    } else if let Some(key) = v.strip_prefix("ansi.") {
                        ansi_key(key, ansi)
                    } else {
                        None
                    }
                })
                .collect();
            (name.clone(), ColorRamp { colors })
        })
        .collect();

    Palette(ramps)
}

/// Parse a raw [`tca_types::Base16`] into a resolved [`Base16`].
/// Values may be `#RRGGBB`, `ansi.<key>`, or `palette.<ramp>.<index>`.
/// Values that cannot be resolved are silently skipped.
#[cfg(feature = "loader")]
fn parse_base16(raw: Option<&tca_types::Base16>, ansi: &Ansi, palette: &Palette) -> Base16 {
    let Some(raw_b16) = raw else {
        return Base16::default();
    };

    let map = raw_b16
        .0
        .iter()
        .filter_map(|(key, value)| {
            let color = if value.starts_with('#') {
                hex_to_color(value)?
            } else if let Some(k) = value.strip_prefix("ansi.") {
                ansi_key(k, ansi)?
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
            Some((key.clone(), color))
        })
        .collect();

    Base16(map)
}
