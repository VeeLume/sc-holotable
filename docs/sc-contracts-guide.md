# `sc-contracts` — consumer guide (v2)

> **Who this is for.** Rust code that wants to render / analyse Star Citizen contracts — primarily `sc-langpatch`, but anything that consumes the crate. For the design rationale, see [sc-contracts-v2.md](sc-contracts-v2.md); the v1 historical spec is [sc-contracts.md](sc-contracts.md). This file is the "how do I use it" guide.

## TL;DR

```rust
use sc_contracts::MissionIndex;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

let install = sc_installs::discover_primary()?;
let assets  = AssetSource::from_install(&install)?;
let data    = AssetData::extract(&assets, &AssetConfig::standard())?;
let dc      = Datacore::parse(&assets, &data, &DatacoreConfig::standard())?;

let index = MissionIndex::build(&dc, &data.locale);

for mission in &index.contracts {
    if let Some(title) = &mission.title {
        println!("{}: {} encounter(s)", title, mission.encounters.len());
    }
}
```

The `MissionIndex` is a plain bundle: every `Mission` plus the registries (ships, blueprints, currency, locations, localities, tag tree) the missions point at. Build it once per `Datacore`; carry it freely; look up by GUID or iterate.

## What's in a `Mission`

```rust
pub struct Mission {
    pub id: Guid,                          // unique per row (no implicit merge in v2)
    pub debug_name: String,                // for diagnostics; never user-facing
    pub origin: MissionOrigin,             // handler kind / source / generator / subcontract_of

    pub title: Option<String>,             // resolved (~mission(...) markers preserved)
    pub title_key: Option<LocaleKey>,      // raw INI key — use this when patching global.ini
    pub description: Option<String>,
    pub description_key: Option<LocaleKey>,
    pub has_runtime_substitution: bool,    // any ~mission(...) marker in title or description

    pub shareable: bool,
    pub illegal_flag: bool,
    pub availability: Availability,        // once_only, max_players, hide_in_mobi_glas, cooldowns

    pub rewards: MissionRewards,           // uec / scrip / reputation / items / blueprint / other
    pub prerequisites: Vec<PrereqView>,    // gating conditions (locality / location / crime stat / …)
    pub mission_span: Vec<Guid>,           // MissionLocality refs; resolve via index.localities

    pub encounters: Vec<Encounter>,        // Ships / NPCs / Entities / Unknown
}
```

## Pools — how to group missions

v2 doesn't merge similar missions into one. Instead it precomputes groupings and lets you pick the axis. Read the field directly:

```rust
// Group by INI key (sc-langpatch's natural patch axis)
for (key, ids) in &index.pools.description_key {
    let block = render_description_block(&index, ids);
    write_ini_entry(key, &block);
}

// Title-key pools — the closest analogue to v1's title_siblings
for (title_key, ids) in &index.pools.title_key {
    if ids.len() > 1 {
        // multiple missions share this title
    }
}
```

`MissionPools` is a plain struct on `MissionIndex.pools`. Fields:

| Field | Type | Use |
|---|---|---|
| `pools.title_key` | `HashMap<LocaleKey, Vec<Guid>>` | siblings sharing the same INI title |
| `pools.description_key` | `HashMap<LocaleKey, Vec<Guid>>` | siblings sharing the same INI description |

For each pool group, query divergence helpers on `MissionIndex` to know what differs across members:

```rust
for (title_key, ids) in &index.pools.title_key {
    if index.blueprint_mixed(ids) {
        // some members have a blueprint reward, others don't → render [BP*]
    }
    if !index.rewards_uec_consistent(ids) {
        // members disagree on UEC reward → omit the line or annotate
    }
    if index.has_runtime_substitution(ids) {
        // any ~mission(...) marker → engine fills at spawn time
    }
}
```

The full helper set, all taking `&[Guid]`:

| Title-relevant (binary tag safety) | Description-relevant (info-block line safety) |
|---|---|
| `blueprint_mixed` | `rewards_uec_consistent` |
| `blueprint_pool_consistent` | `rewards_scrip_consistent` |
| `once_only_mixed` | `rewards_rep_consistent` |
| `shareable_mixed` | `cooldowns_consistent` |
| `illegal_mixed` | `mission_span_consistent` |
| `handler_kind_mixed` | `encounters_shape_consistent` |
| `has_runtime_substitution` (cross-cutting) |

