use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

/// Get the TCA data directory path.
/// 
/// Returns `$XDG_DATA_HOME/tca` on Linux/BSD, or the platform-equivalent on other OSes.
/// Creates the directory if it doesn't exist.
pub fn get_data_dir() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "", "tca")
        .context("Failed to determine project directories")?;
    
    let data_dir = project_dirs.data_dir();
    
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)
            .context(format!("Failed to create data directory: {:?}", data_dir))?;
    }
    
    Ok(data_dir.to_path_buf())
}

/// Get the themes directory path.
/// 
/// Returns `$XDG_DATA_HOME/tca/themes` (or platform equivalent).
/// Creates the directory if it doesn't exist.
pub fn get_themes_dir() -> Result<PathBuf> {
    let data_dir = get_data_dir()?;
    let themes_dir = data_dir.join("themes");
    
    if !themes_dir.exists() {
        fs::create_dir_all(&themes_dir)
            .context(format!("Failed to create themes directory: {:?}", themes_dir))?;
    }
    
    Ok(themes_dir)
}

/// List all available theme files in the shared themes directory.
/// 
/// Returns paths to all `.yaml` and `.yml` files in the themes directory.
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
                if ext == "yaml" || ext == "yml" {
                    themes.push(path);
                }
            }
        }
    }
    
    themes.sort();
    Ok(themes)
}

/// Find a theme by name (with or without extension).
/// 
/// Searches for `<name>.yaml` or `<name>.yml` in the themes directory.
/// Returns the full path if found.
pub fn find_theme(name: &str) -> Result<PathBuf> {
    let themes_dir = get_themes_dir()?;
    
    // Try the name as-is first
    let mut candidates = vec![
        themes_dir.join(name),
    ];
    
    // If no extension, try adding .yaml and .yml
    if !name.ends_with(".yaml") && !name.ends_with(".yml") {
        candidates.push(themes_dir.join(format!("{}.yaml", name)));
        candidates.push(themes_dir.join(format!("{}.yml", name)));
    }
    
    for candidate in candidates {
        if candidate.exists() && candidate.is_file() {
            return Ok(candidate);
        }
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
        .filter_map(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
        })
        .collect())
}

/// Load a theme file from any of these locations (in order):
/// 1. Exact path if it exists
/// 2. Shared themes directory ($XDG_DATA_HOME/tca/themes/)
/// 3. Current directory
/// 
/// Returns the file contents as a string.
pub fn load_theme_file(path_or_name: &str) -> Result<String> {
    let path = Path::new(path_or_name);
    
    // 1. Try exact path
    if path.exists() && path.is_file() {
        return fs::read_to_string(path)
            .context(format!("Failed to read theme file: {:?}", path));
    }
    
    // 2. Try shared themes directory
    if let Ok(shared_path) = find_theme(path_or_name) {
        return fs::read_to_string(&shared_path)
            .context(format!("Failed to read theme file: {:?}", shared_path));
    }
    
    // 3. Try current directory
    if let Ok(content) = fs::read_to_string(path_or_name) {
        return Ok(content);
    }
    
    Err(anyhow::anyhow!(
        "Theme '{}' not found. Searched:\n\
         1. Exact path: {:?}\n\
         2. Shared themes: {:?}\n\
         3. Current directory\n\
         Available shared themes: {:?}",
        path_or_name,
        path,
        get_themes_dir()?,
        list_theme_names()?
    ))
}

/// Load and parse a theme file as YAML.
/// 
/// This is a generic loader - you provide the type to deserialize into.
pub fn load_theme<T: serde::de::DeserializeOwned>(path_or_name: &str) -> Result<T> {
    let content = load_theme_file(path_or_name)?;
    serde_yaml::from_str(&content)
        .context(format!("Failed to parse theme YAML: {}", path_or_name))
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
        // Verify that all returned paths have yaml/yml extension
        for theme_path in themes {
            let ext = theme_path.extension().and_then(|s| s.to_str());
            assert!(matches!(ext, Some("yaml") | Some("yml")));
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
