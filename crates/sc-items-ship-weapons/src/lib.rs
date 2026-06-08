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
//!
//! Classification via the typed [`Items`] envelope; stats read through the raw
//! `Datacore::db()` layer. Verified vs scmdb (C-788 Cannon 325 dmg/shot,
//! Impact Mining Laser 2100 DPS).

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{DataCoreDatabase, Datacore, Guid, Instance, LocaleKey, Value};
use sc_items::Items;
use serde::{Deserialize, Serialize};

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
            let Some(rec) = db.record(&guid) else { continue };
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
                },
            );
        }
        Self { by_guid }
    }

    pub fn get(&self, guid: &Guid) -> Option<&ShipWeaponStats> {
        self.by_guid.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &ShipWeaponStats)> + '_ {
        self.by_guid.iter()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
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
        assert_eq!(ShipWeaponKind::from_item_type(&EItemType::WeaponPersonal), None);
    }

    #[test]
    fn damage_total() {
        let d = Damage { energy: 2100.0, ..Default::default() };
        assert_eq!(d.total(), 2100.0);
    }
}
