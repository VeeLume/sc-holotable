//! Character-armor base-stat sheet — a T1 data crate.
//!
//! Extracts the per-piece base stats a `Char_Armor_*` entity declares:
//! temperature resistance (min/max), radiation (dissipation rate + capacity),
//! and per-damage-type resistance (the "damage mitigation" surface). Keyed by
//! `EntityClassDefinition` GUID.
//!
//! # Scope
//!
//! A thin data sheet, not an armor model — the base values a downstream
//! surface (sc-crafting product stats) reshapes. Companion to
//! `sc-items-fps-weapons`.
//!
//! # Where the values come from
//!
//! | stat | path |
//! |---|---|
//! | temp resistance | `SCItemClothingParams.TemperatureResistance.{Min,Max}Resistance` |
//! | radiation | `SCItemClothingParams.RadiationResistance.{RadiationDissipationRate,MaximumRadiationCapacity}` |
//! | damage resistance | `SCItemSuitArmorParams.damageResistance →(Ref)→ DamageResistanceMacro.damageResistance → DamageResistance.{Physical,Energy,…}Resistance → DamageResistanceEntry{Multiplier,Threshold,DamageCap}` |
//!
//! Classification goes through the typed [`Items`] envelope; stats are read
//! through the raw `Datacore::db()` layer (the armor stat components aren't
//! under a single clean sc-extract feature, and the `DamageResistanceMacro` is
//! a non-pooled `Reference` target — same escape hatch the recoil config uses).

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{DataCoreDatabase, Datacore, Guid, Instance, LocaleKey, Value};
use sc_items::Items;
use serde::{Deserialize, Serialize};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collection.
pub use sc_extract::RecordCollection;

/// Which armor slot a piece occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorKind {
    Arms,
    Legs,
    Torso,
    Helmet,
    Backpack,
    Undersuit,
}

impl ArmorKind {
    fn from_item_type(t: &EItemType) -> Option<Self> {
        Some(match t {
            EItemType::Char_Armor_Arms => ArmorKind::Arms,
            EItemType::Char_Armor_Legs => ArmorKind::Legs,
            EItemType::Char_Armor_Torso => ArmorKind::Torso,
            EItemType::Char_Armor_Helmet => ArmorKind::Helmet,
            EItemType::Char_Armor_Backpack => ArmorKind::Backpack,
            EItemType::Char_Armor_Undersuit => ArmorKind::Undersuit,
            _ => return None,
        })
    }
}

/// One damage-type resistance: `multiplier` is the damage-taken factor
/// (< 1 mitigates), above `threshold`, clamped by `damage_cap`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct ResistanceEntry {
    pub multiplier: f32,
    pub threshold: f32,
    pub damage_cap: f32,
}

/// Per-damage-type resistance block (the "damage mitigation" surface).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct DamageResistance {
    pub physical: Option<ResistanceEntry>,
    pub energy: Option<ResistanceEntry>,
    pub distortion: Option<ResistanceEntry>,
    pub thermal: Option<ResistanceEntry>,
    pub biochemical: Option<ResistanceEntry>,
    pub stun: Option<ResistanceEntry>,
}

/// The base-stat sheet for one armor piece. All values are *base* (unmodified
/// by crafting); `Option` because not every piece populates every field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorStats {
    pub guid: Guid,
    /// `Localization.Name` key (raw, `@`-prefixed).
    pub name_key: Option<LocaleKey>,
    pub kind: ArmorKind,
    pub size: i32,
    pub grade: i32,
    /// Min/max temperature resistance, °C.
    pub temp_resistance_min: Option<f32>,
    pub temp_resistance_max: Option<f32>,
    /// Radiation dissipation rate + maximum radiation capacity.
    pub radiation_dissipation: Option<f32>,
    pub radiation_capacity: Option<f32>,
    /// Per-type damage resistance, when the piece carries armor params.
    pub damage_resistance: Option<DamageResistance>,
}

