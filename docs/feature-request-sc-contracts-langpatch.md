# Feature request — `sc-contracts` gaps surfaced by sc-langpatch migration

> **Consumer:** `sc-langpatch` (`mission_enhancer` module)
> **Filed:** 2026-04-24, revised 2026-05-01 after the typed-difficulty audit ([`tier_investigation.rs`](../crates/sc-contracts/examples/tier_investigation.rs)).
> **Status:** Five sections proposed. The originally-filed §1 (localization keys on `Contract` / `Variation`) shipped in `sc-holotable/v0.1.0` and is removed from this list. The originally-filed `MissionTier` and `Variation.variant_label` sections are dropped per the typed-difficulty finding (see §3).
> **Driver:** Migration of `mission_enhancer` from raw svarog walks to `sc-contracts` v1, then redesign of the description-block layout to render one block per meaningful contract variant.

## Background

`sc-langpatch`'s `mission_enhancer` annotates `global.ini` mission text — adds `[BP]` / `[Solo]` / `[Uniq]` / `[CS Risk]` tags to titles and a multi-line block (blueprint list, cooldowns, rep / scrip rewards, ship encounters) to descriptions.

The migration to `sc_contracts::ContractIndex` collapsed the module from ~1460 lines to ~480. Almost everything the old code did by hand — generator/handler/contract walking, blueprint pool resolution, ship-tag intent matching, AI skill parsing, mixed-BP detection — comes for free from the `Contract` model and `find_bp_conflicts`.

Five pieces still require raw-svarog walks or per-consumer heuristics. They're tagged `TODO(sc-holotable)` in the consumer code and filed here.

## 1. Crimestat risk on `Contract`

### What's missing

Some PvE missions tell you to stop civilians or escort allies. If the player misclicks and shoots one, the game stamps a crimestat. sc-langpatch surfaces this as `[CS Risk]` (moderate, with HUD markers to help) or `[CS Risk!]` (high, no markers). Players have repeatedly asked for it because the in-game briefing doesn't make the consequence clear.

Two signals decide the risk level:

1. **`DontHarmAllies_BP` / `BP_DontHarmAllies` / `DontHarmCivs_BP` / `BP_DontHarmCivs`** — boolean mission properties. Set means "this mission penalises killing allies/civs."
2. **`MissionPropertyValue_NPCSpawnDescriptions → spawnDescriptions[].options[].autoSpawnSettings.missionAlliedMarker`** — true means the spawn carries the friendly HUD marker.

Risk decoded from the combination:

| `DontHarm*` | `missionAlliedMarker` | Result |
|---|---|---|
| set | true (anywhere) | Moderate — friendlies marked, easy to identify |
| set | false (nowhere) | High — friendlies present, no markers |
| unset | true | Moderate — allies in space, collision/stray-fire risk |
| unset | false | None |

### Current consumer workaround

```rust
// sc-langpatch/src-tauri/src/modules/mission_enhancer.rs
fn crimestat_for_contract(db: &DataCoreDatabase, id: Guid) -> CrimestatRisk {
    // Walk paramOverrides.propertyOverrides[] looking for DontHarm* missionVariableNames
    // and MissionPropertyValue_NPCSpawnDescriptions with autoSpawnSettings.missionAlliedMarker.
    // Falls back to template.contractProperties when paramOverrides has nothing.
}
```

~80 lines preserved verbatim from the pre-migration code. The walk is tedious because `propertyOverrides` is a poly array with many subclasses; we only care about two of them.

### Proposed shape

Add to `Contract`:

```rust
pub struct Contract {
    // … existing fields …
    /// Crimestat risk derived from `DontHarm*` properties + spawn-side
    /// `missionAlliedMarker` flags. `None` for missions without friendlies.
    pub crimestat_risk: CrimestatRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrimestatRisk {
    None,
    /// Friendlies present *with* HUD markers — collision / stray-fire risk only.
    Moderate,
    /// Friendlies present *without* HUD markers — cannot distinguish friend from foe.
    High,
}
```

