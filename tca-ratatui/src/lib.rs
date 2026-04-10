//! Terminal Colors Architecture (TCA) theme support for ratatui.
//!
//! This crate provides utilities for loading and using TCA themes in ratatui applications.
//! Themes are [base24](https://github.com/tinted-theming/base24/) YAML files.
//!
//! ## Quick Start - StyleSet
//!
//! The easiest way to theme a ratatui app is with [`StyleSet`]. It pre-builds all the
//! common [`ratatui::style::Style`]s you need from the user's configured theme.
//!
//! ```rust,no_run
//! use tca_ratatui::StyleSet;
//! use ratatui::widgets::{Block, Borders, Paragraph};
//!
//! fn render(frame: &mut ratatui::Frame) {
//!     let styles = StyleSet::default(); // user's configured theme, or built-in fallback
//!
//!     let block = Block::default()
//!         .borders(Borders::ALL)
//!         .style(styles.border);
//!
//!     let text = Paragraph::new("Hello, world!")
//!         .block(block)
//!         .style(styles.primary);
//!
//!     frame.render_widget(text, frame.area());
//! }
//! ```
//!
//! Load by name, or cycle through available themes with [`StyleSetCursor`]:
//!
//! ```rust,no_run
//! use tca_ratatui::{StyleSet, StyleSetCursor};
//!
//! // Load a specific theme by name (any case format works)
//! let styles = StyleSet::from_name("tokyo-night");
//! let styles = StyleSet::from_name("Tokyo Night");
//!
//! // Cycle through all installed + built-in themes
//! let mut cursor = StyleSetCursor::with_all_themes();
//! cursor.next();
//! cursor.prev();
//! cursor.set_current("nord-dark");
//! let styles = cursor.peek().unwrap_or_default();
//! ```
//!
//! ## Lower-Level Access - TcaTheme
//!
//! For direct access to typed colors (e.g. for custom [`ratatui::style::Style`] composition),
//! use [`TcaTheme`] directly:
//!
//! ```rust,no_run
//! use tca_ratatui::TcaTheme;
//! use ratatui::style::Style;
//!
//! fn example() {
//!     let theme = TcaTheme::from_name("tokyo-night");
//!     let theme = TcaTheme::default(); // user's configured default
//!
//!     // ANSI, semantic, and UI colors are all resolved
//!     let error_style = Style::default().fg(theme.semantic.error);
//!     let bg_style = Style::default().bg(theme.ui.bg_primary);
//!     let red = theme.ansi.red;
//!
//!     // Raw base24 slots
//!     let base00 = theme.base24[0]; // darkest background
//! }
//! ```
//!
//! ## Parsing from YAML
//!
//! ```rust,no_run
//! use tca_ratatui::TcaTheme;
//!
//! fn example() {
//!     let yaml = r#"
//! scheme: "My Theme"
//! author: "You"
//! variant: "dark"
//! base00: "1a1b26"
//! base01: "16161e"
//! base02: "2f3549"
//! base03: "444b6a"
//! base04: "787c99"
//! base05: "a9b1d6"
//! base06: "cbccd1"
//! base07: "d5d6db"
//! base08: "f7768e"
//! base09: "ff9e64"
//! base0A: "e0af68"
//! base0B: "9ece6a"
//! base0C: "7dcfff"
//! base0D: "7aa2f7"
//! base0E: "bb9af7"
//! base0F: "f7768e"
//! base10: "1a1b26"
//! base11: "16161e"
//! base12: "f7768e"
//! base13: "e0af68"
//! base14: "9ece6a"
//! base15: "7dcfff"
//! base16: "7aa2f7"
//! base17: "bb9af7"
//! "#;
//!     let theme = TcaTheme::try_from(yaml).expect("Couldn't parse YAML");
//! }
//! ```

#![warn(missing_docs)]

mod styleset;
mod theme;

#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(test)]
mod tests;

pub use styleset::StyleSet;

#[cfg(feature = "fs")]
pub use styleset::StyleSetCursor;
pub use theme::{Ansi, Meta, Semantic, TcaTheme, TcaThemeBuilder, Ui};

#[cfg(feature = "fs")]
pub use tca_types::ThemeCursor;

#[cfg(feature = "fs")]
pub use theme::TcaThemeCursor;

#[cfg(feature = "widgets")]
pub use widgets::ColorPicker;
