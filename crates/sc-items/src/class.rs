//! An item's class (Military, Civilian, …), read from its description.
//!
//! No DCB field holds it (LIVE 4.10): the game writes it only into the
//! localized description, as a `Class: Military` line in the block of
//! `Key: Value` lines that heads it. So this is the one place that reads
//! display text for a value, scoped to that line. Read it from the
//! `global.ini` the game ships (`AssetData::locale`), not a patched one.

use serde::{Deserialize, Serialize};
use tracing::warn;

/// What the `Class:` line of an item's description says.
///
/// Ship components (coolers, power plants, quantum drives, radars,
/// shields and a few others) carry one of the five named classes. On
/// personal weapons and tools the same line names the weapon class
/// (`Ballistic`, `Energy (Laser)`, `Melee`): those are [`Other`](Self::Other).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemClass {
    Military,
    Civilian,
    Industrial,
    Stealth,
    Competition,
    /// Any other value, with its spaces normalized.
    Other(String),
}

impl ItemClass {
    /// The `Class:` line of `description` as `global.ini` holds it: lines
    /// are split on the literal `\n` the file uses, and non-breaking spaces
    /// (some lines use them, `Class:\u{a0}Competition`) count as spaces.
    /// `None` without such a line.
    pub fn from_description(description: &str) -> Option<Self> {
        description.split("\\n").find_map(|line| {
            let line = line.replace('\u{a0}', " ");
            let value = line.trim().strip_prefix("Class:")?;
            let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
            if value.is_empty() {
                warn!(line = %line.trim(), "item description has an empty Class line");
                return None;
            }
            Some(match value.as_str() {
                "Military" => Self::Military,
                "Civilian" => Self::Civilian,
                "Industrial" => Self::Industrial,
                "Stealth" => Self::Stealth,
                "Competition" => Self::Competition,
                _ => Self::Other(value),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_class() {
        let desc = r"Item Type: Cooler\nManufacturer: Aegis Dynamics \nSize: 1\nGrade: C\nClass: Military\n\nText.";
        assert_eq!(ItemClass::from_description(desc), Some(ItemClass::Military));
        assert_eq!(
            ItemClass::from_description("Class:\u{a0}Competition\\nGrade: A"),
            Some(ItemClass::Competition)
        );
        assert_eq!(
            ItemClass::from_description(r"Item Type: Cooler\nGrade: B"),
            None
        );
        // Prose that merely starts with "Class" is no class line.
        assert_eq!(
            ItemClass::from_description("Classic cuts never go out of style."),
            None
        );
    }

    #[test]
    fn weapon_class() {
        assert_eq!(
            ItemClass::from_description("Class: Energy\u{a0}(Laser)"),
            Some(ItemClass::Other("Energy (Laser)".into()))
        );
    }
}
