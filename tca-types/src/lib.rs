//! Core types for Terminal Colors Architecture (TCA)
//!
//! This crate provides the foundational type definitions used across
//! the TCA ecosystem for theme representation and manipulation.

#![warn(missing_docs)]

mod builtin;
mod cursor;
mod theme;
mod util;

pub use builtin::BuiltinTheme;
pub use cursor::ThemeCursor;
pub use theme::{
    hex_to_rgb, Ansi, Base16, HexColorError, Meta, Palette, Semantic, Theme, Ui, UiBg, UiBorder,
    UiCursor, UiFg, UiSelection,
};

#[cfg(feature = "fs")]
mod config;
#[cfg(feature = "fs")]
pub use config::TcaConfig;
#[cfg(feature = "fs")]
pub use util::{all_from_dir, all_themes, all_user_themes, load_theme_file, user_themes_path};
