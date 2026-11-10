//! Where lootables sit in the world: the object containers (socpaks) that
//! place them, and the star-map places those belong to.
//!
//! A socpak's `entdata` entity places a lootable in one of two ways (LIVE
//! 4.10, `placement_probe`):
//! - its `EntityClassGUID` is a lootable entity (corpses, lootable
//!   carryables, a few placed NPCs);
//! - a `looseSubConfigBase` in it names a harvestable slot config
//!   (`multiConfigRef` / `subConfigRef`): a container (weapon rack, crate,
//!   locker) whose slots spawn loot.
//!
//! Where a socpak belongs comes from [`ObjectContainers::places_of`]: the
//! nearest named place up its containment chain, which for modular content
//! (outposts, caves) is usually the planet or moon, not the outpost. A
//! socpak no placement includes belongs nowhere: on LIVE 4.10 that is event
//! and mission content (Siege of Orison, the Fleet Week platforms, hijacked
//! ships, derelicts spawned for missions) and the underground facility
//! layouts (`loc/mod/ugf`), whose placement was not found. NPC actors
//! mostly spawn rather than being placed. The DCB itself names almost none
//! of this (`where_dig`).
//!
//! [`LootPlacements::cook`] needs a **live** p4k; the join
//! ([`LootPlacements::compose`]) is unit-tested.

use std::collections::HashMap;

use sc_extract::object_container::XmlNode;
use sc_extract::{AssetSource, Guid, RecordCollection, Result};
use sc_locations::{ObjectContainers, normalize_socpak_path};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Lootables;

/// How a [`LootPlacement`] places its lootable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Placed {
    /// An entity of the lootable's own class.
    Entity,
    /// An entity (of class `entity`, when it names one) whose harvestable
    /// slots use the lootable config.
    Container { entity: Option<Guid> },
}

/// One socpak's placements of a lootable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LootPlacement {
    /// The socpak (normalized path).
    pub socpak: String,
    pub placed: Placed,
    /// How many entities of the socpak place it so.
    pub count: usize,
    /// The `StarMapObject`s the socpak belongs to, each with how many times
    /// the socpak is realized there ([`ObjectContainers::places_of`]);
    /// empty when nothing includes it.
    pub places: Vec<(Guid, usize)>,
}

impl LootPlacement {
    /// How many entities place the lootable at `place`: [`count`](Self::count)
    /// times the socpak's realizations there.
    pub fn at(&self, place: &Guid) -> usize {
        self.places
            .iter()
            .find(|(p, _)| p == place)
            .map_or(0, |(_, n)| n * self.count)
    }
}

/// Every lootable's placements, keyed by the lootable's GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LootPlacements {
    by_lootable: HashMap<Guid, Vec<LootPlacement>>,
    /// `StarMapObject` → the lootables placed there.
    by_place: HashMap<Guid, Vec<Guid>>,
}

impl LootPlacements {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Walk every object container in the p4k and record the entities that
    /// place one of `lootables`. Empty for a snapshot-backed source.
    pub fn cook(assets: &AssetSource, lootables: &Lootables) -> Result<Self> {
        let roots: Vec<String> = assets
            .find(|n| n.to_ascii_lowercase().ends_with(".socpak"))
            .map(|e| normalize_socpak_path(e.name))
            .collect();
        let (containers, hits) =
            ObjectContainers::walk(assets, &roots, |_, _, root| placements_in(root, lootables))?;
        let placements = Self::compose(&containers, &hits);
        info!(
            lootables = placements.len(),
            places = placements.by_place.len(),
            "loot placements cooked"
        );
        Ok(placements)
    }

    /// Pure join: what each socpak (normalized path) places ⊕ the placement
    /// graph ⇒ placements with their places.
    pub fn compose(
        containers: &ObjectContainers,
        hits: &HashMap<String, Vec<(Guid, Placed)>>,
    ) -> Self {
        let mut counts: HashMap<(Guid, String, Placed), usize> = HashMap::new();
        for (socpak, list) in hits {
            for &(lootable, placed) in list {
                *counts
                    .entry((lootable, socpak.clone(), placed))
                    .or_default() += 1;
            }
        }
        let mut out = Self::default();
        let mut places_of: HashMap<String, Vec<(Guid, usize)>> = HashMap::new();
        for ((lootable, socpak, placed), count) in counts {
            let places = places_of
                .entry(socpak.clone())
                .or_insert_with(|| containers.places_of(&socpak))
                .clone();
            for (place, _) in &places {
                let at = out.by_place.entry(*place).or_default();
                if !at.contains(&lootable) {
                    at.push(lootable);
                }
            }
            out.by_lootable
                .entry(lootable)
                .or_default()
                .push(LootPlacement {
                    socpak,
                    placed,
                    count,
                    places,
                });
        }
        // A stable order whatever order the socpaks were read in.
        for list in out.by_lootable.values_mut() {
            list.sort_by(|a, b| a.socpak.cmp(&b.socpak).then(a.count.cmp(&b.count)));
        }
        for list in out.by_place.values_mut() {
            list.sort_by_key(Guid::to_string);
        }
        out
    }

