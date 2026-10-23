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

    /// A normal jump of `distance` m: travel time, peak speed and the
    /// cooldown after it ([`quantum_trip`]).
    pub fn trip(&self, distance: f64) -> QuantumTrip {
        quantum_trip(&self.travel, distance)
    }

    /// A spline jump of `distance` m, with the drive's `splineJumpParams`;
    /// `None` without them.
    pub fn spline_trip(&self, distance: f64) -> Option<QuantumTrip> {
        self.spline.as_ref().map(|s| quantum_trip(s, distance))
    }

    /// Seconds of cooldown after a normal jump of `distance` m.
    pub fn cooldown(&self, distance: f64) -> f64 {
        self.trip(distance).cooldown
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
    quantum_trip(t, distance).travel
}

/// One quantum jump of `distance` m from rest.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuantumTrip {
    /// Seconds from engage to arrival ([`quantum_time`]).
    pub travel: f64,
    /// The highest speed reached, m/s: `drive_speed` once the trip is long
    /// enough, less on a short one.
    pub peak: f64,
    /// Seconds of cooldown after arrival.
    pub cooldown: f64,
}

/// [`quantum_time`] plus the peak speed and the cooldown that follows.
///
/// The client starts the cooldown when the drive leaves travel
/// (`CSCItemQuantumDrive::RmMulticastOnAbortQuantum`, which a normal
/// arrival also runs) and sizes it by the trip's peak speed: `cooldownTime`
/// × clamp(peak / `driveSpeed`, 0, 1). Full speed gives the full
/// `cooldownTime`, spviewer's "Max Cooldown"; a short hop less. The Khaos
/// (13.5 s) cools for 3.96 s after 1 GM and 9.38 s after 5 GM.
pub fn quantum_trip(t: &QuantumTravel, distance: f64) -> QuantumTrip {
    let tick = |s: f64| (s * 1e7).floor() / 1e7;
    let (a1, a2) = (
        f64::from(t.stage_one_accel_rate),
        f64::from(t.stage_two_accel_rate),
    );
    let (mut e, mut v) = (f64::from(t.engage_speed), f64::from(t.drive_speed));
    let full = v;
    if a1 <= 0.0 || v <= 0.0 || distance <= 0.0 {
        return QuantumTrip {
            travel: 0.0,
            peak: 0.0,
            cooldown: 0.0,
        };
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
    // A trip inside phase 1 peaks at the engage speed it solved; `v` is
    // the drive speed or the cubic's peak otherwise.
    let mut peak = v;
    if distance - 2.0 * d1 <= 0.0 {
        d2 = 0.0;
        t2 = 0.0;
        peak = e;
    }
    let cruise = (distance - (2.0 * d2 + 2.0 * d1)).max(0.0);
    let tc = tick(cruise / v);
    QuantumTrip {
        travel: 2.0 * t1 + 2.0 * t2 + tc,
        peak,
        cooldown: f64::from(t.cooldown_time) * (peak / full).clamp(0.0, 1.0),
    }
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
        // Cooldown follows the peak speed: full from 9.25 GM.
        let cd = |gm: f64| quantum_trip(&k, gm * 1e9).cooldown;
        assert!((cd(1.0) - 3.96).abs() < 0.01);
        assert!((cd(5.0) - 9.38).abs() < 0.01);
        assert_eq!(cd(9.3), 13.5);
        assert_eq!(quantum_trip(&k, 31.92e9).peak, f64::from(k.drive_speed));
        // Inside phase 1 the peak is the solved engage speed.
        let hop = quantum_trip(&k, 0.1e9);
        assert!((hop.peak - (0.1e9 * 5_405_143.0f64).sqrt()).abs() < 1.0);
        // Monotone through the phase boundaries.
        let mut last = 0.0;
        for gm in [0.1, 0.5, 1.3, 1.31, 2.0, 9.2, 9.26, 12.0] {
            let t = quantum_time(&k, gm * 1e9);
            assert!(t > last, "{gm} GM: {t} <= {last}");
            last = t;
        }
    }
}
