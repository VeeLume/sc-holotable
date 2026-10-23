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
use sc_extract::{DataPools, Guid, LocaleKey, LocaleMap};
use sc_items::{Items, RecordCollection};

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

/// One submunition a cluster missile releases: a missile item in one of
/// its own ports.
#[derive(Debug, Clone)]
pub struct Submunition {
    /// The port it sits in (`cluster1` … on the Rattler II).
    pub port: String,
    /// Its `EntityClassDefinition` record.
    pub guid: Guid,
    /// Its explosion damage (`SCItemMissileParams.explosionParams`).
    pub damage: DamageSummary,
}

/// What a cluster missile (`SCItemMissileParams.clusterParams`) releases.
///
/// `clusterParams` itself holds only release timing. The submunitions are
/// the missile items in the missile's own default loadout. On LIVE 4.10
/// the Rattler II (S2) carries it, with 7 Venom rockets of 100 physical +
/// 200 energy. The only other record with it, the Viper (S3), is a
/// `GroundVehicleMissile` and so not a [`Missile`].
#[derive(Debug, Clone)]
pub struct Cluster {
    /// `launchDetachTime`, s: release after launch.
    pub launch_detach_time: f32,
    /// `impactDetachTime`, s.
    pub impact_detach_time: f32,
    /// `detachDelay`, s between two releases.
    pub detach_delay: f32,
    /// `detachSpeed`, m/s.
    pub detach_speed: f32,
    /// The submunitions, in loadout order.
    pub submunitions: Vec<Submunition>,
}

impl Cluster {
    /// Every submunition's damage summed, all of them hitting.
    pub fn damage(&self) -> DamageSummary {
        let mut sum = DamageSummary::default();
        for s in &self.submunitions {
            sum.add(&s.damage);
        }
        sum
    }
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
    /// Manufacturer GUID — look up via `sc_manufacturers::Manufacturers`.
    pub manufacturer_guid: Option<Guid>,
    /// Per-shot explosion damage across all 6 types. `None` when the
    /// `SCItemMissileParams.explosion_params → DamageInfo` chain
    /// doesn't resolve.
    pub damage: Option<DamageSummary>,
    /// The submunitions of a cluster missile; `None` for any other.
    /// [`Self::total_damage`] adds them to [`Self::damage`].
    pub cluster: Option<Cluster>,
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
    /// reach through to the unmodelled ordnance fields (emissions,
    /// audio) via `&DataPools`.
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
        localized_items: &Items,
    ) -> Option<Self> {
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

        let cluster = missile_params
            .cluster_params
            .and_then(|h| h.get(pools))
            .map(|c| Cluster {
                launch_detach_time: c.launch_detach_time,
                impact_detach_time: c.impact_detach_time,
                detach_delay: c.detach_delay,
                detach_speed: c.detach_speed,
                submunitions: submunitions(ecd, pools, ecd_map, record_names),
            });

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
            cluster,
            speed,
            arm_time: missile_params.arm_time,
            tracking,
            name_key,
            desc_key,
            entity_handle: handle,
        })
    }

    /// The body's explosion plus every cluster submunition's, all of them
    /// hitting: the Rattler II 1,370 + 7 × 300 = 3,470. `None` when
    /// neither carries damage.
    pub fn total_damage(&self) -> Option<DamageSummary> {
        let mut sum = self.damage.unwrap_or_default();
        if let Some(c) = &self.cluster {
            sum.add(&c.damage());
        }
        (sum.total() > 0.0).then_some(sum)
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

/// The missile items in `ecd`'s own default loadout (a manual
/// `SEntityComponentDefaultLoadoutParams`), each with its explosion damage.
fn submunitions(
    ecd: &EntityClassDefinition,
    pools: &DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    record_names: &HashMap<Guid, &str>,
) -> Vec<Submunition> {
    let entries = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SEntityComponentDefaultLoadoutParams(h) => {
            match h.get(pools)?.loadout.as_ref()? {
                SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutManualParams(m) => {
                    Some(&m.get(pools)?.entries)
                }
                _ => None,
            }
        }
        _ => None,
    });
    let Some(entries) = entries else {
        return Vec::new();
    };
    entries
        .iter()
        .filter_map(|e| e.get(pools))
        .filter_map(|e| {
            let guid = e
                .entity_class_reference
                .or_else(|| class_by_name(&e.entity_class_name, ecd_map, record_names))?;
            let sub = ecd_map.get(&guid)?.get(pools)?;
            let params = damage::find_component::<SCItemMissileParams>(sub, pools)?;
            Some(Submunition {
                port: e.item_port_name.clone(),
                guid,
                damage: damage::extract_explosion_damage(params.explosion_params, pools),
            })
        })
        .collect()
}

/// An `EntityClassDefinition` record by name, matched case-insensitively
/// as the game's class registry does. The cluster missiles' loadout
/// entries name their submunitions by `entityClassName` with an empty
/// `entityClassReference`, so there is no typed link to follow.
fn class_by_name(
    name: &str,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    record_names: &HashMap<Guid, &str>,
) -> Option<Guid> {
    if name.is_empty() {
        return None;
    }
    let hit = ecd_map.keys().copied().find(|guid| {
        record_names.get(guid).is_some_and(|n| {
            n.strip_prefix("EntityClassDefinition.")
                .unwrap_or(n)
                .eq_ignore_ascii_case(name)
        })
    });
    if hit.is_none() {
        tracing::warn!(class = %name, "cluster loadout names a class with no EntityClassDefinition record");
    }
    hit
}
