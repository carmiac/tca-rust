# tca-ratatui

Ratatui integration for [TCA](https://github.com/carmiac/tca-rust) themes. Loads base24 YAML theme files into ready-to-use `ratatui::style::Style` values and `ratatui::style::Color` values.

## Installation

```sh
cargo add tca-ratatui
```

## Quick Start - StyleSet

The easiest way to theme a ratatui app is with [`StyleSet`]. It pre-builds all the common styles you need from the user's configured theme:

```rust
use tca_ratatui::StyleSet;
use ratatui::widgets::{Block, Borders, Paragraph};

fn render(frame: &mut ratatui::Frame) {
    let styles = StyleSet::default(); // user's configured theme, or built-in fallback

    let block = Block::default()
        .borders(Borders::ALL)
        .style(styles.border);

    let text = Paragraph::new("Hello, world!")
        .block(block)
        .style(styles.primary);

    frame.render_widget(text, frame.area());
}
```

Load by name, or cycle through available themes:

```rust
use tca_ratatui::{StyleSet, StyleSetCursor};

// Load a specific theme by name (any case format works)
let styles = StyleSet::from_name("tokyo-night");
let styles = StyleSet::from_name("Tokyo Night");

// Cycle through all installed + built-in themes
let mut cursor = StyleSetCursor::with_all_themes();
cursor.set_current("nord-dark");
let styles = cursor.peek().unwrap_or_default();
cursor.next();
cursor.prev();
```

Available styles on `StyleSet`:

| Field          | Use for                             |
| -------------- | ----------------------------------- |
| `primary`      | Normal text, main content           |
| `secondary`    | Sidebars, panels, secondary content |
| `muted`        | Disabled, hints, placeholders       |
| `selection`    | Selected items, highlighted rows    |
| `cursor`       | Active cursor                       |
| `cursor_muted` | Inactive cursor                     |
| `border`       | Active/focused borders              |
| `border_muted` | Inactive borders                    |
| `error`        | Error messages                      |
| `warning`      | Warnings                            |
| `info`         | Informational text, spinners        |
| `success`      | Success messages                    |
| `highlight`    | Important text, search matches      |
| `link`         | URLs, hyperlinks                    |

Custom styles can be added at runtime with `insert_custom` / `get_custom`.

## Lower-Level Access - TcaTheme

For direct access to typed colors (e.g. for custom style composition):

```rust
use tca_ratatui::TcaTheme;
use ratatui::style::Style;

let theme = TcaTheme::from_name("tokyo-night");
let theme = TcaTheme::default(); // user's configured default

let error  = Style::default().fg(theme.semantic.error);
let border = Style::default().fg(theme.ui.border_primary);
let bg     = Style::default().bg(theme.ui.bg_primary);
let red    = theme.ansi.red;

// Raw base24 slots (index 0 = base00, index 8 = base08, etc.)
let darkest_bg = theme.base24[0];
```

## Theme Resolution Order

`TcaTheme::from_name("name")` / `TcaTheme::default()`:

1. User theme files: `~/.local/share/tca/themes/<name>.yaml`, or an exact file path
2. Built-in themes: always available, no installation required
3. User config default: `~/.config/tca/tca.toml`
4. Built-in default (`catppuccin-mocha`)

## Built-in Themes

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

## Color Sections

```
TcaTheme
├── meta     — name, author, dark
├── ansi     — 16 ANSI colors (black, red, … bright_white)
├── semantic — error, warning, info, success, highlight, link
├── ui       — bg_primary/secondary, fg_primary/secondary/muted,
│               border_primary/muted, cursor_primary/muted,
│               selection_bg/fg
└── base24   — [Color; 24] raw base24 slot colors (base00–base17)
```

## Features

| Feature   | Default | Description                                  |
| --------- | ------- | -------------------------------------------- |
| `fs`      | enabled | File I/O, `from_name`, cursors, YAML parsing |
| `widgets` | enabled | `ColorPicker` widget                         |

```toml
# Builder-only, no file I/O or widgets
tca-ratatui = { version = "0.7", default-features = false }

# File I/O only, no widgets
tca-ratatui = { version = "0.7", features = ["fs"] }
```

## Examples

```bash
# StyleSet demo: shows all styles, cycles themes with ← →
cargo run --example styleset

# Interactive theme color browser
cargo run --example picker --features widgets

# Browse themes from a directory
cargo run --example picker --features widgets -- path/to/themes/
```

## License

MIT
