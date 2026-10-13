//! Ship-weapon base-stat sheet — a T1 data crate.
//!
//! Covers craftable **WeaponGun** (ship cannons / repeaters) and
//! **WeaponMining** (mining lasers): the universal Integrity plus the
//! `GPP_Weapon_Damage` surface — per-shot projectile damage for guns, or
//! per-second beam DPS for mining lasers ("Laser Power"). Keyed by
//! `EntityClassDefinition` GUID. Companion to `sc-items-fps-weapons`.
//!
//! # Where the values come from
//!
//! | stat | path |
//! |---|---|
//! | integrity | `SHealthComponentParams.Health` |
//! | gun damage / shot | `SCItemWeaponComponentParams` → ammo (`ammoContainerRecord` or local `SAmmoContainerComponentParams` → `AmmoParams.projectileParams.damage`) |
//! | mining DPS | `SCItemWeaponComponentParams.fireActions[0]` (`SWeaponActionFireBeamParams`) `.damagePerSecond` |
//! | mining beams | every `fireActions[i]` beam: `damagePerSecond.DamageEnergy`, `fullDamageRange`, `zeroDamageRange`, `hitType` |
//! | mining laser | `SEntityComponentMiningLaserParams`: `throttleMinimum`, `throttleLerpSpeed`, `usesPowerThrottle`, `miningLaserModifiers`, `filterParams` |
//! | module slots | `SItemPortContainerComponentParams.Ports[]` whose `Types[].Type` is `MiningModifier` |
//!
//! Mining lasers additionally carry [`MiningLaserStats`] (`ShipWeaponStats::mining`).
//! The client turns a beam into rock energy as `energy/s × range falloff ×
//! module multiplier × throttle` (LIVE 4.10): full power up to
//! `full_damage_range`, linearly down to 0 at `zero_damage_range`, and no
//! bonus closer in — see [`MiningBeam::falloff`].
//!
//! Classification via the typed [`Items`] envelope; stats read through the raw
//! `Datacore::db()` layer. Verified vs scmdb (C-788 Cannon 325 dmg/shot,
//! Impact Mining Laser 2100 DPS).

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{DataCoreDatabase, Datacore, Guid, Instance, LocaleKey, Value};
use sc_items::Items;
use sc_items_mining::{FloatModifier, MiningModifiers};
use serde::{Deserialize, Serialize};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collection.
pub use sc_extract::RecordCollection;

/// Ship-weapon family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipWeaponKind {
    /// Ballistic / energy gun — ammo per-shot damage.
    Gun,
    /// Mining laser — continuous beam DPS.
    Mining,
}

impl ShipWeaponKind {
    fn from_item_type(t: &EItemType) -> Option<Self> {
        Some(match t {
            EItemType::WeaponGun => ShipWeaponKind::Gun,
            EItemType::WeaponMining => ShipWeaponKind::Mining,
            _ => return None,
        })
    }
}

/// Damage split by type (per-shot for guns, per-second for mining beams).
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

/// Base-stat sheet for one ship weapon. All values are *base* (unmodified by
/// crafting).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipWeaponStats {
    pub guid: Guid,
    pub name_key: Option<LocaleKey>,
    pub kind: ShipWeaponKind,
    pub size: i32,
    pub grade: i32,
    /// `SHealthComponentParams.Health` — Integrity (HP).
    pub integrity_hp: Option<f32>,
    /// The `GPP_Weapon_Damage` surface: per-shot damage (guns) or per-second
    /// beam DPS (mining). `is_beam` says which.
    pub damage: Option<Damage>,
    /// True when `damage` is a per-second beam rate (mining laser) rather than
    /// per-shot.
    pub is_beam: bool,
    /// Primary fire action rounds-per-minute (guns; `None` for beams).
    pub fire_rate: Option<i32>,
    /// Projectile speed m/s (guns).
    pub ammo_speed: Option<f32>,
    /// Mining-laser stats (`None` for guns).
    pub mining: Option<MiningLaserStats>,
}

