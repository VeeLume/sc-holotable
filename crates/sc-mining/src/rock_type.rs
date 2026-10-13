//! Rock types — the `HarvestablePreset` records that spawn mineable rocks.

use std::collections::HashMap;

use sc_extract::generated::{DataForgeComponentParamsPtr, SGeometryNodeParams};
use sc_extract::{DataPools, Datacore, Guid, Handle, RecordCollection};
use sc_tags::Tags;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// One `HarvestablePreset` whose entity is a mineable rock: what a spawned
/// rock of this kind can be.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RockType {
    /// The `HarvestablePreset` GUID (what sc-gathering's providers list).
    pub guid: Guid,
    /// `entityClass`: the rock's `EntityClassDefinition`.
    pub entity: Guid,
    /// `MineableParams.globalParams`: the rock's [`Ruleset`](crate::Ruleset).
    pub ruleset: Option<Guid>,
    /// `MineableParams.composition`: the rock's [`Composition`](crate::Composition).
    pub composition: Option<Guid>,
    /// `MineableParams.filledFactor`: the share of the volume that has mass
    /// (1 on every 4.10 rock).
    pub filled_factor: f32,
    /// `transformParams.minScale` / `maxScale`: the spawn scale is uniform in
    /// between (client code), so volume goes with its cube.
    pub min_scale: f32,
    pub max_scale: f32,
    /// The meshes the rock can spawn with.
    pub geometries: Vec<Geometry>,
    /// `BreakableComponentParams.numPiecesOverride` (0: the mesh decides).
    pub pieces_override: i32,
}

impl RockType {
    /// The meshes a provider entry spawns this rock with, each equally likely,
    /// given the entry's geometry tags (sc-gathering's
    /// `GatherableElement::geometry_tags`).
    ///
    /// Client code (LIVE 4.10): each listed tag stands for the geometry node
    /// whose `Tags` contain the tag's name, or else for its child tags that
    /// have one; the spawner picks one candidate uniformly; with none it uses
    /// the root geometry. So an "Asteroid" entry (children Asteroid1..3) gives
    /// asteroid_01..03 a third each, and a "Desert" entry on a surface rock
    /// type its root mesh. The engine joins tags to nodes by name, a string
    /// match there is no typed alternative to; a listed tag that matches
    /// nothing is warned about.
    pub fn geometries_for(&self, tags: &Tags, geometry_tags: &[Guid]) -> Vec<&Geometry> {
        fn add<'a>(g: &'a Geometry, out: &mut Vec<&'a Geometry>) {
            if !out.iter().any(|o| std::ptr::eq(*o, g)) {
                out.push(g);
            }
        }
        let mut out: Vec<&Geometry> = Vec::new();
        for tag in geometry_tags {
            let Some(node) = tags.get(tag) else {
                continue;
            };
            if let Some(g) = self.matching(&node.name) {
                add(g, &mut out);
                continue;
            }
            let before = out.len();
            for child in node.children.iter().filter_map(|c| tags.get(c)) {
                if let Some(g) = self.matching(&child.name) {
                    add(g, &mut out);
                }
            }
            if out.len() == before {
                debug!(guid = %self.guid, tag = node.name, "geometry tag matches no mesh: default mesh");
            }
        }
        if out.is_empty() {
            match self.geometries.first() {
                Some(root) => out.push(root),
                None => warn!(guid = %self.guid, "rock type without geometry"),
            }
        }
        out
    }

    /// The geometry whose `Tags` share the most whitespace-separated tokens
    /// with `name` (case-insensitive, whole tokens); ties go to the first.
    fn matching(&self, name: &str) -> Option<&Geometry> {
        let wanted: Vec<&str> = name.split_whitespace().collect();
        self.geometries
            .iter()
            .map(|g| {
                let hits = g
                    .tags
                    .split_whitespace()
                    .filter(|t| wanted.iter().any(|w| w.eq_ignore_ascii_case(t)))
                    .count();
                (hits, g)
            })
            .filter(|(hits, _)| *hits > 0)
            .fold(
                None,
                |best: Option<(usize, &Geometry)>, (hits, g)| match best {
                    Some((b, _)) if b >= hits => best,
                    _ => Some((hits, g)),
                },
            )
            .map(|(_, g)| g)
    }
}

/// One mesh a rock type can spawn with: the entity's root geometry (empty
/// `tags`, listed first) or a tagged sub-geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Geometry {
    /// Geometry file, lower-case.
    pub path: String,
    /// The node's `Tags` (e.g. `Asteroid1`).
    pub tags: String,
}

/// Every mineable rock type, keyed by `HarvestablePreset` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RockTypes {
    by_guid: HashMap<Guid, RockType>,
}

impl RockTypes {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let records = &store.records;
        let mut by_guid = HashMap::new();
        for (&guid, handle) in &records.multi_feature.harvestable_preset {
            let Some(preset) = handle.get(pools) else {
                continue;
            };
            let Some(entity) = preset.entity_class else {
                continue;
            };
            let Some(class) = records
                .multi_feature
                .entity_class_definition
                .get(&entity)
                .and_then(|h| h.get(pools))
            else {
                continue;
            };
            let mut mineable = None;
            let mut geometries = Vec::new();
            let mut pieces_override = 0;
            for component in &class.components {
                match component {
                    DataForgeComponentParamsPtr::MineableParams(h) => mineable = h.get(pools),
                    DataForgeComponentParamsPtr::SGeometryResourceParams(h) => {
                        if let Some(node) = h.get(pools).and_then(|g| g.geometry) {
                            collect(pools, node, &mut geometries);
                        }
                    }
                    DataForgeComponentParamsPtr::BreakableComponentParams(h) => {
                        if let Some(b) = h.get(pools) {
                            pieces_override = b.num_pieces_override;
                        }
                    }
                    _ => {}
                }
            }
            let Some(mineable) = mineable else {
                continue;
            };
            let (min_scale, max_scale) = preset
                .transform_params
                .and_then(|h| h.get(pools))
                .map_or((1.0, 1.0), |t| (t.min_scale, t.max_scale));
            let rock_type = RockType {
                guid,
                entity,
                ruleset: mineable.global_params,
                composition: mineable.composition,
                filled_factor: mineable.filled_factor,
                min_scale,
                max_scale,
                geometries,
                pieces_override,
            };
            debug!(%guid, ?rock_type, "rock type");
            by_guid.insert(guid, rock_type);
        }
        Self { by_guid }
    }
}

/// Every geometry file in a node tree, with its node's tags.
fn collect(pools: &DataPools, root: Handle<SGeometryNodeParams>, out: &mut Vec<Geometry>) {
    let mut stack = vec![root];
    while let Some(h) = stack.pop() {
        let Some(node) = h.get(pools) else {
            continue;
        };
        if let Some(path) = node
            .geometry
            .and_then(|g| g.get(pools))
            .and_then(|d| d.geometry)
            .and_then(|g| g.get(pools))
            .map(|g| g.path.to_ascii_lowercase())
            .filter(|p| !p.is_empty())
        {
            let geometry = Geometry {
                path,
                tags: node.tags.clone(),
            };
            if !out.contains(&geometry) {
                out.push(geometry);
            }
        }
        stack.extend(node.sub_geometry.iter().rev().copied());
    }
}

impl RecordCollection for RockTypes {
    type Item = RockType;

    fn get(&self, guid: &Guid) -> Option<&RockType> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &RockType)> + '_ {
        self.by_guid.iter()
    }
}
