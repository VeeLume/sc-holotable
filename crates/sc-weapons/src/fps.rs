use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid};

use crate::classify::{classify, WeaponCategory};
use crate::damage::{self, DamageSummary};
use crate::fire_action::{self, FireActionKind};

/// An FPS personal weapon with all data materialized into owned fields.
///
/// Same construction pattern as [`crate::ShipWeapon`] but without ship-specific
/// fields (sustain model, power draw, health). FPS sustain models (Volt
/// heat-ramp, Kastak charged modes) are deferred to v2.
#[derive(Debug, Clone)]
pub struct FpsWeapon {
    /// Record GUID.
    pub guid: Guid,
    /// DCB record name (e.g. `"behr_rifle_ballistic_01"`).
    pub record_name: String,
    /// Weapon size (1 = Small/pistol, 2-4 = Medium, 5 = Large).
    pub size: i32,
    /// Weapon grade.
    pub grade: i32,
    /// Item type enum (`WeaponPersonal`).
    pub item_type: EItemType,
    /// Item subtype enum (`Small`, `Medium`, `Large`).
    pub item_sub_type: EItemSubType,
    /// Manufacturer GUID.
    pub manufacturer_guid: Option<Guid>,
    /// All fire actions in declaration order. `fire_actions[0]` is the
    /// primary mode; later entries are alternate modes (Karna burst→charge,
    /// Kastak burst→burst, beam-rifle wrappers, etc.). 97 of 331 FPS
    /// weapons in 4.7 expose >1 mode.
    pub fire_actions: Vec<FireActionKind>,
    /// Magazine size from `SAmmoContainerComponentParams.maxAmmoCount`.
    pub magazine_size: Option<i32>,
    /// Per-shot damage. `None` if ammo could not be resolved.
    pub damage: Option<DamageSummary>,
    /// Ammo projectile speed in m/s.
    pub ammo_speed: Option<f32>,
    /// Ammo lifetime in seconds.
    pub ammo_lifetime: Option<f32>,
    /// Pellet count (shotguns: 8 or 12). `None` for single-projectile.
    pub pellet_count: Option<i32>,
    /// Raw entity handle.
    pub entity_handle: Handle<EntityClassDefinition>,
}

impl FpsWeapon {
    /// Attempt to construct an `FpsWeapon` from an entity handle.
    ///
    /// Returns `None` if classification doesn't match [`WeaponCategory::Fps`].
    pub fn try_new(
        handle: Handle<EntityClassDefinition>,
        guid: Guid,
        pools: &DataPools,
        ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
        ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
        record_names: &HashMap<Guid, &str>,
    ) -> Option<Self> {
        let ecd = handle.get(pools)?;

        let wp = damage::find_component::<SCItemWeaponComponentParams>(ecd, pools)?;
        let attachable = damage::find_component::<SAttachableComponentParams>(ecd, pools)?;
        let item_def = attachable.attach_def.and_then(|h| h.get(pools))?;

        if classify(&item_def.r#type, &item_def.sub_type) != Some(WeaponCategory::Fps) {
            return None;
        }

        let fire_actions: Vec<FireActionKind> = wp
            .fire_actions
            .iter()
            .map(|a| fire_action::extract_fire_action(a, pools))
            .collect();

        let ammo = damage::resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);
        let pellet_count = damage::extract_pellet_count(wp, pools);
        let magazine_size = damage::extract_magazine_size(ecd, pools);

        let record_name = record_names
            .get(&guid)
            .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n))
            .unwrap_or("")
            .to_string();

        Some(FpsWeapon {
            guid,
            record_name,
            size: item_def.size,
            grade: item_def.grade,
            item_type: item_def.r#type.clone(),
            item_sub_type: item_def.sub_type.clone(),
            manufacturer_guid: item_def.manufacturer,
            fire_actions,
            magazine_size,
            damage: ammo.as_ref().map(|a| a.damage),
            ammo_speed: ammo.as_ref().map(|a| a.speed),
            ammo_lifetime: ammo.as_ref().map(|a| a.lifetime),
            pellet_count,
            entity_handle: handle,
        })
    }
}
