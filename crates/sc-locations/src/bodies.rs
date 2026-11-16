//! Planets and moons with their geometry: position, radius, spin and
//! atmosphere, and the orbital markers the game derives from them.
//!
//! The geometry is not in the DataCore. Each body's object container
//! (`objectcontainers/pu/system/<system>/<body>.socpak`) carries it: an
//! `.xml` member holds `planetRadius` (m), `planetRotationSpeed` (hours per
//! turn) and `planetAxis`, and its one `.ale` member embeds `AtmoHeight` (km).
//! The body is joined to its `StarMapObject` through the system container's
//! placement (`objectContainer` + `starmapRecord`), never by a file name.
//!
//! Bodies do not move: the data holds no orbital motion. The spin phase is
//! not in the data either (only the period and axis are), so everything here
//! is in the body's own frame or at its centre.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
//! use sc_locations::{Bodies, Locations, RecordCollection};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let bodies = Bodies::build(&assets, &Locations::build(datacore.records()))?;
//! for b in bodies.values() {
//!     println!("{} r={} m, day {} h, OM {:?} m", b.short_id(), b.radius, b.rotation_period_h, b.om_radius());
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;

use sc_extract::object_container::Socpak;
use sc_extract::{AssetSource, Guid, RecordCollection, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::object_containers::{ObjectContainers, normalize_socpak_path};
use crate::waypoints::system_of;
use crate::{Locations, QuantumTravel};

/// The fixed part of the orbital-marker radius, m. See [`Body::om_radius`].
pub const OM_OFFSET_M: f64 = 10_000.0;

/// One planet or moon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    /// The body's `StarMapObject`.
    pub guid: Guid,
    /// The body's object container, normalized
    /// (`objectcontainers/pu/system/stanton/stanton1a.socpak`).
    pub container: String,
    /// Its system: the star the star map roots it at.
    pub system: Option<Guid>,
    /// Centre, system-global, m.
    pub position: [f64; 3],
    /// `planetRadius`, m.
    pub radius: f64,
    /// `planetRotationSpeed`: hours per turn. 0 on a body that does not turn.
    pub rotation_period_h: f64,
    /// `planetAxis`, the spin axis in the body frame (`0,0,1` on every live
    /// body).
    pub axis: [f64; 3],
    /// `AtmoHeight` from the body's `.ale`, m. `None` without one.
    pub atmosphere_height: Option<f64>,
    /// The star map's quantum radii for the body.
    pub quantum: Option<QuantumTravel>,
}

impl Body {
    /// The container's file stem (`stanton1a`): a stable short id, the one
    /// community tools and sc-nav use. An identifier only, never a join key.
    pub fn short_id(&self) -> &str {
        let file = self.container.rsplit('/').next().unwrap_or(&self.container);
        file.strip_suffix(".socpak").unwrap_or(file)
    }

    /// Distance of the orbital markers from the centre, m:
    /// `sqrt(2) * obstructionRadius + 10 km` ([`OM_OFFSET_M`]).
    ///
    /// The markers are not in the files; the game derives them. This
    /// reproduces Murphy's Navigation Tool on 27 of 30 bodies to within
    /// 1–28 m (the three others changed obstruction since that table). At
    /// this radius two adjacent markers see each other past the obstruction
    /// sphere with about 7 km to spare, which is why routing around a body
    /// through its markers works. `None` without quantum data.
    pub fn om_radius(&self) -> Option<f64> {
        let q = self.quantum?;
        (q.obstruction_radius > 0.0)
            .then(|| std::f64::consts::SQRT_2 * f64::from(q.obstruction_radius) + OM_OFFSET_M)
    }

    /// OM-1 to OM-6 in the body frame, m: ±Z (OM-1 north, OM-2 south), ±Y,
    /// ±X. They turn with the body; placing them in the system frame needs
    /// the spin phase, which the data does not hold.
    pub fn orbital_markers(&self) -> Option<[[f64; 3]; 6]> {
        let r = self.om_radius()?;
        Some([
            [0.0, 0.0, r],
            [0.0, 0.0, -r],
            [0.0, r, 0.0],
            [0.0, -r, 0.0],
            [r, 0.0, 0.0],
            [-r, 0.0, 0.0],
        ])
    }
}

/// Every planet and moon, by `StarMapObject` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bodies {
    by_guid: HashMap<Guid, Body>,
}

/// What a body container's planet node says.
struct Planet {
    radius: f64,
    period_h: f64,
    axis: [f64; 3],
}

impl Bodies {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the systems' object containers (and the bodies' one level down)
    /// and join each body container to its `StarMapObject` in `star_map`.
    /// Empty for a snapshot-backed [`AssetSource`].
    pub fn build(assets: &AssetSource, star_map: &Locations) -> Result<Self> {
        let (graph, found) = ObjectContainers::systems_with(assets, 1, |_, member, root| {
            if !member.to_ascii_lowercase().ends_with(".xml") {
                return Vec::new();
            }
            root.descendants()
                .filter_map(|n| {
                    Some(Planet {
                        radius: n.attr("planetRadius")?.trim().parse().ok()?,
                        period_h: n.attr("planetRotationSpeed")?.trim().parse().ok()?,
                        axis: vec3(n.attr("planetAxis")?)?,
                    })
                })
                .take(1)
                .collect()
        })?;
        let atmospheres = atmospheres(assets, found.keys());

        let mut out = Self::new();
        for (socpak, planets) in found {
            let Some(planet) = planets.into_iter().next() else {
                continue;
            };
            let placements: Vec<_> = graph
                .nesting(&socpak)
                .iter()
                .filter(|&&id| graph.get(id).socpak().ends_with("system.socpak"))
                .filter_map(|&id| Some((graph.get(id).starmap_guid()?, id)))
                .collect();
            if placements.is_empty() {
                warn!(%socpak, "body container with no star-map placement");
            }
            for (guid, id) in placements {
                let Some(position) = graph.global_position(id) else {
                    warn!(%socpak, %guid, "body placement without a position");
                    continue;
                };
                out.insert(Body {
                    guid,
                    container: socpak.clone(),
                    system: system_of(star_map, &guid),
                    position,
                    radius: planet.radius,
                    rotation_period_h: planet.period_h,
                    axis: planet.axis,
                    atmosphere_height: atmospheres.get(&socpak).copied(),
                    quantum: star_map.get(&guid).and_then(|l| l.quantum),
                });
            }
        }
        info!(bodies = out.len(), "built bodies");
        Ok(out)
    }

    /// Insert or replace a body.
    pub fn insert(&mut self, body: Body) {
        self.by_guid.insert(body.guid, body);
    }

    /// The bodies in `system`.
    pub fn in_system<'a>(&'a self, system: &'a Guid) -> impl Iterator<Item = &'a Body> + 'a {
        self.by_guid
            .values()
            .filter(move |b| b.system.as_ref() == Some(system))
    }
}

