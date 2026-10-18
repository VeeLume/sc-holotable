//! A loadout's defences: hull, armour and the combined shield.

use sc_extract::RecordCollection;
use sc_items_ship_components::{
    MinMax, PerDamageType, ShieldEmitter, ShieldGenerator, ShipComponents, VehicleArmor,
};
use serde::{Deserialize, Serialize};

use crate::{Mounted, Vehicle, Vehicles, network_items};

/// What protects a loadout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Defense {
    /// [`Vehicle::hull_health`]: every hull part's `damageMax`.
    pub hull_health: f32,
    /// The hull armour item's sheet and its own HP.
    pub armor: Option<VehicleArmor>,
    pub armor_health: Option<f32>,
    /// The shield generators combined; `None` without one.
    pub shield: Option<Shield>,
    /// The shield controller's emitter (face layout).
    pub emitter: Option<ShieldEmitter>,
}

/// Several shield generators as one, as the client combines them
/// (`FUN_143e2e690`): HP and regen add up, the regen delays are averaged,
/// resistance and absorption are weighted by each generator's HP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shield {
    pub generators: usize,
    pub max_health: f32,
    /// HP/s at full power; the regen a calculator shows follows the
    /// generators' power ([`Readout::shield_made`](crate::Readout::shield_made)).
    pub max_regen: f32,
    pub damaged_regen_delay: f32,
    pub downed_regen_delay: f32,
    pub resistance: Option<PerDamageType<MinMax>>,
    pub absorption: Option<PerDamageType<MinMax>>,
}

impl Shield {
    /// The generators combined; `None` for none.
    pub fn combine(generators: &[&ShieldGenerator]) -> Option<Self> {
        let n = generators.len();
        if n == 0 {
            return None;
        }
        let hp: f32 = generators.iter().map(|g| g.max_health).sum();
        let mean = |f: fn(&ShieldGenerator) -> f32| {
            generators.iter().map(|g| f(g)).sum::<f32>() / n as f32
        };
        let weighted = |f: fn(&ShieldGenerator) -> Option<PerDamageType<MinMax>>| {
            let all: Vec<(f32, PerDamageType<MinMax>)> = generators
                .iter()
                .map(|g| f(g).map(|v| (g.max_health, v)))
                .collect::<Option<_>>()?;
            if hp <= 0.0 {
                return all.first().map(|(_, v)| *v);
            }
            let by = |pick: fn(&PerDamageType<MinMax>) -> MinMax| {
                let (min, max) = all.iter().fold((0.0, 0.0), |(a, b), (w, v)| {
                    let m = pick(v);
                    (a + w * m.min, b + w * m.max)
                });
                MinMax {
                    min: min / hp,
                    max: max / hp,
                }
            };
            Some(PerDamageType {
                physical: by(|v| v.physical),
                energy: by(|v| v.energy),
                distortion: by(|v| v.distortion),
                thermal: by(|v| v.thermal),
                biochemical: by(|v| v.biochemical),
                stun: by(|v| v.stun),
            })
        };
        Some(Self {
            generators: n,
            max_health: hp,
            max_regen: generators.iter().map(|g| g.max_regen).sum(),
            damaged_regen_delay: mean(|g| g.damaged_regen_delay),
            downed_regen_delay: mean(|g| g.downed_regen_delay),
            resistance: weighted(|g| g.resistance),
            absorption: weighted(|g| g.absorption),
        })
    }
}

impl Defense {
    /// The defences of `loadout` on `vehicle`: its own network's items
    /// (a docked vehicle's shield does not count).
    pub fn compute(
        vehicle: &Vehicle,
        loadout: &[Mounted],
        vehicles: &Vehicles,
        components: &ShipComponents,
    ) -> Self {
        let mut generators = Vec::new();
        let (mut armor, mut armor_health, mut emitter) = (None, None, None);
        for (_, item) in network_items(loadout, vehicles) {
            let Some(sheet) = components.get(&item) else {
                continue;
            };
            if let Some(s) = &sheet.shield {
                generators.push(s);
            }
            if armor.is_none() && sheet.armor.is_some() {
                armor = sheet.armor.clone();
                armor_health = sheet.health.as_ref().map(|h| h.health);
            }
            if emitter.is_none() {
                emitter = sheet.emitter.clone();
            }
        }
        Self {
            hull_health: vehicle.hull_health(),
            armor,
            armor_health,
            shield: Shield::combine(&generators),
            emitter,
        }
    }
}
