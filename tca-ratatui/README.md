# tca-ratatui

Ratatui adapter for Terminal Colors Architecture (TCA) themes.

## Overview

This crate provides types and widgets for using TCA themes in Ratatui applications. It converts TCA theme files (TOML) into Ratatui-compatible `Color` values.

## Installation

```toml
[dependencies]
tca-ratatui = "0.1"
```

## Features

- `loader` (default): File I/O and theme loading
- `widgets`: Color picker widget

Disable default features for no-std environments or when using pre-loaded themes:

```toml
tca-ratatui = { version = "0.1", default-features = false }
```

## Usage

### Loading Themes

```rust
use tca_ratatui::TcaTheme;
use ratatui::style::Style;

let theme = TcaTheme::from_file("theme.toml")?;

// Access color sections directly
let error_style = Style::default().fg(theme.ansi.red);
let ui_bg = Style::default().bg(theme.ui.bg_primary);
let semantic_warn = Style::default().fg(theme.semantic.warning);
```

### Color Picker Widget

```rust
use tca_ratatui::ColorPicker;

let picker = ColorPicker::new(&theme)
    .title("Theme Preview")
    .instructions("Arrow keys to navigate");

frame.render_widget(picker, area);
```

## Type Hierarchy

```
TcaTheme
 - Meta       - Name, author, version
 - Palette    - Neutral and hue ramps (red, blue, etc.)
 - Ansi       - 16 ANSI colors
 - Semantic   - Error, warning, success, info, highlight, link
 - Ui         - Background, foreground, border, cursor, selection
```

## Defaults

`Semantic` and `Ui` sections use sensible ANSI color fallbacks when missing from theme files:

- `Semantic`: Red for error, yellow for warning, etc.
- `Ui`: Black background, white foreground, etc.

## Examples

See `examples/` directory:

- `basic.rs` - Theme loading and color access
- `picker.rs` - Interactive theme picker application

Run with:

```bash
cargo run --example picker -- path/to/themes/
```

## TCA Specification

For theme file format details, see the [TCA specification](https://github.com/carmiac/tca).

## License

MIT
