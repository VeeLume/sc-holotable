//! Ship missile + torpedo model.
//!
//! Parallels [`crate::ShipWeapon`] / [`crate::FpsWeapon`] for the
//! `EItemType::Missile` family. Damage hangs off the
//! `SCItemMissileParams.explosion_params` chain (no projectile / ammo
//! container), tracking lives on `targeting_params`, and speed +
//! arming on the GCS / missile-params blocks. Ship missiles
//! (`EItemSubType::Missile`) and torpedoes (`EItemSubType::Torpedo`)
//! share the same data shape and are both surfaced through this
//! type — `item_sub_type` distinguishes them.
//!
//! Tracking signal reuses the generated [`ESignatureType`] enum
//! directly. Unguided ordnance (no `targeting_params`) yields
//! `tracking == None`; the data-quality observation in the feature
//! request — entity names sometimes disagreeing with the resolved
//! `trackingSignalType` — applies, the DCB value is authoritative.

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid, LocaleKey, LocaleMap, LocalizedItemCache};

use crate::classify::{WeaponCategory, classify};
use crate::damage::{self, DamageSummary};

/// Tracking profile for guided ordnance — extracted from
/// `SCItemMissileParams.targeting_params`. Field names are angle/range
/// units stripped from the DCB so consumers don't have to remember
/// per-field unit conventions.
#[derive(Debug, Clone, PartialEq)]
pub struct TrackingProfile {
    /// Signal type the missile homes on — `Infrared` /
    /// `Electromagnetic` / `CrossSection` are the values seen on real
    /// ship missiles in 4.7. Forward-compat falls through to
    /// `ESignatureType::Unrecognized(_)` for any new values a future
    /// patch adds.
    pub signal: ESignatureType,
    /// Seconds of tracking required before lock acquires.
    pub lock_time: f32,
    /// Cone half-angle in degrees inside which the missile can lock.
    pub lock_angle_deg: f32,
    /// Minimum lock range in metres (below this the missile won't lock).
    pub lock_range_min_m: f32,
    /// Maximum lock range in metres (effective max engagement).
    pub lock_range_max_m: f32,
}

/// A ship-mounted missile or torpedo with all data materialized into
/// owned fields. Constructed via [`Missile::try_new`].
#[derive(Debug, Clone)]
pub struct Missile {
    /// Record GUID — stable across game patches.
    pub guid: Guid,
    /// DCB record name with the `EntityClassDefinition.` prefix
    /// stripped (e.g. `"GMISL_S05_IR_TALN_Valkyrie"`).
    pub record_name: String,
    /// Missile size class (1-12), from `SItemDefinition.Size`.
    pub size: i32,
    /// Item subtype (`Missile` or `Torpedo`).
    pub item_sub_type: EItemSubType,
    /// Manufacturer GUID — look up via [`sc_extract::ManufacturerRegistry`].
    pub manufacturer_guid: Option<Guid>,
    /// Per-shot explosion damage across all 6 types. `None` when the
    /// `SCItemMissileParams.explosion_params → DamageInfo` chain
    /// doesn't resolve.
    pub damage: Option<DamageSummary>,
    /// Cruise speed in m/s, from `GCSParams.linearSpeed`. `None` for
    /// missiles without a guidance block (e.g. unguided rockets).
    pub speed: Option<f32>,
    /// Seconds before the warhead arms after launch
    /// (`SCItemMissileParams.armTime`). Always present once we've
    /// matched the missile component.
    pub arm_time: f32,
    /// Guided-missile tracking profile. `None` for unguided ordnance.
    pub tracking: Option<TrackingProfile>,
    /// `Localization.Name` INI key. Raw — `@` preserved. Resolve via
    /// [`Self::display_name`].
    pub name_key: Option<LocaleKey>,
    /// `Localization.Description` INI key. Resolve via
    /// [`Self::description`].
    pub desc_key: Option<LocaleKey>,
    /// Raw entity handle — escape hatch for consumers that want to
    /// reach through to the unmodelled ordnance fields (cluster
    /// params, emissions, audio) via `&DataPools`.
    pub entity_handle: Handle<EntityClassDefinition>,
}

impl Missile {
    /// Attempt to construct a `Missile` from an entity handle.
    ///
    /// Returns `None` if:
    /// - The entity has no `SAttachableComponentParams` / `AttachDef`
    /// - Classification doesn't match [`WeaponCategory::Missile`]
    /// - The entity doesn't carry `SCItemMissileParams`
    pub fn try_new(
        handle: Handle<EntityClassDefinition>,
        guid: Guid,
        pools: &DataPools,
        ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
        record_names: &HashMap<Guid, &str>,
        localized_items: &LocalizedItemCache,
    ) -> Option<Self> {
        let _ = ecd_map; // reserved — mirrors ShipWeapon::try_new arg shape

        let ecd = handle.get(pools)?;

        let attachable = damage::find_component::<SAttachableComponentParams>(ecd, pools)?;
        let item_def = attachable.attach_def.and_then(|h| h.get(pools))?;

        if classify(&item_def.r#type, &item_def.sub_type) != Some(WeaponCategory::Missile) {
            return None;
        }

        let missile_params = damage::find_component::<SCItemMissileParams>(ecd, pools)?;

        let damage_summary =
            damage::extract_explosion_damage(missile_params.explosion_params, pools);
        let damage = if damage_summary.total() > 0.0 {
            Some(damage_summary)
        } else {
            None
        };

        let speed = missile_params
            .gcsparams
            .and_then(|h| h.get(pools))
            .map(|gcs| gcs.linear_speed);

        let tracking = missile_params
            .targeting_params
            .and_then(|h| h.get(pools))
            .map(|t| TrackingProfile {
                signal: t.tracking_signal_type.clone(),
                lock_time: t.lock_time,
                lock_angle_deg: t.locking_angle,
                lock_range_min_m: t.lock_range_min,
                lock_range_max_m: t.lock_range_max,
            });

        let record_name = record_names
            .get(&guid)
            .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n))
            .unwrap_or("")
            .to_string();

        let (name_key, desc_key) = localized_items
            .get(&guid)
            .map(|item| (item.name_key.clone(), item.desc_key.clone()))
            .unwrap_or((None, None));

        Some(Missile {
            guid,
            record_name,
            size: item_def.size,
            item_sub_type: item_def.sub_type.clone(),
            manufacturer_guid: item_def.manufacturer,
            damage,
            speed,
            arm_time: missile_params.arm_time,
            tracking,
            name_key,
            desc_key,
            entity_handle: handle,
        })
    }

    /// Resolve the localized display name through `locale`.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.name_key.as_ref().and_then(|k| locale.resolve(k))
    }

    /// Resolve the localized description through `locale`.
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.desc_key.as_ref().and_then(|k| locale.resolve(k))
    }
}
