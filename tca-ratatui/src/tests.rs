use ratatui::style::Color;

use crate::theme::{
    Ansi, Base16, ColorRamp, Meta, Palette, Semantic, TcaTheme, TcaThemeBuilder, Ui,
};
use std::collections::HashMap;

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
fn test_palette_get_ramp() {
    let mut ramps = HashMap::new();
    ramps.insert(
        "red".to_string(),
        ColorRamp {
            colors: vec![Color::Rgb(180, 0, 0), Color::Rgb(255, 80, 80)],
        },
    );
    let palette = Palette(ramps);

    let ramp = palette.get_ramp("red").unwrap();
    assert_eq!(ramp.len(), 2);
    assert_eq!(ramp.get(0), Some(Color::Rgb(180, 0, 0)));
    assert_eq!(ramp.get(1), Some(Color::Rgb(255, 80, 80)));
    assert!(palette.get_ramp("blue").is_none());
}

#[test]
fn test_palette_ramp_names_sorted() {
    let ramps: HashMap<_, _> = ["red", "blue", "green"]
        .iter()
        .map(|n| (n.to_string(), ColorRamp::default()))
        .collect();
    let palette = Palette(ramps);
    assert_eq!(palette.ramp_names(), vec!["blue", "green", "red"]);
}

#[test]
fn test_palette_default_is_empty() {
    let palette = Palette::default();
    assert!(palette.ramp_names().is_empty());
}

#[test]
fn test_base16_get() {
    let mut map = HashMap::new();
    map.insert("base00".to_string(), Color::Rgb(26, 26, 26));
    let b16 = Base16(map);
    assert_eq!(b16.get("base00"), Some(Color::Rgb(26, 26, 26)));
    assert_eq!(b16.get("base01"), None);
}

#[test]
fn test_base16_default_is_empty() {
    let b16 = Base16::default();
    assert_eq!(b16.get("base00"), None);
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
    assert!(theme.palette.ramp_names().is_empty());
    assert!(theme.base16.get("base00").is_none());
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
            dark: Some(false),
            ..Default::default()
        })
        .build();
    assert_eq!(theme.meta.name, "Custom");
    assert!(!theme.meta.dark.unwrap());
}

#[test]
fn test_theme_name_and_author() {
    let theme = TcaThemeBuilder::new()
        .meta(Meta {
            name: "Nord Dark".to_string(),
            author: Some("Arctic Ice Studio".to_string()),
            ..Default::default()
        })
        .build();
    assert_eq!(theme.meta.name, "Nord Dark");
    assert_eq!(theme.meta.author, Some("Arctic Ice Studio".to_string()));
}

#[test]
fn test_theme_author_none() {
    let theme = TcaThemeBuilder::new().build();
    assert_eq!(theme.meta.author, None);
}

#[cfg(feature = "loader")]
mod loader_tests {
    use super::*;

