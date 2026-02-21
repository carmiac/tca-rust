use tca_types::*;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};

/// Resolve a color reference to its `#RRGGBB` hex value.
fn resolve_color(reference: &str, theme: &Theme) -> Result<String> {
    theme
        .resolve(reference)
        .context(format!("Failed to resolve color reference: '{}'", reference))
}

fn export_base16(theme: &Theme) -> Result<String> {
    let base16 = theme
        .base16
        .as_ref()
        .context("Theme does not have a base16 section. Base16 export requires this section.")?;

    let name = &theme.meta.name;
    let author = theme.meta.author.as_deref().unwrap_or("Unknown");

    let mut output = String::new();
    output.push_str(&format!("scheme: \"{}\"\n", name));
    output.push_str(&format!("author: \"{}\"\n", author));

    for i in 0..16 {
        let key = format!("base{:02X}", i);
        let reference = base16
            .0
            .get(&key)
            .context(format!("Missing base16 color: {}", key))?;
        let color = resolve_color(reference, theme)?;
        let hex = color.trim_start_matches('#');
        output.push_str(&format!("base{:02X}: \"{}\"\n", i, hex));
    }

    Ok(output)
}

fn export_kitty(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# {}\n", theme.meta.name));
    output.push_str("# Generated from TCA theme\n\n");

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
        output.push_str(&format!("{} {}\n", name, hex));
    }

    output.push_str("\n# UI Colors\n");
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let cursor = resolve_color(&theme.ui.cursor.primary, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;
    let sel_fg = resolve_color(&theme.ui.selection.fg, theme)?;
    output.push_str(&format!("foreground {}\n", fg));
    output.push_str(&format!("background {}\n", bg));
    output.push_str(&format!("cursor {}\n", cursor));
    output.push_str(&format!("selection_background {}\n", sel_bg));
    output.push_str(&format!("selection_foreground {}\n", sel_fg));

    Ok(output)
}

fn export_alacritty(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# {}\n", theme.meta.name));
    output.push_str("# Generated from TCA theme\n\n");
    output.push_str("[colors.primary]\n");

    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    output.push_str(&format!("foreground = '{}'\n", fg));
    output.push_str(&format!("background = '{}'\n", bg));

    output.push_str("\n[colors.normal]\n");
    output.push_str(&format!("black = '{}'\n", theme.ansi.black));
    output.push_str(&format!("red = '{}'\n", theme.ansi.red));
    output.push_str(&format!("green = '{}'\n", theme.ansi.green));
    output.push_str(&format!("yellow = '{}'\n", theme.ansi.yellow));
    output.push_str(&format!("blue = '{}'\n", theme.ansi.blue));
    output.push_str(&format!("magenta = '{}'\n", theme.ansi.magenta));
    output.push_str(&format!("cyan = '{}'\n", theme.ansi.cyan));
    output.push_str(&format!("white = '{}'\n", theme.ansi.white));

    output.push_str("\n[colors.bright]\n");
    output.push_str(&format!("black = '{}'\n", theme.ansi.bright_black));
    output.push_str(&format!("red = '{}'\n", theme.ansi.bright_red));
    output.push_str(&format!("green = '{}'\n", theme.ansi.bright_green));
    output.push_str(&format!("yellow = '{}'\n", theme.ansi.bright_yellow));
    output.push_str(&format!("blue = '{}'\n", theme.ansi.bright_blue));
    output.push_str(&format!("magenta = '{}'\n", theme.ansi.bright_magenta));
    output.push_str(&format!("cyan = '{}'\n", theme.ansi.bright_cyan));
    output.push_str(&format!("white = '{}'\n", theme.ansi.bright_white));

    Ok(output)
}

fn export_vim(theme: &Theme) -> Result<String> {
    let name = theme.meta.name.replace(' ', "");
    let display_name = &theme.meta.name;

    let mut output = String::new();
    output.push_str(&format!("\" {}\n", display_name));
    output.push_str("\" Generated from TCA theme\n\n");
    output.push_str("set background=dark\n");
    output.push_str("hi clear\n");
    output.push_str("if exists('syntax_on')\n");
    output.push_str("  syntax reset\n");
    output.push_str("endif\n");
    output.push_str(&format!("let g:colors_name = '{}'\n\n", name));

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
        output.push_str(&format!("let g:terminal_color_{} = '{}'\n", i, hex));
    }

    output.push_str("\n\" UI colors\n");
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    output.push_str(&format!("hi Normal guifg={} guibg={}\n", fg, bg));

    Ok(output)
}

fn export_helix(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# {}\n", theme.meta.name));
    output.push_str("# Generated from TCA theme\n\n");

    output.push_str("[ui]\n");
    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    output.push_str(&format!("background = \"{}\"\n", bg));
    output.push_str(&format!("foreground = \"{}\"\n", fg));

    output.push_str("\n[palette]\n");
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
        output.push_str(&format!("{} = \"{}\"\n", name, hex));
    }

    Ok(output)
}

