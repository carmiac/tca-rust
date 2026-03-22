# Tca-Types

Core type definitions for Terminal Colors Architecture (TCA).

Defines the `Theme` struct that TOML theme files deserialize into, color reference resolution, hex utilities, built-in themes, and a cycling cursor for navigating sets of themes.

## Installation

```sh
cargo add tca-types
```

## Loading a Theme

### From a TOML String

```rust
use tca_types::Theme;

let theme: Theme = toml::from_str(toml_str)?;
```

### From a File or Theme Name (Requires `fs` Feature)

```rust
use tca_types::Theme;

// Exact file path
let theme = Theme::from_name(Some("~/.local/share/tca/themes/nord.toml"));

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

## Resolving Color References

```rust
let theme: Theme = toml::from_str(toml_str)?;

// Resolve any reference format to #RRGGBB
let hex = theme.resolve("palette.neutral.2"); // Some("#24283b")
let hex = theme.resolve("ansi.bright_red");   // Some("#f7768e")
let hex = theme.resolve("base16.base08");     // Some("#f7768e")
let hex = theme.resolve("#ff0000");           // Some("#ff0000")
let hex = theme.resolve("ansi.unknown");      // None
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
├── meta     (required) — name, slug, author, version, description, dark
├── ansi     (required) — 16 direct #RRGGBB values: black … bright_white
├── semantic (required) — error, warning, info, success, highlight, link
├── ui       (required) — bg, fg, border, cursor, selection (each with sub-fields)
├── palette  (optional) — named color ramps, e.g. neutral = ["#111", "#222", …]
└── base16   (optional) — base00–base0F mappings
```

## Color Reference Formats

Fields outside `[ansi]` may use any of these formats:

| Format            | Example             | Resolves via                   |
| ----------------- | ------------------- | ------------------------------ |
| Direct hex        | `#ff0000`           | returned as-is                 |
| ANSI reference    | `ansi.bright_red`   | `[ansi]` section               |
| Palette reference | `palette.neutral.2` | `[palette]` section (0-based)  |
| Base16 reference  | `base16.base08`     | `[base16]` section (recursive) |

`[ansi]` values must always be direct `#RRGGBB` hex — references are not allowed there.

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
# Without file I/O (built-ins and TOML parsing still available)
tca-types = { version = "0.4", default-features = false }
```

## License

MIT
