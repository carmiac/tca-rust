//! Terminal Colors Architecture (TCA) theme support for ratatui.
//!
//! This crate provides utilities for loading and using TCA themes in ratatui applications.
//! Themes are [base24](https://github.com/tinted-theming/base24/) YAML files.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tca_ratatui::TcaTheme;
//! use ratatui::style::Style;
//!
//! fn example() {
//! // Load a TCA theme from a file or name, with reasonable fallback.
//! let theme = TcaTheme::new(Some("theme.yaml"));
//! let theme = TcaTheme::new(Some("Tokyo Night"));
//! let theme = TcaTheme::new(Some("tokyo-night"));
//! let theme = TcaTheme::new(None);
//!
//! // Or from a base24 YAML string.
//! let yaml = r#"
//! scheme: "Tokyo Night"
//! author: "enkia / TCA Project"
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
//! let theme = TcaTheme::try_from(yaml).expect("Couldn't parse YAML");
//!
//! // Use ANSI colors
//! let error_style = Style::default().fg(theme.ansi.red);
//! let success_style = Style::default().fg(theme.ansi.green);
//!
//! // Use semantic colors
//! let error_style = Style::default().fg(theme.semantic.error);
//! let warning_style = Style::default().fg(theme.semantic.warning);
//!
//! // Use UI colors
//! let bg_style = Style::default().bg(theme.ui.bg_primary);
//! let fg_style = Style::default().fg(theme.ui.fg_primary);
//!
//! // Access raw base24 slots directly
//! let base00 = theme.base24[0]; // darkest background
//! let base08 = theme.base24[8]; // red / errors
//! }
//! ```

#![warn(missing_docs)]

mod theme;

#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(test)]
mod tests;

pub use theme::{Ansi, Meta, Semantic, TcaTheme, TcaThemeBuilder, Ui};

#[cfg(feature = "fs")]
pub use tca_types::ThemeCursor;

#[cfg(feature = "fs")]
pub use theme::TcaThemeCursor;

#[cfg(feature = "widgets")]
pub use widgets::ColorPicker;
