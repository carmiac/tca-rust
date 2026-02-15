use tca_ratatui::TcaTheme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = TcaTheme::from_file("../tca-validator/test/example-complete.yaml")?;

    println!("Theme: {}", theme.meta.name);
    println!();

    println!("Neutral ramp tones: {:?}", theme.palette.neutral.tones());
    
    println!();
    println!("All available color ramps:");
    for name in theme.palette.ramp_names() {
        if let Some(ramp) = theme.palette.get_ramp(name) {
            println!("  {}: tones {:?}", name, ramp.tones());
            
            // Show a sample color from this ramp
            if let Some(first_tone) = ramp.tones().first() {
                if let Some(color) = ramp.get(*first_tone) {
                    println!("    (sample tone {}: {:?})", first_tone, color);
                }
            }
        }
    }

    Ok(())
}
