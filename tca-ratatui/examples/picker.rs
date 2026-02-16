use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal, Frame,
};
use std::{env, fs, io, path::Path};
use tca_ratatui::{ColorPicker, TcaTheme};

struct App {
    themes: Vec<TcaTheme>,
    current_index: usize,
    exit: bool,
}

impl App {
    fn new(themes: Vec<TcaTheme>) -> Self {
        Self {
            themes,
            current_index: 0,
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
                if self.current_index > 0 {
                    self.current_index -= 1;
                } else {
                    self.current_index = self.themes.len() - 1;
                }
            }
            KeyCode::Right => {
                self.current_index = (self.current_index + 1) % self.themes.len();
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let theme = &self.themes[self.current_index];

        let picker = ColorPicker::new(theme)
            .title("TCA Theme Picker")
            .instructions("◀ Previous | Next ▶ | Quit Q");

        frame.render_widget(picker, frame.area());
    }
}

fn load_themes_from_directory(dir: &Path) -> io::Result<Vec<TcaTheme>> {
    let mut themes = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if matches!(
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
            "yaml" | "yml"
        ) {
            match TcaTheme::from_file(&path) {
                Ok(theme) => themes.push(theme),
                Err(e) => eprintln!("Failed to load {:?}: {}", path, e),
            }
        }
    }

    themes.sort_by(|a, b| a.meta.name.cmp(&b.meta.name));
    Ok(themes)
}

fn main() -> io::Result<()> {
    let themes = if let Some(dir) = env::args().nth(1) {
        let path = Path::new(&dir);
        if !path.is_dir() {
            eprintln!("Error: '{}' is not a directory", dir);
            eprintln!("Usage: {} <theme-directory>", env::args().next().unwrap());
            return Ok(());
        }
        load_themes_from_directory(path)?
    } else {
        eprintln!("No themes loaded!");
        eprintln!("Usage: {} <theme-directory>", env::args().next().unwrap());
        return Ok(());
    };

    if themes.is_empty() {
        eprintln!("No themes found in {:?}", env::args().nth(1).unwrap());
        eprintln!("Usage: {} <theme-directory>", env::args().next().unwrap());
        return Ok(());
    }

    ratatui::run(|terminal| App::new(themes).run(terminal))
}
