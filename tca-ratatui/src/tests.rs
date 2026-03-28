use ratatui::style::Color;

use crate::theme::{Ansi, ColorRamp, Meta, Semantic, TcaTheme, TcaThemeBuilder, Ui};

#[test]
fn test_color_ramp_get() {
    let ramp = ColorRamp {
        colors: vec![
            Color::Rgb(10, 10, 10),
            Color::Rgb(128, 128, 128),
            Color::Rgb(245, 245, 245),
        ],
    };
    assert_eq!(ramp.get(0), Some(Color::Rgb(10, 10, 10)));
    assert_eq!(ramp.get(1), Some(Color::Rgb(128, 128, 128)));
    assert_eq!(ramp.get(2), Some(Color::Rgb(245, 245, 245)));
    assert_eq!(ramp.get(3), None); // out of bounds
}

#[test]
fn test_color_ramp_len() {
    let ramp = ColorRamp {
        colors: vec![Color::Red, Color::DarkGray, Color::White],
    };
    assert_eq!(ramp.len(), 3);
}

#[test]
fn test_color_ramp_empty() {
    let ramp = ColorRamp::default();
    assert!(ramp.is_empty());
    assert_eq!(ramp.get(0), None);
    assert_eq!(ramp.len(), 0);
}

#[test]
fn test_ansi_default() {
    let ansi = Ansi::default();
    assert_eq!(ansi.red, Color::Red);
    assert_eq!(ansi.green, Color::Green);
    assert_eq!(ansi.blue, Color::Blue);
    assert_eq!(ansi.black, Color::Black);
    assert_eq!(ansi.bright_white, Color::White);
}

#[test]
fn test_semantic_default() {
    let s = Semantic::default();
    assert_eq!(s.error, Color::Red);
    assert_eq!(s.warning, Color::Yellow);
    assert_eq!(s.info, Color::Blue);
    assert_eq!(s.success, Color::Green);
    assert_eq!(s.highlight, Color::Cyan);
    assert_eq!(s.link, Color::Blue);
}

#[test]
fn test_ui_default() {
    let ui = Ui::default();
    assert_eq!(ui.bg_primary, Color::Black);
    assert_eq!(ui.bg_secondary, Color::Black);
    assert_eq!(ui.fg_primary, Color::White);
    assert_eq!(ui.fg_secondary, Color::Gray);
    assert_eq!(ui.fg_muted, Color::DarkGray);
    assert_eq!(ui.border_primary, Color::White);
    assert_eq!(ui.border_muted, Color::DarkGray);
    assert_eq!(ui.cursor_primary, Color::White);
    assert_eq!(ui.cursor_muted, Color::Gray);
    assert_eq!(ui.selection_bg, Color::DarkGray);
    assert_eq!(ui.selection_fg, Color::White);
}

#[test]
fn test_builder_defaults() {
    let theme = TcaThemeBuilder::new().build();
    assert_eq!(theme.meta.name, "Unnamed Theme");
    assert_eq!(theme.semantic.error, Color::Red);
    assert_eq!(theme.ui.bg_primary, Color::Black);
    assert_eq!(theme.base24[0], Color::Reset);
}

#[test]
fn test_builder_override_semantic() {
    let theme = TcaThemeBuilder::new()
        .semantic(Semantic {
            error: Color::Rgb(255, 80, 80),
            ..Default::default()
        })
        .build();
    assert_eq!(theme.semantic.error, Color::Rgb(255, 80, 80));
    assert_eq!(theme.semantic.warning, Color::Yellow); // unchanged default
}

#[test]
fn test_builder_override_meta() {
    let theme = TcaThemeBuilder::new()
        .meta(Meta {
            name: "Custom".to_string(),
            dark: false,
            ..Default::default()
        })
        .build();
    assert_eq!(theme.meta.name, "Custom");
    assert!(!theme.meta.dark);
}

#[test]
fn test_theme_author() {
    let theme_with_author = TcaThemeBuilder::new()
        .meta(Meta {
            name: "Nord Dark".to_string(),
            author: "Arctic Ice Studio".to_string(),
            ..Default::default()
        })
        .build();
    assert_eq!(theme_with_author.meta.name, "Nord Dark");
    assert_eq!(theme_with_author.meta.author, "Arctic Ice Studio");

    let theme_no_author = TcaThemeBuilder::new().build();
    assert_eq!(theme_no_author.meta.author, "");
}

