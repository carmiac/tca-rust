//! XDG-compliant theme loader for Terminal Colors Architecture.
//!
//! Provides filesystem operations for discovering and loading TCA themes
//! from XDG data directories.

#![warn(missing_docs)]

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolve the TCA data directory path without creating it.
///
/// Returns `$XDG_DATA_HOME/tca` on Linux/BSD, or the platform-equivalent on other OS.
fn resolve_data_dir() -> Result<PathBuf> {
    let project_dirs =
        ProjectDirs::from("", "", "tca").context("Failed to determine project directories")?;
    Ok(project_dirs.data_dir().to_path_buf())
}

/// Get the TCA data directory path, creating it if it does not exist.
///
/// Returns `$XDG_DATA_HOME/tca` on Linux/BSD, or the platform-equivalent on other OS.
pub fn get_data_dir() -> Result<PathBuf> {
    let data_dir = resolve_data_dir()?;
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {:?}", data_dir))?;
    }
    Ok(data_dir)
}

/// Get the themes directory path, creating it if it does not exist.
///
/// Returns `$XDG_DATA_HOME/tca/themes` (or platform equivalent).
pub fn get_themes_dir() -> Result<PathBuf> {
    let themes_dir = resolve_data_dir()?.join("themes");
    if !themes_dir.exists() {
        fs::create_dir_all(&themes_dir)
            .with_context(|| format!("Failed to create themes directory: {:?}", themes_dir))?;
    }
    Ok(themes_dir)
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

/// Find a theme by name (with or without `.toml` extension).
///
/// Searches for `<name>.toml` in the themes directory.
/// Returns the full path if found.
pub fn find_theme(name: &str) -> Result<PathBuf> {
    let themes_dir = get_themes_dir()?;

    // If no extension, also try with .toml appended
    let candidate = if !name.ends_with(".toml") {
        themes_dir.join(format!("{}.toml", name))
    } else {
        themes_dir.join(name)
    };

    if candidate.exists() && candidate.is_file() {
        return Ok(candidate);
    }

    Err(anyhow::anyhow!(
        "Theme '{}' not found in {:?}. Available themes: {:?}",
        name,
        themes_dir,
        list_theme_names()?
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
         2. Shared themes: {:?}\n\
         Available shared themes: {:?}",
        path_or_name,
        path,
        get_themes_dir()?,
        list_theme_names()?
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
    fn test_get_data_dir() {
        let dir = get_data_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.to_string_lossy().contains("tca"));
    }

    #[test]
    fn test_get_themes_dir() {
        let dir = get_themes_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.ends_with("themes"));
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
