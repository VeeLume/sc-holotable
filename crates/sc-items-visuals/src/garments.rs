//! What worn items cover, and which of their mesh pieces an outfit hides.
//!
//! A [`Garment`] lists the body zones an item occupies, each on a layer. For an
//! [`Outfit`] the game goes through one zone at a time, from the highest layer
//! down: the top layer's pieces are checked, passing ones add their
//! `visible_layers` to a list, and each lower layer stays only if that list
//! names it. A piece whose zone its item does not register is never hidden.
//! Read from the 4.10 client (`CItemPort::RefreshMeshChunkVisibility`).
//!
//! The per-chunk tag conditions are not evaluated, because the client code for
//! them is only partly read: such a chunk reports [`ZoneState::Conditional`]
//! rather than a guess. The view is assumed to be third person, so
//! [`GarmentZone::hide_in_first_person`] is reported but not applied.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_items::Items;
//! use sc_items_visuals::{Garments, ZoneState};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let garments = Garments::build(&datacore, &items);
//!
//! let worn: Vec<_> = garments.guids().take(2).copied().collect();
//! let outfit = garments.outfit(&worn);
//! let hidden = outfit.state(&worn[0], sc_extract::zone_hash("hips_zone")) == ZoneState::Hidden;
//! println!("{hidden}");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap};

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EMeshChunks, SCItemClothingMeshChunkParams, SCItemClothingParams,
};
use sc_extract::{DataPools, Datacore, Guid, RecordCollection, zone_hash};
use sc_items::Items;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// What one worn item covers (`SCItemClothingParams`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Garment {
    /// `Chunks`.
    pub zones: Vec<GarmentZone>,
    /// `HiddenParts`: item ports this item hides outright.
    pub hidden_ports: Vec<String>,
}

/// One body zone an item occupies (`SCItemClothingMeshChunkParams`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GarmentZone {
    #[serde(with = "mesh_chunk_serde")]
    pub zone: EMeshChunks,
    /// [`zone_hash`] of the zone's name: what
    /// [`Submesh::zone`](sc_extract::Submesh::zone) holds.
    pub hash: u32,
    pub layer: i32,
    /// Lower layers that stay visible under this one.
    pub visible_layers: Vec<i32>,
    pub conditions: Vec<ZoneCondition>,
    pub hide_in_first_person: bool,
}

/// One `SCItemClothingVisibilityConditionParams`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneCondition {
    pub ports: Vec<PortCondition>,
}

/// One `SCItemClothingPortVisibilityConditionParams`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortCondition {
    pub ports: Vec<String>,
    pub disable_tags: Vec<Guid>,
    pub skip_tags: Vec<Guid>,
    pub active_tags: Vec<Guid>,
}

/// Whether an item's piece of a zone shows in an [`Outfit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneState {
    Visible,
    Hidden,
    /// Depends on a tag condition that is not evaluated.
    Conditional,
}

/// Every item's [`Garment`], keyed by `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Garments {
    by_guid: HashMap<Guid, Garment>,
}

/// The zone states of a set of worn items, from [`Garments::outfit`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Outfit {
    states: HashMap<(Guid, u32), ZoneState>,
}

impl Outfit {
    /// The state of `item`'s piece of the zone with this hash. A zone the item
    /// does not register is [`ZoneState::Visible`].
    pub fn state(&self, item: &Guid, zone_hash: u32) -> ZoneState {
        self.states
            .get(&(*item, zone_hash))
            .copied()
            .unwrap_or(ZoneState::Visible)
    }

    /// The hashes of `item`'s zones in `state`, in no particular order.
    pub fn zones_in(&self, item: &Guid, state: ZoneState) -> Vec<u32> {
        self.states
            .iter()
            .filter(|((owner, _), s)| owner == item && **s == state)
            .map(|((_, hash), _)| *hash)
            .collect()
    }
}

/// Whether a layer is still reachable under the layers already walked.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reach {
    Yes,
    Maybe,
    No,
}