#[cfg(feature = "fs")]
mod loader_tests {
    use super::*;

    /// A minimal Nord-like theme in base24 YAML format.
    fn nord_yaml() -> &'static str {
        r#"
scheme: "Nord Dark"
author: "Arctic Ice Studio / TCA Project"
variant: "dark"
base00: "2e3440"
base01: "3b4252"
base02: "434c5e"
base03: "434c5e"
base04: "434c5e"
base05: "eceff4"
base06: "d8dee9"
base07: "eceff4"
base08: "bf616a"
base09: "ebcb8b"
base0A: "ebcb8b"
base0B: "a3be8c"
base0C: "8fbcbb"
base0D: "5e81ac"
base0E: "b48ead"
base0F: "bf616a"
base10: "2e3440"
base11: "000000"
base12: "bf616a"
base13: "ebcb8b"
base14: "a3be8c"
base15: "88c0d0"
base16: "81a1c1"
base17: "b48ead"
"#
    }

    #[test]
    fn test_load_from_yaml_meta() {
        let theme = TcaTheme::try_from(nord_yaml()).unwrap();
        assert_eq!(theme.meta.name, "Nord Dark");
        assert!(theme.meta.dark);
    }

    #[test]
    fn test_load_ansi_hex_resolved() {
        let theme = TcaTheme::try_from(nord_yaml()).unwrap();
        // base00 = #2e3440 → Rgb(46, 52, 64)
        assert_eq!(theme.ansi.black, Color::Rgb(46, 52, 64));
        // base08 = #bf616a → Rgb(191, 97, 106)
        assert_eq!(theme.ansi.red, Color::Rgb(191, 97, 106));
    }

    #[test]
    fn test_load_semantic() {
        let theme = TcaTheme::try_from(nord_yaml()).unwrap();
        // error = base08 = #bf616a → Rgb(191, 97, 106)
        assert_eq!(theme.semantic.error, Color::Rgb(191, 97, 106));
        // warning = base09 = #ebcb8b → Rgb(235, 203, 139)
        assert_eq!(theme.semantic.warning, Color::Rgb(235, 203, 139));
    }

    #[test]
    fn test_load_ui() {
        let theme = TcaTheme::try_from(nord_yaml()).unwrap();
        // bg.primary = base00 = #2e3440 → Rgb(46, 52, 64)
        assert_eq!(theme.ui.bg_primary, Color::Rgb(46, 52, 64));
        // fg.primary = base05 = #eceff4 → Rgb(236, 239, 244)
        assert_eq!(theme.ui.fg_primary, Color::Rgb(236, 239, 244));
    }

    #[test]
    fn test_load_base24_slots() {
        let theme = TcaTheme::try_from(nord_yaml()).unwrap();
        // base24[0] = base00 = #2e3440 → Rgb(46, 52, 64)
        assert_eq!(theme.base24[0], Color::Rgb(46, 52, 64));
        // base24[8] = base08 = #bf616a → Rgb(191, 97, 106)
        assert_eq!(theme.base24[8], Color::Rgb(191, 97, 106));
        assert_eq!(theme.base24.len(), 24);
    }

    #[test]
    fn test_load_invalid_yaml_errors() {
        let yaml = r#"
scheme: "Bad"
variant: "dark"
base00: "zzzzzz"
base01: "3b4252"
base02: "434c5e"
base03: "4c566a"
base04: "d8dee9"
base05: "e5e9f0"
base06: "eceff4"
base07: "8fbcbb"
base08: "bf616a"
base09: "d08770"
base0A: "ebcb8b"
base0B: "a3be8c"
base0C: "88c0d0"
base0D: "81a1c1"
base0E: "b48ead"
base0F: "5e81ac"
base10: "2e3440"
base11: "3b4252"
base12: "bf616a"
base13: "ebcb8b"
base14: "a3be8c"
base15: "88c0d0"
base16: "81a1c1"
base17: "b48ead"
"#;
        assert!(TcaTheme::try_from(yaml).is_err());
    }
}

