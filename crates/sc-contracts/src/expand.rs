//! Contract expansion — walk every `ContractGenerator` root and emit
//! one [`ExpandedContract`] per concrete (handler, contract,
//! optional sub_contract) node in the graph.
//!
//! This is stage 3 of the pipeline in `docs/sc-contracts.md`. Stage 4
//! (merge into final [`crate::Contract`]s) groups these expansions by
//! `(title, description, reward_signature)`.
//!
//! The v1 expander materialises the fields sc-langpatch's existing
//! `mission_enhancer` needs to migrate off its svarog-instance walker:
//! resolved title + description, shareable / once-only / illegal flags,
//! and the blueprint reward (if any). Ship-encounter resolution and
//! the full reward model land in subsequent iterations.

use std::collections::HashSet;

use sc_extract::generated::{
    BaseMissionPropertyValuePtr, BlueprintRewards, CareerContract, Contract, ContractAvailability,
    ContractBoolParam, ContractBoolParamType, ContractClass_Contract,
    ContractGeneratorHandlerBasePtr, ContractIntParam, ContractIntParamType, ContractLegacy,
    ContractParamOverrides, ContractPrerequisite_CompletedContractTags,
    ContractPrerequisite_CrimeStat, ContractPrerequisite_Locality, ContractPrerequisite_Location,
    ContractPrerequisite_LocationProperty, ContractPrerequisite_Reputation,
    ContractPrerequisiteBasePtr, ContractResultBasePtr, ContractResults, DataPools,
    ELocationTypeLevel, Handle, MissionProperty, SubContract, TagList,
};
use sc_extract::{Datacore, Guid, LocaleMap};

use crate::blueprints::BlueprintPoolRegistry;
use crate::classify::SpawnContext;
use crate::currency::RewardCurrencyCatalog;
use crate::locality::LocalityRegistry;
use crate::ships::{ShipCandidate, ShipRegistry};
use crate::titles::{ContractAnchor, ResolvedText, resolve_contract_text};

/// One concrete contract node in the expanded generator graph.
///
/// A contract with three sub-contract tiers produces three
/// `ExpandedContract`s (one per tier, each with the tier's overridden
/// fields). A contract with no sub-contracts produces one.
#[derive(Debug, Clone)]
pub struct ExpandedContract {
    /// GUID of the contract (or sub-contract, when `origin` is
    /// [`ContractOrigin::SubContract`]).
    pub id: Guid,
    /// Internal debug name from the contract record. Useful for
    /// cross-referencing with SCMDB and the DCB.
    pub debug_name: String,

    /// GUID of the owning `ContractGenerator` record.
    pub generator_id: Guid,
    /// Typed handler kind — which of the six `ContractGeneratorHandler_*`
    /// shapes owns this contract.
    pub handler_kind: HandlerKind,
    /// `debugName` on the handler, useful for grouping contracts that
    /// share a generator family (`TarPits_Unlawful_Salvage_Career` →
    /// all TarPits salvage missions).
    pub handler_debug_name: String,

    /// Resolved title text (see [`resolve_contract_text`]).
    pub title: Option<String>,
    /// Resolved description text.
    pub description: Option<String>,
    /// True if the title or description contains at least one
    /// `~mission(variable)` runtime substitution marker. When set,
    /// consumers should mark any text they derive from title/description
    /// as non-guaranteed at display time.
    pub has_runtime_substitution: bool,

    /// `ActiveContractSettings.canBeShared` on the contract's template,
    /// overrideable by a `CanBeShared` bool-param at any inheritance
    /// level. Defaults to `false` when neither source provides a value.
    pub shareable: bool,
    /// True if the contract carries an explicit `Illegal` bool param
    /// override. This is the low-level flag — full illegal
    /// classification combining crime-stat prereqs and modifiers is a
    /// stage-4 concern.
    pub illegal_flag: bool,

    /// Effective availability after applying handler → contract →
    /// sub-contract inheritance (bool + int param overrides).
    pub availability: Availability,

    /// Blueprint reward attached to this contract, if any. Items are
    /// pre-resolved via [`BlueprintPoolRegistry`].
    pub blueprint_reward: Option<BlueprintReward>,

    /// UEC reward. Most contracts use `RewardAmount::Calculated`
    /// (engine-computed at runtime); occasional contracts pay a
    /// `RewardAmount::Fixed(i32)` aUEC amount directly.
    pub reward_uec: RewardAmount,
    /// Scrip rewards — `ContractResult_Item` entries whose entity_class
    /// is in [`RewardCurrencyCatalog`].
    pub reward_scrip: Vec<ScripReward>,
    /// Reputation rewards. Amount is `None` for
    /// `ContractResult_CalculatedReputation` (engine-computed) and
    /// `Some(i32)` for `_LegacyReputation` with a resolved amount.
    pub reward_rep: Vec<RepReward>,
    /// Item rewards — `ContractResult_Item` entries whose entity_class
    /// is **not** a currency (ship unlocks, collector items, …).
    pub reward_items: Vec<ItemReward>,
    /// Other reward kinds (BadgeAward, ScenarioProgress, JournalEntry,
    /// CompletionTag(s), CompletionBounty, RefundBuyIn, ItemsWeighting,
    /// Reward). Detailed field modelling deferred to stage 4 when the
    /// downstream interpretation is clearer.
    pub reward_other: Vec<OtherReward>,

    /// Additional prerequisites — unified view across
    /// `Contract.additional_prerequisites`,
    /// `SubContract.additional_prerequisites`, and
    /// `ContractAvailability.prerequisites` (handler-level).
    /// Resolved into a typed enum; unknown kinds preserve the raw
    /// `(struct_index, instance_index)` pointer.
    pub prerequisites: Vec<PrereqView>,

    /// Ship encounter groups — one per
    /// `MissionPropertyValue_ShipSpawnDescriptions` in the contract
    /// graph. Each group holds named waves whose spawn tag queries
    /// are already resolved through [`ShipRegistry::resolve_spawn`] +
    /// classified through [`SpawnContext::classify`].
    pub encounters: Vec<EncounterGroup>,

