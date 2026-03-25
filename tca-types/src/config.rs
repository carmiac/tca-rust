//! Read and write user TCA preferences.

use anyhow::Result;
use core::fmt;
use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Configuration for TCA user preferences.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TcaSection {
    /// The general default theme. Used if mode can't be detected or other options
    /// aren't defined.
    pub default_theme: Option<String>,
    /// Default dark mode theme.
    pub default_dark_theme: Option<String>,
    /// Default light mode theme.
    pub default_light_theme: Option<String>,
}

/// TCA configuration file.
///
/// A TOML file containing TCA user preferences
/// Stored at `$XDG_CONFIG_HOME/tca/tca.toml`.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TcaConfig {
    #[serde(default)]
    /// TCA theme preferences.
    pub tca: TcaSection,
}

impl TcaConfig {
    /// Returns the path to the TCA config file (`tca.toml` in the app config dir).
    pub fn path() -> Result<PathBuf> {
        let strategy = choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".into(),
            author: "TCA".into(),
            app_name: "tca".into(),
        })?;
        Ok(strategy.config_dir().join("tca.toml"))
    }

    /// Load the user's configuration preferences.
    ///
    /// Returns [`Default`] if the config file doesn't exist or cannot be parsed.
    pub fn load() -> Self {
        let Ok(path) = TcaConfig::path() else {
            return Self::default();
        };
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };
        toml::from_str::<TcaConfig>(&content).unwrap_or_default()
    }

    /// Save the user's configuration preferences.
    pub fn store(&self) -> Result<()> {
        let path = TcaConfig::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}

impl fmt::Display for TcaConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "default: {:?}\ndefault_dark: {:?}\ndefault_light: {:?}",
            self.tca.default_theme.clone().unwrap_or("None".to_string()),
            self.tca
                .default_dark_theme
                .clone()
                .unwrap_or("None".to_string()),
            self.tca
                .default_light_theme
                .clone()
                .unwrap_or("None".to_string()),
        )
    }
}
