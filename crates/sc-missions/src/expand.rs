//! Contract expansion — walk every `ContractGenerator` root and emit
//! one [`Mission`] per concrete (handler, contract,
//! optional sub_contract) node in the graph.
//!
//! This is stage 3 of the pipeline in `docs/sc-missions.md`. Stage 4
//! (merge into final [`crate::Contract`]s) groups these expansions by
//! `(title, description, reward_signature)`.
//!
//! The v1 expander materialises the fields sc-langpatch's existing
//! `mission_enhancer` needs to migrate off its svarog-instance walker:
//! resolved title + description, shareable / once-only / illegal flags,
//! and the blueprint reward (if any). Ship-encounter resolution and
//! the full reward model land in subsequent iterations.

use std::collections::{BTreeMap, HashSet};

use sc_extract::generated::{
    BaseDataSetMatchConditionPtr, BaseMissionPropertyValuePtr, BlueprintRewards, CareerContract,
    Contract, ContractAvailability, ContractBoolParam, ContractBoolParamType,
    ContractClass_Contract, ContractClassBasePtr, ContractDifficultyProfile,
    ContractGeneratorHandlerBasePtr, ContractIntParam, ContractIntParamType, ContractLegacy,
    ContractParamOverrides, ContractPrerequisite_CompletedContractTags,
    ContractPrerequisite_CrimeStat, ContractPrerequisite_Locality, ContractPrerequisite_Location,
    ContractPrerequisite_LocationProperty, ContractPrerequisite_Reputation,
    ContractPrerequisiteBasePtr, ContractResultBasePtr, ContractResults, ContractTemplate,
    DataPools, ELocationTypeLevel, Handle, MissionProperty, SubContract,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap};

use crate::axes::{AxisDiff, SharedTag};
use crate::classify::TagBag;
use crate::currency::RewardCurrencies;
use crate::locality::Localities;
use crate::ships::{ShipCandidate, Ships};
use crate::titles::{ContractAnchor, ResolvedKeys, resolve_contract_keys};

/// One concrete contract node in the expanded generator graph.
///
/// A contract with three sub-contract tiers produces three
/// `Mission`s (one per tier, each with the tier's overridden
/// fields). A contract with no sub-contracts produces one.
#[derive(Debug, Clone)]
pub struct Mission {
    /// GUID of the contract (or sub-contract — see
    /// [`MissionOrigin::subcontract_of`]).
    pub id: Guid,
    /// Internal debug name from the contract record. Useful for
    /// cross-referencing with SCMDB and the DCB.
    pub debug_name: String,

    /// Where the mission came from in the DCB graph — handler kind,
    /// debug name, owning generator, optional parent for sub-contracts.
    pub origin: MissionOrigin,

    /// INI key the title was resolved from. Raw — leading `@`
    /// preserved, matching the workspace localization rule
    /// (`docs/localization.md`). `Some` whenever a key was found in the
    /// inheritance chain, even if the active [`LocaleMap`] doesn't
    /// carry a translation. Resolve via [`Mission::title`].
    pub title_key: Option<LocaleKey>,
    /// INI key the description was resolved from. Same shape and
    /// semantics as [`Self::title_key`]. Resolve via
    /// [`Mission::description`].
    pub description_key: Option<LocaleKey>,

    /// GUID of the `ContractTemplate` this expansion resolved from, when one
    /// is referenced. Carried so consumers can reach template-only data
    /// (e.g. the category — see [`Self::category`]). `None` for the rare
    /// contract with no template.
    pub template_id: Option<Guid>,
    /// Mission category — the `MissionType` record GUID from the template's
    /// `contractDisplayInfo.type` (SCMDB's "Mission Type" column: Bounty
    /// Hunter / Hauling / Mercenary / Salvage / …). Resolve the name + icon
    /// via [`crate::MissionTypes`]. `None` when the template carries no
    /// display info / type.
    pub category: Option<Guid>,
    /// The mission's reputation faction (SCMDB's "Faction"). The rep-*reward*
    /// faction is primary (the faction whose standing you gain); falls back to
    /// the first rep-*prerequisite* faction (the gating faction). A
    /// `FactionReputation` GUID — resolve the name via [`crate::FactionReputations`].
    /// `None` when the mission touches no reputation faction.
    pub faction: Option<Guid>,
    /// Difficulty profile — the four `ContractDifficulty` axes (each a level
    /// `1..=8`) plus the profile reference + weights. Drives the computed
    /// aUEC payout (the evergr3n-formula inputs) and is the hidden axis the
    /// visible payout sorts by. `None` when the contract carries no
    /// `ContractResults.difficulty`.
    pub difficulty: Option<Difficulty>,
    /// `ContractResults.contractBuyInAmount` — upfront cost to accept (0 when
    /// free). A payout-formula input and a displayable cost.
    pub buy_in: i32,
    /// `ContractResults.timeToComplete` minutes — the contract's time budget
    /// (0 when none). A payout-formula input. (Stored as a bare `Single` in
    /// minutes; not seconds — verified against the DCB raw values.)
    pub time_to_complete: f32,

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

    /// All reward outputs grouped together: UEC, scrip, reputation,
    /// items, blueprint pool, miscellaneous. See [`MissionRewards`]
    /// for per-axis details.
    pub rewards: MissionRewards,

    /// Additional prerequisites — unified view across
    /// `Contract.additional_prerequisites`,
    /// `SubContract.additional_prerequisites`, and
    /// `ContractAvailability.prerequisites` (handler-level).
    /// Resolved into a typed enum; unknown kinds preserve the raw
    /// `(struct_index, instance_index)` pointer.
    pub prerequisites: Vec<PrereqView>,

    /// Encounters attached to this mission — one [`Encounter`] per
    /// spawn-shaped `MissionProperty.value` (Ship / NPC / Entity).
    /// Pattern-match for typed access; `Encounter::Unknown` carries
    /// the raw GUID for any unmodeled poly variant. Order preserves
    /// DCB-array order (sub-contract → contract.paramOverrides →
    /// handler → template, first-non-empty per slot).
    pub encounters: Vec<Encounter>,

    /// Localities this expansion is offered at. Every `Locality`
    /// prereq in the expansion's inheritance chain (handler +
    /// contract + sub-contract) produces one entry, deduplicated by
    /// `locality_guid` and resolved through
    /// [`Localities`]. Unresolvable GUIDs are dropped.
    ///
    /// Order follows prereq walking order: handler first (inherited),
    /// then contract, then sub-contract (most-specific last), with
    /// first-seen-wins deduplication.
    pub mission_span: Vec<Guid>,

    /// Completion tags this mission **grants** on success
    /// (`ContractResult_CompletionTags`). These are the edges of the mission
    /// *chain* graph: a tag granted here can satisfy another mission's
    /// `CompletedContractTags` prerequisite. Deduplicated, in grant order. Use
    /// [`crate::Missions::prerequisite_missions`] to walk a mission's
    /// required tags back to the missions that grant them.
    pub grants_completion_tags: Vec<Guid>,

    /// Resolved non-spawn `MissionProperty` values, keyed by the player-facing
    /// `extendedTextToken` — the same token a `~mission(Token|Fmt)` marker in
    /// the title/description references. Built from the handler → contract →
    /// sub-contract override layers (most-specific wins). Spawn-shaped values
    /// (ship / npc / entity) are surfaced as [`Self::encounters`] instead; the
    /// runtime-only location queries are not captured here yet. Used by the
    /// marker substitutor ([`crate::Missions::title_text`]).
    pub variables: BTreeMap<String, MissionVar>,
}

