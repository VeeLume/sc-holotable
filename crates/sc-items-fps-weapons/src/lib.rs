//! FPS-weapon base-stat sheet — a T1 data crate.
//!
//! Extracts the per-weapon base stats that an FPS personal weapon entity
//! declares in its typed component structure: fire rate, per-type damage,
//! projectile speed, spread, recoil (pitch/yaw/smooth), and magazine size.
//! Keyed by `EntityClassDefinition` GUID.
//!
//! # Scope
//!
//! This is deliberately a *thin data sheet*, not a weapon model. It carries
//! the base values a downstream surface (e.g. sc-crafting's product-stats)
//! reshapes — nothing about fire-mode behaviour, sustained DPS, attachments,
//! or localization beyond the name key. The legacy `sc-weapons` crate keeps
//! the richer ship/FPS/missile model; this crate is the focused,
//! crafting-facing base-stat source.
//!
//! # Where the values come from (all stable typed paths)
//!
//! | stat | path |
//! |---|---|
//! | fire rate | `SCItemWeaponComponentParams.fireActions[0].fireRate` |
//! | spread | `fireActions[0].launchParams(SProjectileLauncher).spreadParams` |
//! | recoil | `fireActions[0].recoil → WeaponProceduralRecoilConfigDef.weaponProceduralAimRecoil` |
//! | damage / speed | `ammoContainerRecord → … → AmmoParams.projectileParams` |
//! | magazine | `SAmmoContainerComponentParams.maxAmmoCount` |
//!
//! The recoil config is reached through a `Reference` whose target is not
//! pooled (it is not a seeded record type), so that one hop is read through
//! the raw `Datacore::db()` layer — the documented escape hatch. Everything
//! else goes through the typed pool surface.

use std::collections::HashMap;

use sc_extract::generated::{
    AmmoParams, DamageBasePtr, DataForgeComponentParamsPtr, EItemSubType, EItemType,
    EntityClassDefinition, Handle, ProjectileParamsPtr, SAmmoContainerComponentParams,
    SCItemWeaponComponentParams, SLauncherBasePtr, SWeaponActionParamsPtr,
};
use sc_extract::{DataCoreDatabase, DataPools, Datacore, Guid, LocaleKey};
use sc_items::Items;
use serde::{Deserialize, Serialize};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collection.
pub use sc_extract::RecordCollection;

// Re-export so a single-crate consumer can name the cache type without a
// direct sc-items dep (type identity is preserved — same sc-extract rev).
pub use sc_items::{Item, Items as ItemEnvelope};

/// Per-shot base damage, split by type. Crafting reshapes "Impact Force",
/// which maps to the physical component for ballistic FPS weapons (energy
/// weapons populate `energy`). [`Damage::total`] sums all six.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    pub physical: f32,
    pub energy: f32,
    pub distortion: f32,
    pub thermal: f32,
    pub biochemical: f32,
    pub stun: f32,
}

impl Damage {
    pub fn total(&self) -> f32 {
        self.physical + self.energy + self.distortion + self.thermal + self.biochemical + self.stun
    }
}

/// The base-stat sheet for one FPS weapon entity. Every field is `Option`
/// because not all stats resolve for every weapon family (energy weapons have
/// no ballistic magazine, beam/charged primaries have no integer fire rate,
/// etc.). All values are *base* — unmodified by crafting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FpsWeaponStats {
    pub guid: Guid,
    /// `Localization.Name` key (raw, `@`-prefixed). Resolve via a `LocaleMap`.
    pub name_key: Option<LocaleKey>,
    /// `AttachDef.Size` size class (1 = Small, …).
    pub size: i32,
    /// `AttachDef.Grade`.
    pub grade: i32,
    /// Primary fire action's rounds-per-minute. `None` for charged/beam/
    /// sequence primaries with no scalar fire rate.
    pub fire_rate: Option<i32>,
    /// Per-shot direct projectile damage.
    pub damage: Option<Damage>,
    /// Projectile speed (m/s) from `AmmoParams.speed`.
    pub ammo_speed: Option<f32>,
    /// Projectile lifetime (s) from `AmmoParams.lifetime`.
    pub ammo_lifetime: Option<f32>,
    /// Spread cone `min`/`max` (degrees) from the primary launcher.
    pub spread_min: Option<f32>,
    pub spread_max: Option<f32>,
    /// Aim-recoil pitch (vertical) max, degrees.
    pub recoil_pitch: Option<f32>,
    /// Aim-recoil yaw (horizontal) max, degrees.
    pub recoil_yaw: Option<f32>,
    /// Aim-recoil smoothing time (s) — `recoil_time`.
    pub recoil_smooth: Option<f32>,
    /// Magazine capacity (rounds) from the ammo container.
    pub mag_size: Option<i32>,
}

