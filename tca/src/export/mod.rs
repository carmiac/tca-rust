use anyhow::{Context, Result};
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write};
use tca_types::{hex_to_rgb, Theme};

/// Resolve a color reference using the theme, returning the hex string.
fn resolve_color<'a>(color_ref: &'a str, theme: &'a Theme) -> Result<&'a str> {
    theme
        .resolve(color_ref)
        .with_context(|| format!("Could not resolve color reference: '{}'", color_ref))
}

fn export_base16(theme: &Theme) -> Result<String> {
    let base16 = theme
        .base16
        .as_ref()
        .context("Theme does not have a base16 section. Base16 export requires this section.")?;

    let name = &theme.meta.name;
    let author = theme.meta.author.as_deref().unwrap_or("Unknown");

    let mut output = String::new();
    writeln!(output, "scheme: \"{}\"", name)?;
    writeln!(output, "author: \"{}\"", author)?;
    for i in 0..16 {
        let key = format!("base{:02X}", i);
        let reference = base16
            .get(&key)
            .with_context(|| format!("Missing base16 color: {}", key))?;
        let color = resolve_color(reference, theme)?;
        let hex = color.trim_start_matches('#');
        writeln!(output, "base{:02X}: \"{}\"", i, hex)?;
    }

    Ok(output)
}

fn export_kitty(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "# {}", theme.meta.name)?;
    writeln!(output, "# Generated from TCA theme\n")?;

    let ansi_colors = [
        ("color0", &theme.ansi.black),
        ("color1", &theme.ansi.red),
        ("color2", &theme.ansi.green),
        ("color3", &theme.ansi.yellow),
        ("color4", &theme.ansi.blue),
        ("color5", &theme.ansi.magenta),
        ("color6", &theme.ansi.cyan),
        ("color7", &theme.ansi.white),
        ("color8", &theme.ansi.bright_black),
        ("color9", &theme.ansi.bright_red),
        ("color10", &theme.ansi.bright_green),
        ("color11", &theme.ansi.bright_yellow),
        ("color12", &theme.ansi.bright_blue),
        ("color13", &theme.ansi.bright_magenta),
        ("color14", &theme.ansi.bright_cyan),
        ("color15", &theme.ansi.bright_white),
    ];

    // ANSI values are hex-only — no resolution needed
    for (name, hex) in ansi_colors {
        writeln!(output, "{} {}", name, hex)?;
    }

    writeln!(output, "\n# UI Colors")?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let cursor = resolve_color(&theme.ui.cursor.primary, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;
    let sel_fg = resolve_color(&theme.ui.selection.fg, theme)?;
    writeln!(output, "foreground {}", fg)?;
    writeln!(output, "background {}", bg)?;
    writeln!(output, "cursor {}", cursor)?;
    writeln!(output, "selection_background {}", sel_bg)?;
    writeln!(output, "selection_foreground {}", sel_fg)?;

    Ok(output)
}

fn export_alacritty(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "# {}", theme.meta.name)?;
    writeln!(output, "# Generated from TCA theme\n")?;
    writeln!(output, "[colors.primary]")?;

    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    writeln!(output, "foreground = '{}'", fg)?;
    writeln!(output, "background = '{}'", bg)?;

    writeln!(output, "\n[colors.normal]")?;
    writeln!(output, "black = '{}'", theme.ansi.black)?;
    writeln!(output, "red = '{}'", theme.ansi.red)?;
    writeln!(output, "green = '{}'", theme.ansi.green)?;
    writeln!(output, "yellow = '{}'", theme.ansi.yellow)?;
    writeln!(output, "blue = '{}'", theme.ansi.blue)?;
    writeln!(output, "magenta = '{}'", theme.ansi.magenta)?;
    writeln!(output, "cyan = '{}'", theme.ansi.cyan)?;
    writeln!(output, "white = '{}'", theme.ansi.white)?;

    writeln!(output, "\n[colors.bright]")?;
    writeln!(output, "black = '{}'", theme.ansi.bright_black)?;
    writeln!(output, "red = '{}'", theme.ansi.bright_red)?;
    writeln!(output, "green = '{}'", theme.ansi.bright_green)?;
    writeln!(output, "yellow = '{}'", theme.ansi.bright_yellow)?;
    writeln!(output, "blue = '{}'", theme.ansi.bright_blue)?;
    writeln!(output, "magenta = '{}'", theme.ansi.bright_magenta)?;
    writeln!(output, "cyan = '{}'", theme.ansi.bright_cyan)?;
    writeln!(output, "white = '{}'", theme.ansi.bright_white)?;

    Ok(output)
}

