#[cfg(feature = "fs")]
use anyhow::{Context, Result};
use ratatui::style::Color;
#[cfg(feature = "fs")]
use std::path::PathBuf;

#[cfg(feature = "fs")]
use tca_types::BuiltinTheme;

/// Theme metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {
    /// Human-readable theme name.
    pub name: String,
    /// Theme author name or contact. Empty string if not specified.
    pub author: String,
    /// `true` for dark themes, `false` for light themes.
    pub dark: bool,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "Unnamed Theme".to_string(),
            author: String::new(),
            dark: false,
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
/// Construct via [`TcaThemeBuilder`] or by loading a base24 YAML file.
#[derive(Debug, Clone)]
pub struct TcaTheme {
    /// Theme metadata (name, author, dark).
    pub meta: Meta,
    /// Resolved ANSI 16-color definitions.
    pub ansi: Ansi,
    /// Resolved semantic color roles.
    pub semantic: Semantic,
    /// Resolved UI element colors.
    pub ui: Ui,
    /// Raw base24 slot colors: index 0 = base00, index 23 = base17.
    pub base24: [Color; 24],
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
    /// Returns the canonical name slug for the theme.
    ///
    /// This is the kebab-case version of the theme name.
    /// e.g. "Tokyo Night" => "tokyo-night"
    pub fn name_slug(&self) -> String {
        heck::AsKebabCase(&self.meta.name).to_string()
    }

    /// Returns the canonical file name for the theme.
    ///
    /// This is the kebab-case name + '.yaml'
    /// e.g. "Tokyo Night" => "tokyo-night.yaml"
    pub fn to_filename(&self) -> String {
        let mut theme_name = self.name_slug();
        if !theme_name.ends_with(".yaml") {
            theme_name.push_str(".yaml");
        }
        theme_name
    }

    /// Returns the canonical file path for the theme.
    ///
    /// Note that this is not necessarily the current location of the theme, nor
    /// does it tell you if the theme is locally installed. It just tells you
    /// where it should be installed to.
    #[cfg(feature = "fs")]
    pub fn to_pathbuf(&self) -> Result<PathBuf> {
        use tca_types::user_themes_path;

        let mut path = user_themes_path()?;
        path.push(self.to_filename());
        Ok(path)
    }
}

#[cfg(feature = "fs")]
impl Default for TcaTheme {
    fn default() -> Self {
        TcaTheme::new(None)
    }
}

#[cfg(feature = "fs")]
/// Load a TcaTheme from a base24 YAML string.
impl TryFrom<&str> for TcaTheme {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<TcaTheme, Self::Error> {
        let raw = tca_types::Theme::from_base24_str(value)?;
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
        let ansi = parse_ansi(&raw.ansi)?;

        let c = |hex: &str| hex_to_color(hex).unwrap_or(Color::Reset);

        let defaults = Semantic::default();
        let semantic = Semantic {
            error: hex_to_color(&raw.semantic.error).unwrap_or(defaults.error),
            warning: hex_to_color(&raw.semantic.warning).unwrap_or(defaults.warning),
            info: hex_to_color(&raw.semantic.info).unwrap_or(defaults.info),
            success: hex_to_color(&raw.semantic.success).unwrap_or(defaults.success),
            highlight: hex_to_color(&raw.semantic.highlight).unwrap_or(defaults.highlight),
            link: hex_to_color(&raw.semantic.link).unwrap_or(defaults.link),
        };

        let defaults = Ui::default();
        let ui = Ui {
            bg_primary: hex_to_color(&raw.ui.bg.primary).unwrap_or(defaults.bg_primary),
            bg_secondary: hex_to_color(&raw.ui.bg.secondary).unwrap_or(defaults.bg_secondary),
            fg_primary: hex_to_color(&raw.ui.fg.primary).unwrap_or(defaults.fg_primary),
            fg_secondary: hex_to_color(&raw.ui.fg.secondary).unwrap_or(defaults.fg_secondary),
            fg_muted: hex_to_color(&raw.ui.fg.muted).unwrap_or(defaults.fg_muted),
            border_primary: hex_to_color(&raw.ui.border.primary)
                .unwrap_or(defaults.border_primary),
            border_muted: hex_to_color(&raw.ui.border.muted).unwrap_or(defaults.border_muted),
            cursor_primary: hex_to_color(&raw.ui.cursor.primary)
                .unwrap_or(defaults.cursor_primary),
            cursor_muted: hex_to_color(&raw.ui.cursor.muted).unwrap_or(defaults.cursor_muted),
            selection_bg: hex_to_color(&raw.ui.selection.bg).unwrap_or(defaults.selection_bg),
            selection_fg: hex_to_color(&raw.ui.selection.fg).unwrap_or(defaults.selection_fg),
        };

