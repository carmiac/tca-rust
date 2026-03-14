# tca-types

Core type definitions for Terminal Colors Architecture (TCA).

## Overview

Defines the `Theme` struct that TOML theme files deserialize into, plus color reference resolution and hex utilities. Used as the serialization layer by `tca-loader` and `tca-ratatui`.

## Installation

```toml
[dependencies]
tca-types = "0.1"
```

## Usage

```rust
use tca_types::Theme;

// Deserialize from TOML
let theme: Theme = toml::from_str(toml_str)?;

// Resolve color references to #RRGGBB
let hex = theme.resolve("palette.neutral.2"); // Some("#24283b")
let hex = theme.resolve("ansi.bright_red");   // Some("#f7768e")
let hex = theme.resolve("base16.base08");     // Some("#f7768e")
let hex = theme.resolve("#ff0000");           // Some("#ff0000")

// Hex parsing
use tca_types::hex_to_rgb;
let (r, g, b) = hex_to_rgb("#ff5533")?;      // (255, 85, 51)
```

## Type structure

```
Theme
├── meta     (required) — name, slug, author, version, description, dark
├── ansi     (required) — 16 direct #RRGGBB values: black … bright_white
├── semantic (required) — error, warning, info, success, highlight, link
├── ui       (required) — bg, fg, border, cursor, selection (each with sub-fields)
├── palette  (optional) — named color ramps, e.g. neutral = ["#111", "#222", …]
└── base16   (optional) — base00–base0F mappings
```

## Color references

Fields outside `[ansi]` may use any of these formats:

| Format            | Example             | Resolves via                   |
| ----------------- | ------------------- | ------------------------------ |
| Direct hex        | `#ff0000`           | returned as-is                 |
| ANSI reference    | `ansi.bright_red`   | `[ansi]` section               |
| Palette reference | `palette.neutral.2` | `[palette]` section (0-based)  |
| Base16 reference  | `base16.base08`     | `[base16]` section (recursive) |

`[ansi]` values must always be direct `#RRGGBB` hex — references are not allowed there.

## Built-in themes

`BuiltinTheme` provides 11 embedded themes (no files needed):

```rust
use tca_types::BuiltinTheme;

let theme = BuiltinTheme::Dracula.theme();
let theme = BuiltinTheme::TokyoNight.theme();
let theme = BuiltinTheme::default().theme(); // Dracula
```

## License

MIT
