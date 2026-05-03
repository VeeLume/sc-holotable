use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid};

/// Per-shot damage summed across direct hit and explosion, all 6 types.
///
/// Always sum both sources — distortion weapons have near-zero direct damage
/// and deliver all meaningful damage through explosion.
#[derive(Debug, Clone, Copy, Default)]
pub struct DamageSummary {
    pub physical: f32,
    pub energy: f32,
    pub distortion: f32,
    pub thermal: f32,
    pub biochemical: f32,
    pub stun: f32,
}

impl DamageSummary {
    /// Scalar total across all damage types.
    pub fn total(&self) -> f32 {
        self.physical + self.energy + self.distortion + self.thermal + self.biochemical + self.stun
    }

    fn add_damage_info(&mut self, d: &DamageInfo) {
        self.physical += d.damage_physical;
        self.energy += d.damage_energy;
        self.distortion += d.damage_distortion;
        self.thermal += d.damage_thermal;
        self.biochemical += d.damage_biochemical;
        self.stun += d.damage_stun;
    }

    fn add(&mut self, other: &DamageSummary) {
        self.physical += other.physical;
        self.energy += other.energy;
        self.distortion += other.distortion;
        self.thermal += other.thermal;
        self.biochemical += other.biochemical;
        self.stun += other.stun;
    }
}

/// Resolved ammo data from the ammo chain.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedAmmo {
    pub damage: DamageSummary,
    pub speed: f32,
    pub lifetime: f32,
    /// Penetration distance in metres
    /// (`projectile_params → penetration_params.basePenetrationDistance`).
    /// `None` for non-bullet projectile families until those gain
    /// damage modeling (`TachyonProjectileParams.penetration_params`
    /// exists in the schema but the damage path doesn't model
    /// tachyons yet — keep penetration `None` to mirror that scope).
    pub penetration_m: Option<f32>,
}

/// Resolve the ammo chain for a weapon entity and extract damage + ballistics.
///
/// Two paths:
/// 1. Local `SAmmoContainerComponentParams` on the weapon entity (dominant, 377 weapons)
/// 2. Two-hop: `weapon_params.ammo_container_record` → intermediate entity → AmmoParams (56 weapons)
pub(crate) fn resolve_ammo(
    ecd: &EntityClassDefinition,
    weapon_params: &SCItemWeaponComponentParams,
    pools: &DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<ResolvedAmmo> {
    let ammo = resolve_ammo_params(ecd, weapon_params, pools, ecd_map, ammo_map)?;
    let damage = extract_damage(ammo, pools);
    let penetration_m = extract_penetration(ammo, pools);
    Some(ResolvedAmmo {
        damage,
        speed: ammo.speed,
        lifetime: ammo.lifetime,
        penetration_m,
    })
}

/// Pull `basePenetrationDistance` out of the ammo's projectile-params
/// chain. Currently only `BulletProjectileParams` is wired up — same
/// projectile family that has its damage modeled.
/// `TachyonProjectileParams.penetration_params` carries the same field
/// in the DCB and can be folded in when tachyon damage lands.
fn extract_penetration(ammo: &AmmoParams, pools: &DataPools) -> Option<f32> {
    let proj_ptr = ammo.projectile_params.as_ref()?;
    if let ProjectileParamsPtr::BulletProjectileParams(h) = proj_ptr
        && let Some(bullet) = h.get(pools)
        && let Some(pen) = bullet.penetration_params.and_then(|h| h.get(pools))
    {
        return Some(pen.base_penetration_distance);
    }
    None
}

fn resolve_ammo_params<'a>(
    ecd: &EntityClassDefinition,
    weapon_params: &SCItemWeaponComponentParams,
    pools: &'a DataPools,
    ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    // Path 2: two-hop via ammo_container_record → intermediate entity.
    // The ref never points directly at AmmoParams — always at an entity.
    if let Some(ref_guid) = weapon_params.ammo_container_record
        && let Some(&container_h) = ecd_map.get(&ref_guid)
        && let Some(container_ecd) = container_h.get(pools)
        && let Some(ammo) = find_ammo_via_container(container_ecd, pools, ammo_map)
    {
        return Some(ammo);
    }

    // Path 1: local SAmmoContainerComponentParams on the weapon entity itself
    find_ammo_via_container(ecd, pools, ammo_map)
}

fn find_ammo_via_container<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
    ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
) -> Option<&'a AmmoParams> {
    let ac = find_component::<SAmmoContainerComponentParams>(ecd, pools)?;
    let ammo_guid = ac.ammo_params_record?;
    let &ammo_h = ammo_map.get(&ammo_guid)?;
    ammo_h.get(pools)
}

