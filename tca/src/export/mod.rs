use tca_types::*;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};

fn resolve_color(reference: &str, theme: &Theme) -> Result<String> {
    if reference.starts_with("palette.") {
        let parts: Vec<&str> = reference.split('.').collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid palette reference: {}", reference);
        }

        let ramp_name = parts[1];
        let tone = parts[2];

        if ramp_name == "neutral" {
            theme
                .palette
                .neutral
                .0
                .get(tone)
                .cloned()
                .context(format!("Color not found: {}", reference))
        } else {
            theme
                .palette
                .other
                .get(ramp_name)
                .and_then(|ramp| ramp.0.get(tone))
                .cloned()
                .context(format!("Color not found: {}", reference))
        }
    } else if reference.starts_with("semantic.") {
        let semantic = theme.semantic.as_ref().context("No semantic section")?;
        let key = reference.strip_prefix("semantic.").unwrap();
        let color_ref = semantic
            .get(key)
            .context(format!("Semantic color not found: {}", key))?;
        resolve_color(color_ref, theme)
    } else if reference.starts_with("#") {
        Ok(reference.to_string())
    } else {
        anyhow::bail!("Unknown reference format: {}", reference)
    }
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

    // Export base00-base0F in order
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

    let name = &theme.meta.name;

    output.push_str(&format!("# {}\n", name));
    output.push_str("# Generated from TCA theme\n\n");

    // ANSI colors
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

    for (name, reference) in ansi_colors {
        let color = resolve_color(reference, theme)?;
        output.push_str(&format!("{} {}\n", name, color));
    }

    // UI colors if available
    if let Some(ui) = &theme.ui {
        output.push_str("\n# UI Colors\n");

        if let Some(fg_ref) = ui.get("fg.primary") {
            let color = resolve_color(fg_ref, theme)?;
            output.push_str(&format!("foreground {}\n", color));
        }

        if let Some(bg_ref) = ui.get("bg.primary") {
            let color = resolve_color(bg_ref, theme)?;
            output.push_str(&format!("background {}\n", color));
        }

        if let Some(cursor_ref) = ui.get("cursor.primary") {
            let color = resolve_color(cursor_ref, theme)?;
            output.push_str(&format!("cursor {}\n", color));
        }

        if let Some(sel_bg_ref) = ui.get("selection.bg") {
            let color = resolve_color(sel_bg_ref, theme)?;
            output.push_str(&format!("selection_background {}\n", color));
        }

        if let Some(sel_fg_ref) = ui.get("selection.fg") {
            let color = resolve_color(sel_fg_ref, theme)?;
            output.push_str(&format!("selection_foreground {}\n", color));
        }
    }

    Ok(output)
}

