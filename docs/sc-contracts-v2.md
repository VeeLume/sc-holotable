# sc-contracts v2 — Mission-centric API redesign

> **Status:** proposal, not implemented. Builds on [`docs/sc-contracts.md`](sc-contracts.md) (v1 spec) and [`docs/sc-contracts-guide.md`](sc-contracts-guide.md) (current consumer API).
> **Driver:** the encounter-data audit ([`role_investigation.rs`](../crates/sc-contracts/examples/role_investigation.rs), [`encounter_analytics.rs`](../crates/sc-contracts/examples/encounter_analytics.rs)) surfaced asymmetries and accretion in the v1 surface that incremental fixes can't clean up.
> **Filed:** 2026-05-01.

## Motivation

The v1 surface grew to mirror the DCB type graph (`ContractGenerator → ContractGeneratorHandler{N} → Contract → SubContract → MissionProperty → MissionPropertyValue_ShipSpawnDescriptions → SpawnDescription_ShipGroup → SpawnDescription_ShipOptions → SpawnDescription_Ship`) and bolt derived data on top as we discovered we needed it. The result reads to a consumer as "here's how the DCB stores missions" rather than "here's a mission" — it's organized around *generation*, not around the mission as a player-facing thing.

Concrete pain points the audit revealed:

- **Asymmetric tag handling.** `SpawnDescription_Ship` carries four sibling `TagList`s (positive, negative, markup, entity). v1 surfaces them four different ways: positive becomes a deeply-classified `SpawnContext` with 9 buckets, negative is consumed silently inside `ShipRegistry` and disappears, markup/entity get flat `Vec<String>`s.
- **Generator-shaped names leak into consumers.** `Contract.handler_kind`, `handler_debug_name`, `variations: Vec<Variation>` (DCB SubContract). A patcher tool doesn't think about handlers and sub-contracts — it thinks about missions and their differing variants.
- **The merge step is implicit and lossy in identity.** `Contract` is the result of `merge_expansions()` collapsing N `ExpandedContract` rows by `(title, description, reward_signature)`. The merged `Contract` carries one of the original GUIDs as canonical and discards the rest, leaving `Contract.id` semantically ambiguous: which of the N original entries does it refer to? Worse, the merge bakes one specific equivalence rule into the type, when sc-langpatch actually wants `description_key` equivalence and SCMDB wants `(title, description, rewards)` equivalence — different axes for different consumers.
- **Reward fields scattered across six top-level slots.** `reward_uec`, `reward_scrip`, `reward_rep`, `reward_items`, `reward_other`, `blueprint_reward`. Browsing a `Contract` in the explorer is a vertical wall.
- **Encounter coverage is ship-only.** NPC spawn descriptions (1,317 instances, 2.5% of `MissionProperty`) and Entity spawn descriptions (316, 0.6%) are entirely invisible — together that's ~30% of encounter-shaped data we don't surface. §1's crimestat work needs `MissionPropertyValue_NPCSpawnDescriptions.autoSpawnSettings.missionAlliedMarker` anyway.
- **Naming clash with DCB.** `Contract` is both our merged-mission consumer type *and* a raw DCB record subtype. `Mission` better reflects what the consumer sees (`Contract` is one of three DCB record sources that feed into a single `Mission`).

The cumulative effect is that the public surface is harder to reason about than the data it describes. v2's goal: invert that — make the surface read like the mission, with the DCB plumbing tucked under.

## Goals

