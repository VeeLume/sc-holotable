//! Travel between star-map places: what one leg costs a given ship, and the
//! fastest path between two places, across systems through jump points.
//!
//! [`Travel`] holds the places ([`Waypoints`]), the bodies that block
//! quantum travel ([`Bodies`]) and the jump links ([`JumpLinks`]), all keyed
//! by `StarMapObject` GUID. It is static: bodies do not move in the data, and
//! the spin phase is not in it.
//!
//! # The cost model
//!
//! - **Quantum hop:** spool-up plus the client's travel envelope
//!   ([`sc_vehicles::quantum_trip`]) over the distance less the target's
//!   arrival radius. Calibration is not included. When a hop follows another,
//!   the first one's cooldown is waited out before the next spool-up.
//! - **Blocked line:** a quantum line that passes inside another body's
//!   `obstructionRadius` detours through that body's orbital markers
//!   ([`Body::orbital_markers`]), the way players fly it. Markers are placed
//!   at spin phase zero, so such a route is an [estimate](Hop::estimate).
//!   A place's own body never blocks it: where on the body it sits depends
//!   on the spin phase.
//! - **Spline hop:** two places on the same body. Costed with the drive's
//!   spline-jump parameters over the chord between them; the real path
//!   follows the surface, so this is an estimate.
//! - **Tunnel hop:** entrance, tunnel and exit length (one global record)
//!   at the drive's tunnel speed limit. A floor: the tunnel is flown and no
//!   duration is authored.
//!
//! Every hop says whether it is an estimate. Positions of places on a body
//! ignore its rotation (off by at most a body radius), so any hop touching
//! one is an estimate too, except between two places on the same body.
//!
//! Opening a jump point (alignment and tuning) is not in the tunnel hop: the
//! data has the rates, not how they combine into seconds.
//!
//! # Many stops
//!
//! [`Tour`] lists stops (one place or any of several), their dwell, arrival
//! overhead and ordering rules; [`Travel::order`] plans the fastest order
//! into a [`Plan`] (see the [`order`] module).
//!
//! ```no_run
//! # fn demo(travel: &sc_travel::Travel, drive: &sc_vehicles::Quantum,
//! #         from: sc_extract::Guid, to: sc_extract::Guid) {
//! if let Some(route) = travel.path(&from, &to, drive, |_| true) {
//!     for hop in &route.hops {
//!         println!("{:?} -> {:?}: {:.0} m, {:.0} s", hop.from, hop.to, hop.metres, hop.seconds);
//!     }
//!     println!("{:.0} s in all (estimate: {})", route.seconds(), route.is_estimate());
//! }
//! # }
//! ```

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, RecordCollection, RecordStore};
use sc_locations::{Bodies, Body, JumpLink, JumpLinks, Waypoint, Waypoints};
use sc_vehicles::Quantum;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

pub mod order;
pub use order::{EXACT_STOPS, Plan, StopId, Tour, TourEnd, TourStop, Visit};

/// The jump tunnel's geometry, m: `GlobalJumpTunnelHostParams` →
/// `tunnelGenerationParams`. One record covers every jump point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tunnel {
    /// `entranceLength` (15,000 on LIVE 4.10).
    pub entrance: f64,
    /// `tunnelLength` (110,000).
    pub length: f64,
    /// `exitLength` (8,000).
    pub exit: f64,
}

impl Tunnel {
    /// The tunnel record from `store`; `None` without exactly one.
    pub fn build(store: &RecordStore) -> Option<Self> {
        let records = &store.records.jumppoints.global_jump_tunnel_host_params;
        if records.len() != 1 {
            warn!(
                records = records.len(),
                "expected one GlobalJumpTunnelHostParams"
            );
        }
        let pools = &store.pools;
        let host = records.values().next()?.get(pools)?;
        let g = host.tunnel_generation_params?.get(pools)?;
        Some(Self {
            entrance: f64::from(g.entrance_length),
            length: f64::from(g.tunnel_length),
            exit: f64::from(g.exit_length),
        })
    }

    /// Entrance, tunnel and exit together, m.
    pub fn total(&self) -> f64 {
        self.entrance + self.length + self.exit
    }
}

/// A point a hop starts or ends at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stop {
    /// A star-map place ([`Waypoint`]).
    Place(Guid),
    /// Orbital marker OM-`number` (1 to 6) of a body.
    Marker { body: Guid, number: u8 },
}

/// How a hop is travelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HopKind {
    Quantum,
    /// A spline jump between two places on one body.
    Spline,
    /// Through a jump tunnel, entry to exit.
    Tunnel,
}

