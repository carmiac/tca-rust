# Tca-Ratatui

Ratatui integration for [TCA](https://github.com/carmiac/tca-rust) themes. Loads and resolves TCA theme files into `ratatui::style::Color` values ready to use in widgets and styles.

## Installation

```sh
cargo add tca-ratatui
```

## Loading a single theme

### By name, slug, or file path

```rust
use tca_ratatui::TcaTheme;

// Theme name or slug — checks user themes, then built-ins
let theme = TcaTheme::new(Some("Tokyo Night"));
let theme = TcaTheme::new(Some("tokyo-night"));
let theme = TcaTheme::new(Some("nordDark"));   // flexible slug matching

// Exact file path
let theme = TcaTheme::new(Some("path/to/mytheme.toml"));

// Auto-detect from terminal dark/light mode, then fall back to built-in default
let theme = TcaTheme::new(None);
let theme = TcaTheme::default(); // same as new(None)
```

### From a TOML string

```rust
use tca_ratatui::TcaTheme;

let theme = TcaTheme::try_from(toml_str).expect("invalid theme");
```

### From a `tca_types::Theme`

```rust
use tca_ratatui::TcaTheme;
use tca_types::BuiltinTheme;

let raw = BuiltinTheme::Nord.theme();
let theme = TcaTheme::try_from(raw)?;
```

### Programmatically with `TcaThemeBuilder`

```rust
use tca_ratatui::{TcaThemeBuilder, Semantic, Ui};
use ratatui::style::Color;

let theme = TcaThemeBuilder::new()
    .semantic(Semantic {
        error:   Color::Rgb(255, 80, 80),
        warning: Color::Rgb(255, 200, 0),
        ..Default::default()
    })
    .ui(Ui {
        bg_primary: Color::Rgb(28, 28, 28),
        fg_primary: Color::Rgb(238, 238, 236),
        ..Default::default()
    })
    .build();
```

## Loading a collection of themes

```rust
use tca_ratatui::TcaThemeCursor;

// All built-in themes
let mut cursor = TcaThemeCursor::with_builtins();

// User-installed themes from XDG data dir
let mut cursor = TcaThemeCursor::with_user_themes();

// Built-ins merged with user themes (user overrides on name match)
let mut cursor = TcaThemeCursor::with_all_themes();

// From an explicit list
let mut cursor = TcaThemeCursor::new(vec![theme_a, theme_b]);
```

If you need the raw `tca_types::Theme` type instead, the generic `ThemeCursor<T>` is also re-exported:

```rust
use tca_ratatui::ThemeCursor;
use tca_types::Theme;

let mut cursor: ThemeCursor<Theme> = ThemeCursor::with_builtins();
```

## Theme cursor navigation

```rust
use tca_ratatui::{TcaTheme, TcaThemeCursor};

let mut cursor = TcaThemeCursor::with_builtins();

// Current theme without moving
let theme: &TcaTheme = cursor.peek().unwrap();

// Advance (wraps at end)
let theme: &TcaTheme = cursor.next().unwrap();

// Retreat (wraps at start)
let theme: &TcaTheme = cursor.prev().unwrap();

// Inspect without moving
println!("Showing: {}", cursor.peek().unwrap().meta.name);
println!("{} themes available", cursor.len());

// Iterate all themes without cycling
for theme in cursor.themes() {
    println!("{}", theme.meta.name);
}
```

## Using theme colors

```rust
use ratatui::style::Style;

let normal    = Style::default().fg(theme.ui.fg_primary).bg(theme.ui.bg_primary);
let secondary = Style::default().fg(theme.ui.fg_secondary);
let muted     = Style::default().fg(theme.ui.fg_muted);
let border    = Style::default().fg(theme.ui.border_primary);
let selection = Style::default().bg(theme.ui.selection_bg).fg(theme.ui.selection_fg);
let error     = Style::default().fg(theme.semantic.error);
let warning   = Style::default().fg(theme.semantic.warning);
let ansi_red  = Style::default().fg(theme.ansi.red);

// Palette ramps (if the theme defines them)
if let Some(ramp) = theme.palette.get_ramp("neutral") {
    let dark  = ramp.get(0); // darkest
    let light = ramp.get(ramp.len() - 1); // lightest
}
```

## `TcaTheme` fallback behavior

`TcaTheme::new()` resolution order:

1. User theme files — `~/.local/share/tca/themes/<name>.toml`, or an exact file path
2. Built-in themes — always available, no installation required
3. Auto-detect — dark or light built-in based on the terminal's background color

If `semantic` or `ui` color references can't be resolved, they fall back to sensible ratatui named colors (e.g. `Color::Red` for `semantic.error`).

## Built-in themes

| Slug               | Style |
| ------------------ | ----- |
| `catppuccin-mocha` | dark  |
| `cyberpunk`        | dark  |
| `dracula`          | dark  |
| `everforest-dark`  | dark  |
| `gruvbox-dark`     | dark  |
| `mono`             | dark  |
| `nord`             | dark  |
| `one-dark`         | dark  |
| `rose-pine`        | dark  |
| `solarized-light`  | light |
| `tokyo-night`      | dark  |

## Color sections

```
TcaTheme
├── meta      — name, slug, author, version, description, dark
├── ansi      — 16 ANSI colors (black, red, … bright_white) as Color::Rgb
├── semantic  — error, warning, info, success, highlight, link
├── ui        — bg_primary/secondary, fg_primary/secondary/muted,
│               border_primary/muted, cursor_primary/muted,
│               selection_bg/fg
├── palette   — named color ramps (e.g. neutral.0–7, red.0–2)
└── base16    — base00–base0F if the theme defines them
```

## Features

| Feature | Default | Description |
| --- | --- | --- |
| `fs` | enabled | File I/O, `TcaTheme::new()`, `TcaThemeCursor`, TOML parsing |
| `widgets` | enabled | `ColorPicker` widget |

```toml
# Builder-only, no file I/O or widgets
tca-ratatui = { version = "0.3", default-features = false }

# With widgets only
tca-ratatui = { version = "0.3", features = ["widgets"] }
```

## Examples

```bash
# Interactive theme browser (cycles through all themes)
cargo run --example picker --features widgets

# Browse themes from a directory
cargo run --example picker --features widgets -- path/to/themes/

# Browse built-ins only
cargo run --example picker --features widgets -- --builtin
```

## License

MIT
