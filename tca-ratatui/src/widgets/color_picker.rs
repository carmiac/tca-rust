use crate::{ColorRamp, TcaTheme};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct ColorPicker<'a> {
    theme: &'a TcaTheme,
    title: Option<String>,
    instructions: Option<String>,
}

impl<'a> ColorPicker<'a> {
    pub const fn new(theme: &'a TcaTheme) -> Self {
        Self {
            theme,
            title: None,
            instructions: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }
}

impl Widget for ColorPicker<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        let border_color = theme.ui.as_ref()
            .map(|ui| ui.border_muted)
            .unwrap_or(theme.ansi.white);

        let title_color = theme.ui.as_ref()
            .map(|ui| ui.fg_primary)
            .unwrap_or(theme.ansi.white);

        let mut block = Block::bordered()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title_style(Style::default().fg(title_color));

        if let Some(title) = self.title {
            block = block.title(Line::from(title).centered());
        }

        if let Some(instructions) = self.instructions {
            block = block.title_bottom(Line::from(instructions).centered());
        }

        let inner = block.inner(area);
        block.render(area, buf);

        let chunks = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(inner);

        let mut left_lines = vec![
            Line::from(format!("Theme: {}", theme.name())).style(Style::default().fg(title_color)),
            Line::from(""),
            Line::from("Palette:"),
        ];

        let neutral_line = render_colored_ramp("neutral", &theme.palette.neutral);
        left_lines.push(neutral_line);
        
        for name in theme.palette.ramp_names() {
            if let Some(ramp) = theme.palette.get_ramp(name) {
                left_lines.push(render_colored_ramp(name, ramp));
            }
        }

        left_lines.push(Line::from(""));
        left_lines.push(Line::from("ANSI Colors:"));
        left_lines.extend([
            Line::from("  black").style(Style::default().fg(theme.ansi.black)),
            Line::from("  red").style(Style::default().fg(theme.ansi.red)),
            Line::from("  green").style(Style::default().fg(theme.ansi.green)),
            Line::from("  yellow").style(Style::default().fg(theme.ansi.yellow)),
            Line::from("  blue").style(Style::default().fg(theme.ansi.blue)),
            Line::from("  magenta").style(Style::default().fg(theme.ansi.magenta)),
            Line::from("  cyan").style(Style::default().fg(theme.ansi.cyan)),
            Line::from("  white").style(Style::default().fg(theme.ansi.white)),
            Line::from("  bright_black").style(Style::default().fg(theme.ansi.bright_black)),
            Line::from("  bright_red").style(Style::default().fg(theme.ansi.bright_red)),
            Line::from("  bright_green").style(Style::default().fg(theme.ansi.bright_green)),
            Line::from("  bright_yellow").style(Style::default().fg(theme.ansi.bright_yellow)),
            Line::from("  bright_blue").style(Style::default().fg(theme.ansi.bright_blue)),
            Line::from("  bright_magenta").style(Style::default().fg(theme.ansi.bright_magenta)),
            Line::from("  bright_cyan").style(Style::default().fg(theme.ansi.bright_cyan)),
            Line::from("  bright_white").style(Style::default().fg(theme.ansi.bright_white)),
        ]);

        let mut right_lines = vec![];

        if let Some(semantic) = &theme.semantic {
            right_lines.push(Line::from("Semantic Colors:"));
            right_lines.extend([
                Line::from("  error").style(Style::default().fg(semantic.error)),
                Line::from("  warning").style(Style::default().fg(semantic.warning)),
                Line::from("  success").style(Style::default().fg(semantic.success)),
                Line::from("  info").style(Style::default().fg(semantic.info)),
                Line::from("  highlight").style(Style::default().fg(semantic.highlight)),
                Line::from("  link").style(Style::default().fg(semantic.link)),
            ]);
            right_lines.push(Line::from(""));
        }

        if let Some(ui) = &theme.ui {
            right_lines.push(Line::from("UI Colors:"));
            right_lines.extend([
                Line::from("  bg_primary").style(Style::default().fg(ui.bg_primary).bg(ui.fg_primary)),
                Line::from("  bg_secondary").style(Style::default().fg(ui.bg_secondary).bg(ui.fg_primary)),
                Line::from("  fg_primary").style(Style::default().fg(ui.fg_primary)),
                Line::from("  fg_secondary").style(Style::default().fg(ui.fg_secondary)),
                Line::from("  fg_muted").style(Style::default().fg(ui.fg_muted)),
                Line::from("  border_primary").style(Style::default().fg(ui.border_primary)),
                Line::from("  border_muted").style(Style::default().fg(ui.border_muted)),
                Line::from("  cursor_primary").style(Style::default().bg(ui.cursor_primary)),
                Line::from("  cursor_muted").style(Style::default().bg(ui.cursor_muted)),
                Line::from("  selection_bg").style(Style::default().bg(ui.selection_bg).fg(ui.fg_primary)),
                Line::from("  selection_fg").style(Style::default().fg(ui.selection_fg)),
            ]);
        }

        Paragraph::new(left_lines).render(chunks[0], buf);
        Paragraph::new(right_lines).render(chunks[1], buf);
    }
}

fn render_colored_ramp(name: &str, ramp: &ColorRamp) -> Line<'static> {
    let mut spans = vec![
        ratatui::text::Span::raw(format!("  {}: ", name)),
    ];
    for tone in ramp.tones() {
        if let Some(color) = ramp.get(tone) {
            spans.push(ratatui::text::Span::styled("█", Style::default().fg(color)));
        }
    }
    Line::from(spans)
}
