# tca

CLI tool for validating and exporting [TCA](https://github.com/carmiac/tca-rust) themes.

## Installation

```sh
cargo install tca-cli
```

Or build from source:

```sh
git clone https://github.com/carmiac/tca-rust
cd tca-rust
cargo install --path tca
```

## Usage

```sh
tca <COMMAND> [OPTIONS]
```

### Commands

#### `validate`

Validate a TCA theme file, checking hex color syntax, reference resolution, and WCAG contrast ratios.

```sh
tca validate <THEME>
tca validate path/to/theme.toml
tca validate my-theme          # looks up my-theme.toml in the themes directory
```

#### `export`

Export a theme to a target application's config format.

```sh
tca export <THEME> <FORMAT>
tca export my-theme kitty
tca export path/to/theme.toml alacritty --output ~/.config/alacritty/colors.toml
```

**Supported formats:**

| Format     | Description                        |
|------------|------------------------------------|
| `kitty`    | Kitty terminal color scheme        |
| `alacritty`| Alacritty terminal color scheme    |
| `base16`   | Base16 YAML scheme                 |
| `vim`      | Vim colorscheme                    |
| `helix`    | Helix editor theme                 |
| `starship` | Starship prompt palette            |
| `vscode`   | VS Code color theme                |
| `iterm2`   | iTerm2 color preset                |
| `tmux`     | tmux color configuration           |

#### `list`

List all installed themes from the shared themes directory (`$XDG_DATA_HOME/tca/themes/`).

```sh
tca list
```

## Theme Directory

Themes are stored as `.toml` files in `$XDG_DATA_HOME/tca/themes/` (typically `~/.local/share/tca/themes/` on Linux).
