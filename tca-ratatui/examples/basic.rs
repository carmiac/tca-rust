use std::env;
use std::path::PathBuf;

use ratatui::style::Style;
use tca_ratatui::TcaTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme_path: Option<PathBuf> = env::args_os().nth(1).map(PathBuf::from);

    let theme = match theme_path {
        Some(theme_path) => {
            println!("Loading TCA theme from: {:?}", theme_path);
            TcaTheme::try_from(&theme_path)?
        }
        None => {
            return Err("Usage: basic path/to/theme.toml".into());
        }
    };

    println!("\nTheme: {}", theme.meta.name);
    if let Some(author) = theme.meta.author {
        println!("Author: {}", author);
    }

    println!("\nPalette:");
    for name in theme.palette.ramp_names() {
        if let Some(ramp) = theme.palette.get_ramp(name) {
            println!("  {}: {} entries", name, ramp.len());
        }
    }

    println!("\nANSI Colors:");
    println!("  Red: {:?}", theme.ansi.red);
    println!("  Green: {:?}", theme.ansi.green);
    println!("  Blue: {:?}", theme.ansi.blue);

    println!("\nSemantic Colors:");
    println!("  Error: {:?}", theme.semantic.error);
    println!("  Warning: {:?}", theme.semantic.warning);
    println!("  Success: {:?}", theme.semantic.success);
    println!("  Info: {:?}", theme.semantic.info);

    println!("\nUI Colors:");
    println!("  Background: {:?}", theme.ui.bg_primary);
    println!("  Foreground: {:?}", theme.ui.fg_primary);
    println!("  Selection: {:?}", theme.ui.selection_bg);

    println!("\nExample Styles:");
    let error_style = Style::default().fg(theme.semantic.error);
    println!("  Error style: {:?}", error_style);

    Ok(())
}