/// One hop of a [`Route`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Hop {
    pub from: Stop,
    pub to: Stop,
    pub kind: HopKind,
    /// Distance travelled, m.
    pub metres: f64,
    /// Spool-up plus travel (quantum, spline), or the tunnel transit.
    pub seconds: f64,
    /// The drive's cooldown after this hop, s (0 for a tunnel).
    pub cooldown: f64,
    /// Quantum fuel, SCU (0 for a tunnel: its fuel is not modelled).
    pub fuel: f64,
    /// True when a position or duration behind this hop is not exact (see
    /// the crate docs).
    pub estimate: bool,
}

/// A sequence of hops from one place to another.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub hops: Vec<Hop>,
}

impl Route {
    /// Distance, m.
    pub fn metres(&self) -> f64 {
        self.hops.iter().fold(0.0, |t, h| t + h.metres)
    }

    /// Time, s: every hop, plus the cooldown of a quantum or spline hop that
    /// another one follows.
    pub fn seconds(&self) -> f64 {
        let waits: f64 = self
            .hops
            .windows(2)
            .filter(|w| w[0].kind != HopKind::Tunnel && w[1].kind != HopKind::Tunnel)
            .fold(0.0, |t, w| t + w[0].cooldown);
        self.hops.iter().fold(waits, |t, h| t + h.seconds)
    }

    /// Quantum fuel, SCU.
    pub fn fuel(&self) -> f64 {
        self.hops.iter().fold(0.0, |t, h| t + h.fuel)
    }

    /// True when any hop is an estimate.
    pub fn is_estimate(&self) -> bool {
        self.hops.iter().any(|h| h.estimate)
    }

    fn extend(&mut self, other: Route) {
        self.hops.extend(other.hops);
    }
}

/// The travel graph: places, bodies, jump links and the tunnel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Travel {
    waypoints: Waypoints,
    bodies: Bodies,
    links: JumpLinks,
    tunnel: Option<Tunnel>,
}

/// A hop end with what costing needs.
#[derive(Clone, Copy)]
struct Point {
    stop: Stop,
    pos: [f64; 3],
    /// Quantum travel to it ends this far out.
    arrival: f64,
    /// Bodies a line to or from it does not test (its own body).
    exempt: [Option<Guid>; 2],
    estimate: bool,
}

impl Travel {
    /// Gather the graph. Reads the tunnel record from `datacore`.
    pub fn build(
        datacore: &Datacore,
        waypoints: &Waypoints,
        bodies: &Bodies,
        links: &JumpLinks,
    ) -> Self {
        let tunnel = Tunnel::build(datacore.records());
        info!(
            places = waypoints.len(),
            bodies = bodies.len(),
            links = links.len(),
            tunnel = tunnel.is_some(),
            "built travel graph"
        );
        Self {
            waypoints: waypoints.clone(),
            bodies: bodies.clone(),
            links: links.clone(),
            tunnel,
        }
    }

    pub fn waypoints(&self) -> &Waypoints {
        &self.waypoints
    }
    pub fn bodies(&self) -> &Bodies {
        &self.bodies
    }
    pub fn links(&self) -> &JumpLinks {
        &self.links
    }
    pub fn tunnel(&self) -> Option<&Tunnel> {
        self.tunnel.as_ref()
    }

    /// The fastest way from `from` to `to` within one system, by `drive`:
    /// direct, around blocking bodies through their orbital markers, or a
    /// spline jump on one body. `None` for an unknown place or across
    /// systems ([`path`](Self::path) crosses them).
    pub fn leg(&self, from: &Guid, to: &Guid, drive: &Quantum) -> Option<Route> {
        let (a, b) = (self.waypoints.get(from)?, self.waypoints.get(to)?);
        if a.system.is_none() || a.system != b.system {
            return None;
        }
        if from == to {
            return Some(Route::default());
        }
        // Two places on one body: a spline jump over the chord.
        if let (Some(x), Some(y), Some(p), Some(q)) = (a.anchor, b.anchor, a.local, b.local)
            && x == y
            && self.bodies.get(&x).is_some()
        {
            let metres = dist(p, q);
            let trip = drive
                .spline_trip(metres)
                .unwrap_or_else(|| drive.trip(metres));
            return Some(Route {
                hops: vec![Hop {
                    from: Stop::Place(*from),
                    to: Stop::Place(*to),
                    kind: HopKind::Spline,
                    metres,
                    seconds: spool(drive) + trip.travel,
                    cooldown: trip.cooldown,
                    fuel: drive.fuel(metres),
                    estimate: true,
                }],
            });
        }
        // A place and its own anchor: exact through the authored offset.
        if a.anchor == Some(*to) || b.anchor == Some(*from) {
            let d = self.waypoints.distance(from, to)?;
            let metres = (d - f64::from(b.quantum.arrival_radius)).max(0.0);
            return Some(Route {
                hops: vec![quantum_hop(
                    drive,
                    Stop::Place(*from),
                    Stop::Place(*to),
                    metres,
                    false,
                )],
            });
        }
        self.detour(self.point(a), self.point(b), drive)
    }