fn export_alacritty(theme: &Theme) -> Result<String> {
    let name = &theme.meta.name;

    let mut output = String::new();
    output.push_str(&format!("# {}\n", name));
    output.push_str("# Generated from TCA theme\n\n");
    output.push_str("[colors.primary]\n");

    if let Some(ui) = &theme.ui {
        if let Some(fg_ref) = ui.get("fg.primary") {
            let color = resolve_color(fg_ref, theme)?;
            output.push_str(&format!("foreground = '{}'\n", color));
        }
        if let Some(bg_ref) = ui.get("bg.primary") {
            let color = resolve_color(bg_ref, theme)?;
            output.push_str(&format!("background = '{}'\n", color));
        }
    }

    output.push_str("\n[colors.normal]\n");
    let black = resolve_color(&theme.ansi.black, theme)?;
    let red = resolve_color(&theme.ansi.red, theme)?;
    let green = resolve_color(&theme.ansi.green, theme)?;
    let yellow = resolve_color(&theme.ansi.yellow, theme)?;
    let blue = resolve_color(&theme.ansi.blue, theme)?;
    let magenta = resolve_color(&theme.ansi.magenta, theme)?;
    let cyan = resolve_color(&theme.ansi.cyan, theme)?;
    let white = resolve_color(&theme.ansi.white, theme)?;

    output.push_str(&format!("black = '{}'\n", black));
    output.push_str(&format!("red = '{}'\n", red));
    output.push_str(&format!("green = '{}'\n", green));
    output.push_str(&format!("yellow = '{}'\n", yellow));
    output.push_str(&format!("blue = '{}'\n", blue));
    output.push_str(&format!("magenta = '{}'\n", magenta));
    output.push_str(&format!("cyan = '{}'\n", cyan));
    output.push_str(&format!("white = '{}'\n", white));

    output.push_str("\n[colors.bright]\n");
    let bright_black = resolve_color(&theme.ansi.bright_black, theme)?;
    let bright_red = resolve_color(&theme.ansi.bright_red, theme)?;
    let bright_green = resolve_color(&theme.ansi.bright_green, theme)?;
    let bright_yellow = resolve_color(&theme.ansi.bright_yellow, theme)?;
    let bright_blue = resolve_color(&theme.ansi.bright_blue, theme)?;
    let bright_magenta = resolve_color(&theme.ansi.bright_magenta, theme)?;
    let bright_cyan = resolve_color(&theme.ansi.bright_cyan, theme)?;
    let bright_white = resolve_color(&theme.ansi.bright_white, theme)?;

    output.push_str(&format!("black = '{}'\n", bright_black));
    output.push_str(&format!("red = '{}'\n", bright_red));
    output.push_str(&format!("green = '{}'\n", bright_green));
    output.push_str(&format!("yellow = '{}'\n", bright_yellow));
    output.push_str(&format!("blue = '{}'\n", bright_blue));
    output.push_str(&format!("magenta = '{}'\n", bright_magenta));
    output.push_str(&format!("cyan = '{}'\n", bright_cyan));
    output.push_str(&format!("white = '{}'\n", bright_white));

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

    // Terminal colors
    for i in 0..16 {
        let color_ref = match i {
            0 => &theme.ansi.black,
            1 => &theme.ansi.red,
            2 => &theme.ansi.green,
            3 => &theme.ansi.yellow,
            4 => &theme.ansi.blue,
            5 => &theme.ansi.magenta,
            6 => &theme.ansi.cyan,
            7 => &theme.ansi.white,
            8 => &theme.ansi.bright_black,
            9 => &theme.ansi.bright_red,
            10 => &theme.ansi.bright_green,
            11 => &theme.ansi.bright_yellow,
            12 => &theme.ansi.bright_blue,
            13 => &theme.ansi.bright_magenta,
            14 => &theme.ansi.bright_cyan,
            15 => &theme.ansi.bright_white,
            _ => unreachable!(),
        };
        let color = resolve_color(color_ref, theme)?;
        output.push_str(&format!("let g:terminal_color_{} = '{}'\n", i, color));
    }

    output.push_str("\n\" UI colors\n");
    if let Some(ui) = &theme.ui
        && let Some(bg_ref) = ui.get("bg.primary")
        && let Some(fg_ref) = ui.get("fg.primary") {
        let bg = resolve_color(bg_ref, theme)?;
        let fg = resolve_color(fg_ref, theme)?;
        output.push_str(&format!("hi Normal guifg={} guibg={}\n", fg, bg));
    }

    Ok(output)
}

fn export_helix(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    let name = &theme.meta.name;

    output.push_str(&format!("# {}\n", name));
    output.push_str("# Generated from TCA theme\n\n");

    // UI section
    if let Some(ui) = &theme.ui {
        output.push_str("[ui]\n");
        if let Some(bg_ref) = ui.get("bg.primary") {
            let bg = resolve_color(bg_ref, theme)?;
            output.push_str(&format!("background = \"{}\"\n", bg));
        }
        if let Some(fg_ref) = ui.get("fg.primary") {
            let fg = resolve_color(fg_ref, theme)?;
            output.push_str(&format!("foreground = \"{}\"\n", fg));
        }
    }

    // Palette section
    output.push_str("\n[palette]\n");
    for i in 0..16 {
        let (name, color_ref) = match i {
            0 => ("black", &theme.ansi.black),
            1 => ("red", &theme.ansi.red),
            2 => ("green", &theme.ansi.green),
            3 => ("yellow", &theme.ansi.yellow),
            4 => ("blue", &theme.ansi.blue),
            5 => ("magenta", &theme.ansi.magenta),
            6 => ("cyan", &theme.ansi.cyan),
            7 => ("white", &theme.ansi.white),
            8 => ("bright-black", &theme.ansi.bright_black),
            9 => ("bright-red", &theme.ansi.bright_red),
            10 => ("bright-green", &theme.ansi.bright_green),
            11 => ("bright-yellow", &theme.ansi.bright_yellow),
            12 => ("bright-blue", &theme.ansi.bright_blue),
            13 => ("bright-magenta", &theme.ansi.bright_magenta),
            14 => ("bright-cyan", &theme.ansi.bright_cyan),
            15 => ("bright-white", &theme.ansi.bright_white),
            _ => unreachable!(),
        };
        let color = resolve_color(color_ref, theme)?;
        output.push_str(&format!("{} = \"{}\"\n", name, color));
    }

    Ok(output)
}