/// A resolved non-spawn `MissionProperty` value (see [`Mission::variables`]).
/// The engine picks a final value at spawn from the (possibly gated) options;
/// a static view exposes the option set so consumers can show a single value or
/// a "drawn from" candidate list.
#[derive(Debug, Clone, PartialEq)]
pub enum MissionVar {
    /// A `MissionPropertyValue_StringHash` — one or more label options. Resolve
    /// each option's [`VarOption::label_key`] against a `LocaleMap` at the call
    /// site. Used by cargo-grade / rank-style tokens.
    Choice(Vec<VarOption>),
    /// A `MissionPropertyValue_Integer` — one or more numeric options (e.g.
    /// `MissionMaxSCUSize`, `ScripAmount`). Used by quantity tokens.
    Number(Vec<i64>),
    /// A `MissionPropertyValue_Location(s)` query. The concrete place is chosen
    /// at spawn from tag-defined archetypes, so a static view can only expose
    /// the query's tag facets — the system(s) and setting(s) the `TagSearch`
    /// targets (e.g. `systems=["Nyx"]`, `settings=["Space"]`).
    Location {
        /// System names from the query's `…/System/<X>` positive tags.
        systems: Vec<String>,
        /// Location-setting names from `…/LocationSetting/<X>` positive tags.
        settings: Vec<String>,
    },
}

/// One option of a [`MissionVar::Choice`] StringHash value.
#[derive(Debug, Clone, PartialEq)]
pub struct VarOption {
    /// `textId` — the player-facing label key. Resolve against a `LocaleMap`.
    pub label_key: LocaleKey,
    /// `value` — the raw string the engine stores (a locale key or id token).
    /// Kept for diagnostics / matching; the label is the display value.
    pub value: String,
    /// True when this option carries `dependentProperties` — i.e. it only fires
    /// under a runtime condition (held reputation, chosen cargo). When several
    /// options are gated the static value is a candidate set, not a guarantee.
    pub gated: bool,
}

impl Mission {
    /// Resolve this mission's title against `locale`. Returns `None`
    /// when no title key was found in the inheritance chain or the
    /// key is absent from `locale`.
    pub fn title<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.title_key.as_ref().and_then(|k| locale.resolve(k))
    }

    /// Resolve this mission's description against `locale`. Same
    /// shape as [`Self::title`].
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        self.description_key
            .as_ref()
            .and_then(|k| locale.resolve(k))
    }

    /// True if either the resolved title or description contains a
    /// `~mission(...)` runtime substitution marker the engine fills
    /// in at spawn time. Computed against `locale` so language packs
    /// that normalise marker syntax are reflected. `false` when no
    /// title/description text is available.
    pub fn has_runtime_substitution(&self, locale: &LocaleMap) -> bool {
        let has = |s: Option<&str>| s.map(|t| t.contains("~mission(")).unwrap_or(false);
        has(self.title(locale)) || has(self.description(locale))
    }

    /// `(min, max)` ship count summed across every ship-spawn slot
    /// group on the mission. Per-group min = smallest option's
    /// `concurrent`; per-group max = largest option's `concurrent`.
    /// Entity groups contribute their `amount` range. NPC counts are
    /// not included (they're surfaced separately by consumers).
    ///
    /// Returns `(0, 0)` when the mission has no ship or entity
    /// encounters.
    pub fn ship_count_range(&self) -> (i32, i32) {
        let mut min = 0i32;
        let mut max = 0i32;
        for enc in &self.encounters {
            match enc {
                Encounter::Ships(s) => {
                    for phase in &s.phases {
                        for group in &phase.groups {
                            min += group.concurrent_range.0;
                            max += group.concurrent_range.1;
                        }
                    }
                }
                Encounter::Entities(s) => {
                    for phase in &s.phases {
                        for group in &phase.groups {
                            min += group.concurrent_range.0;
                            max += group.concurrent_range.1;
                        }
                    }
                }
                Encounter::Npcs(_) | Encounter::Unknown { .. } => {}
            }
        }
        (min, max)
    }

    /// CombatClass tag shared by every alternative across every
    /// ship-encounter slot group on this mission. `Some("VeryEasy")`
    /// for a mission whose every spawn alternative carries that
    /// CombatClass tag. `None` when the mission spans multiple
    /// CombatClasses or has no CombatClass tags on its alternatives.
    ///
    /// Renderers use this for a one-line difficulty banner
    /// (`"Encounters · 2-4 ships · VeryEasy"`) and to suppress
    /// per-group CombatClass annotations when redundant.
    ///
    /// Walks every ship-encounter [`SlotGroup`]'s `shared_tags`
    /// (which carry their [`crate::AxisKind`] classification). When
    /// every group's CombatClass shared-tag name agrees, returns
    /// that. When groups disagree or some groups have no CombatClass
    /// shared tag at all, returns `None`.
    pub fn combat_class(&self) -> Option<&str> {
        use crate::AxisKind;
        let mut seen: Option<&str> = None;
        for enc in &self.encounters {
            let Encounter::Ships(s) = enc else { continue };
            for phase in &s.phases {
                for group in &phase.groups {
                    // Groups without any CombatClass tag don't vote on
                    // the mission tier — many "Target" / "Allied" slot
                    // groups carry only faction + hull tags and no
                    // difficulty marker. Only groups whose shared
                    // alternatives explicitly carry a CombatClass tag
                    // contribute to the consensus.
                    let Some(class) = group
                        .shared_tags
                        .iter()
                        .find(|t| t.kind == AxisKind::CombatClass)
                        .map(|t| t.name.as_str())
                    else {
                        continue;
                    };
                    match seen {
                        None => seen = Some(class),
                        Some(prev) if prev == class => {}
                        Some(_) => return None,
                    }
                }
            }
        }
        seen
    }
}

/// Where a [`Mission`] came from. Consolidates v1's separate
/// `handler_kind`, `handler_debug_name`, `generator_id`, and origin
/// enum into one struct with `subcontract_of: Option<Guid>` carrying
/// the parent reference for sub-contracts.
#[derive(Debug, Clone)]
pub struct MissionOrigin {
    /// Typed handler kind — which `ContractGeneratorHandler_*` shape
    /// owns this mission.
    pub kind: HandlerKind,
    /// `debugName` on the handler. Useful for grouping missions that
    /// share a generator family (`TarPits_Unlawful_Salvage_Career` →
    /// all TarPits salvage missions).
    pub source_debug_name: String,
    /// GUID of the owning `ContractGenerator` record.
    pub generator_id: Guid,
    /// `None` for top-level missions. `Some(parent_id)` when this
    /// mission is a sub-contract — `parent_id` is the parent
    /// mission's `id`. Same information v1 carried in the
    /// `ContractOrigin::SubContract { parent_id }` enum variant.
    pub subcontract_of: Option<Guid>,
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

/// All reward outputs for one contract / mission, grouped.
///
/// Replaces the six top-level reward fields v1 carried directly on
/// `Contract` (and on `Mission`). Consumers that want to
/// query a single axis read `rewards.uec` / `rewards.scrip` / etc.;
/// consumers that want "any reward at all" can iterate one struct
/// instead of six.
#[derive(Debug, Clone, Default)]
pub struct MissionRewards {
    /// UEC reward. Most contracts use [`RewardAmount::Calculated`]
    /// (engine-computed at runtime); occasional contracts pay a
    /// [`RewardAmount::Fixed`] aUEC amount directly.
    pub uec: RewardAmount,
    /// Scrip rewards — `ContractResult_Item` entries whose entity_class
    /// is in [`crate::RewardCurrencies`].
    pub scrip: Vec<ScripReward>,
    /// Reputation rewards. Amount is [`None`] for
    /// `ContractResult_CalculatedReputation` (engine-computed) and
    /// [`Some(i32)`] for `_LegacyReputation` with a resolved amount.
    pub reputation: Vec<RepReward>,
    /// Item rewards — `ContractResult_Item` entries whose entity_class
    /// is **not** a currency (ship unlocks, collector items, …).
    pub items: Vec<ItemReward>,
    /// Blueprint pool rewards attached to this contract. Each entry is
    /// a thin reference (`{ chance, pool_guid }`) — resolve the pool's
    /// name + items via [`crate::BlueprintPools::get`] at the
    /// call site.
    ///
    /// Empty when the contract awards no blueprints. Multiple entries
    /// when the contract has multiple `BlueprintRewards` blocks in its
    /// `contractResults` (multi-pool missions).
    pub blueprints: Vec<BlueprintReward>,
    /// Other reward kinds (BadgeAward, ScenarioProgress, JournalEntry,
    /// CompletionTag(s), CompletionBounty, RefundBuyIn, ItemsWeighting,
    /// Reward). Detailed field modelling deferred until a consumer
    /// needs them.
    pub other: Vec<OtherReward>,
}

impl MissionRewards {
    /// True when no reward of any kind is set.
    pub fn is_empty(&self) -> bool {
        matches!(self.uec, RewardAmount::None)
            && self.scrip.is_empty()
            && self.reputation.is_empty()
            && self.items.is_empty()
            && self.blueprints.is_empty()
            && self.other.is_empty()
    }
}

/// The contract's difficulty profile — four authored skill axes plus the
/// weighting profile they're combined under.
///
/// Each axis is the engine's `EDifficultyRange_*` enum, whose variant name
/// ends in its numeric level (`Hands_free_gaming_1` … `No_content_like_this_yet_8`),
/// so [`Self::mechanical_skill`] etc. are that `1..=8` level (`0` if the value
/// didn't parse). Players never see difficulty directly — it drives the
/// **computed aUEC payout**, which is the visible signal, so a consumer
/// splitting missions by payout is implicitly splitting by this. The four
/// levels + [`Self::weights`] + buy-in + time are the inputs the (community-
/// derived "evergr3n") reward formula consumes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Difficulty {
    pub mechanical_skill: u8,
    pub mental_load: u8,
    pub risk_of_loss: u8,
    pub game_knowledge: u8,
    /// `ContractDifficulty.difficultyProfile` reference — a stable, hashable
    /// grouping key (two missions on the same profile + levels compute the
    /// same payout). `None` when the difficulty carries no profile.
    pub profile: Option<Guid>,
    /// `[mechanicalSkill, mentalLoad, riskOfLoss, gameKnowledge]` weights from
    /// the referenced `ContractDifficultyProfile`. `None` when the profile
    /// didn't resolve. Carried for the reward estimator (Part B).
    pub weights: Option<[f32; 4]>,
}