    /// Localities this expansion is offered at. Every `Locality`
    /// prereq in the expansion's inheritance chain (handler +
    /// contract + sub-contract) produces one entry, deduplicated by
    /// `locality_guid` and resolved through
    /// [`LocalityRegistry`]. Unresolvable GUIDs are dropped.
    ///
    /// Order follows prereq walking order: handler first (inherited),
    /// then contract, then sub-contract (most-specific last), with
    /// first-seen-wins deduplication.
    pub mission_span: Vec<Guid>,

    /// Which of the three concrete contract-record subtypes this
    /// expansion came from, plus the sub-contract GUID when applicable.
    pub origin: ContractOrigin,
}

/// Effective availability after inheritance resolution. Handler-level
/// [`ContractAvailability`] provides the base; contract and sub-contract
/// bool / int param overrides override specific fields. Fields that
/// never got a value stay at the `ContractAvailability` default (all
/// zero/false).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Availability {
    pub once_only: bool,
    pub can_reaccept_after_abandoning: bool,
    pub can_reaccept_after_failing: bool,
    pub has_personal_cooldown: bool,
    pub hide_in_mobi_glas: bool,
    pub available_in_prison: bool,
    pub notify_on_available: bool,
    pub max_players_per_instance: i32,
    pub cooldowns: Cooldowns,
}

/// Personal / abandon / fail cooldown model. All times in seconds
/// (matching the DCB's `ContractAvailability` fields). `None` means
/// no cooldown of that kind.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cooldowns {
    /// Post-completion personal cooldown. Present iff
    /// `has_personal_cooldown` is true. `variation` is ± jitter on
    /// the mean.
    pub completion: Option<DurationRange>,
    /// After voluntary abandon.
    pub abandon: Option<DurationRange>,
    /// After fail. When `can_reaccept_after_failing` is false, the
    /// contract cannot be reaccepted regardless of this value.
    pub fail: Option<DurationRange>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DurationRange {
    pub mean_seconds: f32,
    pub variation_seconds: f32,
}

/// How a reward amount is delivered.
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum RewardAmount {
    /// No reward of this kind.
    #[default]
    None,
    /// Engine computes the amount at runtime (most UEC rewards).
    /// `has_marker: bool` indicates whether we saw a
    /// `ContractResult_CalculatedReward` or derived this from a
    /// missing payout — in practice always `true` post-expansion.
    Calculated,
    /// Fixed amount from the DCB.
    Fixed(i32),
}


/// One scrip / typed-currency reward.
#[derive(Debug, Clone)]
pub struct ScripReward {
    pub currency_guid: Guid,
    /// Localized display name from the currency catalog (`MG Scrip`,
    /// `Council Scrip`).
    pub display_name: String,
    /// Catalog record name (stable across patches, useful for
    /// faction-keyed routing).
    pub record_name: String,
    pub amount: i32,
}

/// One reputation reward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepReward {
    pub faction: Option<Guid>,
    pub scope: Option<Guid>,
    /// `Some(n)` when from `ContractResult_LegacyReputation` with a
    /// resolved `SReputationRewardAmount`; `None` when from
    /// `ContractResult_CalculatedReputation`.
    pub amount: Option<i32>,
}

/// One non-currency item reward.
#[derive(Debug, Clone)]
pub struct ItemReward {
    pub entity_class: Guid,
    /// Localized display name from `DisplayNameCache`.
    pub display_name: String,
    pub amount: i32,
}

/// Other reward kinds classified but not yet fully modelled. Keeps
/// the raw reward kind so stage-4 / consumer code can dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum OtherReward {
    BadgeAward,
    ScenarioProgress,
    JournalEntry,
    CompletionTags,
    CompletionBounty,
    ItemsWeighting,
    /// Raw `ContractResult_Reward` — wraps an inline `MissionReward`.
    Reward,
    /// `ContractResult_RefundBuyIn.refundMultiplier`.
    RefundBuyIn(f32),
    /// Polymorphic-enum fallback (feature-gated subclass or new kind).
    Unknown {
        struct_index: u32,
        instance_index: u32,
    },
}

/// Resolved view of a `ContractPrerequisiteBase` subclass. Mirrors
/// the typed enum variants in [`ContractPrerequisiteBasePtr`] but
/// flattens handle lookups so downstream code doesn't need `pools`.
#[derive(Debug, Clone)]
pub enum PrereqView {
    Locality {
        locality: Option<Guid>,
    },
    Location {
        location: Option<Guid>,
    },
    LocationProperty {
        variable_name: String,
        extended_text_token: String,
        level: ELocationTypeLevel,
    },
    CrimeStat {
        include_when_sharing: bool,
        jurisdiction_override: Option<Guid>,
        min: f32,
        max: f32,
    },
    Reputation {
        include_when_sharing: bool,
        faction: Option<Guid>,
        scope: Option<Guid>,
        exclude: bool,
        min_standing: Option<Guid>,
        max_standing: Option<Guid>,
    },
    CompletedContractTags {
        include_when_sharing: bool,
        required_tags: Vec<Guid>,
        required_count: i32,
        excluded_tags: Vec<Guid>,
        excluded_count: i32,
    },
    /// A prereq kind our code doesn't know about. Holds the raw
    /// pointer so consumers with `Datacore` can walk via svarog.
    Unknown {
        struct_index: u32,
        instance_index: u32,
    },
}

/// Which `ContractGeneratorHandler_*` subtype owns this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandlerKind {
    Legacy,
    Career,
    List,
    LinearSeries,
    Tutorial,
    PVPBounty,
    ServiceBeacon,
    /// Handler kind we didn't classify (e.g. a new subtype in a
    /// future patch). Expansion continues but the kind is marked
    /// unknown so the census / merger can flag it.
    Unknown,
}