        let s = &raw.base24;
        let base24 = [
            c(&s.base00), c(&s.base01), c(&s.base02), c(&s.base03),
            c(&s.base04), c(&s.base05), c(&s.base06), c(&s.base07),
            c(&s.base08), c(&s.base09), c(&s.base0a), c(&s.base0b),
            c(&s.base0c), c(&s.base0d), c(&s.base0e), c(&s.base0f),
            c(&s.base10), c(&s.base11), c(&s.base12), c(&s.base13),
            c(&s.base14), c(&s.base15), c(&s.base16), c(&s.base17),
        ];

        let meta = Meta {
            name: raw.meta.name,
            author: raw.meta.author,
            dark: raw.meta.dark,
        };

        Ok(TcaTheme { meta, ansi, semantic, ui, base24 })
    }
}

impl PartialEq for TcaTheme {
    fn eq(&self, other: &Self) -> bool {
        self.name_slug() == other.name_slug()
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
        self.name_slug().cmp(&other.name_slug())
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
            semantic: self.semantic,
            ui: self.ui,
            base24: [Color::Reset; 24],
        }
    }
}

/// Parse `#RRGGBB` hex into [`Color::Rgb`]. Returns `None` on malformed input.
#[cfg(feature = "fs")]
fn hex_to_color(hex: &str) -> Option<Color> {
    let (r, g, b) = tca_types::hex_to_rgb(hex).ok()?;
    Some(Color::Rgb(r, g, b))
}

/// Parse a raw [`tca_types::Ansi`] into a resolved [`Ansi`].
/// Returns an error if any hex color is malformed.
#[cfg(feature = "fs")]
fn parse_ansi(raw: &tca_types::Ansi) -> Result<Ansi> {
    let p = |hex: &str| -> Result<Color> {
        hex_to_color(hex).with_context(|| format!("Invalid hex color in ansi: {:?}", hex))
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
    /// Create a cursor from any iterable of resolved themes. The cursor starts at the first theme.
    pub fn new(themes: impl IntoIterator<Item = TcaTheme>) -> Self {
        Self(tca_types::ThemeCursor::new(themes))
    }

    /// All built-in themes, resolved to [`TcaTheme`].
    /// Themes that fail to resolve are silently skipped.
    pub fn with_builtins() -> Self {
        Self::new(
            tca_types::BuiltinTheme::iter().filter_map(|b| TcaTheme::try_from(b.theme()).ok()),
        )
    }

    /// User-installed themes only, resolved to [`TcaTheme`].
    pub fn with_user_themes() -> Self {
        Self::new(
            tca_types::all_user_themes()
                .into_iter()
                .filter_map(|t| TcaTheme::try_from(t).ok()),
        )
    }

    /// Built-ins + user themes, resolved to [`TcaTheme`].
    /// User themes with matching names override builtins.
    pub fn with_all_themes() -> Self {
        Self::new(
            tca_types::all_themes()
                .into_iter()
                .filter_map(|t| TcaTheme::try_from(t).ok()),
        )
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

    /// Moves the cursor to the theme matching `name` (slug-insensitive) and returns it.
    ///
    /// Accepts fuzzy names: "Nord Dark", "nord-dark", and "nordDark" all match the same theme.
    /// Returns `None` if no matching theme is found.
    pub fn set_current(&mut self, name: &str) -> Option<&TcaTheme> {
        let slug = heck::AsKebabCase(name).to_string();
        let idx = self.0.themes().iter().position(|t| t.name_slug() == slug)?;
        self.0.set_index(idx)
    }
}
