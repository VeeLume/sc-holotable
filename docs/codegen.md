# Code generation — design specification

> Status: **proposal, awaiting review.** Depends on `docs/sc-extract.md`.

## Purpose

`tools/sc-generator` is the offline tool that mirrors the Star Citizen DataCore schema into Rust. It reads a `Data.p4k`, walks the DCB via svarog, and emits the complete set of Rust types plus their `FromInstance` parsing implementations into `sc-extract/src/generated/`. The generated output is committed to the repository — developers do not need a local `Data.p4k` to build the workspace.

This document specifies what the generator produces, how it handles tricky DCB features (inheritance, polymorphism, keyword collisions), and when it should be run.

## Where it lives

```
sc-holotable/
├── tools/
│   └── sc-generator/              workspace member, binary crate
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs            CLI entry
│           ├── walker.rs          DCB schema discovery
│           ├── emitter/           code-emitting modules
│           │   ├── struct.rs
│           │   ├── enum.rs
│           │   ├── from_instance.rs
│           │   ├── record_store.rs
│           │   └── metadata.rs
│           └── ...
└── crates/
    └── sc-extract/
        └── src/
            └── generated/         ← committed output, written by the tool
                ├── mod.rs
                ├── structs_a.rs   alphabetical buckets to keep any
                ├── structs_b.rs   one file under ~5000 lines
                ├── ...
                ├── enums.rs
                ├── from_instance.rs
                ├── record_store.rs
                └── metadata.rs
```

The generator is a workspace member so it's built with `cargo build --workspace`, but it is never invoked from `build.rs`. **No automatic codegen at build time.** Running the generator is an explicit developer action.

## Inputs and outputs

**Inputs:**
- `--p4k <path>` — path to `Data.p4k` (required)
- `--out-root <path>` — where to write generated files; defaults to `crates/sc-extract/src/generated/`
- `--check` — don't write files; exit non-zero if output would differ from committed files (for CI)

**Outputs:**

Everything in `sc-extract/src/generated/` is overwritten:

1. **Struct definitions** — one Rust `struct` per DataCore struct, mirroring fields and types.
2. **Enum definitions** — one Rust `enum` per DataCore enum, plus polymorphic variant enums for abstract bases.
3. **`FromInstance` implementations** — generated parsing code that reads from svarog `Instance` and produces owned values.
4. **Serde derives** — `Serialize`, `Deserialize`, `Debug`, `Clone`, `Default` where possible, with `#[serde(default)]` on every field.
5. **`RecordStore`** — a struct with per-type `HashMap<Guid, T>` fields, plus accessor methods.
6. **`RecordView` enum** — one variant per top-level record type, for type-erased access.
7. **`metadata.rs`** — constants describing the DCB this output was built from.

**No hand-written code in the generated directory.** If a file needs hand-written additions, it belongs elsewhere in `sc-extract`.

## The `metadata.rs` file

```rust
// sc-extract/src/generated/metadata.rs (GENERATED — do not edit)

/// SC game build this output was produced from.
pub const GENERATED_GAME_VERSION: &str = "4.6.173.39432";
pub const GENERATED_GAME_BRANCH:  &str = "sc-alpha-4.6.1";
pub const GENERATED_BUILD_ID:     &str = "12345";

/// SHA-256 of the `Data.p4k` the generator consumed.
pub const GENERATED_P4K_SHA256: &str = "7f3c2a9e4b1d5f8a...";

/// UTC ISO-8601 timestamp when the generator ran.
pub const GENERATED_AT: &str = "2026-04-10T14:32:17Z";

/// Version of the generator binary that produced this output.
/// Bumped independently of the game version when generator logic changes.
pub const GENERATOR_VERSION: &str = "0.1.0";

/// Schema version for the data envelope. Bumped when generator output
/// changes in a way that makes old snapshots unreadable.
pub const SCHEMA_VERSION: u32 = 1;

/// Total count of DCB structs/enums emitted. Sanity check / diff signal.
pub const STRUCT_COUNT: usize = 1847;
pub const ENUM_COUNT: usize = 213;
```

