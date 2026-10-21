//! Per-kind stat sheets, read through the typed generated surface.

use sc_extract::generated::{
    DamageBasePtr, DamageInfo, DamageResistanceBasePtr, DamageResistanceEntry,
    DataForgeComponentParamsPtr, EFaceType, ESignatureType, EThrusterType, EntityClassDefinition,
    Handle, IFCSParams, SBaseResourceUnitPtr, SCItemQuantumDriveParams, SCItemRadarComponentParams,
    SCItemShieldGeneratorParams, SCItemThrusterParams, SCItemVehicleArmorParams,
    SHealthComponentParams, SQuantumDriveParams, Vec3,
};
use sc_extract::{DataPools, Guid};
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
    /// `fuelBurnRatePer10KNewtonRN`: hydrogen, SCU per second per 10 kN of
    /// thrust, the rate the client burns (`FUN_1447adbf0`; the legacy
    /// `fuelBurnRatePer10KNewton` float is unused).
    pub fuel_burn_per_10kn: f64,
    /// `minHealthThrustMultiplier`: the thrust left at 0 health.
    pub min_health_thrust_multiplier: f32,
    /// `onlyActiveInVTOL`.
    pub only_active_in_vtol: bool,
}

/// `IFCSParams` on a flight controller, the legacy fields the PU runs
/// (`IFCSGameModeParams_Default.enableNewModel` is false in LIVE 4.10).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightControl {
    /// `scmSpeed`, m/s.
    pub scm_speed: f32,
    /// `boostSpeedForward` / `boostSpeedBackward`, m/s.
    pub boost_speed_forward: f32,
    pub boost_speed_backward: f32,
    /// `maxSpeed`, m/s (Navigation).
    pub max_speed: f32,
    /// `maxAngularVelocity`, deg/s: x pitch, y roll, z yaw.
    pub max_angular_velocity: [f32; 3],
    /// `positiveLinearScale` / `negativeLinearScale` (x strafe, y forward /
    /// back, z up / down).
    pub positive_linear_scale: [f32; 3],
    pub negative_linear_scale: [f32; 3],
    /// `linearAccelDecay` / `angularAccelDecay`.
    pub linear_accel_decay: f32,
    pub angular_accel_decay: f32,
    /// `afterburner`.
    pub afterburner: Option<Afterburner>,
}

/// `AfterburnerParams` (the legacy `IFCSParams.afterburner`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Afterburner {
    /// `afterburnAccelMultiplierPositive` / `Negative` (x, y, z as for the
    /// linear scales).
    pub accel_multiplier_positive: [f32; 3],
    pub accel_multiplier_negative: [f32; 3],
    /// `afterburnAngVelocityMultiplier` / `afterburnAngAccelMultiplier`
    /// (x pitch, y roll, z yaw).
    pub angular_velocity_multiplier: [f32; 3],
    pub angular_accel_multiplier: [f32; 3],
    /// `afterburnerRampUpTime` / `afterburnerRampDownTime`, s.
    pub ramp_up_time: f32,
    pub ramp_down_time: f32,
    /// `capacitorMax`.
    pub capacitor_max: f32,
    /// `capacitorAfterburnerIdleCost` / `LinearCost` / `AngularCost`, per s.
    pub idle_cost: f32,
    pub linear_cost: f32,
    pub angular_cost: f32,
    /// `capacitorRegenDelayAfterUse`, s, and `capacitorRegenPerSec`.
    pub regen_delay: f32,
    pub regen_per_sec: f32,
    /// `afterburnerCapacitorThresholdRatio`.
    pub threshold_ratio: f32,
}

/// `SCItemRadarComponentParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Radar {
    /// `signatureDetection[]`, one per signature type.
    pub detection: Vec<SignatureDetection>,
    /// `aimAssist.distanceMinAssignment` / `distanceMaxAssignment` /
    /// `outsideRangeBufferDistance`, m.
    pub aim_assist_min: f32,
    pub aim_assist_max: f32,
    pub aim_assist_buffer: f32,
    /// `pingProperties.cooldownTime`, s.
    pub ping_cooldown: Option<f32>,
}

