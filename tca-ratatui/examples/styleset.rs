//! Demonstrates using [`StyleSet`] to theme a ratatui application.
//!
//! Run with:
//! ```bash
//! cargo run --example styleset
//! cargo run --example styleset -- tokyo-night
//! cargo run --example styleset -- path/to/theme.yaml
//! ```
//!
//! Keys: <- -> cycle themes, Q quit.

use std::{env, io};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use tca_ratatui::StyleSetCursor;

struct App {
    cursor: StyleSetCursor,
    exit: bool,
}

impl App {
    fn new(cursor: StyleSetCursor) -> Self {
        Self {
            cursor,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q' | 'Q') => self.exit = true,
            KeyCode::Left => {
                self.cursor.prev();
            }
            KeyCode::Right => {
                self.cursor.next();
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let styles = self.cursor.peek().unwrap_or_default();
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // title bar
                Constraint::Min(0),    // content
                Constraint::Length(1), // status bar
            ])
            .split(area);

        // Title bar using primary style
        let title = Paragraph::new("StyleSet Demo:  ← → cycle themes, Q quit")
            .block(Block::default().borders(Borders::ALL).style(styles.border))
            .style(styles.primary);
        frame.render_widget(title, chunks[0]);

        // Content area: show all styles in a grid
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let left = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("primary    ", styles.primary),
                Span::styled(" normal text ", styles.primary),
            ]),
            Line::from(vec![
                Span::styled("secondary  ", styles.primary),
                Span::styled(" sidebar / panels ", styles.secondary),
            ]),
            Line::from(vec![
                Span::styled("muted      ", styles.primary),
                Span::styled(" disabled / hints ", styles.muted),
            ]),
            Line::from(vec![
                Span::styled("selection  ", styles.primary),
                Span::styled(" selected item ", styles.selection),
            ]),
            Line::from(vec![
                Span::styled("cursor     ", styles.primary),
                Span::styled(" ▌ cursor ", styles.cursor),
            ]),
            Line::from(vec![
                Span::styled("cursor_mut ", styles.primary),
                Span::styled(" ▌ inactive ", styles.cursor_muted),
            ]),
        ])
        .block(
            Block::default()
                .title(" UI Styles ")
                .borders(Borders::ALL)
                .style(styles.border),
        )
        .style(styles.primary);

        let right = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("error      ", styles.primary),
                Span::styled(" something went wrong ", styles.error),
            ]),
            Line::from(vec![
                Span::styled("warning    ", styles.primary),
                Span::styled(" proceed with caution ", styles.warning),
            ]),
            Line::from(vec![
                Span::styled("info       ", styles.primary),
                Span::styled(" loading… ", styles.info),
            ]),
            Line::from(vec![
                Span::styled("success    ", styles.primary),
                Span::styled(" all good! ", styles.success),
            ]),
            Line::from(vec![
                Span::styled("highlight  ", styles.primary),
                Span::styled(" important text ", styles.highlight),
            ]),
            Line::from(vec![
                Span::styled("link       ", styles.primary),
                Span::styled(" https://example.com ", styles.link),
            ]),
            Line::from(vec![
                Span::styled("border     ", styles.primary),
                Span::styled(" ┌─────────┐ ", styles.border),
            ]),
            Line::from(vec![
                Span::styled("border_mut ", styles.primary),
                Span::styled(" ┌─────────┐ ", styles.border_muted),
            ]),
        ])
        .block(
            Block::default()
                .title(" Semantic Styles ")
                .borders(Borders::ALL)
                .style(styles.border),
        )
        .style(styles.primary);

        frame.render_widget(left, content_chunks[0]);
        frame.render_widget(right, content_chunks[1]);

        // Status bar
        let status = Paragraph::new(format!(
            " Theme {}/{} — ← → to cycle",
            self.cursor.index() + 1,
            self.cursor.len()
        ))
        .style(styles.muted);
        frame.render_widget(status, chunks[2]);
    }
}

fn main() -> anyhow::Result<()> {
    let arg = env::args().nth(1);

    let cursor = match arg.as_deref() {
        Some(name) => {
            let mut c = StyleSetCursor::with_all_themes();
            c.set_current(name);
            c
        }
        None => StyleSetCursor::with_all_themes(),
    };

    if cursor.is_empty() {
        eprintln!("No themes found.");
        return Ok(());
    }

    Ok(ratatui::run(|terminal| App::new(cursor).run(terminal))?)
}