impl Garments {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            if items.get(&guid).is_none() {
                continue;
            }
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let clothing = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::SCItemClothingParams(h) => h.get(pools),
                _ => None,
            });
            if let Some(clothing) = clothing {
                debug!(%guid, "garment");
                by_guid.insert(guid, garment(clothing, pools));
            }
        }
        Self { by_guid }
    }

    /// The state of every registered zone when `worn` is put on together.
    /// Items without a [`Garment`] are ignored.
    pub fn outfit(&self, worn: &[Guid]) -> Outfit {
        type Layers<'a> = BTreeMap<i32, Vec<(&'a Guid, &'a GarmentZone)>>;
        let mut zones: BTreeMap<u32, Layers> = BTreeMap::new();
        for guid in worn {
            let Some(garment) = self.by_guid.get(guid) else {
                continue;
            };
            // A port keeps one registration per zone: a later chunk replaces.
            let own: HashMap<u32, &GarmentZone> =
                garment.zones.iter().map(|z| (z.hash, z)).collect();
            for (hash, zone) in own {
                zones
                    .entry(hash)
                    .or_default()
                    .entry(zone.layer)
                    .or_default()
                    .push((guid, zone));
            }
        }

        let mut states = HashMap::new();
        for (hash, layers) in zones {
            let (mut certain, mut possible) = (BTreeSet::new(), BTreeSet::new());
            let mut evaluated = false;
            for (layer, entries) in layers.iter().rev() {
                let reach = if !evaluated || certain.contains(layer) {
                    Reach::Yes
                } else if possible.contains(layer) {
                    Reach::Maybe
                } else {
                    Reach::No
                };
                for (guid, zone) in entries {
                    let state = match reach {
                        Reach::No => ZoneState::Hidden,
                        Reach::Yes if zone.conditions.is_empty() => {
                            certain.extend(zone.visible_layers.iter().copied());
                            evaluated = true;
                            ZoneState::Visible
                        }
                        Reach::Yes | Reach::Maybe => {
                            possible.extend(zone.visible_layers.iter().copied());
                            evaluated = true;
                            ZoneState::Conditional
                        }
                    };
                    states.insert((**guid, hash), state);
                }
            }
        }
        Outfit { states }
    }
}

impl RecordCollection for Garments {
    type Item = Garment;

    fn get(&self, guid: &Guid) -> Option<&Garment> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Garment)> + '_ {
        self.by_guid.iter()
    }
}

fn garment(clothing: &SCItemClothingParams, pools: &DataPools) -> Garment {
    Garment {
        zones: clothing
            .chunks
            .iter()
            .filter_map(|h| h.get(pools))
            .filter_map(|chunk| garment_zone(chunk, pools))
            .collect(),
        hidden_ports: clothing
            .hidden_parts
            .iter()
            .filter_map(|h| h.get(pools))
            .filter(|p| !p.port_name.is_empty())
            .map(|p| p.port_name.clone())
            .collect(),
    }
}

fn garment_zone(chunk: &SCItemClothingMeshChunkParams, pools: &DataPools) -> Option<GarmentZone> {
    // Some records leave MeshChunk blank; no mesh piece can name that.
    if matches!(&chunk.mesh_chunk, EMeshChunks::Unrecognized(name) if name.is_empty()) {
        return None;
    }
    let conditions = chunk
        .visibility_conditions
        .iter()
        .filter_map(|h| h.get(pools))
        .map(|condition| ZoneCondition {
            ports: condition
                .port_visibility
                .iter()
                .filter_map(|h| h.get(pools))
                .map(|p| PortCondition {
                    ports: p.ports.clone(),
                    disable_tags: p.disable_tags.clone(),
                    skip_tags: p.skip_tags.clone(),
                    active_tags: p.active_tags.clone(),
                })
                .collect(),
        })
        .collect();
    Some(GarmentZone {
        hash: zone_hash(chunk.mesh_chunk.as_dcb_str()),
        zone: chunk.mesh_chunk.clone(),
        layer: chunk.layer,
        visible_layers: chunk.visible_layers.clone(),
        conditions,
        hide_in_first_person: chunk.hide_in_first_person,
    })
}

