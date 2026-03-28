# tca-ratatui

Ratatui integration for [TCA](https://github.com/carmiac/tca-rust) themes. Loads base24 YAML theme files into `ratatui::style::Color` values ready to use in widgets and styles.

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
let theme = TcaTheme::new(Some("path/to/mytheme.yaml"));

// Auto-detect from terminal dark/light mode, then fall back to built-in default
let theme = TcaTheme::new(None);
let theme = TcaTheme::default(); // same as new(None)
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

println!("Showing: {}", cursor.peek().unwrap().meta.name);
println!("{} themes available", cursor.len());

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

// Raw base24 slot access (index 0 = base00, index 8 = base08, etc.)
let darkest_bg = theme.base24[0];
let red_accent = theme.base24[8];
```

## `TcaTheme` fallback behavior

`TcaTheme::new()` resolution order:

1. User theme files — `~/.local/share/tca/themes/<name>.yaml`, or an exact file path
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
| `nord-dark`        | dark  |
| `one-dark`         | dark  |
| `rose-pine`        | dark  |
| `solarized-light`  | light |
| `tokyo-night`      | dark  |

## Color sections

```
TcaTheme
├── meta    — name, author, dark
├── ansi    — 16 ANSI colors (black, red, … bright_white) as Color::Rgb
├── semantic — error, warning, info, success, highlight, link
├── ui      — bg_primary/secondary, fg_primary/secondary/muted,
│              border_primary/muted, cursor_primary/muted,
│              selection_bg/fg
└── base24  — [Color; 24] raw base24 slot colors (base00–base17)
```

## Features

| Feature | Default | Description |
| --- | --- | --- |
| `fs` | enabled | File I/O, `TcaTheme::new()`, `TcaThemeCursor`, base24 YAML parsing |
| `widgets` | enabled | `ColorPicker` widget |

```toml
# Builder-only, no file I/O or widgets
tca-ratatui = { version = "0.4", default-features = false }

# With widgets only
tca-ratatui = { version = "0.4", features = ["widgets"] }
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