    /// The lootables placed at a `StarMapObject`.
    pub fn at(&self, place: &Guid) -> &[Guid] {
        self.by_place.get(place).map_or(&[], Vec::as_slice)
    }
}

impl RecordCollection for LootPlacements {
    type Item = Vec<LootPlacement>;

    fn get(&self, guid: &Guid) -> Option<&Vec<LootPlacement>> {
        self.by_lootable.get(guid)
    }

    fn len(&self) -> usize {
        self.by_lootable.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Vec<LootPlacement>)> + '_ {
        self.by_lootable.iter()
    }
}

/// The lootables a socpak member's entities place.
fn placements_in(root: &XmlNode, lootables: &Lootables) -> Vec<(Guid, Placed)> {
    let mut out = Vec::new();
    for entity in root.find_all("Entity") {
        let class = entity
            .attr("EntityClassGUID")
            .and_then(|v| v.parse::<Guid>().ok());
        if let Some(c) = class
            && lootables.contains(&c)
        {
            out.push((c, Placed::Entity));
        }
        // The entity's own components, not those of entities nested in it
        // (find_all visits those as entities of their own).
        let mut stack: Vec<&XmlNode> = entity.children.iter().collect();
        while let Some(node) = stack.pop() {
            if node.tag == "Entity" {
                continue;
            }
            if node.tag == "looseSubConfigBase" {
                for key in ["multiConfigRef", "subConfigRef"] {
                    if let Some(g) = node.attr(key).and_then(|v| v.parse::<Guid>().ok())
                        && lootables.contains(&g)
                    {
                        out.push((g, Placed::Container { entity: class }));
                    }
                }
            }
            stack.extend(node.children.iter());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(placements: serde_json::Value) -> ObjectContainers {
        serde_json::from_value(serde_json::json!({ "placements": placements })).unwrap()
    }

    fn placement(key: u8, socpak: &str, nests: &str, starmap: Option<&str>) -> serde_json::Value {
        let mut k = [0u8; 16];
        k[0] = key;
        serde_json::json!({
            "key": k,
            "disp": format!("{key:08x}-0000-0000-0000-000000000000"),
            "socpak": socpak,
            "parent": null,
            "nests": nests,
            "starmap": starmap,
            "starmap_guid": starmap,
            "name": "",
            "pos": null,
            "orbit": null,
            "is_leaf": starmap.is_some(),
        })
    }

    const MOON: &str = "11111111-1111-1111-1111-111111111111";
    const RACK: &str = "22222222-2222-2222-2222-222222222222";
    const CORPSE: &str = "33333333-3333-3333-3333-333333333333";

    /// Two racks in an outpost placed twice on a moon: 4 there; a corpse in
    /// a socpak nothing includes belongs nowhere.
    #[test]
    fn compose_counts_entities_times_realizations() {
        let g = graph(serde_json::json!([
            placement(1, "sys.socpak", "moon.socpak", Some(MOON)),
            placement(2, "moon.socpak", "outpost.socpak", None),
            placement(3, "moon.socpak", "outpost.socpak", None),
        ]));
        let guid = |s: &str| s.parse::<Guid>().unwrap();
        let rack = (guid(RACK), Placed::Container { entity: None });
        let hits = HashMap::from([
            ("outpost.socpak".to_string(), vec![rack, rack]),
            (
                "cave.socpak".to_string(),
                vec![(guid(CORPSE), Placed::Entity)],
            ),
        ]);
        let p = LootPlacements::compose(&g, &hits);
        let racks = p.get(&guid(RACK)).unwrap();
        assert_eq!(racks.len(), 1);
        assert_eq!(racks[0].count, 2);
        assert_eq!(racks[0].at(&guid(MOON)), 4);
        assert_eq!(p.at(&guid(MOON)), [guid(RACK)]);
        assert!(p.get(&guid(CORPSE)).unwrap()[0].places.is_empty());
    }
}