Each is a one-pass O(n) walk over the pool's members. They're cheap; call only what you read.

## Encounters

`Mission.encounters: Vec<Encounter>` covers all three spawn-shaped poly variants on `MissionProperty.value`:

```rust
match enc {
    Encounter::Ships(s)    => render_ship_encounter(s),
    Encounter::Npcs(s)     => render_npc_encounter(s),
    Encounter::Entities(s) => render_entity_encounter(s),
    Encounter::Unknown { variable_name, raw_guid } => {
        // unmodeled poly variant — walk via datacore.db().record(raw_guid)
    }
}
```

For consumers that don't care about the spawn shape:

```rust
println!("  {}: {}", enc.variable_name(), enc.extended_text_token());
```

### Ship encounters

```rust
pub struct ShipEncounter {
    pub variable_name: String,      // mission_variable_name on the underlying MissionProperty
    pub extended_text_token: String,// rare — sometimes "AmbushTarget" or similar
    pub phases: Vec<EncounterPhase<ShipSlot>>,
}

pub struct EncounterPhase<S> {
    pub name: String,               // SpawnDescription_*Group.Name (Wave1 / SalvageableShip / …)
    pub slots: Vec<S>,
}

pub struct ShipSlot {
    pub concurrent: i32,
    pub weight: f32,
    pub candidates: Vec<ShipCandidate>,         // resolved via ShipRegistry
    pub initial_damage_settings: Option<Guid>,  // pre-damaged spawn marker (salvage / cargo recovery)
    pub include_location_ai_spawn_tags: bool,

    pub positive: TagBag,           // identity + state tags from SpawnDescription_Ship.tags
    pub negative: TagBag,           // exclusion tags
    pub markup: TagBag,             // UI markup hints
    pub entity: TagBag,             // entity-filter hints
}
```

### NPC encounters

```rust
pub struct NpcSlot {
    pub priority: i32,
    pub weight: f32,
    pub include_location_ai_spawn_tags: bool,
    pub mission_allied_marker: bool,    // typed Allied/Hostile signal — the §1 crimestat input
    pub is_critical: bool,
    pub faction_override: Option<Guid>,
    pub identifier_tags: TagBag,
}
```

### Entity encounters

```rust
pub struct EntitySlot {
    pub amount: i32,
    pub weight: f32,
    pub positive: TagBag,
    pub negative: TagBag,
    pub markup: TagBag,
    pub entity: TagBag,
}
```

## TagBag — symmetric tag access

Each `TagList` slot on a spawn description becomes a `TagBag`:

```rust
pub struct TagBag {
    pub guids: Vec<Guid>,           // sorted, escape hatch
    pub names: Vec<String>,         // parallel to guids
}
```

Classification methods walk the tag tree on demand. Pass `&index.tag_tree`:

```rust
let tree = &index.tag_tree;
let factions: Vec<&str>      = slot.positive.factions(tree).collect();
let cargo: Vec<&str>         = slot.positive.cargo(tree).collect();
let spawn_ids: Vec<&str>     = slot.positive.spawn_identifiers(tree).collect();
let ai_traits: Vec<&str>     = slot.positive.ai_traits(tree).collect();
let mission_tags: Vec<&str>  = slot.positive.mission_tags(tree).collect();
let directives: Vec<&str>    = slot.positive.directives().collect();
let ai_skill: Option<u32>    = slot.positive.ai_skill();
let ace_pilot: bool          = slot.positive.ace_pilot();
```

Heuristic predicates derived from the SC 4.7 audit, name-stable so no tree lookup needed:

```rust
slot.positive.is_cargo_recovery()      // EnableInteractions + EmptyCrew
slot.positive.is_pre_damaged_wreck()   // DisablePowerInteractions
slot.positive.is_salvage_target()      // AvailableToSalvage
```

## Rewards

```rust
pub struct MissionRewards {
    pub uec: RewardAmount,                 // None / Calculated / Fixed(i32)
    pub scrip: Vec<ScripReward>,
    pub reputation: Vec<RepReward>,
    pub items: Vec<ItemReward>,
    pub blueprint: Option<BlueprintReward>,
    pub other: Vec<OtherReward>,
}
```

Most missions use `RewardAmount::Calculated` — the engine computes the aUEC at runtime; the static DCB doesn't carry the number. `Fixed(n)` is for the rare contract that hardcodes an amount.

## Resolving GUIDs

Several mission fields hold GUIDs. Resolve via the registries:

