# `sc-contracts` — design specification

> Status: **proposal, awaiting review.** Depends on `docs/sc-extract.md` and `docs/codegen.md`.

## Purpose

`sc-contracts` provides the domain layer for Star Citizen's contract / mission system. It answers the question **"what contracts are available in the game, and what does each one do?"** by walking the DCB's static generator / handler / contract graph, expanding each generator into the concrete variations it can produce, resolving every reference the contract touches (rewards, prerequisites, spawns, locations), and merging near-duplicate variations into a single effective contract.

Target scale: **~1,497 effective contracts** post-merge, matching the SCMDB catalog. The ~2,400 raw contract records in the DCB collapse to that number because sub-contract tiers, location-keyed variations, and repeated per-location re-specs of the same contract all merge back into one logical entry.

## Consumers

| Crate / app | What it uses |
|---|---|
| `sc-langpatch` (driving consumer) | Title / description / cooldowns / rewards / encounters / solo / uniq / crimestat — for localization enrichment. Today has a partial implementation on the untyped svarog layer; migration target. |
| `bulkhead` (future) | Ship-encounter resolution for combat simulation — which ships against what AI skill at what rep level. |
| Future tooling (contract browser, mission planner) | Full contract set with search / filter / variation display. |
| `streamdeck-starcitizen` | Does **not** use `sc-contracts`. |

## Scope

**What `sc-contracts` owns:**

- **Generator expansion.** Walks every `ContractGenerator` root record through its `Legacy` / `Career` / `List` / `LinearSeries` / `TutorialSeriesDef` / `PVPBountyDef` handler, enumerates every contract + sub-contract the generator can produce, applies handler→contract inheritance for availability / param overrides, and emits one `ExpandedContract` per concrete node.
- **Variation merging.** Groups near-duplicate `ExpandedContract`s into one `Contract` with a `variations` list that describes how the underlying expansions differed (location, reward tier, encounter roster). The merge rule is calibrated against the SCMDB target of ~1,497.
- **Reference resolution.** Every GUID the contract points at is resolved to a useful value or kept as a handle with a clearly-typed referent: faction names, reputation scopes, location templates, blueprint-pool item lists, ship-spawn queries → concrete entity-class candidates, reward entity-class → aUEC / scrip / item disambiguation.
- **(Dropped — use `sc_extract::TagTree`.)** The spec originally proposed a `TagRegistry` in this crate. `sc_extract::TagTree` (already built into every `DatacoreSnapshot`) provides everything it would have — GUID ↔ name maps, hierarchical `ancestors` / `descendants` / `is_descendant_of` walks, root-to-leaf path display. Reach through `datacore.snapshot().tag_tree` instead of adding a redundant registry. Dropping this also removes a step from §"Implementation sequencing".
- **Ship-entity registry** (`ShipRegistry`). Indexes every `EntityClassDefinition` that is **reachable from a contract spawn query** by its tag set, display name, and size. Enables `resolve(positive_tags, negative_tags) → Vec<ShipCandidate>` for spawn descriptions. Built once from the DCB, data-derived (see §"Ship-entity registry" — no name-pattern filtering).
- **A `ContractIndex` owned struct** that bundles the expanded / merged contracts together with the tag and ship registries, so downstream code has one entry point.

**What `sc-contracts` does not own:**

- Generated raw types — they live in `sc_extract::generated`. Escape hatch for anything the model doesn't cover is `Datacore::db()` → svarog; consumers reach through that directly rather than a sc-contracts raw layer.
- ~~Localization string resolution~~ (**scope change 2026-04-24**). Originally deferred to consumers, but contract title / description resolution in particular required a non-trivial inheritance walk (contract → sub-contract → handler → template) that every consumer would otherwise reinvent — and `paramOverrides`-only walkers silently return empty strings for entire contract families (TarPits, Adagio, …). `sc_contracts::resolve_contract_text` now owns this; [`Contract.title`] and [`Contract.description`] are resolved `String`s. Consumers still use `sc_extract::LocaleMap` for general localisation needs; sc-contracts just absorbs the contract-specific resolution logic.
- Runtime mission / scenario state — we model DCB templates only.
- Mission-objective *semantics* beyond kind classification — "scan 3 boxes" requires engine knowledge we don't yet have.

**Why tag + ship registries live here, not in a shared crate:**

These caches are currently built ad-hoc inside sc-langpatch's `mission_enhancer.rs`. Putting them in sc-contracts — the first consumer that genuinely needs them — keeps the workspace lean. When a second crate needs them (sc-entities, sc-loadouts), they graduate into a shared helper. Today that's premature.

## Architecture: a four-stage pipeline

```
┌──────────────────────────────────────────────────────────────┐
│  Datacore + LocaleMap                                        │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────────────────────────┐
│  1. Ingest   build registries                                │
│              TagRegistry  (Tag records → tag name)           │
│              ShipRegistry (entities reachable from spawn    │
│                            queries in the contract graph)   │
│              BlueprintPoolRegistry  (pools → item sets)      │
│              RewardCurrencyCatalog  (aUEC / scrip GUIDs)     │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────────────────────────┐
│  2. Expand   ContractGenerator → ExpandedContract list       │
│              per (handler, contract, sub_contract) triple    │
│              apply inheritance (handler → contract overrides)│
│              ~2,400 expansions                               │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────────────────────────┐
│  3. Resolve  rewards / prereqs / encounters / locations      │
│              GUIDs → typed values via registries             │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────────────────────────┐
│  4. Merge    group near-duplicate expansions into Contract   │
│              target: ~1,497 post-merge (SCMDB calibration)   │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────────────────────────────────┐
│  ContractIndex { contracts, tags, ships, blueprints, … }     │
└──────────────────────────────────────────────────────────────┘
```

Every stage is a pure function of the previous. The `ContractIndex` builder is one call; consumers do not see intermediate state unless they ask.

## Top-level API

```rust
pub struct ContractIndex {
    pub contracts:  Vec<Contract>,
    pub ships:      ShipRegistry,
    pub blueprints: BlueprintPoolRegistry,
    pub currency:   RewardCurrencyCatalog,
    pub locations:  LocationRegistry,
    pub localities: LocalityRegistry,
    // tags reached via sc_extract::TagTree (see Scope §)
}

impl ContractIndex {
    /// Build the index from a parsed datacore + resolved locale.
    ///
    /// Runs all four pipeline stages. ~2,400 contract records in, ~1,497
    /// effective contracts out. Order of output is stable across runs.
    pub fn build(datacore: &Datacore, locale: &LocaleMap) -> Self;

    pub fn iter(&self) -> impl Iterator<Item = &Contract>;

    /// Find by contract id (post-merge — first GUID in the variation set).
    pub fn get(&self, id: CigGuid) -> Option<&Contract>;
}
```

