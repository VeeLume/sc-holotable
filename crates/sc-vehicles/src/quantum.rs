//! Quantum travel: time, fuel and range, as the client computes them (LIVE
//! 4.10.197).

use sc_extract::generated::ResourceNetworkResource;
use sc_extract::{Guid, RecordCollection};
use sc_items_ship_components::{QuantumTravel, ShipComponentKind, ShipComponents};
use serde::{Deserialize, Serialize};

use crate::{Mounted, ResourceProfiles, ResourceUnits, Vehicles, network_items};

/// A loadout's quantum drive and quantum fuel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantum {
    /// The drive's `EntityClassDefinition`.
    pub drive: Guid,
    /// `SCItemQuantumDriveParams.params`: normal travel.
    pub travel: QuantumTravel,
    /// `splineJumpParams`: travel along a navigation spline.
    pub spline: Option<QuantumTravel>,
    /// Quantum fuel per Mm travelled, SCU: the drive's `Travelling` state's
    /// QuantumFuel consumption (micro-units per Mm; the top-level
    /// `quantumFuelRequirement` is the legacy, unused value).
    pub fuel_per_mm: f64,
    /// What the loadout's quantum fuel tanks hold, SCU.
    pub fuel_capacity: f64,
}

impl Quantum {
    /// The quantum drive on `loadout`'s own network and its tanks; `None`
    /// without a drive.
    pub fn compute(
        loadout: &[Mounted],
        vehicles: &Vehicles,
        components: &ShipComponents,
        profiles: &ResourceProfiles,
    ) -> Option<Self> {
        let mut drive = None;
        let mut fuel_capacity = 0.0;
        for (_, item) in network_items(loadout, vehicles) {
            let Some(sheet) = components.get(&item) else {
                continue;
            };
            if drive.is_none()
                && let Some(q) = &sheet.quantum
            {
                drive = Some((item, q));
            }
            if sheet.kind == ShipComponentKind::QuantumFuelTank {
                fuel_capacity += sheet
                    .capacity
                    .as_ref()
                    .and_then(|c| c.to_scu())
                    .map_or(0.0, f64::from);
            }
        }
        let (item, q) = drive?;
        let fuel_per_mm = profiles
            .get(&item)
            .and_then(|p| p.state("Travelling"))
            .and_then(|s| s.consumption(&ResourceNetworkResource::QuantumFuel))
            .and_then(|d| d.consumes.as_ref())
            .map_or(0.0, |a| match a.per_second {
                ResourceUnits::Micro(n) => f64::from(n) / 1e6,
                other => other.standard(),
            });
        Some(Self {
            drive: item,
            travel: q.travel,
            spline: q.spline,
            fuel_per_mm,
            fuel_capacity,
        })
    }

    /// Seconds to travel `distance` m from rest ([`quantum_time`]), spool
    /// and calibration not included.
    pub fn time(&self, distance: f64) -> f64 {
        quantum_time(&self.travel, distance)
    }

    /// Quantum fuel for `distance` m, SCU.
    pub fn fuel(&self, distance: f64) -> f64 {
        distance / 1e6 * self.fuel_per_mm
    }

    /// How far a full tank goes, m.
    pub fn range(&self) -> f64 {
        if self.fuel_per_mm > 0.0 {
            self.fuel_capacity / self.fuel_per_mm * 1e6
        } else {
            f64::INFINITY
        }
    }
}