These constants are available to `sc-extract` at compile time. `parse_from_p4k` stamps the active constants into every `ExtractedData` it produces so snapshots carry their provenance.

## Generation rules

### Inheritance is flattened

DataCore structs can inherit from parent structs — a child's full property list is its parent's properties followed by its own. svarog's `c_header.rs` preserves this by embedding a `_parent` composition field in the child C struct.

**We flatten.** The generated Rust struct lists every field (inherited first, then own) as peers:

```rust
// DCB:
//   struct EntityComponentParams (abstract base, defines: enabled, name)
//   struct SCItemWeaponComponentParams : EntityComponentParams (adds: connection_params, fire_actions, ...)

// Generated Rust:
pub struct SCItemWeaponComponentParams {
    // Inherited from EntityComponentParams (written first):
    pub enabled: bool,
    pub name: String,
    // Own fields:
    pub connection_params: SWeaponConnectionParams,
    pub fire_actions: Vec<SWeaponActionParamsVariant>,
    pub weapon_regen_consumer_params: Option<SWeaponRegenConsumerParams>,
    // ... rest
}
```

**Rationale:**

- No `_parent: ParentType` field means no pointless nesting for consumers to walk.
- The abstract bases in practice are almost always empty metadata containers. Their only purpose is polymorphic dispatch (see next section), not field sharing.
- Traceability isn't lost — the generator emits a doc comment on each generated struct listing its parent chain, so you can still see "this field came from the abstract base".

```rust
/// DataCore type: SCItemWeaponComponentParams
/// Inherits: EntityComponentParams → IComponentParams
pub struct SCItemWeaponComponentParams { /* ... */ }
```

### Polymorphism via tagged enums

When the DCB has an abstract base type with multiple concrete descendants (common pattern: `SWeaponActionParams` with `SWeaponActionFireRapidParams`, `SWeaponActionFireSingleParams`, `SWeaponActionSequenceParams` as concrete subclasses), the generator emits a tagged enum that can hold any variant:

```rust
pub enum SWeaponActionParamsVariant {
    SWeaponActionFireRapidParams(SWeaponActionFireRapidParams),
    SWeaponActionFireSingleParams(SWeaponActionFireSingleParams),
    SWeaponActionSequenceParams(SWeaponActionSequenceParams),
    /// Unknown concrete type — kept as raw bytes for forward compatibility.
    Unknown { struct_index: u32 },
}
```

The `Unknown` variant makes parsing forward-compatible: if a new game patch adds a new concrete type before the generator has been rerun, the parser stores the `struct_index` and continues instead of failing.

Fields of the abstract base type become fields of the enum variants — NOT a common prefix on the enum itself — because flattening (above) already put them into each concrete struct.

### Arrays

```rust
// DCB:
//   fire_actions: array<SWeaponActionParams>  (abstract base element type)

// Generated:
pub fire_actions: Vec<SWeaponActionParamsVariant>,

// DCB:
//   spawn_points: array<Vec3>  (concrete element type)

// Generated:
pub spawn_points: Vec<Vec3>,
```

`Vec<T>` for homogeneous arrays; `Vec<FooVariant>` for polymorphic arrays.

### Reference semantics — flat-pool model

The generator emits a **flat pool** model rather than nested `Option<Box<T>>` trees. Every generated struct type has its own `Vec<Option<T>>` inside `DataPools`, and references between structs become typed [`Handle<T>`][sc_extract_generated::Handle] values — `u32` slot indices tagged with `T` in `PhantomData`. Top-level records additionally get a per-type `HashMap<CigGuid, Handle<T>>` in `RecordIndex` so consumers can still look up by GUID.

