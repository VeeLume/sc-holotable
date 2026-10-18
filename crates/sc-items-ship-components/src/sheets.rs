//! Per-kind stat sheets, read through the typed generated surface.

use sc_extract::DataPools;
use sc_extract::generated::{
    DamageBasePtr, DamageInfo, DamageResistanceBasePtr, DamageResistanceEntry,
    DataForgeComponentParamsPtr, EFaceType, EThrusterType, EntityClassDefinition, Handle,
    SCItemQuantumDriveParams, SCItemShieldGeneratorParams, SCItemThrusterParams,
    SCItemVehicleArmorParams, SHealthComponentParams, SQuantumDriveParams,
};
use sc_resources::CargoQuantity;
use serde::{Deserialize, Serialize};

/// One value per damage type, in the data's order.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct PerDamageType<T> {
    pub physical: T,
    pub energy: T,
    pub distortion: T,
    pub thermal: T,
    pub biochemical: T,
    pub stun: T,
}

impl<T: Copy> PerDamageType<T> {
    /// From a six-element list in data order (`ShieldResistance[]`,
    /// `ShieldAbsorption[]`); `None` when it has another length.
    fn from_slice(v: &[T]) -> Option<Self> {
        let &[physical, energy, distortion, thermal, biochemical, stun] = v else {
            return None;
        };
        Some(Self {
            physical,
            energy,
            distortion,
            thermal,
            biochemical,
            stun,
        })
    }
}

/// A `{Min, Max}` pair.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

/// `SHealthComponentParams`: an item's own hit points and how it takes
/// damage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// `Health`.
    pub health: f32,
    /// `DamageResistances.*Resistance.Multiplier`: damage taken per point
    /// dealt (1 = unresisted).
    pub damage_multiplier: Option<PerDamageType<f32>>,
}

/// `SCItemShieldGeneratorParams`.
///
/// Several generators on a ship add up their HP and regen; delays are
/// averaged and resistance / absorption weighted by HP (client
/// `FUN_143e2e690`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShieldGenerator {
    /// `MaxShieldHealth`.
    pub max_health: f32,
    /// `MaxShieldRegen`, HP/s at full power.
    pub max_regen: f32,
    /// `DecayRatio`: the share of HP per second lost above the target.
    pub decay_ratio: f32,
    /// `DamagedRegenDelay`, s: regen pauses after a face is hit.
    pub damaged_regen_delay: f32,
    /// `DownedRegenDelay`, s: regen pauses after a face reaches 0.
    pub downed_regen_delay: f32,
    /// `ReservePool{Initial,Max}HealthRatio`, `ReservePool{Regen,Drain}RateRatio`.
    pub reserve_initial_ratio: f32,
    pub reserve_max_ratio: f32,
    pub reserve_regen_ratio: f32,
    pub reserve_drain_ratio: f32,
    /// `ShieldResistance[]`: the share of a hit the shield cancels, between
    /// `min` and `max` with power.
    pub resistance: Option<PerDamageType<MinMax>>,
    /// `ShieldAbsorption[]`: the share of what is left that the face takes
    /// rather than passing to the hull, between `min` (empty face) and `max`
    /// (full face).
    pub absorption: Option<PerDamageType<MinMax>>,
}

/// `SCItemShieldEmitterParams`, on the ship's shield controller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShieldEmitter {
    /// `FaceType`: how the shield HP is split into faces.
    #[serde(with = "face_type_serde")]
    pub face_type: EFaceType,
    /// `MaxReallocation`: the share of HP the crew can move between faces.
    pub max_reallocation: f32,
    /// `ReconfigurationCooldown`, s.
    pub reconfiguration_cooldown: f32,
    /// `usePowerForAbsorptionCalculations`.
    pub use_power_for_absorption: bool,
}

/// `SCItemVehicleArmorParams`: the hull armour item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleArmor {
    /// `damageMultiplier`: damage the hull takes per point that gets
    /// through.
    pub damage_multiplier: Option<PerDamageType<f32>>,
    /// `armorDeflection.deflectionValue`: a hit at or below this (scaled by
    /// the armour's health) does no damage.
    pub deflection: Option<PerDamageType<f32>>,
    /// `armorPenetrationResistance.basePenetrationReduction`.
    pub penetration_reduction: f32,
    /// `armorPenetrationResistance.penetrationAbsorptionForType`.
    pub penetration_absorption: Option<PerDamageType<f32>>,
    /// `signalInfrared` / `signalElectromagnetic` / `signalCrossSection`:
    /// multipliers on the ship's signatures.
    pub signal_infrared: f32,
    pub signal_electromagnetic: f32,
    pub signal_cross_section: f32,
}