- A consumer-facing `Mission` type whose fields map to what a player or patcher tool actually thinks about.
- **One `Mission` per expansion row, no implicit merging.** Every `Mission` has an unambiguous GUID. The "what counts as the same mission" question becomes a consumer-side pooling decision, not a baked-in equivalence rule.
- **`Pool` API for explicit grouping.** Consumers pick the axis they care about (title key, description key, full equivalence, generator template, …) and receive groups + per-axis divergence flags. v1's `merge_expansions` becomes one of several pooling functions; v1's `title_siblings` becomes "the consumer asks for a title-key pool".
- Generator / handler / sub-contract internals hidden behind `Mission.origin` (lightweight). No `Variation` type — sub-contracts are sibling Missions, distinguished by their fields, optionally linked via `MissionOrigin.subcontract_of`.
- Symmetric tag-bag treatment across all four `SpawnDescription_Ship` tag lists, with raw GUIDs preserved as the escape hatch.
- Encounter widening to cover Ship + NPC + Entity spawn variants under one `Encounter` enum.
- Wave layer kept (waves are a real difficulty signal even when individual wave names aren't presentable).
- Reward fields collapsed into one `MissionRewards` struct.
- Zero loss of v1 information — anything v1 exposed is reachable in v2 (possibly under a new path).
- Crate name unchanged (`sc-contracts`) — type renames don't justify path churn.

## Out of scope (for v2)

- **Mission-objective cargo (delivery / hauling).** `MissionPropertyValue_HaulingOrders` and `MissionPropertyValue_DeliveryOrder` are real poly variants we ignore today, but they're a different shape from spawn-encounters (the cargo *is* the objective, no spawn list). Out of v2; flagged for a future `MissionObjective` type.
- **NPC spawn FPS-context modeling.** v2 surfaces NPC spawns as encounters with their own slot shape; deep classification of FPS-specific fields (closet/room/defendArea tag scopes, faction overrides) is left for a v3 follow-up driven by an actual FPS consumer.
- **A typed `EncounterRole` / `MissionTier` enum.** The role_investigation audit showed every classification we want to expose is derivable from the raw fields we already surface. v2 keeps the bias toward forwarding signals, not pre-classifying. Helper functions (`is_salvage(slot)`, `is_cargo_recovery(slot)`, …) can land alongside but don't go on the type.
- **Crate rename.** `sc-contracts` matches the DCB. Consumers updating their type imports already absorb the breaking change of v2; renaming the path on top is gratuitous churn.

## Decisions

These drive the type sketches below. Each decision is small enough to be revisited individually if the wider design shifts.

### D1. `Contract` → `Mission`, one per expansion row, no implicit merge

The DCB's raw `Contract` record stays raw (reachable via `datacore.db()`). Our consumer type becomes `Mission` and corresponds **1:1 with one `ExpandedContract` row** — that is, one `(generator, handler, contract, optional sub_contract)` tuple. Every `Mission` has a unique GUID with unambiguous identity. `ContractIndex` → `MissionIndex`.

v1's `merge_expansions()` step is removed from the core pipeline. What it did — collapsing rows that share `(title, description, reward_signature)` — becomes one of several pooling axes consumers can opt into (see D8). Different consumers want different equivalence rules; baking one into the type was the wrong call.

Mission count goes from ~1,642 (v1 merged) to ~4,590 (v1 expansion) on SC 4.7. That's the right number of conceptual "things a player can pick up at the contract terminal" — the merge collapsed sub-contract variants that the player perceives as separate.

### D2. Pools as precomputed `MissionIndex` fields

```rust
pub struct MissionIndex {
    pub missions: Vec<Mission>,
    pub by_id: HashMap<Guid, usize>,        // O(1) lookup
    pub pools: MissionPools,                // ← precomputed groupings
    // ... registries, tag_tree
}

pub struct MissionPools {
    pub title_key: HashMap<LocaleKey, Vec<Guid>>,
    pub description_key: HashMap<LocaleKey, Vec<Guid>>,
    pub full_equivalence: HashMap<FullEquivKey, Vec<Guid>>,
    // future axes are non-breaking additions: locality, generator, currency, ...
}

impl MissionIndex {
    pub fn get(&self, id: &Guid) -> Option<&Mission>;
    pub fn iter_pool<'a>(&'a self, ids: &'a [Guid])
        -> impl Iterator<Item = &'a Mission> + 'a;

    // Divergence helpers — opt-in, called per-group only when needed.
    pub fn blueprint_mixed(&self, ids: &[Guid]) -> bool;
    pub fn rewards_uec_consistent(&self, ids: &[Guid]) -> bool;
    pub fn shareable_mixed(&self, ids: &[Guid]) -> bool;
    pub fn once_only_mixed(&self, ids: &[Guid]) -> bool;
    pub fn has_runtime_substitution(&self, ids: &[Guid]) -> bool;
    // ... one per axis; cheap, transparent, replaceable
}
```

Two design choices that justify themselves:

- **Pools are precomputed at index build time, not function calls.** Building `MissionPools` walks the missions once per axis (~14k HashMap inserts on SC 4.7). Memory cost ≈ 100-200 KB. Reading is direct field access. Consumers that don't use a pool still pay for it, but at this scale it doesn't matter, and the API reads as "the index *has* these pools" — closer to documentation than function call.
- **Divergence is opt-in helper methods, not a precomputed struct.** `PoolDivergence` would have eagerly computed every flag for every pool group whether the consumer cared or not. Methods on `MissionIndex` taking `&[Guid]` give the same answers and are cheap; consumers call only what they read.

Pool values are `Vec<Guid>` rather than `Vec<&Mission>` to avoid the self-referential lifetime trap (`MissionPools` lives next to `Mission` inside the same `MissionIndex`). `iter_pool` does the lookup walk — one HashMap probe per element, nothing measurable.

Rationale: sc-langpatch's actual patch axis is the description key, not a merged Mission. A description-key pool maps directly onto its work — group missions by description, decide whether to apply `[BP]` based on whether all members of the pool have a blueprint, render the description block once with mismatch markers when the pool diverges. v1's `find_bp_conflicts` collapses to `index.pools.title_key.iter().filter(|(_, ids)| index.blueprint_mixed(ids))` — same answer, expressed at the right level of abstraction.

### D3. Wave layer kept, renamed `EncounterPhase`

Most encounters today have one unnamed wave with N slots — collapsing those is fine. But Combat Gauntlet missions have ordered `Wave1/Wave2/Wave3`, and bounty escort missions have `EscortReinforcementsWave01/02/03`. Wave count itself is a difficulty signal independent of names: 9 ships across 3 phases is easier than 9 simultaneously. We keep the layer.

Renaming `EncounterWave` → `EncounterPhase` reflects the broader semantic ("one phase of an encounter"), since wave names are sometimes empty / `SalvageableShip` / `Allies` — not always wave-shaped.

### D4. Hide generator / handler / sub-contract structure behind `MissionOrigin`

`Mission.origin: MissionOrigin` exposes a lightweight summary (kind, source debug name, optional sub-contract parent ref). With no merge step there's no merged_count or canonical-expansion-id question — every Mission is its own row.

```rust
pub struct MissionOrigin {
    pub kind: OriginKind,                  // Career / Legacy / List / LinearSeries / Tutorial / ...
    pub source_debug_name: String,         // originating ContractGenerator's debug_name
    pub subcontract_of: Option<Guid>,      // None for top-level; Some(parent_mission_id) for sub-contracts
}
```

Consumers that need the full generator/handler graph reach into `index.raw_expansions` (kept for diagnostics and audit scripts); the default flow doesn't see it.

### D5. `TagBag` symmetry across all four tag lists

```rust
pub struct TagBag {
    pub guids: Vec<Guid>,    // sorted, the escape hatch
    pub names: Vec<String>,  // parallel to guids, sorted
}
```

Every `SpawnDescription_Ship` tag list (positive, negative, markup, entity) gets the same `TagBag`. Consumers can read `bag.names` for display, walk `bag.guids` against the tag tree for custom classification, and `bag.is_empty()` for skipping.

Pre-classification of the positive bag (`factions`, `cargo`, `spawn_identifiers`, `ai_traits`, `mission_tags`, `directives`) becomes **free helper functions** on `TagBag`, taking a `&TagTree` parameter. The `TagTree` is on `MissionIndex` so consumers always have it.

```rust
impl TagBag {
    pub fn factions<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn cargo<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn ai_skill(&self) -> Option<u32>;
    pub fn ace_pilot(&self) -> bool;
    // etc.
}
```

This deletes `SpawnContext` outright. The classifier logic moves into `impl TagBag` methods, the data shape becomes uniform, and we stop materializing 6 `Vec<String>` per slot when a consumer often only needs one.

### D6. Rewards collapse into one struct

```rust
pub struct MissionRewards {
    pub uec: RewardAmount,           // None / Calculated / Fixed(i32)
    pub scrip: Vec<ScripReward>,
    pub reputation: Vec<RepReward>,
    pub items: Vec<ItemReward>,
    pub blueprint: Option<BlueprintReward>,
    pub other: Vec<OtherReward>,
}
```

`Mission.rewards: MissionRewards`. v1's six top-level reward fields become one. The cluster API's `reward_uec` / `reward_scrip` / `reward_rep` / `blueprint_pool` divergence axes still work — they just read through `mission.rewards.*`.

### D7. `Encounter` enum, widened to Ship + NPC + Entity

```rust
pub enum Encounter {
    Ships(ShipEncounter),
    Npcs(NpcEncounter),
    Entities(EntityEncounter),
    /// Variant compiled, not currently extracted — placeholder for a
    /// raw DCB shape we don't model yet. Carries the underlying
    /// `MissionProperty` GUID so consumers can reach the raw layer.
    Unknown { variable_name: String, raw_guid: Guid },
}

pub struct ShipEncounter   { /* name, ext_token, phases: Vec<EncounterPhase> */ }
pub struct NpcEncounter    { /* name, ext_token, phases: Vec<NpcPhase> + AutoSpawnSettings */ }
pub struct EntityEncounter { /* name, ext_token, phases: Vec<EntityPhase> */ }
```

`Mission.encounters: Vec<Encounter>` lets consumers iterate uniformly; pattern-match for typed access. NPC encounters pull the typed `mission_allied_marker: bool` out of `AutoSpawnSettings` and surface it on the encounter / phase — this is what the §1 crimestat work depends on.

### D8. Encounter ordering: preserve DCB-array order, document explicitly

`Mission.encounters` order = source-walk order (sub_contract → contract.paramOverrides → handler → template, first non-empty wins per slot). Within an encounter, phase order = `spawn_descriptions[]` order. We don't sort by name or kind — consumers do that themselves if they want a specific display order.

This is what v1 already does; v2 just promises it explicitly so consumers can rely on it.

## Type sketches

Annotated, not final. Field-level decisions can shift; structure shouldn't.

```rust
//! Top level
pub struct MissionIndex {
    pub missions: Vec<Mission>,
    pub by_id: HashMap<Guid, usize>,       // mission lookup; populated at build
    pub pools: MissionPools,               // precomputed groupings (D2)
    // Registries kept for consumer escape-hatch lookups
    pub locations: LocationRegistry,
    pub localities: LocalityRegistry,
    pub blueprints: BlueprintPoolRegistry,
    pub currencies: RewardCurrencyCatalog,
    pub ships: ShipRegistry,
    pub tag_tree: TagTreeRef,              // borrows from snapshot; needed by TagBag methods
    /// Diagnostic / audit access to the unprocessed expansion rows.
    /// Most consumers ignore this; v1's `expand_sample` example walks
    /// it directly. Each Mission corresponds 1:1 with one row here.
    pub raw_expansions: Vec<ExpandedRow>,
}

pub struct MissionPools {
    pub title_key: HashMap<LocaleKey, Vec<Guid>>,
    pub description_key: HashMap<LocaleKey, Vec<Guid>>,
    pub full_equivalence: HashMap<FullEquivKey, Vec<Guid>>,
    // Future axes land as additional fields without breaking consumers.
}

pub struct Mission {
    pub id: Guid,                          // unique; this is the source ExpandedContract's GUID
    pub debug_name: String,                // for diagnostics; never user-facing
    pub origin: MissionOrigin,             // lightweight "where did this come from"

    // Localization
    pub title: Option<String>,             // resolved
    pub title_key: Option<LocaleKey>,      // raw locale key (sc-langpatch's INI patcher needs this)
    pub description: Option<String>,
    pub description_key: Option<LocaleKey>,
    pub has_runtime_substitution: bool,    // ~mission(...) markers present anywhere in title/desc

    // Policy
    pub shareable: bool,
    pub illegal: bool,
    pub availability: MissionAvailability, // once_only, max_players, hide_in_mobi_glas, cooldowns

    // Where can it run
    pub locality_span: Vec<Guid>,          // LocalityRegistry refs; resolve via index.localities

    // Rewards (collapsed)
    pub rewards: MissionRewards,

    // Encounters (widened)
    pub encounters: Vec<Encounter>,

    // Prereqs (gating conditions)
    pub prerequisites: Vec<PrereqView>,
}

pub struct MissionOrigin {
    pub kind: OriginKind,                  // Career / Legacy / List / LinearSeries / Tutorial / ...
    pub source_debug_name: String,         // the originating ContractGenerator's debug_name
    pub subcontract_of: Option<Guid>,      // None for top-level; Some(parent_mission_id) for sub-contracts
}

//! Pool consumption (replaces v1's variations + title_siblings + find_bp_conflicts)
//
// MissionPools is data on MissionIndex (sketch above); divergence is
// opt-in methods on MissionIndex.

impl MissionIndex {
    pub fn get(&self, id: &Guid) -> Option<&Mission>;
    pub fn iter_pool<'a>(&'a self, ids: &'a [Guid])
        -> impl Iterator<Item = &'a Mission> + 'a;

    // Title-relevant divergence (drives `[BP]` / `[Solo]` / `[Uniq]` tag safety):
    pub fn blueprint_mixed(&self, ids: &[Guid]) -> bool;
    pub fn once_only_mixed(&self, ids: &[Guid]) -> bool;
    pub fn shareable_mixed(&self, ids: &[Guid]) -> bool;
    pub fn illegal_mixed(&self, ids: &[Guid]) -> bool;
    // Description-relevant divergence (drives info-block line safety):
    pub fn rewards_uec_consistent(&self, ids: &[Guid]) -> bool;
    pub fn rewards_scrip_consistent(&self, ids: &[Guid]) -> bool;
    pub fn rewards_rep_consistent(&self, ids: &[Guid]) -> bool;
    pub fn blueprint_pool_consistent(&self, ids: &[Guid]) -> bool;
    pub fn locality_span_consistent(&self, ids: &[Guid]) -> bool;
    pub fn encounters_shape_consistent(&self, ids: &[Guid]) -> bool;
    pub fn cooldowns_consistent(&self, ids: &[Guid]) -> bool;
    // Cross-cutting:
    pub fn has_runtime_substitution(&self, ids: &[Guid]) -> bool;
    pub fn origin_kind_mixed(&self, ids: &[Guid]) -> bool;
}

pub struct MissionAvailability {
    pub once_only: bool,
    pub max_players: Option<u32>,
    pub hide_in_mobi_glas: bool,
    pub cooldowns: Cooldowns,
}

pub struct MissionRewards {
    pub uec: RewardAmount,
    pub scrip: Vec<ScripReward>,
    pub reputation: Vec<RepReward>,
    pub items: Vec<ItemReward>,
    pub blueprint: Option<BlueprintReward>,
    pub other: Vec<OtherReward>,
}

//! Encounter side
pub enum Encounter {
    Ships(ShipEncounter),
    Npcs(NpcEncounter),
    Entities(EntityEncounter),
    Unknown { variable_name: String, raw_guid: Guid },
}

pub struct ShipEncounter {
    pub variable_name: String,
    pub extended_text_token: String,       // "AmbushTarget" etc, often empty
    pub phases: Vec<EncounterPhase>,       // was `waves`
}

pub struct EncounterPhase {
    pub name: String,                      // "Wave1" / "SalvageableShip" / "" / ...
    pub slots: Vec<EncounterSlot>,
}

pub struct EncounterSlot {
    pub concurrent: i32,
    pub weight: f32,
    pub candidates: Vec<ShipCandidate>,
    pub initial_damage_settings: Option<Guid>,
    pub include_location_ai_spawn_tags: bool,

    /// Four sibling tag lists, all symmetric.
    pub positive: TagBag,
    pub negative: TagBag,
    pub markup: TagBag,
    pub entity: TagBag,
}

pub struct TagBag {
    pub guids: Vec<Guid>,
    pub names: Vec<String>,
}

impl TagBag {
    pub fn is_empty(&self) -> bool;

    // Tag-tree-driven classifiers (replace v1 SpawnContext fields)
    pub fn factions<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn cargo<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn spawn_identifiers<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn ai_traits<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn mission_tags<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str>;
    pub fn directives<'a>(&'a self) -> impl Iterator<Item = &'a str>;
    pub fn ai_skill(&self) -> Option<u32>;
    pub fn ace_pilot(&self) -> bool;

    // High-level mission-shape predicates derived from the analytics
    // findings — opt-in helpers, kept on the type so consumers don't
    // re-derive them. Take &TagTree because they walk the subtree.
    pub fn is_salvage_target(&self, tree: &TagTree) -> bool;        // AvailableToSalvage tag
    pub fn is_cargo_recovery(&self, tree: &TagTree) -> bool;        // EnableInteractions + EmptyCrew
    pub fn is_pre_damaged_wreck(&self, tree: &TagTree) -> bool;     // DisablePowerInteractions
}

pub struct NpcEncounter {
    pub variable_name: String,
    pub extended_text_token: String,
    pub mission_allied_marker: bool,        // typed Allied/Hostile signal §1 needs
    pub phases: Vec<NpcEncounterPhase>,
}
// NpcEncounterPhase + NpcSlot mirror the Ship side's shape with FPS-relevant fields.
// Detailed FPS-side modeling is v3 work.

pub struct EntityEncounter { /* same shape, generic entity slot */ }
```

## Worked migration: `bp_title_conflicts` example

v1 today:

```rust
let index = ContractIndex::build(&datacore, &locale);
for c in &index.contracts {
    if let Some(bp) = &c.blueprint_reward {
        let title = c.title.as_deref().unwrap_or(&c.debug_name);
        let key = c.title_key.as_ref().map(|k| k.as_str()).unwrap_or("");
        // ...
    }
}
let conflicts = sc_contracts::find_bp_conflicts(&index.contracts);
```

v2:

```rust
let index = MissionIndex::build(&datacore, &locale);

// Per-mission iteration looks the same modulo rename + reward grouping:
for m in &index.missions {
    if let Some(bp) = &m.rewards.blueprint {
        let title = m.title.as_deref().unwrap_or(&m.debug_name);
        let key = m.title_key.as_ref().map(|k| k.as_str()).unwrap_or("");
        // ...
    }
}

// Conflict detection is now a precomputed-pool lookup, not a bespoke helper:
let conflicts: Vec<_> = index
    .pools
    .title_key
    .iter()
    .filter(|(_, ids)| index.blueprint_mixed(ids))
    .collect();
```

`find_bp_conflicts` disappears. The same answer falls out of `pools.title_key + blueprint_mixed`. Equivalent helpers for scrip, rep, locality, etc. live on `MissionIndex` and are trivial to add — each one is a one-liner over `&[Guid]`.

## Worked migration: sc-langpatch description-block patcher

This is what motivated the pool API. v1 made sc-langpatch synthesize the right grouping itself by walking `Contract.title_siblings` and re-checking equivalence; v2 hands the grouping over directly via `index.pools.description_key`.

v2:

```rust
// Patcher's actual axis: the description key, because that's what
// gets written into global.ini.
for (key, ids) in &index.pools.description_key {
    let block = render_description_block(&index, ids);
    if index.has_runtime_substitution(ids) {
        // Engine fills ~mission(...) at spawn time — annotate, don't lie.
    }
    write_ini_entry(key, &block);
}

fn render_description_block(index: &MissionIndex, ids: &[Guid]) -> String {
    let mut out = String::new();
    let head = index.get(&ids[0]).unwrap();

    // Single value across all members → render confidently.
    // Mixed → either render with mismatch markers or skip the line.
    if index.rewards_uec_consistent(ids) {
        out.push_str(&format_uec(head.rewards.uec));
    }
    if head.rewards.blueprint.is_some() && index.blueprint_pool_consistent(ids) {
        out.push_str(&format_blueprint(head.rewards.blueprint.as_ref().unwrap()));
    } else if index.blueprint_mixed(ids) {
        out.push_str("[BP*]");  // some but not all members have a blueprint
    }
    // ...
    out
}
```

The structure of the patcher is now visible at a glance: iterate the pool by the patch-axis, ask the index whether each axis is consistent, render conservatively where it isn't, write once.

## Worked migration: explorer encounter rendering

v1 today (excerpt):

```rust
for g in &c.encounters {
    ui.text(format!("  ▸ {}", g.variable_name));
    for w in &g.waves {
        for slot in &w.slots {
            if !slot.context.factions.is_empty() {
                ui.text(format!("    factions: {}", slot.context.factions.join(", ")));
            }
            // ... 5 more buckets
        }
    }
}
```

v2:

```rust
for enc in &m.encounters {
    match enc {
        Encounter::Ships(s) => render_ship_encounter(ui, s, &index.tag_tree),
        Encounter::Npcs(s)  => render_npc_encounter(ui, s, &index.tag_tree),
        Encounter::Entities(s) => render_entity_encounter(ui, s, &index.tag_tree),
        Encounter::Unknown { variable_name, .. } => {
            ui.text(format!("  ▸ {} (unmodeled)", variable_name)).dim();
        }
    }
}

fn render_ship_encounter(ui: &mut Context, s: &ShipEncounter, tree: &TagTree) {
    ui.text(format!("  ▸ {}", s.variable_name));
    for phase in &s.phases {
        for slot in &phase.slots {
            let factions: Vec<&str> = slot.positive.factions(tree).collect();
            if !factions.is_empty() {
                ui.text(format!("    factions: {}", factions.join(", ")));
            }
            // negative/markup/entity bags now visible too — uniform shape:
            if !slot.negative.is_empty() {
                ui.text(format!("    excluded: {}", slot.negative.names.join(", ")));
            }
        }
    }
}
```

The match adds a few lines but lets us render NPC and Entity encounters distinctly. Tag-classifier methods are slightly more verbose at call sites (`slot.positive.factions(tree).collect()`) than v1's `slot.context.factions`, but the data only materializes when asked for.

## Phasing

The refactor is large enough to warrant landing it in checkpoints. Each phase compiles + tests + leaves the workspace in a working state.

1. **Phase 1 — TagBag symmetry on existing Encounter shape.** ✅ landed `a00a0d1`. Replaced `SpawnContext` with `TagBag` + classifier methods, surfaced negative/markup/entity bags symmetrically.
2. **Phase 2 — Reward consolidation.** ✅ landed `d2668a3`. Six top-level reward fields on Contract collapsed into one `MissionRewards` struct.
3. **Phase 3 — Pool fields + divergence methods.** ✅ landed `4d74d8f`. Added `MissionPools` to ContractIndex with `title_key` / `description_key` HashMaps + opt-in divergence methods (`blueprint_mixed`, `rewards_uec_consistent`, …). Cluster API deprecated.
4. **Phase 4 — Remove implicit merge from the pipeline.** ✅ landed `c636f51`. `ContractIndex.contracts` now holds `Vec<ExpandedContract>` directly (4,590 rows on SC 4.7 vs the 1,642 merged). `merge.rs`, `clusters.rs`, `Variation`, `title_siblings`, `find_bp_conflicts`, the cluster API and `tui/clusters.rs` all deleted.
5. **Phase 5 — `Contract` → `Mission` rename.** ✅ landed `b271e82`. `ExpandedContract → Mission`, `ContractIndex → MissionIndex`. Three handler fields + `ContractOrigin` enum consolidated into `MissionOrigin` struct with `subcontract_of: Option<Guid>`.
6. **Phase 6 — `Encounter` enum + NPC widening.** ⏳ pending. Convert ship-only `Vec<EncounterGroup>` into `Vec<Encounter>` with `Ships / Npcs / Entities / Unknown` variants. NPC extraction surfaces `mission_allied_marker` (the typed Allied/Hostile signal sc-langpatch §1's crimestat work needs); Entity extraction can land in the same phase or follow.
7. **Phase 7 — `EncounterWave` → `EncounterPhase` rename.** ⏳ pending. Smallest cosmetic; saved for last so it's easy to revert if real-world consumer feedback prefers `Wave`.

Phases 1–4 reshape the data model; phase 5 is rename-only; phase 6 adds new data; phase 7 is naming.

Phase order matters: shipping the Pool API (3) **before** removing the merge (4) means consumers can migrate off `find_bp_conflicts` / `title_siblings` *before* those concepts vanish, instead of being forced to do both at once.

## Open questions

- **Divergence helpers vs raw difference enumeration.** `MissionIndex::*_mixed` / `*_consistent` methods return bools — quick to read, but lose *what* differs (e.g., `rewards_uec_consistent` returns false but doesn't say what the differing values are). For most consumer logic the bool is enough; sc-langpatch only needs to know whether to render-or-omit a line. If a consumer wants the actual deltas they iterate `iter_pool(ids)` and compare. Acceptable, or should one or two helpers also return a `Vec<T>` of distinct values for richer rendering?
- **`is_salvage_target` / `is_cargo_recovery` predicates.** The encounter_analytics finding (`EnableInteractions` ↔ `DisablePowerInteractions` flip) is a clean discriminator, but baking it into `TagBag` methods commits to the current 4.7 game shape. If CIG reorganizes the `AI ▸ ...` subtree in 5.0 these silently break. Acceptable risk, or should helpers live in a separate `sc_contracts::heuristics` module so it's clear they're discovery-driven not part of the type contract?
- **`include_location_ai_spawn_tags`** — keep on slot, or fold into a `merges_at_runtime: bool` flag and document the runtime merging once at the encounter level? It's a per-slot field in the DCB but most consumers will treat it uniformly within a phase.
- **NPC encounter shape detail.** How much FPS-side classification belongs in v2 vs v3? The phase-2 sc-langpatch use case only needs `mission_allied_marker` and a basic slot count. Closet/room/defendArea tag scopes can probably wait.

## Cross-references

- Driving-consumer feature requests:
  - [`docs/feature-request-sc-contracts-langpatch.md`](feature-request-sc-contracts-langpatch.md) §1 (crimestat) needs NPC encounters; §2 (EncounterRole) consumes the post-v2 surface; §4 (empty candidates diagnostics) layers on top.
- v1 reference docs (kept):
  - [`docs/sc-contracts.md`](sc-contracts.md) — v1 design spec.
  - [`docs/sc-contracts-guide.md`](sc-contracts-guide.md) — current consumer guide; updates wholesale once v2 lands.
- Audit scripts driving the design:
  - [`crates/sc-contracts/examples/role_investigation.rs`](../crates/sc-contracts/examples/role_investigation.rs)
  - [`crates/sc-contracts/examples/encounter_analytics.rs`](../crates/sc-contracts/examples/encounter_analytics.rs)
  - [`crates/sc-contracts/examples/tier_investigation.rs`](../crates/sc-contracts/examples/tier_investigation.rs)
