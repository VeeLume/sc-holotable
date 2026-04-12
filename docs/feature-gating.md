# Feature-gated generated code

> Design document for compile-time reduction via feature-gated type generation.
> Status: **proposed** — not yet implemented.

## Problem

The `sc-extract-generated` crate emits ~1,935 struct types across 270k lines of Rust. Every type gets `serde::Serialize + Deserialize` derives, `Extract` impls, and `Pooled` impls. This dominates cold compile times:

| Profile | Cold build time |
|---|---|
| `cargo check` (dev) | ~6 min |
| `dev-opt` | ~21 min |
| `release` | ~16 min |

Consumers typically need only ~10-20 record types (covering ~50-200 generated types after transitive dependency resolution). The other ~1,700 types are dead code that still costs compile time.

## Prior art: bulkhead

Bulkhead (`E:\vscode\rust\bulkhead`) already implements path-based selective generation. Its `explore` crate's codegen module:

1. Accepts an array of **DCB record path prefixes** (e.g., `libs/foundry/records/entities/scitem/ships/weapons`)
2. Iterates `db.all_records()` and matches `record.file_name()` with `.starts_with()`
3. Selected records become "seeds" — their struct types are collected
4. **Type closure discovery**: all types reachable through fields of seed types are included
5. Generates only the types in the closure

Configuration is via CLI `--path` flags with brace expansion support:
```
libs/foundry/records/entities/scitem/{ships,vehicles,weapons}
libs/foundry/records/entities/spaceships
```

## Proposed solution: path-prefix features

### Why path-based, not type-based

DCB records have hierarchical names: `"libs.foundry.records.entities.scitem.ships.weapons.AEGS_Avenger_Nose_S3"`. The path prefix tells you what domain the record belongs to.

Type-name filtering (e.g., "only extract EntityClassDefinition") doesn't work because `EntityClassDefinition` is the generic container for *everything* — weapons, ships, contracts, armor, quantum drives. You can't filter by type alone.

Path-based filtering naturally groups records by domain, matching how the game's content is organized.

### API for record paths (svarog)

```rust
// Record::name() → full hierarchical path from string table 2
record.name() // → Some("libs.foundry.records.entities.scitem.ships.weapons.AEGS_Avenger_Nose_S3")

// Record::file_name() → source file path from string table 1
record.file_name() // → Some("libs/foundry/records/entities/scitem/ships/weapons/...")
```

Both are available at runtime on every `svarog_datacore::Record`.

## Feature definition

Features are defined as **sets of path prefixes**. The generator resolves them to type closures.

```toml
# In a config file or hardcoded in the generator
[features.weapons]
paths = [
    "libs/foundry/records/entities/scitem/ships/weapons",
    "libs/foundry/records/entities/scitem/fps_weapons",
    "libs/foundry/records/ammoparams",
]

[features.contracts]
paths = [
    "libs/foundry/records/missions",
    "libs/foundry/records/contracts",
]

[features.vehicles]
paths = [
    "libs/foundry/records/entities/spaceships",
    "libs/foundry/records/entities/groundvehicles",
]
```

### Resolution process (at generation time)

1. **Match records to features**: iterate `db.all_records()`, match `record.file_name()` against each feature's path prefixes
2. **Collect seed struct types**: for each matched record, note its `struct_index` → struct type name
3. **BFS type closure per feature**: from seed types, walk all fields (Class, StrongPointer, WeakPointer) transitively to find every type the feature needs
4. **Core types**: types reachable from multiple features are assigned to `core` (always compiled)
5. **Emit cfg attributes**: types exclusive to one feature get `#[cfg(feature = "...")]`

### Consumer usage

```toml
# Cargo.toml — combat simulator
[dependencies]
sc-extract = { git = "...", features = ["weapons"] }
# Compiles: core types + weapon/ammo types (~200 types, <1 min)

# Cargo.toml — localization tool
[dependencies]
sc-extract = { git = "...", features = ["contracts"] }
# Compiles: core types + contract/mission types (~150 types, <1 min)

# Cargo.toml — full analysis tool
[dependencies]
sc-extract = { git = "...", features = ["full"] }
# Compiles: all 1,935 types (~6 min)
```

## Implementation structure

### Generated code layout