```rust
// MissionLocality GUIDs in mission.mission_span
for g in &mission.mission_span {
    if let Some(view) = index.localities.get(g) {
        println!("  {} :: {}", view.name, view.region_label);
        for loc in &view.locations {
            println!("    - {} ({:?})", loc.record_name, loc.system);
        }
    }
}

// Blueprint pool GUID
if let Some(bp) = &mission.rewards.blueprint {
    if let Some(pool) = index.blueprints.get(&bp.pool_guid) {
        for item in &pool.items {
            println!("  bp item: {}", item.display_name);
        }
    }
}

// Currency GUID on a scrip reward
for scrip in &mission.rewards.scrip {
    if let Some(currency) = index.currency.get(&scrip.currency_guid) {
        println!("  scrip: {} × {}", scrip.amount, currency.display_name);
    }
}

// Lookup mission by id
if let Some(other) = index.get(some_guid) {
    // …
}

// Iterate a pool's members
for member in index.iter_pool(ids) {
    // …
}
```

## Consumer workflows

### Render a contract title

```rust
let display = mission
    .title
    .as_deref()
    .unwrap_or(&mission.debug_name);

if mission.has_runtime_substitution {
    // The title contains ~mission(...) markers the engine fills at spawn.
    // Use the runtime title visible in-game rather than this static one
    // when accuracy matters.
}

println!("{display}");
```

### Patch INI titles by description key

```rust
for (description_key, ids) in &index.pools.description_key {
    let head = index.get(ids[0]).unwrap();
    let mut block = String::new();

    if !index.rewards_uec_consistent(ids) {
        block.push_str("[varies] ");
    } else if let RewardAmount::Calculated = head.rewards.uec {
        // engine-computed; no number to display
    } else if let RewardAmount::Fixed(n) = head.rewards.uec {
        block.push_str(&format!("{n} aUEC\n"));
    }

    if index.blueprint_mixed(ids) {
        block.push_str("[BP*]\n");        // mixed BP presence
    } else if head.rewards.blueprint.is_some() {
        block.push_str("[BP]\n");
    }

    write_ini(description_key.as_str(), &block);
}
```

### Render a ship encounter

```rust
fn render(enc: &Encounter, tree: &TagTree) {
    let Encounter::Ships(s) = enc else { return };
    println!("  ▸ {}", s.variable_name);
    for phase in &s.phases {
        println!("      phase \"{}\" — {} slot(s)", phase.name, phase.slots.len());
        for slot in &phase.slots {
            let factions: Vec<&str> = slot.positive.factions(tree).collect();
            let ships: Vec<&str> = slot
                .candidates
                .iter()
                .map(|c| c.display_name.as_str())
                .collect();
            println!("        {}× factions={:?} ships={:?}",
                     slot.concurrent, factions, ships);
        }
    }
}
```

## Escape hatches

- **Raw DCB queries** — `datacore.db()` (svarog `DataCoreDatabase`) and `datacore.records().pools` (flat-pool generated types). Use when the typed model doesn't cover a case; if you find yourself reaching here often, file a feature request.
- **Tag tree** — `index.tag_tree` for ad-hoc subtree walks beyond the `TagBag` classifier set.

## Performance notes

- `MissionIndex::build` takes ~2-3 s in release on SC 4.7 LIVE (dominated by ship registry construction).
- `MissionPools::build` is O(n_missions × axes), cheap.
- Divergence helpers walk pool members once per call; don't memoise unless you're calling them in tight loops.
- The 4,590 missions on SC 4.7 produce 11,599 ship-encounter slots + 3,749 NPC slots + 228 entity slots; expect numbers in this range.

## Version compatibility

Crate version is tied to the SC version that generated `sc-extract-generated`. Each game patch may shift tag GUIDs or reorganise records; rebuild after a regen and run `cargo test -p sc-contracts --features tui` to catch obvious shape changes. The investigation examples (`spawn_dig`, `encounter_analytics`, `salvage_pool`) are reproducible audits — re-run them after a patch to spot drift.

## See also

- [sc-contracts-v2.md](sc-contracts-v2.md) — design spec for v2 (the implementation this guide covers).
- [sc-contracts.md](sc-contracts.md) — v1 historical spec; superseded.
- [feature-request-sc-contracts-langpatch.md](feature-request-sc-contracts-langpatch.md) — open feature requests filed by sc-langpatch.
