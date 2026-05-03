//! Per-localization-key pools across every weapon family.
//!
//! Multiple [`crate::ShipWeapon`] / [`crate::FpsWeapon`] / [`crate::Missile`]
//! entities frequently share a single `global.ini` localization key
//! (CIG ships variants under one display name). [`WeaponPools`]
//! groups every materialized weapon by its `name_key` /
//! `desc_key` so consumers can dedupe or render variant lists per
//! INI key without re-walking the iterators.
//!
//! Mirrors the shape of [`sc_contracts::MissionPools`] and follows
//! the workspace localization rule (`docs/localization.md`):
//! collisions live next to the iterator that produces them, keys
//! stay raw with the leading `@` preserved, and resolution against
//! a [`sc_extract::LocaleMap`] is the call site's job.

use std::collections::HashMap;

use sc_extract::{Guid, LocaleKey};

use crate::fps::FpsWeapon;
use crate::missile::Missile;
use crate::ship::ShipWeapon;

/// Precomputed `LocaleKey → Vec<Guid>` groupings across every weapon
/// family. Each entry is one localization key plus every entity GUID
/// that shares it; the GUID lookup goes back through the original
/// iterator's output.
///
/// More axes are non-breaking additions: a future grouping (by
/// manufacturer, by size, by tag) lands as a sibling field without
/// disturbing the existing ones.
#[derive(Debug, Clone, Default)]
pub struct WeaponPools {
    /// Weapons grouped by `name_key`. Entries with no name key are
    /// not included.
    pub name_key: HashMap<LocaleKey, Vec<Guid>>,
    /// Weapons grouped by `desc_key`. Entries with no description key
    /// are not included.
    pub desc_key: HashMap<LocaleKey, Vec<Guid>>,
}

impl WeaponPools {
    /// Build the pools from materialized weapon lists. O(n) per axis
    /// across the union of inputs. Pass empty slices for families a
    /// consumer doesn't need.
    pub fn build(
        ship_weapons: &[ShipWeapon],
        fps_weapons: &[FpsWeapon],
        missiles: &[Missile],
    ) -> Self {
        let mut pools = Self::default();
        for w in ship_weapons {
            pools.insert(w.guid, w.name_key.as_ref(), w.desc_key.as_ref());
        }
        for w in fps_weapons {
            pools.insert(w.guid, w.name_key.as_ref(), w.desc_key.as_ref());
        }
        for m in missiles {
            pools.insert(m.guid, m.name_key.as_ref(), m.desc_key.as_ref());
        }
        pools
    }

    fn insert(&mut self, guid: Guid, name: Option<&LocaleKey>, desc: Option<&LocaleKey>) {
        if let Some(k) = name {
            self.name_key.entry(k.clone()).or_default().push(guid);
        }
        if let Some(k) = desc {
            self.desc_key.entry(k.clone()).or_default().push(guid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_inputs_produce_empty_pools() {
        let pools = WeaponPools::build(&[], &[], &[]);
        assert!(pools.name_key.is_empty());
        assert!(pools.desc_key.is_empty());
    }
}