fn export_starship(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# Starship configuration for {}\n", theme.meta.name));
    output.push_str("# Generated from TCA theme\n");
    output.push_str("# Add this to your starship.toml\n\n");

    output.push_str("[palettes.tca]\n");
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
        output.push_str(&format!("{} = \"{}\"\n", name, hex));
    }

    output.push_str("\n# Enable the palette\npalette = \"tca\"\n");

    Ok(output)
}

fn export_vscode(theme: &Theme) -> Result<String> {
    let name = &theme.meta.name;

    let mut theme_json = serde_json::json!({
        "name": name,
        "type": if theme.meta.dark.unwrap_or(true) { "dark" } else { "light" },
        "colors": {}
    });

    let colors = theme_json
        .get_mut("colors")
        .and_then(|c| c.as_object_mut())
        .context("Theme JSON missing 'colors' object")?;

    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;
    colors.insert("editor.background".to_string(), serde_json::json!(bg));
    colors.insert("editor.foreground".to_string(), serde_json::json!(fg));
    colors.insert("editor.selectionBackground".to_string(), serde_json::json!(sel_bg));

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
        colors.insert(key.to_string(), serde_json::json!(hex));
    }

    Ok(serde_json::to_string_pretty(&theme_json)?)
}

fn export_iterm2(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
    output.push_str("<plist version=\"1.0\">\n");
    output.push_str("<dict>\n");

    let hex_to_rgb_f = |hex: &str| -> Result<(f64, f64, f64)> {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;
        Ok((r, g, b))
    };

    let write_color = |output: &mut String, key: &str, hex: &str| -> Result<()> {
        let (r, g, b) = hex_to_rgb_f(hex)?;
        output.push_str(&format!("\t<key>{}</key>\n", key));
        output.push_str("\t<dict>\n");
        output.push_str("\t\t<key>Color Space</key><string>sRGB</string>\n");
        output.push_str(&format!("\t\t<key>Red Component</key><real>{}</real>\n", r));
        output.push_str(&format!("\t\t<key>Green Component</key><real>{}</real>\n", g));
        output.push_str(&format!("\t\t<key>Blue Component</key><real>{}</real>\n", b));
        output.push_str("\t</dict>\n");
        Ok(())
    };

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
        write_color(&mut output, key, hex)?;
    }

    let bg = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg = resolve_color(&theme.ui.fg.primary, theme)?;
    let cursor = resolve_color(&theme.ui.cursor.primary, theme)?;
    write_color(&mut output, "Background Color", &bg)?;
    write_color(&mut output, "Foreground Color", &fg)?;
    write_color(&mut output, "Cursor Color", &cursor)?;

    output.push_str("</dict>\n</plist>\n");
    Ok(output)
}

fn export_tmux(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# TCA Theme: {}\n", theme.meta.name));
    output.push_str("# Generated by tca-export\n");
    output.push_str("# Usage: source this file in your tmux.conf\n\n");

    let bg_primary = resolve_color(&theme.ui.bg.primary, theme)?;
    let fg_primary = resolve_color(&theme.ui.fg.primary, theme)?;
    let fg_muted = resolve_color(&theme.ui.fg.muted, theme)?;
    let sel_bg = resolve_color(&theme.ui.selection.bg, theme)?;

    let active_border = &theme.ansi.blue;
    let inactive_border = &theme.ansi.bright_black;

    output.push_str("# Status bar\n");
    output.push_str(&format!("set-option -g status-style \"bg={},fg={}\"\n", bg_primary, fg_primary));
    output.push_str(&format!("set-option -g status-left-style \"bg={},fg={}\"\n", bg_primary, fg_primary));
    output.push_str(&format!("set-option -g status-right-style \"bg={},fg={}\"\n", bg_primary, fg_primary));
    output.push('\n');

    output.push_str("# Window status\n");
    output.push_str(&format!("set-option -g window-status-style \"bg={},fg={}\"\n", bg_primary, fg_muted));
    output.push_str(&format!("set-option -g window-status-current-style \"bg={},fg={}\"\n", sel_bg, fg_primary));
    output.push('\n');

    output.push_str("# Pane borders\n");
    output.push_str(&format!("set-option -g pane-border-style \"fg={}\"\n", inactive_border));
    output.push_str(&format!("set-option -g pane-active-border-style \"fg={}\"\n", active_border));
    output.push('\n');

    output.push_str("# Message style\n");
    output.push_str(&format!("set-option -g message-style \"bg={},fg={}\"\n", sel_bg, fg_primary));
    output.push_str(&format!("set-option -g message-command-style \"bg={},fg={}\"\n", bg_primary, fg_primary));
    output.push('\n');

    output.push_str("# Copy mode\n");
    output.push_str(&format!("set-window-option -g mode-style \"bg={},fg={}\"\n", sel_bg, fg_primary));

    Ok(output)
}

pub fn run(file_path: &str, format: &str, output: Option<&str>) -> Result<()> {
    let content = tca_loader::load_theme_file(file_path)?;

    let theme: Theme =
        toml::from_str(&content).context("Failed to parse theme file as TOML")?;

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
            .context(format!("Failed to write to file: {:?}", output_path))?;
        eprintln!("Exported to {:?}", output_path);
    } else {
        io::stdout().write_all(exported.as_bytes())?;
    }

    Ok(())
}
