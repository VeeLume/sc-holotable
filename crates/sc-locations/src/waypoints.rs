//! Quantum-travel destinations with their positions: planets, moons,
//! Lagrange points, jump points and stations, per system.
//!
//! The kinds are the star map's own and do not follow the game's labels:
//! on LIVE 4.10 the Lagrange points (`ARC L1`) are `Asteroid`, the
//! Stanton–Pyro and Pyro–Nyx gateways `Anomaly`, the Lagrange rest stops
//! `Manmade_VisibleOnInteraction`; depth 1 also reaches outposts and landing
//! zones. Nyx's planets are placeholders without placements.
//!
//! Positions come from the system object containers
//! ([`ObjectContainers::systems`]), joined to the star map by typed GUID
//! ([`Universe`]). A `*system.socpak` placement's `Pos` is system-global;
//! one level down (a station on a planet) adds its offset with the body's
//! rotation ignored, an error of at most a body radius on routes of tens of
//! Gm ([`Place::global_position`](crate::Place::global_position)). Bodies
//! do not move: the data holds no orbital motion.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
//! use sc_locations::{Locations, ObjectContainers, RecordCollection, Universe, Waypoints};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let universe = Universe::join(
//!     Locations::build(datacore.records()),
//!     ObjectContainers::systems(&assets, 1)?,
//! );
//! let waypoints = Waypoints::build(&universe);
//! for (_, w) in waypoints.iter() {
//!     println!("{:?} {:?} {:?}", w.display_name(&data.locale), w.kind, w.position);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;

use sc_extract::{Guid, LocaleKey, LocaleMap, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::{LocationKind, QuantumTravel, Universe};

/// A star-map object quantum travel can target, where it is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    /// The `StarMapObject` record.
    pub guid: Guid,
    pub name_key: Option<LocaleKey>,
    #[serde(with = "crate::enum_serde::location_kind")]
    pub kind: LocationKind,
    /// Its system: the star the star map roots it at (Stanton, Pyro, Nyx).
    pub system: Option<Guid>,
    /// System-global position, m. Exact for a system-level place (a body, a
    /// Lagrange or jump point); for one placed on a body, the body's
    /// rotation is ignored (see the module docs).
    pub position: [f64; 3],
    /// The system-level place this one is authored under (the body an
    /// outpost or station sits on, the Lagrange point a rest stop is at).
    /// `None` for a system-level place.
    #[serde(default)]
    pub anchor: Option<Guid>,
    /// The offset from `anchor` in the anchor's own frame, m, as authored:
    /// exact, so two places under one anchor are exactly as far apart as
    /// their offsets say ([`Waypoints::distance`] uses it).
    #[serde(default)]
    pub local: Option<[f64; 3]>,
    pub quantum: QuantumTravel,
}

/// The system of a star-map place: the star the star map roots it at (the
/// star or solar system itself for those).
pub(crate) fn system_of(star_map: &crate::Locations, guid: &Guid) -> Option<Guid> {
    star_map.ancestors(guid).last().map(|a| a.guid).or_else(|| {
        star_map
            .get(guid)
            .filter(|l| matches!(l.kind, LocationKind::Star | LocationKind::SolarSystem))
            .map(|l| l.guid)
    })
}

impl Waypoint {
    /// The display name through `locale`; `None` without a key or text.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        let name = locale.resolve(self.name_key.as_ref()?)?;
        (!name.is_empty()).then_some(name)
    }
}

/// Every positioned star-map object with quantum travel data
/// (`arrivalRadius` > 0), by `StarMapObject` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Waypoints {
    by_guid: HashMap<Guid, Waypoint>,
}

impl Waypoints {
    pub fn new() -> Self {
        Self::default()
    }

    /// The waypoints `universe` positions: built on
    /// [`ObjectContainers::systems`](crate::ObjectContainers::systems) it
    /// holds the bodies, Lagrange and jump points (depth 0) and the
    /// stations on them (depth 1).
    pub fn build(universe: &Universe) -> Self {
        let star_map = universe.star_map();
        let mut by_guid = HashMap::new();
        for (guid, location) in star_map.iter() {
            let Some(quantum) = location.quantum.filter(|q| q.arrival_radius > 0.0) else {
                continue;
            };
            let Some(position) = universe.by_guid(*guid).and_then(|p| p.global_position()) else {
                continue;
            };
            let (anchor, local) = universe
                .by_guid(*guid)
                .and_then(|p| p.anchor())
                .map_or((None, None), |(a, l)| (Some(a), Some(l)));
            by_guid.insert(
                *guid,
                Waypoint {
                    guid: *guid,
                    name_key: location.name_key.clone(),
                    kind: location.kind.clone(),
                    system: system_of(star_map, guid),
                    position,
                    anchor,
                    local,
                    quantum,
                },
            );
        }
        Self { by_guid }
    }

    /// Centre-to-centre distance between two waypoints, m; `None` across
    /// systems (each has its own frame) or for an unknown GUID.
    ///
    /// Exact between two places under the same anchor and between a place
    /// and its anchor (both from the authored offsets); otherwise from the
    /// system-global positions, off by at most the anchors' radii.
    pub fn distance(&self, a: &Guid, b: &Guid) -> Option<f64> {
        let (a, b) = (self.by_guid.get(a)?, self.by_guid.get(b)?);
        if a.system != b.system {
            return None;
        }
        let (p, q) = match (a.anchor, b.anchor, a.local, b.local) {
            (Some(x), Some(y), Some(p), Some(q)) if x == y => (p, q),
            (_, Some(y), _, Some(q)) if y == a.guid => ([0.0; 3], q),
            (Some(x), _, Some(p), _) if x == b.guid => (p, [0.0; 3]),
            _ => (a.position, b.position),
        };
        let d: f64 = (0..3).map(|i| (p[i] - q[i]).powi(2)).sum();
        Some(d.sqrt())
    }

    /// The quantum jump from `a` to `b`, m: the distance less `b`'s arrival
    /// radius, where travel drops out.
    pub fn travel(&self, a: &Guid, b: &Guid) -> Option<f64> {
        let arrival = f64::from(self.by_guid.get(b)?.quantum.arrival_radius);
        Some((self.distance(a, b)? - arrival).max(0.0))
    }

    /// The waypoints in `system`.
    pub fn in_system<'a>(&'a self, system: &'a Guid) -> impl Iterator<Item = &'a Waypoint> + 'a {
        self.by_guid
            .values()
            .filter(move |w| w.system.as_ref() == Some(system))
    }
}

impl RecordCollection for Waypoints {
    type Item = Waypoint;

    fn get(&self, guid: &Guid) -> Option<&Waypoint> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Waypoint)> + '_ {
        self.by_guid.iter()
    }
}
