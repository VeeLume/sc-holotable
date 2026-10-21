//! Per-kind stat sheets, read through the typed generated surface.

use sc_extract::generated::{
    BezierCurve, DamageBasePtr, DamageInfo, DamageResistanceBasePtr, DamageResistanceEntry,
    DataForgeComponentParamsPtr, EFaceType, ESignatureType, EThrusterType, EntityClassDefinition,
    Handle, IFCSParams, SBaseResourceUnitPtr, SCItemQuantumDriveParams, SCItemRadarComponentParams,
    SCItemShieldGeneratorParams, SCItemThrusterParams, SCItemVehicleArmorParams,
    SHealthComponentParams, SQuantumDriveParams, Vec3,
};
use sc_extract::{DataPools, Guid, RecordStore};
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
    /// `capacitorAssignmentInputOutputRegen` / `...RegenNavMode` /
    /// `...Usage`: regeneration and drain against the capacitor assignment
    /// (`CapacitorAssignmentInputOutput_Linear_50_100`: 0.5 + 0.5x).
    #[serde(default)]
    pub regen_curve: Option<Curve>,
    #[serde(default)]
    pub nav_regen_curve: Option<Curve>,
    #[serde(default)]
    pub usage_curve: Option<Curve>,
    /// `afterburnerAngCapacitorScalingCurve`, when
    /// `afterburnerAngCapacitorScaling` is set: how much of the angular
    /// multipliers the assignment grants.
    #[serde(default)]
    pub angular_scaling: Option<Curve>,
}

/// The capacitor assignment level boost starts at: 2 of 6, the client's
/// 1/1/1 split of the three capacitors (client 4.10.197, `FUN_143de8180`;
/// not in the data).
pub const DEFAULT_CAPACITOR_LEVEL: u8 = 2;

/// [`Afterburner::at`]: boost at one capacitor assignment level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Boost {
    /// From empty to full, s, including the regen delay after use.
    pub regen_s: f32,
    /// From full to empty at rest, s (the idle cost; moving adds the linear
    /// and angular costs). `None` when nothing drains at rest.
    pub deplete_s: Option<f32>,
    /// The boosted angular velocity multipliers at full ramp (x pitch, y
    /// roll, z yaw, as [`Afterburner::angular_velocity_multiplier`]).
    pub angular_velocity_multiplier: [f32; 3],
}

impl Afterburner {
    /// Boost at capacitor assignment `level` of 6 (clamped): the client
    /// gives the flight controller `x = level / 6`
    /// ([`DEFAULT_CAPACITOR_LEVEL`] at spawn). Client 4.10.197
    /// (`FUN_145b4dd60`, `sub_145b32710`):
    ///
    /// - regen: `capacitorMax / (capacitorRegenPerSec × regen(x))` plus
    ///   `capacitorRegenDelayAfterUse`;
    /// - drain at rest: `capacitorMax / (idleCost × usage(x))`;
    /// - angular velocity: `1 + (multiplier − 1) × scaling(x)`.
    ///
    /// A missing curve counts as 1. The Constellation Mk5 refills in 26.7 s
    /// (+1.1) at 6/6 and drains in 20 s, as spviewer shows.
    pub fn at(&self, level: u8) -> Boost {
        let x = f32::from(level.min(6)) / 6.0;
        let curve = |c: &Option<Curve>| c.as_ref().map_or(1.0, |c| c.at(x));
        let regen = self.regen_per_sec * curve(&self.regen_curve);
        let drain = self.idle_cost * curve(&self.usage_curve);
        let scaling = curve(&self.angular_scaling);
        Boost {
            regen_s: self.capacitor_max / regen + self.regen_delay,
            deplete_s: (drain > 0.0).then(|| self.capacitor_max / drain),
            angular_velocity_multiplier: self
                .angular_velocity_multiplier
                .map(|m| 1.0 + (m - 1.0) * scaling),
        }
    }
}

/// A `BezierCurve`: cubic segments between keys, `y` as a function of `x`.
/// The data stores each key as three points, in-handle, point, out-handle,
/// all in absolute coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Curve {
    /// `[in_handle, point, out_handle]` per key, in `x` order.
    pub keys: Vec<[[f32; 2]; 3]>,
}

