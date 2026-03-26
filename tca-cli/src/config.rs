use anyhow::Result;
use tca_types::TcaConfig;

use crate::ConfigCommand;

pub fn run(cmd: &Option<ConfigCommand>) -> Result<()> {
    let mut config = TcaConfig::load();
    match cmd {
        Some(ConfigCommand::Show) | None => {
            println!("{}", config);
        }
        Some(ConfigCommand::Set { key, theme }) => {
            let key = key.as_str();
            match key {
                "default" => config.tca.default_theme = Some(theme.to_string()),
                "default_dark" => config.tca.default_dark_theme = Some(theme.to_string()),
                "default_light" => config.tca.default_light_theme = Some(theme.to_string()),
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unknown key: '{}'. Valid keys: default, default_dark, default_light",
                        key
                    ))
                }
            }
            config.store()?;
        }
    }

    Ok(())
}