`Contract` is fully owned — no `&DataPools` lifetime, no `Handle` fields. Consumers drop the `Datacore` after building the index and carry the `ContractIndex` around freely.

## The `Contract` model

```rust
pub struct Contract {
    // Identity
    pub id: CigGuid,                     // canonical — first variation's contract GUID
    pub debug_name: String,              // from the canonical expansion
    pub generator_id: CigGuid,           // owning ContractGenerator record
    pub handler_kind: HandlerKind,       // Legacy | Career | List | LinearSeries | Tutorial | PVPBounty

    // Display
    pub title: LocaleKey,
    pub description: Option<LocaleKey>,

    // Flags
    pub shareable: bool,
    pub illegal: IllegalKind,            // None | CrimeStatGated | LawLicenseOverride | IgnoresCriminality
    pub repeatability: Repeatability,    // OnceOnly | Repeatable | RepeatableAfterFail | RepeatableAfterAbandon | Always
    pub cooldowns: Cooldowns,            // { completion, abandon, fail } — each Option<DurationRange>

    // Rewards (sum across reward list; currency disambiguated via RewardCurrencyCatalog)
    pub reward_uec:   i32,
    pub reward_scrip: Vec<ScripReward>,         // per-currency-kind (Pyro vs Stanton vs …)
    pub reward_rep:   Vec<RepReward>,           // { faction, scope, amount: Option<i32> }
    pub reward_blueprints: Option<BlueprintReward>,  // { chance, items: Vec<BlueprintItemRef> }
    pub reward_items: Vec<ItemReward>,          // non-currency ContractResult_Item + ItemsWeighting
    pub reward_other: Vec<OtherReward>,         // BadgeAward, ScenarioProgress, JournalEntry, CompletionBounty, …

    // Prerequisites
    pub required_rep:   Vec<RepReq>,
    pub required_chain: Vec<ChainReq>,          // CompletedContractTags — tags + count + exclude
    pub crime_stat_range: Option<(f32, f32)>,   // min/max crime stat if gated

    // Spatial
    pub locations: Vec<LocationRef>,            // location template guids + optional resolved name
    pub encounters: Vec<EncounterGroup>,        // hostile/allied spawn groups with resolved ship candidates

    // Meta
    pub difficulty: Option<DifficultyProfile>,
    pub time_to_complete: Option<Duration>,
    pub buy_in: i32,

    // Variations — the underlying expansions that merged into this contract
    pub variations: Vec<Variation>,

    /// Whether other Contracts in the index share this contract's (title,
    /// description) pair, and whether their rewards match.
    ///
    /// `SharedDiffering` is the signal consumers like sc-langpatch need
    /// when they annotate a contract's displayed text with reward info —
    /// the annotation is not universal, because another contract with the
    /// same displayed name / body text awards different rewards.
    pub reward_certainty: RewardCertainty,
}

pub struct Variation {
    pub contract_id: CigGuid,
    pub sub_contract_id: Option<CigGuid>,
    pub location: Option<CigGuid>,               // if variation is location-keyed
    pub facets: VariationFacets,                 // { reward_uec_delta, reward_blueprints_delta, encounter_tier, … }
}
```

Variations are first-class: when the game has "Deliver package" with a Pyro and a Stanton flavor that award different scrip, the `Contract` keeps *one* entry with `variations.len() == 2`. When a career contract has three tiers, same thing. When two generators produce effectively-identical contracts, they also merge.

This is the shape SCMDB presents publicly, and the shape consumers (searchable browser, sc-langpatch enrichment, analytics) actually want.

## Generator expansion

```
ContractGenerator                             (DCB root record)
└─ generators[] : ContractGeneratorHandlerBase    (polymorphic: 6 kinds)
   │
   ├─ _Legacy       → legacyContracts[] : ContractLegacy
   ├─ _Career       → contracts[]       : CareerContract     (+ introContracts[] : Contract)
   │                  factionReputation, reputationScope
   ├─ _List         → contracts[]       : Contract
   ├─ _LinearSeries → contracts[]       : Contract           (+ ordering metadata)
   ├─ _Tutorial…    → same
   └─ _PVPBountyDef → bounty-specific fields
```

Each concrete contract record carries `paramOverrides` (`ContractParamOverrides`) and `subContracts` (`Vec<SubContract>`). A `SubContract` is a `ContractParamOverrides` plus its own prerequisite set — it represents a *tier* (e.g. Tier 1 / Tier 2 / Tier 3 of a career-faction contract progression).

Expansion emits one `ExpandedContract` per `(handler, contract, Option<sub_contract>)` triple. Fields resolve top-down:

- Availability: handler's `defaultAvailability`, overridden by contract's if present, overridden by sub_contract's if present.
- Param overrides: merged in the same order — later wins on conflict.
- Prerequisites: concatenated (all levels' prereqs apply simultaneously).
- Rewards: sub_contract's `contractResults` if set, else contract's.

Expansion does not merge or dedup — that's stage 4.

## Resolution

Stage 3 turns GUIDs into useful values using the stage-1 registries:

| Reference | Resolver | Result |
|---|---|---|
| `ContractResult_Item.entityClass` | `RewardCurrencyCatalog` | `ScripReward` (faction-scoped), `ItemReward`, or plain UEC |
| `BlueprintRewards.blueprintPool` | `BlueprintPoolRegistry` | `Vec<BlueprintItemRef { entity_class_guid, display_name: LocaleKey }>` |
| `ContractPrerequisite_Reputation.factionReputation` + `.scope` | consumer (Datacore lookup) | keep as `(CigGuid, CigGuid)` — faction registries are a separate concern |
| `MissionPropertyValue_ShipSpawnDescriptions` → `ships[].options[].tags` | `TagRegistry` + `ShipRegistry` | `Vec<ShipCandidate { display_name, size, match_tags }>` |
| `MissionPropertyValue_Locations` | (none) | keep GUIDs as `LocationRef` — optional name resolution from Datacore during ingest |
| `ContractResult_LegacyReputation.contractResultReputationAmounts.reward` | consumer (Datacore lookup) | resolve to `SReputationReward.reputationAmount` |
| `ContractResult_CalculatedReputation` | — | `amount: None` — engine-computed at runtime |

## Variation merging

Two expansions merge when they are the **same mission for display / enrichment purposes** — same text, same rewards — and differ only in spatial / cosmetic details like which location the mission spawns at. Two expansions that share title and description but award different rewards stay separate, because a consumer annotating the shared title with reward info needs to know the annotation is not universal.

**Merge key:**

```
(title_key, description_key, reward_signature)
```

- `title_key` and `description_key` are the `LocaleKey`s after handler→contract→sub_contract override resolution.
- `reward_signature` is a stable hash of the fully resolved reward list — UEC amount, per-faction scrip amounts, rep rewards (faction, scope, amount), blueprint pool GUID + chance, item rewards (entity class + amount), and other rewards (badge / scenario-progress / journal-entry / …) sorted into a canonical order so list order doesn't matter.

Everything else — location, encounter roster, cooldown timings, prereqs — becomes a variation facet. An expansion's location and its spawn-description differences do **not** break the merge; a reward difference always does, even if the difference is just "one variant has a blueprint and the other does not."

The motivating case is sc-langpatch's `mission_enhancer`, which needs to annotate mission text with reward info. When three expansions share a title and two of them award blueprints but the third does not, the enricher must either (a) mark the blueprint info as non-guaranteed or (b) keep the two blueprint-awarding variants as a separate `Contract` from the no-blueprint one. This crate does both — the merge keeps blueprint-awarding expansions together as one `Contract` and the no-blueprint variant as a separate `Contract`, *and* it exposes a certainty flag on each so enrichers can mark the shared text accordingly.

```rust
pub enum RewardCertainty {
    /// No other Contract in the index shares this (title, description) pair.
    /// Reward info applies unconditionally.
    Unique,
    /// Other Contracts share this (title, description) — but all of them
    /// have the same reward signature, so they merged. Reward info applies
    /// unconditionally; the shared title just means multiple locations /
    /// generator variants.
    SharedUniform,
    /// Other Contracts share this (title, description) but have *different*
    /// reward signatures (this is what keeps them as separate Contracts).
    /// Consumers annotating the shared title with reward info should mark
    /// the annotation as non-guaranteed — a player seeing the title in
    /// game may encounter this contract's rewards or one of the siblings'.
    SharedDiffering,
}
```

`reward_certainty` is set during a post-merge sweep that groups all emitted `Contract`s by `(title_key, description_key)` and compares their reward signatures. It cannot be computed from a single contract in isolation.

**Calibration target:** ~1,497 effective contracts post-merge, matching SCMDB. The `contract_census` example prints the raw expansion count (~2,400), the post-merge count under this rule, and the distribution of `RewardCertainty` across the output. If the post-merge count is materially off, the candidates are:

- `reward_signature` is too strict — e.g. small UEC jitter (1000 vs 1050) splits would-be merges. Mitigation: bucket scalar reward amounts before hashing, or ignore scalar differences under a threshold. Census output tells us if this is real.
- `description_key` is too strict — variants with rewritten descriptions should probably still merge. Mitigation: fall back to title-only if description differs but other fields match.
- The signature is *too lenient* — genuinely different missions with coincidentally identical text + rewards merge. Unlikely given title/description specificity, but census will flag suspicious merges (groups where `debug_name` patterns disagree).

We commit to the strict rule as specified; calibration adjustments land as census output warrants, not pre-emptively.

## Tag resolution: use `sc_extract::TagTree`

sc-extract already builds a `TagTree` from every DCB `Tag` record as part of `Datacore::parse` (under `DatacoreConfig::build_tag_tree`). It has GUID ↔ name maps, hierarchical navigation, and `path(&Guid) → Vec<&str>` for display. sc-contracts does not duplicate this — reach through `datacore.snapshot().tag_tree` for any name resolution.

The small amount of sc-contracts-specific tag classification (difficulty tags, ship-class tags, AI-skill parsing) lives as free functions in the crate's internal modules, not as methods on a new type. Each is a tiny string-set check or regex.

**Reference graph (`sc_extract::ReferenceGraph`).** Also available from the snapshot. Useful for reverse lookups — "which contracts reference this location template?" or "which contracts have this faction as a rep prerequisite?" — which we'll want for variation-merging diagnostics and future chain-graph reconstruction (v2). Not used by the ship registry or expansion logic in v1, but consumers (and the census) can reach for it without sc-contracts re-implementing it.

## Ship-entity registry

```rust
pub struct ShipRegistry {
    ships: Vec<ShipEntity>,
    /// Union of every tag present on any ship. Used to filter
    /// spawn-query tags down to ship-relevant ones.
    ship_relevant_tags: HashSet<CigGuid>,
}

pub struct ShipEntity {
    pub entity_guid: CigGuid,
    pub display_name: String,      // resolved from AttachDef.Localization.Name via LocaleMap
    pub tags: HashSet<CigGuid>,
    pub size: i32,                 // from SAttachableComponentParams.AttachDef.Size
}

impl ShipRegistry {
    pub fn build(db: &DataCoreDatabase, locale: &LocaleMap) -> Self;

    /// Resolve a spawn tag query (positive ∩ !negative, ship-relevant tags only).
    /// Returns candidates sorted by size ascending, deduped by display name.
    pub fn resolve_spawn(
        &self,
        positive: &HashSet<CigGuid>,
        negative: &HashSet<CigGuid>,
    ) -> Vec<ShipCandidate>;
}

pub struct ShipCandidate {
    pub display_name: String,
    pub size: i32,
}
```

**Data-derived construction.** sc-langpatch filters `EntityClassDefinition` records by record-name substring (`_PU_AI` / `_pu_ai`) and uses the result as the AI-ship pool. This is known to be incomplete — contracts in the live DCB list spawn tags that resolve against entities outside that substring pool, and sc-langpatch silently drops those spawns. Avoid porting the filter. Workspace rule §5 forbids name-substring filtering where a data-derived alternative exists.

The alternative is two-pass: the first pass walks every `MissionPropertyValue_ShipSpawnDescriptions` in the contract graph and collects every `Tag` GUID referenced in spawn queries (positive + negative) into a `spawn_referenced_tags: HashSet<CigGuid>`. The second pass walks all `EntityClassDefinition` records and keeps any whose tag set intersects `spawn_referenced_tags`. The resulting pool is exactly the set of entities contracts can possibly spawn, and contains nothing that contracts cannot — the game data defines the pool, not our naming heuristic.

Display names and sizes come from the entity's `SAttachableComponentParams` component — `AttachDef.Localization.Name` (→ `LocaleMap`, exposed via `sc_extract::DisplayNameCache`) and `AttachDef.Size`. These are typed accesses through the generated layer, no string matching.

**Query semantics — three tag classes via the tag tree.** The `sc_extract::TagTree` has 65 top-level roots (`Ship`, `AI`, `Missions`, `UI`, …). Classification is purely tree-based (workspace rule §5):

1. **Ship-selective** — descendants of the `Ship` root (`Cutter`, `Pisces`, `Gladius`, `Idris_M`). 832 tags in 4.7. Identify a specific model / variant.
2. **Carried non-ship** — live under other roots (usually `AI` or `Missions`) but are attached to some pool entities (`Criminal`, `PU`, `VeryHard`, `Ninetails`). Narrow the pool but don't pick a model.
3. **Pure context** — non-ship and attached to zero pool entities (runtime directives like `ArriveViaQT` / `IgnoreCrimes`, role markers like `Target` / `Defenders`, AI skill tags like `HumanPilot70`, difficulty words the engine composes with location context).

`resolve_spawn` dispatches on **intent** in the original positive set:

- **Ship-targeted query** (contains ≥1 `Ship`-root tag). The contract wanted a specific model. Drop pure-context tags. If a ship tag survived the filter, strict ALL-OF match on survivors + negative exclusions. If every ship tag was dropped (zero carriers — broken or typo'd selector like `Relient_Tana` in Gilly Mission01 Wave3), return empty rather than over-match against the surviving broad tags.
- **Broad query** (no `Ship`-root tag in the original positive set). Contract intentionally didn't pin a model (e.g. TarPits salvage Wave3 `{VeryHard, Criminal, PU}`). Drop pure-context tags; strict-match the rest.

The tag tree itself has only `children` references in the DCB — no `parent` field. `sc_extract::TagTree::from_database` derives parents from children in a second pass so `ancestors()` / `path()` / `is_descendant_of()` work (fixed 2026-04-24).

**Coverage** (SC 4.7 LIVE census 2026-04-24): 94.1% of spawn options resolve to ≥1 candidate (5,920 of 6,291). The residual 5.9% split into (a) queries with only pure-context positives (`Target` / `Defenders` / etc. — await runtime location AI-spawn merge), and (b) ship-targeted queries where the selector tag has zero carriers (broken data). SCMDB's mission display shows the same behavior on the Gilly Mission01 Wave3 case — confirming honest-empty is the right answer.

**Initial build numbers**: 3,218 ship entities, 223 spawn-referenced tags, 913 ship-relevant tags, size distribution {0:33, 1:1192, 2:614, 3:390, 4:275, 5:301, 6:234, 7:179}.

**Non-ship context: `SpawnContext`** (landed). Subtree-driven classification of the non-ship positive tags:

```rust
pub struct SpawnContext {
    pub ai_skill:     Option<u32>,     // parsed from HumanPilotN
    pub ace_pilot:    bool,             // AcePilot tag present
    pub factions:     Vec<String>,      // descendants of AI ▸ Faction
    pub cargo:        Vec<String>,      // descendants of AI ▸ CargoManifest
    pub ai_traits:    Vec<String>,      // other AI descendants (Target, Defenders, …)
    pub mission_tags: Vec<String>,      // Missions root
    pub directives:   Vec<String>,      // name starts with Arrive/Ignore
    pub other:        Vec<String>,      // uncategorised — never silently dropped
}
```

Classification walks each tag's `TagTree::path()` and dispatches on the root + first category. Works because the DCB organises spawn-context tags into principled subtrees (`AI ▸ Faction` / `AI ▸ CargoManifest` / `AI ▸ SkillDefinitions`) — the classifier respects that structure rather than pattern-matching names.

Consumers like sc-langpatch get per-wave enrichment for free: Gilly's Combat Gauntlet Missions 01→08 surface as `ai_skill=10…90` with cargo progressing `Scraps ▸ Minimal ▸ Half ▸ Full` — matching the mission's difficulty curve without any extra parsing. The model layer's `Wave` struct embeds `SpawnContext` directly.

## Requirements map

| Wishlist item | Source + handling | Model field |
|---|---|---|
| Title, Description | Resolved via [`resolve_contract_text`] which walks `sub_contract.stringParamOverrides → contract.paramOverrides.stringParamOverrides → handler.contractParams.stringParamOverrides → template.stringParamOverrides`, then looks up each `LocaleKey` in `sc_extract::LocaleMap`. Many contracts (TarPits salvage, Adagio salvage, …) carry title / description only on the handler or template — naive walkers that read only `paramOverrides` see empty strings. | `title: String`, `description: Option<String>` — resolved text. `~mission(...)` runtime-substitution markers are preserved verbatim; consumers check [`ResolvedText::has_runtime_substitution`] to detect them. |
| Reward UEC | `ContractResult_CalculatedReward` (engine-computed at runtime — see below) for most contracts; occasional `ContractResult_Item` with aUEC entity_class for fixed amounts | `reward_uec: RewardAmount::{Calculated(CalcInputs), Fixed(i32), None}` — majority of contracts report `Calculated` because the DCB does not store the amount. `CalcInputs` carries the DCB scalars a community formula consumes (difficulty profile weights, difficulty enum levels, time_to_complete, buy_in). Estimator ships as opt-in in `sc_contracts::estimators::estimate_uec`. |
| Reward Blueprint | `BlueprintRewards { chance, blueprintPool }` resolved through [`BlueprintPoolRegistry`]. Two-stage item-name resolution: crafted-entity's `DisplayNameCache` entry (primary) → `CraftingBlueprint.blueprintName` locale lookup (fallback). CIG ships placeholder text `<= PLACEHOLDER =>` for un-finalised strings — detected and treated as unresolved. | `reward_blueprints: Option<BlueprintReward { chance, items: Vec<BlueprintItem { crafted_entity_guid, display_name, weight }> }>` |
| Reward Scripts | `ContractResult_Item` where `entityClass` is in `RewardCurrencyCatalog::scrip_kinds` | `reward_scrip: Vec<ScripReward { kind, amount }>` |
| Reward Rep | `ContractResult_LegacyReputation` (amount from `SReputationReward.reputationAmount`) + `ContractResult_CalculatedReputation` (amount = None) | `reward_rep: Vec<RepReward { faction, scope, amount: Option<i32> }>` |
| Other Rewards | `ContractResult_ItemsWeighting`, `_BadgeAward`, `_CompletionBounty`, `_ScenarioProgress`, `_JournalEntry`, `_CompletionTag(s)`, `_RefundBuyIn`, `_Reward` | `reward_items: Vec<ItemReward>` + `reward_other: Vec<OtherReward>` |
| Required completed Mission (Chains) | `ContractPrerequisite_CompletedContractTags { required_tags, excluded_tags, counts }` | `required_chain: Vec<ChainReq>` — v1 keeps tag GUIDs; graph reconstruction (which contracts unlock which) is v2 |
| Required Rep & Max Rep | `ContractPrerequisite_Reputation { faction, scope, min_standing, max_standing, exclude }` | `required_rep: Vec<RepReq>` |
| Shareable | template `ContractClass_Contract.additionalParams.canBeShared` | `shareable: bool` |
| Illegal | `ContractPrerequisite_CrimeStat { min, max }` + modifiers `MissionModifier_LawLicense`, `_IgnoreMissionPlayerCriminality`, `_HostileMission` | `illegal: IllegalKind` (opinionated summary of the modifier combo) + `crime_stat_range: Option<(f32, f32)>` |
| Repeatable | `ContractAvailability.once_only` + `can_reaccept_after_abandoning` + `can_reaccept_after_failing` | `repeatability: Repeatability` |
| Cooldown (Completion / Abandon / Fail) | `ContractAvailability.{personal_cooldown_time, personal_cooldown_time_variation, abandoned_cooldown_time, abandoned_cooldown_time_variation, can_reaccept_after_failing}` | `cooldowns: Cooldowns { completion, abandon, fail }` each `Option<DurationRange { mean, variation }>` |
| Possible Ship Encounters | `paramOverrides.propertyOverrides[{ missionVariableName: *ShipSpawnDescriptions* \| *MissionTargets* \| *WaveShips*, value: MissionPropertyValue_ShipSpawnDescriptions }]` → `ShipRegistry::resolve_spawn` per option | `encounters: Vec<EncounterGroup { role, name, waves: Vec<Wave { count, ai_skill, candidates: Vec<ShipCandidate> }> }>` |
| Mission Steps | `MissionModuleHierarchy.submissions[].objectiveHandler` | v1: `mission_steps: Vec<MissionStep>` with `kind: ObjectiveKind { NearLocation, Local, Unknown(String) }` + associated scalars. Full objective-semantics modelling deferred to v2 as the objective-kind inventory grows. |
| Locations | `paramOverrides.propertyOverrides[{ missionVariableName: *Locations*, value: MissionPropertyValue_Locations }]` + `ContractPrerequisite_Location{,Locality,Property}` | `locations: Vec<LocationRef { guid, name: Option<String> }>` |
| Variations | merge stage — see §"Variation merging" | `variations: Vec<Variation>` + `reward_certainty: RewardCertainty` (post-merge signal for "is reward info universal across this contract's displayed title?") |

## DCB pitfalls worth flagging up-front

These bit sc-langpatch and will bite the new code if we're not careful:

- **Contract subclass discrimination.** `Contract`, `ContractLegacy`, `CareerContract` all derive from `ContractBase` but live in separate pools with different field shapes. Expansion must branch per handler kind; reading every pool with a single generic accessor silently drops records.
- **Handler kinds are *not* uniform.** sc-langpatch walks `generators[]` as untyped instances and calls `get_array("contracts")` blindly — works for Career, misses Legacy (`legacyContracts`), misses PVPBounty's bounty-specific fields. The typed layer forces per-kind branching. Use it.
- **String params carry a typed `param` enum.** `ContractStringParam.param` is `ContractStringParamType` (enum variants `Title`, `Description`, …). Match the variant, not a stringified value.
- **Every polymorphic enum has an `Unknown { struct_index, instance_index }` fallback.** Preserve it — a future patch can add a prerequisite or reward kind we haven't classified. Dropping into "unknown, carry the raw handle" is better than silently omitting the record.
- **Availability lives in three places** — handler, contract, sub_contract. Expansion's inheritance rule is *later wins on conflict, merge otherwise*. sc-langpatch reads only the handler and misses contract-level overrides entirely.
- **Title / description live in four places** — sub_contract, contract.paramOverrides, handler.contractParams, **and** the referenced `template` record. Most contract families only carry their text on the handler or template; a walker that reads only `paramOverrides` silently loses every TarPits / Adagio / Covalex salvage contract's title. [`sc_contracts::resolve_contract_text`] walks all four levels and looks up the resolved text in [`sc_extract::LocaleMap`].
- **Runtime substitution markers survive resolution.** Many titles and descriptions embed `~mission(variable)` markers the game engine fills at spawn time. The TarPits salvage title resolves to `~mission(Contractor|SalvageContractorTitle)`, which at runtime expands through the contracting faction's string table into (e.g.) `TarPits_ShipStrip_title_001` = `~mission(ship) clean up`, and *then* `~mission(ship)` is filled from the spawn pool. The engine picks the title's ship and the actual spawn ship independently; they can and do diverge (the in-game "description mentions Caterpillar but spawns a Starfarer" bug lives exactly at this seam). [`ResolvedText::has_runtime_substitution`] flags titles carrying these markers so consumers can display the spawn pool alongside the text and mark the text as non-guaranteed.
- **Reward amounts can be engine-computed — at scale.** Both `ContractResult_CalculatedReputation` and `ContractResult_CalculatedReward` carry no amount; the game computes them at runtime from the contract's difficulty profile, buy-in, time-to-complete, and (for some contract types) cargo/distance context. The `CalculatedReward` case covers ~34% of all reward records in 4.7 LIVE (census 2026-04-24). Our model surfaces this with `RewardAmount::Calculated(CalcInputs)` rather than reporting `0`. The `CalcInputs` payload carries the DCB scalars needed by downstream estimators, and `sc_contracts::estimators::estimate_uec` ships the community-derived formula (known as "evergr3n's formula" — used by SCMDB). The estimator is documented as community-derived and not engine-authoritative; consumers who want the raw DCB truth pattern-match on the enum, consumers who want a displayable number call the estimator.
- **`contractResultReputationAmounts` is a `Class`, not a `Reference`.** sc-langpatch reads it as `get_instance(…).get("reward") → Reference → record.get_i32("reputationAmount")`. Works because svarog inlines class fields. In the typed layer: `result.contract_result_reputation_amounts → SReputationAmountParams.reward → SReputationReward.reputationAmount` — two hops, both typed.
- **Fail cooldown has a flag AND a time field.** `can_reaccept_after_failing: bool` gates whether the abandoned cooldown applies to fails at all. The `Cooldowns { fail }` representation must distinguish "no fail cooldown" (instant retry) from "abandoned cooldown reused for fail" from "can't reaccept ever after fail." Census will surface which convention the DCB actually uses.
- **Ship-pool construction is data-derived, not name-filtered.** sc-langpatch uses a `_PU_AI` / `_pu_ai` substring filter on `EntityClassDefinition` record names; the filter under-matches and drops contract-referenced ships. This crate derives the pool from spawn-query tags in the contract graph itself — see §"Ship-entity registry". This also matches workspace design principle §5 (no name matching where data-derived alternatives exist). The corresponding coverage invariant is: **every spawn query in the resolved contract graph produces ≥1 ship candidate**; census emits a warning count when that fails, which is the regression signal that replaces sc-langpatch's "ship_registry.len() < 100" alarm.
- **Stack-bump on Windows** still applies. Any example or test that parses the full DCB from a live P4K needs `--features full`; the `parse_real_p4k` example depends on `.cargo/config.toml`'s `/STACK:8388608` link arg.

## Census and calibration

`examples/contract_census.rs` — a diagnostic binary that walks the contract graph directly from the generated pools (no crate code required) and prints structural stats. **Initial run (2026-04-24, SC 4.7 LIVE)** produced:

- 106 `ContractGenerator` records, 248 handlers, 4,590 expansions (handler × contract × sub_contract), 1,800 sub-contracts.
- Handler-kind distribution: Career 71%, List 25%, Tutorial / PVPBounty / Legacy / LinearSeries / ServiceBeacon <1% each. Legacy has a single handler carrying 419 legacy contracts (so it's vestigial by handler count but not by contract count).
- Contract subclasses: `CareerContract` 1,933, `Contract` 422, `ContractLegacy` 419, intro Contracts 16.
- Zero unknown handler / prereq / reward kinds → the typed enums cover the full 4.7 surface.
- Reward-kind histogram surfaced the **`CalculatedReward` discovery** noted in §"DCB pitfalls worth flagging up-front" — 34% of reward records are engine-computed.
- `ContractResult_Item.entity_class` top-N is small and readable: the merc scrip (99 uses) and council scrip (84 uses) dominate; Wikelo favours and collector ship one-offs fill out the long tail. **aUEC does not appear in this list** — confirming the CalculatedReward hypothesis. Currency catalog will be populated from the top ~5 entries.
- 929 distinct titles across 2,377 title-bearing expansions. Largest title group: 98 expansions (`RedWind_HaulCargo_AToB_title_01`). 480 expansions have no title (sub-contract tiers that inherit from parent — next-iteration handling).

The diagnostic continues to emit in subsequent runs:

- Handler-kind histogram (how many generators of each kind).
- Expansion counts: total `ExpandedContract`s before merge (expect ~2,400).
- Post-merge count against the 1,497 SCMDB target under the committed rule + under relaxation candidates (UEC bucketing, title-only fallback) so any over-fragmentation is visible.
- `RewardCertainty` distribution across the post-merge output — how many contracts are `Unique` / `SharedUniform` / `SharedDiffering`. The `SharedDiffering` count is the population sc-langpatch has to annotate as non-guaranteed.
- Contract-subclass distribution (`Contract` / `ContractLegacy` / `CareerContract`).
- Prereq-kind histogram.
- Reward-kind histogram.
- `ContractResult_Item.entity_class` top-N — for `RewardCurrencyCatalog` calibration.
- `MissionProperty.mission_variable_name` top-N — for encounter classification.
- Unresolved-handle counters: prereq `Unknown` fallbacks, reward `Unknown` fallbacks, **empty-candidate spawns** (the sc-langpatch filter-drop regression signal — ≥1 means the ship registry is missing entities contracts reference), missing titles. Non-zero values are lint signals.
- Spot-check rows: title + variation count for 10 named contracts the user can eyeball against SCMDB.

This is the primary tool for calibrating merge rules and validating resolution against real data. Ships alongside the crate; runs in <10s with `--features full`.

## Open questions

1. **Reward-signature strictness.** The committed merge rule groups on `(title_key, description_key, reward_signature)` where `reward_signature` is an exact-match hash. If census shows the strict rule over-fragments (post-merge count ≫ 1,497), the first mitigation is bucketing scalar reward amounts (UEC / scrip) before hashing. No need to pick pre-emptively — census decides.

   *Census update 2026-04-24:* `CalculatedReward` (34%) has no scalar amount, so all `Calculated` variants merge freely against each other — they differ only by their `CalcInputs`, and inputs are part of the signature only if the consumer wants to split on difficulty. v1 leaves `CalcInputs` *out* of the signature so same-text missions with same difficulty profile and different post-engine UEC amounts merge. Revisit if the 1,497 target fails.
2. **Scrip disambiguation.** Which `entityClass` GUIDs are scrip currency vs generic items? Current hypothesis: a small fixed set, distinct per faction (Pyro scrip vs Stanton scrip). `contract_census`'s entity-class top-N will confirm. `RewardCurrencyCatalog` starts with a hardcoded list, derived from census output, kept in a `currency.rs` table.
3. **Location-name resolution during ingest vs deferred.** Resolving `LocationRef.name` requires walking location-template records; ~hundreds exist. Option: resolve eagerly during ingest (simpler API, slightly larger memory); option: leave `name: None` and let consumers resolve via their own `sc_extract::svarog` access. Lean: eager — consumers always want names.
4. **Objective kind inventory.** `ObjectiveHandler_Local` and `_NearLocation` are in the `contracts` feature. Are there objective kinds in `missiondata`, `missionbroker`, or leaf features? Census should include a top-level polymorphic-enum inventory for objectives. v1 handles the 2 kinds we've seen; v2 expands.
5. **`Datacore` or `DataCoreDatabase`?** `ContractIndex::build` takes a parameter. `Datacore` is sc-extract's public entry; `DataCoreDatabase` is svarog's. Accessing the typed pools requires `Datacore::pools()`; accessing the `records_by_type` iterators requires `Datacore::db()`. Builder wants both — take `&Datacore` and reach through as needed.
6. **`specta` feature flag.** sc-langpatch is Tauri and will eventually want TS bindings for `Contract`. Ship v1 without `specta`; add it when sc-langpatch migrates — same pattern as sc-weapons.
7. **evergr3n's formula — source of truth.** The community-derived UEC estimator used by SCMDB needs to be ported into `sc_contracts::estimators::estimate_uec`. Inputs we have access to from the DCB: `ContractDifficulty.{mechanical_skill, mental_load, risk_of_loss, game_knowledge}` as enum-choice levels, the per-dimension `ContractDifficultyProfile.{mechanical_skill_weight, mental_load_weight, risk_of_loss_weight, game_knowledge_weight}`, plus `ContractResults.{time_to_complete, contract_buy_in_amount}`. The enum-to-numeric mapping (e.g. `VeryEasy=1 … Super=6`) and the combining function need to be transcribed from whichever source SCMDB publishes. Until we have the source, `estimate_uec` returns `None` and a doc comment links the open question.

## Implementation sequencing

1. **Census against generated types.** ✅ Landed 2026-04-24; `examples/contract_census.rs`. First output is summarised in §"Census and calibration".
2. **Registries.** ✅ All landed 2026-04-24.
   - `TagRegistry` **dropped** (use `sc_extract::TagTree`).
   - `ShipRegistry` — 3,218 entities, 94.1% spawn-query coverage, intent-based resolver.
   - `BlueprintPoolRegistry` — 45 pools, ~100% item-name resolution.
   - `SpawnContext` — tag-tree subtree classifier (ship/AI/Missions/directive/other).
   - `resolve_contract_text` — 4-level inheritance + `LocaleMap` resolution with runtime-substitution marker detection.
   - `RewardCurrencyCatalog` — typed via `SItemDefinition.type == EItemType::Currency`, 2 entities in 4.7 LIVE (merc scrip 99 uses, council scrip 84 uses). Banu favours intentionally classified as items (not `EItemType::Currency`).
3. **Expansion.** ✅ v2 landed 2026-04-24. [`expand_all`] produces 4,590 `ExpandedContract`s matching the census count. Each row carries the full contract picture after inheritance resolution:
   - **Identity**: `id`, `debug_name`, `generator_id`, `handler_kind` + `handler_debug_name`, `origin` (which of Contract/Legacy/Career/SubContract with parent backlink).
   - **Text**: resolved `title` / `description` with `has_runtime_substitution` flag (sub→contract→handler→template inheritance).
   - **Availability** struct: `once_only`, `can_reaccept_after_abandoning`, `can_reaccept_after_failing`, `has_personal_cooldown`, `hide_in_mobi_glas`, `available_in_prison`, `notify_on_available`, `max_players_per_instance`, plus `cooldowns: { completion, abandon, fail }` each `Option<DurationRange { mean_seconds, variation_seconds }>`. Inheritance: handler's `ContractAvailability` base overlaid by contract / sub-contract `bool_param_overrides` (typed `ContractBoolParamType`) + `int_param_overrides` (typed `ContractIntParamType`).
   - **Flags**: `shareable` (template `canBeShared` base, overrideable by `CanBeShared` bool param), `illegal_flag` (from `Illegal` bool param).
   - **Rewards**: `reward_uec: RewardAmount::{Calculated, Fixed, None}`, `reward_scrip: Vec<ScripReward>` (typed-currency via catalog), `reward_rep: Vec<RepReward { faction, scope, amount: Option<i32> }>` (amount resolved through `SReputationRewardAmount.reputationAmount`; `None` for `CalculatedReputation`), `reward_items: Vec<ItemReward>` (non-currency `ContractResult_Item`), `reward_other: Vec<OtherReward>` (BadgeAward / ScenarioProgress / JournalEntry / CompletionTags / CompletionBounty / ItemsWeighting / Reward / RefundBuyIn(f32) / Unknown), plus `blueprint_reward: Option<BlueprintReward>` (pool name + resolved items).
   - **Prerequisites**: `prerequisites: Vec<PrereqView>` unifying handler `defaultAvailability.prerequisites` + contract / sub-contract `additional_prerequisites`. Typed enum covering `Locality`, `Location`, `LocationProperty`, `CrimeStat`, `Reputation`, `CompletedContractTags`, plus `Unknown { struct_index, instance_index }` for feature-gated / new kinds.
   - **Encounters**: `Vec<EncounterGroup { variable_name, waves: Vec<EncounterWave { name, slots: Vec<EncounterSlot { concurrent, weight, candidates, context }> }> }>` — ship spawn queries resolved through `ShipRegistry::resolve_spawn` (intent-based with Ship-subtree classifier) with per-slot `SpawnContext` breakdown (AI skill, faction, cargo, arrival, …).
   - **Numbers on 4.7 LIVE**: 3,886 (85%) with title, 3,549 (77%) with runtime-substitution marker, 3,977 shareable, 164 once-only, 885 illegal, **3,933 (86%) Calculated UEC / 0 Fixed** (confirms aUEC is always engine-computed), 513 with scrip, 3,603 (79%) with rep, 72 with items, 4,161 (91%) with prereqs, 4,471 (97%) with cooldown, 1,052 with blueprints.
   - **Validated against Gilly Combat Gauntlet**: Mission03=Hornet, 04=Razor EX, 05=Freelancer+Sabre+Vanguard, 06=Connie+Gladius, 07=Hammerhead+Hornet+Arrow, 08=Polaris+Gladius+Corsair. Exact ship pools + structured SpawnContext per slot.
4. **Merge + `Contract` + conflict helpers.** ✅ v1 landed 2026-04-24. [`merge_expansions`] groups `ExpandedContract`s by `(title, description, reward_signature)` — text-less expansions fall back to `(handler_debug_name, debug_name)` so unrelated text-less contracts don't collapse. Each `Contract` carries canonical fields from the merge group plus `variations: Vec<Variation>` with per-variation `extra_prerequisites` (the unique `Locality` GUIDs / gate prereqs that differ across tiers). A post-merge sweep populates `Contract.title_siblings: Vec<Guid>` with the IDs of other contracts sharing the same `(title, description)` — the data primitive consumers build annotations against.

   **Conflict helpers** build on `title_siblings`: [`find_bp_conflicts(&[Contract]) → Vec<BpConflictGroup>`] returns every title where siblings disagree on blueprint rewards, with `has_mixed_presence: bool` flagging the critical case where some variants have a BP and others don't. This is the API sc-langpatch calls to render `[BP]*` on mission titles. Analogous `find_scrip_conflicts` / `find_rep_conflicts` helpers follow the same shape (future work).

   **Design note**: the original proposal had a `RewardCertainty::{Unique, SharedUniform, SharedDiffering}` enum on each `Contract`. It was dropped in favour of `title_siblings` because (a) `SharedUniform` was unreachable under strict merging (matching rewards + matching title always collapse into one Contract), and (b) consumers flagged `SharedDiffering` still had to walk the data to know *what* differs — so a direct sibling pointer gives them the actionable primitive without the intermediate signal that's just a "go look" hint.

   **Numbers on 4.7 LIVE**: 4,590 expansions → **1,642 Contracts** (9.7% over the 1,497 SCMDB target). Collapse ratio 2.8×. Largest variation group: 67 (a single contract collapsing 67 sub-contract tiers). **366 contracts (22.3%) have non-empty `title_siblings`** — shared-title populations. `find_bp_conflicts` returns **39 groups globally, 8 with mixed BP presence** (the `[BP]*` population). Most shared-title groups are TarPits / Adaigo / CFP salvage + handyman families where the same `~mission(Contractor|…)` template is used across multiple reward tiers.

   **Validation**: Gilly Combat Gauntlet collapses to 8 Contracts (one per scenario) each with `variants: 4 (CSSS)` = parent + 3 sub-contracts sharing rewards; MG Scrip escalates 1→3→5→10→12→14→16→20 across scenarios. Pyro blueprint conflicts include "Destroy Headhunter Stolen Data" (4 sibs across 2 regional pools — RegionAB=ADP armor, RegionCD=A03 Sniper), "Small Purchase Order: Hand Mined Materials" (Stanton vs Pyro item themes — Strata vs Novikov Exploration suit), "Pilot Seeking Combat Support" (Nyx variant has a BP, Pyro variants don't — textbook mixed-presence case).

   **Future tuning** (if SCMDB alignment needs tightening): the first lever per spec is bucketing scalar reward amounts (UEC, scrip) before hashing — currently strict, so e.g. a 1000 vs 1050 UEC difference splits the merge. Left to post-v1 based on the ±5% target vs the current +9.7%.
5. **evergr3n's formula.** Port into `estimators::estimate_uec`. Deferred to post-v1 per user direction — reverse-engineering from SCMDB's numeric outputs when we have time.
6. **sc-langpatch migration.** Rewrite `mission_enhancer` on top of `ContractIndex::build`. The migration is the correctness test.

Remainder of step 2 is the next reviewable slice. Steps 3–4 are another. Steps 5 and 6 are their own work items.

## Future features (planned, not v1)

These are concrete extensions with specific data paths identified. They land when a consumer needs them.

### Mission-span / locality resolution ✅ landed 2026-04-24

**Problem.** In Pyro and Nyx, a single accepted contract can dispatch you anywhere across the system. Stanton "feels OK" because missions cluster tightly, but Pyro contracts routinely span 3-5 planets. More pointedly for sc-langpatch: same-title Pyro contracts can ship *different* blueprint pools depending on which region they're offered in (`BP_MISSIONREWARD_..._AB` vs `..._CD`) — invisible on the rewards alone, but obvious once mission_span is resolved.

**Model.**

```rust
pub enum SystemKey { Stanton, Pyro, Nyx, Other(String) }

pub struct LocationRef {
    pub guid: Guid,
    pub record_name: String,             // "Pyro3_L1", "RR_P6_LEO_CLINIC"
    pub system: SystemKey,
    pub body: Option<String>,            // "Pyro3" / "Stanton1" when there's an intermediate parent
}

pub struct LocalityView {
    pub guid: Guid,
    pub name: String,                    // record name minus `MissionLocality.` prefix
    pub locations: Vec<LocationRef>,
    pub systems: BTreeSet<SystemKey>,    // distinct systems covered
}

pub struct LocationRegistry { /* guid → LocationRef */ }
pub struct LocalityRegistry { /* guid → LocalityView */ }

pub struct ExpandedContract { /* …, */ pub mission_span: Vec<Guid> }
pub struct Contract        { /* …, */ pub mission_span: Vec<Guid> }
```

**Implementation.** `LocationRegistry::build(&datacore)` walks every `StarMapObject` and iteratively resolves the `parent` chain (cycle-guarded). The root-most ancestor's record name is normalised (`PyroStar` → `Pyro`, `NyxSolarSystem` → `Nyx` — stripping `Star` / `SolarSystem` suffixes) to produce `SystemKey`. Body is `Some(name)` only when the chain has at least one intermediate node between the location and root, so Lagrange points / moons surface `body: Some("Pyro3")` without redundantly labelling top-level planets. `LocalityRegistry::build(&datacore, &locations)` then resolves every `MissionLocality.available_locations` through the location registry, aggregating `BTreeSet<SystemKey>` per locality.

`expand_all` takes `&LocalityRegistry` and populates `ExpandedContract.mission_span` from the Locality prereq chain (handler + contract + sub-contract), first-seen-wins-dedup. `merge_expansions` takes the union across variations for `Contract.mission_span`.

**Scope decisions.**
- `mission_span` is `Vec<Guid>`, not inline `Vec<LocalityView>`, to avoid duplicating ~100k LocationRefs across ExpandedContracts. Consumers look up through `LocalityRegistry::get`.
- System classification uses *parent-chain traversal* (data-derived, workspace rule §5), not record-name prefix matching. Only the root-node normalisation (`PyroStar` → `Pyro`) uses name matching, and that is the root's canonical identity in the DCB — not a reachability heuristic.
- No inheritance-diff resolution: every variation's Locality prereqs are merged into the Contract's span. The `variations[i].extra_prerequisites` list still holds the per-variation unique prereqs for consumers that care which sub-contract adds which region.

**SC 4.7 LIVE numbers.**
- 805 Contracts (49% of 1,642) have non-empty mission_span; 2,451 locality references total.
- 359 contracts cross multiple systems (primarily Stanton↔Pyro career chains).
- All 8 mixed-BP-presence `find_bp_conflicts` groups have clearly distinguishable spans: `[Stanton] Stanton1..4` vs `[Pyro] Pyro2..6` (Shubin mining), `[Pyro] RegionA, RegionB` vs `[Pyro] RegionC, RegionD` (CFP Headhunters family), `[Nyx] Nyx` vs `[Pyro] RegionA..D` (CFP DefendShip fork).

**sc-langpatch hook.** Enables the German community's `[BP]*` annotation pattern to be precise: instead of "this title has multiple reward variants", the annotator can render `[Region A/B: BP_AB (14 items)] [Region C/D: BP_CD (12 items)]` for the same title, using `Contract.title_siblings` + `LocalityRegistry::get(contract.mission_span[i])` to build the per-region callout.

### Other deferred items

- **Objective semantics** — currently exposing `MissionModuleHierarchy` objective kinds; full per-kind field modelling (scan N boxes, deliver X, kill Y) lands when the inventory stabilises.
- **Contract chain graph reconstruction** — turning `ContractResult_CompletionTags` emitters into edges that feed `ContractPrerequisite_CompletedContractTags` consumers, so consumers can render "Mission A unlocks Mission B" chains.
- **Region/danger classification** — per the German community's "Pyro Region B (Bloom) — Gefahr 4/10" display, we'd need to walk region/danger records. Adjacent to mission-span.
- **Turn-in cargo requirements** — hauling / salvage missions with specific cargo drop-off (Torite, Agricium, etc.). Lives in `HaulingOrder_*` types, out of the contract domain per spec.

## Out of scope for this document

- Runtime mission / scenario state — DCB templates only.
- Localization string resolution — consumers combine `LocaleKey` outputs with a `LocaleMap`.
- Escape-hatch access to types this crate doesn't model — consumers with `Datacore` use `datacore.db()` / `datacore.pools()` directly, same pattern as every other sc- crate.
