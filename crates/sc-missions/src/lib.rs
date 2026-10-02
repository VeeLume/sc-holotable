//! Star Citizen contract / mission data.
//!
//! The DCB stores missions the way the engine *generates* them:
//! `ContractGenerator → handler → Contract → SubContract → MissionProperty →
//! spawn descriptions`. This crate walks that graph, resolves every GUID the
//! contracts touch (tags, ship entities, blueprint pools, reward currencies,
//! localities, reputation), and emits one [`Missions`] whose rows read like a
//! mission, with the generator plumbing tucked behind [`MissionOrigin`].
//!
//! # One `Mission` per expansion row
//!
//! A [`Mission`] is exactly one `(generator, handler, contract, optional
//! sub_contract)` tuple with an unambiguous [`Mission::id`]. Nothing is merged:
//! "what counts as the same mission" differs per consumer (a `global.ini`
//! patcher groups by description key, a browser by title + rewards), so
//! grouping is an explicit, consumer-chosen step over [`MissionPools`] plus the
//! `*_mixed` / `*_consistent` divergence helpers on [`Missions`].
//!
//! # Pipeline
//!
//! ```text
//! Datacore
//!     → ingest  (tag / ship / blueprint / currency / locality / reputation registries)
//!     → expand  (generator × handler × contract × sub_contract)
//!     → resolve (GUIDs → typed values via registries)
//!     → Missions (with precomputed pools + reverse indices)
//! ```
//!
//! [`Missions::build`] is self-contained: it builds its own `Tags` and `Items`
//! rather than taking them, because forcing every standalone caller to
//! pre-build foundations is worse ergonomics than the sharing is worth.
//!
//! # Quick start
//!
//! ```no_run
//! use sc_missions::{AssetConfig, AssetData, AssetSource, Datacore, Missions, RecordCollection};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//!
//! let missions = Missions::build(&datacore);
//! let locale = &data.locale;
//!
//! for mission in missions.values() {
//!     // Text is never stored resolved — keys live on the struct, strings are
//!     // looked up against whichever `LocaleMap` is current (so a language-pack
//!     // overlay Just Works). `title_text` also substitutes the `~mission(...)`
//!     // markers it can pin statically; the rest render as `[Token]`.
//!     let title = missions
//!         .title_text(mission, locale)
//!         .unwrap_or_else(|| mission.debug_name.clone());
//!     println!("{title}: {} encounter(s)", mission.encounters.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Grouping: pools + divergence helpers
//!
//! The patcher workflow: pick a pool axis, then ask what differs across its
//! members before rendering a shared line.
//!
//! ```no_run
//! # use sc_missions::{Missions, RewardAmount, RecordCollection};
//! # fn demo(missions: &Missions) {
//! for (description_key, ids) in &missions.pools.description_key {
//!     let Some(head) = missions.get(&ids[0]) else { continue };
//!     let mut block = String::new();
//!
//!     if !missions.rewards_uec_consistent(ids) {
//!         block.push_str("[varies]\n");
//!     } else if let RewardAmount::Fixed(n) = head.rewards.uec {
//!         block.push_str(&format!("{n} aUEC\n"));
//!     } // `Calculated`: the engine computes it at runtime — see the `payout` feature.
//!
//!     if missions.blueprint_mixed(ids) {
//!         block.push_str("[BP*]\n"); // some members award a blueprint, some don't
//!     } else if !head.rewards.blueprints.is_empty() {
//!         block.push_str("[BP]\n");
//!     }
//!
//!     // The bare key (no leading `@`) is what goes back into global.ini.
//!     println!("{} => {block}", description_key.stripped());
//! }
//! # }
//! ```
//!
//! Each helper is a single O(n) walk over the pool's members; call only what
//! you read. They answer *whether* members differ, not *how* — iterate
//! [`Missions::iter_pool`] when you need the distinct values.
//!
//! # Encounters
//!
//! [`Mission::encounters`] covers all three spawn-shaped `MissionProperty`
//! variants. Order is DCB source-walk order (sub-contract → contract overrides
//! → handler → template, first non-empty wins per slot) and is part of the
//! contract — nothing is sorted.
//!
//! An [`EncounterPhase`] holds [`SlotGroup`]s. **Groups all fire; the options
//! inside one group are weighted alternatives of which the engine picks one.**
//! Summing `concurrent` across options over-counts (the "6× Scythe instead of
//! 1–3" bug) — use [`SlotGroup::concurrent_range`] or
//! [`Mission::ship_count_range`].
//!
//! ```no_run
//! # use sc_missions::{Encounter, Items, LocaleMap, Missions};
//! # fn demo(missions: &Missions, mission: &sc_missions::Mission, items: &Items, locale: &LocaleMap) {
//! let tags = &missions.tag_tree;
//! for enc in &mission.encounters {
//!     let Encounter::Ships(ships) = enc else {
//!         // Npcs / Entities have the same phase → group → option shape.
//!         // `Unknown` carries the raw MissionProperty GUID for `datacore.db()`.
//!         println!("  {} ({})", enc.variable_name(), enc.extended_text_token());
//!         continue;
//!     };
//!     for phase in &ships.phases {
//!         for group in &phase.groups {
//!             let (min, max) = group.concurrent_range;
//!             println!("  phase {:?}: {min}–{max} ship(s), one of:", phase.name);
//!             for slot in &group.options {
//!                 let factions: Vec<&str> = slot.positive.factions(tags).collect();
//!                 let hulls: Vec<&str> = slot
//!                     .candidates
//!                     .iter()
//!                     .filter_map(|c| missions.ships.display_name(&c.entity_guid, items, locale))
//!                     .collect();
//!                 println!("    factions={factions:?} skill={:?} hulls={hulls:?}",
//!                     slot.positive.ai_skill());
//!             }
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! An empty `candidates` list means *unknown*, not "nothing spawns": a small
//! share of slots carry tag queries no entity satisfies (likely CIG data bugs).
//!
//! All four tag lists of a spawn description surface as the same [`TagBag`]
//! (sorted GUIDs + parallel names). Classification is on-demand methods taking
//! `&Tags`; the `is_salvage_target` / `is_cargo_recovery` /
//! `is_pre_damaged_wreck` predicates are discovery-driven heuristics over the
//! current `AI` tag subtree, not part of the type contract.
//!
//! # Resolving the GUIDs a mission holds
//!
//! | Field | Resolve through |
//! |---|---|
//! | [`Mission::mission_span`] | [`Missions::localities`] → [`LocalityView`] |
//! | [`MissionRewards::blueprints`] | [`Missions::blueprints`] → [`BlueprintPool`] |
//! | [`ScripReward::currency_guid`] | [`Missions::currency`] |
//! | [`Mission::faction`], rep rewards / prereqs | [`Missions::factions`], [`Missions::rep_standings`] |
//! | [`Mission::category`] | [`Missions::mission_types`] |
//! | [`HaulingLeg::resource`] | [`HaulingLeg::commodity`] over `sc_resources::Resources` |
//! | `CompletedContractTags` prereqs | [`Missions::prerequisite_missions`] (the mission-chain graph) |
//!
//! Reverse joins: [`Missions::missions_for_pool`] and
//! [`Missions::missions_for_item`] ("which missions drop blueprint X").
//!
//! # Escape hatches
//!
//! Consumers holding a `Datacore` reach through `datacore.db()` (raw svarog,
//! re-exported under [`raw`]) or `datacore.records().pools` (generated types).
//! [`Missions::tag_tree`] is public for ad-hoc subtree walks beyond the
//! `TagBag` classifier set.
//!
//! # After a game patch
//!
//! Tag GUIDs and record shapes shift between builds. After regenerating the
//! bindings, re-run the committed audits — `encounter_analytics` is the
//! regression baseline for the encounter graph, `spawn_dig` lists the
//! empty-candidate slots:
//!
//! ```text
//! cargo run -p sc-missions --release --example encounter_analytics
//! ```

