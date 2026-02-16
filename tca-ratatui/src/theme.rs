use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[cfg(feature = "loader")]
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Ansi {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

#[derive(Debug, Clone)]
pub struct Semantic {
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub success: Color,
    pub highlight: Color,
    pub link: Color,
}

impl Default for Semantic {
    fn default() -> Self {
        Self {
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
            success: Color::Green,
            highlight: Color::Cyan,
            link: Color::Blue,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ui {
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_muted: Color,
    pub border_primary: Color,
    pub border_muted: Color,
    pub cursor_primary: Color,
    pub cursor_muted: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            bg_primary: Color::Black,
            bg_secondary: Color::Black,
            fg_primary: Color::White,
            fg_secondary: Color::Gray,
            fg_muted: Color::DarkGray,
            border_primary: Color::White,
            border_muted: Color::DarkGray,
            cursor_primary: Color::White,
            cursor_muted: Color::Gray,
            selection_bg: Color::DarkGray,
            selection_fg: Color::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorRamp {
    colors: std::collections::HashMap<u8, Color>,
}

impl ColorRamp {
    pub fn get(&self, tone: u8) -> Option<Color> {
        self.colors.get(&tone).copied()
    }

    pub fn tones(&self) -> Vec<u8> {
        let mut tones: Vec<u8> = self.colors.keys().copied().collect();
        tones.sort_unstable();
        tones
    }
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub neutral: ColorRamp,
    pub ramps: std::collections::HashMap<String, ColorRamp>,
}

impl Palette {
    pub fn get_ramp(&self, name: &str) -> Option<&ColorRamp> {
        self.ramps.get(name)
    }

    pub fn ramp_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.ramps.keys().map(|s| s.as_str()).collect();
        names.sort_unstable();
        names
    }
}

#[derive(Debug, Clone)]
pub struct TcaTheme {
    pub meta: Meta,
    pub palette: Palette,
    pub ansi: Ansi,
    pub semantic: Semantic,
    pub ui: Ui,
}

impl TcaTheme {
    pub fn name(&self) -> &str {
        &self.meta.name
    }

    pub fn author(&self) -> Option<&str> {
        self.meta.author.as_deref()
    }
}

#[cfg(feature = "loader")]
pub struct ThemeLoader;

#[cfg(feature = "loader")]
impl ThemeLoader {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<TcaTheme> {
        let content = std::fs::read_to_string(path.as_ref())
            .context("Failed to read theme file")?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> Result<TcaTheme> {
        let raw: RawTheme = serde_yaml::from_str(content)
            .context("Failed to parse theme YAML")?;
        raw.into_theme()
    }
}

#[cfg(feature = "loader")]
impl TcaTheme {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        ThemeLoader::from_file(path)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        ThemeLoader::from_yaml(content)
    }
}

#[cfg(feature = "loader")]
#[derive(Debug, Deserialize)]
struct RawTheme {
    meta: Meta,
    palette: RawPalette,
    ansi: RawAnsi,
    #[serde(default)]
    semantic: Option<RawSemantic>,
    #[serde(default)]
    ui: Option<RawUi>,
}

#[cfg(feature = "loader")]
#[derive(Debug, Deserialize)]
struct RawPalette {
    neutral: std::collections::HashMap<String, String>,
    #[serde(flatten)]
    other: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

#[cfg(feature = "loader")]
#[derive(Debug, Deserialize)]
struct RawAnsi {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    #[serde(rename = "brightBlack")]
    bright_black: String,
    #[serde(rename = "brightRed")]
    bright_red: String,
    #[serde(rename = "brightGreen")]
    bright_green: String,
    #[serde(rename = "brightYellow")]
    bright_yellow: String,
    #[serde(rename = "brightBlue")]
    bright_blue: String,
    #[serde(rename = "brightMagenta")]
    bright_magenta: String,
    #[serde(rename = "brightCyan")]
    bright_cyan: String,
    #[serde(rename = "brightWhite")]
    bright_white: String,
}

#[cfg(feature = "loader")]
#[derive(Debug, Deserialize)]
struct RawSemantic {
    error: String,
    warning: String,
    info: String,
    success: String,
    highlight: String,
    link: String,
}

#[cfg(feature = "loader")]
#[derive(Debug, Deserialize)]
struct RawUi {
    #[serde(rename = "bg.primary")]
    bg_primary: String,
    #[serde(rename = "bg.secondary")]
    bg_secondary: String,
    #[serde(rename = "fg.primary")]
    fg_primary: String,
    #[serde(rename = "fg.secondary")]
    fg_secondary: String,
    #[serde(rename = "fg.muted")]
    fg_muted: String,
    #[serde(rename = "border.primary")]
    border_primary: String,
    #[serde(rename = "border.muted")]
    border_muted: String,
    #[serde(rename = "cursor.primary")]
    cursor_primary: String,
    #[serde(rename = "cursor.muted")]
    cursor_muted: String,
    #[serde(rename = "selection.bg")]
    selection_bg: String,
    #[serde(rename = "selection.fg")]
    selection_fg: String,
}

#[cfg(feature = "loader")]
impl RawTheme {
    fn into_theme(self) -> Result<TcaTheme> {
        let resolve = |reference: &str| -> Result<Color> {
            if reference.starts_with("palette.") {
                let parts: Vec<&str> = reference.split('.').collect();
                if parts.len() != 3 {
                    anyhow::bail!("Invalid palette reference: {}", reference);
                }

                let ramp_name = parts[1];
                let tone = parts[2];

                let hex = if ramp_name == "neutral" {
                    self.palette.neutral.get(tone)
                } else {
                    self.palette.other.get(ramp_name)
                        .and_then(|ramp| ramp.get(tone))
                }
                .context(format!("Color not found: {}", reference))?;

                parse_hex(hex)
            } else if reference.starts_with('#') {
                parse_hex(reference)
            } else {
                anyhow::bail!("Unknown reference format: {}", reference)
            }
        };

        let ansi = Ansi {
            black: resolve(&self.ansi.black)?,
            red: resolve(&self.ansi.red)?,
            green: resolve(&self.ansi.green)?,
            yellow: resolve(&self.ansi.yellow)?,
            blue: resolve(&self.ansi.blue)?,
            magenta: resolve(&self.ansi.magenta)?,
            cyan: resolve(&self.ansi.cyan)?,
            white: resolve(&self.ansi.white)?,
            bright_black: resolve(&self.ansi.bright_black)?,
            bright_red: resolve(&self.ansi.bright_red)?,
            bright_green: resolve(&self.ansi.bright_green)?,
            bright_yellow: resolve(&self.ansi.bright_yellow)?,
            bright_blue: resolve(&self.ansi.bright_blue)?,
            bright_magenta: resolve(&self.ansi.bright_magenta)?,
            bright_cyan: resolve(&self.ansi.bright_cyan)?,
            bright_white: resolve(&self.ansi.bright_white)?,
        };

        let semantic = self.semantic.map(|s| {
            Ok::<_, anyhow::Error>(Semantic {
                error: resolve(&s.error)?,
                warning: resolve(&s.warning)?,
                info: resolve(&s.info)?,
                success: resolve(&s.success)?,
                highlight: resolve(&s.highlight)?,
                link: resolve(&s.link)?,
            })
        }).transpose()?.unwrap_or_default();

        let ui = self.ui.map(|u| {
            Ok::<_, anyhow::Error>(Ui {
                bg_primary: resolve(&u.bg_primary)?,
                bg_secondary: resolve(&u.bg_secondary)?,
                fg_primary: resolve(&u.fg_primary)?,
                fg_secondary: resolve(&u.fg_secondary)?,
                fg_muted: resolve(&u.fg_muted)?,
                border_primary: resolve(&u.border_primary)?,
                border_muted: resolve(&u.border_muted)?,
                cursor_primary: resolve(&u.cursor_primary)?,
                cursor_muted: resolve(&u.cursor_muted)?,
                selection_bg: resolve(&u.selection_bg)?,
                selection_fg: resolve(&u.selection_fg)?,
            })
        }).transpose()?.unwrap_or_default();

        let mut palette_neutral_colors = std::collections::HashMap::new();
        for (tone_str, hex) in &self.palette.neutral {
            if let Ok(tone) = tone_str.parse::<u8>() {
                palette_neutral_colors.insert(tone, parse_hex(hex)?);
            }
        }
        
        let mut palette_ramps: std::collections::HashMap<String, ColorRamp> = std::collections::HashMap::new();
        for (ramp_name, ramp_map) in &self.palette.other {
            let mut colors = std::collections::HashMap::new();
            for (tone_str, hex) in ramp_map {
                if let Ok(tone) = tone_str.parse::<u8>() {
                    colors.insert(tone, parse_hex(hex)?);
                }
            }
            palette_ramps.insert(ramp_name.clone(), ColorRamp { colors });
        }

        let palette = Palette {
            neutral: ColorRamp { colors: palette_neutral_colors },
            ramps: palette_ramps,
        };

        Ok(TcaTheme {
            meta: self.meta,
            palette,
            ansi,
            semantic,
            ui,
        })
    }
}

#[cfg(feature = "loader")]
fn parse_hex(hex: &str) -> Result<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        anyhow::bail!("Invalid hex color: #{}", hex);
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(Color::Rgb(r, g, b))
}
