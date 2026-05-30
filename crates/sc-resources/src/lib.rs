//! Resource catalog over the DCB's `ResourceType` records.
//!
//! Every resource entity in the game — raw ores (`Aluminum`, `Quantainium`,
//! …), refined materials, etc. — is a `ResourceType` record in
//! `libs/foundry/records/resourcetypedatabase/resourcetypedatabase.xml`.
//! SC 4.8 holds ~206 records (ores, refined metals, drugs, commodities,
//! ship ammunition sizes 1–9, infrastructure resources, …); ~30 of those
//! form the raw→refined graph via [`Resource::refined_version`].
//!
//! # Who references this
//!
//! - `sc-crafting`: every `CraftingCost_Resource` and `CraftingResult_Resource`
//!   carries a resource GUID. The crafting recipe ingredient model bottoms
//!   out here.
//! - `sc-crafting` quality subsystem: `CraftingQualityQuantizationRecord`
//!   discretizes per-resource quality into crafting tiers, keyed by
//!   resource GUID.
//! - `sc-crafting` global params: `dismantleBlacklistResources` references
//!   resource GUIDs.
//!
//! # Refining graph
//!
//! `Resource.refined_version` is currently *the* refining mechanism — a
//! raw resource points at its refined counterpart. `refined_version_of`
//! walks the graph forwards. The schema also defines a per-blueprint
//! `CraftingProcess_Refining` but it is dormant + 0 records in SC 4.8.
//!
//! # Not yet modelled
//!
//! `ResourceType.density_type` (polymorphic `ResourceTypeDensityType_*`)
//! and `ResourceType.properties: Vec<ResourceTypePropertiesPtr>` (also
//! polymorphic) are reachable on the typed record but the variants need
//! a separate dig before locking. Added as a follow-up when sc-crafting
//! needs them — see `docs/sc-crafting.md` open questions.

use std::collections::HashMap;

use sc_extract::generated::{RecordLookup, ResourceType};
use sc_extract::{Guid, LocaleKey, RecordStore};
use serde::{Deserialize, Serialize};

/// A single resource entry, projected from the typed `ResourceType` pool.
///
/// Fields currently captured cover the load-bearing data: name +
/// description for display, `refined_version` for the refining graph,
/// thumbnail paths for UI, the dismantle-validation flag, and the
/// optional default-cargo-containers + RTT thumbnail entity refs.
/// `density_type` and `properties` are deliberately deferred — see
/// the module docs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    /// The resource's GUID. Stable across patches.
    pub guid: Guid,
    /// Display-name locale key.
    pub name_key: LocaleKey,
    /// Description locale key.
    pub description_key: LocaleKey,
    /// Default raster thumbnail asset path.
    pub thumbnail_path: String,
    /// Default SVG thumbnail asset path.
    pub thumbnail_path_svg: String,
    /// Entity class used for the runtime-rendered thumbnail (RTT), if any.
    pub rtt_thumbnail_entity_class: Option<Guid>,
    /// **The refining edge:** the resource produced by refining `self`.
    /// `None` for already-refined / non-refinable resources.
    pub refined_version: Option<Guid>,
    /// True when this resource is subject to default-cargo-box validation.
    pub validate_default_cargo_box: bool,
    // `default_cargo_containers: Option<Handle<SResourceTypeDefaultCargoContainers>>`
    // on the raw record is an inline nested struct (Handle, not Guid). Not
    // surfaced in the MVP — add when a consumer needs cargo-box defaults.
}

/// Flat lookup over every `ResourceType` record in the DCB. Build once,
/// share by reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    by_guid: HashMap<Guid, Resource>,
}