/// What a mining laser brings to a rock beyond its headline power.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningLaserStats {
    /// One entry per beam fire action, in `fireActions` order. Module power
    /// multipliers address beams by this index
    /// (`sc_items_mining::MiningModule::damage_multiplier`).
    pub beams: Vec<MiningBeam>,
    /// `throttleMinimum` — the lowest throttle the laser can hold (0..1).
    pub throttle_minimum: f32,
    /// `throttleLerpSpeed` — how fast the throttle follows the request.
    pub throttle_lerp_speed: f32,
    /// `usesPowerThrottle` — when set, the throttle drives power draw instead
    /// of scaling the energy delivered to the rock (false on every 4.10 laser).
    pub uses_power_throttle: bool,
    /// The laser's own mining modifiers (resistance, instability, window, …).
    pub modifiers: MiningModifiers,
    /// `filterParams.filterModifier` ("inert material level").
    pub inert_filter: Option<FloatModifier>,
    /// Number of item ports that accept a mining module.
    pub module_slots: u32,
}

/// One beam fire action of a mining laser.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningBeam {
    /// Index into the laser's `fireActions`.
    pub fire_action: i32,
    /// `hitType` — which hit path the beam takes on the rock.
    pub hit_type: String,
    /// `damagePerSecond.DamageEnergy` — energy per second at full power
    /// ("Laser Power" for the fracturing beam).
    pub energy_per_second: f32,
    /// `fullDamageRange` (m) — full power up to here ("Optimal Range").
    pub full_damage_range: f32,
    /// `zeroDamageRange` (m) — power reaches 0 here ("Maximum Range").
    pub zero_damage_range: f32,
}

/// The `hitType` of the extraction beam. The client routes a beam hit to the
/// extraction path by its hit type (resolved to a hit-type registry id); the
/// DCB has no typed role field, so this registry key is the discriminator.
/// `build` warns when a multi-beam laser has none, i.e. when the key drifts.
const EXTRACTION_HIT_TYPE: &str = "Extraction";

impl MiningLaserStats {
    /// The fracturing beam ("Laser Power"): the first beam that is not the
    /// extraction beam (fire action 0 on every 4.10 laser, `ElectricArc`).
    pub fn fracture_beam(&self) -> Option<&MiningBeam> {
        self.beams
            .iter()
            .find(|b| b.hit_type != EXTRACTION_HIT_TYPE)
    }

    /// The extraction beam (fire action 1 on every 4.10 laser).
    pub fn extraction_beam(&self) -> Option<&MiningBeam> {
        self.beams
            .iter()
            .find(|b| b.hit_type == EXTRACTION_HIT_TYPE)
    }
}

impl MiningBeam {
    /// Power fraction at `distance_m` from the beam's start to the hit point,
    /// as the client computes it: 1 up to `full_damage_range`, then linear
    /// down to 0 at `zero_damage_range`, clamped to 0..1.
    pub fn falloff(&self, distance_m: f32) -> f32 {
        if self.full_damage_range >= self.zero_damage_range {
            return 1.0;
        }
        let f = 1.0
            - (distance_m - self.full_damage_range)
                / (self.zero_damage_range - self.full_damage_range);
        f.clamp(0.0, 1.0)
    }
}

/// Index of every ship weapon's [`ShipWeaponStats`], keyed by GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShipWeapons {
    by_guid: HashMap<Guid, ShipWeaponStats>,
}

impl ShipWeapons {
    /// Walk the [`Items`] envelope, keep ship guns + mining lasers, and read
    /// each one's base stats from its component structure (raw `db()` layer).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (&guid, item) in items.iter() {
            let Some(kind) = ShipWeaponKind::from_item_type(&item.item_type) else {
                continue;
            };
            let Some(rec) = db.record(&guid) else {
                continue;
            };
            let inst = rec.as_instance();

            let integrity_hp = find_component(db, &inst, "SHealthComponentParams")
                .and_then(|h| h.get_f32("Health"));