Computation slots naturally next to `expand.rs`'s existing per-contract walk through `paramOverrides.propertyOverrides[]` — most of the property visit already happens for prereq + encounter extraction. Adding two more match arms (one per signal) is essentially free at extraction time.

### Open questions

- **Does any contract ship `DontHarm*` on a sub-contract (variation) instead of the parent?** If yes, `Variation.crimestat_risk` may also need to differ. From a quick scan of sc-langpatch's pre-migration data the parent always wins, but worth confirming before locking the API.
- **Naming.** `crimestat_risk` matches the existing `illegal_flag` naming convention on `Contract`.
- **Should this live on `Variation` only, with `Contract.crimestat_risk` synthesized as the max?** Probably yes if (a) above is real.

### Validation targets

When implemented, the following sc-langpatch fixtures should produce the expected risk level:

- **Career bounty contracts with civilian witnesses** (e.g. `PU_CFP_Hunter_Quest`-style series) → High when no HUD markers.
- **Escort / defend missions** (`*_DefendShip_*`, `*_Escort_*`) → Moderate (markers present).
- **Pure pirate-vs-pirate bounties** (target NPCs, no allies) → None.

## 2. `EncounterRole` on `EncounterGroup`

### What's missing

`EncounterGroup.variable_name` is a free-form `String` carrying schema noise like `Hostile_ShipSpawnDescriptions`, `SalvageSpawnDescription_BP`, `MissionTargets`, `WaveShips`, `EscortShip`, `ShipToDefend`, `BP_SpawnDescriptions`. Every consumer that wants to display a contract needs to classify it into a small set of roles (`Hostile`, `Allied`, `Salvage`, `EscortTarget`, `Reinforcement`, `Wave(N)`, …) and ends up sniffing substrings. That's both error-prone and per-consumer duplicated work.

### Current consumer workaround

```rust
// sc-langpatch/src-tauri/src/modules/mission_enhancer.rs
fn classify_role(var_name: &str) -> &'static str {
    let lower = var_name.to_lowercase();
    if lower.contains("salvage") { "Salvage" }
    else if lower.contains("hostile")
        || lower.contains("missiontargets")
        || lower.contains("waveships") { "Hostile" }
    else if lower.contains("attacked")
        || lower.contains("allied")
        || lower.contains("escort")
        || lower.contains("defend") { "Allied" }
    else { "Hostile" } // default for unknown shapes — unsafe
}

fn meaningful_sub_role(var_name: &str) -> Option<String> {
    // sniff `Wave\d+` and `ShipToDefend`/`Escort` out of the name
}
```

The "default Hostile" fallback misclassifies new mission types whenever the DCB grows a new naming pattern.

### Proposed shape

Add to `EncounterGroup`:

```rust
pub struct EncounterGroup {
    /// Raw mission variable name (kept for diagnostics).
    pub variable_name: String,
    /// Classified role of this spawn group.
    pub role: EncounterRole,
    /// Optional sub-role for grouping waves / escort / reinforcement
    /// flavours. None when the role itself fully describes the group.
    pub sub_role: Option<EncounterSubRole>,
    pub waves: Vec<EncounterWave>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncounterRole {
    /// Enemy ships the player engages.
    Hostile,
    /// Friendly ships fighting alongside the player (not the escort target).
    Allied,
    /// Wrecks / derelicts the player salvages.
    Salvage,
    /// Player-protected ship (the convoy ship in escort missions).
    EscortTarget,
    /// Naming pattern not recognised by the upstream classifier.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncounterSubRole {
    /// Ordered combat wave (Wave 1, Wave 2, …).
    Wave(u8),
    /// Reinforcements that arrive after a trigger.
    Reinforcement,
}
```

Classification rule lives in `expand.rs::collect_property_encounters` (next to where `variable_name` is read), so every consumer sees the same answer. The `Unknown` variant is the safety valve: when a new DCB name shape lands, the consumer renders `Encounter` (generic) instead of misclassifying as `Hostile`.

### Failure mode

