# tca-types

Core type definitions for Terminal Colors Architecture (TCA).

## Overview

Provides domain types for representing TCA themes. This crate is format-agnostic and contains no I/O or rendering logic.

## Installation

```toml
[dependencies]
tca-types = "0.1"
```

## Usage

```rust
use tca_types::Theme;

// Themes are typically deserialized from YAML
let theme: Theme = serde_yaml::from_str(yaml_content)?;

// Resolve palette references to hex colors
let color = theme.resolve("palette.red.5")?; // Returns "#ff5555"
let direct = theme.resolve("#ff0000")?;       // Returns "#ff0000"

// Convert hex to RGB
use tca_types::hex_to_rgb;
let (r, g, b) = hex_to_rgb("#ff5555")?;
```

## Type Structure

```
Theme
 - Meta       - Name, author, version, description
 - Palette    - Neutral and hue ramps (red, blue, green, etc.)
 - Ansi       - 16 ANSI color definitions
 - Base16     - Optional Base16 scheme
 - Semantic   - Optional semantic colors (error, warning, etc.)
 - Ui         - Optional UI colors (background, foreground, etc.)
```

## Color References

TCA supports two reference formats:

- **Hex colors:** `#ff0000`
- **Palette references:** `palette.red.5`

The `Theme::resolve()` method resolves references to hex values.

## Serialization

All types derive `Serialize` and `Deserialize` for use with serde.

## License

MIT
