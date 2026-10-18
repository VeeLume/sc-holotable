//! A loadout's flight performance under the legacy IFCS the PU runs
//! (LIVE 4.10: `IFCSGameModeParams_Default.enableNewModel` is false).

use std::collections::HashMap;

use sc_extract::generated::EThrusterType;
use sc_extract::{AssetSource, Mesh, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::{ShipComponentKind, ShipComponents};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{Mounted, Vehicle, Vehicles};

/// Standard gravity, m/s², for accelerations in G.
const G: f64 = 9.80665;

/// Where each of a hull's item ports pushes: the +Y axis of the helper node
/// the port sits on, in the ship's frame (x right, y forward, z up), keyed
/// by lower-case port name.
///
/// The engine attaches an item port to the hull helper bone of the same
/// name; no typed link exists, so this joins on the name (scoped to one
/// hull's nodes).
pub fn port_axes(assets: &AssetSource, vehicle: &Vehicle) -> HashMap<String, [f64; 3]> {
    let Some(path) = vehicle.geometry.as_deref() else {
        return HashMap::new();
    };
    let mesh = match Mesh::read(assets, path) {
        Ok(m) => m,
        Err(error) => {
            warn!(path, %error, "hull geometry unreadable; no thruster axes");
            return HashMap::new();
        }
    };
    // World transforms: parents come before children in the node table,
    // but resolve by walking up to be safe.
    let world = |mut i: usize| {
        let mut m = mesh.nodes[i].transform;
        while let Some(p) = mesh.nodes[i].parent {
            m = mul(&mesh.nodes[p].transform, &m);
            i = p;
        }
        m
    };
    mesh.nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.name.starts_with("hardpoint_"))
        .map(|(i, n)| {
            let w = world(i);
            let y = [f64::from(w[0][1]), f64::from(w[1][1]), f64::from(w[2][1])];
            (n.name.to_ascii_lowercase(), normalize(y))
        })
        .collect()
}

/// A loadout's flight figures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    /// Hull plus everything mounted, docked vehicles included, kg.
    pub mass: f64,
    /// `scmSpeed`, `boostSpeedForward` / `Backward`, `maxSpeed`, m/s.
    pub scm_speed: f32,
    pub boost_speed_forward: f32,
    pub boost_speed_backward: f32,
    pub max_speed: f32,
    /// deg/s: pitch, roll, yaw; boosted × the afterburner's angular
    /// velocity multipliers.
    pub angular_velocity: [f32; 3],
    pub angular_velocity_boosted: [f32; 3],
    /// G per direction, and boosted × the afterburner's acceleration
    /// multipliers.
    pub acceleration: Accelerations,
    pub acceleration_boosted: Accelerations,
    /// Hydrogen burnt at full thrust, SCU per minute, per thruster type:
    /// `thrustCapacity` / 10 kN × `fuelBurnRatePer10KNewtonRN` per second
    /// (`FUN_1447adbf0`). Nothing burns at rest (the flight controller adds
    /// no idle cost in LIVE 4.10).
    pub hydrogen_burn: Burn,
    /// What the loadout's hydrogen tanks hold, SCU.
    pub hydrogen_capacity: f64,
}

/// Hydrogen per minute at full thrust, SCU, by thruster type.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Burn {
    pub main: f64,
    pub retro: f64,
    pub maneuver: f64,
}

/// Accelerations per direction, G.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Accelerations {
    pub forward: f64,
    pub backward: f64,
    pub left: f64,
    pub right: f64,
    pub up: f64,
    pub down: f64,
}