| DCB | Generated Rust | Notes |
|---|---|---|
| `Class` (inline struct field) | `Option<Handle<Foo>>` | handle into `DataPools`'s pool for `Foo`; materialised by `Builder::alloc_nested` |
| `Class` (array element) | `Vec<Handle<Foo>>` | each element routed through `alloc_nested`; inline classes are distinct per occurrence |
| `StrongPointer<Foo>` | `Option<Handle<Foo>>` | pool-backed instances dedup on `(struct_index, instance_index)` |
| `WeakPointer<Foo>` | `Option<Handle<Foo>>` | same treatment as StrongPointer — serializable |
| `Reference<Foo>` (by GUID) | `Option<CigGuid>` | stays as a GUID; cross-record, resolved via `records.foo.get(&guid)` |

**Why flat pools?** The earlier generation emitted `Option<Box<T>>` and built the record tree via recursive `FromInstance::from_instance` calls. DCB composition nests 50+ levels deep in places, and that walked a deeper call stack than the default 1 MB Windows main-thread allowed, forcing a 128 MB worker-thread stack workaround. The flat-pool model replaces the recursive walk with an iterative drain of a heap-allocated worklist inside `Builder`, so stack depth stays O(1) regardless of DCB nesting. It also deduplicates pool-backed instances that are pointed at from multiple places by their native `(struct_index, instance_index)` identity.

**Why a single generic `Handle<T>` instead of per-type newtypes?** An intermediate draft emitted one `{Name}Id(pub u32)` newtype per type plus matching `impl Index<TId>` and inherent `get` blocks — ~20k extra top-level items in the generated crate. At that scale rustc's cgu partitioner + serde-derive's `__Visitor` helpers tipped into cross-cgu symbol-visibility failures (~40k LNK2019 errors on Windows). Moving handles to a single `Handle<T>(u32, PhantomData<fn() -> T>)` with blanket `Index<Handle<T>>` and `Handle<T>::get` — gated on a one-line-each generated `impl Pooled for T` — keeps type safety while dropping generated item count by half.

**Reachability pruning.** The generator only emits DCB struct types that are **transitively reachable from records**: seed set is every distinct `struct_index` in `db.records()`, BFS frontier through every Class / StrongPointer / WeakPointer property target, emit the closure. On a real `Data.p4k`, that drops emission from **6,545 → ~1,935 types** (~70% reduction) with no functional change, because the pruned types were declared in the DCB schema but never referenced by any record or record-reachable struct. A generation-time self-check walks every emitted field and panics if any target isn't in the reachable set — if it ever fires it means the BFS has a hole and the pruning is unsafe.

**Accessing nested values.** Consumers follow a handle by calling `Handle::get` or using the blanket `Index<Handle<T>>` impl:

```rust
// Generic accessor — flattens `Option<T>` in the pool staging slot:
if let Some(component) = weapon.components[0].get(&pools) {
    // component: &SomeComponent
}
// Or raw via the Index blanket impl:
let opt: &Option<SomeComponent> = &pools[weapon.components[0]];
```

**Patch resilience.** `Builder::seed_database` resolves each known record type's `struct_index` against the live DCB by *name* (`Extract::TYPE_NAME` — the original DCB type name) and builds a per-run dispatch table. A patch that adds or reorders struct types does not silently drop records: existing record types stay recognised even if their runtime index shifted, and only genuinely new record types are missed (those need a regeneration to be picked up). In **debug builds** the dispatcher additionally tracks unknown record struct_indices and panics at the end of seeding with the full list — a direct signal that the generator is stale relative to the runtime DCB. In release builds that check is dead-code-eliminated; unknown records are silently skipped and the app keeps working with whatever subset it does understand.

### Rust keyword collisions

DataCore field names like `type`, `ref`, `match`, `move`, `static`, `trait` collide with Rust keywords. The generator uses raw identifiers:

```rust
// DCB field: "type"
// Generated:
pub r#type: String,
```

This matches svarog's `escape_c_keyword` function in `c_header.rs:517`, adapted to Rust's keyword list.

### Field name translation