mod axes;
mod blueprint_pools;
mod categories;
mod classify;
mod currency;
mod expand;
mod index;
mod locality;
mod markers;
mod pools;
mod reputation;
mod ships;
mod titles;

#[cfg(feature = "payout")]
mod payout;

#[cfg(feature = "tui")]
pub mod tui;

pub use axes::{AxisDiff, AxisKind, AxisValues, SharedTag};
// Blueprint *pools* are a mission-reward mechanic — owned here.
pub use blueprint_pools::{BlueprintPool, BlueprintPoolEntry, BlueprintPools};
pub use categories::{MissionTypeInfo, MissionTypes};
pub use classify::{TagBag, parse_ai_skill};
pub use currency::{CurrencyInfo, RewardCurrencies};
pub use expand::{
    Availability, BlueprintReward, Cooldowns, Difficulty, DurationRange, Encounter, EncounterPhase,
    EntityEncounter, EntitySlot, HandlerKind, HaulingLeg, ItemReward, Mission, MissionOrigin,
    MissionRewards, MissionVar, NpcEncounter, NpcSlot, OtherReward, PrereqView, RepReward,
    RewardAmount, ScripReward, ShipEncounter, ShipSlot, SlotGroup, VarOption, expand_all,
};
pub use index::Missions;
// Re-export the canonical accessor trait (get / iter / len / values) so consumers
// can bring it into scope alongside the collection.
pub use locality::{Localities, LocalityView, LocationRef, Locations, SystemKey};
#[cfg(feature = "payout")]
pub use payout::UecCurve;
pub use reputation::{FactionRep, FactionReputations, ReputationStandings, Standing};
pub use sc_extract::RecordCollection;
// Re-exported so consumers can read `LocationRef::kind` without a direct
// sc-locations dep. Type identity is preserved (same workspace `sc-extract`).
pub use pools::MissionPools;
pub use sc_locations::LocationKind;
pub use ships::{ShipCandidate, ShipEntity, Ships};
pub use titles::{ContractAnchor, ResolvedKeys, resolve_contract_keys};

// ── Narrow-consumer re-exports ──────────────────────────────────────────────
//
// Lets a consumer depend on `sc-missions` alone and still construct the
// arguments `Missions::build` takes, without adding a direct
// `sc-extract` dep. Type identity is preserved across re-exports because
// every aggregation crate pulls the same `sc-extract` rev.
pub use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, ExtractSnapshot, Guid, LocaleKey, LocaleMap,
    SnapshotMeta,
};
// Item envelope now lives in sc-items; re-export for single-crate consumers.
pub use sc_items::{Item, Items};

/// Escape hatch for raw DCB queries when the typed model doesn't cover
/// a case. Reach for these only as a last resort; if you find yourself
/// here often, file a feature request so the model can grow to cover it.
pub mod raw {
    pub use sc_extract::svarog_datacore;
    pub use sc_extract::{DataCoreDatabase, Instance, Value};
}