/// Where the expansion came from in the source record graph. Kept so
/// consumers with `Datacore` in hand can jump back to the raw
/// generated type if needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractOrigin {
    /// `Contract` record (career-handler intro lists and List /
    /// LinearSeries handlers use this shape).
    Contract,
    ContractLegacy,
    CareerContract,
    /// A sub-contract inside a parent `Contract` / `CareerContract` /
    /// `ContractLegacy`. `parent` is the parent record's GUID.
    SubContract {
        parent: Guid,
    },
}

/// A group of related ship-spawn waves emitted for a contract —
/// typically the `MissionPropertyValue_ShipSpawnDescriptions` attached
/// to one `missionVariableName` slot (e.g. `BP_SpawnDescriptions`,
/// `Hostile_ShipSpawnDescriptions`, …).
#[derive(Debug, Clone)]
pub struct EncounterGroup {
    /// `missionVariableName` the group was attached to — useful for
    /// classifying the group's role (`BP_SpawnDescriptions`,
    /// `MissionTargets`, `Hostile_*`, `WaveShips`, …).
    pub variable_name: String,
    /// Named sub-waves inside the group. For a bounty mission these
    /// are `Wave1 / Wave2 / Wave3`; for a salvage target they might
    /// be `SalvageableShip`, etc.
    pub waves: Vec<EncounterWave>,
}

/// One named wave inside an [`EncounterGroup`], plus its resolved
/// ship candidates and classified spawn context.
#[derive(Debug, Clone)]
pub struct EncounterWave {
    /// `SpawnDescription_ShipGroup.Name` — `Wave1`, `SalvageableShip`,
    /// sometimes empty.
    pub name: String,
    /// One slot per `SpawnDescription_ShipOptions` in the wave. Each
    /// slot represents a single ship picked per spawn; multiple slots
    /// means multiple ships concurrently.
    pub slots: Vec<EncounterSlot>,
}

/// A single ship slot inside a wave.
#[derive(Debug, Clone)]
pub struct EncounterSlot {
    /// `concurrentAmount` from `SpawnDescription_Ship` — how many
    /// ships this slot spawns at once.
    pub concurrent: i32,
    /// Pick weight relative to other options in the same slot.
    pub weight: f32,
    /// Ship candidates the slot can spawn, resolved via
    /// [`ShipRegistry::resolve_spawn`] with the intent-based filter.
    /// Empty when the query is broken (e.g. Gilly Mission01 Wave3's
    /// typo'd `Relient_Tana` tag) or awaits runtime location context.
    pub candidates: Vec<ShipCandidate>,
    /// Non-ship tags classified by [`SpawnContext`] — AI skill,
    /// faction, cargo descriptor, mission-role markers, etc.
    pub context: SpawnContext,
}

/// Materialised blueprint reward for a single contract.
#[derive(Debug, Clone)]
pub struct BlueprintReward {
    /// 0.0 – 1.0 chance the blueprint is awarded.
    pub chance: f32,
    /// GUID of the `BlueprintPoolRecord`.
    pub pool_guid: Guid,
    /// Pool's human-readable name (`BP_MISSIONREWARD_*`).
    pub pool_name: String,
    /// Resolved pool items. Empty when the pool was missing from
    /// [`BlueprintPoolRegistry`] (shouldn't happen on a clean DCB).
    pub items: Vec<crate::BlueprintItem>,
}

// ── Entry point ─────────────────────────────────────────────────────────────

/// Walk every `ContractGenerator` in the live DCB and produce one
/// [`ExpandedContract`] per (handler, contract, optional sub_contract)
/// node.
///
/// Order is stable across runs: generators iterate in
/// `RecordIndex` map order (by GUID), handlers in declaration order,
/// contracts in declaration order, sub-contracts in declaration order.
pub fn expand_all(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
) -> Vec<ExpandedContract> {
    let pools = &datacore.records().pools;
    let tree = &datacore.snapshot().tag_tree;
    let mut out = Vec::new();

    for (gen_guid, gen_handle) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gen_handle.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            walk_handler(
                datacore,
                locale,
                ships,
                blueprints,
                currency,
                localities,
                pools,
                tree,
                *gen_guid,
                handler_ptr,
                &mut out,
            );
        }
    }

    out
}