DataCore uses camelCase (`fireRate`, `heatPerShot`, `ammoContainerParams`). The generator translates to snake_case (`fire_rate`, `heat_per_shot`, `ammo_container_params`) to follow Rust conventions.

**The translation is documented on each generated struct** so consumers can trace back to the original DCB name:

```rust
/// DataCore type: SCItemWeaponComponentParams
pub struct SCItemWeaponComponentParams {
    /// DataCore field: `fireRate`
    pub fire_rate: f32,
    /// DataCore field: `heatPerShot`
    pub heat_per_shot: f32,
}
```

### Serde derives

Every generated struct derives `Serialize`, `Deserialize`, `Debug`, `Clone`, and `Default` (where possible — `Default` is skipped for types with non-`Default` field types).

Every field — including required ones — gets `#[serde(default)]`. This is the schema-evolution escape valve: adding a new field to a generated struct doesn't break deserialization of older snapshots. The field just gets its default value.

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SCItemWeaponComponentParams {
    #[serde(default)]
    pub fire_rate: f32,
    #[serde(default)]
    pub heat_per_shot: f32,
    #[serde(default)]
    pub simplified_heat_params: Option<SWeaponSimplifiedHeatParams>,
    // ...
}
```

### `FromInstance` implementations

Every generated struct gets a matching `impl FromInstance`. The generator emits one line of parsing code per field, dispatched on the field's DCB `DataType`:

```rust
impl FromInstance for SCItemWeaponComponentParams {
    fn from_instance(inst: &Instance<'_>) -> Option<Self> {
        Some(Self {
            fire_rate: inst.get_f32("fireRate").unwrap_or_default(),
            heat_per_shot: inst.get_f32("heatPerShot").unwrap_or_default(),
            connection_params: inst.get_instance("connectionParams")
                .and_then(|i| SWeaponConnectionParams::from_instance(&i))
                .unwrap_or_default(),
            fire_actions: inst.get_array("fireActions")
                .map(|arr| {
                    arr.filter_map(|v| SWeaponActionParamsVariant::from_value(&v, inst.db()))
                        .collect()
                })
                .unwrap_or_default(),
            // ... one line per field
        })
    }
}
```

**Failure semantics:** missing fields produce default values via `unwrap_or_default()`. `from_instance` returns `None` only when the instance is fundamentally not the expected struct type (e.g., wrong struct_index). This keeps parsing robust against partial data.

Polymorphic enums implement a slightly different pattern, `from_value`, which dispatches on the concrete type:

```rust
impl SWeaponActionParamsVariant {
    pub fn from_value(value: &Value<'_>, db: &DataCoreDatabase) -> Option<Self> {
        match value {
            Value::Class { struct_index, data } => {
                let inst = Instance::from_inline_data(db, *struct_index, data);
                match db.struct_name(*struct_index as usize)? {
                    "SWeaponActionFireRapidParams" =>
                        SWeaponActionFireRapidParams::from_instance(&inst)
                            .map(Self::SWeaponActionFireRapidParams),
                    "SWeaponActionFireSingleParams" =>
                        SWeaponActionFireSingleParams::from_instance(&inst)
                            .map(Self::SWeaponActionFireSingleParams),
                    "SWeaponActionSequenceParams" =>
                        SWeaponActionSequenceParams::from_instance(&inst)
                            .map(Self::SWeaponActionSequenceParams),
                    _ => Some(Self::Unknown { struct_index: *struct_index }),
                }
            }
            Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                let inst = db.instance(r.struct_index, r.instance_index);
                // ... same match
            }
            _ => None,
        }
    }
}
```

### `RecordStore` generation

The generator discovers which types are top-level records (appear in `record_definition_count`) and emits a `RecordStore` with one `HashMap<Guid, T>` per such type:

```rust
// Generated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordStore {
    pub entity_class_definitions: HashMap<Guid, EntityClassDefinition>,
    pub tags: HashMap<Guid, Tag>,
    pub manufacturers: HashMap<Guid, SCItemManufacturer>,
    pub jurisdictions: HashMap<Guid, Jurisdiction>,
    pub blueprint_pool_records: HashMap<Guid, BlueprintPoolRecord>,
    pub contract_generators: HashMap<Guid, ContractGenerator>,
    // ... one field per top-level record type
}