/// `SCItemQuantumDriveParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantumDrive {
    /// `params`: normal quantum travel.
    pub travel: QuantumTravel,
    /// `splineJumpParams`: travel along a spline (jump points); `None` when
    /// absent.
    pub spline: Option<QuantumTravel>,
    /// `quantumFuelRequirement`, per Mm travelled.
    pub fuel_requirement: f32,
    /// `jumpRange`, m.
    pub jump_range: f32,
    /// `disconnectRange`, m.
    pub disconnect_range: f32,
}

/// `SQuantumDriveParams`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuantumTravel {
    /// `driveSpeed`, m/s.
    pub drive_speed: f32,
    /// `cooldownTime`, s.
    pub cooldown_time: f32,
    /// `spoolUpTime`, s.
    pub spool_up_time: f32,
    /// `stageOneAccelRate` / `stageTwoAccelRate`, m/s².
    pub stage_one_accel_rate: f32,
    pub stage_two_accel_rate: f32,
    /// `engageSpeed`, m/s.
    pub engage_speed: f32,
    /// `calibrationRate`, `min/maxCalibrationRequirement`,
    /// `calibrationDelayInSeconds`.
    pub calibration_rate: f32,
    pub min_calibration_requirement: f32,
    pub max_calibration_requirement: f32,
    pub calibration_delay: f32,
}

/// `SCItemThrusterParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thruster {
    /// `thrustCapacity`, N.
    pub thrust_capacity: f32,
    /// `thrusterType`.
    #[serde(with = "thruster_type_serde")]
    pub thruster_type: EThrusterType,
    /// `fuelBurnRatePer10KNewton`: hydrogen per second per 10 kN of thrust.
    pub fuel_burn_rate_per_10kn: f32,
    /// `minHealthThrustMultiplier`: the thrust left at 0 health.
    pub min_health_thrust_multiplier: f32,
    /// `onlyActiveInVTOL`.
    pub only_active_in_vtol: bool,
}

/// The typed sheets of one entity, each `None` when it lacks the component.
#[derive(Debug, Default)]
pub(crate) struct Sheets {
    pub health: Option<ComponentHealth>,
    pub shield: Option<ShieldGenerator>,
    pub emitter: Option<ShieldEmitter>,
    pub armor: Option<VehicleArmor>,
    pub quantum: Option<QuantumDrive>,
    pub thruster: Option<Thruster>,
    pub capacity: Option<CargoQuantity>,
}

pub(crate) fn sheets(ecd: &EntityClassDefinition, pools: &DataPools) -> Sheets {
    use DataForgeComponentParamsPtr as C;
    let mut out = Sheets::default();
    for c in &ecd.components {
        match c {
            C::SHealthComponentParams(h) => out.health = h.get(pools).map(|h| health(h, pools)),
            C::SCItemShieldGeneratorParams(h) => {
                out.shield = h.get(pools).map(|s| shield(s, pools));
            }
            C::SCItemShieldEmitterParams(h) => {
                out.emitter = h.get(pools).map(|e| ShieldEmitter {
                    face_type: e.face_type.clone(),
                    max_reallocation: e.max_reallocation,
                    reconfiguration_cooldown: e.reconfiguration_cooldown,
                    use_power_for_absorption: e.use_power_for_absorption_calculations,
                });
            }
            C::SCItemVehicleArmorParams(h) => out.armor = h.get(pools).map(|a| armor(a, pools)),
            C::SCItemQuantumDriveParams(h) => {
                out.quantum = h.get(pools).and_then(|q| quantum(q, pools));
            }
            C::SCItemThrusterParams(h) => out.thruster = h.get(pools).map(thruster),
            C::ResourceContainer(h) => {
                out.capacity = h
                    .get(pools)
                    .and_then(|r| r.capacity.as_ref())
                    .map(|c| CargoQuantity::from_ptr(c, pools));
            }
            _ => {}
        }
    }
    out
}

fn health(h: &SHealthComponentParams, pools: &DataPools) -> ComponentHealth {
    let damage_multiplier = match h.damage_resistances.as_ref() {
        Some(DamageResistanceBasePtr::DamageResistance(r)) => r.get(pools).map(|r| {
            let m = |e: Option<Handle<DamageResistanceEntry>>| {
                e.and_then(|e| e.get(pools)).map_or(1.0, |e| e.multiplier)
            };
            PerDamageType {
                physical: m(r.physical_resistance),
                energy: m(r.energy_resistance),
                distortion: m(r.distortion_resistance),
                thermal: m(r.thermal_resistance),
                biochemical: m(r.biochemical_resistance),
                stun: m(r.stun_resistance),
            }
        }),
        _ => None,
    };
    ComponentHealth {
        health: h.health,
        damage_multiplier,
    }
}