impl Resources {
    /// Construct an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the catalog from a parsed [`RecordStore`] (typed
    /// `ResourceType` pool). The build is cheap — ~30 records in SC 4.8.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut catalog = Self::new();
        for (&guid, &handle) in &store.records.multi_feature.resource_type {
            let Some(rt) = handle.get(pools) else {
                continue;
            };
            catalog.insert(resource_for(guid, rt));
        }
        catalog
    }

    /// Insert or replace a resource entry.
    pub fn insert(&mut self, resource: Resource) {
        self.by_guid.insert(resource.guid, resource);
    }

    /// Look up a resource by GUID.
    pub fn get(&self, guid: &Guid) -> Option<&Resource> {
        self.by_guid.get(guid)
    }

    /// One step along the refining graph: the resource that `guid`
    /// refines into, or `None` if there's no refined version (or the
    /// referenced resource isn't in the catalog).
    pub fn refined_version_of(&self, guid: &Guid) -> Option<&Resource> {
        let next = self.by_guid.get(guid)?.refined_version?;
        self.by_guid.get(&next)
    }

    /// Iterate over every resource. Order is unspecified.
    pub fn all(&self) -> impl Iterator<Item = &Resource> + '_ {
        self.by_guid.values()
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

/// Project one typed `ResourceType` into a [`Resource`]. Shared by
/// [`Resources::build`] and [`ResourcesBuilder`].
fn resource_for(guid: Guid, rt: &ResourceType) -> Resource {
    Resource {
        guid,
        name_key: rt.display_name.clone(),
        description_key: rt.description.clone(),
        thumbnail_path: rt.default_thumbnail_path.clone(),
        thumbnail_path_svg: rt.default_thumbnail_path_svg.clone(),
        rtt_thumbnail_entity_class: rt.rtt_thumbnail_entity_class,
        refined_version: rt.refined_version,
        validate_default_cargo_box: rt.validate_default_cargo_box,
    }
}

/// [`sc_extract::RecordVisitor`] that builds a [`Resources`] catalog
/// inside a bundled walk. Declares interest in `ResourceType` records.
/// Equivalent to [`Resources::build`] but fusible with other visitors.
#[derive(Default)]
pub struct ResourcesBuilder {
    inner: Resources,
}

impl sc_extract::RecordVisitor for ResourcesBuilder {
    type Output = Resources;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["ResourceType"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = ResourceType::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(rt) = handle.get(&store.pools) else {
            return;
        };
        self.inner.insert(resource_for(item.guid, rt));
    }

    fn finish(self) -> Resources {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn make(guid: Guid, name: &str, refined: Option<Guid>) -> Resource {
        Resource {
            guid,
            name_key: LocaleKey::from(format!("@resource_{name}")),
            description_key: LocaleKey::from(format!("@resource_{name}_desc")),
            thumbnail_path: format!("textures/{name}.dds"),
            thumbnail_path_svg: format!("textures/{name}.svg"),
            rtt_thumbnail_entity_class: None,
            refined_version: refined,
            validate_default_cargo_box: true,
        }
    }

    #[test]
    fn new_catalog_is_empty() {
        let cat = Resources::new();
        assert!(cat.is_empty());
        assert_eq!(cat.len(), 0);
    }

    #[test]
    fn insert_and_lookup() {
        let mut cat = Resources::new();
        let raw = g(1);
        let refined = g(2);
        cat.insert(make(raw, "Aluminum_Raw", Some(refined)));
        cat.insert(make(refined, "Aluminum", None));
        assert_eq!(cat.len(), 2);
        assert_eq!(cat.get(&raw).map(|r| r.guid), Some(raw));
        assert_eq!(cat.refined_version_of(&raw).map(|r| r.guid), Some(refined));
        assert!(cat.refined_version_of(&refined).is_none());
    }

    #[test]
    fn refined_version_of_unknown_returns_none() {
        let cat = Resources::new();
        assert!(cat.refined_version_of(&g(99)).is_none());
    }

    #[test]
    fn refined_version_of_dangling_pointer_returns_none() {
        // resource exists but its `refined_version` GUID isn't in the catalog
        let mut cat = Resources::new();
        cat.insert(make(g(1), "Orphan", Some(g(255))));
        assert!(cat.refined_version_of(&g(1)).is_none());
    }

    #[test]
    fn serde_round_trip() {
        let mut cat = Resources::new();
        cat.insert(make(g(1), "Gold", None));
        let json = serde_json::to_string(&cat).unwrap();
        let decoded: Resources = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.get(&g(1)).map(|r| r.name_key.as_ref()), Some("@resource_Gold"));
    }
}
