# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0]

### Changed

- Migrated theme format from custom TOML to [base24](https://github.com/tinted-theming/base24/) YAML (spec v0.4.0). Theme files now use `.yaml` extensions and the flat base24 key/value format.
- Custom flat-YAML parser replaces `serde_yaml`; `serde_json`/`jsonschema`/`toml` removed as theme dependencies.
- `Theme` struct redesigned: `Base24Slots` holds raw hex values; all semantic fields (`Ansi`, `Semantic`, `Ui*`) are resolved at load time from base24 slots.

### Added

- `Theme::from_base24_str()` — parse a base24 YAML string into a fully resolved `Theme`.
- Built-in themes converted to base24 YAML.

### Removed

- `Ansi::get()` (dead code after base24 migration).
- `tca import` and `tca export` CLI commands.

## [0.5.0]

### Added

- `tca add` - install themes from local files, directories, or by downloading from a remote git repository by name (fuzzy matching supported)
- `tca config` - show or set user configuration values
- `tca init` - create a default config file and install built-in themes as TOML files
- `Theme::to_pathbuf()` and `Theme::to_filename()` in `tca-types` - derive a theme's canonical install path from its name
- `ThemeCursor::set_index()` — move the cursor to an arbitrary index.
- `ThemeCursor<Theme>::set_current()` — move the cursor to a theme by name (slug-insensitive: "Nord Dark", "nord-dark", "nordDark" all match).
- `TcaThemeCursor::set_current()` — same slug-insensitive name lookup for the ratatui cursor.
- `PartialOrd` / `Ord` for `TcaTheme` in `tca-ratatui`, ordering by name slug.

### Changed

- `ThemeCursor::new()` now accepts `impl IntoIterator<Item = T>` instead of `Vec<T>`, removing the `T: Ord` requirement from the generic impl block.

## [0.4.0]

### Added

- `ThemeCursor<T>` generic cycling cursor in `tca-types`. Comes with `with_builtins()`, `with_user_themes()`, `with_all_themes()`, and `new(vec)` constructors on `ThemeCursor<Theme>`.
- `TcaThemeCursor` newtype in `tca-ratatui` wrapping `ThemeCursor<TcaTheme>`. Full navigation API with its own `with_builtins()`, `with_user_themes()`, `with_all_themes()`, and `new(vec)` constructors.
- `BuiltinTheme::iter()` convenience wrapper that hides the `strum` dependency from callers.

### Changed

- `tca` crate renamed to `tca-cli` on crates.io (binary name `tca` is unchanged).
- `tca-ratatui`: feature flag corrected from `loader` to `fs` (aligns with `tca-types`).
- `tca-ratatui`: `toml` and `terminal-colorsaurus` are now optional dependencies, only pulled in under the `fs` feature.

## [0.3.0]

### Added

- Flexible theme name resolution. e.g. "Nord Dark", "Nord-Dark", "nord-dark", "nord dark", "nordDark" all resolve to the same theme.
- TcaTheme::default() is the same as TcaTheme::new(None)

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

[Unreleased]: https://github.com/carmiac/tca-rust/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/carmiac/tca-rust/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/carmiac/tca-rust/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/carmiac/tca-rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/carmiac/tca-rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/carmiac/tca-rust/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/carmiac/tca-rust/releases/tag/v0.1.0
