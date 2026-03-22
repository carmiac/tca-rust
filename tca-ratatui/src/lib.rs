//! Terminal Colors Architecture (TCA) theme support for ratatui.
//!
//! This crate provides utilities for loading and using TCA themes in ratatui applications.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tca_ratatui::TcaTheme;
//! use ratatui::style::Style;
//! use std::path::Path;
//!
//! fn example() {
//! // Load a TCA theme from a file or name, with reasonable fallback.
//! let theme = TcaTheme::new(Some("theme.toml"));
//! let theme = TcaTheme::new(Some("Tokyo Night"));
//! let theme = TcaTheme::new(Some("tokyo-night"));
//! let theme = TcaTheme::new(None);
//!
//! // Or from a TOML string.
//! let toml_str = r###"
//! [theme]
//! name = "Tokyo Night"
//! slug = "tokyo-night"
//! author = "enkia / TCA Project"
//! version = "1.0.0"
//! description = "A clean, dark theme inspired by the city lights of Tokyo at night. Based on Tokyo Night by enkia."
//! dark = true
//!
//! [ansi]
//! black = "#15161e"
//! red = "#f7768e"
//! green = "#9ece6a"
//! yellow = "#e0af68"
//! blue = "#7aa2f7"
//! magenta = "#bb9af7"
//! cyan = "#7dcfff"
//! white = "#a9b1d6"
//! bright_black = "#414868"
//! bright_red = "#f7768e"
//! bright_green = "#9ece6a"
//! bright_yellow = "#e0af68"
//! bright_blue = "#7aa2f7"
//! bright_magenta = "#bb9af7"
//! bright_cyan = "#7dcfff"
//! bright_white = "#c0caf5"
//!
//! # Palette: darkest→lightest
//! # neutral: bg_dark→fg
//! # 0=#15161e  1=#1a1b26  2=#24283b  3=#292e42  4=#414868  5=#565f89  6=#a9b1d6  7=#c0caf5
//! [palette]
//! neutral = [
//!   "#15161e",
//!   "#1a1b26",
//!   "#24283b",
//!   "#292e42",
//!   "#414868",
//!   "#565f89",
//!   "#a9b1d6",
//!   "#c0caf5",
//! ]
//! red = ["#6b1a2a", "#db4b4b", "#f7768e"]
//! green = ["#2d4a1e", "#41a146", "#9ece6a"]
//! yellow = ["#5c4400", "#c09230", "#e0af68"]
//! blue = ["#1a2a6b", "#3d59a1", "#7aa2f7"]
//! purple = ["#3d2370", "#7953c7", "#bb9af7"]
//! cyan = ["#144a60", "#0db9d7", "#7dcfff"]
//! orange = ["#5c2e00", "#c47214", "#ff9e64"]
//! teal = ["#17403d", "#1abc9c", "#73daca"]
//! magenta = ["#5c1a5c", "#9d4bb6", "#c586c0"]
//!
//! [semantic]
//! error = "palette.red.2"
//! warning = "palette.orange.2"
//! info = "palette.cyan.2"
//! success = "palette.green.2"
//! highlight = "palette.yellow.2"
//! link = "palette.blue.2"
//!
//! [ui.bg]
//! primary = "palette.neutral.1"
//! secondary = "palette.neutral.2"
//!
//! [ui.fg]
//! primary = "palette.neutral.7"
//! secondary = "palette.neutral.6"
//! muted = "palette.neutral.5"
//!
//! [ui.border]
//! primary = "palette.blue.1"
//! muted = "palette.neutral.3"
//!
//! [ui.cursor]
//! primary = "palette.blue.2"
//! muted = "palette.neutral.5"
//!
//! [ui.selection]
//! bg = "palette.neutral.3"
//! fg = "palette.neutral.7"
//! "###;
//! let theme = TcaTheme::try_from(toml_str).expect("Couldn't parse TOML");
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
//! }
//! ```

#![warn(missing_docs)]

mod theme;

#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(test)]
mod tests;

pub use theme::{Ansi, Base16, ColorRamp, Meta, Palette, Semantic, TcaTheme, TcaThemeBuilder, Ui};

#[cfg(feature = "widgets")]
pub use widgets::ColorPicker;