/// Index of every armor piece's [`ArmorStats`], keyed by entity GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Armor {
    by_guid: HashMap<Guid, ArmorStats>,
}

impl Armor {
    /// Walk the [`Items`] envelope, keep `Char_Armor_*` entities, and read each
    /// piece's base stats from its component structure (raw `db()` layer).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (&guid, item) in items.iter() {
            let Some(kind) = ArmorKind::from_item_type(&item.item_type) else {
                continue;
            };
            let Some(rec) = db.record(&guid) else {
                continue;
            };
            let inst = rec.as_instance();

            let clothing = find_component(db, &inst, "SCItemClothingParams");
            let (temp_resistance_min, temp_resistance_max) = clothing
                .as_ref()
                .and_then(|c| c.get_instance("TemperatureResistance"))
                .map(|t| (t.get_f32("MinResistance"), t.get_f32("MaxResistance")))
                .unwrap_or((None, None));
            let (radiation_dissipation, radiation_capacity) = clothing
                .as_ref()
                .and_then(|c| c.get_instance("RadiationResistance"))
                .map(|r| {
                    (
                        r.get_f32("RadiationDissipationRate"),
                        r.get_f32("MaximumRadiationCapacity"),
                    )
                })
                .unwrap_or((None, None));

            let damage_resistance = find_component(db, &inst, "SCItemSuitArmorParams")
                .and_then(|suit| resolve_damage_resistance(db, &suit));

            by_guid.insert(
                guid,
                ArmorStats {
                    guid,
                    name_key: item.name_key.clone(),
                    kind,
                    size: item.size,
                    grade: item.grade,
                    temp_resistance_min,
                    temp_resistance_max,
                    radiation_dissipation,
                    radiation_capacity,
                    damage_resistance,
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for Armor {
    type Item = ArmorStats;

    fn get(&self, guid: &Guid) -> Option<&ArmorStats> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ArmorStats)> + '_ {
        self.by_guid.iter()
    }
}

// ── extraction helpers (raw db layer) ───────────────────────────────────────

/// Find the first component instance on an entity whose type name matches.
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

/// `SCItemSuitArmorParams.damageResistance` is a cross-record `Reference` to a
/// `DamageResistanceMacro`; resolve it and pull the per-type entries.
fn resolve_damage_resistance(
    db: &DataCoreDatabase,
    suit: &Instance<'_>,
) -> Option<DamageResistance> {
    let macro_guid = match suit.get("damageResistance")? {
        Value::Reference(Some(r)) => r.guid,
        _ => return None,
    };
    let macro_inst = db.record(&macro_guid)?.as_instance();
    let dr = macro_inst.get_instance("damageResistance")?;
    Some(DamageResistance {
        physical: entry(&dr, "PhysicalResistance"),
        energy: entry(&dr, "EnergyResistance"),
        distortion: entry(&dr, "DistortionResistance"),
        thermal: entry(&dr, "ThermalResistance"),
        biochemical: entry(&dr, "BiochemicalResistance"),
        stun: entry(&dr, "StunResistance"),
    })
}

fn entry(dr: &Instance<'_>, field: &str) -> Option<ResistanceEntry> {
    let e = dr.get_instance(field)?;
    Some(ResistanceEntry {
        multiplier: e.get_f32("Multiplier").unwrap_or_default(),
        threshold: e.get_f32("Threshold").unwrap_or_default(),
        damage_cap: e.get_f32("DamageCap").unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_kind_from_item_type() {
        assert_eq!(
            ArmorKind::from_item_type(&EItemType::Char_Armor_Arms),
            Some(ArmorKind::Arms)
        );
        assert_eq!(ArmorKind::from_item_type(&EItemType::WeaponPersonal), None);
    }

    #[test]
    fn empty_index() {
        let a = Armor::default();
        assert!(a.is_empty());
    }
}