/// Pull the `DamageInfo` payload out of a single [`ExplosionParams`]
/// handle and accumulate it into a [`DamageSummary`]. Returns the empty
/// summary when the chain doesn't resolve (no explosion params, no
/// damage pointer, or the variant isn't `DamageInfo`).
///
/// Shared between the bullet detonation path inside [`extract_damage`]
/// and the missile model in `crate::missile`, where ordinance damage
/// hangs off `SCItemMissileParams.explosion_params` directly.
pub(crate) fn extract_explosion_damage(
    explosion: Option<Handle<ExplosionParams>>,
    pools: &DataPools,
) -> DamageSummary {
    let mut summary = DamageSummary::default();
    if let Some(expl) = explosion.and_then(|h| h.get(pools))
        && let Some(DamageBasePtr::DamageInfo(dh)) = &expl.damage
        && let Some(d) = dh.get(pools)
    {
        summary.add_damage_info(d);
    }
    summary
}

fn extract_damage(ammo: &AmmoParams, pools: &DataPools) -> DamageSummary {
    let mut summary = DamageSummary::default();

    let Some(proj_ptr) = &ammo.projectile_params else {
        return summary;
    };

    // TachyonProjectileParams, CounterMeasureProjectileParams, etc. are
    // not handled in v1 — damage stays at zero.
    if let ProjectileParamsPtr::BulletProjectileParams(h) = proj_ptr
        && let Some(bullet) = h.get(pools)
    {
        // Direct damage
        if let Some(DamageBasePtr::DamageInfo(dh)) = &bullet.damage
            && let Some(d) = dh.get(pools)
        {
            summary.add_damage_info(d);
        }
        // Explosion damage — same chain missiles use directly.
        let explosion = bullet
            .detonation_params
            .and_then(|h| h.get(pools))
            .and_then(|det| det.explosion_params);
        summary.add(&extract_explosion_damage(explosion, pools));
    }

    summary
}

/// Find a component on an entity by matching the poly enum variant.
pub(crate) fn find_component<'a, T: 'static>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a T> {
    use std::any::TypeId;

    for comp in &ecd.components {
        // SAFETY for each branch: the TypeId equality guarantees T is the
        // concrete component type, so the pointer cast is sound.
        if TypeId::of::<T>() == TypeId::of::<SCItemWeaponComponentParams>()
            && let DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const SCItemWeaponComponentParams as *const T) });
        }
        if TypeId::of::<T>() == TypeId::of::<SAttachableComponentParams>()
            && let DataForgeComponentParamsPtr::SAttachableComponentParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const SAttachableComponentParams as *const T) });
        }
        if TypeId::of::<T>() == TypeId::of::<SAmmoContainerComponentParams>()
            && let DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const SAmmoContainerComponentParams as *const T) });
        }
        if TypeId::of::<T>() == TypeId::of::<SHealthComponentParams>()
            && let DataForgeComponentParamsPtr::SHealthComponentParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const SHealthComponentParams as *const T) });
        }
        if TypeId::of::<T>() == TypeId::of::<ItemResourceComponentParams>()
            && let DataForgeComponentParamsPtr::ItemResourceComponentParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const ItemResourceComponentParams as *const T) });
        }
        if TypeId::of::<T>() == TypeId::of::<SCItemMissileParams>()
            && let DataForgeComponentParamsPtr::SCItemMissileParams(h) = comp
        {
            let r = h.get(pools)?;
            return Some(unsafe { &*(r as *const SCItemMissileParams as *const T) });
        }
    }
    None
}

/// Extract the weapon's total ammo capacity from
/// `SAmmoContainerComponentParams.maxAmmoCount`. This is the physical
/// round count (ballistic) — distinct from burst_seconds / overheat
/// budget. Returns `None` for energy weapons (no ammo container).
pub(crate) fn extract_total_ammo(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<i32> {
    let ac = find_component::<SAmmoContainerComponentParams>(ecd, pools)?;
    Some(ac.max_ammo_count)
}

/// Extract pellet count from `SProjectileLauncher` on the primary fire action.
pub(crate) fn extract_pellet_count(
    weapon_params: &SCItemWeaponComponentParams,
    pools: &DataPools,
) -> Option<i32> {
    let action = weapon_params.fire_actions.first()?;
    let launcher = match action {
        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
            h.get(pools)?.launch_params.as_ref()
        }
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
            h.get(pools)?.launch_params.as_ref()
        }
        _ => None,
    };
    match launcher? {
        SLauncherBasePtr::SProjectileLauncher(h) => {
            let pl = h.get(pools)?;
            if pl.pellet_count > 1 {
                Some(pl.pellet_count)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_summary_total() {
        let d = DamageSummary {
            physical: 10.0,
            energy: 20.0,
            distortion: 5.0,
            thermal: 1.0,
            biochemical: 0.0,
            stun: 2.0,
        };
        assert!((d.total() - 38.0).abs() < f32::EPSILON);
    }

    #[test]
    fn damage_summary_default_is_zero() {
        let d = DamageSummary::default();
        assert_eq!(d.total(), 0.0);
    }
}
