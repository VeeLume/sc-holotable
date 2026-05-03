use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid, LocaleKey, LocaleMap, LocalizedItemCache};

use crate::classify::{WeaponCategory, classify};
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
    /// Total ammo capacity (physical round count) from
    /// `SAmmoContainerComponentParams.maxAmmoCount`.
    pub total_ammo: Option<i32>,
    /// Per-shot damage. `None` if ammo could not be resolved.
    pub damage: Option<DamageSummary>,
    /// Ammo projectile speed in m/s.
    pub ammo_speed: Option<f32>,
    /// Ammo lifetime in seconds.
    pub ammo_lifetime: Option<f32>,
    /// Ammo penetration distance in metres. Same field as
    /// [`crate::ShipWeapon::penetration_m`].
    pub penetration_m: Option<f32>,
    /// Pellet count (shotguns: 8 or 12). `None` for single-projectile.
    pub pellet_count: Option<i32>,
    /// `Localization.Name` INI key. Raw — `@` preserved. Resolve via
    /// [`Self::display_name`].
    pub name_key: Option<LocaleKey>,
    /// `Localization.Description` INI key. Resolve via
    /// [`Self::description`].
    pub desc_key: Option<LocaleKey>,
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
        localized_items: &LocalizedItemCache,
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
        let total_ammo = damage::extract_total_ammo(ecd, pools);

        let record_name = record_names
            .get(&guid)
            .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n))
            .unwrap_or("")
            .to_string();

        let (name_key, desc_key) = localized_items
            .get(&guid)
            .map(|item| (item.name_key.clone(), item.desc_key.clone()))
            .unwrap_or((None, None));

        Some(FpsWeapon {
            guid,
            record_name,
            size: item_def.size,
            grade: item_def.grade,
            item_type: item_def.r#type.clone(),
            item_sub_type: item_def.sub_type.clone(),
            manufacturer_guid: item_def.manufacturer,
            fire_actions,
            total_ammo,
            damage: ammo.as_ref().map(|a| a.damage),
            ammo_speed: ammo.as_ref().map(|a| a.speed),
            ammo_lifetime: ammo.as_ref().map(|a| a.lifetime),
            penetration_m: ammo.as_ref().and_then(|a| a.penetration_m),
            pellet_count,
            name_key,
            desc_key,
            entity_handle: handle,
        })
    }

    // =========================================================
    // Localization
    // =========================================================

    /// Resolve the localized display name through `locale`. Returns
    /// `None` when the underlying entity has no `Localization.Name`
    /// key or the key is absent from `locale`.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.name_key.as_ref().and_then(|k| locale.resolve(k))
    }

    /// Resolve the localized description through `locale`.
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.desc_key.as_ref().and_then(|k| locale.resolve(k))
    }
}
