# `sc-contracts` — consumer guide

> **Who this is for.** Rust code that wants to render / analyse Star Citizen contracts — primarily `sc-langpatch`, but anything that consumes the crate. If you want the design rationale, see [sc-contracts.md](sc-contracts.md); this file is the "how do I use it" guide.

## TL;DR

```rust
use sc_contracts::ContractIndex;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

let install = sc_installs::discover_primary()?;
let assets  = AssetSource::from_install(&install)?;
let data    = AssetData::extract(&assets, &AssetConfig::standard())?;
let dc      = Datacore::parse(&assets, &data, &DatacoreConfig::standard())?;

let index = ContractIndex::build(&dc, &data.locale);

for c in &index.contracts {
    if let Some(title) = &c.title {
        println!("{}: {} rewards", title, summarise_rewards(c));
    }
}
```

That's the whole API surface in one screen. The rest of this document explains what you get and how to use each field.

---

## Cargo setup

```toml
[dependencies]
sc-contracts = { git = "…", rev = "…" }
sc-extract   = { git = "…", rev = "…", features = ["contracts", "servicebeacon"] }
sc-installs  = { git = "…", rev = "…" }
```

`sc-contracts` enables the required `contracts` + `servicebeacon` features on `sc-extract` transitively, but the consumer still needs its own `sc-extract` dependency to hold the `Datacore` it feeds in. Use the same pinned git commit across all three.

## The four-stage pipeline (conceptual only)

You usually don't touch these — `ContractIndex::build` runs all four. They are named so that when you read a field comment that says "populated during stage 3" you know what that means.

1. **Ingest** — build registries (ships, blueprints, currency, locations, localities) from the `Datacore`.
2. **Expand** — walk every `ContractGenerator`, emit one `ExpandedContract` per `(handler, contract, optional sub_contract)` node. ~4,900 rows on SC 4.7 LIVE.
3. **Resolve** — fill reward / availability / cooldown / prereq / encounter fields on each expansion (integrated with stage 2).
4. **Merge** — group expansions by `(title, description, reward_signature)` into `Contract` entries. ~1,640 rows on SC 4.7 LIVE (SCMDB target is ~1,497; we run 9.7% high, see §Merge behavior).

## `ContractIndex`

```rust
pub struct ContractIndex {
    pub contracts:  Vec<Contract>,
    pub ships:      ShipRegistry,
    pub blueprints: BlueprintPoolRegistry,
    pub currency:   RewardCurrencyCatalog,
    pub locations:  LocationRegistry,
    pub localities: LocalityRegistry,
}
```

All fields are `pub` — `ContractIndex` is a data bundle, not an opaque handle. If you only need a subset, destructure at build time and drop the rest:

```rust
let ContractIndex { contracts, localities, blueprints, .. } = ContractIndex::build(&dc, &locale);
```

Accessors beyond direct field access:

| Method | Purpose |
|---|---|
| `index.get(id)` | O(1) lookup by canonical contract GUID (the parent contract's GUID). |
| `index.get_by_variation(id)` | O(n) lookup that also matches sub-contract GUIDs. Use when you have a raw DCB GUID and don't know which kind. |
| `index.iter()` | Iterate every merged contract in index order. |
| `index.len()` / `is_empty()` | Standard. |

## The `Contract` model

One `Contract` = one logical mission. Its `variations: Vec<Variation>` holds every `ExpandedContract` that collapsed into this entry; the canonical fields at the top level are taken from the first variation (preferring a non-sub-contract origin).

```rust
pub struct Contract {
    pub id:              Guid,                  // canonical contract GUID
    pub debug_name:      String,                // internal DCB name (cross-ref with SCMDB)
    pub generator_id:    Guid,
    pub handler_kind:    HandlerKind,           // Legacy / Career / List / LinearSeries / Tutorial / …
    pub handler_debug_name: String,

    pub title:       Option<String>,            // resolved locale text
    pub description: Option<String>,
    pub has_runtime_substitution: bool,         // text contains `~mission(...)` markers

    pub shareable:   bool,
    pub illegal_flag: bool,
    pub availability: Availability,             // once_only + cooldowns + flags

    pub reward_uec:      RewardAmount,          // None / Calculated / Fixed(i32)
    pub reward_scrip:    Vec<ScripReward>,      // MG Scrip / Council Scrip
    pub reward_rep:      Vec<RepReward>,        // reputation deltas
    pub reward_items:    Vec<ItemReward>,       // non-currency items
    pub reward_other:    Vec<OtherReward>,      // badges / journal / etc.
    pub blueprint_reward: Option<BlueprintReward>,

    pub prerequisites:   Vec<PrereqView>,       // common across all variations
    pub encounters:      Vec<EncounterGroup>,   // ship spawns

    pub variations:      Vec<Variation>,        // 1+; per-variation extras
    pub mission_span:    Vec<Guid>,             // locality GUIDs (look up via index.localities)
    pub title_siblings:  Vec<Guid>,             // other Contracts sharing (title, description)
}
```

### Titles, descriptions, and the `~mission(…)` trap

`title` and `description` are localized strings resolved through a 4-level inheritance chain (`sub_contract → contract.paramOverrides → handler.contractParams → template.stringParamOverrides`). Either can be `None` when no level in the chain supplied text.

**Runtime substitution.** If `has_runtime_substitution == true`, the engine fills `~mission(variable)` markers at spawn time with values that vary per in-game instance. Examples:

- `~mission(TargetName)` — the named NPC for this particular mission instance.
- `~mission(Location|Address)` — resolved from the spawn location.
- `~mission(ship)` — the spawned enemy ship variant.

**Implication for rendering.** Any annotation that describes the contract's rewards / ship pool / blueprint chance *in general* may not match what an individual in-game mission actually spawns. Flag text derived from such contracts as non-guaranteed at display time. Titles on TarPits / Gilly Mission07 / Adaigo salvage families all carry this marker.

### Rewards

```rust
pub enum RewardAmount {
    None,
    Calculated,    // engine computes at runtime (most UEC rewards)
    Fixed(i32),
}

pub struct ScripReward  { pub currency_guid: Guid, pub display_name: String, pub record_name: String, pub amount: i32 }
pub struct RepReward    { pub faction: Option<Guid>, pub scope: Option<Guid>, pub amount: Option<i32> }
pub struct ItemReward   { pub entity_class: Guid, pub display_name: String, pub amount: i32 }
pub struct BlueprintReward { pub chance: f32, pub pool_guid: Guid, pub pool_name: String, pub items: Vec<BlueprintItem> }
pub enum OtherReward    { BadgeAward, ScenarioProgress, JournalEntry, CompletionTags, CompletionBounty, ItemsWeighting, Reward, RefundBuyIn(f32), Unknown { struct_index: u32, instance_index: u32 } }
```

Important:

- **UEC is always `Calculated`** on SC 4.7 LIVE data — no contract stores a fixed aUEC amount directly. The engine formula isn't exposed in the DCB; consumers that want to show "≈ X aUEC" can either omit the number or plug in their own heuristic.
- **`RepReward.amount: None`** means `ContractResult_CalculatedReputation` (engine-computed). `Some(n)` means `ContractResult_LegacyReputation` with a resolved amount. Both routes contribute to reputation; consumers rendering rep deltas should handle both.
- **`BlueprintReward.items[i].display_name`** prefers the *crafted entity's* display name (`"Arclight Pistol"`) over `CraftingBlueprint.blueprintName` (which CIG frequently leaves as `<= PLACEHOLDER =>`). Empty `display_name` means neither path resolved — rare.
- **`BlueprintReward.chance`** is 0.0–1.0; multiply by 100 for a percentage.
- **Scrip vs items.** The split is typed — `reward_scrip` contains entities whose `SItemDefinition.type` is `EItemType::Currency`. `reward_items` is everything else from `ContractResult_Item` (ship unlocks, collectibles, …). Banu favours currently land in `reward_items` because CIG tagged them as Cargo, not Currency — that's a data observation, not a bug in sc-contracts.

### Availability and cooldowns

```rust
pub struct Availability {
    pub once_only: bool,
    pub can_reaccept_after_abandoning: bool,
    pub can_reaccept_after_failing:    bool,
    pub has_personal_cooldown:         bool,
    pub hide_in_mobi_glas:             bool,
    pub available_in_prison:           bool,
    pub notify_on_available:           bool,
    pub max_players_per_instance:      i32,
    pub cooldowns: Cooldowns,
}

pub struct Cooldowns {
    pub completion: Option<DurationRange>,  // after mission completion
    pub abandon:    Option<DurationRange>,  // after voluntary abandon
    pub fail:       Option<DurationRange>,  // after fail — gated by can_reaccept_after_failing
}

pub struct DurationRange { pub mean_seconds: f32, pub variation_seconds: f32 }
```

`cooldowns.fail` is synthesised: the DCB has no separate fail-cooldown field, so sc-contracts surfaces the abandon cooldown when `can_reaccept_after_failing == true` and `None` otherwise. This gives consumers a clean "can I reaccept after failing, and if so after how long" signal.

### Prerequisites

```rust
pub enum PrereqView {
    Locality { locality: Option<Guid> },
    Location { location: Option<Guid> },
    LocationProperty { variable_name: String, extended_text_token: String, level: ELocationTypeLevel },
    CrimeStat { include_when_sharing: bool, jurisdiction_override: Option<Guid>, min: f32, max: f32 },
    Reputation { include_when_sharing: bool, faction: Option<Guid>, scope: Option<Guid>, exclude: bool, min_standing: Option<Guid>, max_standing: Option<Guid> },
    CompletedContractTags { include_when_sharing: bool, required_tags: Vec<Guid>, required_count: i32, excluded_tags: Vec<Guid>, excluded_count: i32 },
    Unknown { struct_index: u32, instance_index: u32 },  // escape hatch
}
```

`Contract.prerequisites` holds the **common** prereqs shared by every variation. Per-variation extras live on `Variation.extra_prerequisites`. For the Gilly Mission08 pattern (one contract with 4 sub-contracts each pinning a different Stanton planet), the Locality prereqs land on variations, not on the parent `Contract.prerequisites`.

### Encounters

```rust
pub struct EncounterGroup { pub variable_name: String, pub waves: Vec<EncounterWave> }
pub struct EncounterWave  { pub name: String, pub slots: Vec<EncounterSlot> }
pub struct EncounterSlot  { pub concurrent: i32, pub weight: f32, pub candidates: Vec<ShipCandidate>, pub context: SpawnContext }

pub struct ShipCandidate  { pub entity_guid: Guid, pub display_name: String, /* …size, manufacturer, etc. */ }
pub struct SpawnContext   { /* classified non-ship tags: ai_skill, factions, cargo, ai_traits, mission_tags, … */ }
```

One `EncounterGroup` per `MissionPropertyValue_ShipSpawnDescriptions` on the contract. Groups hold named waves (e.g., `Wave1`, `SalvageableShip`); waves hold slots (one ship-picked-at-a-time each); slots carry the resolved candidate list plus a classified spawn context.

**Empty `candidates`** means either (a) a broken tag selector (Gilly Mission01 Wave3's `Relient_Tana` typo — SCMDB exhibits the same bug) or (b) a pure-context query that needs runtime location context to resolve. Honest empty is preferable to a generic over-match.

### Variations

```rust
pub struct Variation {
    pub expansion_id: Guid,
    pub origin: ContractOrigin,                 // Contract / ContractLegacy / CareerContract / SubContract { parent }
    pub extra_prerequisites: Vec<PrereqView>,   // prereqs unique to this variation
}
```

The `origin` tells you whether this row came from the parent contract or one of its sub-contracts. `extra_prerequisites` captures the per-tier gate — usually the variation's specific `Locality` GUID (pickup region) or a rep gate (career tier).

### `mission_span` and `LocalityRegistry`

`Contract.mission_span` is a `Vec<Guid>` of `MissionLocality` record GUIDs — the regions / system areas where the contract is offered. Deduplicated union across all variations; first-seen wins.

Look up via `index.localities.get(guid) -> Option<&LocalityView>`:

```rust
pub struct LocalityView {
    pub guid: Guid,
    pub name: String,                     // stable record name (`RegionA`, `Pyro3`, `TarPits`)
    pub locations: Vec<LocationRef>,      // sorted by (system, body, record_name)
    pub systems: BTreeSet<SystemKey>,     // distinct systems covered
    pub region_label: String,             // player-facing summary
}

pub struct LocationRef {
    pub guid: Guid,
    pub record_name: String,              // e.g. "Pyro3_L1", "RR_P6_LEO_CLINIC"
    pub display_name: String,             // localized: "Sirus", "Hurston", empty for unnamed stations
    pub system: SystemKey,
    pub body: Option<String>,             // "Pyro3" / "Stanton1" — only set when there's an intermediate parent
    pub body_display_name: String,        // "Bloom", "Hurston" — localized version of `body`
}

pub enum SystemKey { Stanton, Pyro, Nyx, Other(String) }
```

**`region_label`** is a precomputed one-line summary suitable for display. Shapes:

- `"Pyro: Bloom, Rat's Nest, Starlight Service Station"` — single system, body list
- `"Pyro: Checkmate, Patch City, Bloom, +3 more"` — capped at 5 bodies
- `"Pyro (system-wide)"` — only system-level locations
- `"Stanton + Pyro"` — multiple systems (body detail suppressed)
- `""` — empty locality (shouldn't occur on clean data)

**Why this matters for sc-langpatch.** Same-title Pyro contracts can ship different blueprint pools depending on which region they're offered in (`_AB` vs `_CD`). `mission_span` + `region_label` let you render a precise callout:

```
Destroy Headhunter Stolen Data
  [BP]*  varies by region:
    RegionA (Pyro: Checkmate, Patch City)                 → BP_..._AB, 14 items
    RegionB (Pyro: Bloom, Rat's Nest, Starlight Station)  → BP_..._AB, 14 items
    RegionC (Pyro: Gaslight, Pyro V)                      → BP_..._CD, 12 items
    RegionD (Pyro: Terminus)                              → BP_..._CD, 12 items
```

### `title_siblings` and `find_bp_conflicts`

`title_siblings` is the primitive: the GUIDs of other `Contract`s that share this contract's `(title, description)` pair. Empty when the title is unique.

```rust
let shared: Vec<&Contract> = contract
    .title_siblings
    .iter()
    .filter_map(|id| index.get(*id))
    .collect();
```

`find_bp_conflicts(&index.contracts) -> Vec<BpConflictGroup>` is the purpose-built helper for the blueprint case — returns every title where siblings disagree on their blueprint pool, with `has_mixed_presence: bool` flagging the "some variants have BP, some don't" case (the critical case for `[BP]*` annotation).

```rust
pub struct BpConflictGroup {
    pub title: String,
    pub description: Option<String>,
    pub members: Vec<BpConflictMember>,
    pub distinct_pool_count: usize,
    pub has_mixed_presence: bool,
}

pub struct BpConflictMember {
    pub contract_id: Guid,
    pub debug_name: String,
    pub handler_debug_name: String,
    pub handler_kind: HandlerKind,
    pub blueprint_pool: Option<Guid>,    // None when this variant has no BP
    pub pool_name: Option<String>,
    pub item_count: usize,
}
```

Analogous `find_scrip_conflicts` / `find_rep_conflicts` helpers aren't implemented yet — if you need them, the pattern is a copy of `find_bp_conflicts` with the reward comparison swapped.

---

## Workflows

### Rendering a contract title safely

```rust
fn render_title(c: &Contract) -> String {
    let Some(title) = &c.title else { return format!("<{}>", c.debug_name) };
    if c.has_runtime_substitution {
        // title contains ~mission(...) markers; treat derived annotations as non-guaranteed
        format!("{title} *")
    } else {
        title.clone()
    }
}
```

### Annotating blueprint rewards with conflict awareness

```rust
use sc_contracts::{find_bp_conflicts, ContractIndex};

let index = ContractIndex::build(&dc, &locale);
let conflicts = find_bp_conflicts(&index.contracts);
let conflict_ids: std::collections::HashSet<_> =
    conflicts.iter().flat_map(|g| g.members.iter().map(|m| m.contract_id)).collect();

for c in &index.contracts {
    let prefix = if conflict_ids.contains(&c.id) { "[BP]*" } else if c.blueprint_reward.is_some() { "[BP]" } else { "" };
    // … render `{prefix} {title}` …
}
```

### Rendering the mission span

```rust
fn render_span(c: &Contract, index: &ContractIndex) -> String {
    if c.mission_span.is_empty() { return String::new() }
    c.mission_span
        .iter()
        .filter_map(|g| index.localities.get(g))
        .map(|v| if v.region_label.is_empty() { v.name.clone() } else { format!("{} ({})", v.name, v.region_label) })
        .collect::<Vec<_>>()
        .join("; ")
}
```

### Rendering ship encounters

```rust
for group in &contract.encounters {
    println!("  {} ({} waves):", group.variable_name, group.waves.len());
    for wave in &group.waves {
        for slot in &wave.slots {
            let names: Vec<&str> = slot.candidates.iter()
                .map(|c| c.display_name.as_str())
                .filter(|n| !n.is_empty())
                .collect();
            println!("    [{}×] {}", slot.concurrent, names.join(" | "));
            // `slot.context` holds AI skill, faction tags, cargo descriptors
            if let Some(skill) = slot.context.ai_skill {
                println!("        AI skill: {skill:?}");
            }
        }
    }
}
```

### Formatting cooldowns

```rust
fn format_cd(cd: &sc_contracts::DurationRange) -> String {
    let mean = cd.mean_seconds;
    if mean < 60.0 {
        format!("{mean:.0}s")
    } else if mean < 3600.0 {
        format!("{:.1}m", mean / 60.0)
    } else {
        format!("{:.1}h", mean / 3600.0)
    }
}
```

### Matching SCMDB entries

SCMDB uses the parent contract's internal name as its identifier. For every `Contract` in the index:

- `contract.debug_name` is the parent's DCB name — usually matches the SCMDB row's identifier.
- `contract.id` is the parent's GUID — stable across regenerations if the DCB doesn't reshuffle.

sc-contracts runs 9.7% over SCMDB's 1,497 target (1,642 Contracts on SC 4.7 LIVE). The extras are mostly:

- Contracts with scalar-differing rewards (50 aUEC vs 55 aUEC) that currently don't merge — the `reward_signature` is value-exact.
- Some text-less contracts whose merge falls back to `(handler_debug_name, debug_name)` synthesis.

If you need tighter alignment, `merge_stats` gives you `variation_count_histogram` + `sibling_count_histogram` for diagnostics. Tuning is a future work item.

---

## Escape hatches

When the model doesn't cover what you need, reach through the `Datacore` the `ContractIndex` was built from:

- **Raw DCB inspection.** `datacore.db()` → `svarog_datacore::DataCoreDatabase`. Use `db.record(&guid)` for root records, `db.instance(struct_index, instance_index)` for anything pointed at by a poly `Unknown { struct_index, instance_index }` variant.
- **Typed generated types.** `datacore.records().pools` → `sc_extract::generated::DataPools` with `Handle<T>::get(&pools)` on every record type. Read-only.
- **Locale.** Pass your own `LocaleMap` into `ContractIndex::build`, or call `locale.get(key)` directly for any `LocaleKey` the crate surfaces (`ScripReward.display_name` is pre-resolved; raw LocaleKeys appear only on types sc-contracts chose not to resolve for you — rare).

Never depend on `svarog-*` crates directly — go through `sc_extract::svarog_datacore`, `sc_extract::svarog_common`, etc.

---

## Gotchas

- **`Contract.mission_span` is a `Vec<Guid>`, not a `Vec<LocalityView>`.** Materialising inline would duplicate the resolved location data across every contract. Always look up through `index.localities.get(guid)`.
- **`contract.prerequisites` only holds the common ones.** Per-variation prereqs (including the variation-specific `Locality`) live on `Variation.extra_prerequisites`. Merging them back into a single view is a consumer concern because the right merge depends on the rendering intent.
- **`RewardAmount::Calculated` is the default for UEC.** Don't treat it as "no reward"; it's "engine computes at runtime".
- **Empty `encounters[…].candidates`** is *load-bearing information* — it signals either a broken selector (a real in-game bug) or a pure-context query. Don't fall back to a generic "any ship" pool; that's the design error the crate was built to avoid.
- **`handler_kind == PVPBounty` or `ServiceBeacon` currently emits zero expansions.** These handlers are skipped in v1. If you need them, file it — they're scoped but not implemented.
- **Cooldown times are seconds, not milliseconds.** `DurationRange.mean_seconds` and `variation_seconds` are both `f32` seconds matching the DCB's `ContractAvailability` fields directly.
- **`has_runtime_substitution == true` on the contract doesn't mean every variation does.** The flag is derived from the resolved title/desc, which are taken from the first variation. In the current merge-group rule the text is identical across variations, so this matters only for hypothetical future edge cases.

---

## Performance

On SC 4.7 LIVE, build times (release build, warm disk, no snapshot):

| Stage | Time |
|---|---|
| `AssetSource::from_install` + `AssetData::extract` | ~3–5s (cold, includes p4k + locale) |
| `Datacore::parse` | ~2–3s |
| `ContractIndex::build` (all four stages) | ~2–3s |
| **Total from zero** | ~8–11s |

`ContractIndex::build` is dominated by:

- `ShipRegistry::build` — ~1s (two passes over all entity class definitions)
- `expand_all` — ~1s (walks 4,926 contract graph nodes)
- Everything else — <100ms each

`ContractIndex` itself is `Clone + Debug`. A full clone on SC 4.7 LIVE is ~100ms and allocates ~30MB — avoid cloning in hot paths.

Snapshot support lands through `sc-extract::ExtractSnapshot` — save a `Datacore` once per game patch, load it thereafter. This skips the ~5s p4k extraction and brings cold-start to ~3–4s.

---

## Version / compatibility

`sc-contracts` targets the current live SC version at the time of the crate's commit. The generator (`sc-generator`) regenerates type bindings when a new patch lands; after that, `ContractIndex::build` should keep working as long as:

- New `ContractGeneratorHandler_*` subtypes fall through to `HandlerKind::Unknown` (no expansions, non-fatal).
- New `ContractPrerequisiteBase*` / `ContractResultBase*` / `BaseMissionPropertyValue*` subclasses route through their enum's `Unknown { struct_index, instance_index }` variant (non-fatal, escape-hatch consumers can walk raw).
- Field renames / removals trigger a regeneration; the generator's debug-build `seed_database` assertion will panic with the list of unknown types.

If `ContractIndex::build` panics on a new patch, first regenerate with `cargo run -p sc-generator -- --p4k <path>` and retry.

---

## Where to file issues

- **Crate bugs** (wrong counts, incorrect resolution, panics): github issue on `sc-holotable`.
- **Missing features** that fit sc-contracts' scope (see `docs/sc-contracts.md` §Scope): same.
- **DCB-shape questions**: the design spec [docs/sc-contracts.md](sc-contracts.md), then the datacore format reference [docs/datacore.md](datacore.md) / `D:\Obsidian\Star Citizen\Game Files\Datacore.md`.
- **Status / what's currently landing**: [status.md](../status.md) is always the current truth.
