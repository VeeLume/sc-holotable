//! Tier 2 — resolve a `HarvestablePreset` to its mineable-rock data: the
//! within-rock [`Deposit`], the rock's `MiningGlobalParams` reference (the
//! gathering-mode ground truth), and its `Resource`-channel scan signal.
//!
//! All reached through the rock's entity class, in one pass:
//! ```text
//! HarvestablePreset.entityClass → EntityClassDefinition.components ∋
//!   MineableParams        → globalParams (mode) + composition (Deposit)
//!   SSCSignatureSystemParams → radarProperties.baseSignatureParams.signatures[4]
//! ```
//! Needs `entities-mineable` + `mining` + `resourcetypedatabase`.

use sc_extract::generated::{
    DataForgeComponentParamsPtr, SSCSignatureParamsBasePtr, SSCSignatureSystemParams,
};
use sc_extract::{Guid, LocaleKey, RecordStore};
use serde::{Deserialize, Serialize};

/// `ESignatureType::Resource` — the only non-zero signature channel on mineable
/// rocks; its value ×1000 is the in-game "sig" readout.
const RESOURCE_CHANNEL: usize = 4;

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

/// Everything Tier 2 reads off a mineable rock's entity class.
#[derive(Debug, Clone, Default)]
pub(crate) struct RockData {
    pub deposit: Option<Deposit>,
    /// `MiningGlobalParams` GUID — the rock's tool family (mode ground truth).
    pub global_params: Option<Guid>,
    /// `Resource`-channel scan signature (raw; ×1000 is the displayed "sig").
    pub signal: Option<f32>,
}

/// Resolve a `HarvestablePreset` GUID to its rock data in a single entity-class
/// pass. Empty for plants/salvage (no `MineableParams`) or missing links.
pub(crate) fn resolve_rock(harvestable: Guid, store: &RecordStore) -> RockData {
    let mut out = RockData::default();
    let pools = &store.pools;
    let records = &store.records;

    let Some(preset) = records
        .multi_feature
        .harvestable_preset
        .get(&harvestable)
        .and_then(|h| h.get(pools))
    else {
        return out;
    };
    let Some(class) = preset.entity_class.and_then(|ec| {
        records
            .multi_feature
            .entity_class_definition
            .get(&ec)
            .and_then(|h| h.get(pools))
    }) else {
        return out;
    };

    for component in &class.components {
        match component {
            DataForgeComponentParamsPtr::MineableParams(h) => {
                if let Some(mp) = h.get(pools) {
                    out.global_params = mp.global_params;
                    out.deposit = mp.composition.and_then(|cg| composition_deposit(cg, store));
                }
            }
            DataForgeComponentParamsPtr::SSCSignatureSystemParams(h) => {
                if let Some(sig) = h.get(pools) {
                    out.signal = resource_signal(sig, store);
                }
            }
            _ => {}
        }
    }
    out
}

/// Resolve a `MineableComposition` GUID into a [`Deposit`].
fn composition_deposit(composition: Guid, store: &RecordStore) -> Option<Deposit> {
    let pools = &store.pools;
    let records = &store.records;
    let comp = records
        .multi_feature
        .mineable_composition
        .get(&composition)?
        .get(pools)?;
    let parts = comp
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
        name: comp.deposit_name.clone(),
        minimum_distinct_elements: comp.minimum_distinct_elements,
        parts,
    })
}

/// Read the `Resource`-channel signature off a `SSCSignatureSystemParams` component.
fn resource_signal(sig: &SSCSignatureSystemParams, store: &RecordStore) -> Option<f32> {
    let pools = &store.pools;
    let radar = sig.radar_properties?.get(pools)?;
    let base = radar.base_signature_params.as_ref()?;
    let params = match base {
        SSCSignatureParamsBasePtr::SSCSignatureSystemBaseSignatureParams(h) => h.get(pools)?,
        _ => return None,
    };
    params.signatures.get(RESOURCE_CHANNEL).copied()
}
