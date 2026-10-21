//! A vehicle's interior: the static geometry (walls, floors, ceilings,
//! fixtures) and the entities (lockers, weapon racks, beds, doors, screens)
//! its object containers place, in the ship frame.
//!
//! The vehicle record lists its containers
//! (`VehicleComponentParams.objectContainers`: a `.socpak` and its offset
//! in the ship); each `.soc` in a socpak places its brushes through an
//! `IncludedObjects` chunk ([`sc_extract::IncludedObjects`]) and its
//! entities as `<Entity>` nodes, each naming its class by
//! `EntityClassGUID`. Nested containers are not read (none of the listed
//! ships checked uses one). Interiors are no damage parts: they are drawn,
//! not hit.
//!
//! A container's offset is taken in the ship frame. Its `boneName` names a
//! hull node that, in 311 of the 341 listed-ship containers with both, sits
//! exactly at that offset (most others within 1.5 cm); offset × bone would
//! put the Mk V's front interior 30 m ahead of its nose. Where the two
//! differ (the Constellation `_GS` hulls, by about 3.5 m) which one the
//! engine uses is not verified.

use sc_extract::object_container::{self, XmlNode};
use sc_extract::{AssetSource, Guid};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::{Transform, Vehicle, mul};

/// One object container a vehicle places
/// (`VehicleComponentParams.objectContainers`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectContainer {
    /// `fileName`: the `.socpak`.
    pub file: String,
    /// `Offset`: position, m, and `Ang3` rotation, degrees, in the ship
    /// frame.
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    /// `boneName`: the hull node it belongs to (usually at the offset; not
    /// used for placement).
    pub bone: Option<String>,
}

impl ObjectContainer {
    /// Its offset as a transform (rotation `Rz · Ry · Rx`, then the position).
    pub fn transform(&self) -> Transform {
        let [x, y, z] = self.rotation.map(f32::to_radians);
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();
        let [px, py, pz] = self.position;
        [
            [cy * cz, sx * sy * cz - cx * sz, cx * sy * cz + sx * sz, px],
            [cy * sz, sx * sy * sz + cx * cz, cx * sy * sz - sx * cz, py],
            [-sy, sx * cy, cx * cy, pz],
        ]
    }
}

/// The static geometry of a vehicle's object containers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Interior {
    pub brushes: Vec<Brush>,
    /// Entities with a class (`EntityClassGUID`); engine entities without
    /// one (lights, areas) are left out.
    #[serde(default)]
    pub entities: Vec<PlacedEntity>,
    /// Containers that could not be read (path, reason).
    pub unread: Vec<(String, String)>,
}

/// One placed geometry file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brush {
    /// Index into [`Vehicle::object_containers`].
    pub container: usize,
    /// `.cgf` / `.cga` path.
    pub geometry: String,
    /// Ship frame.
    pub transform: Transform,
    /// The container's tint palette for it, as a path.
    pub palette: Option<String>,
}

/// One entity a container places.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedEntity {
    /// Index into [`Vehicle::object_containers`].
    pub container: usize,
    /// `EntityClassGUID`: its `EntityClassDefinition` record.
    pub class: Guid,
    /// `Name`: its instance name in the container.
    pub name: String,
    /// Ship frame: the container's offset, then the entity's `Pos` and
    /// `Rotate` (a quaternion w, x, y, z). An entity attached to a parent
    /// entity is placed as if it had none.
    pub transform: Transform,
}

impl Interior {
    /// Read every brush and entity `vehicle`'s object containers place.
    pub fn read(assets: &AssetSource, vehicle: &Vehicle) -> Self {
        let mut out = Self::default();
        for (index, oc) in vehicle.object_containers.iter().enumerate() {
            let base = oc.transform();
            let bytes = match assets.read_data(&oc.file) {
                Ok(b) => b,
                Err(error) => {
                    warn!(file = oc.file, %error, "object container unreadable");
                    out.unread.push((oc.file.clone(), error.to_string()));
                    continue;
                }
            };
            let mut pak = match object_container::Socpak::open(bytes) {
                Ok(p) => p,
                Err(error) => {
                    out.unread.push((oc.file.clone(), error.to_string()));
                    continue;
                }
            };
            let before = out.brushes.len();
            for i in 0..pak.len() {
                let Some(name) = pak.name(i) else { continue };
                if !name.to_ascii_lowercase().ends_with(".soc") {
                    continue;
                }
                let soc = match pak.read(i) {
                    Ok(soc) => soc,
                    Err(error) => {
                        out.unread
                            .push((format!("{}:{name}", oc.file), error.to_string()));
                        continue;
                    }
                };
                match object_container::decode(&soc) {
                    Ok(Some(root)) => {
                        out.entities.extend(
                            root.find_all("Entity")
                                .filter_map(|e| placed(e, index, &base)),
                        );
                    }
                    Ok(None) => {}
                    Err(error) => {
                        out.unread
                            .push((format!("{}:{name}", oc.file), error.to_string()));
                    }
                }
                let sets = match object_container::included_objects(&soc) {
                    Ok(s) => s,
                    Err(error) => {
                        out.unread
                            .push((format!("{}:{name}", oc.file), error.to_string()));
                        continue;
                    }
                };
                for set in sets {
                    for o in &set.objects {
                        let Some(geometry) = set.geometry.get(o.geometry) else {
                            continue;
                        };
                        out.brushes.push(Brush {
                            container: index,
                            geometry: geometry.clone(),
                            transform: mul(&base, &o.transform),
                            palette: o.palette.and_then(|p| set.palettes.get(p)).cloned(),
                        });
                    }
                }
            }
            debug!(
                file = oc.file,
                brushes = out.brushes.len() - before,
                "object container"
            );
        }
        out
    }
}

/// An `<Entity>` node with a class, placed in the ship frame by `base`.
fn placed(e: &XmlNode, container: usize, base: &Transform) -> Option<PlacedEntity> {
    let class = e
        .attr("EntityClassGUID")?
        .trim()
        .trim_matches(['{', '}'])
        .parse::<Guid>()
        .ok()
        .filter(|g| *g != Guid::default())?;
    let numbers = |raw: Option<&str>| -> Vec<f32> {
        raw.unwrap_or("")
            .split(',')
            .filter_map(|v| v.trim().parse().ok())
            .collect()
    };
    let p = numbers(e.attr("Pos"));
    let q = numbers(e.attr("Rotate"));
    let pos = if p.len() == 3 {
        [p[0], p[1], p[2]]
    } else {
        [0.0; 3]
    };
    let (w, x, y, z) = if q.len() == 4 {
        (q[0], q[1], q[2], q[3])
    } else {
        (1.0, 0.0, 0.0, 0.0)
    };
    let local = [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - w * z),
            2.0 * (x * z + w * y),
            pos[0],
        ],
        [
            2.0 * (x * y + w * z),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - w * x),
            pos[1],
        ],
        [
            2.0 * (x * z - w * y),
            2.0 * (y * z + w * x),
            1.0 - 2.0 * (x * x + y * y),
            pos[2],
        ],
    ];
    Some(PlacedEntity {
        container,
        class,
        name: e.attr("Name").unwrap_or_default().to_string(),
        transform: mul(base, &local),
    })
}