fn shield(s: &SCItemShieldGeneratorParams, pools: &DataPools) -> ShieldGenerator {
    let pairs = |v: Vec<MinMax>| PerDamageType::from_slice(&v);
    ShieldGenerator {
        max_health: s.max_shield_health,
        max_regen: s.max_shield_regen,
        decay_ratio: s.decay_ratio,
        damaged_regen_delay: s.damaged_regen_delay,
        downed_regen_delay: s.downed_regen_delay,
        reserve_initial_ratio: s.reserve_pool_initial_health_ratio,
        reserve_max_ratio: s.reserve_pool_max_health_ratio,
        reserve_regen_ratio: s.reserve_pool_regen_rate_ratio,
        reserve_drain_ratio: s.reserve_pool_drain_rate_ratio,
        resistance: pairs(
            s.shield_resistance
                .iter()
                .filter_map(|r| r.get(pools))
                .map(|r| MinMax {
                    min: r.min,
                    max: r.max,
                })
                .collect(),
        ),
        absorption: pairs(
            s.shield_absorption
                .iter()
                .filter_map(|r| r.get(pools))
                .map(|r| MinMax {
                    min: r.min,
                    max: r.max,
                })
                .collect(),
        ),
    }
}

fn armor(a: &SCItemVehicleArmorParams, pools: &DataPools) -> VehicleArmor {
    let info = |h: Option<Handle<DamageInfo>>| h.and_then(|h| h.get(pools)).map(per_type);
    let damage_multiplier = match a.damage_multiplier.as_ref() {
        Some(DamageBasePtr::DamageInfo(h)) => info(Some(*h)),
        _ => None,
    };
    let penetration = a.armor_penetration_resistance.and_then(|h| h.get(pools));
    VehicleArmor {
        damage_multiplier,
        deflection: a
            .armor_deflection
            .and_then(|h| h.get(pools))
            .and_then(|d| info(d.deflection_value)),
        penetration_reduction: penetration.map_or(0.0, |p| p.base_penetration_reduction),
        penetration_absorption: penetration.and_then(|p| info(p.penetration_absorption_for_type)),
        signal_infrared: a.signal_infrared,
        signal_electromagnetic: a.signal_electromagnetic,
        signal_cross_section: a.signal_cross_section,
    }
}

fn per_type(d: &DamageInfo) -> PerDamageType<f32> {
    PerDamageType {
        physical: d.damage_physical,
        energy: d.damage_energy,
        distortion: d.damage_distortion,
        thermal: d.damage_thermal,
        biochemical: d.damage_biochemical,
        stun: d.damage_stun,
    }
}

fn quantum(q: &SCItemQuantumDriveParams, pools: &DataPools) -> Option<QuantumDrive> {
    let travel = |h: Option<Handle<SQuantumDriveParams>>| {
        h.and_then(|h| h.get(pools)).map(|p| QuantumTravel {
            drive_speed: p.drive_speed,
            cooldown_time: p.cooldown_time,
            spool_up_time: p.spool_up_time,
            stage_one_accel_rate: p.stage_one_accel_rate,
            stage_two_accel_rate: p.stage_two_accel_rate,
            engage_speed: p.engage_speed,
            calibration_rate: p.calibration_rate,
            min_calibration_requirement: p.min_calibration_requirement,
            max_calibration_requirement: p.max_calibration_requirement,
            calibration_delay: p.calibration_delay_in_seconds,
        })
    };
    Some(QuantumDrive {
        travel: travel(q.params)?,
        spline: travel(q.spline_jump_params),
        fuel_requirement: q.quantum_fuel_requirement,
        jump_range: q.jump_range,
        disconnect_range: q.disconnect_range,
    })
}

fn thruster(t: &SCItemThrusterParams) -> Thruster {
    Thruster {
        thrust_capacity: t.thrust_capacity,
        thruster_type: t.thruster_type.clone(),
        fuel_burn_rate_per_10kn: t.fuel_burn_rate_per10_knewton,
        min_health_thrust_multiplier: t.min_health_thrust_multiplier,
        only_active_in_vtol: t.only_active_in_vtol,
    }
}

/// `serde` adapters for the generated enums (they derive nothing).
mod face_type_serde {
    use sc_extract::generated::EFaceType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &EFaceType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EFaceType, D::Error> {
        Ok(EFaceType::from_dcb_str(&String::deserialize(d)?))
    }
}

mod thruster_type_serde {
    use sc_extract::generated::EThrusterType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &EThrusterType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EThrusterType, D::Error> {
        Ok(EThrusterType::from_dcb_str(&String::deserialize(d)?))
    }
}
