//! Tier 2 (resource side) — resolve a `HarvestablePreset` to its within-rock
//! deposit: the `MineableComposition` reached through the rock's entity class.
//!
//! Chain (all typed, GUID references):
//! ```text
//! HarvestablePreset.entityClass → EntityClassDefinition
//!   .components ∋ MineableParams.composition → MineableComposition
//!     .compositionArray[] → MineableCompositionPart.mineableElement → MineableElement.resourceType
//! ```
//! Needs the `entities-mineable` (rock entity classes), `mining`
//! (composition/element), and `resourcetypedatabase` features.

use sc_extract::generated::DataForgeComponentParamsPtr;
use sc_extract::{Guid, LocaleKey, RecordStore};
use serde::{Deserialize, Serialize};

/// The within-rock composition a gatherable resolves to (the `MineableComposition`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// `depositName` locale key (e.g. `@items_commodities_raw_ice`) — the
    /// resource's display identity. Resolve through a `LocaleMap` at the call site.
    pub name: LocaleKey,
    /// Floor on how many distinct elements a rolled rock contains.
    pub minimum_distinct_elements: i32,
    /// Per-element content bands.
    pub parts: Vec<DepositPart>,
}

/// One element band within a [`Deposit`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositPart {
    /// `ResourceType` GUID (via `MineableElement.resourceType`).
    pub resource: Option<Guid>,
    pub min_percentage: f32,
    pub max_percentage: f32,
    pub probability: f32,
    pub quality_scale: f32,
}

/// Resolve a `HarvestablePreset` GUID to its [`Deposit`], or `None` when the
/// preset isn't a mineable rock (plants/salvage take a different chain) or any
/// link is missing/unloaded.
pub(crate) fn deposit_for(harvestable: Guid, store: &RecordStore) -> Option<Deposit> {
    let pools = &store.pools;
    let records = &store.records;

    let preset = records
        .multi_feature
        .harvestable_preset
        .get(&harvestable)?
        .get(pools)?;
    let entity_class = preset.entity_class?;
    let class = records
        .multi_feature
        .entity_class_definition
        .get(&entity_class)?
        .get(pools)?;

    // The mineable rock carries a MineableParams component pointing at its composition.
    let mineable = class.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::MineableParams(h) => h.get(pools),
        _ => None,
    })?;
    let composition = records
        .multi_feature
        .mineable_composition
        .get(&mineable.composition?)?
        .get(pools)?;

    let parts = composition
        .composition_array
        .iter()
        .filter_map(|h| h.get(pools))
        .map(|part| DepositPart {
            resource: part.mineable_element.and_then(|me| {
                records
                    .multi_feature
                    .mineable_element
                    .get(&me)
                    .and_then(|h| h.get(pools))
                    .and_then(|el| el.resource_type)
            }),
            min_percentage: part.min_percentage,
            max_percentage: part.max_percentage,
            probability: part.probability,
            quality_scale: part.quality_scale,
        })
        .collect();

    Some(Deposit {
        name: composition.deposit_name.clone(),
        minimum_distinct_elements: composition.minimum_distinct_elements,
        parts,
    })
}
