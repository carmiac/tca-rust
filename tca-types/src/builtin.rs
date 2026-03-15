#![allow(missing_docs)]
//! Built in hardcoded themes for tca-rust
use crate::Theme;
use strum::{EnumIter, EnumString, IntoStaticStr};

/// Enum of built in themes.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum BuiltinTheme {
    CatppuccinMocha,
    Cyberpunk,
    #[default]
    Dracula,
    EverforestDark,
    GruvboxDark,
    Mono,
    Nord,
    OneDark,
    RosePine,
    SolarizedLight,
    TokyoNight,
}

impl BuiltinTheme {
    pub fn theme(&self) -> Theme {
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

    pub fn default_dark() -> Self {
        BuiltinTheme::default()
    }
    pub fn default_light() -> Self {
        BuiltinTheme::SolarizedLight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn all_builtin_themes_load_without_panic() {
        for variant in BuiltinTheme::iter() {
            let theme = variant.theme();
            assert!(!theme.meta.name.is_empty(), "{variant:?} has empty name");
        }
    }
}