fn export_vim(theme: &Theme) -> Result<String> {
    let name = theme.meta.name.replace(' ', "");
    let display_name = &theme.meta.name;
    let background = if theme.meta.dark.unwrap_or(true) { "dark" } else { "light" };

    let mut output = String::new();
    writeln!(output, "\" {}", display_name)?;
    writeln!(output, "\" Generated from TCA theme\n")?;
    writeln!(output, "set background={}", background)?;
    writeln!(output, "hi clear")?;
    writeln!(output, "if exists('syntax_on')")?;
    writeln!(output, "  syntax reset")?;
    writeln!(output, "endif")?;
    writeln!(output, "let g:colors_name = '{}'\n", name)?;

    let ansi_refs: [&str; 16] = [
        &theme.ansi.black,
        &theme.ansi.red,
        &theme.ansi.green,
        &theme.ansi.yellow,
        &theme.ansi.blue,
        &theme.ansi.magenta,
        &theme.ansi.cyan,
        &theme.ansi.white,
        &theme.ansi.bright_black,
        &theme.ansi.bright_red,
        &theme.ansi.bright_green,
        &theme.ansi.bright_yellow,
        &theme.ansi.bright_blue,
        &theme.ansi.bright_magenta,
        &theme.ansi.bright_cyan,
        &theme.ansi.bright_white,
    ];
    for (i, hex) in ansi_refs.iter().enumerate() {
        writeln!(output, "let g:terminal_color_{} = '{}'", i, hex)?;
    }

    writeln!(output, "\n\" UI colors")?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    writeln!(output, "hi Normal guifg={} guibg={}", fg, bg)?;

    Ok(output)
}

fn export_helix(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "# {}", theme.meta.name)?;
    writeln!(output, "# Generated from TCA theme\n")?;

    writeln!(output, "[ui]")?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    writeln!(output, "background = \"{}\"", bg)?;
    writeln!(output, "foreground = \"{}\"", fg)?;

    writeln!(output, "\n[palette]")?;
    let palette_entries = [
        ("black", &theme.ansi.black),
        ("red", &theme.ansi.red),
        ("green", &theme.ansi.green),
        ("yellow", &theme.ansi.yellow),
        ("blue", &theme.ansi.blue),
        ("magenta", &theme.ansi.magenta),
        ("cyan", &theme.ansi.cyan),
        ("white", &theme.ansi.white),
        ("bright-black", &theme.ansi.bright_black),
        ("bright-red", &theme.ansi.bright_red),
        ("bright-green", &theme.ansi.bright_green),
        ("bright-yellow", &theme.ansi.bright_yellow),
        ("bright-blue", &theme.ansi.bright_blue),
        ("bright-magenta", &theme.ansi.bright_magenta),
        ("bright-cyan", &theme.ansi.bright_cyan),
        ("bright-white", &theme.ansi.bright_white),
    ];
    for (name, hex) in palette_entries {
        writeln!(output, "{} = \"{}\"", name, hex)?;
    }

    Ok(output)
}

fn export_starship(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "# Starship configuration for {}", theme.meta.name)?;
    writeln!(output, "# Generated from TCA theme")?;
    writeln!(output, "# Add this to your starship.toml\n")?;

    writeln!(output, "[palettes.tca]")?;
    let entries = [
        ("black", &theme.ansi.black),
        ("red", &theme.ansi.red),
        ("green", &theme.ansi.green),
        ("yellow", &theme.ansi.yellow),
        ("blue", &theme.ansi.blue),
        ("magenta", &theme.ansi.magenta),
        ("cyan", &theme.ansi.cyan),
        ("white", &theme.ansi.white),
        ("bright_black", &theme.ansi.bright_black),
        ("bright_red", &theme.ansi.bright_red),
        ("bright_green", &theme.ansi.bright_green),
        ("bright_yellow", &theme.ansi.bright_yellow),
        ("bright_blue", &theme.ansi.bright_blue),
        ("bright_magenta", &theme.ansi.bright_magenta),
        ("bright_cyan", &theme.ansi.bright_cyan),
        ("bright_white", &theme.ansi.bright_white),
    ];
    for (name, hex) in entries {
        writeln!(output, "{} = \"{}\"", name, hex)?;
    }

    writeln!(output, "\n# Enable the palette\npalette = \"tca\"")?;

    Ok(output)
}

