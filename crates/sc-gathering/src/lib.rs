//! `sc-gathering` — the resource-gathering domain (mining / salvage / plants).
//!
//! It joins, it does not duplicate: rarity/cluster come from the
//! `HarvestableProviderPreset` spine here; resource identity resolves to
//! `sc-resources`' catalog; quality to `sc-crafting`; and *where* a provider
//! applies resolves through `sc-locations`' `LocationContainers` (the system-OC
//! `StarMapObject ↔ socpak` bridge). See `docs/resource-gathering.md`.
//!
//! **Status: Tier 1** — the provider spine (groups → elements with normalized
//! rarity + clusters). Resource identity + gathering mode (Tier 2), quality
//! (Tier 3), and the location join land next.

mod provider;

pub use provider::{Cluster, ClusterBand, GatherableElement, Provider, ProviderGroup};

use std::collections::HashMap;

use sc_extract::{Guid, RecordStore};
use serde::{Deserialize, Serialize};

/// Every resource provider, keyed by its `HarvestableProviderPreset` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gathering {
    by_guid: HashMap<Guid, Provider>,
}

impl Gathering {
    /// Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the provider spine from a parsed [`RecordStore`] (the `harvestable`
    /// feature). DCB-only and offline — the location join is layered on
    /// separately (it needs the live p4k).
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut g = Self::new();
        for (&guid, &handle) in &store.records.harvestable.harvestable_provider_preset {
            let Some(preset) = handle.get(pools) else {
                continue;
            };
            g.by_guid
                .insert(guid, provider::provider_for(guid, preset, store));
        }
        g
    }

    /// The provider for a body/field, by its `HarvestableProviderPreset` GUID.
    pub fn provider(&self, guid: &Guid) -> Option<&Provider> {
        self.by_guid.get(guid)
    }

    /// Every provider. Order is unspecified.
    pub fn providers(&self) -> impl Iterator<Item = &Provider> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}
