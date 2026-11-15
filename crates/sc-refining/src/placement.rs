//! Where refineries sit: the object containers (socpaks) whose `entdata`
//! places an entity of a refinery class, and the star-map places those
//! belong to.
//!
//! Where a socpak belongs comes from [`ObjectContainers::places_of`]: the
//! nearest named place up its containment chain (for a refinery, the station
//! it is part of). A socpak no placement includes belongs nowhere; such
//! placements are kept, with no places.
//!
//! [`RefineryPlacements::cook`] needs a **live** p4k; the join
//! ([`RefineryPlacements::compose`]) is unit-tested.

use std::collections::{HashMap, HashSet};

use sc_extract::object_container::XmlNode;
use sc_extract::{AssetSource, Guid, RecordCollection, Result};
use sc_locations::{ObjectContainers, normalize_socpak_path};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::Refineries;

/// One socpak's placements of a refinery class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefineryPlacement {
    /// The socpak (normalized path) that places it.
    pub socpak: String,
    /// How many entities of the class the socpak places.
    pub count: usize,
    /// The star-map places (`StarMapObject` GUIDs) the socpak belongs to,
    /// each with how many placement paths reach it. Empty when none does.
    pub places: Vec<(Guid, usize)>,
}

/// Every refinery class's placements, keyed by its `EntityClassDefinition`
/// GUID, with the places that have a refinery.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefineryPlacements {
    by_refinery: HashMap<Guid, Vec<RefineryPlacement>>,
    places: HashSet<Guid>,
}

impl RefineryPlacements {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Walk every object container in the p4k for entities of a refinery
    /// class and join each socpak to its places.
    pub fn cook(assets: &AssetSource, refineries: &Refineries) -> Result<Self> {
        let roots: Vec<String> = assets
            .find(|n| n.to_ascii_lowercase().ends_with(".socpak"))
            .map(|e| normalize_socpak_path(e.name))
            .collect();
        let (containers, hits) =
            ObjectContainers::walk(assets, &roots, |_, _, root| placed_in(root, refineries))?;
        let placements = Self::compose(&containers, &hits);
        info!(
            refineries = placements.len(),
            places = placements.places.len(),
            "refinery placements cooked"
        );
        Ok(placements)
    }

    /// Pure join: the refinery classes each socpak (normalized path) places
    /// ⊕ the placement graph ⇒ placements with their places.
    pub fn compose(containers: &ObjectContainers, hits: &HashMap<String, Vec<Guid>>) -> Self {
        let mut out = Self::default();
        for (socpak, classes) in hits {
            let mut counts: HashMap<Guid, usize> = HashMap::new();
            for class in classes {
                *counts.entry(*class).or_default() += 1;
            }
            let places = containers.places_of(socpak);
            out.places.extend(places.iter().map(|(place, _)| *place));
            for (class, count) in counts {
                out.by_refinery
                    .entry(class)
                    .or_default()
                    .push(RefineryPlacement {
                        socpak: socpak.clone(),
                        count,
                        places: places.clone(),
                    });
            }
        }
        // A stable order whatever order the socpaks were read in.
        for list in out.by_refinery.values_mut() {
            list.sort_by(|a, b| a.socpak.cmp(&b.socpak));
        }
        out
    }

    /// The star-map places (`StarMapObject` GUIDs) that have a refinery.
    pub fn places(&self) -> impl Iterator<Item = &Guid> + '_ {
        self.places.iter()
    }

    /// Whether a star-map place has a refinery.
    pub fn has_refinery(&self, place: &Guid) -> bool {
        self.places.contains(place)
    }
}

impl RecordCollection for RefineryPlacements {
    type Item = Vec<RefineryPlacement>;

    fn get(&self, guid: &Guid) -> Option<&Vec<RefineryPlacement>> {
        self.by_refinery.get(guid)
    }

    fn len(&self) -> usize {
        self.by_refinery.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Vec<RefineryPlacement>)> + '_ {
        self.by_refinery.iter()
    }
}

/// The refinery classes a socpak member's entities are of.
fn placed_in(root: &XmlNode, refineries: &Refineries) -> Vec<Guid> {
    root.find_all("Entity")
        .filter_map(|e| e.attr("EntityClassGUID")?.parse::<Guid>().ok())
        .filter(|class| refineries.contains(class))
        .collect()
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

    const STATION: &str = "11111111-1111-1111-1111-111111111111";
    const REFINERY: &str = "2d213d33-8bc1-4828-98e3-e464b1ae6ce6";

    /// A refinery deck inside a station reaches the station; one in a
    /// module nothing includes is kept with no places.
    #[test]
    fn compose_joins_socpaks_to_places() {
        let g = graph(serde_json::json!([
            placement(1, "sys.socpak", "station.socpak", Some(STATION)),
            placement(2, "station.socpak", "refinery_deck.socpak", None),
        ]));
        let guid = |s: &str| s.parse::<Guid>().unwrap();
        let hits = HashMap::from([
            ("refinery_deck.socpak".to_string(), vec![guid(REFINERY)]),
            (
                "loose_module.socpak".to_string(),
                vec![guid(REFINERY), guid(REFINERY)],
            ),
        ]);
        let p = RefineryPlacements::compose(&g, &hits);
        let list = p.get(&guid(REFINERY)).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].socpak, "loose_module.socpak");
        assert_eq!(list[0].count, 2);
        assert!(list[0].places.is_empty());
        assert_eq!(list[1].places, [(guid(STATION), 1)]);
        assert!(p.has_refinery(&guid(STATION)));
        assert_eq!(p.places().count(), 1);
    }
}
