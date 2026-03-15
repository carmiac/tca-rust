# tca-loader

XDG-compliant theme file discovery and loading for TCA.

## Overview

Provides filesystem operations for discovering and loading TCA theme files, and reading user preferences. Used internally by `tca-ratatui` — most projects won't need to depend on this directly.

## Installation

```sh
cargo add tca-loader
```

## Theme directory

Themes are `.toml` files stored in `~/.local/share/tca-themes/` (or `$XDG_DATA_HOME/tca-themes/`). Any file placed there is automatically discoverable by all TCA-powered apps.

## API

### Discovery

```rust
use tca_loader::{get_themes_dir, list_themes, list_theme_names};

let dir   = get_themes_dir()?;          // PathBuf to the themes directory
let paths = list_themes()?;             // Vec<PathBuf> — all .toml files
let names = list_theme_names()?;        // Vec<String>  — names without extension
```

### Loading

```rust
use tca_loader::load_theme_file;

// Accepts a name ("nord"), a name with extension ("nord.toml"),
// or any file path ("/path/to/theme.toml")
let toml_str: String = load_theme_file("nord")?;

// Parse with tca-types
let theme: tca_types::Theme = toml::from_str(&toml_str)?;
```

### User preferences

```rust
use tca_loader::TcaConfig;

// Reads ~/.config/tca/tca.toml
let config = TcaConfig::load();

// Returns the best theme name for the current terminal mode
// (dark_theme, light_theme, or default_theme depending on what's configured)
if let Some(name) = config.mode_aware_theme() {
    println!("preferred theme: {name}");
}

// Persist preferences
let config = TcaConfig { default_dark_theme: Some("nord".into()), ..Default::default() };
config.store();
```

## License

MIT
