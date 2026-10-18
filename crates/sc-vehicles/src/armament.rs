//! What a loadout fights with: guns, missiles and countermeasures.

use sc_extract::generated::EItemType;
use sc_extract::{Guid, RecordCollection};
use sc_items::Items;
use serde::{Deserialize, Serialize};

use crate::{Mounted, Vehicles, network_items};

/// A loadout's weapons by kind, each with its port path (docked vehicles
/// left out). Join a gun or a missile to `sc-weapons` for its stats, a
/// countermeasure launcher to `sc-items-ship-components` for its
/// ammunition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Armament {
    /// `WeaponGun` items (on gimbals, turrets and fixed mounts alike).
    pub guns: Vec<(String, Guid)>,
    /// `Missile` items, one per missile on its rack.
    pub missiles: Vec<(String, Guid)>,
    /// `WeaponDefensive` items: flare and chaff launchers.
    pub countermeasures: Vec<(String, Guid)>,
}

impl Armament {
    pub fn compute(loadout: &[Mounted], vehicles: &Vehicles, items: &Items) -> Self {
        let mut out = Self::default();
        for (path, item) in network_items(loadout, vehicles) {
            let Some(i) = items.get(&item) else {
                continue;
            };
            let list = match i.item_type {
                EItemType::WeaponGun => &mut out.guns,
                EItemType::Missile => &mut out.missiles,
                EItemType::WeaponDefensive => &mut out.countermeasures,
                _ => continue,
            };
            list.push((path, item));
        }
        out
    }
}
