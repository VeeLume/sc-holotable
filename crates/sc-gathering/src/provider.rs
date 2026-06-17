//! The provider spine — `HarvestableProviderPreset` → groups → elements + clusters.
//!
//! This is Tier 1: structure + normalized rarity, resolved from the DCB alone
//! (the `harvestable` feature). Each element carries its `HarvestablePreset` GUID
//! unresolved; Tier 2 follows that to a `ResourceType` + gathering mode (via the
//! element's `MiningGlobalParams` family) + scan signal.

use sc_extract::generated::{HarvestableElementGroup, HarvestableProviderPreset};
use sc_extract::{Guid, RecordPaths, RecordStore};
use serde::{Deserialize, Serialize};

use crate::mineable::{self, Deposit};
use crate::mode::GatheringMode;

/// One body/field's resource-provider spine, resolved from a
/// `HarvestableProviderPreset` (keyed by its record GUID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub guid: Guid,
    pub groups: Vec<ProviderGroup>,
}

/// A weighted gathering group (`SpaceShip_Mineables`, `FPS_Mineables`,
/// `Salvage_*`, …). `mode_share` is `group_probability / Σ group_probability`
/// across the provider's groups. `name` is the raw `groupName` label — the typed
/// gathering **mode** is derived in Tier 2 from the element's `MiningGlobalParams`
/// family, not by matching this string.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderGroup {
    pub name: String,
    /// Tool family, derived from the elements' `MiningGlobalParams` family
    /// (mineables) or this group's `name` (plants/salvage).
    pub mode: GatheringMode,
    pub group_probability: f32,
    pub mode_share: f32,
    pub elements: Vec<GatherableElement>,
}

/// One harvestable within a group. `share` is `relative_probability` normalized
/// within the group. `harvestable` is the `HarvestablePreset` GUID; `deposit`,
/// `global_params`, and `signal` are its resolved mineable-rock data (Tier 2,
/// mineables only — `None` for plants/salvage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatherableElement {
    pub harvestable: Option<Guid>,
    pub relative_probability: f32,
    pub share: f32,
    pub cluster: Option<Cluster>,
    pub deposit: Option<Deposit>,
    /// `MiningGlobalParams` GUID — the rock's tool-family ground truth (the basis
    /// for the group's [`GatheringMode`]). `None` for plants/salvage.
    pub global_params: Option<Guid>,
    /// `Resource`-channel scan signature (raw; ×1000 is the displayed "sig").
    pub signal: Option<f32>,
}

/// Cluster spawn: a probability plus **weighted discrete sizes** (each band has
/// its own size + weight — not a uniform range). The displayed "4–6" is the
/// envelope of the bands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub probability_of_clustering: f32,
    pub bands: Vec<ClusterBand>,
}

impl Cluster {
    /// Smallest `min_size` across the bands (the low end of "4–6").
    pub fn min_size(&self) -> i32 {
        self.bands.iter().map(|b| b.min_size).min().unwrap_or(0)
    }
    /// Largest `max_size` across the bands (the high end of "4–6").
    pub fn max_size(&self) -> i32 {
        self.bands.iter().map(|b| b.max_size).max().unwrap_or(0)
    }
}

/// One weighted size band of a [`Cluster`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterBand {
    pub min_size: i32,
    pub max_size: i32,
    pub weight: f32,
}

/// Resolve one provider preset into the typed spine.
pub(crate) fn provider_for(
    guid: Guid,
    preset: &HarvestableProviderPreset,
    store: &RecordStore,
    paths: &RecordPaths,
) -> Provider {
    let pools = &store.pools;

    let raw_groups: Vec<&HarvestableElementGroup> = preset
        .harvestable_groups
        .iter()
        .filter_map(|h| h.get(pools))
        .collect();
    let group_sum: f32 = raw_groups.iter().map(|g| g.group_probability).sum();

    let mut groups = Vec::with_capacity(raw_groups.len());
    for g in raw_groups {
        let elems: Vec<_> = g.harvestables.iter().filter_map(|h| h.get(pools)).collect();
        let elem_sum: f32 = elems.iter().map(|e| e.relative_probability).sum();
        let elements: Vec<GatherableElement> = elems
            .into_iter()
            .map(|e| {
                let rock = e
                    .harvestable
                    .map(|h| mineable::resolve_rock(h, store))
                    .unwrap_or_default();
                GatherableElement {
                    harvestable: e.harvestable,
                    relative_probability: e.relative_probability,
                    share: norm(e.relative_probability, elem_sum),
                    cluster: e.clustering.and_then(|cg| cluster_for(cg, store)),
                    deposit: rock.deposit,
                    global_params: rock.global_params,
                    signal: rock.signal,
                }
            })
            .collect();
        groups.push(ProviderGroup {
            mode: group_mode(&elements, &g.group_name, paths),
            name: g.group_name.clone(),
            group_probability: g.group_probability,
            mode_share: norm(g.group_probability, group_sum),
            elements,
        });
    }

    Provider { guid, groups }
}

/// Derive a group's mode from the rock's `MiningGlobalParams` family (the
/// per-element ground truth), falling back to the `groupName` for plants/salvage.
fn group_mode(
    elements: &[GatherableElement],
    group_name: &str,
    paths: &RecordPaths,
) -> GatheringMode {
    let from_global_params = elements
        .iter()
        .find_map(|e| e.global_params)
        .and_then(|gp| paths.get(&gp))
        .map(|rp| GatheringMode::classify(&rp.name));
    from_global_params.unwrap_or_else(|| GatheringMode::classify(group_name))
}

/// Resolve a `HarvestableClusterPreset` GUID into a [`Cluster`].
fn cluster_for(guid: Guid, store: &RecordStore) -> Option<Cluster> {
    let pools = &store.pools;
    let preset = store
        .records
        .harvestable
        .harvestable_cluster_preset
        .get(&guid)?
        .get(pools)?;
    let bands = preset
        .cluster_params_array
        .iter()
        .filter_map(|h| h.get(pools))
        .map(|p| ClusterBand {
            min_size: p.min_size,
            max_size: p.max_size,
            weight: p.relative_probability,
        })
        .collect();
    Some(Cluster {
        probability_of_clustering: preset.probability_of_clustering,
        bands,
    })
}

/// `value / sum`, guarding the empty/zero-sum group.
fn norm(value: f32, sum: f32) -> f32 {
    if sum > 0.0 { value / sum } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_envelope() {
        let c = Cluster {
            probability_of_clustering: 100.0,
            bands: vec![
                ClusterBand {
                    min_size: 4,
                    max_size: 4,
                    weight: 0.6,
                },
                ClusterBand {
                    min_size: 5,
                    max_size: 5,
                    weight: 0.3,
                },
                ClusterBand {
                    min_size: 6,
                    max_size: 6,
                    weight: 0.1,
                },
            ],
        };
        assert_eq!(c.min_size(), 4);
        assert_eq!(c.max_size(), 6); // displayed "4–6"
    }

    #[test]
    fn norm_guards_zero() {
        assert_eq!(norm(6.0, 43.5), 6.0 / 43.5);
        assert_eq!(norm(1.0, 0.0), 0.0);
    }
}