- New / unknown variable name → `EncounterRole::Unknown`, `sub_role: None`. Consumer should treat that as a generic encounter and not infer hostility.

### Validation targets

| `variable_name` | Expected `(role, sub_role)` |
|---|---|
| `Hostile_ShipSpawnDescriptions` | `(Hostile, None)` |
| `Wave1_ShipSpawnDescriptions` | `(Hostile, Some(Wave(1)))` |
| `WaveShips_BP` | `(Hostile, None)` |
| `SalvageSpawnDescription_BP` | `(Salvage, None)` |
| `EscortShip` | `(EscortTarget, None)` |
| `ShipToDefend` | `(EscortTarget, None)` |
| `Allied_ShipSpawnDescriptions` | `(Allied, None)` |
| `Reinforcements_ShipSpawn` | `(Allied, Some(Reinforcement))` |
| `BP_SpawnDescriptions` (no role keyword) | `(Unknown, None)` |

## 3. `Contract.difficulty` — typed difficulty scales

### Background — why the originally-filed `MissionTier` was dropped

The original draft of this doc proposed a `Contract.tier: Option<MissionTier>` enum mapping bounty contracts to CIG's named risk tiers (`Low`, `Moderate`, `High`, `VeryHigh`, `Extreme`) plus `Numbered(u8)` for `Tier 1 / Tier 2 / …` families. The classifier was specced to walk typed DCB difficulty fields with a `debug_name` substring fallback — same pattern sc-langpatch uses today.

The audit script [`crates/sc-contracts/examples/tier_investigation.rs`](../crates/sc-contracts/examples/tier_investigation.rs) ran against SC 4.7 LIVE to confirm the typed source. Findings:

- `ContractDifficulty` records carry four `1-7` scales: `risk_of_loss`, `mechanical_skill`, `mental_load`, `game_knowledge`. Coverage: **99.2% of `CareerContract`**, **65.5% of `Contract`**, 0% of `ContractLegacy`.
- The four scales **do** vary across tiered siblings — but `VeryEasy` and `Easy` of the same family collapse to identical quadruples. The typed data is **coarser** than CIG's named tiers.
- `SubContract` has no `contract_results` field, so all sub-contracts in a family share the parent's single `ContractDifficulty` record. (One `Contract` in 4.7 has 29 sub-contracts; two have 11.)
- The `Tag` tree carries `VeryEasy` / `VeryHard` etc. on entities, not contracts — those are AI-difficulty markers, not mission-tier markers.
- `ContractIntParamType` and `ContractBoolParamType` have no difficulty/tier/risk variants — only cooldowns and behavioural flags.

Verdict: typed difficulty data exists and is high-quality but doesn't cleanly map to CIG's five named tiers. Forcing it would require the same `debug_name` substring matching it was meant to replace, which violates the workspace rule §5 ("no string matching where typed / data-derived alternatives exist").

The originally-filed `Variation.variant_label` proposal had the same root problem — the dominant axis of variation in tiered families is the named tier, which we just established we can't recover. Dropped along with `MissionTier`.

### What we surface instead

Expose the typed scales raw and let consumers decide labelling policy.

```rust
pub struct Contract {
    // … existing fields …
    /// Typed difficulty profile resolved through
    /// `ContractResults → ContractDifficulty`. `None` when the contract
    /// chain doesn't reference one (every `ContractLegacy`, ~35% of
    /// plain `Contract`).
    pub difficulty: Option<ContractDifficultyView>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractDifficultyView {
    /// GUID of the underlying `ContractDifficulty` record, so consumers
    /// that want to dedupe across siblings can group by profile.
    pub profile_guid: Guid,
    /// CIG's `1-7` scales. Coarser than the named tiers (VeryEasy/Easy
    /// of the same family resolve to identical quadruples).
    pub risk_of_loss: u8,
    pub mechanical_skill: u8,
    pub mental_load: u8,
    pub game_knowledge: u8,
}
```