```
crates/sc-extract-generated/src/generated/
├── core/           # always compiled
│   ├── types_*.rs
│   ├── enums.rs
│   └── mod.rs
├── weapons/        # #[cfg(feature = "weapons")]
│   ├── types_*.rs
│   └── mod.rs
├── contracts/      # #[cfg(feature = "contracts")]
│   ├── types_*.rs
│   └── mod.rs
��── vehicles/       # #[cfg(feature = "vehicles")]
│   ├── types_*.rs
│   └── mod.rs
├── record_store.rs # dispatch + pools (cfg-gated per feature)
└── mod.rs
```

### DataPools splitting

`DataPools` splits into per-feature sub-structs:

```rust
#[derive(Default, Serialize, Deserialize)]
pub struct DataPools {
    pub core: CorePools,
    #[cfg(feature = "weapons")]
    #[serde(default)]
    pub weapons: WeaponPools,
    #[cfg(feature = "contracts")]
    #[serde(default)]
    pub contracts: ContractPools,
    #[cfg(feature = "vehicles")]
    #[serde(default)]
    pub vehicles: VehiclePools,
}
```

Each sub-struct is self-contained with its own serde derive. `#[serde(default)]` ensures snapshots saved with more features can still be loaded by binaries with fewer features (missing pools deserialize as empty).

### seed_database dispatch

The dispatch table in `seed_database` conditionally registers types:

```rust
// Always:
if let Some(&i) = name_to_idx.get(Tag::TYPE_NAME) {
    dispatch[i] = Some(extract_record_tag);
}

// Only with "weapons" feature:
#[cfg(feature = "weapons")]
if let Some(&i) = name_to_idx.get(AmmoParams::TYPE_NAME) {
    dispatch[i] = Some(extract_record_ammo_params);
}
```

## Expected impact

| Config | Types compiled | Est. cold check | Est. dev-opt build |
|---|---|---|---|
| `core` only | ~50-100 | <30s | ~3 min |
| `core` + `weapons` | ~200-300 | ~1 min | ~5 min |
| `full` | 1,935 | ~6 min | ~21 min |

## Complementary: runtime path filtering

Even with feature gating, a consumer might want to further filter at runtime — e.g., "only extract weapons from the LIVE build, skip PTU-only experimental weapons". This can be added as a runtime filter on `Builder`:

```rust
let filter = RecordFilter::new()
    .include_paths(&["libs/foundry/records/entities/scitem/ships/weapons"])
    .exclude_paths(&["libs/foundry/records/entities/scitem/ships/weapons/experimental"]);

let store = Builder::new(&db)
    .with_record_filter(&filter)
    .consume_database()
    .finish();
```

This is complementary to feature gating — it further reduces runtime extraction time within the compiled type set.

## Implementation plan

1. **Investigate record paths in real Data.p4k** — dump `record.file_name()` / `record.name()` for all records, group by prefix, validate that the expected weapons/contracts/vehicles groupings hold.
2. **Define feature seeds** — a config mapping feature names → path prefixes, validated against real data.
3. **Extend the generator** — compute per-feature type closures and emit cfg-gated code.
4. **Split DataPools/RecordIndex** into per-feature sub-structs.
5. **Add Cargo features** to sc-extract-generated and sc-extract.
6. **Verify** — `cargo check --features weapons` compiles fast, tests pass.

## Automatic feature generation

Instead of manually mapping path prefixes to feature names, the generator can **auto-derive features** from the path hierarchy:

### Algorithm

1. Iterate all records, collect `(file_name, struct_index)` pairs
2. Extract the path prefix at a configurable depth (e.g., depth 5 → `libs/foundry/records/entities/scitem`)
3. Each unique prefix becomes a **path group**
4. BFS from each group's struct types to compute type closures
5. For each type, record which groups need it
6. Emit `#[cfg(any(feature = "group_a", feature = "group_b"))]` on types needed by specific groups only

### Auto-naming

Path groups get Cargo feature names derived from their prefix:
- `libs/foundry/records/entities/scitem` → `entities-scitem`
- `libs/foundry/records/ammoparams` → `ammoparams`
- `libs/foundry/records/missions` → `missions`

A mapping file (`generated/feature_map.toml`) is emitted alongside the code, documenting which paths map to which features.

### Manual aliases (the ergonomic layer)

Auto-generated features are precise but unwieldy. Manual aliases group them:

```toml
# In Cargo.toml
[features]
weapons = ["entities-scitem-ships-weapons", "entities-scitem-fps-weapons", "ammoparams"]
contracts = ["missions", "contracts"]
vehicles = ["entities-spaceships", "entities-groundvehicles"]
full = ["weapons", "contracts", "vehicles", ...]  # enables everything
```

This gives both flexibility (fine-grained auto features) and ergonomics (coarse manual aliases). Consumers can use either level.

### When auto-generation helps

- **New game patches add paths** — auto-generation picks them up without manual config updates
- **Developer exploration** — `cargo build --features entities-scitem-ships-weapons` "just works" without knowing the feature map
- **Precise control** — a tool that only needs mining lasers can enable just `entities-scitem-mining` without pulling in all weapons

### When manual features are better

- **Higher-level domain crates** (sc-weapons, sc-contracts) want a single stable feature name that doesn't change when CIG renames internal paths
- **Consumer Cargo.toml readability** — `features = ["weapons"]` is clearer than `features = ["entities-scitem-ships-weapons", "entities-scitem-fps-weapons", "ammoparams"]`

**Both layers coexist.** Auto features are the foundation; manual aliases are the interface.

## Overlapping features (how shared types work)

When two features both need the same type (because records in both path prefixes reference it through field BFS), that type must compile whenever *either* feature is active.

### Mechanism: `cfg(any(...))`

```rust
// Type only needed by weapons:
#[cfg(feature = "weapons")]
pub struct SCItemWeaponComponentParams { ... }

// Type needed by both weapons AND vehicles:
#[cfg(any(feature = "weapons", feature = "vehicles"))]
pub struct SAttachableComponentParams { ... }

// Type needed by 3+ features → promote to core (no cfg):
pub struct Tag { ... }  // always compiled
```

### Promotion threshold

If a type is needed by N or more features, it's cheaper to always compile it (no cfg) than to emit a long `cfg(any(...))` list. The generator uses a threshold:

- **Needed by 1 feature**: `#[cfg(feature = "X")]`
- **Needed by 2-3 features**: `#[cfg(any(feature = "X", feature = "Y", ...))]`
- **Needed by 4+ features (or by all)**: no cfg → always compiled (promoted to core)

This keeps the cfg attributes manageable while minimizing core bloat.

### Why this is correct for Cargo

Cargo features are **additive** — enabling a feature can only add code, never remove it. Our cfg scheme is purely additive:
- Each feature adds its types to compilation
- No type is ever removed by enabling a feature
- No `cfg(not(feature = ...))` is used anywhere

This means any combination of features is valid. `weapons + contracts` compiles the union of both type sets. No conflicts are possible.

### DataPools/RecordIndex field overlap

Pool fields for shared types use the same `cfg(any(...))`:

```rust
pub struct DataPoolsSa {
    #[cfg(any(feature = "weapons", feature = "vehicles"))]
    pub s_attachable_component_params: Vec<Option<SAttachableComponentParams>>,

    #[cfg(feature = "weapons")]
    pub s_ammo_container_params: Vec<Option<SAmmoContainerParams>>,
}
```

Serde handles this with `#[serde(default, skip_serializing_if = "Vec::is_empty")]` — missing features deserialize as empty Vecs, and empty Vecs are omitted from serialization.

## Investigating real data

The generator now supports `--dump-paths` to see the actual path distribution:

```bash
cargo run -p sc-generator --release -- --p4k "C:/Games/StarCitizen/LIVE/Data.p4k" --dump-paths
```

This prints record counts grouped by path prefix at depth 4 and 5, showing how records cluster and what struct types live where. Run this to validate feature boundaries before implementing.

## Risks

- **Path instability across game patches**: CIG might reorganize paths. Mitigated by re-running the generator after each patch — auto-features adapt automatically.
- **Cross-feature type sharing**: handled by `cfg(any(...))` and the promotion threshold. Core might grow if many types are universally shared, but the BFS makes this predictable.
- **Snapshot portability**: handled by `#[serde(default)]` — loading a "full" snapshot with a "weapons-only" binary just gives empty pools for non-weapon types.
- **Compile-time of cfg checking**: rustc still parses cfg'd-out items for syntax (though it doesn't type-check or codegen them). Very large generated files might still be slow to parse. Mitigation: keep the per-feature module split so cfg'd-out files aren't even `mod`-included.
