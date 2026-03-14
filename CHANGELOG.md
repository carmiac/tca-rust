# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0]

### Added

- Built-in themes embedded directly in `tca-types` (no files required): `catppuccin-mocha`, `cyberpunk`, `dracula`, `everforest-dark`, `gruvbox-dark`, `mono`, `nord`, `one-dark`, `rose-pine`, `solarized-light`, `tokyo-night`.
- `TcaTheme::new()` in `tca-ratatui` is the primary entry point for loading a theme. Resolves a theme name or file path through a 4-step fallback chain: user theme files -> built-in themes -> user preference config -> terminal dark/light auto-detection.
- `load_all_builtin()` in `tca-ratatui` returns all built-in themes as `Vec<TcaTheme>`.
- `BuiltinTheme` enum in `tca-types` with `EnumIter` support for iterating all built-ins.
- `TcaConfig` in `tca-loader` reads and writes user theme preferences from `~/.config/tca/tca.toml`. `mode_aware_theme()` returns the best theme name for the current terminal's dark or light mode.

### Changed

- `TryFrom<tca_types::Theme> for TcaTheme` is now `#[doc(hidden)]`.
- Updated all crate READMEs.

### Removed

- `load_all_from_dir()` and `load_all_from_theme_dir()` removed from `tca-ratatui`'s public API. Use `tca_loader::list_themes()` combined with `TcaTheme::new()` instead.

## [0.1.0]

### Initial Release

[Unreleased]: https://github.com/carmiac/tca-rust/commits/main
[0.2.0]: https://github.com/carmiac/tca-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/carmiac/tca-rust/releases/tag/v0.1.0
