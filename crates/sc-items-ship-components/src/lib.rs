//! Ship-component base-stat sheet — a T1 data crate.
//!
//! Covers the craftable ship components that share the common **Integrity**
//! stat (`SHealthComponentParams.Health`) plus their per-domain stat: quantum
//! drives (drive speed + fuel), shields (max HP + regen), coolers / power
//! plants (resource generation via the item-resource network). Keyed by
//! `EntityClassDefinition` GUID. Companion to `sc-items-fps-weapons` /
//! `sc-items-armor`.
//!
//! # Where the values come from
//!
//! | stat | path |
//! |---|---|
//! | integrity | `SHealthComponentParams.Health` |
//! | drive speed | `SCItemQuantumDriveParams.params.driveSpeed` (m/s ÷ 1e6 → Mm/s) |
//! | quantum fuel | `SCItemQuantumDriveParams.quantumFuelRequirement` |
//! | shield HP / regen | `SCItemShieldGeneratorParams.{MaxShieldHealth,MaxShieldRegen}` |
//! | coolant / power | `ItemResourceComponentParams.states[].deltas[].generation` (resource = Coolant/Power) → `resourceAmountPerSecond` |
//!
//! Classification goes through the typed [`Items`] envelope; stats are read
//! via the raw `Datacore::db()` layer (`SCItemQuantumDriveParams` etc. aren't
//! under one clean sc-extract feature). Verified against scmdb samples (Zephyr
//! QD, Steward shield, Kelvid cooler, Charger power plant).

use std::collections::HashMap;

use sc_extract::generated::EItemType;
use sc_extract::{DataCoreDatabase, Datacore, Guid, Instance, LocaleKey, Value};
use sc_items::Items;
use serde::{Deserialize, Serialize};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collection.
pub use sc_extract::RecordCollection;

/// Which ship-component family a piece belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipComponentKind {
    Cooler,
    PowerPlant,
    QuantumDrive,
    Shield,
    Radar,
    DockingCollar,
}

impl ShipComponentKind {
    fn from_item_type(t: &EItemType) -> Option<Self> {
        Some(match t {
            EItemType::Cooler => ShipComponentKind::Cooler,
            EItemType::PowerPlant => ShipComponentKind::PowerPlant,
            EItemType::QuantumDrive => ShipComponentKind::QuantumDrive,
            EItemType::Shield => ShipComponentKind::Shield,
            EItemType::Radar => ShipComponentKind::Radar,
            EItemType::DockingCollar => ShipComponentKind::DockingCollar,
            _ => return None,
        })
    }
}

/// Base-stat sheet for one ship component. All values are *base* (unmodified
/// by crafting); every domain-specific field is `Option` (only the relevant
/// ones populate per kind).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipComponentStats {
    pub guid: Guid,
    pub name_key: Option<LocaleKey>,
    pub kind: ShipComponentKind,
    pub size: i32,
    pub grade: i32,
    /// `SHealthComponentParams.Health` — the universal "Integrity" stat (HP).
    pub integrity_hp: Option<f32>,
    /// Quantum drive cruise speed, **Mm/s** (`params.driveSpeed` ÷ 1e6).
    pub quantum_drive_speed: Option<f32>,
    /// Quantum fuel required per unit distance (`quantumFuelRequirement`).
    pub quantum_fuel_requirement: Option<f32>,
    /// Shield maximum HP (`MaxShieldHealth`).
    pub shield_max_health: Option<f32>,
    /// Shield regen per second (`MaxShieldRegen`).
    pub shield_regen: Option<f32>,
    /// Coolant generated per second (item-resource network).
    pub coolant_rate: Option<f32>,
    /// Power generated (pips; item-resource network).
    pub power_output: Option<f32>,
    /// Radar aim-assist min/max assignment distance (m).
    pub radar_aim_assist_min: Option<f32>,
    pub radar_aim_assist_max: Option<f32>,
}

/// Index of every ship component's [`ShipComponentStats`], keyed by GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShipComponents {
    by_guid: HashMap<Guid, ShipComponentStats>,
}

impl ShipComponents {
    /// Walk the [`Items`] envelope, keep ship-component entities, and read each
    /// one's base stats from its component structure (raw `db()` layer).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (&guid, item) in items.iter() {
            let Some(kind) = ShipComponentKind::from_item_type(&item.item_type) else {
                continue;
            };
            let Some(rec) = db.record(&guid) else {
                continue;
            };
            let inst = rec.as_instance();