impl Flight {
    /// The flight figures of `loadout` on `vehicle`, with `axes` from
    /// [`port_axes`]; `None` without a flight controller.
    ///
    /// Acceleration per direction is the thrust that pushes that way over
    /// the mass: each thruster counts once, along the direction its port's
    /// axis is most aligned with, by that component (a side maneuvering
    /// thruster tilted 20° up adds to strafe, not to up), VTOL-only
    /// thrusters left out. This is inferred from the data (the client's
    /// acceleration solver was not decompiled); it reproduces spviewer's G
    /// figures for the Centaurus.
    pub fn compute(
        vehicle: &Vehicle,
        loadout: &[Mounted],
        vehicles: &Vehicles,
        items: &Items,
        components: &ShipComponents,
        axes: &HashMap<String, [f64; 3]>,
    ) -> Option<Self> {
        let mut mass = f64::from(vehicle.hull_mass().unwrap_or(0.0));
        let mut thrust = [0.0f64; 6]; // +x, -x, +y, -y, +z, -z
        let mut control = None;
        let mut burn = Burn::default();
        let mut hydrogen_capacity = 0.0;
        let mut stack: Vec<(&Mounted, Option<&str>, bool)> =
            loadout.iter().map(|m| (m, None, false)).collect();
        while let Some((m, outer, docked)) = stack.pop() {
            let Some(item) = m.item else {
                continue;
            };
            let carried = vehicles.get(&item);
            if let Some(v) = carried {
                mass += f64::from(v.hull_mass().unwrap_or(0.0));
            } else if let Some(kg) = items.get(&item).and_then(|i| i.mass) {
                mass += f64::from(kg);
            }
            let sheet = components.get(&item);
            if !docked {
                if control.is_none() {
                    control = sheet.and_then(|s| s.flight.clone());
                }
                if let Some(s) = sheet
                    && s.kind == ShipComponentKind::FuelTank
                {
                    hydrogen_capacity += s
                        .capacity
                        .as_ref()
                        .and_then(|c| c.to_scu())
                        .map_or(0.0, f64::from);
                }
                if let Some(t) = sheet.and_then(|s| s.thruster.as_ref())
                    && !t.only_active_in_vtol
                {
                    let per_min = f64::from(t.thrust_capacity) / 1e4 * t.fuel_burn_per_10kn * 60.0;
                    match t.thruster_type {
                        EThrusterType::Main => burn.main += per_min,
                        EThrusterType::Retro => burn.retro += per_min,
                        _ => burn.maneuver += per_min,
                    }
                    match axis(axes, &m.port, outer) {
                        Some(a) => push(&mut thrust, a, f64::from(t.thrust_capacity)),
                        None => warn!(port = %m.port, "thruster port has no hull helper"),
                    }
                }
            }
            let port = outer.unwrap_or(&m.port);
            stack.extend(
                m.children
                    .iter()
                    .map(|c| (c, Some(port), docked || carried.is_some())),
            );
        }
        let control = control?;
        let g = |n: f64| if mass > 0.0 { n / mass / G } else { 0.0 };
        let acceleration = Accelerations {
            right: g(thrust[0]),
            left: g(thrust[1]),
            forward: g(thrust[2]),
            backward: g(thrust[3]),
            up: g(thrust[4]),
            down: g(thrust[5]),
        };
        let (acceleration_boosted, angular_velocity_boosted) = match &control.afterburner {
            Some(ab) => {
                let (p, n) = (
                    ab.accel_multiplier_positive.map(f64::from),
                    ab.accel_multiplier_negative.map(f64::from),
                );
                (
                    Accelerations {
                        right: acceleration.right * p[0],
                        left: acceleration.left * n[0],
                        forward: acceleration.forward * p[1],
                        backward: acceleration.backward * n[1],
                        up: acceleration.up * p[2],
                        down: acceleration.down * n[2],
                    },
                    std::array::from_fn(|i| {
                        control.max_angular_velocity[i] * ab.angular_velocity_multiplier[i]
                    }),
                )
            }
            None => (acceleration, control.max_angular_velocity),
        };
        Some(Self {
            mass,
            scm_speed: control.scm_speed,
            boost_speed_forward: control.boost_speed_forward,
            boost_speed_backward: control.boost_speed_backward,
            max_speed: control.max_speed,
            angular_velocity: control.max_angular_velocity,
            angular_velocity_boosted,
            acceleration,
            acceleration_boosted,
            hydrogen_burn: burn,
            hydrogen_capacity,
        })
    }
}

/// The axis of a thruster's port, or of the outermost port above it (a
/// thruster mounted on a bracket).
fn axis<'a>(
    axes: &'a HashMap<String, [f64; 3]>,
    port: &str,
    outer: Option<&str>,
) -> Option<&'a [f64; 3]> {
    axes.get(&port.to_ascii_lowercase())
        .or_else(|| outer.and_then(|o| axes.get(&o.to_ascii_lowercase())))
}

/// Add a thruster to the direction it is most aligned with, by that
/// component of its axis.
fn push(thrust: &mut [f64; 6], axis: &[f64; 3], capacity: f64) {
    let i = (0..3)
        .max_by(|&a, &b| axis[a].abs().total_cmp(&axis[b].abs()))
        .unwrap_or(0);
    let c = axis[i];
    if c > 0.0 {
        thrust[2 * i] += capacity * c;
    } else {
        thrust[2 * i + 1] -= capacity * c;
    }
}

fn mul(a: &[[f32; 4]; 3], b: &[[f32; 4]; 3]) -> [[f32; 4]; 3] {
    let mut o = [[0.0; 4]; 3];
    for r in 0..3 {
        for c in 0..3 {
            o[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c];
        }
        o[r][3] = a[r][0] * b[0][3] + a[r][1] * b[1][3] + a[r][2] * b[2][3] + a[r][3];
    }
    o
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n > 0.0 { v.map(|c| c / n) } else { v }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thrust_projection() {
        let mut t = [0.0; 6];
        // A side maneuvering thruster on the left pushes right and a little
        // up (Centaurus hardpoint_thruster_mav_front_side_left): it counts
        // for strafe only.
        push(&mut t, &[0.940, 0.0, 0.342], 1000.0);
        push(&mut t, &[0.0, -1.0, 0.0], 500.0);
        assert!((t[0] - 940.0).abs() < 1e-9);
        assert_eq!((t[4], t[3]), (0.0, 500.0));
    }
}