impl RecordStore {
    pub fn len(&self) -> usize {
        self.entity_class_definitions.len()
            + self.tags.len()
            + self.manufacturers.len()
            // ... sum of all stores
    }

    pub fn get(&self, guid: &Guid) -> Option<RecordView<'_>> {
        if let Some(r) = self.entity_class_definitions.get(guid) {
            return Some(RecordView::EntityClassDefinition(r));
        }
        if let Some(r) = self.tags.get(guid) {
            return Some(RecordView::Tag(r));
        }
        // ... one block per type
        None
    }

    pub fn records_of_type(&self, type_name: &str) -> impl Iterator<Item = RecordView<'_>> + '_ {
        // Match on type_name, return iterator over the appropriate store
        // ... generated dispatch
    }

    // Typed accessors:
    pub fn entity_class(&self, guid: &Guid) -> Option<&EntityClassDefinition> {
        self.entity_class_definitions.get(guid)
    }
    pub fn tag_record(&self, guid: &Guid) -> Option<&Tag> {
        self.tags.get(guid)
    }
    // ... one pair per top-level type
}

#[derive(Debug)]
pub enum RecordView<'a> {
    EntityClassDefinition(&'a EntityClassDefinition),
    Tag(&'a Tag),
    SCItemManufacturer(&'a SCItemManufacturer),
    // ... one variant per top-level type
}

impl RecordView<'_> {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::EntityClassDefinition(_) => "EntityClassDefinition",
            Self::Tag(_) => "Tag",
            // ...
        }
    }
}
```

`RecordStore::get` is O(number of top-level types) at worst since it tries each per-type map in turn. We could add a master `Guid → RecordKind` index to make this O(1), but given the per-type map count is ~50 and each lookup is a hash miss, the constant factor is tiny. Defer the optimization until it matters.

## File organization inside `generated/`

One file per ~5000 lines, split by alphabetical bucket. No semantic grouping (that would be a hidden dependency on hand-curated category rules, which we deliberately avoided at the config-file level):

```
generated/
├── mod.rs                    re-exports from sub-modules
├── structs_a.rs              types starting with A
├── structs_b.rs
├── ...
├── structs_z.rs
├── enums.rs                  all generated enums (small enough for one file)
├── from_instance.rs          all FromInstance impls, same alphabetical split if large
├── record_store.rs           RecordStore + RecordView
└── metadata.rs               version constants
```

The split is mechanical and generator-driven. Developers navigate by `cargo doc` or by IDE type search, not by hand-walking the file tree.

## Running the generator

### Developer workflow on patch day

```bash
# New SC patch just dropped
cargo run -p sc-generator -- --p4k "C:\Games\StarCitizen\LIVE\Data.p4k"

# Review generated diff
git diff crates/sc-extract/src/generated/

# If anything broke at the call sites, fix hand-written domain crates
cargo build --workspace

# Run tests
cargo test --workspace

