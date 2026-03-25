use anyhow::Result;

pub fn run() -> Result<()> {
    println!("Built in Themes:");
    for theme in tca_types::BuiltinTheme::iter() {
        println!("  {}", theme.theme().meta.name);
    }

    if let Ok(dir) = tca_types::user_themes_path() {
        println!("\nThemes from user directory: {}", dir.display());
        let mut user_themes = tca_types::all_user_themes();
        if user_themes.is_empty() {
            println!("No themes found.");
            println!("Add .toml theme files to theme directory.");
        } else {
            user_themes.sort();
            for theme in user_themes.iter() {
                println!("  {}", theme.meta.name);
            }
        }
    }

    Ok(())
}