// ── Handler dispatch ────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn walk_handler(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    ptr: &ContractGeneratorHandlerBasePtr,
    out: &mut Vec<ExpandedContract>,
) {
    use ContractGeneratorHandlerBasePtr as H;
    match ptr {
        H::ContractGeneratorHandler_Legacy(h) => {
            let Some(handler) = h.get(pools) else { return };
            let ctx = HandlerCtx {
                kind: HandlerKind::Legacy,
                debug_name: handler.debug_name.clone(),
                contract_params: handler.contract_params.as_ref(),
                handler_availability: handler.default_availability.as_ref(),
            };
            for c_handle in &handler.legacy_contracts {
                if let Some(c) = c_handle.get(pools) {
                    emit_from_legacy(
                        datacore,
                        locale,
                        ships,
                        blueprints,
                        currency,
                        localities,
                        pools,
                        tree,
                        generator_id,
                        &ctx,
                        c,
                        out,
                    );
                }
            }
        }
        H::ContractGeneratorHandler_Career(h) => {
            let Some(handler) = h.get(pools) else { return };
            let ctx = HandlerCtx {
                kind: HandlerKind::Career,
                debug_name: handler.debug_name.clone(),
                contract_params: handler.contract_params.as_ref(),
                handler_availability: handler.default_availability.as_ref(),
            };
            for c_handle in &handler.contracts {
                if let Some(c) = c_handle.get(pools) {
                    emit_from_career(
                        datacore,
                        locale,
                        ships,
                        blueprints,
                        currency,
                        localities,
                        pools,
                        tree,
                        generator_id,
                        &ctx,
                        c,
                        out,
                    );
                }
            }
            for c_handle in &handler.intro_contracts {
                if let Some(c) = c_handle.get(pools) {
                    emit_from_contract(
                        datacore,
                        locale,
                        ships,
                        blueprints,
                        currency,
                        localities,
                        pools,
                        tree,
                        generator_id,
                        &ctx,
                        c,
                        out,
                    );
                }
            }
        }
        H::ContractGeneratorHandler_List(h) => emit_list_like(
            datacore,
            locale,
            ships,
            blueprints,
            currency,
            localities,
            pools,
            tree,
            generator_id,
            HandlerKind::List,
            h.get(pools).map(|h| ListLikeHandler {
                debug_name: &h.debug_name,
                contract_params: h.contract_params.as_ref(),
                default_availability: h.default_availability.as_ref(),
                contracts: &h.contracts,
            }),
            out,
        ),
        H::ContractGeneratorHandler_LinearSeries(h) => emit_list_like(
            datacore,
            locale,
            ships,
            blueprints,
            currency,
            localities,
            pools,
            tree,
            generator_id,
            HandlerKind::LinearSeries,
            h.get(pools).map(|h| ListLikeHandler {
                debug_name: &h.debug_name,
                contract_params: h.contract_params.as_ref(),
                default_availability: h.default_availability.as_ref(),
                contracts: &h.contracts,
            }),
            out,
        ),
        H::ContractGeneratorHandler_TutorialSeriesDef(h) => emit_list_like(
            datacore,
            locale,
            ships,
            blueprints,
            currency,
            localities,
            pools,
            tree,
            generator_id,
            HandlerKind::Tutorial,
            h.get(pools).map(|h| ListLikeHandler {
                debug_name: &h.debug_name,
                contract_params: h.contract_params.as_ref(),
                default_availability: h.default_availability.as_ref(),
                contracts: &h.contracts,
            }),
            out,
        ),
        H::ContractGeneratorHandler_PVPBountyDef(_) => {}
        H::ContractGeneratorHandler_ServiceBeacon(_) => {}
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_list_like<'p>(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    kind: HandlerKind,
    handler: Option<ListLikeHandler<'p>>,
    out: &mut Vec<ExpandedContract>,
) {
    let Some(ListLikeHandler {
        debug_name,
        contract_params,
        default_availability,
        contracts,
    }) = handler
    else {
        return;
    };
    let ctx = HandlerCtx {
        kind,
        debug_name: debug_name.clone(),
        contract_params,
        handler_availability: default_availability,
    };
    for c_handle in contracts {
        if let Some(c) = c_handle.get(pools) {
            emit_from_contract(
                datacore,
                locale,
                ships,
                blueprints,
                currency,
                localities,
                pools,
                tree,
                generator_id,
                &ctx,
                c,
                out,
            );
        }
    }
}

// ── Per-concrete-type emitters ──────────────────────────────────────────────

struct HandlerCtx<'p> {
    kind: HandlerKind,
    debug_name: String,
    contract_params: Option<&'p Handle<ContractParamOverrides>>,
    handler_availability: Option<&'p Handle<ContractAvailability>>,
}

/// Shared handler-shape alias — List / LinearSeries / Tutorial all have
/// `Vec<Handle<Contract>>` plus the common `debug_name` / params /
/// default-availability layout.
struct ListLikeHandler<'p> {
    debug_name: &'p String,
    contract_params: Option<&'p Handle<ContractParamOverrides>>,
    default_availability: Option<&'p Handle<ContractAvailability>>,
    contracts: &'p Vec<Handle<Contract>>,
}