/// Index of every FPS weapon's [`FpsWeaponStats`], keyed by entity GUID.
/// Build once via [`FpsWeapons::build`], share by reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FpsWeapons {
    by_guid: HashMap<Guid, FpsWeaponStats>,
}

impl FpsWeapons {
    /// Walk every `EntityClassDefinition`, keep the FPS personal weapons
    /// (typed `EItemType::WeaponPersonal`, non-`Gadget`), and materialize the
    /// base-stat sheet for each. Classification uses the shared [`Items`]
    /// envelope; stats come from the typed weapon component (+ one raw hop
    /// for the recoil config).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let db = datacore.db();
        let ecd_map = &store.records.multi_feature.entity_class_definition;
        let ammo_map = &store.records.multi_feature.ammo_params;

        let mut by_guid = HashMap::new();
        for (&guid, &handle) in ecd_map {
            let Some(item) = items.get(&guid) else {
                continue;
            };
            if !matches!(item.item_type, EItemType::WeaponPersonal) {
                continue;
            }
            if matches!(item.item_sub_type, EItemSubType::Gadget) {
                continue;
            }
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let Some(wc) = weapon_params(ecd, pools) else {
                continue;
            };

            let (fire_rate, spread_min, spread_max, recoil_guid) =
                primary_action(wc.fire_actions.first(), pools);
            let (recoil_pitch, recoil_yaw, recoil_smooth) = match recoil_guid {
                Some(g) => resolve_recoil(db, &g),
                None => (None, None, None),
            };
            let ammo = resolve_ammo(ecd, wc, pools, ecd_map, ammo_map);
            let damage = ammo.map(|a| damage_of(a, pools));
            let ammo_speed = ammo.map(|a| a.speed);
            let ammo_lifetime = ammo.map(|a| a.lifetime);
            let mag_size = mag_size(ecd, wc, pools, ecd_map);

            by_guid.insert(
                guid,
                FpsWeaponStats {
                    guid,
                    name_key: item.name_key.clone(),
                    size: item.size,
                    grade: item.grade,
                    fire_rate,
                    damage,
                    ammo_speed,
                    ammo_lifetime,
                    spread_min,
                    spread_max,
                    recoil_pitch,
                    recoil_yaw,
                    recoil_smooth,
                    mag_size,
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for FpsWeapons {
    type Item = FpsWeaponStats;

    fn get(&self, guid: &Guid) -> Option<&FpsWeaponStats> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &FpsWeaponStats)> + '_ {
        self.by_guid.iter()
    }
}

// ── extraction helpers ──────────────────────────────────────────────────────

/// First `SCItemWeaponComponentParams` on the entity's components.
fn weapon_params<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SCItemWeaponComponentParams> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
        _ => None,
    })
}

/// First `SAmmoContainerComponentParams` on the entity's components.
fn ammo_container<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAmmoContainerComponentParams> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
        _ => None,
    })
}

/// Pull `(fire_rate, spread_min, spread_max, recoil_config_guid)` off the
/// primary fire action. Single / Rapid / Burst share these field names; other
/// primaries (charged/beam/sequence) yield `None`s.
fn primary_action(
    action: Option<&SWeaponActionParamsPtr>,
    pools: &DataPools,
) -> (Option<i32>, Option<f32>, Option<f32>, Option<Guid>) {
    use SWeaponActionParamsPtr as P;
    let parts: Option<(i32, Option<&SLauncherBasePtr>, Option<Guid>)> = match action {
        Some(P::SWeaponActionFireSingleParams(h)) => h
            .get(pools)
            .map(|s| (s.fire_rate, s.launch_params.as_ref(), s.recoil)),
        Some(P::SWeaponActionFireRapidParams(h)) => h
            .get(pools)
            .map(|s| (s.fire_rate, s.launch_params.as_ref(), s.recoil)),
        Some(P::SWeaponActionFireBurstParams(h)) => h
            .get(pools)
            .map(|s| (s.fire_rate, s.launch_params.as_ref(), s.recoil)),
        _ => None,
    };
    let Some((fire_rate, launch, recoil)) = parts else {
        return (None, None, None, None);
    };
    let (spread_min, spread_max) = spread(launch, pools);
    (Some(fire_rate), spread_min, spread_max, recoil)
}

