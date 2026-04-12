//! Star Citizen release channels.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Star Citizen release channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum Channel {
    Live,
    Hotfix,
    Ptu,
    Eptu,
    TechPreview,
}

impl Channel {
    /// All channels in enum-declaration order (matches priority ordering).
    pub const ALL: &'static [Channel] = &[
        Channel::Live,
        Channel::Hotfix,
        Channel::Ptu,
        Channel::Eptu,
        Channel::TechPreview,
    ];

    /// Lower is higher priority. LIVE is 0, TechPreview is 4.
    pub fn priority(self) -> u8 {
        match self {
            Self::Live => 0,
            Self::Hotfix => 1,
            Self::Ptu => 2,
            Self::Eptu => 3,
            Self::TechPreview => 4,
        }
    }

    /// Display-friendly uppercase name.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Live => "LIVE",
            Self::Hotfix => "HOTFIX",
            Self::Ptu => "PTU",
            Self::Eptu => "EPTU",
            Self::TechPreview => "TECH",
        }
    }

    /// Lowercase channel name for use in launcher version strings.
    /// For example `"4.7.1-live.11592622"` uses `"live"`.
    pub fn lowercase_name(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Hotfix => "hotfix",
            Self::Ptu => "ptu",
            Self::Eptu => "eptu",
            Self::TechPreview => "tech",
        }
    }

    /// Parse a channel name from a launcher log line or a directory name.
    /// Accepts case-insensitive variants like `"LIVE"`, `"Live"`, `"PTU"`,
    /// `"TechPreview"`, `"TECH-PREVIEW"`, `"tech"`.
    pub fn from_str_loose(s: &str) -> Option<Self> {
        let upper = s.to_uppercase();
        match upper.as_str() {
            "LIVE" => Some(Self::Live),
            "HOTFIX" => Some(Self::Hotfix),
            "PTU" => Some(Self::Ptu),
            "EPTU" => Some(Self::Eptu),
            "TECHPREVIEW" | "TECH-PREVIEW" | "TECH" => Some(Self::TechPreview),
            _ => None,
        }
    }

    /// Infer the channel from the final component of an install path.
    /// Used by [`crate::Installation::from_root`] when the caller doesn't
    /// specify a channel explicitly.
    pub(crate) fn from_path_component(component: &str) -> Result<Self, Error> {
        Self::from_str_loose(component).ok_or_else(|| Error::UnknownChannel(component.to_string()))
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_channel_names_case_insensitive() {
        assert_eq!(Channel::from_str_loose("LIVE"), Some(Channel::Live));
        assert_eq!(Channel::from_str_loose("Live"), Some(Channel::Live));
        assert_eq!(Channel::from_str_loose("live"), Some(Channel::Live));
        assert_eq!(Channel::from_str_loose("PTU"), Some(Channel::Ptu));
        assert_eq!(Channel::from_str_loose("EPTU"), Some(Channel::Eptu));
        assert_eq!(Channel::from_str_loose("Hotfix"), Some(Channel::Hotfix));
    }

    #[test]
    fn parse_tech_preview_variants() {
        assert_eq!(
            Channel::from_str_loose("TechPreview"),
            Some(Channel::TechPreview)
        );
        assert_eq!(
            Channel::from_str_loose("TECH-PREVIEW"),
            Some(Channel::TechPreview)
        );
        assert_eq!(Channel::from_str_loose("TECH"), Some(Channel::TechPreview));
    }

    #[test]
    fn parse_unknown_is_none() {
        assert_eq!(Channel::from_str_loose("unknown"), None);
        assert_eq!(Channel::from_str_loose(""), None);
        assert_eq!(Channel::from_str_loose("release"), None);
    }

    #[test]
    fn priority_order_matches_spec() {
        assert!(Channel::Live.priority() < Channel::Hotfix.priority());
        assert!(Channel::Hotfix.priority() < Channel::Ptu.priority());
        assert!(Channel::Ptu.priority() < Channel::Eptu.priority());
        assert!(Channel::Eptu.priority() < Channel::TechPreview.priority());
    }

    #[test]
    fn display_and_lowercase_names() {
        assert_eq!(Channel::Live.display_name(), "LIVE");
        assert_eq!(Channel::Live.lowercase_name(), "live");
        assert_eq!(Channel::Eptu.display_name(), "EPTU");
        assert_eq!(Channel::Eptu.lowercase_name(), "eptu");
    }

    #[test]
    fn display_trait_matches_display_name() {
        assert_eq!(format!("{}", Channel::Live), "LIVE");
        assert_eq!(format!("{}", Channel::TechPreview), "TECH");
    }

    #[test]
    fn all_constant_round_trips_display_name() {
        assert_eq!(Channel::ALL.len(), 5);
        for c in Channel::ALL {
            // Every channel should round-trip through its display name.
            assert_eq!(Channel::from_str_loose(c.display_name()), Some(*c));
        }
    }

    #[test]
    fn from_path_component_errors_on_unknown() {
        let err = Channel::from_path_component("wut").unwrap_err();
        assert!(matches!(err, Error::UnknownChannel(_)));
    }
}
