# TCA Rust

Terminal Colors Architecture (TCA) implementation for Rust.

This repository contains the complete Rust ecosystem for TCA themes, including the core CLI tool, type definitions, theme loader, and Ratatui UI library.

## Packages

This workspace contains 4 crates:

- **[tca-types](tca-types/)** - Core types and data structures
- **[tca-loader](tca-loader/)** - XDG-compliant theme loading
- **[tca](tca/)** - CLI tool for validation and export
- **[tca-ratatui](tca-ratatui/)** - Ratatui UI library with widgets

## Quick Start

### Install the CLI Tool

```bash
cargo install --path tca
```

### Use in Your Project

```toml
# For basic types
[dependencies]
tca-types = "0.1"

# For theme loading
[dependencies]
tca-loader = "0.1"

# For Ratatui apps
[dependencies]
tca-ratatui = "0.1"
```

## Development

```bash
# Build all crates
cargo build --workspace

# Test all crates
cargo test --workspace

# Run clippy
cargo clippy --workspace

# Build CLI tool
cargo build --release -p tca
```

## Related Projects

- [tca-themes](https://github.com/carmiac/tca-themes) - Theme collection
- [tca-go](https://github.com/carmiac/tca-go) - Go implementation
- [tca-python](https://github.com/carmiac/tca-python) - Python implementation

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of notable changes.

## License

MIT
