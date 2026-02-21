//! Terminal Colors Architecture (TCA) theme support for ratatui.
//!
//! This crate provides utilities for loading and using TCA themes in ratatui applications.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tca_ratatui::TcaTheme;
//! use ratatui::style::Style;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load a TCA theme from a file
//! let theme = TcaTheme::from_file("theme.toml")?;
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
//! # Ok(())
//! # }
//! ```

mod theme;

#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(test)]
mod tests;

pub use theme::{Ansi, Base16, ColorRamp, Meta, Palette, Semantic, TcaTheme, TcaThemeBuilder, Ui};

#[cfg(feature = "loader")]
pub use theme::ThemeLoader;

#[cfg(feature = "widgets")]
pub use widgets::ColorPicker;