Inheritance: the parent's `ContractDifficulty` is propagated to all sub-contract `Variation`s during merge, since `SubContract` itself has no `contract_results` field.

### Consumer guidance

`risk_of_loss` is the closest single signal to "named tier" but is not a substitute. Recommended consumer patterns:

- **Sort sibling clusters from low to high:** sort by `(risk_of_loss, mechanical_skill, mental_load, game_knowledge)`.
- **Render a coarse difficulty hint:** map `risk_of_loss ∈ {1, 2}` → "Low risk", `{3, 4}` → "Moderate risk", `{5, 6, 7}` → "High risk". Players will recognise the hint without needing CIG's exact label.
- **Recover named tiers (sc-langpatch's `tier_label`):** keep the existing `debug_name` substring match local to the consumer. The named-tier vocabulary is CIG's UX choice, not a data fact, and labelling is presentation logic, not data shape.

### Failure mode

- `Contract.difficulty == None` → consumer skips the difficulty hint or uses the substring fallback. Roughly 35% of plain `Contract` records and 100% of `ContractLegacy`.

### Validation targets

- A bounty family with 5 risk variants → 5 distinct `ContractDifficultyView` profile GUIDs (or fewer if VeryEasy/Easy collapse — flag in audit).
- A `CareerContract` with sub-contracts → the parent's `ContractDifficultyView` propagated to every `Variation`.
- A `ContractLegacy` → `difficulty: None`.

The audit script [`crates/sc-contracts/examples/tier_investigation.rs`](../crates/sc-contracts/examples/tier_investigation.rs) is the reproducible evidence; running it against a future patch verifies the data shape hasn't drifted.

## 4. Empty `EncounterSlot.candidates` — diagnostics + broadening

### What's missing

`ShipRegistry::resolve_spawn` returns an empty `candidates` vec for some spawn slots — even after enabling `entityclassdefinition` and feeding a populated base-locale `DisplayNameCache`. `sc-contracts/src/expand.rs` already documents the case as "Empty when the query is broken (e.g. Gilly Mission01 Wave3's typo'd `Relient_Tana` tag) or awaits runtime location context." Consumers can't currently distinguish the failure modes:

- **Typo'd tag** — DCB bug; matching is impossible.
- **Tag-only spawn** (e.g. `LargeCombatShip` + faction, no ship-selective tag) — query is valid but `resolve_spawn` doesn't broaden.
- **Runtime location context** — engine fills the pool at spawn time; static analysis can't.
- **Feature gate gap** — relevant `EntityClassDefinition`s aren't compiled in.

The hard requirement on the consumer side is: **never render a generic placeholder like "capital ships" / "ships" / "transport/cargo" in mission text.** Either resolve a specific list, or omit the encounter block.

### Asks

1. **Broaden `resolve_spawn`** to fall back to a tag-set match (size + faction + role) when no ship-selective tags are present, returning the full pool that would match. This lets sc-langpatch render `1× (Cutlass Black, Cutlass Red, Drake Caterpillar)` instead of nothing.
2. **Surface diagnostic info** on the slot — an `EncounterSlot.resolution: SlotResolution` enum so consumers can tell why candidates is empty:
   ```rust
   pub enum SlotResolution {
       Resolved,                         // candidates populated normally
       BroadenedFromTags,                // pool synthesized from size+faction tag match
       UnrecognisedTag(Vec<String>),     // tag(s) didn't match any registered entity (likely DCB typo)
       RuntimeContextRequired,           // engine fills at spawn time; we can't
       FeatureGated,                     // EntityClassDefinition feature not enabled
   }
   ```
3. **Document** which feature flags downstream consumers must enable for ship resolution. `entityclassdefinition` alone may not be sufficient if specific entity buckets (`entities-spaceships`, `entities-scitem-ships`, …) are also required.

### Consumer behaviour today

sc-langpatch deletes its `classify_tag_only` helper and skips any slot/group with empty candidates. If that means a contract has no encounter info displayed, that's preferable to wrong info — and the missing data flags an upstream gap to investigate via `SlotResolution`.

## 5. `SpawnContext` typed cargo / value / legality (low priority)

### What's missing

`SpawnContext.cargo: Vec<String>` holds tag names like `"Full Cargo"`, `"Half Cargo"`, `"Scraps Cargo"`, `"Minimal Cargo"`, `"Empty Cargo"`; `SpawnContext.ai_traits: Vec<String>` holds `"HighValue"`, `"MediumValue"`, `"LowValue"`, `"Mixed"`, `"Legal"`, `"Illegal"` (and unrelated noise like `"Bounty"`, `"General"` that consumers must filter out). Every consumer that wants to render plunder-relevant info ends up writing the same whitelist:

```rust
// sc-langpatch
fn cargo_volume_label(t: &str) -> Option<&'static str> {
    matches!(t, "Full Cargo" | "Half Cargo" | "Scraps Cargo" | "Minimal Cargo" | "Empty Cargo")
        .then_some(t)
}
fn value_tier_label(t: &str) -> Option<&'static str> { /* HighValue/MediumValue/LowValue/Mixed */ }
fn legality_label(t: &str)    -> Option<&'static str> { /* Legal/Illegal/Ilegal-typo'd */ }
```

This isn't dangerous (the tags come from a stable tag tree, not free-form names), but it's per-consumer duplication.

### Proposed shape

Add typed projections to `SpawnContext`:

```rust
pub struct SpawnContext {
    // … existing fields, kept for compatibility …
    pub cargo_volume: Option<CargoVolume>,
    pub value_tier: Option<ValueTier>,
    pub legality: Option<Legality>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CargoVolume { Full, Half, Scraps, Minimal, Empty }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueTier { High, Medium, Low, Mixed }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Legality { Legal, Illegal }
```

Resolved during `SpawnContext::classify` from the same tag-tree subtree walk that already populates `cargo` / `ai_traits`. Keep the raw `cargo: Vec<String>` and `ai_traits: Vec<String>` fields for forward compatibility — consumers just gain a typed shortcut.

### Priority

Low — current local whitelists work and the tag names are stable. File so the request isn't lost; do not block sc-langpatch on it.

## Priority

In sc-langpatch terms: **none of these block the migration.** The raw walks and substring heuristics work and are isolated.

Loose ranking:

1. **§1 Crimestat risk** — biggest code reduction (~80 lines), real domain modelling in sc-contracts' wheelhouse.
2. **§2 EncounterRole** — small but eliminates a fragile substring classifier shared across consumers.
3. **§3 Contract.difficulty** — typed-data lift; trivial to implement (the chain is `ContractResults → ContractDifficulty` and the latter is a flat record).
4. **§4 Empty-candidate diagnostics** — non-trivial; requires deciding the broadening heuristic. Defer until at least one other consumer asks.
5. **§5 Typed SpawnContext** — quality-of-life only.

Reasonable bundling: §1 + §2 + §3 land together as a "Phase A" pass — they all touch `expand.rs` in adjacent code, none affects the public model in a way that breaks existing consumers.

## Cross-references

- Consumer code with `TODO(sc-holotable)` markers: `E:\vscode\rust\sc-langpatch\src-tauri\src\modules\mission_enhancer.rs`
- Property-walk integration point for crimestat: `crates/sc-contracts/src/expand.rs` (per-contract `propertyOverrides` traversal)
- Encounter-role classifier integration point: `crates/sc-contracts/src/expand.rs::collect_property_encounters` (where `variable_name` is read)
- Difficulty audit script: `crates/sc-contracts/examples/tier_investigation.rs`
- Tag-tree subtree classifier (cargo / value / legality): `crates/sc-contracts/src/classify.rs::SpawnContext::classify`
- Empty-candidate diagnostics: `crates/sc-contracts/src/ships.rs::ShipRegistry::resolve_spawn`
- Existing `Contract` surface: `docs/sc-contracts-guide.md` §"The `Contract` model"
- Sister request for the weapons module: `docs/feature-request-sc-weapons-langpatch.md`
