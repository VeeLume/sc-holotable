//! `sc-gathering` — the resource-gathering domain (mining / salvage / plants).
//!
//! It joins, it does not duplicate: rarity/cluster come from the
//! `HarvestableProviderPreset` spine here; resource identity resolves to
//! `sc-resources`' catalog; quality to `sc-crafting`; and *where* a provider
//! applies resolves through `sc-locations`' `ObjectContainers` (the
//! `StarMapObject ↔ realized-socpak` bridge). See `docs/resource-gathering.md`.
//!
//! **Status: Tier 1** — the provider spine (groups → elements with normalized
//! rarity + clusters). Resource identity + gathering mode (Tier 2), quality
//! (Tier 3), and the location join land next.

mod location;
mod mineable;
mod mode;
mod provider;

pub use location::ProviderLocations;
pub use mineable::{Deposit, DepositPart};
pub use mode::GatheringMode;
pub use provider::{Cluster, ClusterBand, GatherableElement, Provider, ProviderGroup};

use std::collections::HashMap;

use sc_extract::{Guid, RecordPaths, RecordStore};
use serde::{Deserialize, Serialize};

// Re-export the canonical accessor trait (get / iter / len / values) so consumers
// can bring it into scope alongside the collection.
pub use sc_extract::RecordCollection;

/// Every resource provider, keyed by its `HarvestableProviderPreset` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Providers {
    by_guid: HashMap<Guid, Provider>,
}

impl Providers {
    /// Empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the provider spine from a parsed [`RecordStore`] + [`RecordPaths`]
    /// (the latter resolves the `MiningGlobalParams` record names that classify a
    /// group's gathering mode). DCB-only and offline — the location join is
    /// layered on separately (it needs the live p4k).
    pub fn build(store: &RecordStore, paths: &RecordPaths) -> Self {
        let pools = &store.pools;
        let mut g = Self::new();
        for (&guid, &handle) in &store.records.harvestable.harvestable_provider_preset {
            let Some(preset) = handle.get(pools) else {
                continue;
            };
            g.by_guid
                .insert(guid, provider::provider_for(guid, preset, store, paths));
        }
        g
    }
}

impl sc_extract::RecordCollection for Providers {
    type Item = Provider;

    fn get(&self, guid: &Guid) -> Option<&Provider> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Provider)> + '_ {
        self.by_guid.iter()
    }
}
