use crate::{BuiltinTheme, Theme};

/// A cycling cursor over a list of themes of type `T`.
///
/// `peek()` returns the current theme without moving.
/// `next()` and `prev()` advance or retreat the cursor and return the new current theme.
/// Both wrap around at the ends.
///
/// To iterate over all themes without cycling, use [`ThemeCursor::themes`].
///
/// # Type parameters
///
/// - [`ThemeCursor<tca_types::Theme>`] — raw themes; see [`ThemeCursor::with_builtins`] etc.
/// - [`tca_ratatui::TcaThemeCursor`] — resolved Ratatui themes; convenience constructors
///   are available via `tca_ratatui`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeCursor<T> {
    themes: Vec<T>,
    index: usize,
}

impl<T> ThemeCursor<T> {
    /// Create a cursor from any iterable of themes. The cursor starts at the first theme.
    pub fn new(themes: impl IntoIterator<Item = T>) -> Self {
        let themes = themes.into_iter().collect();
        Self { index: 0, themes }
    }

    /// Returns the current theme without moving the cursor.
    pub fn peek(&self) -> Option<&T> {
        self.themes.get(self.index)
    }

    /// Advances the cursor to the next theme (wrapping) and returns it.
    ///
    /// Returns `None` if the cursor is empty.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.index = (self.index + 1) % self.themes.len();
        self.themes.get(self.index)
    }

    /// Retreats the cursor to the previous theme (wrapping) and returns it.
    ///
    /// Returns `None` if the cursor is empty.
    pub fn prev(&mut self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        if self.index == 0 {
            self.index = self.themes.len() - 1;
        } else {
            self.index -= 1;
        }
        self.themes.get(self.index)
    }

    /// Returns a slice of all themes in the cursor.
    pub fn themes(&self) -> &[T] {
        &self.themes
    }

    /// Returns the number of themes.
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Returns `true` if the cursor contains no themes.
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    /// Moves the cursor to `index` and returns the theme at that position.
    ///
    /// Returns `None` if `index` is out of bounds.
    pub fn set_index(&mut self, index: usize) -> Option<&T> {
        if index < self.themes.len() {
            self.index = index;
            self.themes.get(index)
        } else {
            None
        }
    }
}

/// Convenience constructors for [`ThemeCursor<Theme>`].
impl ThemeCursor<Theme> {
    /// All built-in themes.
    pub fn with_builtins() -> Self {
        ThemeCursor::new(BuiltinTheme::iter().map(|b| b.theme()))
    }

    /// User-installed themes only.
    #[cfg(feature = "fs")]
    pub fn with_user_themes() -> Self {
        use crate::all_user_themes;
        ThemeCursor::new(all_user_themes())
    }

    /// Built-ins + user themes. User themes with matching names override builtins.
    #[cfg(feature = "fs")]
    pub fn with_all_themes() -> Self {
        use crate::all_themes;
        ThemeCursor::new(all_themes())
    }