fn export_starship(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    let name = &theme.meta.name;

    output.push_str(&format!("# Starship configuration for {}\n", name));
    output.push_str("# Generated from TCA theme\n");
    output.push_str("# Add this to your starship.toml\n\n");

    output.push_str("[palettes.tca]\n");
    for i in 0..16 {
        let (name, color_ref) = match i {
            0 => ("black", &theme.ansi.black),
            1 => ("red", &theme.ansi.red),
            2 => ("green", &theme.ansi.green),
            3 => ("yellow", &theme.ansi.yellow),
            4 => ("blue", &theme.ansi.blue),
            5 => ("magenta", &theme.ansi.magenta),
            6 => ("cyan", &theme.ansi.cyan),
            7 => ("white", &theme.ansi.white),
            8 => ("bright_black", &theme.ansi.bright_black),
            9 => ("bright_red", &theme.ansi.bright_red),
            10 => ("bright_green", &theme.ansi.bright_green),
            11 => ("bright_yellow", &theme.ansi.bright_yellow),
            12 => ("bright_blue", &theme.ansi.bright_blue),
            13 => ("bright_magenta", &theme.ansi.bright_magenta),
            14 => ("bright_cyan", &theme.ansi.bright_cyan),
            15 => ("bright_white", &theme.ansi.bright_white),
            _ => unreachable!(),
        };
        let color = resolve_color(color_ref, theme)?;
        output.push_str(&format!("{} = \"{}\"\n", name, color));
    }

    output.push_str("\n# Enable the palette\npalette = \"tca\"\n");

    Ok(output)
}

fn export_vscode(theme: &Theme) -> Result<String> {
    let name = &theme.meta.name;
    let _author = theme.meta.author.as_deref().unwrap_or("Unknown");

    let mut theme_json = serde_json::json!({
        "name": name,
        "type": "dark",
        "colors": {}
    });

    let colors = theme_json["colors"].as_object_mut().unwrap();

    if let Some(ui) = &theme.ui {
        if let Some(bg_ref) = ui.get("bg.primary") {
            let bg = resolve_color(bg_ref, theme)?;
            colors.insert("editor.background".to_string(), serde_json::json!(bg));
        }
        if let Some(fg_ref) = ui.get("fg.primary") {
            let fg = resolve_color(fg_ref, theme)?;
            colors.insert("editor.foreground".to_string(), serde_json::json!(fg));
        }
        if let Some(sel_bg_ref) = ui.get("selection.bg") {
            let sel_bg = resolve_color(sel_bg_ref, theme)?;
            colors.insert("editor.selectionBackground".to_string(), serde_json::json!(sel_bg));
        }
    }

    // Terminal colors
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

    for (key, color_ref) in terminal_colors {
        let color = resolve_color(color_ref, theme)?;
        colors.insert(key.to_string(), serde_json::json!(color));
    }

    let output = serde_json::to_string_pretty(&theme_json)?;
    Ok(output)
}

fn export_iterm2(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
    output.push_str("<plist version=\"1.0\">\n");
    output.push_str("<dict>\n");

    let hex_to_rgb = |hex: &str| -> Result<(f64, f64, f64)> {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;
        Ok((r, g, b))
    };

    let write_color = |output: &mut String, key: &str, hex: &str| -> Result<()> {
        let (r, g, b) = hex_to_rgb(hex)?;
        output.push_str(&format!("\t<key>{}</key>\n", key));
        output.push_str("\t<dict>\n");
        output.push_str("\t\t<key>Color Space</key>\n");
        output.push_str("\t\t<string>sRGB</string>\n");
        output.push_str("\t\t<key>Red Component</key>\n");
        output.push_str(&format!("\t\t<real>{}</real>\n", r));
        output.push_str("\t\t<key>Green Component</key>\n");
        output.push_str(&format!("\t\t<real>{}</real>\n", g));
        output.push_str("\t\t<key>Blue Component</key>\n");
        output.push_str(&format!("\t\t<real>{}</real>\n", b));
        output.push_str("\t</dict>\n");
        Ok(())
    };

    // ANSI colors
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

    for (key, color_ref) in ansi_colors {
        let color = resolve_color(color_ref, theme)?;
        write_color(&mut output, key, &color)?;
    }

    // UI colors
    if let Some(ui) = &theme.ui {
        if let Some(bg_ref) = ui.get("bg.primary") {
            let bg = resolve_color(bg_ref, theme)?;
            write_color(&mut output, "Background Color", &bg)?;
        }
        if let Some(fg_ref) = ui.get("fg.primary") {
            let fg = resolve_color(fg_ref, theme)?;
            write_color(&mut output, "Foreground Color", &fg)?;
        }
        if let Some(cursor_ref) = ui.get("cursor.primary") {
            let cursor = resolve_color(cursor_ref, theme)?;
            write_color(&mut output, "Cursor Color", &cursor)?;
        }
    }

    output.push_str("</dict>\n");
    output.push_str("</plist>\n");

    Ok(output)
}