            let mut damage = None;
            let mut is_beam = false;
            let mut fire_rate = None;
            let mut ammo_speed = None;
            if let Some(wc) = find_component(db, &inst, "SCItemWeaponComponentParams") {
                let primary = wc.get_array("fireActions").and_then(|mut it| it.next());
                let primary = primary.as_ref().and_then(|v| value_to_instance(db, v));
                if primary.as_ref().and_then(|p| p.type_name())
                    == Some("SWeaponActionFireBeamParams")
                {
                    // Mining laser / beam: per-second damage.
                    is_beam = true;
                    damage = primary
                        .as_ref()
                        .and_then(|p| p.get_instance("damagePerSecond"))
                        .map(|d| damage_from_info(&d));
                } else {
                    // Gun: per-shot damage via the ammo chain.
                    fire_rate = primary.as_ref().and_then(|p| p.get_i32("fireRate"));
                    if let Some(ammo) = resolve_ammo(db, &wc, &inst) {
                        damage = ammo
                            .get_instance("projectileParams")
                            .and_then(|proj| proj.get_instance("damage"))
                            .map(|d| damage_from_info(&d));
                        ammo_speed = ammo.get_f32("speed");
                    }
                }
            }

            let mining = (kind == ShipWeaponKind::Mining)
                .then(|| mining_laser_stats(db, &inst))
                .flatten();

            by_guid.insert(
                guid,
                ShipWeaponStats {
                    guid,
                    name_key: item.name_key.clone(),
                    kind,
                    size: item.size,
                    grade: item.grade,
                    integrity_hp,
                    damage,
                    is_beam,
                    fire_rate,
                    ammo_speed,
                    mining,
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for ShipWeapons {
    type Item = ShipWeaponStats;

    fn get(&self, guid: &Guid) -> Option<&ShipWeaponStats> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ShipWeaponStats)> + '_ {
        self.by_guid.iter()
    }
}

// ── extraction helpers (raw db layer) ───────────────────────────────────────

fn find_component<'a>(
    db: &'a DataCoreDatabase,
    entity: &Instance<'a>,
    type_name: &str,
) -> Option<Instance<'a>> {
    for p in entity.properties() {
        if let Value::Array(_) = p.value
            && let Some(arr) = entity.get_array(p.name)
        {
            for elem in arr {
                if let Some(ci) = value_to_instance(db, &elem)
                    && ci.type_name() == Some(type_name)
                {
                    return Some(ci);
                }
            }
        }
    }
    None
}

fn value_to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
    match v {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

/// Mining-laser stats from the laser component, its beam fire actions and its
/// item ports. `None` when the entity has no `SEntityComponentMiningLaserParams`.
fn mining_laser_stats(db: &DataCoreDatabase, entity: &Instance<'_>) -> Option<MiningLaserStats> {
    let laser = find_component(db, entity, "SEntityComponentMiningLaserParams")?;
    let beams = find_component(db, entity, "SCItemWeaponComponentParams")
        .and_then(|wc| wc.get_array("fireActions"))
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, v): (usize, Value<'_>)| {
            let fa = value_to_instance(db, &v)?;
            if fa.type_name() != Some("SWeaponActionFireBeamParams") {
                return None;
            }
            Some(MiningBeam {
                fire_action: i32::try_from(i).ok()?,
                hit_type: fa.get_str("hitType").unwrap_or_default().to_string(),
                energy_per_second: fa
                    .get_instance("damagePerSecond")
                    .and_then(|d| d.get_f32("DamageEnergy"))
                    .unwrap_or_default(),
                full_damage_range: fa.get_f32("fullDamageRange").unwrap_or_default(),
                zero_damage_range: fa.get_f32("zeroDamageRange").unwrap_or_default(),
            })
        })
        .collect::<Vec<_>>();
    let module_slots = find_component(db, entity, "SItemPortContainerComponentParams")
        .and_then(|pc| pc.get_array("Ports"))
        .into_iter()
        .flatten()
        .filter_map(|p| value_to_instance(db, &p))
        .filter(|port| {
            port.get_array("Types").into_iter().flatten().any(|t| {
                value_to_instance(db, &t)
                    .and_then(|ti| ti.get("Type"))
                    .and_then(|v| v.as_str())
                    == Some("MiningModifier")
            })
        })
        .count();
    if beams.len() > 1 && !beams.iter().any(|b| b.hit_type == EXTRACTION_HIT_TYPE) {
        tracing::warn!(
            hit_types = ?beams.iter().map(|b| b.hit_type.as_str()).collect::<Vec<_>>(),
            "mining laser has several beams but none with hitType {EXTRACTION_HIT_TYPE:?}; \
             the extraction key may have drifted"
        );
    }
    Some(MiningLaserStats {
        beams,
        throttle_minimum: laser.get_f32("throttleMinimum").unwrap_or_default(),
        throttle_lerp_speed: laser.get_f32("throttleLerpSpeed").unwrap_or_default(),
        uses_power_throttle: laser.get_bool("usesPowerThrottle").unwrap_or_default(),
        modifiers: laser
            .get_instance("miningLaserModifiers")
            .map(|m| MiningModifiers::read(&m))
            .unwrap_or_default(),
        inert_filter: laser
            .get_instance("filterParams")
            .as_ref()
            .and_then(MiningModifiers::read_filter),
        module_slots: u32::try_from(module_slots).unwrap_or(u32::MAX),
    })
}

