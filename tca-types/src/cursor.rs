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
    use crate::Theme;

    fn make_theme(name: &str) -> Theme {
        // Build a minimal base24 YAML string so tests don't depend on struct internals.
        let yaml = format!(
            r#"scheme: "{name}"
author: "test"
base00: "1c1c1c"
base01: "2c2c2c"
base02: "3c3c3c"
base03: "555753"
base04: "888a85"
base05: "eeeeec"
base06: "d3d7cf"
base07: "eeeeec"
base08: "cc0000"
base09: "c4a000"
base0A: "c4a000"
base0B: "4e9a06"
base0C: "06989a"
base0D: "3465a4"
base0E: "75507b"
base0F: "cc0000"
base10: "1c1c1c"
base11: "000000"
base12: "ef2929"
base13: "fce94f"
base14: "8ae234"
base15: "34e2e2"
base16: "729fcf"
base17: "ad7fa8"
"#
        );
        Theme::from_base24_str(&yaml).expect("test theme YAML is invalid")
    }

    fn cursor_with_names(names: &[&str]) -> ThemeCursor<Theme> {
        ThemeCursor::new(names.iter().map(|n| make_theme(n)))
    }

    fn name(t: Option<&Theme>) -> &str {
        t.map(|t| t.meta.name.as_str()).unwrap_or("<none>")
    }

    // --- empty cursor ---

    #[test]
    fn empty_cursor() {
        let mut c = ThemeCursor::<Theme>::new(vec![]);
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
        assert!(c.peek().is_none());
        assert!(c.next().is_none());
        assert!(c.prev().is_none());
        assert!(c.themes().is_empty());
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