/// Seconds a quantum jump of `distance` m takes from rest, as the client's
/// travel envelope runs it (`CTravelEnvelope::SetupEnvelope`,
/// `FUN_144dbfc00`): five phases mirrored about the middle.
///
/// 1. Accelerate at `stage_one_accel_rate` to `engage_speed`.
/// 2. Ramp the acceleration linearly to `stage_two_accel_rate` while
///    reaching `drive_speed`.
/// 3. Cruise.
/// 4. Ramp down (mirror of 2).
/// 5. Decelerate (mirror of 1).
///
/// A trip too short for full speed solves for the peak instead: within
/// phase 1, a quadratic; within phase 2, a cubic. Each phase's time is held
/// in 1e-7 s ticks, truncated. Spool-up and calibration come before and
/// are not included.
///
/// The Khaos (Constellation Mk V Centaurus) needs 9.25 GM to reach full
/// speed and takes 170.6 s for 31.92 GM. spviewer shows 2:46: an older
/// accelerate / decelerate model, rounded up.
pub fn quantum_time(t: &QuantumTravel, distance: f64) -> f64 {
    let tick = |s: f64| (s * 1e7).floor() / 1e7;
    let (a1, a2) = (
        f64::from(t.stage_one_accel_rate),
        f64::from(t.stage_two_accel_rate),
    );
    let (mut e, mut v) = (f64::from(t.engage_speed), f64::from(t.drive_speed));
    if a1 <= 0.0 || v <= 0.0 || distance <= 0.0 {
        return 0.0;
    }
    let mut t1 = tick(e / a1);
    let mut d1 = 0.5 * a1 * t1 * t1;
    if distance < 2.0 * d1 {
        let t = (distance / a1).sqrt();
        e = t * a1;
        v = t * a1 + e;
        t1 = tick(t);
        d1 = distance / 2.0;
    }
    let mut t2 = tick(2.0 * (v - e) / (a1 + a2));
    let j = if t2 > 0.0 { (a2 - a1) / t2 } else { 0.0 };
    let mut d2 = (0.5 * a1 * t2 * t2 + t2 * e + j / 6.0 * t2.powi(3)).abs();
    if 2.0 * d1 - f64::EPSILON < distance && distance < 2.0 * d2 + 2.0 * d1 {
        let half = (distance - 2.0 * d1) * 0.5;
        let t = cubic_root(j / 6.0, 0.5 * a1, e, -half);
        v = 0.5 * j * t * t + a1 * t + e;
        t2 = tick(t);
        d2 = half;
    }
    if distance - 2.0 * d1 <= 0.0 {
        d2 = 0.0;
        t2 = 0.0;
    }
    let cruise = (distance - (2.0 * d2 + 2.0 * d1)).max(0.0);
    let tc = tick(cruise / v);
    2.0 * t1 + 2.0 * t2 + tc
}

/// The smallest positive root of `a t³ + b t² + c t + d` with `a, b, c ≥ 0`
/// and `d < 0` (the function rises from `d`), by bisection.
fn cubic_root(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let f = |t: f64| ((a * t + b) * t + c) * t + d;
    let (mut lo, mut hi) = (0.0, 1.0);
    while f(hi) < 0.0 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if f(m) < 0.0 {
            lo = m;
        } else {
            hi = m;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn khaos() -> QuantumTravel {
        QuantumTravel {
            drive_speed: 250_857_100.0,
            cooldown_time: 13.5,
            spool_up_time: 7.6,
            stage_one_accel_rate: 5_405_143.0,
            stage_two_accel_rate: 8_177_143.0,
            engage_speed: 84e6,
            calibration_rate: 1000.0,
            min_calibration_requirement: 5000.0,
            max_calibration_requirement: 10_000.0,
            calibration_delay: 1.5,
        }
    }

    #[test]
    fn envelope() {
        let k = khaos();
        // Full speed: 31.92 GM in 2:50.6; 10 GM in 83.2 s.
        assert!((quantum_time(&k, 31.92e9) - 170.58).abs() < 0.01);
        assert!((quantum_time(&k, 10e9) - 83.20).abs() < 0.01);
        // Short trips (no cruise): 5 GM 60.1 s, 1 GM 27.2 s.
        assert!((quantum_time(&k, 5e9) - 60.1).abs() < 0.05);
        assert!((quantum_time(&k, 1e9) - 27.2).abs() < 0.05);
        // Monotone through the phase boundaries.
        let mut last = 0.0;
        for gm in [0.1, 0.5, 1.3, 1.31, 2.0, 9.2, 9.26, 12.0] {
            let t = quantum_time(&k, gm * 1e9);
            assert!(t > last, "{gm} GM: {t} <= {last}");
            last = t;
        }
    }
}