    /// The fastest route from `from` to `to`, by `drive`, through the jump
    /// links `usable` accepts (`|_| true` for all of them). Within one system
    /// this is [`leg`](Self::leg).
    ///
    /// `usable` is the caller's choice of links, not the in-game opening of a
    /// jump point (any jump drive opens one): the data cannot say whether a
    /// link exists right now (the Stanton–Nyx link is temporary), and a
    /// player may want to avoid one. Opening the jump point (alignment and
    /// tuning) is not in the tunnel hop's time yet: the data has the rates,
    /// not how they combine into seconds.
    ///
    /// `None` when no route uses only usable links, or when it must cross
    /// systems and `drive` has no jump drive or the tunnel record is missing.
    pub fn path(
        &self,
        from: &Guid,
        to: &Guid,
        drive: &Quantum,
        usable: impl Fn(&JumpLink) -> bool,
    ) -> Option<Route> {
        let (a, b) = (self.waypoints.get(from)?, self.waypoints.get(to)?);
        if a.system == b.system {
            return self.leg(from, to, drive);
        }
        let speed = drive.tunnel_speed.filter(|s| *s > 0.0)?;
        let tunnel = self.tunnel?;
        let links: Vec<&JumpLink> = self.links.values().filter(|l| usable(l)).collect();
        // Nodes: the two ends and every usable jump point.
        let mut nodes: Vec<Guid> = vec![*from, *to];
        for l in &links {
            for g in [l.entry, l.exit] {
                if !nodes.contains(&g) && self.waypoints.get(&g).is_some() {
                    nodes.push(g);
                }
            }
        }
        let tunnel_hop = |l: &JumpLink| Hop {
            from: Stop::Place(l.entry),
            to: Stop::Place(l.exit),
            kind: HopKind::Tunnel,
            metres: tunnel.total(),
            seconds: tunnel.total() / speed,
            cooldown: 0.0,
            fuel: 0.0,
            estimate: true,
        };
        let mut legs: HashMap<(usize, usize), Option<Route>> = HashMap::new();
        let edge = |i: usize, j: usize, legs: &mut HashMap<(usize, usize), Option<Route>>| {
            if let Some(l) = links
                .iter()
                .find(|l| l.entry == nodes[i] && l.exit == nodes[j])
            {
                return Some(Route {
                    hops: vec![tunnel_hop(l)],
                });
            }
            legs.entry((i, j))
                .or_insert_with(|| self.leg(&nodes[i], &nodes[j], drive))
                .clone()
        };
        let best = dijkstra(nodes.len(), 0, 1, |i, j| {
            edge(i, j, &mut legs).map(|r| r.seconds())
        })?;
        let mut route = Route::default();
        for w in best.windows(2) {
            route.extend(edge(w[0], w[1], &mut legs)?);
        }
        Some(route)
    }

    fn point(&self, w: &Waypoint) -> Point {
        Point {
            stop: Stop::Place(w.guid),
            pos: w.position,
            arrival: f64::from(w.quantum.arrival_radius),
            exempt: [Some(w.guid), w.anchor],
            estimate: w.anchor.is_some(),
        }
    }

    /// The fastest hops from `a` to `b` in one system, through any orbital
    /// markers needed to clear the bodies in between.
    fn detour(&self, a: Point, b: Point, drive: &Quantum) -> Option<Route> {
        let system = self
            .waypoints
            .get(&match a.stop {
                Stop::Place(g) => g,
                Stop::Marker { body, .. } => body,
            })?
            .system?;
        let bodies: Vec<&Body> = self.bodies.in_system(&system).collect();
        let mut points = vec![a, b];
        for body in &bodies {
            let Some(markers) = body.orbital_markers() else {
                continue;
            };
            for (i, m) in markers.iter().enumerate() {
                points.push(Point {
                    stop: Stop::Marker {
                        body: body.guid,
                        number: i as u8 + 1,
                    },
                    pos: add(body.position, *m),
                    arrival: 0.0,
                    exempt: [None, None],
                    estimate: true,
                });
            }
        }
        let clear = |p: &Point, q: &Point| {
            bodies.iter().all(|body| {
                let skip =
                    p.exempt.contains(&Some(body.guid)) || q.exempt.contains(&Some(body.guid));
                let r = body
                    .quantum
                    .map_or(0.0, |t| f64::from(t.obstruction_radius));
                skip || r <= 0.0 || segment_distance(p.pos, q.pos, body.position) >= r
            })
        };
        let hop = |i: usize, j: usize| {
            let (p, q) = (&points[i], &points[j]);
            let metres = (dist(p.pos, q.pos) - q.arrival).max(0.0);
            quantum_hop(drive, p.stop, q.stop, metres, p.estimate || q.estimate)
        };
        let best = dijkstra(points.len(), 0, 1, |i, j| {
            if !clear(&points[i], &points[j]) {
                return None;
            }
            let h = hop(i, j);
            // A hop into a marker is followed by another: wait its cooldown.
            Some(h.seconds + if j == 1 { 0.0 } else { h.cooldown })
        })?;
        Some(Route {
            hops: best.windows(2).map(|w| hop(w[0], w[1])).collect(),
        })
    }
}

