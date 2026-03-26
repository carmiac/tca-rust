use anyhow::Result;
use std::fs;

use crate::{add::download_all_themes, REPO, REPO_BRANCH, REPO_DIR};

pub fn run(all: bool, none: bool, force: bool) -> Result<()> {
    if force || !tca_types::TcaConfig::path()?.exists() {
        println!("Writing default config...");
        let config = tca_types::TcaConfig::default();
        config.store()?;
    }

    if !none {
        println!("Creating theme directory...");
        let dir = tca_types::user_themes_path()?;
        fs::create_dir_all(dir)?;
        println!("Adding built in theme files...");
        for theme in tca_types::BuiltinTheme::iter() {
            let t = theme.theme();
            let path = t.to_pathbuf()?;
            let content = toml::to_string(&t)?;
            println!("  {}", t.meta.name);
            fs::write(&path, content)?;
        }
        if all {
            download_all_themes(&REPO.to_string(), REPO_DIR, &REPO_BRANCH.to_string())?
        }
    }
    Ok(())
}