impl Radar {
    /// The aim-assist distance, m, at the radar's functionality ratio
    /// (client LIVE 4.10.197, `FUN_1433e2750` on
    /// `SSCEvtNotify_FunctionalityRatioChanged`): `min + (max − min) ×
    /// ratio`, continuous; `None` (no aim assist) at a ratio of 0.
    ///
    /// The functionality ratio (`GetConsumptionFunctionalityRatio`) is
    /// what each resource the radar consumes supplies over what it asks,
    /// multiplied over its resources (power, coolant), then scaled by
    /// health and temperature curves (1 for a sound, cool radar). With its
    /// coolant met, it is the power ratio: granted over requested segments,
    /// so it moves a pip at a time (the Constellation Mk V's Lephari at 6 of
    /// 8 reaches 1,860 + 0.75 × 1,130 = 2,707.5 m).
    ///
    /// [`aim_assist_buffer`](Self::aim_assist_buffer) is no part of the
    /// distance: assist starts within it and, once on, holds out to
    /// distance + buffer. Crafting quality scales min and max first
    /// (`GPP_Radar_Min/MaxAimAssistDistance`); not applied here.
    pub fn aim_assist(&self, ratio: f32) -> Option<f32> {
        (ratio > f32::EPSILON)
            .then_some(self.aim_assist_min + (self.aim_assist_max - self.aim_assist_min) * ratio)
    }
}

/// One `SCItemRadarSignatureDetection`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignatureDetection {
    /// The signature type: the entry's index in `ESignatureType` order
    /// (Infrared, Electromagnetic, CrossSection, Decibel, Resource,
    /// Identity, CommsSignal, Interactable; the eight entries match the
    /// eight values, the mapping by index is inferred).
    #[serde(with = "signature_type_serde")]
    pub signature: ESignatureType,
    pub sensitivity: f32,
    pub piercing: f32,
    /// `permitPassiveDetection` / `permitActiveDetection`.
    pub passive: bool,
    pub active: bool,
}

/// `SAmmoContainerComponentParams`: a countermeasure launcher's, a
/// ballistic gun's or a missile rack's ammunition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ammo {
    /// `initialAmmoCount` / `maxAmmoCount`.
    pub initial: i32,
    pub max: i32,
    /// `maxRestockCount`.
    pub max_restock: u32,
    /// `ammoParamsRecord`: the `AmmoParams` it fires.
    pub ammo_params: Option<Guid>,
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
    pub flight: Option<FlightControl>,
    pub radar: Option<Radar>,
    pub ammo: Option<Ammo>,
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
            C::SCItemThrusterParams(h) => out.thruster = h.get(pools).map(|t| thruster(t, pools)),
            C::IFCSParams(h) => out.flight = h.get(pools).map(|f| flight(f, pools)),
            C::SCItemRadarComponentParams(h) => out.radar = h.get(pools).map(|r| radar(r, pools)),
            C::SAmmoContainerComponentParams(h) => {
                out.ammo = h.get(pools).map(|a| Ammo {
                    initial: a.initial_ammo_count,
                    max: a.max_ammo_count,
                    max_restock: a.max_restock_count,
                    ammo_params: a.ammo_params_record,
                });
            }
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

/// An entity's `SHealthComponentParams` as a [`ComponentHealth`]; `None`
/// without one.
pub fn component_health(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<ComponentHealth> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SHealthComponentParams(h) => {
            h.get(pools).map(|h| health(h, pools))
        }
        _ => None,
    })
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

fn flight(f: &IFCSParams, pools: &DataPools) -> FlightControl {
    let v = |h: Option<Handle<Vec3>>| {
        h.and_then(|h| h.get(pools))
            .map_or([0.0; 3], |v| [v.x, v.y, v.z])
    };
    FlightControl {
        scm_speed: f.scm_speed,
        boost_speed_forward: f.boost_speed_forward,
        boost_speed_backward: f.boost_speed_backward,
        max_speed: f.max_speed,
        max_angular_velocity: v(f.max_angular_velocity),
        positive_linear_scale: v(f.positive_linear_scale),
        negative_linear_scale: v(f.negative_linear_scale),
        linear_accel_decay: f.linear_accel_decay,
        angular_accel_decay: f.angular_accel_decay,
        afterburner: f
            .afterburner
            .and_then(|h| h.get(pools))
            .map(|a| Afterburner {
                accel_multiplier_positive: v(a.afterburn_accel_multiplier_positive),
                accel_multiplier_negative: v(a.afterburn_accel_multiplier_negative),
                angular_velocity_multiplier: v(a.afterburn_ang_velocity_multiplier),
                angular_accel_multiplier: v(a.afterburn_ang_accel_multiplier),
                ramp_up_time: a.afterburner_ramp_up_time,
                ramp_down_time: a.afterburner_ramp_down_time,
                capacitor_max: a.capacitor_max,
                idle_cost: a.capacitor_afterburner_idle_cost,
                linear_cost: a.capacitor_afterburner_linear_cost,
                angular_cost: a.capacitor_afterburner_angular_cost,
                regen_delay: a.capacitor_regen_delay_after_use,
                regen_per_sec: a.capacitor_regen_per_sec,
                threshold_ratio: a.afterburner_capacitor_threshold_ratio,
            }),
    }
}