fn damage_from_info(info: &Instance<'_>) -> Damage {
    Damage {
        physical: info.get_f32("DamagePhysical").unwrap_or_default(),
        energy: info.get_f32("DamageEnergy").unwrap_or_default(),
        distortion: info.get_f32("DamageDistortion").unwrap_or_default(),
        thermal: info.get_f32("DamageThermal").unwrap_or_default(),
        biochemical: info.get_f32("DamageBiochemical").unwrap_or_default(),
        stun: info.get_f32("DamageStun").unwrap_or_default(),
    }
}

/// Resolve the weapon's `AmmoParams` instance: two-hop via
/// `ammoContainerRecord → container entity → ammoParamsRecord`, else a local
/// `SAmmoContainerComponentParams` on the weapon entity.
fn resolve_ammo<'a>(
    db: &'a DataCoreDatabase,
    wc: &Instance<'a>,
    entity: &Instance<'a>,
) -> Option<Instance<'a>> {
    if let Some(Value::Reference(Some(r))) = wc.get("ammoContainerRecord")
        && let Some(container) = db.record(&r.guid)
        && let Some(ammo) = ammo_via_container(db, &container.as_instance())
    {
        return Some(ammo);
    }
    ammo_via_container(db, entity)
}

fn ammo_via_container<'a>(db: &'a DataCoreDatabase, entity: &Instance<'a>) -> Option<Instance<'a>> {
    let ac = find_component(db, entity, "SAmmoContainerComponentParams")?;
    let ammo_guid = match ac.get("ammoParamsRecord")? {
        Value::Reference(Some(r)) => r.guid,
        _ => return None,
    };
    Some(db.record(&ammo_guid)?.as_instance())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_from_item_type() {
        assert_eq!(
            ShipWeaponKind::from_item_type(&EItemType::WeaponGun),
            Some(ShipWeaponKind::Gun)
        );
        assert_eq!(
            ShipWeaponKind::from_item_type(&EItemType::WeaponMining),
            Some(ShipWeaponKind::Mining)
        );
        assert_eq!(
            ShipWeaponKind::from_item_type(&EItemType::WeaponPersonal),
            None
        );
    }

    #[test]
    fn beam_falloff() {
        let beam = MiningBeam {
            fire_action: 0,
            hit_type: String::new(),
            energy_per_second: 2340.0,
            full_damage_range: 60.0,
            zero_damage_range: 180.0,
        };
        assert_eq!(beam.falloff(10.0), 1.0); // no bonus closer in
        assert_eq!(beam.falloff(60.0), 1.0);
        assert!((beam.falloff(120.0) - 0.5).abs() < 1e-6);
        assert_eq!(beam.falloff(200.0), 0.0);
    }

    #[test]
    fn damage_total() {
        let d = Damage {
            energy: 2100.0,
            ..Default::default()
        };
        assert_eq!(d.total(), 2100.0);
    }
}
