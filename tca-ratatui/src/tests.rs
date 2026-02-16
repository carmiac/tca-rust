use ratatui::style::Color;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{Ansi, ColorRamp, Palette, Semantic, Ui};

    #[test]
    fn test_color_ramp_get() {
        let mut colors = std::collections::HashMap::new();
        colors.insert(1, Color::Rgb(10, 10, 10));
        colors.insert(3, Color::Rgb(128, 128, 128));
        colors.insert(5, Color::Rgb(245, 245, 245));

        let ramp = ColorRamp { colors };

        assert_eq!(ramp.get(1), Some(Color::Rgb(10, 10, 10)));
        assert_eq!(ramp.get(3), Some(Color::Rgb(128, 128, 128)));
        assert_eq!(ramp.get(5), Some(Color::Rgb(245, 245, 245)));
        assert_eq!(ramp.get(4), None);
    }

    #[test]
    fn test_color_ramp_tones() {
        let mut colors = std::collections::HashMap::new();
        colors.insert(3, Color::Red);
        colors.insert(1, Color::DarkGray);
        colors.insert(5, Color::White);

        let ramp = ColorRamp { colors };
        let tones = ramp.tones();

        assert_eq!(tones, vec![1, 3, 5]);
    }

    #[test]
    fn test_palette_get_ramp() {
        let neutral = ColorRamp {
            colors: std::collections::HashMap::new(),
        };

        let mut red_colors = std::collections::HashMap::new();
        red_colors.insert(5, Color::Red);
        let red_ramp = ColorRamp { colors: red_colors };

        let mut ramps = std::collections::HashMap::new();
        ramps.insert("red".to_string(), red_ramp);

        let palette = Palette { neutral, ramps };

        assert!(palette.get_ramp("red").is_some());
        assert!(palette.get_ramp("blue").is_none());
    }

    #[test]
    fn test_palette_ramp_names() {
        let neutral = ColorRamp {
            colors: std::collections::HashMap::new(),
        };

        let mut ramps = std::collections::HashMap::new();
        ramps.insert(
            "red".to_string(),
            ColorRamp {
                colors: std::collections::HashMap::new(),
            },
        );
        ramps.insert(
            "blue".to_string(),
            ColorRamp {
                colors: std::collections::HashMap::new(),
            },
        );
        ramps.insert(
            "green".to_string(),
            ColorRamp {
                colors: std::collections::HashMap::new(),
            },
        );

        let palette = Palette { neutral, ramps };
        let names = palette.ramp_names();

        assert_eq!(names, vec!["blue", "green", "red"]);
    }

    #[test]
    fn test_semantic_default() {
        let semantic = Semantic::default();

        assert_eq!(semantic.error, Color::Red);
        assert_eq!(semantic.warning, Color::Yellow);
        assert_eq!(semantic.success, Color::Green);
        assert_eq!(semantic.info, Color::Blue);
        assert_eq!(semantic.highlight, Color::Cyan);
        assert_eq!(semantic.link, Color::Blue);
    }

    #[test]
    fn test_ui_default() {
        let ui = Ui::default();

        assert_eq!(ui.bg_primary, Color::Black);
        assert_eq!(ui.fg_primary, Color::White);
        assert_eq!(ui.fg_secondary, Color::Gray);
        assert_eq!(ui.fg_muted, Color::DarkGray);
        assert_eq!(ui.border_primary, Color::White);
        assert_eq!(ui.border_muted, Color::DarkGray);
    }

    #[test]
    fn test_ansi_partial_eq() {
        let ansi1 = Ansi {
            black: Color::Black,
            red: Color::Red,
            green: Color::Green,
            yellow: Color::Yellow,
            blue: Color::Blue,
            magenta: Color::Magenta,
            cyan: Color::Cyan,
            white: Color::White,
            bright_black: Color::DarkGray,
            bright_red: Color::LightRed,
            bright_green: Color::LightGreen,
            bright_yellow: Color::LightYellow,
            bright_blue: Color::LightBlue,
            bright_magenta: Color::LightMagenta,
            bright_cyan: Color::LightCyan,
            bright_white: Color::White,
        };

        let ansi2 = ansi1.clone();
        assert_eq!(ansi1, ansi2);
    }

    #[test]
    fn test_semantic_partial_eq() {
        let sem1 = Semantic::default();
        let sem2 = Semantic::default();
        assert_eq!(sem1, sem2);

        let sem3 = Semantic {
            error: Color::LightRed,
            ..Default::default()
        };
        assert_ne!(sem1, sem3);
    }

    #[test]
    fn test_ui_partial_eq() {
        let ui1 = Ui::default();
        let ui2 = Ui::default();
        assert_eq!(ui1, ui2);

        let ui3 = Ui {
            bg_primary: Color::Rgb(10, 10, 10),
            ..Default::default()
        };
        assert_ne!(ui1, ui3);
    }
}
