use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal, Frame,
};
use tca_ratatui::{ColorPicker, TcaTheme, TcaThemeCursor};

use std::{env, io};

struct App {
    themes: TcaThemeCursor,
    exit: bool,
}

impl App {
    fn new(themes: TcaThemeCursor) -> Self {
        Self {
            themes,
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
                self.themes.prev();
            }
            KeyCode::Right => {
                self.themes.next();
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        if let Some(theme) = &self.themes.peek() {
            let picker = ColorPicker::new(theme)
                .title("TCA Theme Picker")
                .instructions("◀ Previous | Next ▶ | Quit Q");
            frame.render_widget(picker, frame.area());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("Usage: picker [OPTIONS] [THEME_DIR]");
        println!();
        println!("Arguments:");
        println!("  [THEME_DIR]   Load themes from a specific directory");
        println!();
        println!("Options:");
        println!("  --builtin     Only load built-in themes");
        println!("  --user        Only load user themes");
        println!("  -h, --help    Print this help message");
        println!();
        println!("Keys:");
        println!("  ◀ / ▶   Previous / next theme");
        println!("  Q        Quit");
        return Ok(());
    }

    let builtin_flag = args.iter().any(|a| a == "--builtin");
    let user_flag = args.iter().any(|a| a == "--user");
    let themes_dir = args.iter().skip(1).find(|a| !a.starts_with('-'));

    let themes = match themes_dir {
        Some(dir) => TcaThemeCursor::new(
            tca_types::all_from_dir(dir)
                .into_iter()
                .map(|t| TcaTheme::try_from(t).unwrap_or_default()),
        ),
        None => {
            if builtin_flag {
                TcaThemeCursor::with_builtins()
            } else if user_flag {
                TcaThemeCursor::with_user_themes()
            } else {
                TcaThemeCursor::with_all_themes()
            }
        }
    };
    if themes.is_empty() {
        eprintln!(
            "No themes found in {:?}",
            themes_dir.unwrap_or(&"user theme directory.".to_string())
        );
        eprintln!(
            "Usage: {} [--builtin] [theme-directory]",
            args.first().map(String::as_str).unwrap_or("picker")
        );
        return Ok(());
    }

    Ok(ratatui::run(|terminal| App::new(themes).run(terminal))?)
}