fn export_tmux(theme: &Theme) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("# TCA Theme: {}\n", theme.meta.name));
    output.push_str("# Generated by tca-export\n");
    output.push_str("# Usage: source this file in your tmux.conf\n\n");

    let resolve = |color_ref: &str| -> Result<String> {
        let hex = resolve_color(color_ref, theme)?;
        Ok(hex)
    };

    let bg_primary = if let Some(ui) = &theme.ui {
        ui.get("bg.primary")
            .or_else(|| ui.get("bg_primary"))
            .and_then(|s| resolve(s).ok())
    } else {
        None
    };

    let fg_primary = if let Some(ui) = &theme.ui {
        ui.get("fg.primary")
            .or_else(|| ui.get("fg_primary"))
            .and_then(|s| resolve(s).ok())
    } else {
        None
    };

    let fg_muted = if let Some(ui) = &theme.ui {
        ui.get("fg.muted")
            .or_else(|| ui.get("fg_muted"))
            .and_then(|s| resolve(s).ok())
    } else {
        None
    };

    let selection_bg = if let Some(ui) = &theme.ui {
        ui.get("selection.bg")
            .or_else(|| ui.get("selection_bg"))
            .and_then(|s| resolve(s).ok())
    } else {
        None
    };

    let default_black = resolve(&theme.ansi.black)?;
    let default_white = resolve(&theme.ansi.white)?;
    let default_bright_black = resolve(&theme.ansi.bright_black)?;
    let default_blue = resolve(&theme.ansi.blue)?;

    let status_bg = bg_primary.as_deref().unwrap_or(&default_black);
    let status_fg = fg_primary.as_deref().unwrap_or(&default_white);
    let status_inactive_fg = fg_muted.as_deref().unwrap_or(&default_bright_black);
    let active_border = &default_blue;
    let inactive_border = &default_bright_black;
    let selection = selection_bg.as_deref().unwrap_or(&default_blue);

    output.push_str("# Status bar\n");
    output.push_str(&format!("set-option -g status-style \"bg={},fg={}\"\n", status_bg, status_fg));
    output.push_str(&format!("set-option -g status-left-style \"bg={},fg={}\"\n", status_bg, status_fg));
    output.push_str(&format!("set-option -g status-right-style \"bg={},fg={}\"\n", status_bg, status_fg));
    output.push('\n');

    output.push_str("# Window status\n");
    output.push_str(&format!("set-option -g window-status-style \"bg={},fg={}\"\n", status_bg, status_inactive_fg));
    output.push_str(&format!("set-option -g window-status-current-style \"bg={},fg={}\"\n", selection, status_fg));
    output.push('\n');

    output.push_str("# Pane borders\n");
    output.push_str(&format!("set-option -g pane-border-style \"fg={}\"\n", inactive_border));
    output.push_str(&format!("set-option -g pane-active-border-style \"fg={}\"\n", active_border));
    output.push('\n');

    output.push_str("# Message style\n");
    output.push_str(&format!("set-option -g message-style \"bg={},fg={}\"\n", selection, status_fg));
    output.push_str(&format!("set-option -g message-command-style \"bg={},fg={}\"\n", status_bg, status_fg));
    output.push('\n');

    output.push_str("# Copy mode\n");
    output.push_str(&format!("set-window-option -g mode-style \"bg={},fg={}\"\n", selection, status_fg));

    Ok(output)
}

pub fn run(file_path: &str, format: &str, output: Option<&str>) -> Result<()> {
    let content = tca_loader::load_theme_file(file_path)?;

    let theme: Theme = serde_yaml::from_str(&content).context("Failed to parse theme file")?;

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
        _ => return Err(anyhow::anyhow!("Unknown format: {}. Available formats: base16, kitty, alacritty, vim, helix, starship, vscode, iterm2, tmux", format)),
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