#[allow(clippy::too_many_arguments)]
fn emit_from_contract(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &Contract,
    out: &mut Vec<ExpandedContract>,
) {
    let anchor = ContractAnchor::Contract(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    out.push(build_expansion(
        datacore,
        locale,
        ships,
        blueprints,
        currency,
        localities,
        pools,
        tree,
        generator_id,
        ctx,
        anchor,
        None,
        c.additional_prerequisites.as_slice(),
        contract_results,
        param_overrides,
        c.id,
        &c.debug_name,
        c.template,
        ContractOrigin::Contract,
    ));
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            out.push(build_expansion(
                datacore,
                locale,
                ships,
                blueprints,
                currency,
                localities,
                pools,
                tree,
                generator_id,
                ctx,
                anchor,
                Some(sub),
                c.additional_prerequisites.as_slice(),
                contract_results,
                param_overrides,
                sub.id,
                &c.debug_name,
                c.template,
                ContractOrigin::SubContract { parent: c.id },
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_from_legacy(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &ContractLegacy,
    out: &mut Vec<ExpandedContract>,
) {
    let anchor = ContractAnchor::ContractLegacy(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    out.push(build_expansion(
        datacore,
        locale,
        ships,
        blueprints,
        currency,
        localities,
        pools,
        tree,
        generator_id,
        ctx,
        anchor,
        None,
        c.additional_prerequisites.as_slice(),
        contract_results,
        param_overrides,
        c.id,
        &c.debug_name,
        c.template,
        ContractOrigin::ContractLegacy,
    ));
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            out.push(build_expansion(
                datacore,
                locale,
                ships,
                blueprints,
                currency,
                localities,
                pools,
                tree,
                generator_id,
                ctx,
                anchor,
                Some(sub),
                c.additional_prerequisites.as_slice(),
                contract_results,
                param_overrides,
                sub.id,
                &c.debug_name,
                c.template,
                ContractOrigin::SubContract { parent: c.id },
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_from_career(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &CareerContract,
    out: &mut Vec<ExpandedContract>,
) {
    let anchor = ContractAnchor::CareerContract(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    out.push(build_expansion(
        datacore,
        locale,
        ships,
        blueprints,
        currency,
        localities,
        pools,
        tree,
        generator_id,
        ctx,
        anchor,
        None,
        c.additional_prerequisites.as_slice(),
        contract_results,
        param_overrides,
        c.id,
        &c.debug_name,
        c.template,
        ContractOrigin::CareerContract,
    ));
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            out.push(build_expansion(
                datacore,
                locale,
                ships,
                blueprints,
                currency,
                localities,
                pools,
                tree,
                generator_id,
                ctx,
                anchor,
                Some(sub),
                c.additional_prerequisites.as_slice(),
                contract_results,
                param_overrides,
                sub.id,
                &c.debug_name,
                c.template,
                ContractOrigin::SubContract { parent: c.id },
            ));
        }
    }
}

// ── Core builder ────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_expansion(
    datacore: &Datacore,
    locale: &LocaleMap,
    ships: &ShipRegistry,
    blueprints: &BlueprintPoolRegistry,
    currency: &RewardCurrencyCatalog,
    localities: &LocalityRegistry,
    pools: &DataPools,
    tree: &sc_extract::TagTree,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    anchor: ContractAnchor<'_>,
    sub_contract: Option<&SubContract>,
    additional_prerequisites: &[ContractPrerequisiteBasePtr],
    contract_results: Option<&Handle<ContractResults>>,
    contract_param_overrides: Option<&Handle<ContractParamOverrides>>,
    id: Guid,
    debug_name: &str,
    template_guid: Option<Guid>,
    origin: ContractOrigin,
) -> ExpandedContract {
    let ResolvedText { title, description } =
        resolve_contract_text(sub_contract, anchor, ctx.contract_params, datacore, locale);
    let has_runtime_substitution = ResolvedText {
        title: title.clone(),
        description: description.clone(),
    }
    .has_runtime_substitution();

    let flags = resolve_flags_and_availability(
        pools,
        datacore,
        template_guid,
        sub_contract,
        contract_param_overrides,
        ctx.contract_params,
        ctx.handler_availability,
    );

    let blueprint_reward = resolve_blueprint_reward(pools, blueprints, contract_results);
    let (reward_uec, reward_scrip, reward_rep, reward_items, reward_other) =
        resolve_rewards(pools, datacore, currency, contract_results);

    let prerequisites = resolve_prerequisites(
        pools,
        additional_prerequisites,
        sub_contract,
        ctx.handler_availability,
    );

    let encounters = resolve_encounters(
        pools,
        ships,
        tree,
        sub_contract,
        contract_param_overrides,
        ctx.contract_params,
    );

    let mission_span = collect_mission_span(&prerequisites, localities);

    ExpandedContract {
        id,
        debug_name: debug_name.to_string(),
        generator_id,
        handler_kind: ctx.kind,
        handler_debug_name: ctx.debug_name.clone(),
        title,
        description,
        has_runtime_substitution,
        shareable: flags.shareable,
        illegal_flag: flags.illegal,
        availability: flags.availability,
        blueprint_reward,
        reward_uec,
        reward_scrip,
        reward_rep,
        reward_items,
        reward_other,
        prerequisites,
        encounters,
        mission_span,
        origin,
    }
}

/// Extract the unique Locality GUIDs from a prerequisite list (the
/// handler / contract / sub-contract chain), keeping only those the
/// registry resolved. First-seen-wins deduplication preserves prereq order.
fn collect_mission_span(prereqs: &[PrereqView], localities: &LocalityRegistry) -> Vec<Guid> {
    let mut seen: HashSet<Guid> = HashSet::new();
    let mut out: Vec<Guid> = Vec::new();
    for p in prereqs {
        if let PrereqView::Locality {
            locality: Some(g), ..
        } = p
            && localities.get(g).is_some()
            && seen.insert(*g)
        {
            out.push(*g);
        }
    }
    out
}

// ── Field resolvers ─────────────────────────────────────────────────────────

/// Resolved bool flags + availability bundle produced by
/// [`resolve_flags_and_availability`].
struct ResolvedFlags {
    shareable: bool,
    illegal: bool,
    availability: Availability,
}

/// Combined resolver for shareable / illegal / availability — they
/// all draw from the same bool + int param override inheritance
/// chain, plus `ActiveContractSettings.canBeShared` on the template.
///
/// Precedence (highest first): sub-contract → contract → handler.
/// Template's `canBeShared` is the base for shareable when no
/// bool-param override is present.
#[allow(clippy::too_many_arguments)]
fn resolve_flags_and_availability(
    pools: &DataPools,
    datacore: &Datacore,
    template_guid: Option<Guid>,
    sub: Option<&SubContract>,
    contract_params: Option<&Handle<ContractParamOverrides>>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> ResolvedFlags {
    // Base: handler's ContractAvailability (all the cooldown fields,
    // once_only, reaccept flags). If absent, everything stays at
    // Default::default() (all zero / false).
    let mut a = Availability::default();
    if let Some(h) = handler_availability
        && let Some(av) = h.get(pools)
    {
        fill_availability_from_base(&mut a, av);
    }

    // Shareable base: template.contractClass.additionalParams.canBeShared.
    let mut shareable = resolve_shareable_from_template(template_guid, datacore);
    let mut illegal = false;

    // Overlay bool + int param overrides from handler → contract → sub
    // (handler first so later ones win).
    for source in [handler_params, contract_params] {
        if let Some(h) = source
            && let Some(po) = h.get(pools)
        {
            apply_bool_overrides(
                pools,
                &po.bool_param_overrides,
                &mut a,
                &mut shareable,
                &mut illegal,
            );
            apply_int_overrides(pools, &po.int_param_overrides, &mut a);
        }
    }
    if let Some(sub) = sub {
        apply_bool_overrides(
            pools,
            &sub.bool_param_overrides,
            &mut a,
            &mut shareable,
            &mut illegal,
        );
        apply_int_overrides(pools, &sub.int_param_overrides, &mut a);
    }

    // Rebuild cooldowns from the (possibly updated) fields.
    a.cooldowns = Cooldowns {
        completion: if a.has_personal_cooldown {
            Some(DurationRange {
                mean_seconds: a_personal_mean(&a, pools, handler_availability),
                variation_seconds: a_personal_var(&a, pools, handler_availability),
            })
        } else {
            None
        },
        abandon: {
            let mean = a_abandon_mean(&a, pools, handler_availability);
            if mean > 0.0 {
                Some(DurationRange {
                    mean_seconds: mean,
                    variation_seconds: a_abandon_var(&a, pools, handler_availability),
                })
            } else {
                None
            }
        },
        fail: {
            // The DCB has no separate "fail cooldown" field; `can_reaccept_after_failing`
            // gates whether the abandon cooldown applies to failed missions too.
            // Surface the abandon cooldown here when fail-reaccept is allowed and
            // leave None otherwise so consumers see the distinction.
            if a.can_reaccept_after_failing {
                let mean = a_abandon_mean(&a, pools, handler_availability);
                if mean > 0.0 {
                    Some(DurationRange {
                        mean_seconds: mean,
                        variation_seconds: a_abandon_var(&a, pools, handler_availability),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        },
    };

    ResolvedFlags {
        shareable,
        illegal,
        availability: a,
    }
}

fn fill_availability_from_base(a: &mut Availability, base: &ContractAvailability) {
    a.once_only = base.once_only;
    a.can_reaccept_after_abandoning = base.can_reaccept_after_abandoning;
    a.can_reaccept_after_failing = base.can_reaccept_after_failing;
    a.has_personal_cooldown = base.has_personal_cooldown;
    a.hide_in_mobi_glas = base.hide_in_mobi_glas;
    a.available_in_prison = base.available_in_prison;
    a.notify_on_available = base.notify_on_available;
    a.max_players_per_instance = base.max_players_per_instance;
    // Cooldowns reassembled at the end after int-overrides land.
}

// Cooldown field getters — prefer the overridden field on Availability
// (reflected via int params) but fall back to the handler's base.
fn a_personal_mean(
    a: &Availability,
    pools: &DataPools,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> f32 {
    let _ = a;
    handler_availability
        .and_then(|h| h.get(pools))
        .map(|av| av.personal_cooldown_time)
        .unwrap_or(0.0)
}
fn a_personal_var(
    a: &Availability,
    pools: &DataPools,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> f32 {
    let _ = a;
    handler_availability
        .and_then(|h| h.get(pools))
        .map(|av| av.personal_cooldown_time_variation)
        .unwrap_or(0.0)
}
fn a_abandon_mean(
    a: &Availability,
    pools: &DataPools,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> f32 {
    let _ = a;
    handler_availability
        .and_then(|h| h.get(pools))
        .map(|av| av.abandoned_cooldown_time)
        .unwrap_or(0.0)
}
fn a_abandon_var(
    a: &Availability,
    pools: &DataPools,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> f32 {
    let _ = a;
    handler_availability
        .and_then(|h| h.get(pools))
        .map(|av| av.abandoned_cooldown_time_variation)
        .unwrap_or(0.0)
}

fn resolve_shareable_from_template(template_guid: Option<Guid>, datacore: &Datacore) -> bool {
    let Some(guid) = template_guid else {
        return false;
    };
    let db = datacore.db();
    let Some(record) = db.record(&guid) else {
        return false;
    };
    let inst = record.as_instance();
    let Some(cclass) = inst.get_instance("contractClass") else {
        return false;
    };
    if let Some(additional) = cclass.get_instance("additionalParams")
        && let Some(b) = additional.get_bool("canBeShared")
    {
        return b;
    }
    false
}

fn apply_bool_overrides(
    pools: &DataPools,
    overrides: &[Handle<ContractBoolParam>],
    a: &mut Availability,
    shareable: &mut bool,
    illegal: &mut bool,
) {
    for h in overrides {
        let Some(p) = h.get(pools) else { continue };
        use ContractBoolParamType as P;
        match p.param {
            P::OnceOnly => a.once_only = p.value,
            P::CanReacceptAfterAbandoning => a.can_reaccept_after_abandoning = p.value,
            P::CanReacceptAfterFailing => a.can_reaccept_after_failing = p.value,
            P::HasPersonalCooldown => a.has_personal_cooldown = p.value,
            P::HideInMobiGlas => a.hide_in_mobi_glas = p.value,
            P::NotifyOnAvailable => a.notify_on_available = p.value,
            P::CanBeShared => *shareable = p.value,
            P::Illegal => *illegal = p.value,
            _ => {}
        }
    }
}

fn apply_int_overrides(
    pools: &DataPools,
    overrides: &[Handle<ContractIntParam>],
    a: &mut Availability,
) {
    for h in overrides {
        let Some(p) = h.get(pools) else { continue };
        if matches!(p.param, ContractIntParamType::MaxPlayersPerInstance) {
            a.max_players_per_instance = p.value;
        }
    }
}

/// Classify every `ContractResultBasePtr` in the contract's results
/// into UEC / scrip / rep / items / other buckets.
fn resolve_rewards(
    pools: &DataPools,
    datacore: &Datacore,
    currency: &RewardCurrencyCatalog,
    results: Option<&Handle<ContractResults>>,
) -> (
    RewardAmount,
    Vec<ScripReward>,
    Vec<RepReward>,
    Vec<ItemReward>,
    Vec<OtherReward>,
) {
    let mut uec = RewardAmount::None;
    let mut scrip = Vec::new();
    let mut rep = Vec::new();
    let mut items = Vec::new();
    let mut other = Vec::new();

    let Some(results_handle) = results else {
        return (uec, scrip, rep, items, other);
    };
    let Some(results) = results_handle.get(pools) else {
        return (uec, scrip, rep, items, other);
    };
    let display_names = &datacore.snapshot().display_names;
    let records = &datacore.records().records;
    let db = datacore.db();

    use ContractResultBasePtr as R;
    for r in &results.contract_results {
        match r {
            R::ContractResult_CalculatedReward(_) => {
                // Engine-computed UEC. Upgrade to Calculated unless we
                // already have a Fixed amount (Fixed wins since it's
                // more specific).
                if !matches!(uec, RewardAmount::Fixed(_)) {
                    uec = RewardAmount::Calculated;
                }
            }
            R::ContractResult_Item(ih) => {
                let Some(item) = ih.get(pools) else { continue };
                let Some(ec) = item.entity_class else {
                    continue;
                };
                if let Some(info) = currency.get(&ec) {
                    scrip.push(ScripReward {
                        currency_guid: ec,
                        display_name: info.display_name.clone(),
                        record_name: info.record_name.clone(),
                        amount: item.amount,
                    });
                } else {
                    let name = display_names.get(&ec).unwrap_or("").to_string();
                    items.push(ItemReward {
                        entity_class: ec,
                        display_name: name,
                        amount: item.amount,
                    });
                }
            }
            R::ContractResult_LegacyReputation(lh) => {
                let Some(lrep) = lh.get(pools) else { continue };
                let Some(amounts_h) = lrep.contract_result_reputation_amounts.as_ref() else {
                    continue;
                };
                let Some(amounts) = amounts_h.get(pools) else {
                    continue;
                };
                // Resolve the `reward` GUID → SReputationRewardAmount.reputationAmount
                let amount = amounts
                    .reward
                    .as_ref()
                    .and_then(|g| {
                        records
                            .multi_feature
                            .sreputation_reward_amount
                            .get(g)
                            .copied()
                    })
                    .and_then(|h| h.get(pools))
                    .map(|rra| rra.reputation_amount);

                rep.push(RepReward {
                    faction: amounts.faction_reputation,
                    scope: amounts.reputation_scope,
                    amount,
                });
            }
            R::ContractResult_CalculatedReputation(ch) => {
                let Some(crep) = ch.get(pools) else { continue };
                rep.push(RepReward {
                    faction: crep.faction_reputation,
                    scope: crep.reputation_scope,
                    amount: None,
                });
            }
            R::BlueprintRewards(_) => {
                // Handled separately by resolve_blueprint_reward.
            }
            R::ContractResult_BadgeAward(_) => other.push(OtherReward::BadgeAward),
            R::ContractResult_ScenarioProgress(_) => other.push(OtherReward::ScenarioProgress),
            R::ContractResult_JournalEntry(_) => other.push(OtherReward::JournalEntry),
            R::ContractResult_CompletionTags(_) => other.push(OtherReward::CompletionTags),
            R::ContractResult_CompletionBounty(_) => other.push(OtherReward::CompletionBounty),
            R::ContractResult_ItemsWeighting(_) => other.push(OtherReward::ItemsWeighting),
            R::ContractResult_Reward(_) => other.push(OtherReward::Reward),
            R::ContractResult_RefundBuyIn(rh) => {
                if let Some(refund) = rh.get(pools) {
                    other.push(OtherReward::RefundBuyIn(refund.refund_multiplier));
                }
            }
            R::ContractResultBase(_) => {
                // Bare base — shouldn't occur in real data but stays valid.
            }
            R::Unknown {
                struct_index,
                instance_index,
            } => {
                let _ = db; // escape hatch — consumer can walk raw
                other.push(OtherReward::Unknown {
                    struct_index: *struct_index,
                    instance_index: *instance_index,
                });
            }
        }
    }

    (uec, scrip, rep, items, other)
}

/// Flatten every `ContractPrerequisiteBasePtr` reachable from this
/// contract's level + the handler's default-availability level into a
/// single `Vec<PrereqView>`. Order: handler first (inherited), then
/// contract, then sub-contract (most-specific last).
fn resolve_prerequisites(
    pools: &DataPools,
    additional_prerequisites: &[ContractPrerequisiteBasePtr],
    sub: Option<&SubContract>,
    handler_availability: Option<&Handle<ContractAvailability>>,
) -> Vec<PrereqView> {
    let mut out = Vec::new();

    if let Some(h) = handler_availability
        && let Some(av) = h.get(pools)
    {
        for p in &av.prerequisites {
            out.push(classify_prereq(pools, p));
        }
    }
    for p in additional_prerequisites {
        out.push(classify_prereq(pools, p));
    }
    if let Some(sub) = sub {
        for p in &sub.additional_prerequisites {
            out.push(classify_prereq(pools, p));
        }
    }

    out
}

fn classify_prereq(pools: &DataPools, p: &ContractPrerequisiteBasePtr) -> PrereqView {
    use ContractPrerequisiteBasePtr as P;
    match p {
        P::ContractPrerequisite_Locality(h) => h
            .get(pools)
            .map(|q: &ContractPrerequisite_Locality| PrereqView::Locality {
                locality: q.locality_available,
            })
            .unwrap_or(PrereqView::Locality { locality: None }),
        P::ContractPrerequisite_Location(h) => h
            .get(pools)
            .map(|q: &ContractPrerequisite_Location| PrereqView::Location {
                location: q.location_available,
            })
            .unwrap_or(PrereqView::Location { location: None }),
        P::ContractPrerequisite_LocationProperty(h) => h
            .get(pools)
            .map(
                |q: &ContractPrerequisite_LocationProperty| PrereqView::LocationProperty {
                    variable_name: q.property_variable_name.clone(),
                    extended_text_token: q.property_extended_text_token.clone(),
                    level: q.location_level_type.clone(),
                },
            )
            .unwrap_or(PrereqView::LocationProperty {
                variable_name: String::new(),
                extended_text_token: String::new(),
                level: ELocationTypeLevel::Unrecognized(String::new()),
            }),
        P::ContractPrerequisite_CrimeStat(h) => h
            .get(pools)
            .map(|q: &ContractPrerequisite_CrimeStat| PrereqView::CrimeStat {
                include_when_sharing: q.include_prerequisite_when_sharing,
                jurisdiction_override: q.crime_stat_jurisdiction_override,
                min: q.min_crime_stat,
                max: q.max_crime_stat,
            })
            .unwrap_or(PrereqView::CrimeStat {
                include_when_sharing: false,
                jurisdiction_override: None,
                min: 0.0,
                max: 0.0,
            }),
        P::ContractPrerequisite_Reputation(h) => h
            .get(pools)
            .map(
                |q: &ContractPrerequisite_Reputation| PrereqView::Reputation {
                    include_when_sharing: q.include_prerequisite_when_sharing,
                    faction: q.faction_reputation,
                    scope: q.scope,
                    exclude: q.exclude,
                    min_standing: q.min_standing,
                    max_standing: q.max_standing,
                },
            )
            .unwrap_or(PrereqView::Reputation {
                include_when_sharing: false,
                faction: None,
                scope: None,
                exclude: false,
                min_standing: None,
                max_standing: None,
            }),
        P::ContractPrerequisite_CompletedContractTags(h) => h
            .get(pools)
            .map(|q: &ContractPrerequisite_CompletedContractTags| {
                PrereqView::CompletedContractTags {
                    include_when_sharing: q.include_prerequisite_when_sharing,
                    required_tags: q
                        .required_completed_contract_tags
                        .as_ref()
                        .and_then(|h| h.get(pools))
                        .map(|tl| tl.tags.clone())
                        .unwrap_or_default(),
                    required_count: q.required_count_value,
                    excluded_tags: q
                        .excluded_completed_contract_tags
                        .as_ref()
                        .and_then(|h| h.get(pools))
                        .map(|tl| tl.tags.clone())
                        .unwrap_or_default(),
                    excluded_count: q.excluded_count_value,
                }
            })
            .unwrap_or(PrereqView::CompletedContractTags {
                include_when_sharing: false,
                required_tags: Vec::new(),
                required_count: 0,
                excluded_tags: Vec::new(),
                excluded_count: 0,
            }),
        P::Unknown {
            struct_index,
            instance_index,
        } => PrereqView::Unknown {
            struct_index: *struct_index,
            instance_index: *instance_index,
        },
        _ => PrereqView::Unknown {
            struct_index: 0,
            instance_index: 0,
        },
    }
}

/// Scan `contractResults.contract_results[]` for a `BlueprintRewards`
/// entry and resolve its pool through [`BlueprintPoolRegistry`].
fn resolve_blueprint_reward(
    pools: &DataPools,
    blueprints: &BlueprintPoolRegistry,
    results: Option<&Handle<ContractResults>>,
) -> Option<BlueprintReward> {
    let results_handle = results?;
    let results = results_handle.get(pools)?;
    for result_ptr in &results.contract_results {
        if let ContractResultBasePtr::BlueprintRewards(h) = result_ptr
            && let Some(br) = h.get(pools)
        {
            return materialise_blueprint(br, blueprints);
        }
    }
    None
}

/// Walk every `MissionPropertyValue_ShipSpawnDescriptions` reachable
/// through the contract's `paramOverrides.propertyOverrides` and the
/// sub-contract's `property_overrides`, plus the handler's
/// `contractParams.propertyOverrides` as the base layer. Sub and
/// contract layers append on top (they do not replace by name — the
/// DCB appears to allow multiple spawn groups in a single contract).
fn resolve_encounters(
    pools: &DataPools,
    ships: &ShipRegistry,
    tree: &sc_extract::TagTree,
    sub: Option<&SubContract>,
    contract_params: Option<&Handle<ContractParamOverrides>>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
) -> Vec<EncounterGroup> {
    let mut out: Vec<EncounterGroup> = Vec::new();

    // Handler-level base.
    if let Some(h) = handler_params
        && let Some(po) = h.get(pools)
    {
        collect_property_encounters(pools, ships, tree, &po.property_overrides, &mut out);
    }

    // Contract-level overrides append.
    if let Some(h) = contract_params
        && let Some(po) = h.get(pools)
    {
        collect_property_encounters(pools, ships, tree, &po.property_overrides, &mut out);
    }

    // Sub-contract overrides append last.
    if let Some(sub) = sub {
        collect_property_encounters(pools, ships, tree, &sub.property_overrides, &mut out);
    }

    out
}

fn collect_property_encounters(
    pools: &DataPools,
    ships: &ShipRegistry,
    tree: &sc_extract::TagTree,
    props: &[Handle<MissionProperty>],
    out: &mut Vec<EncounterGroup>,
) {
    for prop_h in props {
        let Some(prop) = prop_h.get(pools) else {
            continue;
        };
        let Some(value_ptr) = prop.value.as_ref() else {
            continue;
        };
        let BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(val_h) =
            value_ptr
        else {
            continue;
        };
        let Some(val) = val_h.get(pools) else {
            continue;
        };

        let mut group = EncounterGroup {
            variable_name: prop.mission_variable_name.clone(),
            waves: Vec::new(),
        };
        for group_h in &val.spawn_descriptions {
            let Some(wave_group) = group_h.get(pools) else {
                continue;
            };
            let mut wave = EncounterWave {
                name: wave_group.name.clone(),
                slots: Vec::new(),
            };
            for options_h in &wave_group.ships {
                let Some(options) = options_h.get(pools) else {
                    continue;
                };
                for option_h in &options.options {
                    let Some(option) = option_h.get(pools) else {
                        continue;
                    };
                    let pos = tag_list_guids(pools, option.tags.as_ref());
                    let neg = tag_list_guids(pools, option.negative_tags.as_ref());
                    let candidates = ships.resolve_spawn(&pos, &neg);
                    let context = SpawnContext::classify(tree, &pos);
                    wave.slots.push(EncounterSlot {
                        concurrent: option.concurrent_amount,
                        weight: option.weight,
                        candidates,
                        context,
                    });
                }
            }
            if !wave.slots.is_empty() {
                group.waves.push(wave);
            }
        }
        if !group.waves.is_empty() {
            out.push(group);
        }
    }
}

fn tag_list_guids(pools: &DataPools, h: Option<&Handle<TagList>>) -> HashSet<Guid> {
    let Some(h) = h else { return HashSet::new() };
    let Some(list) = h.get(pools) else {
        return HashSet::new();
    };
    list.tags.iter().copied().collect()
}

fn materialise_blueprint(
    br: &BlueprintRewards,
    blueprints: &BlueprintPoolRegistry,
) -> Option<BlueprintReward> {
    let pool_guid = br.blueprint_pool?;
    let Some(pool) = blueprints.get(&pool_guid) else {
        return Some(BlueprintReward {
            chance: br.chance,
            pool_guid,
            pool_name: String::new(),
            items: Vec::new(),
        });
    };
    Some(BlueprintReward {
        chance: br.chance,
        pool_guid,
        pool_name: pool.name.clone(),
        items: pool.items.clone(),
    })
}

// Silences unused import when no code path reaches it under a
// particular feature combination.
#[allow(dead_code)]
fn _anchor_class(_c: &ContractClass_Contract) {}