/// How a reward amount is delivered.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
///
/// Display name is intentionally absent — resolve via
/// [`crate::RewardCurrencies::display_name`] at the call site.
#[derive(Debug, Clone)]
pub struct ScripReward {
    pub currency_guid: Guid,
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
///
/// Display name is intentionally absent — resolve through
/// [`sc_items::Items`] keyed by `entity_class` and the
/// active [`LocaleMap`].
#[derive(Debug, Clone)]
pub struct ItemReward {
    pub entity_class: Guid,
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

/// One encounter attached to a mission — Phase 6 of the v2 redesign
/// widens the model from ship-only to all three spawn-shaped poly
/// variants on `BaseMissionPropertyValuePtr`.
///
/// Pattern-match on the variant for typed access; iterate uniformly
/// when consumers don't care about the shape (e.g. counting total
/// encounters per mission). The [`Self::Unknown`] variant carries the
/// raw `MissionProperty` GUID so consumers can walk the underlying
/// instance via `datacore.db()` if a future poly variant lands.
#[derive(Debug, Clone)]
pub enum Encounter {
    /// Ship spawn description — by far the most common shape (~3,045
    /// of 4,678 spawn-encoded properties on SC 4.7 LIVE).
    Ships(ShipEncounter),
    /// NPC spawn description — FPS-side encounters. Carries the typed
    /// `mission_allied_marker` flag from `AutoSpawnSettings` that v1
    /// crimestat work needs.
    Npcs(NpcEncounter),
    /// Generic entity spawn description — ~316 properties on SC 4.7,
    /// covers entity-shaped spawn buckets that aren't ship-or-NPC.
    Entities(EntityEncounter),
    /// Unmodeled poly variant — carries the raw `MissionProperty`
    /// GUID so the consumer can walk the underlying instance via
    /// `datacore.db().record(raw_guid)`.
    Unknown {
        variable_name: String,
        raw_guid: Guid,
    },
}

impl Encounter {
    /// `mission_variable_name` of the underlying `MissionProperty`,
    /// regardless of which spawn variant it carries.
    pub fn variable_name(&self) -> &str {
        match self {
            Encounter::Ships(s) => &s.variable_name,
            Encounter::Npcs(s) => &s.variable_name,
            Encounter::Entities(s) => &s.variable_name,
            Encounter::Unknown { variable_name, .. } => variable_name,
        }
    }