/// Spread cone min/max from a projectile launcher's `spreadParams`.
fn spread(launch: Option<&SLauncherBasePtr>, pools: &DataPools) -> (Option<f32>, Option<f32>) {
    let Some(SLauncherBasePtr::SProjectileLauncher(h)) = launch else {
        return (None, None);
    };
    let Some(launcher) = h.get(pools) else {
        return (None, None);
    };
    match launcher.spread_params.and_then(|sp| sp.get(pools)) {
        Some(sp) => (Some(sp.min), Some(sp.max)),
        None => (None, None),
    }
}

/// Resolve the recoil config (raw — its `Reference` target is not pooled) and
/// pull the aim-recoil pitch/yaw maxima + smoothing time. The authoritative
/// values live in `weaponProceduralAimRecoil.curveAimRecoil`
/// (`{pitch,yaw}MaxDegrees`, `recoilSmoothTime`) — verified against the P6-LR
/// sniper (1.55° / 0.44° / 0.090s). The sibling `max` Vec2 is a separate,
/// often-zero field and is *not* what the stat panel shows.
fn resolve_recoil(db: &DataCoreDatabase, guid: &Guid) -> (Option<f32>, Option<f32>, Option<f32>) {
    let Some(cfg) = db.record(guid) else {
        return (None, None, None);
    };
    let Some(aim) = cfg.get_instance("weaponProceduralAimRecoil") else {
        return (None, None, None);
    };
    let Some(curve) = aim.get_instance("curveAimRecoil") else {
        return (None, None, None);
    };
    (
        curve.get_f32("pitchMaxDegrees"),
        curve.get_f32("yawMaxDegrees"),
        curve.get_f32("recoilSmoothTime"),
    )
}

/// Resolve the weapon's ammo: two-hop via `ammoContainerRecord → container
/// entity → AmmoParams`, else a local container on the weapon entity.
fn resolve_ammo<'a>(
    ecd: &EntityClassDefinition,
    wc: &SCItemWeaponComponentParams,
    pools: &'a DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    if let Some(rg) = wc.ammo_container_record
        && let Some(&ch) = ecd_map.get(&rg)
        && let Some(container) = ch.get(pools)
        && let Some(ammo) = ammo_via_container(container, pools, ammo_map)
    {
        return Some(ammo);
    }
    ammo_via_container(ecd, pools, ammo_map)
}

fn ammo_via_container<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    let ac = ammo_container(ecd, pools)?;
    let g = ac.ammo_params_record?;
    ammo_map.get(&g)?.get(pools)
}

/// Direct projectile damage (bullet families). Non-bullet projectiles
/// (tachyon/countermeasure) leave damage at zero — same scope as the legacy
/// model's v1 damage path.
fn damage_of(ammo: &AmmoParams, pools: &DataPools) -> Damage {
    let mut d = Damage::default();
    if let Some(ProjectileParamsPtr::BulletProjectileParams(h)) = &ammo.projectile_params
        && let Some(bullet) = h.get(pools)
        && let Some(DamageBasePtr::DamageInfo(dh)) = &bullet.damage
        && let Some(di) = dh.get(pools)
    {
        d.physical = di.damage_physical;
        d.energy = di.damage_energy;
        d.distortion = di.damage_distortion;
        d.thermal = di.damage_thermal;
        d.biochemical = di.damage_biochemical;
        d.stun = di.damage_stun;
    }
    d
}

/// Magazine capacity from the ammo container (`maxAmmoCount`), local or via
/// `ammoContainerRecord`.
fn mag_size(
    ecd: &EntityClassDefinition,
    wc: &SCItemWeaponComponentParams,
    pools: &DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
) -> Option<i32> {
    if let Some(ac) = ammo_container(ecd, pools) {
        return Some(ac.max_ammo_count);
    }
    if let Some(rg) = wc.ammo_container_record
        && let Some(&ch) = ecd_map.get(&rg)
        && let Some(container) = ch.get(pools)
        && let Some(ac) = ammo_container(container, pools)
    {
        return Some(ac.max_ammo_count);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_total_sums_all_types() {
        let d = Damage {
            physical: 100.0,
            energy: 5.0,
            ..Default::default()
        };
        assert!((d.total() - 105.0).abs() < f32::EPSILON);
    }

    #[test]
    fn empty_index() {
        let w = FpsWeapons::default();
        assert!(w.is_empty());
        assert_eq!(w.len(), 0);
    }
}
