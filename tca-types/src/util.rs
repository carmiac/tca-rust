//! Utilities to help load and manage collections of TCA Themes.
use crate::{theme::Theme, BuiltinTheme};
#[cfg(feature = "fs")]
use anyhow::{Context, Result};
#[cfg(feature = "fs")]
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
#[cfg(feature = "fs")]
use std::{collections::HashMap, fs, path::Path, path::PathBuf};

/// Get the themes directory path, creating it if it does not exist.
///
/// Returns `$XDG_DATA_HOME/tca/themes/` (or platform equivalent).
#[cfg(feature = "fs")]
pub fn user_themes_path() -> Result<PathBuf> {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "TCA".to_string(),
        app_name: "tca".to_string(),
    })?;
    let themes_dir = strategy.data_dir().join("themes");
    fs::create_dir_all(&themes_dir)?;

    Ok(themes_dir)
}

/// Get all themes from a given directory.
#[cfg(feature = "fs")]
pub fn all_from_dir(dir: &str) -> Vec<Theme> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
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
    }
    items
}

/// Get all local user themes.
#[cfg(feature = "fs")]
pub fn all_user_themes() -> Vec<Theme> {
    let Ok(themes_dir) = user_themes_path() else {
        return Vec::new();
    };
    all_from_dir(themes_dir.to_str().unwrap())
}

/// Get a vec of all available themes.
pub fn all_themes() -> Vec<Theme> {
    #[cfg(feature = "fs")]
    {
        // First we get all of the built in themes as a set, then update with locally installed ones.
        let mut themes: HashMap<_, _> = all_user_themes()
            .into_iter()
            .map(|t| (t.meta.name.clone(), t))
            .collect();
        for t in BuiltinTheme::iter() {
            let theme = t.theme();
            themes.entry(theme.meta.name.clone()).or_insert(theme);
        }
        themes.into_values().collect()
    }

    #[cfg(not(feature = "fs"))]
    {
        BuiltinTheme::iter().map(|t| t.theme()).collect()
    }
}

/// Find a path to a theme by name.
///
/// Converts name to kebab-case and searches for `<name>.toml` in the
/// themes directory.
/// Returns the full path if found.
#[cfg(feature = "fs")]
pub fn find_theme_path(name: &str) -> Result<PathBuf> {
    let themes_dir = user_themes_path()?;

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

#[cfg(feature = "fs")]
/// Finds a theme file by exact path or theme name and reads it into a String.
pub fn load_theme_file(path_or_name: &str) -> Result<String> {
    let path = Path::new(path_or_name);

    // 1. Try exact path (handles absolute paths and relative paths from cwd)
    if path.exists() && path.is_file() {
        return fs::read_to_string(path)
            .with_context(|| format!("Failed to read theme file: {:?}", path));
    }

    // 2. Try shared themes directory
    if let Ok(shared_path) = find_theme_path(path_or_name) {
        return fs::read_to_string(&shared_path)
            .with_context(|| format!("Failed to read theme file: {:?}", shared_path));
    }

    Err(anyhow::anyhow!(
        "Theme '{}' not found. Searched:\n\
         1. Exact path: {:?}\n\
         2. Shared themes: {:?}\n",
        path_or_name,
        path,
        user_themes_path()?,
    ))
}

#[cfg(feature = "fs")]
pub fn mode_aware_theme_name() -> Option<String> {
    use crate::config::TcaConfig;
    use terminal_colorsaurus::{theme_mode, QueryOptions, ThemeMode};
    let cfg = TcaConfig::load();

    match theme_mode(QueryOptions::default()).ok() {
        Some(ThemeMode::Dark) => cfg
            .tca
            .default_dark_theme
            .clone()
            .or(cfg.tca.default_theme.clone()),
        Some(ThemeMode::Light) => cfg
            .tca
            .default_light_theme
            .clone()
            .or(cfg.tca.default_theme.clone()),
        None => cfg.tca.default_theme.clone(),
    }
}