            let integrity_hp = find_component(db, &inst, "SHealthComponentParams")
                .and_then(|h| h.get_f32("Health"));

            let qd = find_component(db, &inst, "SCItemQuantumDriveParams");
            let quantum_drive_speed = qd
                .as_ref()
                .and_then(|q| q.get_instance("params"))
                .and_then(|p| p.get_f32("driveSpeed"))
                .map(|m_per_s| m_per_s / 1_000_000.0);
            let quantum_fuel_requirement = qd
                .as_ref()
                .and_then(|q| q.get_f32("quantumFuelRequirement"));

            let shield = find_component(db, &inst, "SCItemShieldGeneratorParams");
            let shield_max_health = shield.as_ref().and_then(|s| s.get_f32("MaxShieldHealth"));
            let shield_regen = shield.as_ref().and_then(|s| s.get_f32("MaxShieldRegen"));

            let (coolant_rate, power_output) =
                find_component(db, &inst, "ItemResourceComponentParams")
                    .map(|irc| resource_generation(db, &irc))
                    .unwrap_or((None, None));

            let radar = find_component(db, &inst, "SCItemRadarComponentParams")
                .and_then(|r| r.get_instance("aimAssist"));
            let radar_aim_assist_min = radar
                .as_ref()
                .and_then(|a| a.get_f32("distanceMinAssignment"));
            let radar_aim_assist_max = radar
                .as_ref()
                .and_then(|a| a.get_f32("distanceMaxAssignment"));

            by_guid.insert(
                guid,
                ShipComponentStats {
                    guid,
                    name_key: item.name_key.clone(),
                    kind,
                    size: item.size,
                    grade: item.grade,
                    integrity_hp,
                    quantum_drive_speed,
                    quantum_fuel_requirement,
                    shield_max_health,
                    shield_regen,
                    coolant_rate,
                    power_output,
                    radar_aim_assist_min,
                    radar_aim_assist_max,
                },
            );
        }
        Self { by_guid }
    }
}

impl sc_extract::RecordCollection for ShipComponents {
    type Item = ShipComponentStats;

    fn get(&self, guid: &Guid) -> Option<&ShipComponentStats> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ShipComponentStats)> + '_ {
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

/// Walk the item-resource network's generation deltas and pull the coolant /
/// power generation rate. The `generation.resource` enum (`"Coolant"` /
/// `"Power"`) is the only structural tag distinguishing them — a typed-enum
/// match, the data's own discriminator (no alternative). The per-second amount
/// is `standardResourceUnits` (coolant) or `units` (power pips).
fn resource_generation(db: &DataCoreDatabase, irc: &Instance<'_>) -> (Option<f32>, Option<f32>) {
    let mut coolant = None;
    let mut power = None;
    let Some(states) = irc.get_array("states") else {
        return (None, None);
    };
    for state_v in states {
        let Some(state) = value_to_instance(db, &state_v) else {
            continue;
        };
        let Some(deltas) = state.get_array("deltas") else {
            continue;
        };
        for delta_v in deltas {
            let Some(delta) = value_to_instance(db, &delta_v) else {
                continue;
            };
            let Some(generation) = delta.get_instance("generation") else {
                continue;
            };
            let resource = generation.get_str("resource").unwrap_or_default();
            // Coolant uses a float `standardResourceUnits`; power uses an
            // integer `units` (pip count) — try both representations.
            let amount = generation
                .get_instance("resourceAmountPerSecond")
                .and_then(|u| {
                    u.get_f32("standardResourceUnits")
                        .or_else(|| u.get_f32("units"))
                        .or_else(|| u.get_i32("units").map(|v| v as f32))
                        .or_else(|| u.get_i32("standardResourceUnits").map(|v| v as f32))
                });
            match resource {
                "Coolant" => coolant = coolant.or(amount),
                "Power" => power = power.or(amount),
                _ => {}
            }
        }
    }
    (coolant, power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_from_item_type() {
        assert_eq!(
            ShipComponentKind::from_item_type(&EItemType::QuantumDrive),
            Some(ShipComponentKind::QuantumDrive)
        );
        assert_eq!(
            ShipComponentKind::from_item_type(&EItemType::WeaponPersonal),
            None
        );
    }

    #[test]
    fn empty_index() {
        assert!(ShipComponents::default().is_empty());
    }
}
