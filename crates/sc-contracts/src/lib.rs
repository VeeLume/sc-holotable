//! Star Citizen contract / mission data.
//!
//! Star Citizen ships roughly 2,400 contract records in the DCB, produced
//! by a few hundred contract generators. Generator expansion + sub-contract
//! tier enumeration produce ~4,590 contract rows on SC 4.7 LIVE. v2 of
//! this crate exposes those rows directly; "what counts as the same
//! mission" is a consumer-side pooling decision, not a baked-in
//! equivalence rule.
//!
//! This crate walks the generator graph, resolves every GUID the contracts
//! touch (tags, ship entities, blueprint pools, reward currencies), and
//! emits a single [`MissionIndex`] holding the contract list, the
//! registries used to build it, and a precomputed [`MissionPools`]
//! grouping by title key / description key.
//!
//! # Pipeline
//!
//! ```text
//! Datacore + LocaleMap
//!     → ingest  (tag / ship / blueprint / currency registries)
//!     → expand  (generator × handler × contract × sub_contract)
//!     → resolve (GUIDs → typed values via registries)
//!     → MissionIndex (with precomputed pools)
//! ```
//!
//! # Scope
//!
//! The crate owns the generator-expansion pipeline plus the tag and
//! ship-entity registries it needs along the way. Registries live here
//! because sc-contracts is currently the only consumer that needs them;
//! a second consumer would graduate them into a shared helper.
//!
//! Escape hatch for anything the model does not cover: consumers with a
//! `Datacore` reach through `datacore.db()` (raw svarog) or
//! `datacore.pools()` (flat-pool generated types) directly. There is no
//! dedicated raw layer in this crate.
//!
//! # Driving consumer
//!
//! `sc-langpatch` is the primary driver — its contract-annotation work
//! motivated the crate. The full design is at `docs/sc-contracts.md`;
//! `docs/sc-contracts-v2.md` documents the in-flight v2 redesign.

mod blueprints;
mod classify;
mod currency;
mod expand;
mod index;
mod locality;
mod pools;
mod ships;
mod titles;

#[cfg(feature = "tui")]
pub mod tui;

pub use blueprints::{BlueprintItem, BlueprintPool, BlueprintPoolRegistry};
pub use classify::{TagBag, parse_ai_skill};
pub use currency::{CurrencyInfo, RewardCurrencyCatalog};
pub use expand::{
    Availability, BlueprintReward, Cooldowns, DurationRange, Encounter, EncounterPhase,
    EntityEncounter, EntitySlot, HandlerKind, ItemReward, Mission, MissionOrigin, MissionRewards,
    NpcEncounter, NpcSlot, OtherReward, PrereqView, RepReward, RewardAmount, ScripReward,
    ShipEncounter, ShipSlot, expand_all,
};
pub use index::MissionIndex;
pub use locality::{LocalityRegistry, LocalityView, LocationRef, LocationRegistry, SystemKey};
pub use pools::MissionPools;
pub use ships::{ShipCandidate, ShipEntity, ShipRegistry};
pub use titles::{ContractAnchor, ResolvedKeys, resolve_contract_keys};

// ── Narrow-consumer re-exports ──────────────────────────────────────────────
//
// Lets a consumer depend on `sc-contracts` alone and still construct the
// arguments `MissionIndex::build` takes, without adding a direct
// `sc-extract` dep. Type identity is preserved across re-exports because
// every aggregation crate pulls the same `sc-extract` rev.
pub use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, ExtractSnapshot, Guid,
    LocaleKey, LocaleMap, LocalizedItem, LocalizedItemCache, SnapshotMeta,
};

/// Escape hatch for raw DCB queries when the typed model doesn't cover
/// a case. Reach for these only as a last resort; if you find yourself
/// here often, file a feature request so the model can grow to cover it.
pub mod raw {
    pub use sc_extract::svarog_datacore;
    pub use sc_extract::{DataCoreDatabase, Instance, Value};
}
