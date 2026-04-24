//! Star Citizen contract / mission data.
//!
//! Star Citizen ships roughly 2,400 contract records in the DCB, produced
//! by a few hundred contract generators. After generator expansion, sub-
//! contract tier enumeration, and similarity merging those collapse to
//! ~1,497 effective contracts (the SCMDB catalog size).
//!
//! This crate walks the generator graph, resolves every GUID the contracts
//! touch (tags, ship entities, blueprint pools, reward currencies), and
//! emits a single [`ContractIndex`] holding the merged [`Contract`] list
//! plus the registries used to build it.
//!
//! # Pipeline
//!
//! ```text
//! Datacore + LocaleMap
//!     → ingest  (tag / ship / blueprint / currency registries)
//!     → expand  (generator × handler × contract × sub_contract)
//!     → resolve (GUIDs → typed values via registries)
//!     → merge   (similarity-group expansions into Contract entries)
//!     → ContractIndex
//! ```
//!
//! # Scope
//!
//! The crate owns the generator-expansion → merged-contract pipeline plus
//! the tag and ship-entity registries it needs along the way. The
//! registries live here because sc-contracts is currently the only
//! consumer that needs them; if a second crate ever does, they graduate
//! to a shared helper.
//!
//! Escape hatch for anything the model does not cover: consumers with a
//! `Datacore` reach through `datacore.db()` (raw svarog) or
//! `datacore.pools()` (flat-pool generated types) directly. There is no
//! dedicated raw layer in this crate.
//!
//! # Driving consumer
//!
//! `sc-langpatch` is the primary driver — its contract-annotation work
//! motivated the crate. The full design is at `docs/sc-contracts.md`.
//!
//! # Status
//!
//! Design approved, implementation in progress. See `docs/sc-contracts.md`
//! for the spec, `status.md` for the current implementation state.

mod blueprints;
mod classify;
mod currency;
mod expand;
mod index;
mod locality;
mod merge;
mod ships;
mod titles;

pub use blueprints::{BlueprintItem, BlueprintPool, BlueprintPoolRegistry};
pub use classify::{SpawnContext, parse_ai_skill};
pub use currency::{CurrencyInfo, RewardCurrencyCatalog};
pub use expand::{
    Availability, BlueprintReward, ContractOrigin, Cooldowns, DurationRange, EncounterGroup,
    EncounterSlot, EncounterWave, ExpandedContract, HandlerKind, ItemReward, OtherReward,
    PrereqView, RepReward, RewardAmount, ScripReward, expand_all,
};
pub use index::ContractIndex;
pub use locality::{LocalityRegistry, LocalityView, LocationRef, LocationRegistry, SystemKey};
pub use merge::{
    BpConflictGroup, BpConflictMember, Contract, MergeStats, Variation, find_bp_conflicts,
    merge_expansions, merge_stats,
};
pub use ships::{ShipCandidate, ShipEntity, ShipRegistry};
pub use titles::{ContractAnchor, ResolvedText, resolve_contract_text};
