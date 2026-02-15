use tca_ratatui::TcaTheme;
use ratatui::style::Style;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme_path = "../tca-validator/test/example-complete.yaml";
    
    println!("Loading TCA theme from: {}", theme_path);
    let theme = TcaTheme::from_file(theme_path)?;

    println!("\nTheme: {}", theme.name());
    if let Some(author) = theme.author() {
        println!("Author: {}", author);
    }

    println!("\nPalette:");
    println!("  Neutral: {:?}", theme.palette.neutral.tones());
    for name in theme.palette.ramp_names() {
        if let Some(ramp) = theme.palette.get_ramp(name) {
            println!("  {}: {:?}", name, ramp.tones());
        }
    }

    println!("\nANSI Colors:");
    println!("  Red: {:?}", theme.ansi.red);
    println!("  Green: {:?}", theme.ansi.green);
    println!("  Blue: {:?}", theme.ansi.blue);

    if let Some(semantic) = &theme.semantic {
        println!("\nSemantic Colors:");
        println!("  Error: {:?}", semantic.error);
        println!("  Warning: {:?}", semantic.warning);
        println!("  Success: {:?}", semantic.success);
        println!("  Info: {:?}", semantic.info);
    }

    if let Some(ui) = &theme.ui {
        println!("\nUI Colors:");
        println!("  Background: {:?}", ui.bg_primary);
        println!("  Foreground: {:?}", ui.fg_primary);
        println!("  Selection: {:?}", ui.selection_bg);
    }

    println!("\nExample Styles:");
    let error_style = theme.semantic
        .as_ref()
        .map(|s| Style::default().fg(s.error))
        .unwrap_or_default();
    println!("  Error style: {:?}", error_style);

    Ok(())
}
