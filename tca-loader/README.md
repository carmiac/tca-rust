# tca-loader

XDG-compliant theme loader for Terminal Colors Architecture.

## Overview

Provides filesystem operations for discovering and loading TCA themes from XDG data directories. Follows the XDG Base Directory specification.

## Installation

```toml
[dependencies]
tca-loader = "0.1"
```

## Usage

### Theme Discovery

```rust
use tca_loader::{get_themes_dir, list_themes, list_theme_names};

// Get XDG theme directory path
let themes_dir = get_themes_dir()?;
// Returns: ~/.local/share/tca/themes (or equivalent)

// List all theme files
let theme_paths = list_themes()?;

// List theme names (without extensions)
let names = list_theme_names()?;
```

### Loading Themes

```rust
use tca_loader::{find_theme, load_theme};
use tca_types::Theme;

// Find theme by name (searches XDG directory)
let path = find_theme("gruvbox")?;

// Load with specific type
let theme: Theme = load_theme("gruvbox")?;

// Or load from explicit path
use tca_loader::load_theme_file;
let theme: Theme = load_theme_file(&path)?;
```

### Generic Type Support

Works with any deserializable type:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct CustomTheme {
    name: String,
    colors: Vec<String>,
}

let custom: CustomTheme = load_theme("mytheme")?;
```

## XDG Compliance

Theme directory resolution follows XDG Base Directory spec:

1. `$XDG_DATA_HOME/tca/themes/` (default: `~/.local/share/tca/themes/`)
2. Falls back to first writable location if directory doesn't exist

## License

MIT