impl RecordCollection for Bodies {
    type Item = Body;

    fn get(&self, guid: &Guid) -> Option<&Body> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Body)> + '_ {
        self.by_guid.iter()
    }
}

/// `AtmoHeight` (km → m) from the one `.ale` member of each container in
/// `containers` (normalized paths). A container without exactly one `.ale`,
/// or one without the key, has none.
fn atmospheres<'a>(
    assets: &AssetSource,
    containers: impl Iterator<Item = &'a String>,
) -> HashMap<String, f64> {
    let wanted: std::collections::HashSet<&String> = containers.collect();
    let mut out = HashMap::new();
    let is_socpak = |n: &str| {
        n.len() > 7
            && n.is_char_boundary(n.len() - 7)
            && n[n.len() - 7..].eq_ignore_ascii_case(".socpak")
    };
    for entry in assets.find(is_socpak) {
        let socpak = normalize_socpak_path(entry.name);
        if !wanted.contains(&socpak) {
            continue;
        }
        let Some(height) = assets
            .read_entry(&entry)
            .ok()
            .and_then(|bytes| Socpak::open(bytes).ok())
            .and_then(|mut pak| {
                let ale: Vec<usize> = (0..pak.len())
                    .filter(|&m| {
                        pak.name(m)
                            .is_some_and(|n| n.to_ascii_lowercase().ends_with(".ale"))
                    })
                    .collect();
                match ale.as_slice() {
                    [m] => pak.read(*m).ok(),
                    _ => None,
                }
            })
            .and_then(|bytes| atmo_height_km(&bytes))
        else {
            continue;
        };
        out.insert(socpak, height * 1000.0);
    }
    out
}

/// The `"AtmoHeight"` number in an `.ale` chunk's embedded JSON, km.
fn atmo_height_km(bytes: &[u8]) -> Option<f64> {
    const KEY: &[u8] = b"\"AtmoHeight\"";
    let at = bytes.windows(KEY.len()).position(|w| w == KEY)? + KEY.len();
    let rest = String::from_utf8_lossy(bytes.get(at..(at + 48).min(bytes.len()))?);
    let value = rest.trim_start_matches(|c: char| c == ':' || c.is_whitespace());
    let end = value
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | 'e' | 'E' | '+')))
        .unwrap_or(value.len());
    value[..end].parse().ok()
}

fn vec3(raw: &str) -> Option<[f64; 3]> {
    let mut it = raw.split(',').map(|v| v.trim().parse::<f64>());
    let v = [it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?];
    it.next().is_none().then_some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hurston() -> Body {
        Body {
            guid: Guid::from_bytes([1; 16]),
            container: "objectcontainers/pu/system/stanton/stanton1.socpak".into(),
            system: None,
            position: [0.0; 3],
            radius: 1_000_000.0,
            rotation_period_h: 2.48,
            axis: [0.0, 0.0, 1.0],
            atmosphere_height: Some(65_000.0),
            quantum: Some(QuantumTravel {
                arrival_radius: 1_050_000.0,
                obstruction_radius: 1_008_900.0,
                adoption_radius: 128_354_900.0,
                arrival_point_detection_offset: 10_000.0,
                sub_point_radius_multiplier: 1.5,
            }),
        }
    }

    #[test]
    fn short_id_is_the_container_stem() {
        assert_eq!(hurston().short_id(), "stanton1");
    }

    /// Murphy's Navigation Tool gives Hurston's OM radius as 1,436.814 km.
    #[test]
    fn om_radius_reproduces_the_reference() {
        let om = hurston().om_radius().unwrap();
        assert!((om - 1_436_814.0).abs() < 30.0, "{om}");
        let markers = hurston().orbital_markers().unwrap();
        assert_eq!(markers[0], [0.0, 0.0, om]);
        assert_eq!(markers[5], [-om, 0.0, 0.0]);
    }

    #[test]
    fn atmo_height_reads_the_json_number() {
        let ale = b"\x00\x01{ \"AtmoHeight\" : 38.0,\n \"Other\": 1 }";
        assert_eq!(atmo_height_km(ale), Some(38.0));
        assert_eq!(atmo_height_km(b"no key here"), None);
    }

    #[test]
    fn serde_round_trip() {
        let mut bodies = Bodies::new();
        bodies.insert(hurston());
        let json = serde_json::to_string(&bodies).unwrap();
        let back: Bodies = serde_json::from_str(&json).unwrap();
        assert_eq!(back.get(&hurston().guid), Some(&hurston()));
    }
}
