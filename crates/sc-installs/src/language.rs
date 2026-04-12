//! Star Citizen localization languages.

use serde::{Deserialize, Serialize};

/// Supported Star Citizen localization languages.
///
/// These correspond to directory names under `<install>/data/Localization/`
/// in the game's installation.
///
/// **Initially ships with English only.** Star Citizen uses regional folder
/// name variants (e.g. `chinese_(simplified)`, `spanish_(latin_american)`)
/// that aren't safe to guess from a language name alone. Additional
/// languages will be added as consumer needs arise, after the folder-name
/// mapping has been verified against a real SC install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum Language {
    /// English. Folder: `english`.
    English,
}

impl Language {
    /// All supported languages in enum-declaration order.
    pub const ALL: &'static [Language] = &[Language::English];

    /// The directory name Star Citizen uses inside `<install>/data/Localization/`.
    pub fn folder_name(self) -> &'static str {
        match self {
            Self::English => "english",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_folder_name() {
        assert_eq!(Language::English.folder_name(), "english");
    }

    #[test]
    fn folder_names_are_lowercase() {
        for lang in Language::ALL {
            assert_eq!(lang.folder_name(), lang.folder_name().to_lowercase());
        }
    }
}