fn spool(drive: &Quantum) -> f64 {
    f64::from(drive.travel.spool_up_time)
}

fn quantum_hop(drive: &Quantum, from: Stop, to: Stop, metres: f64, estimate: bool) -> Hop {
    let trip = drive.trip(metres);
    Hop {
        from,
        to,
        kind: HopKind::Quantum,
        metres,
        seconds: spool(drive) + trip.travel,
        cooldown: trip.cooldown,
        fuel: drive.fuel(metres),
        estimate,
    }
}

/// Shortest path from `start` to `goal` over `n` nodes with edge costs
/// `cost(i, j)` (`None` for no edge), as node indices. Dense O(n²): the
/// graphs here have under a hundred nodes.
fn dijkstra(
    n: usize,
    start: usize,
    goal: usize,
    mut cost: impl FnMut(usize, usize) -> Option<f64>,
) -> Option<Vec<usize>> {
    let mut best = vec![f64::INFINITY; n];
    let mut prev = vec![usize::MAX; n];
    let mut done = vec![false; n];
    best[start] = 0.0;
    loop {
        let u = (0..n)
            .filter(|&i| !done[i] && best[i].is_finite())
            .min_by(|&i, &j| best[i].total_cmp(&best[j]))?;
        if u == goal {
            break;
        }
        done[u] = true;
        for v in (0..n).filter(|&v| !done[v] && v != u) {
            if let Some(c) = cost(u, v)
                && best[u] + c < best[v]
            {
                best[v] = best[u] + c;
                prev[v] = u;
            }
        }
    }
    let mut path = vec![goal];
    while let Some(&p) = path.last().map(|&i| &prev[i])
        && p != usize::MAX
    {
        path.push(p);
    }
    path.reverse();
    Some(path)
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = sub(a, b);
    dot(d, d).sqrt()
}

/// Distance from `c` to the segment `p`–`q`.
fn segment_distance(p: [f64; 3], q: [f64; 3], c: [f64; 3]) -> f64 {
    let d = sub(q, p);
    let len2 = dot(d, d);
    let t = if len2 > 0.0 {
        (dot(sub(c, p), d) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    dist(add(p, [d[0] * t, d[1] * t, d[2] * t]), c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_distance_clamps_to_the_ends() {
        let (p, q) = ([0.0, 0.0, 0.0], [10.0, 0.0, 0.0]);
        assert_eq!(segment_distance(p, q, [5.0, 3.0, 0.0]), 3.0);
        assert_eq!(segment_distance(p, q, [-4.0, 3.0, 0.0]), 5.0);
        assert_eq!(segment_distance(p, p, [0.0, 2.0, 0.0]), 2.0);
    }

    #[test]
    fn dijkstra_takes_the_cheaper_detour() {
        // 0 -> 1 directly costs 10; 0 -> 2 -> 1 costs 3 + 3.
        let cost = |i: usize, j: usize| match (i.min(j), i.max(j)) {
            (0, 1) => Some(10.0),
            (0, 2) | (1, 2) => Some(3.0),
            _ => None,
        };
        assert_eq!(dijkstra(3, 0, 1, cost), Some(vec![0, 2, 1]));
        assert_eq!(dijkstra(3, 0, 1, |_, _| None), None);
    }

    #[test]
    fn route_waits_cooldowns_between_quantum_hops_only() {
        let hop = |kind, seconds, cooldown| Hop {
            from: Stop::Place(Guid::from_bytes([1; 16])),
            to: Stop::Place(Guid::from_bytes([2; 16])),
            kind,
            metres: 1.0,
            seconds,
            cooldown,
            fuel: 0.0,
            estimate: false,
        };
        let route = Route {
            hops: vec![
                hop(HopKind::Quantum, 10.0, 4.0),
                hop(HopKind::Quantum, 20.0, 5.0),
                hop(HopKind::Tunnel, 100.0, 0.0),
                hop(HopKind::Quantum, 30.0, 6.0),
            ],
        };
        // 160 s of hops, one 4 s wait (the 5 s cooldown precedes the tunnel).
        assert_eq!(route.seconds(), 164.0);
        assert_eq!(route.metres(), 4.0);
    }

    #[test]
    fn serde_round_trip() {
        let travel = Travel::default();
        let back: Travel = serde_json::from_str(&serde_json::to_string(&travel).unwrap()).unwrap();
        assert!(back.tunnel().is_none() && back.waypoints().is_empty());
    }
}
