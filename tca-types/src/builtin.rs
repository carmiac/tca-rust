//! Built in hardcoded themes for tca-rust
use crate::theme::Theme;
use strum::EnumIter;
use strum::{EnumString, IntoStaticStr};

/// Enum of built in themes.

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    IntoStaticStr,
    serde::Serialize,
)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum BuiltinTheme {
    /// Catppuccin Mocha — dark, pastel-toned.
    CatppuccinMocha,
    /// Cyberpunk — high-contrast neon on dark.
    Cyberpunk,
    /// Dracula — the classic dark purple theme (default).
    #[default]
    Dracula,
    /// Everforest Dark — muted green forest tones.
    EverforestDark,
    /// Gruvbox Dark — warm retro earth tones.
    GruvboxDark,
    /// Mono — minimal monochrome.
    Mono,
    /// Nord — cool arctic blue palette.
    Nord,
    /// One Dark — Atom-inspired dark theme.
    OneDark,
    /// Rosé Pine — soho vibes, muted rose tones.
    RosePine,
    /// Solarized Light — the classic light theme.
    SolarizedLight,
    /// Tokyo Night — vibrant dark blues and purples.
    TokyoNight,
}

/// Built in themes to provide an easy on ramp for users who haven't installed theme files.
impl BuiltinTheme {
    /// Get the actual Theme from the Enum.
    pub fn theme(self) -> Theme {
        let src = match self {
            BuiltinTheme::CatppuccinMocha => include_str!("themes/catppuccin-mocha.toml"),
            BuiltinTheme::Cyberpunk => include_str!("themes/cyberpunk.toml"),
            BuiltinTheme::Dracula => include_str!("themes/dracula.toml"),
            BuiltinTheme::EverforestDark => include_str!("themes/everforest-dark.toml"),
            BuiltinTheme::GruvboxDark => include_str!("themes/gruvbox-dark.toml"),
            BuiltinTheme::Mono => include_str!("themes/mono.toml"),
            BuiltinTheme::Nord => include_str!("themes/nord-dark.toml"),
            BuiltinTheme::OneDark => include_str!("themes/one-dark.toml"),
            BuiltinTheme::RosePine => include_str!("themes/rose-pine.toml"),
            BuiltinTheme::SolarizedLight => include_str!("themes/solarized-light.toml"),
            BuiltinTheme::TokyoNight => include_str!("themes/tokyo-night.toml"),
        };

        toml::from_str(src).expect("Built in theme TOML is invalid.")
    }

    /// Returns a nice default light Theme.
    pub fn default_light() -> BuiltinTheme {
        BuiltinTheme::SolarizedLight
    }

    /// Iterator to get all the built in themes.
    pub fn iter() -> impl Iterator<Item = BuiltinTheme> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_themes_load_without_panic() {
        for variant in BuiltinTheme::iter() {
            let theme = variant.theme();
            assert!(!theme.meta.name.is_empty(), "{variant:?} has empty name");
        }
    }
}