    /// Moves the cursor to the theme matching `name` (slug-insensitive) and returns it.
    ///
    /// Accepts fuzzy names: "Nord Dark", "nord-dark", and "nordDark" all match the same theme.
    /// Returns `None` if no matching theme is found.
    pub fn set_current(&mut self, name: &str) -> Option<&Theme> {
        let slug = heck::AsKebabCase(name).to_string();
        let idx = self.themes.iter().position(|t| t.name_slug() == slug)?;
        self.set_index(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ansi, Meta, Semantic, Theme, Ui, UiBg, UiBorder, UiCursor, UiFg, UiSelection};

    fn make_theme(name: &str) -> Theme {
        Theme {
            meta: Meta {
                name: name.to_string(),
                author: None,
                version: None,
                description: None,
                dark: None,
            },
            ansi: Ansi {
                black: "#000000".into(),
                red: "#cc0000".into(),
                green: "#4e9a06".into(),
                yellow: "#c4a000".into(),
                blue: "#3465a4".into(),
                magenta: "#75507b".into(),
                cyan: "#06989a".into(),
                white: "#d3d7cf".into(),
                bright_black: "#555753".into(),
                bright_red: "#ef2929".into(),
                bright_green: "#8ae234".into(),
                bright_yellow: "#fce94f".into(),
                bright_blue: "#729fcf".into(),
                bright_magenta: "#ad7fa8".into(),
                bright_cyan: "#34e2e2".into(),
                bright_white: "#eeeeec".into(),
            },
            palette: None,
            base16: None,
            semantic: Semantic {
                error: "#cc0000".into(),
                warning: "#c4a000".into(),
                info: "#3465a4".into(),
                success: "#4e9a06".into(),
                highlight: "#c4a000".into(),
                link: "#06989a".into(),
            },
            ui: Ui {
                bg: UiBg {
                    primary: "#1c1c1c".into(),
                    secondary: "#2c2c2c".into(),
                },
                fg: UiFg {
                    primary: "#eeeeec".into(),
                    secondary: "#d3d7cf".into(),
                    muted: "#888a85".into(),
                },
                border: UiBorder {
                    primary: "#555753".into(),
                    muted: "#2c2c2c".into(),
                },
                cursor: UiCursor {
                    primary: "#eeeeec".into(),
                    muted: "#888a85".into(),
                },
                selection: UiSelection {
                    bg: "#3465a4".into(),
                    fg: "#eeeeec".into(),
                },
            },
        }
    }

    fn cursor_with_names(names: &[&str]) -> ThemeCursor<Theme> {
        ThemeCursor::new(names.iter().map(|n| make_theme(n)))
    }

    fn name(t: Option<&Theme>) -> &str {
        t.map(|t| t.meta.name.as_str()).unwrap_or("<none>")
    }

    // --- empty cursor ---

    #[test]
    fn empty_cursor_peek_is_none() {
        let c = ThemeCursor::<Theme>::new(vec![]);
        assert!(c.peek().is_none());
    }

    #[test]
    fn empty_cursor_next_is_none() {
        let mut c = ThemeCursor::<Theme>::new(vec![]);
        assert!(c.next().is_none());
    }

    #[test]
    fn empty_cursor_prev_is_none() {
        let mut c = ThemeCursor::<Theme>::new(vec![]);
        assert!(c.prev().is_none());
    }

    #[test]
    fn empty_cursor_len_and_is_empty() {
        let c = ThemeCursor::<Theme>::new(vec![]);
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
    }

    // --- single-element cursor ---

    #[test]
    fn single_peek_returns_only_theme() {
        let c = cursor_with_names(&["Alpha"]);
        assert_eq!(name(c.peek()), "Alpha");
    }

    #[test]
    fn single_next_wraps_to_itself() {
        let mut c = cursor_with_names(&["Alpha"]);
        assert_eq!(name(c.next()), "Alpha");
        assert_eq!(name(c.next()), "Alpha");
    }

    #[test]
    fn single_prev_wraps_to_itself() {
        let mut c = cursor_with_names(&["Alpha"]);
        assert_eq!(name(c.prev()), "Alpha");
        assert_eq!(name(c.prev()), "Alpha");
    }

    // --- multi-element cursor ---

    #[test]
    fn peek_returns_first_on_creation() {
        let c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        assert_eq!(name(c.peek()), "Alpha");
    }

    #[test]
    fn next_advances_through_all_themes() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        assert_eq!(name(c.next()), "Beta");
        assert_eq!(name(c.next()), "Gamma");
    }

    #[test]
    fn next_wraps_from_last_to_first() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        c.next(); // Beta
        c.next(); // Gamma
        assert_eq!(name(c.next()), "Alpha"); // wraps
    }

    #[test]
    fn prev_wraps_from_first_to_last() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        assert_eq!(name(c.prev()), "Gamma"); // wraps back from first
    }

    #[test]
    fn prev_retreats_in_order() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        c.next(); // Beta
        c.next(); // Gamma
        assert_eq!(name(c.prev()), "Beta");
        assert_eq!(name(c.prev()), "Alpha");
    }

    #[test]
    fn next_then_prev_returns_to_start() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        c.next(); // Beta
        c.prev(); // back to Alpha
        assert_eq!(name(c.peek()), "Alpha");
    }

    #[test]
    fn full_cycle_forward_returns_to_start() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        c.next();
        c.next();
        c.next(); // wraps back to Alpha
        assert_eq!(name(c.peek()), "Alpha");
    }

    #[test]
    fn full_cycle_backward_returns_to_start() {
        let mut c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        c.prev(); // Gamma
        c.prev(); // Beta
        c.prev(); // Alpha
        assert_eq!(name(c.peek()), "Alpha");
    }

    // --- themes() accessor ---

    #[test]
    fn themes_returns_all_in_order() {
        let c = cursor_with_names(&["Alpha", "Beta", "Gamma"]);
        let names: Vec<&str> = c.themes().iter().map(|t| t.meta.name.as_str()).collect();
        assert_eq!(names, vec!["Alpha", "Beta", "Gamma"]);
    }

    #[test]
    fn themes_is_empty_for_empty_cursor() {
        let c = ThemeCursor::<Theme>::new(vec![]);
        assert!(c.themes().is_empty());
    }

    // --- with_builtins ---

    #[test]
    fn with_builtins_is_non_empty() {
        let c = ThemeCursor::with_builtins();
        assert!(!c.is_empty());
        assert!(c.peek().is_some());
    }

    #[test]
    fn with_builtins_can_cycle() {
        let mut c = ThemeCursor::with_builtins();
        let first = c.peek().unwrap().meta.name.clone();
        // Cycle forward through all themes and wrap back
        for _ in 0..c.len() {
            c.next();
        }
        assert_eq!(c.peek().unwrap().meta.name, first);
    }
}
