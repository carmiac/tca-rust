# Tca-Types

Core type definitions for Terminal Colors Architecture (TCA).

Parses [base24](https://github.com/tinted-theming/base24/) YAML theme files and derives the TCA semantic layer (ANSI, semantic, and UI color roles) from the base24 slot mappings defined in the [TCA specification](https://github.com/carmiac/tca-themes/blob/main/SPECIFICATION.md).

## Installation

```sh
cargo add tca-types
```

## Loading a Theme

### From a base24 YAML String

```rust
use tca_types::Theme;

let theme = Theme::from_base24_str(yaml_str)?;
```

### From a File or Theme Name (Requires `fs` Feature)

```rust
use tca_types::Theme;

// Exact file path to a base24 YAML file
let theme = Theme::from_name(Some("~/.local/share/tca/themes/nord-dark.yaml"));

// Theme name or slug — resolves user themes then built-ins
let theme = Theme::from_name(Some("Tokyo Night"));
let theme = Theme::from_name(Some("tokyo-night"));

// Auto-detect from terminal dark/light mode
let theme = Theme::from_name(None);
```

### From Built-in Themes (no Files Needed)

```rust
use tca_types::BuiltinTheme;

let theme = BuiltinTheme::TokyoNight.theme();
let theme = BuiltinTheme::Nord.theme();
let theme = BuiltinTheme::default().theme();       // Dracula (dark)
let theme = BuiltinTheme::default_light().theme(); // SolarizedLight
```

## Loading a Collection of Themes

```rust
use tca_types::{all_themes, all_user_themes, all_from_dir, BuiltinTheme};

// All user-installed themes from XDG data dir
let themes = all_user_themes();

// Built-ins merged with user themes (user overrides built-in on name match)
let themes = all_themes();

// All themes from a specific directory
let themes = all_from_dir("/path/to/themes");

// Iterate over all built-ins
for builtin in BuiltinTheme::iter() {
    let theme = builtin.theme();
    println!("{}", theme.meta.name);
}
```

## Theme Cursor

`ThemeCursor<T>` is a cycling cursor for navigating a collection of themes. `peek()` shows the current theme; `next()` and `prev()` move the cursor (both wrap).

```rust
use tca_types::{ThemeCursor, Theme};

// From built-in themes
let mut cursor = ThemeCursor::with_builtins();

// From user themes
let mut cursor = ThemeCursor::with_user_themes();

// Built-ins + user themes merged
let mut cursor = ThemeCursor::with_all_themes();

// From an explicit list
let mut cursor = ThemeCursor::new(vec![theme_a, theme_b, theme_c]);

// Navigate
let current: &Theme = cursor.peek().unwrap();
let next:    &Theme = cursor.next().unwrap(); // advances, wraps at end
let prev:    &Theme = cursor.prev().unwrap(); // retreats, wraps at start

// Inspect without moving
println!("Current: {}", cursor.peek().unwrap().meta.name);
println!("{} themes total", cursor.len());

// Iterate all themes without cycling
for theme in cursor.themes() {
    println!("{}", theme.meta.name);
}
```

## Accessing Colors

All color fields on `Theme` contain resolved `#rrggbb` hex strings — no further resolution needed.

```rust
let theme = Theme::from_name(Some("Tokyo Night"));

// Semantic colors
println!("{}", theme.semantic.error);    // e.g. "#f7768e"
println!("{}", theme.semantic.warning);  // e.g. "#ff9e64"
println!("{}", theme.semantic.info);     // e.g. "#7dcfff"
println!("{}", theme.semantic.success);  // e.g. "#9ece6a"
println!("{}", theme.semantic.link);     // e.g. "#7aa2f7"

// UI colors
println!("{}", theme.ui.bg.primary);     // e.g. "#1a1b26"
println!("{}", theme.ui.fg.primary);     // e.g. "#c0caf5"
println!("{}", theme.ui.border.primary); // e.g. "#3b4261"

// ANSI colors
println!("{}", theme.ansi.red);          // e.g. "#f7768e"
println!("{}", theme.ansi.bright_blue);  // e.g. "#7aa2f7"

// Raw base24 slots for interoperability with other base24 tools
println!("{}", theme.base24.base08);     // e.g. "#f7768e"
```

## Resolving Color References

```rust
// Resolve a direct hex passthrough
let hex = theme.resolve("#ff0000");        // Some("#ff0000")

// Resolve a base24 slot reference (case-insensitive)
let hex = theme.resolve("base24.base08"); // Some("#f7768e")
let hex = theme.resolve("base24.BASE0E"); // Some("#bb9af7")
let hex = theme.resolve("base24.unknown"); // None
```

## Hex Utilities

```rust
use tca_types::hex_to_rgb;

let (r, g, b) = hex_to_rgb("#ff5533")?; // (255, 85, 51)
let (r, g, b) = hex_to_rgb("ff5533")?;  // also works without #
```

## Type Structure

```
Theme
├── meta     — name, author, dark (bool, derived from luminance)
├── ansi     — 16 #rrggbb values: black … bright_white  (derived from base24)
├── semantic — error, warning, info, success, highlight, link  (derived from base24)
├── ui       — bg, fg, border, cursor, selection  (derived from base24)
└── base24   — raw slots base00–base17 for direct interoperability
```

## Theme File Format

TCA themes are [base24](https://github.com/tinted-theming/base24/) YAML files.

**File naming:** `kebab-case.yaml`  
**Install location:** `$XDG_DATA_HOME/tca/themes/` (default: `~/.local/share/tca/themes/`)

```yaml
scheme: "Tokyo Night"
author: "Enkia"
variant: "dark"
base00: "1a1b26"
base01: "16161e"
base02: "2f3549"
base03: "444b6a"
base04: "787c99"
base05: "a9b1d6"
base06: "cbccd1"
base07: "d5d6db"
base08: "f7768e"
base09: "ff9e64"
base0A: "e0af68"
base0B: "9ece6a"
base0C: "7dcfff"
base0D: "7aa2f7"
base0E: "bb9af7"
base0F: "f7768e"
base10: "1a1b26"
base11: "000000"
base12: "ff7a93"
base13: "ffd479"
base14: "b9f27c"
base15: "7da6ff"
base16: "7da6ff"
base17: "bb9af7"
```

## Base24 → TCA Mapping

| TCA field             | base24 slot |
| --------------------- | ----------- |
| `ansi.black`          | base00      |
| `ansi.red`            | base08      |
| `ansi.green`          | base0b      |
| `ansi.yellow`         | base0a      |
| `ansi.blue`           | base0d      |
| `ansi.magenta`        | base0e      |
| `ansi.cyan`           | base0c      |
| `ansi.white`          | base05      |
| `ansi.bright_black`   | base03      |
| `ansi.bright_red`     | base12      |
| `ansi.bright_green`   | base14      |
| `ansi.bright_yellow`  | base13      |
| `ansi.bright_blue`    | base16      |
| `ansi.bright_magenta` | base17      |
| `ansi.bright_cyan`    | base15      |
| `ansi.bright_white`   | base07      |
| `semantic.error`      | base08      |
| `semantic.warning`    | base09      |
| `semantic.info`       | base0c      |
| `semantic.success`    | base0b      |
| `semantic.highlight`  | base0e      |
| `semantic.link`       | base0d      |
| `ui.bg.primary`       | base00      |
| `ui.bg.secondary`     | base01      |
| `ui.fg.primary`       | base05      |
| `ui.fg.secondary`     | base06      |
| `ui.fg.muted`         | base04      |
| `ui.border.primary`   | base02      |
| `ui.border.muted`     | base01      |
| `ui.cursor.primary`   | base05      |
| `ui.cursor.muted`     | base04      |
| `ui.selection.bg`     | base02      |
| `ui.selection.fg`     | base05      |

## Built-in Themes

| Variant           | Slug               | Style |
| ----------------- | ------------------ | ----- |
| `CatppuccinMocha` | `catppuccin-mocha` | dark  |
| `Cyberpunk`       | `cyberpunk`        | dark  |
| `Dracula`         | `dracula`          | dark  |
| `EverforestDark`  | `everforest-dark`  | dark  |
| `GruvboxDark`     | `gruvbox-dark`     | dark  |
| `Mono`            | `mono`             | dark  |
| `Nord`            | `nord`             | dark  |
| `OneDark`         | `one-dark`         | dark  |
| `RosePine`        | `rose-pine`        | dark  |
| `SolarizedLight`  | `solarized-light`  | light |
| `TokyoNight`      | `tokyo-night`      | dark  |

## Features

| Feature | Default | Description |
| --- | --- | --- |
| `fs` | enabled | File I/O, `Theme::from_name`, `all_themes`, `ThemeCursor::with_user_themes` / `with_all_themes` |

```toml
# Without file I/O (built-ins and YAML parsing still available)
tca-types = { version = "0.5", default-features = false }
```

## License

MIT
