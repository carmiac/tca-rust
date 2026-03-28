# tca-cli

CLI tool for managing [TCA](https://github.com/carmiac/tca-themes) themes. Themes are [base24](https://github.com/tinted-theming/base24/) YAML files stored in `$XDG_DATA_HOME/tca/themes/` (default: `~/.local/share/tca/themes/`).

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

### `tca validate <theme>`

Validates a base24 YAML theme file. Checks that the file parses correctly and reports WCAG contrast warnings/errors for all UI and semantic color pairs.

```sh
tca validate path/to/theme.yaml
tca validate nord-dark          # looks up nord-dark.yaml in the themes directory
```

Exit code is non-zero if any contrast errors are found.

### `tca list`

Lists all available themes: built-in themes and any themes installed in the user themes directory.

```sh
tca list
```

### `tca add [OPTIONS] [THEME]...`

Adds one or more themes to the user themes directory. Each argument can be:

- A path to a `.yaml` theme file.
- A path to a directory, all `.yaml` files in it are copied.
- A theme name, downloaded from the remote theme repository.

```sh
tca add path/to/theme.yaml
tca add path/to/themes/dir/
tca add nord-dark
tca add "Tokyo Night"          # case-insensitive, any common casing works

tca add --all                  # download every theme from the repository
```

### `tca init [OPTIONS]`

Creates a default config file and writes the built-in themes to the user themes directory.

```sh
tca init               # write config + install built-in themes
tca init --all         # write config + download all themes from the repository
tca init --none        # write config only, no theme files
tca init --force       # overwrite existing config file
```

### `tca config [SUBCOMMAND]`

Shows or sets user configuration. Config is stored at `$XDG_CONFIG_HOME/tca/tca.toml` (default: `~/.config/tca/tca.toml`).

```sh
tca config                         # show current config
tca config show                    # same as above
tca config set default "Dracula"
tca config set default_dark  "Tokyo Night"
tca config set default_light "Solarized Light"
```

Theme names can be in any standard case (`"Tokyo Night"`, `"tokyo-night"`, `"TokyoNight"` all work).

## Theme Directory

Themes are base24 YAML files (`*.yaml`) in `$XDG_DATA_HOME/tca/themes/` (default `~/.local/share/tca/themes/`).

Use `tca init` or `tca add` to populate this directory.

## License

MIT