fn export_vscode(theme: &Theme) -> Result<String> {
    let name = &theme.meta.name;
    let theme_type = if theme.meta.dark.unwrap_or(true) { "dark" } else { "light" };

    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let bg_secondary = resolve_color(&theme.ui.bg.secondary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let fg_secondary = resolve_color(&theme.ui.fg.secondary, theme)?;
    let fg_muted = resolve_color(&theme.ui.fg.muted, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;
    let sel_fg = resolve_color(&theme.ui.selection.fg, theme)?;
    let cursor = resolve_color(&theme.ui.cursor.primary, theme)?;
    let border = resolve_color(&theme.ui.border.primary, theme)?;
    let border_muted = resolve_color(&theme.ui.border.muted, theme)?;

    let mut colors = serde_json::Map::new();

    // Editor
    colors.insert("editor.background".into(), serde_json::json!(bg));
    colors.insert("editor.foreground".into(), serde_json::json!(fg));
    colors.insert("editor.selectionBackground".into(), serde_json::json!(sel_bg));
    colors.insert("editor.selectionForeground".into(), serde_json::json!(sel_fg));
    colors.insert("editor.lineHighlightBackground".into(), serde_json::json!(bg_secondary));
    colors.insert("editorCursor.foreground".into(), serde_json::json!(cursor));
    colors.insert("editorLineNumber.foreground".into(), serde_json::json!(fg_muted));
    colors.insert("editorLineNumber.activeForeground".into(), serde_json::json!(fg));

    // Borders and focus
    colors.insert("focusBorder".into(), serde_json::json!(border));
    colors.insert("editorGroup.border".into(), serde_json::json!(border));
    colors.insert("panel.border".into(), serde_json::json!(border));

    // Sidebar
    colors.insert("sideBar.background".into(), serde_json::json!(bg_secondary));
    colors.insert("sideBar.foreground".into(), serde_json::json!(fg_secondary));
    colors.insert("sideBar.border".into(), serde_json::json!(border_muted));

    // Activity bar
    colors.insert("activityBar.background".into(), serde_json::json!(bg));
    colors.insert("activityBar.foreground".into(), serde_json::json!(fg));
    colors.insert("activityBar.border".into(), serde_json::json!(border_muted));

    // Status bar
    colors.insert("statusBar.background".into(), serde_json::json!(bg));
    colors.insert("statusBar.foreground".into(), serde_json::json!(fg));

    // Tabs
    colors.insert("tab.activeBackground".into(), serde_json::json!(bg));
    colors.insert("tab.activeForeground".into(), serde_json::json!(fg));
    colors.insert("tab.inactiveBackground".into(), serde_json::json!(bg_secondary));
    colors.insert("tab.inactiveForeground".into(), serde_json::json!(fg_muted));
    colors.insert("editorGroupHeader.tabsBackground".into(), serde_json::json!(bg_secondary));

    // Panel
    colors.insert("panel.background".into(), serde_json::json!(bg_secondary));

    // Semantic indicators
    let err = resolve_color(&theme.semantic.error, theme)?;
    let warn = resolve_color(&theme.semantic.warning, theme)?;
    let info = resolve_color(&theme.semantic.info, theme)?;
    let link = resolve_color(&theme.semantic.link, theme)?;
    colors.insert("editorError.foreground".into(), serde_json::json!(err));
    colors.insert("editorWarning.foreground".into(), serde_json::json!(warn));
    colors.insert("editorInfo.foreground".into(), serde_json::json!(info));
    colors.insert("editorLink.activeForeground".into(), serde_json::json!(link));

    // Terminal ANSI colors
    let terminal_colors = [
        ("terminal.ansiBlack", &theme.ansi.black),
        ("terminal.ansiRed", &theme.ansi.red),
        ("terminal.ansiGreen", &theme.ansi.green),
        ("terminal.ansiYellow", &theme.ansi.yellow),
        ("terminal.ansiBlue", &theme.ansi.blue),
        ("terminal.ansiMagenta", &theme.ansi.magenta),
        ("terminal.ansiCyan", &theme.ansi.cyan),
        ("terminal.ansiWhite", &theme.ansi.white),
        ("terminal.ansiBrightBlack", &theme.ansi.bright_black),
        ("terminal.ansiBrightRed", &theme.ansi.bright_red),
        ("terminal.ansiBrightGreen", &theme.ansi.bright_green),
        ("terminal.ansiBrightYellow", &theme.ansi.bright_yellow),
        ("terminal.ansiBrightBlue", &theme.ansi.bright_blue),
        ("terminal.ansiBrightMagenta", &theme.ansi.bright_magenta),
        ("terminal.ansiBrightCyan", &theme.ansi.bright_cyan),
        ("terminal.ansiBrightWhite", &theme.ansi.bright_white),
    ];
    for (key, hex) in terminal_colors {
        colors.insert(key.into(), serde_json::json!(hex));
    }

    let theme_json = serde_json::json!({
        "name": name,
        "type": theme_type,
        "colors": colors,
    });

    Ok(serde_json::to_string_pretty(&theme_json)?)
}

fn write_iterm_color(output: &mut String, key: &str, hex: &str) -> Result<()> {
    let (r8, g8, b8) =
        hex_to_rgb(hex).with_context(|| format!("Invalid hex color: {}", hex))?;
    let (r, g, b) = (r8 as f64 / 255.0, g8 as f64 / 255.0, b8 as f64 / 255.0);
    writeln!(output, "\t<key>{}</key>", key)?;
    writeln!(output, "\t<dict>")?;
    writeln!(output, "\t\t<key>Color Space</key><string>sRGB</string>")?;
    writeln!(output, "\t\t<key>Red Component</key><real>{}</real>", r)?;
    writeln!(output, "\t\t<key>Green Component</key><real>{}</real>", g)?;
    writeln!(output, "\t\t<key>Blue Component</key><real>{}</real>", b)?;
    writeln!(output, "\t</dict>")?;
    Ok(())
}

fn export_iterm2(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(output, "<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">")?;
    writeln!(output, "<plist version=\"1.0\">")?;
    writeln!(output, "<dict>")?;

    let ansi_colors = [
        ("Ansi 0 Color", &theme.ansi.black),
        ("Ansi 1 Color", &theme.ansi.red),
        ("Ansi 2 Color", &theme.ansi.green),
        ("Ansi 3 Color", &theme.ansi.yellow),
        ("Ansi 4 Color", &theme.ansi.blue),
        ("Ansi 5 Color", &theme.ansi.magenta),
        ("Ansi 6 Color", &theme.ansi.cyan),
        ("Ansi 7 Color", &theme.ansi.white),
        ("Ansi 8 Color", &theme.ansi.bright_black),
        ("Ansi 9 Color", &theme.ansi.bright_red),
        ("Ansi 10 Color", &theme.ansi.bright_green),
        ("Ansi 11 Color", &theme.ansi.bright_yellow),
        ("Ansi 12 Color", &theme.ansi.bright_blue),
        ("Ansi 13 Color", &theme.ansi.bright_magenta),
        ("Ansi 14 Color", &theme.ansi.bright_cyan),
        ("Ansi 15 Color", &theme.ansi.bright_white),
    ];
    for (key, hex) in ansi_colors {
        write_iterm_color(&mut output, key, hex)?;
    }

    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let cursor = resolve_color(&theme.ui.cursor.primary, theme)?;
    write_iterm_color(&mut output, "Background Color", bg)?;
    write_iterm_color(&mut output, "Foreground Color", fg)?;
    write_iterm_color(&mut output, "Cursor Color", cursor)?;

    writeln!(output, "</dict>\n</plist>")?;
    Ok(output)
}

fn export_tmux(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    writeln!(output, "# TCA Theme: {}", theme.meta.name)?;
    writeln!(output, "# Generated by tca-export")?;
    writeln!(output, "# Usage: source this file in your tmux.conf\n")?;

    let bg_primary = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg_primary = resolve_color(&theme.ui.fg.primary, theme)?;
    let fg_muted = resolve_color(&theme.ui.fg.muted, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;

    let active_border = &theme.ansi.blue;
    let inactive_border = &theme.ansi.bright_black;

    writeln!(output, "# Status bar")?;
    writeln!(output, "set-option -g status-style \"bg={},fg={}\"", bg_primary, fg_primary)?;
    writeln!(output, "set-option -g status-left-style \"bg={},fg={}\"", bg_primary, fg_primary)?;
    writeln!(output, "set-option -g status-right-style \"bg={},fg={}\"", bg_primary, fg_primary)?;
    writeln!(output)?;

    writeln!(output, "# Window status")?;
    writeln!(output, "set-option -g window-status-style \"bg={},fg={}\"", bg_primary, fg_muted)?;
    writeln!(output, "set-option -g window-status-current-style \"bg={},fg={}\"", sel_bg, fg_primary)?;
    writeln!(output)?;

    writeln!(output, "# Pane borders")?;
    writeln!(output, "set-option -g pane-border-style \"fg={}\"", inactive_border)?;
    writeln!(output, "set-option -g pane-active-border-style \"fg={}\"", active_border)?;
    writeln!(output)?;

    writeln!(output, "# Message style")?;
    writeln!(output, "set-option -g message-style \"bg={},fg={}\"", sel_bg, fg_primary)?;
    writeln!(output, "set-option -g message-command-style \"bg={},fg={}\"", bg_primary, fg_primary)?;
    writeln!(output)?;

    writeln!(output, "# Copy mode")?;
    writeln!(output, "set-window-option -g mode-style \"bg={},fg={}\"", sel_bg, fg_primary)?;

    Ok(output)
}

pub fn run(file_path: &str, format: &str, output: Option<&str>) -> Result<()> {
    let content = tca_types::load_theme_file(file_path)?;

    let theme: Theme = toml::from_str(&content).context("Failed to parse theme file as TOML")?;

    let exported = match format.to_lowercase().as_str() {
        "base16" => export_base16(&theme)?,
        "kitty" => export_kitty(&theme)?,
        "alacritty" => export_alacritty(&theme)?,
        "vim" => export_vim(&theme)?,
        "helix" => export_helix(&theme)?,
        "starship" => export_starship(&theme)?,
        "vscode" => export_vscode(&theme)?,
        "iterm2" => export_iterm2(&theme)?,
        "tmux" => export_tmux(&theme)?,
        _ => return Err(anyhow::anyhow!(
            "Unknown format: {}. Available formats: base16, kitty, alacritty, vim, helix, starship, vscode, iterm2, tmux",
            format
        )),
    };

    if let Some(output_path) = output {
        fs::write(output_path, exported)
            .with_context(|| format!("Failed to write to file: {:?}", output_path))?;
        eprintln!("Exported to {:?}", output_path);
    } else {
        io::stdout().write_all(exported.as_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tca_types::{Ansi, Meta, Semantic, Ui, UiBg, UiBorder, UiCursor, UiFg, UiSelection};

    fn make_test_theme() -> Theme {
        Theme {
            meta: Meta {
                name: "Test Theme".to_string(),
                slug: None,
                author: Some("Test Author".to_string()),
                version: None,
                description: None,
                dark: Some(true),
            },
            ansi: Ansi {
                black: "#000000".to_string(),
                red: "#cc0000".to_string(),
                green: "#4e9a06".to_string(),
                yellow: "#c4a000".to_string(),
                blue: "#3465a4".to_string(),
                magenta: "#75507b".to_string(),
                cyan: "#06989a".to_string(),
                white: "#d3d7cf".to_string(),
                bright_black: "#555753".to_string(),
                bright_red: "#ef2929".to_string(),
                bright_green: "#8ae234".to_string(),
                bright_yellow: "#fce94f".to_string(),
                bright_blue: "#729fcf".to_string(),
                bright_magenta: "#ad7fa8".to_string(),
                bright_cyan: "#34e2e2".to_string(),
                bright_white: "#eeeeec".to_string(),
            },
            palette: None,
            base16: None,
            semantic: Semantic {
                error: "#cc0000".to_string(),
                warning: "#c4a000".to_string(),
                info: "#3465a4".to_string(),
                success: "#4e9a06".to_string(),
                highlight: "#c4a000".to_string(),
                link: "#06989a".to_string(),
            },
            ui: Ui {
                bg: UiBg {
                    primary: "#1c1c1c".to_string(),
                    secondary: "#2c2c2c".to_string(),
                },
                fg: UiFg {
                    primary: "#eeeeec".to_string(),
                    secondary: "#d3d7cf".to_string(),
                    muted: "#888a85".to_string(),
                },
                border: UiBorder {
                    primary: "#555753".to_string(),
                    muted: "#2c2c2c".to_string(),
                },
                cursor: UiCursor {
                    primary: "#eeeeec".to_string(),
                    muted: "#888a85".to_string(),
                },
                selection: UiSelection {
                    bg: "#3465a4".to_string(),
                    fg: "#eeeeec".to_string(),
                },
            },
        }
    }

    #[test]
    fn test_export_kitty() {
        let theme = make_test_theme();
        let out = export_kitty(&theme).unwrap();
        assert!(out.contains("# Test Theme"), "missing header");
        assert!(out.contains("color0 #000000"), "missing ANSI black");
        assert!(out.contains("color15 #eeeeec"), "missing ANSI bright_white");
        assert!(out.contains("background #1c1c1c"), "missing background");
        assert!(out.contains("foreground #eeeeec"), "missing foreground");
        assert!(out.contains("cursor #eeeeec"), "missing cursor");
        assert!(out.contains("selection_background #3465a4"), "missing selection_bg");
    }

    #[test]
    fn test_export_alacritty() {
        let theme = make_test_theme();
        let out = export_alacritty(&theme).unwrap();
        assert!(out.contains("[colors.primary]"), "missing primary section");
        assert!(out.contains("[colors.normal]"), "missing normal section");
        assert!(out.contains("[colors.bright]"), "missing bright section");
        assert!(out.contains("background = '#1c1c1c'"), "missing background");
        assert!(out.contains("foreground = '#eeeeec'"), "missing foreground");
        assert!(out.contains("black = '#000000'"), "missing normal black");
    }

    #[test]
    fn test_export_vim_dark() {
        let theme = make_test_theme();
        let out = export_vim(&theme).unwrap();
        assert!(out.contains("set background=dark"), "missing dark background");
        assert!(out.contains("let g:colors_name = 'TestTheme'"), "missing colors_name");
        assert!(out.contains("let g:terminal_color_0 = '#000000'"), "missing terminal color");
        assert!(out.contains("hi Normal guifg=#eeeeec guibg=#1c1c1c"), "missing Normal highlight");
    }

    #[test]
    fn test_export_vim_light() {
        let mut theme = make_test_theme();
        theme.meta.dark = Some(false);
        let out = export_vim(&theme).unwrap();
        assert!(out.contains("set background=light"), "should be light background");
    }

    #[test]
    fn test_export_helix() {
        let theme = make_test_theme();
        let out = export_helix(&theme).unwrap();
        assert!(out.contains("[ui]"), "missing ui section");
        assert!(out.contains("[palette]"), "missing palette section");
        assert!(out.contains("background = \"#1c1c1c\""), "missing background");
        assert!(out.contains("foreground = \"#eeeeec\""), "missing foreground");
        assert!(out.contains("black = \"#000000\""), "missing black palette entry");
    }

    #[test]
    fn test_export_starship() {
        let theme = make_test_theme();
        let out = export_starship(&theme).unwrap();
        assert!(out.contains("[palettes.tca]"), "missing palette section");
        assert!(out.contains("palette = \"tca\""), "missing palette enable");
        assert!(out.contains("black = \"#000000\""), "missing black entry");
    }

    #[test]
    fn test_export_vscode() {
        let theme = make_test_theme();
        let out = export_vscode(&theme).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
        assert_eq!(v["type"], "dark");
        assert_eq!(v["name"], "Test Theme");
        assert_eq!(v["colors"]["editor.background"], "#1c1c1c");
        assert_eq!(v["colors"]["editor.foreground"], "#eeeeec");
        assert_eq!(v["colors"]["editorCursor.foreground"], "#eeeeec");
        assert_eq!(v["colors"]["sideBar.background"], "#2c2c2c");
        assert_eq!(v["colors"]["activityBar.background"], "#1c1c1c");
        assert_eq!(v["colors"]["terminal.ansiBlack"], "#000000");
        assert_eq!(v["colors"]["editorError.foreground"], "#cc0000");
    }

    #[test]
    fn test_export_iterm2() {
        let theme = make_test_theme();
        let out = export_iterm2(&theme).unwrap();
        assert!(out.contains("<?xml"), "missing XML header");
        assert!(out.contains("<plist"), "missing plist tag");
        assert!(out.contains("Ansi 0 Color"), "missing ANSI 0");
        assert!(out.contains("Background Color"), "missing background");
        assert!(out.contains("Foreground Color"), "missing foreground");
        assert!(out.contains("sRGB"), "missing color space");
    }

    #[test]
    fn test_export_tmux() {
        let theme = make_test_theme();
        let out = export_tmux(&theme).unwrap();
        assert!(out.contains("# TCA Theme: Test Theme"), "missing header");
        assert!(out.contains("set-option -g status-style"), "missing status-style");
        assert!(out.contains("set-option -g pane-border-style"), "missing pane-border");
        assert!(out.contains("set-option -g pane-active-border-style"), "missing active border");
    }
}
