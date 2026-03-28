use std::env;

use ratatui::style::Style;
use tca_ratatui::TcaTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args().nth(1);
    let theme_path: Option<&str> = arg.as_deref();

    if theme_path.is_none() {
        return Err("Usage: basic path/to/theme.yaml".into());
    }
    println!("Loading TCA theme from: {:?}", theme_path);
    let theme = TcaTheme::new(theme_path);

    println!("\nTheme: {}", theme.meta.name);
        if !theme.meta.author.is_empty() {
            println!("Author: {}", theme.meta.author);
        }

    println!("\nBase24 Slots:");
    for (i, &color) in theme.base24.iter().enumerate() {
        println!("  base{:02x}: {:?}", i, color);
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
