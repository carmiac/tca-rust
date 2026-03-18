//! XDG-compliant theme loader for Terminal Colors Architecture.
//!
//! Provides filesystem operations for discovering and loading TCA themes
//! from XDG data directories.

#![warn(missing_docs)]

use anyhow::{Context, Result};
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for TCA user preferences.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TcaConfig {
    /// The general default theme. Used if mode can't be detected or other options
    /// aren't defined.
    pub default_theme: Option<String>,
    /// Default dark mode theme.
    pub default_dark_theme: Option<String>,
    /// Default light mode theme.
    pub default_light_theme: Option<String>,
}

/// Returns the path to the TCA config file (`tca.toml` in the app config dir).
fn config_file_path() -> Result<PathBuf> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "TCA".to_string(),
        app_name: "tca".to_string(),
    })?;
    Ok(strategy.config_dir().join("tca.toml"))
}

impl TcaConfig {
    /// Load the user's configuration preferences.
    ///
    /// Returns [`Default`] if the config file doesn't exist or cannot be parsed.
    pub fn load() -> Self {
        let Ok(path) = config_file_path() else {
            return Self::default();
        };
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }

    /// Save the user's configuration preferences.
    pub fn store(&self) {
        let path = config_file_path().expect("Could not determine TCA config path.");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Could not create TCA config directory.");
        }
        let content = toml::to_string(self).expect("Could not serialize TCA config.");
        fs::write(&path, content).expect("Could not save TCA config.");
    }

    /// Get the best default theme, based on user preference and current terminal
    /// color mode.
    pub fn mode_aware_theme(&self) -> Option<String> {
        // Fallback order:
        // Mode preference - if None or mode can't be determined then default
        use terminal_colorsaurus::{theme_mode, QueryOptions, ThemeMode};
        match theme_mode(QueryOptions::default()).ok() {
            Some(ThemeMode::Dark) => self
                .default_dark_theme
                .clone()
                .or(self.default_theme.clone()),
            Some(ThemeMode::Light) => self
                .default_light_theme
                .clone()
                .or(self.default_theme.clone()),
            None => self.default_theme.clone(),
        }
    }
}

/// Get the themes directory path, creating it if it does not exist.
///
/// Returns `$XDG_DATA_HOME/tca-themes` (or platform equivalent).
pub fn get_themes_dir() -> Result<PathBuf> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "TCA".to_string(),
        app_name: "tca-themes".to_string(),
    })
    .unwrap();
    let data_dir = strategy.data_dir();
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir)
}

/// List all available theme files in the shared themes directory.
///
/// Returns paths to all `.toml` files in the themes directory.
pub fn list_themes() -> Result<Vec<PathBuf>> {
    let themes_dir = get_themes_dir()?;

    let mut themes = Vec::new();

    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext == "toml" {
                    themes.push(path);
                }
            }
        }
    }

    themes.sort();
    Ok(themes)
}

/// Find a theme by name.
///
/// Converts name to kebab-case and searches for `<name>.toml` in the
/// themes directory.
/// Returns the full path if found.
pub fn find_theme(name: &str) -> Result<PathBuf> {
    let themes_dir = get_themes_dir()?;

    let name = heck::AsKebabCase(name).to_string();
    // If no extension, also try with .toml appended
    let candidate = if !name.ends_with(".toml") {
        themes_dir.join(format!("{}.toml", name))
    } else {
        themes_dir.join(&name)
    };
    if candidate.exists() && candidate.is_file() {
        return Ok(candidate);
    }

    Err(anyhow::anyhow!(
        "Theme '{}' not found in {:?}.",
        name,
        themes_dir,
    ))
}

/// List all theme names (without paths or extensions).
pub fn list_theme_names() -> Result<Vec<String>> {
    let themes = list_themes()?;

    Ok(themes
        .iter()
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(String::from))
        .collect())
}

/// Load a theme file from one of these locations (searched in order):
///
/// 1. Exact path, if the argument resolves to an existing file
/// 2. Shared themes directory (`$XDG_DATA_HOME/tca/themes/`)
///
/// Returns the file contents as a string.
pub fn load_theme_file(path_or_name: &str) -> Result<String> {
    let path = Path::new(path_or_name);

    // 1. Try exact path (handles absolute paths and relative paths from cwd)
    if path.exists() && path.is_file() {
        return fs::read_to_string(path)
            .with_context(|| format!("Failed to read theme file: {:?}", path));
    }

    // 2. Try shared themes directory
    if let Ok(shared_path) = find_theme(path_or_name) {
        return fs::read_to_string(&shared_path)
            .with_context(|| format!("Failed to read theme file: {:?}", shared_path));
    }

    Err(anyhow::anyhow!(
        "Theme '{}' not found. Searched:\n\
         1. Exact path: {:?}\n\
         2. Shared themes: {:?}\n",
        path_or_name,
        path,
        get_themes_dir()?,
    ))
}

/// Load all themes from a given directory as raw [`tca_types::Theme`] values.
///
/// Entries that cannot be read or parsed are skipped with a message to stderr.
pub fn load_all_from_dir(dir: &str) -> Result<Vec<tca_types::Theme>> {
    let mut items: Vec<tca_types::Theme> = vec![];
    for entry in fs::read_dir(dir)? {
        let path = match entry {
            Err(e) => {
                eprintln!("Could not read dir entry: {}", e);
                continue;
            }
            Ok(e) => e.path(),
        };
        if path.is_file() & path.extension().is_some_and(|x| x == "toml") {
            match fs::read_to_string(&path) {
                Err(e) => {
                    eprintln!("Could not read: {:?}.\nError: {}", path, e);
                    continue;
                }
                Ok(theme_str) => match toml::from_str(&theme_str) {
                    Err(e) => {
                        eprintln!("Could not parse: {:?}.\nError: {}", path, e);
                        continue;
                    }
                    Ok(item) => items.push(item),
                },
            }
        }
    }
    Ok(items)
}

/// Load all locally installed themes from the shared theme directory.
///
/// Returns raw [`tca_types::Theme`] values. Entries that cannot be read or
/// parsed are skipped with a message to stderr.
pub fn load_all_from_theme_dir() -> Result<Vec<tca_types::Theme>> {
    let dir = get_themes_dir()?;
    let dir_str = dir
        .to_str()
        .context("Data directory path is not valid UTF-8")?;
    load_all_from_dir(dir_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_themes_dir() {
        let dir = get_themes_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.ends_with("tca-themes"));
    }

    #[test]
    fn test_list_themes() {
        let themes = list_themes().unwrap();
        // Verify that all returned paths have toml extension
        for theme_path in themes {
            let ext = theme_path.extension().and_then(|s| s.to_str());
            assert_eq!(ext, Some("toml"));
        }
    }

    #[test]
    fn test_list_theme_names() {
        let names = list_theme_names().unwrap();
        // Theme names should not have file extensions
        for name in names {
            assert!(!name.contains('.'));
        }
    }
}