    /// Raw `extendedTextToken` from the underlying `MissionProperty`.
    /// Empty for [`Self::Unknown`].
    pub fn extended_text_token(&self) -> &str {
        match self {
            Encounter::Ships(s) => &s.extended_text_token,
            Encounter::Npcs(s) => &s.extended_text_token,
            Encounter::Entities(s) => &s.extended_text_token,
            Encounter::Unknown { .. } => "",
        }
    }
}

/// Ship spawn encounter — the original v1 shape, now wrapped in
/// [`Encounter::Ships`].
#[derive(Debug, Clone)]
pub struct ShipEncounter {
    /// `missionVariableName` the property was attached to.
    pub variable_name: String,
    /// `MissionProperty.extendedTextToken` — almost always empty in
    /// 4.7 (96.3%); occasional values include `AmbushTarget`,
    /// `CargoShip`. Forwarded raw for consumer inspection.
    pub extended_text_token: String,
    /// Ordered phases — `Wave1 / Wave2 / Wave3` in combat-gauntlet
    /// missions, `SalvageableShip` for salvage targets, often empty.
    /// Phase order matches DCB-array order; consumers that want a
    /// specific display order sort themselves.
    pub phases: Vec<EncounterPhase<ShipSlot>>,
}

/// NPC (FPS) spawn encounter.
#[derive(Debug, Clone)]
pub struct NpcEncounter {
    pub variable_name: String,
    pub extended_text_token: String,
    pub phases: Vec<EncounterPhase<NpcSlot>>,
}

/// Generic entity spawn encounter. Less constrained than
/// [`ShipEncounter`] / [`NpcEncounter`] — used for spawn buckets that
/// don't fit the ship or NPC mold.
#[derive(Debug, Clone)]
pub struct EntityEncounter {
    pub variable_name: String,
    pub extended_text_token: String,
    pub phases: Vec<EncounterPhase<EntitySlot>>,
}

/// One named phase inside an encounter, plus its slot groups. Generic
/// over the slot type so all three encounter kinds share the same shape.
///
/// Names like `Wave1` / `Reinforcements` / `SalvageableShip` come
/// from `SpawnDescription_*Group.Name` directly — see
/// [`crate::Mission`] doc for the wave-name reliability caveat.
///
/// Each phase contains one or more [`SlotGroup`]s. All groups in a
/// phase fire **concurrently** when the phase triggers. Inside a
/// single group the engine picks **one** of the `options` based on
/// runtime state (player profile / weight RNG / party size). See the
/// [`SlotGroup`] doc for the full rationale.
#[derive(Debug, Clone)]
pub struct EncounterPhase<S> {
    /// `SpawnDescription_*Group.Name` — sometimes empty, sometimes
    /// `Wave1`, sometimes a flavour name like `Allies` (which the
    /// encounter_analytics audit showed can mislead — civilians in
    /// Ambush missions are flagged as `Allies` despite not being
    /// combat allies).
    pub name: String,
    /// Groups in this phase. Each group fires concurrently with the
    /// others when the phase triggers. Within a single group the
    /// engine picks one of the inner [`SlotGroup::options`].
    pub groups: Vec<SlotGroup<S>>,
}

/// A set of weighted alternatives for a single concurrent slot — the
/// engine picks **one** option per spawn.
///
/// Maps directly to `SpawnDescription_ShipOptions` (and the analogous
/// `SpawnDescription_EntityOptions`) in the DCB. The
/// `SpawnDescription_NPCGroup` shape doesn't have this nested
/// alternatives layer, so NPC `SlotGroup`s always have exactly one
/// option. For ships, ~14% of groups have more than one option in
/// the LIVE DCB (see `docs/feature-request-encounter-alternatives.md`
/// for the full census).
///
/// When `options.len() == 1`, the group is a "pure concurrent slot"
/// and the option's `concurrent` / `amount` is exactly what fires.
/// When `options.len() > 1`, the engine picks one option — display
/// code should render alternatives as a range or surface meaningful
/// variance axes (see [`AxisDiff`] and [`Self::axes`]).
///
/// # Selection rule
///
/// The picker is opaque to static analysis. Observed behaviour:
/// `weight` biases the pick (skewed weights like 5:1 imply
/// percentage-based selection); uniform weights with concurrent
/// scaling (1,2,3 ships) imply selection by player skill rating;
/// alternatives that differ only on hull pick by RNG. Display code
/// should NEVER pretend it knows which option fires — render the
/// range or the alternatives honestly.
#[derive(Debug, Clone)]
pub struct SlotGroup<S> {
    /// The weighted alternatives. Engine picks one per spawn.
    pub options: Vec<S>,
    /// `(min, max)` of the count field across `options`. For ship
    /// groups this is `concurrent`; for entity groups it's `amount`.
    /// NPC groups always have one option so min == max.
    pub concurrent_range: (i32, i32),
    /// True when every option's `weight` is the same (within 1e-4).
    /// Renderers showing percentages can skip them when this is true.
    pub weight_uniform: bool,
    /// Per-axis classification of how the `options`' tag sets vary.
    /// Populated by [`AxisDiff::compute`] from each option's positive
    /// tag list. Empty axes when `options.len() < 2`.
    pub axes: AxisDiff,
    /// Tags present on EVERY option's positive tag list — the
    /// "implied context" for this group. Each entry carries its
    /// [`crate::AxisKind`] classification so consumers can scan for
    /// e.g. the CombatClass shared across alternatives without
    /// re-walking the tag tree. Sorted by tag name for deterministic
    /// display. Empty when `options.len() == 0`.
    pub shared_tags: Vec<SharedTag>,
}

impl<S> SlotGroup<S> {
    /// Convenience: true when there's only one option (no real choice
    /// to display).
    pub fn is_singleton(&self) -> bool {
        self.options.len() <= 1
    }
}

impl<S> EncounterPhase<S> {
    /// Flat iterator over every option across every group in this
    /// phase. **Loses the alternatives boundary** — only use this for
    /// "give me everything" walks (debugging, census, raw dumps). For
    /// any display or filtering work, walk [`Self::groups`] directly
    /// so the engine's pick-one-per-group semantics stay visible.
    pub fn all_options(&self) -> impl Iterator<Item = &S> {
        self.groups.iter().flat_map(|g| g.options.iter())
    }
    /// Total option count across all groups in this phase.
    pub fn option_count(&self) -> usize {
        self.groups.iter().map(|g| g.options.len()).sum()
    }
}

/// A single ship slot inside an encounter phase.
///
/// The four [`TagBag`]s correspond directly to the four `TagList`
/// fields on the underlying `SpawnDescription_Ship`. They're surfaced
/// symmetrically so consumers can read or classify any of them with
/// the same shape — see [`TagBag`] for classifier methods.
#[derive(Debug, Clone)]
pub struct ShipSlot {
    /// `concurrentAmount` from `SpawnDescription_Ship` — how many
    /// ships this slot spawns at once.
    pub concurrent: i32,
    /// Pick weight relative to other options in the same slot.
    pub weight: f32,
    /// Ship candidates the slot can spawn, resolved via
    /// [`Ships::resolve_spawn`] with the intent-based filter.
    /// Empty when the query is broken (e.g. Gilly Mission01 Wave3's
    /// typo'd `Relient_Tana` tag) or awaits runtime location context.
    pub candidates: Vec<ShipCandidate>,
    /// `SpawnDescription_Ship.initialDamageSettings` — when set, the
    /// ship spawns pre-damaged. Strong typed signal in `4.7` for two
    /// distinct mission types: salvage missions (scrape / fracture
    /// the ship itself) and cargo-recovery missions (collect cargo
    /// from a damaged ship). Mission type disambiguates the role —
    /// the flag itself doesn't. 4% of ship-spawn options carry it.
    /// Forwarded as the raw record GUID; resolution to a damage
    /// profile is consumer-side.
    pub initial_damage_settings: Option<Guid>,
    /// `SpawnDescription_Ship.includeLocationAISpawnTags` — when true
    /// the engine merges in the spawn location's AI-spawn tags at
    /// runtime, so static tag analysis is incomplete for this slot.
    pub include_location_ai_spawn_tags: bool,
    /// `SpawnDescription_Ship.tags` — the slot's positive tag query.
    /// Drives ship-candidate resolution; classifier methods on
    /// [`TagBag`] surface AI skill, factions, cargo descriptors,
    /// spawn identifiers, mission tags, directives, and role-state
    /// predicates (`is_cargo_recovery`, `is_pre_damaged_wreck`,
    /// `is_salvage_target`).
    pub positive: TagBag,
    /// `SpawnDescription_Ship.negativeTags` — tags that *exclude*
    /// matches. Newly visible in v2; consumed by [`Ships`]
    /// during resolution but kept here so consumers can audit what
    /// the slot explicitly rejected.
    pub negative: TagBag,
    /// `SpawnDescription_Ship.markupTags` — UI markup hints
    /// (`Medium` difficulty marker). Populated on 1.7% of ship-spawn
    /// options in 4.7.
    pub markup: TagBag,
    /// `SpawnDescription_Ship.entityTags` — populated on 2.9% of
    /// options in 4.7. Semantics not yet pinned down; forwarded for
    /// consumer inspection.
    pub entity: TagBag,
}

/// A single NPC slot inside an encounter phase.
///
/// Surfaces the high-value typed fields from `SpawnDescription_NPCOption`
/// and `AutoSpawnSettings` — notably [`Self::mission_allied_marker`],
/// the typed Allied / Hostile signal v1 crimestat work needs. Detailed
/// FPS scope tags (closet / room / defendArea / scheduleArea) are not
/// surfaced yet — they belong in a future v3 follow-up driven by an
/// actual FPS consumer.
#[derive(Debug, Clone)]
pub struct NpcSlot {
    /// `priority` from `SpawnDescription_NPCOption`. Used by the
    /// engine for spawn-priority ordering.
    pub priority: i32,
    /// Pick weight relative to other options in the same phase.
    pub weight: f32,
    /// `SpawnDescription_NPCOption.includeLocationAISpawnTags`.
    pub include_location_ai_spawn_tags: bool,
    /// `AutoSpawnSettings.missionAlliedMarker` — when true, the spawn
    /// carries the friendly HUD marker. Critical signal for crimestat
    /// risk: a friendly without a marker means the player can't
    /// distinguish them from an enemy at a glance. `false` (and
    /// `false` when the slot has no `auto_spawn_settings`) is the
    /// default.
    pub mission_allied_marker: bool,
    /// `AutoSpawnSettings.isCritical` — engine-level "this NPC must
    /// spawn" flag. False when no `auto_spawn_settings` is attached.
    pub is_critical: bool,
    /// `AutoSpawnSettings.factionOverride` — when set, overrides the
    /// NPC's faction at spawn time. None when no override or no
    /// `auto_spawn_settings`.
    pub faction_override: Option<Guid>,
    /// `SpawnDescription_NPCOption.identifierTags` — typed tag bag
    /// that classifies the NPC archetype (`Civilian`, `Marine`, etc.).
    pub identifier_tags: TagBag,
}

/// A single generic-entity slot inside an encounter phase.
///
/// `SpawnDescription_Entity` carries the same four tag lists as
/// `SpawnDescription_Ship` plus an `amount` count — but no candidate
/// resolution and no per-slot location-tag merge flag.
#[derive(Debug, Clone)]
pub struct EntitySlot {
    /// `amount` from `SpawnDescription_Entity` — how many entities
    /// this slot spawns. Different name from ship's `concurrent`
    /// because the engine uses different fields; preserved literally.
    pub amount: i32,
    /// Pick weight relative to other options in the same slot.
    pub weight: f32,
    /// `SpawnDescription_Entity.tags` — positive tag query.
    pub positive: TagBag,
    /// `SpawnDescription_Entity.negativeTags`.
    pub negative: TagBag,
    /// `SpawnDescription_Entity.markupTags`.
    pub markup: TagBag,
    /// `SpawnDescription_Entity.entityTags`.
    pub entity: TagBag,
}

/// Reference to a blueprint pool awarded by a single contract.
///
/// Slim by design — only the chance and the pool GUID. Consumers
/// resolve the pool's name and items via
/// [`crate::BlueprintPools::get`] at render time. Keeps the
/// mission model decoupled from the (potentially large) item list and
/// keeps one source of truth for pool contents.
///
/// A mission with multiple `BlueprintRewards` entries in its
/// `contractResults` produces multiple `BlueprintReward` rows in
/// [`MissionRewards::blueprints`].
#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintReward {
    /// 0.0 – 1.0 chance the blueprint is awarded.
    pub chance: f32,
    /// GUID of the `BlueprintPoolRecord`. Look up via
    /// [`crate::BlueprintPools::get`] for name + items.
    pub pool_guid: Guid,
}

// ── Entry point ─────────────────────────────────────────────────────────────

/// Walk every `ContractGenerator` in the live DCB and produce one
/// [`Mission`] per (handler, contract, optional sub_contract)
/// node.
///
/// Order is stable across runs: generators iterate in
/// `RecordIndex` map order (by GUID), handlers in declaration order,
/// contracts in declaration order, sub-contracts in declaration order.
pub fn expand_all(
    datacore: &Datacore,
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    tree: &sc_tags::Tags,
) -> Vec<Mission> {
    let pools = &datacore.records().pools;
    let mut out = Vec::new();

    for (gen_guid, gen_handle) in &datacore.records().records.multi_feature.contract_generator {
        let Some(generator) = gen_handle.get(pools) else {
            continue;
        };
        for handler_ptr in &generator.generators {
            walk_handler(
                datacore,
                ships,
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
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
    generator_id: Guid,
    ptr: &ContractGeneratorHandlerBasePtr,
    out: &mut Vec<Mission>,
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
                        ships,
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
            let career_faction = handler.faction_reputation;
            let career_scope = handler.reputation_scope;
            for c_handle in &handler.contracts {
                if let Some(c) = c_handle.get(pools) {
                    emit_from_career(
                        datacore,
                        ships,
                        currency,
                        localities,
                        pools,
                        tree,
                        generator_id,
                        &ctx,
                        c,
                        career_faction,
                        career_scope,
                        out,
                    );
                }
            }
            for c_handle in &handler.intro_contracts {
                if let Some(c) = c_handle.get(pools) {
                    emit_from_contract(
                        datacore,
                        ships,
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
            ships,
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
            ships,
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
            ships,
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
        H::ContractGeneratorHandler_ServiceBeacon(h) => emit_list_like(
            datacore,
            ships,
            currency,
            localities,
            pools,
            tree,
            generator_id,
            HandlerKind::ServiceBeacon,
            h.get(pools).map(|h| ListLikeHandler {
                debug_name: &h.debug_name,
                contract_params: h.contract_params.as_ref(),
                default_availability: h.default_availability.as_ref(),
                // ServiceBeacon names its contract vec `serviceBeaconContracts`
                // but the shape is the same list-like handler.
                contracts: &h.service_beacon_contracts,
            }),
            out,
        ),
        H::ContractGeneratorHandler_PVPBountyDef(_) => {}
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_list_like<'p>(
    datacore: &Datacore,
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
    generator_id: Guid,
    kind: HandlerKind,
    handler: Option<ListLikeHandler<'p>>,
    out: &mut Vec<Mission>,
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
                ships,
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
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &Contract,
    out: &mut Vec<Mission>,
) {
    let anchor = ContractAnchor::Contract(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    out.push(build_expansion(
        datacore,
        ships,
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
        None, /*Contract*/
    ));
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            out.push(build_expansion(
                datacore,
                ships,
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
                Some(c.id),
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_from_legacy(
    datacore: &Datacore,
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &ContractLegacy,
    out: &mut Vec<Mission>,
) {
    let anchor = ContractAnchor::ContractLegacy(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    out.push(build_expansion(
        datacore,
        ships,
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
        None, /*Legacy*/
    ));
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            out.push(build_expansion(
                datacore,
                ships,
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
                Some(c.id),
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_from_career(
    datacore: &Datacore,
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
    generator_id: Guid,
    ctx: &HandlerCtx<'_>,
    c: &CareerContract,
    // Career rep gate lives on the handler (`factionReputation` / `reputationScope`)
    // + the contract (`minStanding` / `maxStanding`), not as a prerequisite.
    career_faction: Option<Guid>,
    career_scope: Option<Guid>,
    out: &mut Vec<Mission>,
) {
    let anchor = ContractAnchor::CareerContract(c);
    let contract_results = c.contract_results.as_ref();
    let param_overrides = c.param_overrides.as_ref();

    let mut m = build_expansion(
        datacore,
        ships,
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
        None, /*Career*/
    );
    apply_career_rep(
        &mut m,
        career_faction,
        career_scope,
        c.min_standing,
        c.max_standing,
    );
    out.push(m);
    for sub_h in &c.sub_contracts {
        if let Some(sub) = sub_h.get(pools) {
            let mut m = build_expansion(
                datacore,
                ships,
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
                Some(c.id),
            );
            apply_career_rep(
                &mut m,
                career_faction,
                career_scope,
                c.min_standing,
                c.max_standing,
            );
            out.push(m);
        }
    }
}

/// Stamp a career contract's reputation gate onto the mission. Career missions
/// carry their rep requirement as the handler's `factionReputation` plus the
/// contract's `minStanding` / `maxStanding` (a `Reference`, not a
/// `ContractPrerequisite_Reputation`), so it's surfaced here as a synthetic
/// [`PrereqView::Reputation`] — only when a standing bound is present (a real
/// gate). The faction also backfills [`Mission::faction`] when nothing else set it.
fn apply_career_rep(
    m: &mut Mission,
    faction: Option<Guid>,
    scope: Option<Guid>,
    min_standing: Option<Guid>,
    max_standing: Option<Guid>,
) {
    if min_standing.is_some() || max_standing.is_some() {
        m.prerequisites.push(PrereqView::Reputation {
            include_when_sharing: false,
            faction,
            scope,
            exclude: false,
            min_standing,
            max_standing,
        });
    }
    if m.faction.is_none() {
        m.faction = faction;
    }
}

// ── Core builder ────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_expansion(
    datacore: &Datacore,
    ships: &Ships,
    currency: &RewardCurrencies,
    localities: &Localities,
    pools: &DataPools,
    tree: &sc_tags::Tags,
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
    subcontract_of: Option<Guid>,
) -> Mission {
    let ResolvedKeys {
        title_key,
        description_key,
    } = resolve_contract_keys(sub_contract, anchor, ctx.contract_params, datacore);

    let flags = resolve_flags_and_availability(
        pools,
        datacore,
        template_guid,
        sub_contract,
        contract_param_overrides,
        ctx.contract_params,
        ctx.handler_availability,
    );

    let blueprints = resolve_blueprint_rewards(pools, contract_results);
    let (uec, scrip, reputation, items, other) =
        resolve_rewards(pools, datacore, currency, contract_results);
    let rewards = MissionRewards {
        uec,
        scrip,
        reputation,
        items,
        blueprints,
        other,
    };

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

    let variables = resolve_variables(
        pools,
        tree,
        sub_contract,
        contract_param_overrides,
        ctx.contract_params,
    );

    let mission_span = collect_mission_span(&prerequisites, localities);

    let category = resolve_category(template_guid, datacore);
    let faction = resolve_faction(&rewards, &prerequisites);
    let (difficulty, buy_in, time_to_complete) = resolve_meta(pools, datacore, contract_results);
    let grants_completion_tags = resolve_granted_completion_tags(pools, contract_results);

    let origin = MissionOrigin {
        kind: ctx.kind,
        source_debug_name: ctx.debug_name.clone(),
        generator_id,
        subcontract_of,
    };
    Mission {
        id,
        debug_name: debug_name.to_string(),
        origin,
        title_key,
        description_key,
        template_id: template_guid,
        category,
        faction,
        difficulty,
        buy_in,
        time_to_complete,
        shareable: flags.shareable,
        illegal_flag: flags.illegal,
        availability: flags.availability,
        rewards,
        prerequisites,
        encounters,
        mission_span,
        grants_completion_tags,
        variables,
    }
}

/// Collect the completion tags a contract **grants** — every
/// `ContractResult_CompletionTags` entry's per-tag `tag` reference, across the
/// contract's `ContractResults`. Sorted + deduplicated. Empty when the
/// contract grants none (or has no results).
fn resolve_granted_completion_tags(
    pools: &DataPools,
    results: Option<&Handle<ContractResults>>,
) -> Vec<Guid> {
    let mut seen: HashSet<Guid> = HashSet::new();
    let mut out: Vec<Guid> = Vec::new();
    let Some(results) = results.and_then(|h| h.get(pools)) else {
        return out;
    };
    for r in &results.contract_results {
        if let ContractResultBasePtr::ContractResult_CompletionTags(h) = r
            && let Some(ct) = h.get(pools)
        {
            for tag_h in &ct.completion_tags {
                if let Some(tag) = tag_h.get(pools).and_then(|t| t.tag)
                    && seen.insert(tag)
                {
                    out.push(tag);
                }
            }
        }
    }
    out
}

/// Mission category — walk `template → contractDisplayInfo → type`. Returns
/// the `MissionType` record GUID (resolve its name/icon via
/// [`crate::MissionTypes`]). `None` when the template, its display info, or
/// the type reference is absent.
fn resolve_category(template_guid: Option<Guid>, datacore: &Datacore) -> Option<Guid> {
    let guid = template_guid?;
    let pools = &datacore.records().pools;
    let template = datacore.resolve::<ContractTemplate>(&guid)?;
    let display_info = template.contract_display_info.as_ref()?.get(pools)?;
    display_info.r#type
}

/// The mission's reputation faction (SCMDB's "Faction"). Prefers the
/// rep-*reward* faction (the faction whose standing you earn); falls back to
/// the first rep-*prerequisite* faction (the gating faction). `None` when no
/// reputation faction is touched.
fn resolve_faction(rewards: &MissionRewards, prereqs: &[PrereqView]) -> Option<Guid> {
    rewards
        .reputation
        .iter()
        .find_map(|r| r.faction)
        .or_else(|| {
            prereqs.iter().find_map(|p| match p {
                PrereqView::Reputation { faction, .. } => *faction,
                _ => None,
            })
        })
}

/// Resolve the payout-formula inputs from the contract's `ContractResults`:
/// the [`Difficulty`] profile (levels + weights), the buy-in, and the time
/// budget. All default to empty when there are no results.
fn resolve_meta(
    pools: &DataPools,
    datacore: &Datacore,
    results: Option<&Handle<ContractResults>>,
) -> (Option<Difficulty>, i32, f32) {
    let Some(results) = results.and_then(|h| h.get(pools)) else {
        return (None, 0, 0.0);
    };
    let difficulty = results
        .difficulty
        .as_ref()
        .and_then(|h| h.get(pools))
        .map(|d| {
            let profile = d.difficulty_profile;
            let weights = profile
                .and_then(|g| datacore.resolve::<ContractDifficultyProfile>(&g))
                .map(|p| {
                    [
                        p.mechanical_skill_weight,
                        p.mental_load_weight,
                        p.risk_of_loss_weight,
                        p.game_knowledge_weight,
                    ]
                });
            Difficulty {
                mechanical_skill: difficulty_level(d.mechanical_skill.as_dcb_str()),
                mental_load: difficulty_level(d.mental_load.as_dcb_str()),
                risk_of_loss: difficulty_level(d.risk_of_loss.as_dcb_str()),
                game_knowledge: difficulty_level(d.game_knowledge.as_dcb_str()),
                profile,
                weights,
            }
        });
    (
        difficulty,
        results.contract_buy_in_amount,
        results.time_to_complete,
    )
}

/// Parse the trailing `_N` level off an `EDifficultyRange_*` DCB string
/// (`"Easy_PvE_only_action_3"` → `3`). `0` when there's no numeric suffix
/// (e.g. an `Unrecognized` value).
fn difficulty_level(dcb: &str) -> u8 {
    dcb.rsplit('_')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Extract the unique Locality GUIDs from a prerequisite list (the
/// handler / contract / sub-contract chain), keeping only those the
/// registry resolved. First-seen-wins deduplication preserves prereq order.
fn collect_mission_span(prereqs: &[PrereqView], localities: &Localities) -> Vec<Guid> {
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
    // Typed walk: template (Reference) → contractClass poly → its
    // additionalParams → canBeShared. `canBeShared` only exists on the
    // ContractClass_Contract subclass; every other variant (ServiceBeacon,
    // PVPBounty, GlobalEvent, base) lacks the field, so a non-match means
    // "not shareable".
    let pools = &datacore.records().pools;
    let Some(template) = datacore.resolve::<ContractTemplate>(&guid) else {
        return false;
    };
    let Some(ContractClassBasePtr::ContractClass_Contract(cclass)) = &template.contract_class
    else {
        return false;
    };
    cclass
        .get(pools)
        .and_then(|cc| cc.additional_params)
        .and_then(|ap| ap.get(pools))
        .map(|settings| settings.can_be_shared)
        .unwrap_or(false)
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
    currency: &RewardCurrencies,
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
                        record_name: info.record_name.clone(),
                        amount: item.amount,
                    });
                } else {
                    items.push(ItemReward {
                        entity_class: ec,
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
/// entry and resolve its pool through [`BlueprintPools`].
/// Collect every `BlueprintRewards` entry under this contract's
/// `ContractResults`. Returns an empty vec when there are no blueprint
/// rewards (or when `results` is `None`).
///
/// Multi-pool missions have multiple `BlueprintRewards` entries side
/// by side in `contract_results`; previously this resolver returned on
/// the first hit and silently dropped the rest. Now every entry that
/// has a non-null `blueprint_pool` GUID becomes a [`BlueprintReward`].
/// Entries with no pool GUID are skipped (the DCB does carry a few).
fn resolve_blueprint_rewards(
    pools: &DataPools,
    results: Option<&Handle<ContractResults>>,
) -> Vec<BlueprintReward> {
    let mut out: Vec<BlueprintReward> = Vec::new();
    let Some(results_handle) = results else {
        return out;
    };
    let Some(results) = results_handle.get(pools) else {
        return out;
    };
    for result_ptr in &results.contract_results {
        if let ContractResultBasePtr::BlueprintRewards(h) = result_ptr
            && let Some(br) = h.get(pools)
            && let Some(reward) = materialise_blueprint(br)
        {
            out.push(reward);
        }
    }
    out
}

/// Walk every `MissionPropertyValue_ShipSpawnDescriptions` reachable
/// through the contract's `paramOverrides.propertyOverrides` and the
/// sub-contract's `property_overrides`, plus the handler's
/// `contractParams.propertyOverrides` as the base layer. Sub and
/// contract layers append on top (they do not replace by name — the
/// DCB appears to allow multiple spawn groups in a single contract).
fn resolve_encounters(
    pools: &DataPools,
    ships: &Ships,
    tree: &sc_tags::Tags,
    sub: Option<&SubContract>,
    contract_params: Option<&Handle<ContractParamOverrides>>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
) -> Vec<Encounter> {
    let mut out: Vec<Encounter> = Vec::new();

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
    ships: &Ships,
    tree: &sc_tags::Tags,
    props: &[Handle<MissionProperty>],
    out: &mut Vec<Encounter>,
) {
    for prop_h in props {
        let Some(prop) = prop_h.get(pools) else {
            continue;
        };
        let Some(value_ptr) = prop.value.as_ref() else {
            continue;
        };
        let var_name = prop.mission_variable_name.clone();
        let ext_token = prop.extended_text_token.clone();
        match value_ptr {
            BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(val_h) => {
                if let Some(enc) =
                    build_ship_encounter(pools, ships, tree, val_h, &var_name, &ext_token)
                {
                    out.push(Encounter::Ships(enc));
                }
            }
            BaseMissionPropertyValuePtr::MissionPropertyValue_NPCSpawnDescriptions(val_h) => {
                if let Some(enc) = build_npc_encounter(pools, tree, val_h, &var_name, &ext_token) {
                    out.push(Encounter::Npcs(enc));
                }
            }
            BaseMissionPropertyValuePtr::MissionPropertyValue_EntitySpawnDescriptions(val_h) => {
                if let Some(enc) = build_entity_encounter(pools, tree, val_h, &var_name, &ext_token)
                {
                    out.push(Encounter::Entities(enc));
                }
            }
            // Non-spawn variants are skipped — they're property values, not
            // encounters. Phase 6 doesn't classify them.
            _ => {}
        }
    }
}

/// Walk every non-spawn `MissionProperty` reachable through the handler →
/// contract → sub-contract override layers, collecting its value into a map
/// keyed by `extendedTextToken`. Most-specific layer wins (later overwrites),
/// matching how the engine resolves a `~mission(Token)` marker. Spawn-shaped
/// values (ship / npc / entity) are handled by [`resolve_encounters`] instead.
fn resolve_variables(
    pools: &DataPools,
    tree: &sc_tags::Tags,
    sub: Option<&SubContract>,
    contract_params: Option<&Handle<ContractParamOverrides>>,
    handler_params: Option<&Handle<ContractParamOverrides>>,
) -> BTreeMap<String, MissionVar> {
    let mut out = BTreeMap::new();
    for params in [handler_params, contract_params] {
        if let Some(h) = params
            && let Some(po) = h.get(pools)
        {
            collect_property_vars(pools, tree, &po.property_overrides, &mut out);
        }
    }
    if let Some(sub) = sub {
        collect_property_vars(pools, tree, &sub.property_overrides, &mut out);
    }
    out
}

fn collect_property_vars(
    pools: &DataPools,
    tree: &sc_tags::Tags,
    props: &[Handle<MissionProperty>],
    out: &mut BTreeMap<String, MissionVar>,
) {
    for prop_h in props {
        let Some(prop) = prop_h.get(pools) else {
            continue;
        };
        if prop.extended_text_token.is_empty() {
            continue;
        }
        let Some(value_ptr) = prop.value.as_ref() else {
            continue;
        };
        let var = match value_ptr {
            BaseMissionPropertyValuePtr::MissionPropertyValue_StringHash(h) => {
                h.get(pools).map(|v| {
                    MissionVar::Choice(
                        v.options
                            .iter()
                            .filter_map(|o| o.get(pools))
                            .map(|o| VarOption {
                                label_key: o.text_id.clone(),
                                value: o.value.clone(),
                                gated: !o.dependent_properties.is_empty(),
                            })
                            .collect(),
                    )
                })
            }
            BaseMissionPropertyValuePtr::MissionPropertyValue_Integer(h) => h.get(pools).map(|v| {
                MissionVar::Number(
                    v.options
                        .iter()
                        .filter_map(|o| o.get(pools))
                        .map(|o| o.value as i64)
                        .collect(),
                )
            }),
            // Location query → static tag facets (system / setting). The plural
            // `_Locations` carries the same `matchConditions` shape.
            BaseMissionPropertyValuePtr::MissionPropertyValue_Location(h) => h
                .get(pools)
                .and_then(|v| location_var(pools, tree, &v.match_conditions)),
            BaseMissionPropertyValuePtr::MissionPropertyValue_Locations(h) => h
                .get(pools)
                .and_then(|v| location_var(pools, tree, &v.match_conditions)),
            // Float / AIName / spawn / org variants aren't captured here: Float
            // is rare, spawns are encounters, names are runtime generators.
            _ => None,
        };
        if let Some(var) = var {
            let empty = match &var {
                MissionVar::Choice(o) => o.is_empty(),
                MissionVar::Number(n) => n.is_empty(),
                MissionVar::Location { systems, settings } => {
                    systems.is_empty() && settings.is_empty()
                }
            };
            if !empty {
                // Most-specific layer wins.
                out.insert(prop.extended_text_token.clone(), var);
            }
        }
    }
}

/// Reduce a location query's `matchConditions` to its static tag facets — the
/// system(s) and setting(s) named by every `TagSearch` positive tag. The
/// concrete place is a runtime pick; these facets are the most a static view
/// can say (e.g. "Nyx · Space"). Returns `None` when no facet tag resolves.
fn location_var(
    pools: &DataPools,
    tree: &sc_tags::Tags,
    conditions: &[BaseDataSetMatchConditionPtr],
) -> Option<MissionVar> {
    let mut systems: Vec<String> = Vec::new();
    let mut settings: Vec<String> = Vec::new();
    for cond in conditions {
        let BaseDataSetMatchConditionPtr::DataSetMatchCondition_TagSearch(h) = cond else {
            continue;
        };
        let Some(ts) = h.get(pools) else { continue };
        for term_h in &ts.tag_search {
            let Some(term) = term_h.get(pools) else {
                continue;
            };
            for tag in &term.positive_tags {
                let path = tree.path(tag);
                push_facet(&path, "System", &mut systems);
                push_facet(&path, "LocationSetting", &mut settings);
            }
        }
    }
    if systems.is_empty() && settings.is_empty() {
        None
    } else {
        Some(MissionVar::Location { systems, settings })
    }
}

/// Push the tag-path segment immediately after `marker` (e.g. the `Nyx` in
/// `…/System/Nyx`) into `out`, deduplicated.
fn push_facet(path: &[&str], marker: &str, out: &mut Vec<String>) {
    if let Some(pos) = path.iter().position(|s| *s == marker)
        && let Some(name) = path.get(pos + 1)
    {
        let v = (*name).to_string();
        if !out.contains(&v) {
            out.push(v);
        }
    }
}

fn build_ship_encounter(
    pools: &DataPools,
    ships: &Ships,
    tree: &sc_tags::Tags,
    val_h: &Handle<sc_extract::generated::MissionPropertyValue_ShipSpawnDescriptions>,
    var_name: &str,
    ext_token: &str,
) -> Option<ShipEncounter> {
    let val = val_h.get(pools)?;
    let mut phases: Vec<EncounterPhase<ShipSlot>> = Vec::new();
    for group_h in &val.spawn_descriptions {
        let Some(wave_group) = group_h.get(pools) else {
            continue;
        };
        let mut phase = EncounterPhase {
            name: wave_group.name.clone(),
            groups: Vec::new(),
        };
        for options_h in &wave_group.ships {
            let Some(options) = options_h.get(pools) else {
                continue;
            };
            let mut group_options: Vec<ShipSlot> = Vec::new();
            for option_h in &options.options {
                let Some(option) = option_h.get(pools) else {
                    continue;
                };
                let positive = TagBag::from_handle(pools, tree, option.tags.as_ref());
                let negative = TagBag::from_handle(pools, tree, option.negative_tags.as_ref());
                let markup = TagBag::from_handle(pools, tree, option.markup_tags.as_ref());
                let entity = TagBag::from_handle(pools, tree, option.entity_tags.as_ref());
                let pos_set: HashSet<Guid> = positive.guids.iter().copied().collect();
                let neg_set: HashSet<Guid> = negative.guids.iter().copied().collect();
                let candidates = ships.resolve_spawn(&pos_set, &neg_set);
                group_options.push(ShipSlot {
                    concurrent: option.concurrent_amount,
                    weight: option.weight,
                    candidates,
                    initial_damage_settings: option.initial_damage_settings,
                    include_location_ai_spawn_tags: option.include_location_aispawn_tags,
                    positive,
                    negative,
                    markup,
                    entity,
                });
            }
            if let Some(group) = build_ship_group(group_options, tree) {
                phase.groups.push(group);
            }
        }
        if !phase.groups.is_empty() {
            phases.push(phase);
        }
    }
    if phases.is_empty() {
        return None;
    }
    Some(ShipEncounter {
        variable_name: var_name.to_string(),
        extended_text_token: ext_token.to_string(),
        phases,
    })
}

/// Assemble a [`SlotGroup`] from a flat option list. Computes the
/// per-axis [`AxisDiff`] and the convenience min/max ranges.
fn build_ship_group(options: Vec<ShipSlot>, tree: &sc_tags::Tags) -> Option<SlotGroup<ShipSlot>> {
    if options.is_empty() {
        return None;
    }
    let per_option_tags: Vec<Vec<Guid>> = options
        .iter()
        .map(|opt| opt.positive.guids.clone())
        .collect();
    let (axes, shared_tags) = AxisDiff::compute(&per_option_tags, tree);
    let min = options.iter().map(|o| o.concurrent).min().unwrap_or(0);
    let max = options.iter().map(|o| o.concurrent).max().unwrap_or(0);
    let weight_uniform = options
        .iter()
        .all(|o| (o.weight - options[0].weight).abs() < 1e-4);
    Some(SlotGroup {
        options,
        concurrent_range: (min, max),
        weight_uniform,
        axes,
        shared_tags,
    })
}

fn build_npc_encounter(
    pools: &DataPools,
    tree: &sc_tags::Tags,
    val_h: &Handle<sc_extract::generated::MissionPropertyValue_NPCSpawnDescriptions>,
    var_name: &str,
    ext_token: &str,
) -> Option<NpcEncounter> {
    let val = val_h.get(pools)?;
    let mut phases: Vec<EncounterPhase<NpcSlot>> = Vec::new();
    for group_h in &val.spawn_descriptions {
        let Some(npc_group) = group_h.get(pools) else {
            continue;
        };
        let mut phase = EncounterPhase {
            name: npc_group.name.clone(),
            groups: Vec::new(),
        };
        // NPCs have no nested alternatives layer (no `options` inside
        // each `option`) — every `SpawnDescription_NPCOption` becomes
        // its own singleton SlotGroup. Keeping the shape uniform with
        // ship/entity encounters means consumers don't have to special-
        // case NPC encounters when walking groups.
        for option_h in &npc_group.options {
            let Some(option) = option_h.get(pools) else {
                continue;
            };
            // Pull the high-level fields from AutoSpawnSettings if attached.
            let (mission_allied_marker, is_critical, faction_override) = option
                .auto_spawn_settings
                .as_ref()
                .and_then(|h| h.get(pools))
                .map(|s| (s.mission_allied_marker, s.is_critical, s.faction_override))
                .unwrap_or((false, false, None));
            let identifier_tags = TagBag::from_handle(pools, tree, option.identifier_tags.as_ref());
            let slot = NpcSlot {
                priority: option.priority,
                weight: option.weight,
                include_location_ai_spawn_tags: option.include_location_aispawn_tags,
                mission_allied_marker,
                is_critical,
                faction_override,
                identifier_tags,
            };
            let per_option_tags: Vec<Vec<Guid>> = vec![slot.identifier_tags.guids.clone()];
            let (axes, shared_tags) = AxisDiff::compute(&per_option_tags, tree);
            phase.groups.push(SlotGroup {
                options: vec![slot],
                concurrent_range: (1, 1),
                weight_uniform: true,
                axes,
                shared_tags,
            });
        }
        if !phase.groups.is_empty() {
            phases.push(phase);
        }
    }
    if phases.is_empty() {
        return None;
    }
    Some(NpcEncounter {
        variable_name: var_name.to_string(),
        extended_text_token: ext_token.to_string(),
        phases,
    })
}

fn build_entity_encounter(
    pools: &DataPools,
    tree: &sc_tags::Tags,
    val_h: &Handle<sc_extract::generated::MissionPropertyValue_EntitySpawnDescriptions>,
    var_name: &str,
    ext_token: &str,
) -> Option<EntityEncounter> {
    let val = val_h.get(pools)?;
    let mut phases: Vec<EncounterPhase<EntitySlot>> = Vec::new();
    for group_h in &val.spawn_descriptions {
        let Some(ent_group) = group_h.get(pools) else {
            continue;
        };
        let mut phase = EncounterPhase {
            name: ent_group.name.clone(),
            groups: Vec::new(),
        };
        for options_h in &ent_group.entities {
            let Some(options) = options_h.get(pools) else {
                continue;
            };
            let mut group_options: Vec<EntitySlot> = Vec::new();
            for option_h in &options.options {
                let Some(option) = option_h.get(pools) else {
                    continue;
                };
                group_options.push(EntitySlot {
                    amount: option.amount,
                    weight: option.weight,
                    positive: TagBag::from_handle(pools, tree, option.tags.as_ref()),
                    negative: TagBag::from_handle(pools, tree, option.negative_tags.as_ref()),
                    markup: TagBag::from_handle(pools, tree, option.markup_tags.as_ref()),
                    entity: TagBag::from_handle(pools, tree, option.entity_tags.as_ref()),
                });
            }
            if let Some(group) = build_entity_group(group_options, tree) {
                phase.groups.push(group);
            }
        }
        if !phase.groups.is_empty() {
            phases.push(phase);
        }
    }
    if phases.is_empty() {
        return None;
    }
    Some(EntityEncounter {
        variable_name: var_name.to_string(),
        extended_text_token: ext_token.to_string(),
        phases,
    })
}

fn build_entity_group(
    options: Vec<EntitySlot>,
    tree: &sc_tags::Tags,
) -> Option<SlotGroup<EntitySlot>> {
    if options.is_empty() {
        return None;
    }
    let per_option_tags: Vec<Vec<Guid>> =
        options.iter().map(|o| o.positive.guids.clone()).collect();
    let (axes, shared_tags) = AxisDiff::compute(&per_option_tags, tree);
    let min = options.iter().map(|o| o.amount).min().unwrap_or(0);
    let max = options.iter().map(|o| o.amount).max().unwrap_or(0);
    let weight_uniform = options
        .iter()
        .all(|o| (o.weight - options[0].weight).abs() < 1e-4);
    Some(SlotGroup {
        options,
        concurrent_range: (min, max),
        weight_uniform,
        axes,
        shared_tags,
    })
}

/// Lift a raw `BlueprintRewards` DCB entry into the slim
/// [`BlueprintReward`] reference. Returns `None` when the entry has
/// no `blueprint_pool` GUID set — those carry no actionable info.
///
/// The pool's name and items are not looked up here; consumers resolve
/// them via [`crate::BlueprintPools::get`] at render time.
fn materialise_blueprint(br: &BlueprintRewards) -> Option<BlueprintReward> {
    let pool_guid = br.blueprint_pool?;
    Some(BlueprintReward {
        chance: br.chance,
        pool_guid,
    })
}

// Silences unused import when no code path reaches it under a
// particular feature combination.
#[allow(dead_code)]
fn _anchor_class(_c: &ContractClass_Contract) {}
