//! A StyleSet is a collection of Ratatui Styles created from a TcaTheme.
//!
//! There are a number of preset styles covering many TUI needs, and more can
//! be added as needed for a given application.
use std::collections::HashMap;

use ratatui::style::Style;

use crate::TcaTheme;

/// The main StyleSet struct with predefined styles as named fields
#[derive(Debug, Clone)]
pub struct StyleSet {
    /// Human-readable theme name.
    pub name: String,
    /// Theme author name or contact. Empty string if not specified.
    pub author: String,
    /// `true` for dark themes, `false` for light themes.
    pub is_dark: bool,
    /// The primary style for normal text and other elements.
    pub primary: Style,
    /// A secondary style, for sidebars or other secondary elements.
    pub secondary: Style,
    /// A muted style for disabled/deselected elements.
    pub muted: Style,
    /// A style for borders and other decorative elements.
    pub border: Style,
    /// A style for disabled borders and other decorative elements.
    pub border_muted: Style,
    /// A style for selected text or elements.
    pub selection: Style,
    /// A cursor style
    pub cursor: Style,
    /// A disabled cursor style
    pub cursor_muted: Style,
    /// A style for errors
    pub error: Style,
    /// A style for warnings
    pub warning: Style,
    /// A style for informative text or elements such as spinners
    pub info: Style,
    /// It works!
    pub success: Style,
    /// A style for highlighted information or elements
    pub highlight: Style,
    /// URLs or other links
    pub link: Style,
    // User-defined extensions
    custom: HashMap<String, Style>,
}

impl StyleSet {
    /// Create a `StyleSet` from a theme name, with reasonable fallbacks.
    ///
    /// Accepts any common case format: `"Nord Dark"`, `"nord-dark"`, `"NordDark"`.
    #[cfg(feature = "fs")]
    pub fn from_name(name: &str) -> Self {
        TcaTheme::from_name(name).into()
    }

    /// Create a light `StyleSet` from the user's configured light default.
    ///
    /// Fallback order:
    /// 1. User configured light theme.
    /// 2. Built-in default light theme.
    #[cfg(feature = "fs")]
    pub fn from_default_light_cfg() -> Self {
        TcaTheme::from_default_light_cfg().into()
    }

    /// Add a custom style by name
    pub fn insert_custom(&mut self, key: impl Into<String>, style: Style) {
        self.custom.insert(key.into(), style);
    }

    /// Get a custom style by name
    pub fn get_custom(&self, key: &str) -> Option<&Style> {
        self.custom.get(key)
    }
}

impl From<TcaTheme> for StyleSet {
    fn from(value: TcaTheme) -> Self {
        StyleSet {
            name: value.meta.name.clone(),
            author: value.meta.author.clone(),
            is_dark: value.meta.dark,
            primary: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.fg_primary),
            secondary: Style::default()
                .bg(value.ui.bg_secondary)
                .fg(value.ui.fg_secondary),
            muted: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.fg_muted),
            border: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.border_primary),
            border_muted: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.border_muted),
            selection: Style::default()
                .bg(value.ui.selection_bg)
                .fg(value.ui.selection_fg),
            cursor: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.cursor_primary),
            cursor_muted: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.ui.cursor_muted),
            error: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.error),
            warning: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.warning),
            info: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.info),
            success: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.success),
            highlight: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.highlight),
            link: Style::default()
                .bg(value.ui.bg_primary)
                .fg(value.semantic.link),
            custom: HashMap::default(),
        }
    }
}

impl From<&TcaTheme> for StyleSet {
    fn from(value: &TcaTheme) -> Self {
        value.clone().into()
    }
}

/// Returns the user's configured default theme as a `StyleSet`, falling back to the built-in default.
///
/// Reads `$XDG_CONFIG_HOME/tca/tca.toml` if the `fs` feature is enabled.
/// For a guaranteed no-I/O default, convert directly from a `BuiltinTheme`.
#[cfg(feature = "fs")]
impl Default for StyleSet {
    fn default() -> Self {
        TcaTheme::default().into()
    }
}

/// A cycling cursor over a collection of [`StyleSet`]s, backed by a [`TcaThemeCursor`].
///
/// Converts [`TcaTheme`] to [`StyleSet`] on access, so the full theme collection
/// is held in memory as themes (not pre-converted styles).
#[cfg(feature = "fs")]
#[derive(Debug)]
pub struct StyleSetCursor(crate::theme::TcaThemeCursor);

#[cfg(feature = "fs")]
impl StyleSetCursor {
    /// Create a cursor from an arbitrary iterator of [`TcaTheme`]s.
    pub fn new(themes: impl IntoIterator<Item = TcaTheme>) -> Self {
        Self(crate::theme::TcaThemeCursor::new(themes))
    }

    /// All built-in themes.
    pub fn with_builtins() -> Self {
        Self(crate::theme::TcaThemeCursor::with_builtins())
    }

    /// User-installed themes only.
    pub fn with_user_themes() -> Self {
        Self(crate::theme::TcaThemeCursor::with_user_themes())
    }

    /// Built-ins + user themes.
    pub fn with_all_themes() -> Self {
        Self(crate::theme::TcaThemeCursor::with_all_themes())
    }

    /// The current [`StyleSet`] without advancing the cursor.
    pub fn peek(&self) -> Option<StyleSet> {
        self.0.peek().map(Into::into)
    }

    /// Advance to the next theme (wrapping) and return it as a [`StyleSet`].
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<StyleSet> {
        self.0.next().map(Into::into)
    }

    /// Move to the previous theme (wrapping) and return it as a [`StyleSet`].
    pub fn prev(&mut self) -> Option<StyleSet> {
        self.0.prev().map(Into::into)
    }

    /// Move the cursor to the theme matching `name` (slug-insensitive).
    pub fn set_current(&mut self, name: &str) -> Option<StyleSet> {
        self.0.set_current(name).map(Into::into)
    }

    /// Number of themes in the cursor.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the cursor contains no themes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
