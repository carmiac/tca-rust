# tca-ratatui

Ratatui integration for [TCA](https://github.com/carmiac/tca-rust) themes. One call gets you a fully resolved theme from the user's installed themes, built-ins, or a sensible auto-detected default.

## Quick start

```sh
cargo add tca-ratatui
```

```rust
use tca_ratatui::TcaTheme;
use ratatui::style::Style;

// Load by name, file path, or let the library choose
let theme = TcaTheme::new(Some("tokyo-night"));
let theme = TcaTheme::new(Some("path/to/theme.toml"));
let theme = TcaTheme::new(None); // auto-detects dark/light
let theme = TcaTheme::default();

// Use the colors
let normal = Style::default().fg(theme.ui.fg_primary).bg(theme.ui.bg_primary);
let error  = Style::default().fg(theme.semantic.error);
let border = Style::default().fg(theme.ui.border_primary);
let ansi   = Style::default().fg(theme.ansi.cyan);
```

## How `TcaTheme::new()` resolves a theme

Each step falls through to the next on failure:

1. **User theme files** — `~/.local/share/tca-themes/<name>.toml`, or an exact file path
2. **Built-in themes** — always available, no installation required
3. **User preference** — configured default in `~/.config/tca/tca.toml`
4. **Auto-detect** — picks dark or light built-in from the terminal's background color

This means your app works out of the box and gives users full control without any extra code on your part.

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
├── meta      — name, author, version, description
├── ansi      — 16 ANSI colors (black, red, … bright_white)
├── semantic  — error, warning, info, success, highlight, link
├── ui        — bg_primary/secondary, fg_primary/secondary/muted,
│               border_primary/muted, cursor_primary/muted,
│               selection_bg/fg
├── palette   — named color ramps (e.g. neutral.0–7, red.0–2)
└── base16    — base00–base0F if the theme defines them
```

Fallbacks: if `semantic` or `ui` colors can't be resolved from the theme file, they fall back to sensible ratatui named colors (e.g. `Color::Red` for `semantic.error`).

## Programmatic themes

Use `TcaThemeBuilder` to construct a theme entirely in code:

```rust
use tca_ratatui::{TcaThemeBuilder, Semantic};
use ratatui::style::Color;

let theme = TcaThemeBuilder::new()
    .semantic(Semantic {
        error: Color::Rgb(255, 80, 80),
        ..Default::default()
    })
    .build();
```

## Loading from a TOML string

```rust
let theme = TcaTheme::try_from(toml_str).expect("invalid theme");
```

## Features

| Feature   | Default | Description                               |
| --------- | ------- | ----------------------------------------- |
| `loader`  | X       | File I/O, `TcaTheme::new()`, TOML parsing |
| `widgets` | -       | `ColorPicker` widget                      |

```toml
# Widgets
tca-ratatui = { version = "0.2", features = ["widgets"] }
```

## Examples

```bash
# Interactive theme browser
cargo run --example picker --features widgets

# Pass a directory of .toml files
cargo run --example picker --features widgets -- path/to/themes/

# Browse built-ins
cargo run --example picker --features widgets -- --builtin
```

## License

MIT