mod mesh_chunk_serde {
    use sc_extract::generated::EMeshChunks;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &EMeshChunks, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EMeshChunks, D::Error> {
        Ok(EMeshChunks::from_dcb_str(&String::deserialize(d)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZONE: &str = "hips_zone";

    fn item(n: u8) -> Guid {
        let mut text = String::from("00000000-0000-0000-0000-0000000000");
        text.push_str(&format!("{n:02x}"));
        text.parse().expect("a valid guid")
    }

    fn zone(name: &str, layer: i32, shows: &[i32], conditional: bool) -> GarmentZone {
        GarmentZone {
            zone: EMeshChunks::from_dcb_str(name),
            hash: zone_hash(name),
            layer,
            visible_layers: shows.to_vec(),
            conditions: if conditional {
                vec![ZoneCondition { ports: Vec::new() }]
            } else {
                Vec::new()
            },
            hide_in_first_person: false,
        }
    }

    fn garments(list: Vec<Vec<GarmentZone>>) -> (Garments, Vec<Guid>) {
        let mut out = Garments::new();
        let mut guids = Vec::new();
        for (i, zones) in list.into_iter().enumerate() {
            let guid = item(i as u8);
            out.by_guid.insert(
                guid,
                Garment {
                    zones,
                    hidden_ports: Vec::new(),
                },
            );
            guids.push(guid);
        }
        (out, guids)
    }

    fn states(list: Vec<Vec<GarmentZone>>) -> Vec<ZoneState> {
        let (g, guids) = garments(list);
        let outfit = g.outfit(&guids);
        guids
            .iter()
            .map(|guid| outfit.state(guid, zone_hash(ZONE)))
            .collect()
    }

    #[test]
    fn a_higher_layer_hides_a_lower_one() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 0, &[], false)],
                vec![zone(ZONE, 2, &[], false)],
            ]),
            vec![ZoneState::Hidden, ZoneState::Visible]
        );
    }

    #[test]
    fn items_on_one_layer_share_it() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 2, &[], false)],
                vec![zone(ZONE, 2, &[], false)],
            ]),
            vec![ZoneState::Visible, ZoneState::Visible]
        );
    }

    #[test]
    fn visible_layers_keep_a_lower_layer() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 0, &[], false)],
                vec![zone(ZONE, 1, &[], false)],
                vec![zone(ZONE, 2, &[0], false)],
            ]),
            vec![ZoneState::Visible, ZoneState::Hidden, ZoneState::Visible]
        );
    }

    #[test]
    fn a_kept_layer_can_keep_further_layers() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 0, &[], false)],
                vec![zone(ZONE, 1, &[0], false)],
                vec![zone(ZONE, 2, &[1], false)],
            ]),
            vec![ZoneState::Visible, ZoneState::Visible, ZoneState::Visible]
        );
    }

    #[test]
    fn a_conditional_top_still_covers_what_it_does_not_list() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 0, &[], false)],
                vec![zone(ZONE, 2, &[], true)],
            ]),
            vec![ZoneState::Hidden, ZoneState::Conditional]
        );
    }

    #[test]
    fn a_conditional_list_leaves_the_lower_layer_undecided() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 0, &[], false)],
                vec![zone(ZONE, 2, &[0], true)],
            ]),
            vec![ZoneState::Conditional, ZoneState::Conditional]
        );
    }

    #[test]
    fn unregistered_zones_and_unknown_items_are_visible() {
        let (g, guids) = garments(vec![vec![zone(ZONE, 1, &[], false)]]);
        let outfit = g.outfit(&[guids[0], item(9)]);
        assert_eq!(
            outfit.state(&guids[0], zone_hash("head_zone")),
            ZoneState::Visible
        );
        assert_eq!(outfit.state(&item(9), zone_hash(ZONE)), ZoneState::Visible);
    }

    #[test]
    fn a_later_chunk_replaces_an_earlier_one_for_the_same_zone() {
        assert_eq!(
            states(vec![
                vec![zone(ZONE, 3, &[], false), zone(ZONE, 1, &[], false)],
                vec![zone(ZONE, 2, &[], false)],
            ]),
            vec![ZoneState::Hidden, ZoneState::Visible]
        );
    }

    #[test]
    fn zones_in_lists_by_state() {
        let (g, guids) = garments(vec![
            vec![zone(ZONE, 0, &[], false), zone("head_zone", 0, &[], false)],
            vec![zone(ZONE, 2, &[], false)],
        ]);
        let outfit = g.outfit(&guids);
        assert_eq!(
            outfit.zones_in(&guids[0], ZoneState::Hidden),
            vec![zone_hash(ZONE)]
        );
    }
}
