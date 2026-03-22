# Tca-Cli

CLI tool for validating and exporting [TCA](https://github.com/carmiac/tca-rust) theme files.

## Installation

```sh
cargo install tca-cli
```

Or from source:

```sh
git clone https://github.com/carmiac/tca-rust
cd tca-rust
cargo install --path tca-cli
```

## Commands

### `validate`

Checks a theme file for correctness: hex color syntax, color reference resolution, and WCAG contrast ratios.

```sh
tca validate path/to/theme.toml
tca validate nord                    # looks up nord.toml in the themes directory
tca validate nord --schema schema.json
```

### `export`

Converts a theme to another application's config format.

```sh
tca export nord kitty
tca export path/to/theme.toml alacritty --output ~/.config/alacritty/colors.toml
```

**Supported formats:**

| Format      | Output                          |
| ----------- | ------------------------------- |
| `kitty`     | Kitty terminal color scheme     |
| `alacritty` | Alacritty terminal color scheme |
| `base16`    | Base16 YAML scheme              |
| `vim`       | Vim colorscheme                 |
| `helix`     | Helix editor theme              |
| `starship`  | Starship prompt palette         |
| `vscode`    | VS Code color theme JSON        |
| `iterm2`    | iTerm2 color preset plist       |
| `tmux`      | tmux status/pane color config   |

### `list`

Lists all installed themes.

```sh
tca list
```

## Theme Directory

Themes are `.toml` files in `~/.local/share/tca-themes/` (or `$XDG_DATA_HOME/tca-themes/`).

## Roadmap

- Init command to create default config and add default theme files
- Download and install themes
- Import base16/24 themes into TCA format
- Export to base24

## License

MIT