    fn nord_toml() -> &'static str {
        r##"
[theme]
name = "Nord Dark"
slug = "nord-dark"
author = "TCA Project"
version = "1.0.0"
dark = true

[ansi]
black          = "#2e3440"
red            = "#bf616a"
green          = "#a3be8c"
yellow         = "#ebcb8b"
blue           = "#5e81ac"
magenta        = "#b48ead"
cyan           = "#8fbcbb"
white          = "#d8dee9"
bright_black   = "#434c5e"
bright_red     = "#bf616a"
bright_green   = "#a3be8c"
bright_yellow  = "#ebcb8b"
bright_blue    = "#81a1c1"
bright_magenta = "#b48ead"
bright_cyan    = "#88c0d0"
bright_white   = "#eceff4"

[palette]
neutral = ["ansi.black", "#3b4252", "ansi.bright_black", "ansi.white", "ansi.bright_white"]
frost   = ["#5e81ac", "#81a1c1", "#88c0d0", "#8fbcbb"]
orange  = ["#d08770"]

[base16]
base00 = "palette.neutral.0"
base08 = "ansi.red"
base09 = "palette.orange.0"

[semantic]
error     = "ansi.red"
warning   = "ansi.yellow"
success   = "ansi.green"
info      = "palette.frost.1"
highlight = "ansi.magenta"
link      = "palette.frost.2"

[ui.bg]
primary   = "palette.neutral.0"
secondary = "palette.neutral.1"

[ui.fg]
primary   = "palette.neutral.4"
secondary = "palette.neutral.3"
muted     = "palette.neutral.2"

[ui.border]
primary = "palette.frost.0"
muted   = "palette.neutral.2"

[ui.cursor]
primary = "palette.neutral.3"
muted   = "palette.neutral.2"

[ui.selection]
bg = "palette.neutral.2"
fg = "palette.neutral.4"
"##
    }

    #[test]
    fn test_load_from_toml_meta() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        assert_eq!(theme.meta.name, "Nord Dark");
        assert_eq!(theme.meta.slug.as_deref(), Some("nord-dark"));
        assert_eq!(theme.meta.dark, Some(true));
    }

    #[test]
    fn test_load_ansi_hex_resolved() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        // #2e3440 → Rgb(46, 52, 64)
        assert_eq!(theme.ansi.black, Color::Rgb(46, 52, 64));
        // #bf616a → Rgb(191, 97, 106)
        assert_eq!(theme.ansi.red, Color::Rgb(191, 97, 106));
    }

    #[test]
    fn test_load_palette_ramps() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        let neutral = theme.palette.get_ramp("neutral").unwrap();
        assert_eq!(neutral.len(), 5);
        // index 0 = "ansi.black" = #2e3440
        assert_eq!(neutral.get(0), Some(Color::Rgb(46, 52, 64)));
        // index 1 = "#3b4252" literal
        assert_eq!(neutral.get(1), Some(Color::Rgb(59, 66, 82)));

        let frost = theme.palette.get_ramp("frost").unwrap();
        assert_eq!(frost.len(), 4);
    }

    #[test]
    fn test_load_base16() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        // base00 = palette.neutral.0 = ansi.black = #2e3440
        assert_eq!(theme.base16.get("base00"), Some(Color::Rgb(46, 52, 64)));
        // base08 = ansi.red = #bf616a
        assert_eq!(theme.base16.get("base08"), Some(Color::Rgb(191, 97, 106)));
        // base09 = palette.orange.0 = #d08770
        assert_eq!(theme.base16.get("base09"), Some(Color::Rgb(208, 135, 112)));
    }

    #[test]
    fn test_load_semantic() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        assert_eq!(theme.semantic.error, Color::Rgb(191, 97, 106)); // ansi.red
        assert_eq!(theme.semantic.warning, Color::Rgb(235, 203, 139)); // ansi.yellow
    }

    #[test]
    fn test_load_ui() {
        let theme = TcaTheme::try_from(nord_toml()).unwrap();
        // bg.primary = palette.neutral.0 = #2e3440
        assert_eq!(theme.ui.bg_primary, Color::Rgb(46, 52, 64));
        // fg.primary = palette.neutral.4 = ansi.bright_white = #eceff4
        assert_eq!(theme.ui.fg_primary, Color::Rgb(236, 239, 244));
    }

    #[test]
    fn test_load_no_palette_section() {
        let toml = r##"
[theme]
name = "Minimal"

[ansi]
black   = "#000000"
red     = "#800000"
green   = "#008000"
yellow  = "#808000"
blue    = "#000080"
magenta = "#800080"
cyan    = "#008080"
white   = "#c0c0c0"
bright_black   = "#808080"
bright_red     = "#ff0000"
bright_green   = "#00ff00"
bright_yellow  = "#ffff00"
bright_blue    = "#0000ff"
bright_magenta = "#ff00ff"
bright_cyan    = "#00ffff"
bright_white   = "#ffffff"

[semantic]
error     = "#ff0000"
warning   = "#ffff00"
info      = "#0000ff"
success   = "#00ff00"
highlight = "#00ffff"
link      = "#0000ff"

[ui.bg]
primary   = "#000000"
secondary = "#000000"
[ui.fg]
primary   = "#ffffff"
secondary = "#c0c0c0"
muted     = "#808080"
[ui.border]
primary = "#ffffff"
muted   = "#808080"
[ui.cursor]
primary = "#ffffff"
muted   = "#808080"
[ui.selection]
bg = "#808080"
fg = "#ffffff"
"##;
        let theme = TcaTheme::try_from(toml).unwrap();
        assert!(theme.palette.ramp_names().is_empty());
        assert!(theme.base16.get("base00").is_none());
    }

    #[test]
    fn test_load_invalid_ansi_hex_errors() {
        let toml = r##"
[theme]
name = "Bad"
[ansi]
black   = "not-a-hex"
red     = "#800000"
green   = "#008000"
yellow  = "#808000"
blue    = "#000080"
magenta = "#800080"
cyan    = "#008080"
white   = "#c0c0c0"
bright_black   = "#808080"
bright_red     = "#ff0000"
bright_green   = "#00ff00"
bright_yellow  = "#ffff00"
bright_blue    = "#0000ff"
bright_magenta = "#ff00ff"
bright_cyan    = "#00ffff"
bright_white   = "#ffffff"
[semantic]
error     = "#ff0000"
warning   = "#ffff00"
info      = "#0000ff"
success   = "#00ff00"
highlight = "#00ffff"
link      = "#0000ff"
[ui.bg]
primary   = "#000000"
secondary = "#000000"
[ui.fg]
primary   = "#ffffff"
secondary = "#c0c0c0"
muted     = "#808080"
[ui.border]
primary = "#ffffff"
muted   = "#808080"
[ui.cursor]
primary = "#ffffff"
muted   = "#808080"
[ui.selection]
bg = "#808080"
fg = "#ffffff"
"##;
        assert!(TcaTheme::try_from(toml).is_err());
    }
}