impl Curve {
    /// `y` at `x`, clamped to the first and last key; each segment is solved
    /// for `x(t)` by bisection (the curves are monotonic in `x`).
    pub fn at(&self, x: f32) -> f32 {
        let (Some(first), Some(last)) = (self.keys.first(), self.keys.last()) else {
            return 1.0;
        };
        if x <= first[1][0] {
            return first[1][1];
        }
        if x >= last[1][0] {
            return last[1][1];
        }
        let Some(w) = self
            .keys
            .windows(2)
            .find(|w| x >= w[0][1][0] && x <= w[1][1][0])
        else {
            return last[1][1];
        };
        let p = [w[0][1], w[0][2], w[1][0], w[1][1]];
        let bezier = |t: f32, i: usize| {
            let u = 1.0 - t;
            u * u * u * p[0][i]
                + 3.0 * u * u * t * p[1][i]
                + 3.0 * u * t * t * p[2][i]
                + t * t * t * p[3][i]
        };
        let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if bezier(mid, 0) < x {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        bezier(0.5 * (lo + hi), 1)
    }

    fn from_dcb(c: &BezierCurve, pools: &DataPools) -> Option<Self> {
        let points: Vec<[f32; 2]> = c
            .points
            .iter()
            .filter_map(|h| h.get(pools))
            .map(|v| [v.x, v.y])
            .collect();
        let keys: Vec<[[f32; 2]; 3]> = points.chunks_exact(3).map(|k| [k[0], k[1], k[2]]).collect();
        (!keys.is_empty()).then_some(Self { keys })
    }
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

/// `SCItemMissileControllerParams`: how a missile controller fires. On
/// LIVE 4.10 the common `Controller_Missile` arms 4 at once and waits 4 s
/// between launches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissileController {
    /// `maxArmedMissiles`: missiles armed at once.
    pub max_armed: i32,
    /// `launchCooldownTime`, s.
    pub launch_cooldown: f32,
    /// `lockAngleAtMin` / `lockAngleAtMax`, degrees.
    pub lock_angle_min: f32,
    pub lock_angle_max: f32,
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
    pub missiles: Option<MissileController>,
}

pub(crate) fn sheets(ecd: &EntityClassDefinition, store: &RecordStore) -> Sheets {
    use DataForgeComponentParamsPtr as C;
    let pools = &store.pools;
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
            C::IFCSParams(h) => out.flight = h.get(pools).map(|f| flight(f, store)),
            C::SCItemRadarComponentParams(h) => out.radar = h.get(pools).map(|r| radar(r, pools)),
            C::SAmmoContainerComponentParams(h) => {
                out.ammo = h.get(pools).map(|a| Ammo {
                    initial: a.initial_ammo_count,
                    max: a.max_ammo_count,
                    max_restock: a.max_restock_count,
                    ammo_params: a.ammo_params_record,
                });
            }
            C::SCItemMissileControllerParams(h) => {
                out.missiles = h.get(pools).map(|m| MissileController {
                    max_armed: m.max_armed_missiles,
                    launch_cooldown: m.launch_cooldown_time,
                    lock_angle_min: m.lock_angle_at_min,
                    lock_angle_max: m.lock_angle_at_max,
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

fn flight(f: &IFCSParams, store: &RecordStore) -> FlightControl {
    let pools = &store.pools;
    let assignment = |g: Option<sc_extract::Guid>| {
        let def = store
            .records
            .multi_feature
            .capacitor_assignment_input_output_def
            .get(&g?)?
            .get(pools)?;
        Curve::from_dcb(def.input_output_mapping?.get(pools)?, pools)
    };
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
                regen_curve: assignment(a.capacitor_assignment_input_output_regen),
                nav_regen_curve: assignment(a.capacitor_assignment_input_output_regen_nav_mode),
                usage_curve: assignment(a.capacitor_assignment_input_output_usage),
                angular_scaling: a
                    .afterburner_ang_capacitor_scaling
                    .then(|| a.afterburner_ang_capacitor_scaling_curve?.get(pools))
                    .flatten()
                    .and_then(|c| Curve::from_dcb(c, pools)),
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
    fn centaurus_boost() {
        // Controller_Flight_RSI_Constellation_Mk5, legacy `afterburner`.
        let linear_50_100 = Curve {
            keys: vec![
                [[-0.1, 1.0], [0.0, 0.5], [0.0, 0.5]],
                [[1.0, 1.0], [1.0, 1.0], [0.0, 0.0]],
            ],
        };
        let s_curve = Curve {
            keys: vec![
                [[-0.4583936, -0.1252766], [0.0, 0.0], [0.4583936, 0.1252766]],
                [[0.3870683, 0.9817872], [1.0, 1.0], [1.612932, 1.018213]],
            ],
        };
        let a = Afterburner {
            accel_multiplier_positive: [1.0; 3],
            accel_multiplier_negative: [1.0; 3],
            angular_velocity_multiplier: [1.5, 2.0, 1.5],
            angular_accel_multiplier: [1.0; 3],
            ramp_up_time: 2.0,
            ramp_down_time: 0.2,
            capacitor_max: 20.0,
            idle_cost: 1.0,
            linear_cost: 0.0,
            angular_cost: 0.0,
            regen_delay: 1.1,
            regen_per_sec: 0.75,
            threshold_ratio: 0.25,
            regen_curve: Some(linear_50_100.clone()),
            nav_regen_curve: Some(linear_50_100),
            usage_curve: None,
            angular_scaling: Some(s_curve),
        };
        let close = |a: f32, b: f32| (a - b).abs() < 1e-2;
        let full = a.at(6);
        assert!(close(full.regen_s, 26.667 + 1.1), "{full:?}");
        assert_eq!(full.deplete_s, Some(20.0));
        assert!(close(full.angular_velocity_multiplier[1], 2.0));
        let default = a.at(DEFAULT_CAPACITOR_LEVEL);
        assert!(close(default.regen_s, 40.0 + 1.1), "{default:?}");
        assert!(
            close(default.angular_velocity_multiplier[0], 1.162),
            "{default:?}"
        );
        assert!(
            close(default.angular_velocity_multiplier[1], 1.323),
            "{default:?}"
        );
        let off = a.at(0);
        assert!(close(off.regen_s, 53.333 + 1.1));
        assert_eq!(off.angular_velocity_multiplier, [1.0; 3]);
    }

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