fn radar(r: &SCItemRadarComponentParams, pools: &DataPools) -> Radar {
    const TYPES: [ESignatureType; 8] = [
        ESignatureType::Infrared,
        ESignatureType::Electromagnetic,
        ESignatureType::CrossSection,
        ESignatureType::Decibel,
        ESignatureType::Resource,
        ESignatureType::Identity,
        ESignatureType::CommsSignal,
        ESignatureType::Interactable,
    ];
    let aim = r.aim_assist.and_then(|h| h.get(pools));
    Radar {
        detection: r
            .signature_detection
            .iter()
            .zip(TYPES)
            .filter_map(|(d, signature)| {
                let d = d.get(pools)?;
                Some(SignatureDetection {
                    signature,
                    sensitivity: d.sensitivity,
                    piercing: d.piercing,
                    passive: d.permit_passive_detection,
                    active: d.permit_active_detection,
                })
            })
            .collect(),
        aim_assist_min: aim.map_or(0.0, |a| a.distance_min_assignment),
        aim_assist_max: aim.map_or(0.0, |a| a.distance_max_assignment),
        aim_assist_buffer: aim.map_or(0.0, |a| a.outside_range_buffer_distance),
        ping_cooldown: r
            .ping_properties
            .and_then(|h| h.get(pools))
            .map(|p| p.cooldown_time),
    }
}

/// A resource-unit amount in standard units (1 SCU).
fn standard_units(u: &SBaseResourceUnitPtr, pools: &DataPools) -> f64 {
    match u {
        SBaseResourceUnitPtr::SStandardResourceUnit(h) => h
            .get(pools)
            .map_or(0.0, |u| f64::from(u.standard_resource_units)),
        SBaseResourceUnitPtr::SCentiResourceUnit(h) => h
            .get(pools)
            .map_or(0.0, |u| f64::from(u.centi_resource_units) / 100.0),
        SBaseResourceUnitPtr::SMicroResourceUnit(h) => h
            .get(pools)
            .map_or(0.0, |u| f64::from(u.micro_resource_units) / 1e6),
        SBaseResourceUnitPtr::SPowerSegmentResourceUnit(h) => {
            h.get(pools).map_or(0.0, |u| f64::from(u.units))
        }
        _ => 0.0,
    }
}

fn thruster(t: &SCItemThrusterParams, pools: &DataPools) -> Thruster {
    Thruster {
        thrust_capacity: t.thrust_capacity,
        thruster_type: t.thruster_type.clone(),
        fuel_burn_per_10kn: t
            .fuel_burn_rate_per10_knewton_rn
            .as_ref()
            .map_or(0.0, |u| standard_units(u, pools)),
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

mod signature_type_serde {
    use sc_extract::generated::ESignatureType;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &ESignatureType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<ESignatureType, D::Error> {
        Ok(ESignatureType::from_dcb_str(&String::deserialize(d)?))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lephari_aim_assist() {
        let lephari = Radar {
            detection: Vec::new(),
            aim_assist_min: 1860.0,
            aim_assist_max: 2990.0,
            aim_assist_buffer: 120.0,
            ping_cooldown: None,
        };
        // Centaurus power-on: 6 of 8 segments.
        assert_eq!(lephari.aim_assist(0.75), Some(2707.5));
        assert_eq!(lephari.aim_assist(1.0), Some(2990.0));
        assert_eq!(lephari.aim_assist(0.0), None);
    }
}
