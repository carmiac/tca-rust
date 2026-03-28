# TCA Rust

[![CI](https://github.com/carmiac/tca-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/carmiac/tca-rust/actions) [![tca-types on crates.io](https://img.shields.io/crates/v/tca-types?label=tca-types)](https://crates.io/crates/tca-types) [![tca-ratatui on crates.io](https://img.shields.io/crates/v/tca-ratatui?label=tca-ratatui)](https://crates.io/crates/tca-ratatui) [![tca-cli on crates.io](https://img.shields.io/crates/v/tca-cli?label=tca-cli)](https://crates.io/crates/tca-cli) [![docs.rs](https://img.shields.io/docsrs/tca-types?label=docs)](https://docs.rs/tca-types) [![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Terminal Colors Architecture (TCA) for Rust: consistent, user-configurable theming for terminal applications.

## For Developers

Add one line to your app and get beautiful theming with zero configuration:

```rust
let theme = TcaTheme::default();
```

Your app immediately works with 11 built-in themes and any theme the user installs. You don't have to ship themes or write config parsers - TCA handles it.

```rust
// Use resolved colors directly in ratatui
let style = Style::default()
    .fg(theme.ui.fg_primary)
    .bg(theme.ui.bg_primary);

let error_style = Style::default().fg(theme.semantic.error);
let border_style = Style::default().fg(theme.ui.border_primary);
```

## For Users

Drop any `.yaml` theme file into `~/.local/share/tca/themes/` and it becomes available to every TCA-powered app on the system with no per-app configuration needed.

```bash
# Install a theme from the tca-themes collection
cp tokyo-night.yaml ~/.local/share/tca/themes/

# Your app picks it up automatically
myapp --theme tokyo-night
```

Browse available themes at [tca-themes](https://github.com/tinted-theming/schemes/tree/spec-0.11/base24).

## How Theme Resolution Works

`TcaTheme::new(Some("name"))` tries each step in order, falling back gracefully:

1. User theme files - searches `~/.local/share/tca/themes/<name>.yaml`, or accepts an exact file path
1. Built-in themes - catppuccin-mocha, cyberpunk, dracula, everforest-dark, gruvbox-dark, mono, nord, one-dark, rose-pine, solarized-light, tokyo-night
1. User preference - reads `~/.config/tca/tca.toml` for a configured default
1. Auto-detect - picks a dark or light built-in based on the terminal's background color

Passing `None` skips steps 1-2 and goes straight to the user's preference or auto-detection.

## Packages

| Crate                           | Purpose                                   |
| ------------------------------- | ----------------------------------------- |
| **[tca-ratatui](tca-ratatui/)** | Ratatui integration - `TcaTheme`, widgets |
| **[tca-types](tca-types/)**     | Core types and base24 YAML parsing        |
| **[tca-cli](tca-cli/)**         | CLI — `validate`, `add`, `list`, `init`   |

## Getting Started

```toml
[dependencies]
tca-ratatui = "0.6"
```

```rust
use tca_ratatui::TcaTheme;
use ratatui::style::Style;

let theme = TcaTheme::new(None);
```

That's it. See [tca-ratatui](tca-ratatui/) for the full guide.

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```

## Related

- [tca-themes](https://github.com/carmiac/tca-themes) - community theme collection
- [tca-go](https://github.com/carmiac/tca-go) - Go implementation
- [tca-python](https://github.com/carmiac/tca-python) - Python implementation

## License

MIT
