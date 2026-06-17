//! The provider spine — `HarvestableProviderPreset` → groups → elements + clusters.
//!
//! This is Tier 1: structure + normalized rarity, resolved from the DCB alone
//! (the `harvestable` feature). Each element carries its `HarvestablePreset` GUID
//! unresolved; Tier 2 follows that to a `ResourceType` + gathering mode (via the
//! element's `MiningGlobalParams` family) + scan signal.

use sc_extract::generated::{HarvestableElementGroup, HarvestableProviderPreset};
use sc_extract::{Guid, RecordStore};
use serde::{Deserialize, Serialize};

use crate::mineable::{self, Deposit};

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
    pub group_probability: f32,
    pub mode_share: f32,
    pub elements: Vec<GatherableElement>,
}

/// One harvestable within a group. `share` is `relative_probability` normalized
/// within the group. `harvestable` is the `HarvestablePreset` GUID; `deposit` is
/// its resolved within-rock composition (Tier 2, mineables only — `None` for
/// plants/salvage). Mode + signal land in the following increments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatherableElement {
    pub harvestable: Option<Guid>,
    pub relative_probability: f32,
    pub share: f32,
    pub cluster: Option<Cluster>,
    pub deposit: Option<Deposit>,
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
        let elements = elems
            .into_iter()
            .map(|e| GatherableElement {
                harvestable: e.harvestable,
                relative_probability: e.relative_probability,
                share: norm(e.relative_probability, elem_sum),
                cluster: e.clustering.and_then(|cg| cluster_for(cg, store)),
                deposit: e.harvestable.and_then(|h| mineable::deposit_for(h, store)),
            })
            .collect();
        groups.push(ProviderGroup {
            name: g.group_name.clone(),
            group_probability: g.group_probability,
            mode_share: norm(g.group_probability, group_sum),
            elements,
        });
    }

    Provider { guid, groups }
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