# Commit generated output + any hand-written fixes together
git commit -am "chore(generator): update for SC 4.7.0"
```

### CI check

```bash
cargo run -p sc-generator -- --p4k "$SC_P4K" --check
```

`--check` exits non-zero if the committed output would differ from fresh generation. Used as a post-patch CI step to detect drift between committed files and what the generator would produce. Requires a `Data.p4k` in the CI environment, which may or may not be practical — worst case we just skip the check and rely on manual runs.

### When to bump `SCHEMA_VERSION`

- A generated struct field is **removed** or **renamed** (serde default handles additions, not removals)
- A field's type **changes incompatibly** (e.g., `i32` → `String`)
- `RecordStore` gains a new top-level type store (incompatible with older snapshots that lack the field)
- `ReferenceGraph` internal structure changes

### When to bump `GENERATOR_VERSION`

- Generator logic changes that affect output even when the DCB schema hasn't — e.g., changed keyword-escape rules, changed field-name translation, changed inheritance-flattening behavior.

These are independent. A game patch alone doesn't bump either version — only the constants in `metadata.rs` change.

## Design decisions

### No config.toml, no target mapping

Earlier drafts had a `targets.sc-weapons = [...]` config. Dropped because maintaining it would be manual toil that breaks on every game patch. The generator emits everything into `sc-extract/src/generated/` and consumers import what they need. No partitioning decision, no hand-curation of "which types go where".

### No build-time codegen

Codegen never runs from `build.rs`. The generator is a manual tool. Reasons:

1. Developers without a `Data.p4k` should still be able to build the workspace — they can if output is committed.
2. `build.rs` that depends on a ~500 MB game file breaks CI, breaks reproducibility, and slows every build.
3. Generator runs are an explicit action tied to game patches, not to source edits.

### Committed generated output

The `generated/` directory is in git. Every developer sees the same types. Code review catches schema drift. Snapshot fixtures in tests can be committed and regenerated alongside the types.

Yes, this means committing tens of thousands of lines of machine-generated code. That's fine — machine-generated code is legible, diff-friendly, and in its own directory. IDE navigation works. `cargo doc` works. Nothing about "committed generated code" is actually a problem in practice.

### Inheritance flattened, not preserved as composition

See rationale above under "Inheritance is flattened". Abstract bases are metadata; flattening removes nesting for consumers without losing information (doc comments preserve parent chain).

### `Unknown` variant for polymorphic enums

Forward compatibility. If a new patch adds a concrete subclass before the generator has been rerun, parsing falls back to `Unknown { struct_index }` instead of failing. Consumers that see `Unknown` know they need to regenerate.

### Everything gets `#[serde(default)]`

Schema evolution. Old snapshots stay loadable after generated types gain new fields. The runtime cost is zero; the flexibility is significant.

### Raw identifiers for keyword collisions

`r#type` is legal Rust and does exactly what we want. The alternative — renaming to `type_` — breaks the doc comment that says "DataCore field: type" and makes the mapping asymmetric. Raw identifiers preserve the 1:1 correspondence.

## Open questions

1. **Does the generator ever need to *read* hand-written code in `sc-extract` to avoid name collisions?** E.g., if `sc-extract` hand-writes a type `Guid` (re-exported from svarog), can the generator safely emit a type also called `Guid`? Probably yes as long as they live in different modules (`sc_extract::Guid` vs `sc_extract::generated::Guid` would collide in `use` statements). Worth enforcing a naming rule: generated types go in `sc_extract::generated::*` and must not shadow identifiers in `sc_extract::*`.

2. **Should the generator emit tests alongside the types?** Auto-generated round-trip tests (parse → serialize → parse → compare) would catch codegen regressions early. Low cost, high value. My lean: yes, in a separate `generated/tests.rs` file, excluded from committed diff noise via `#[cfg(test)]`.

3. **How does the generator handle the `text_length2` string table in DCB v6+?** Bulkhead's `docs/datacore.md` mentions it. I assume svarog already handles this and the generator just reads `.name()` / `.type_name()` through svarog's API without caring. Worth verifying when implementing.

4. **`--check` mode in CI — is a Data.p4k available?** If not, the `--check` feature is only useful locally. Not a blocker; worth knowing.

## Out of scope for this document

- The shape of hand-written wrappers (`ShipWeapon`, `FpsWeapon`, `Ammo`) in domain crates. Those are specified in `docs/sc-ammo.md` and `docs/sc-weapons.md`.
- The `ReferenceGraph`, `TagTree`, `ManufacturerRegistry`, and `LocaleMap` types — those are hand-written in `sc-extract` and specified in `docs/sc-extract.md`.
- Non-Windows game data (there is none).
