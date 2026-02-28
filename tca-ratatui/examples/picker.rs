use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal, Frame,
};
use tca_ratatui::{load_all_from_dir, load_all_from_theme_dir};
use tca_ratatui::{ColorPicker, TcaTheme};

use std::{env, io};

struct App {
    themes: Vec<TcaTheme>,
    theme_index: usize,
    exit: bool,
}

impl App {
    fn new(themes: Vec<TcaTheme>) -> Self {
        Self {
            themes,
            theme_index: 0,
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
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q' | 'Q') => self.exit = true,
            KeyCode::Left => {
                self.theme_index = self.theme_index.saturating_sub(1);
            }
            KeyCode::Right => {
                self.theme_index = (self.theme_index + 1).min(self.themes.len().saturating_sub(1));
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        if let Some(theme) = &self.themes.get(self.theme_index) {
            let picker = ColorPicker::new(theme)
                .title("TCA Theme Picker")
                .instructions("◀ Previous | Next ▶ | Quit Q");
            frame.render_widget(picker, frame.area());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let themes_dir: Option<String> = env::args().nth(1);

    let mut themes = match &themes_dir {
        Some(dir) => load_all_from_dir(dir)?,
        None => load_all_from_theme_dir()?,
    };
    themes.sort_by_key(|t| t.meta.name.to_string());

    if themes.is_empty() {
        eprintln!(
            "No themes found in {:?}",
            themes_dir.unwrap_or("user theme directory.".to_string())
        );
        eprintln!(
            "Usage: {} [theme-directory]",
            env::args().next().unwrap_or_default()
        );
        return Ok(());
    }

    Ok(ratatui::run(|terminal| App::new(themes).run(terminal))?)
}
