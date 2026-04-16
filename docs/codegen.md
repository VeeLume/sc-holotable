# Code generation — design specification

> Status: **implemented.** The generated output lives in `crates/sc-extract-generated/src/generated/` (a workspace-internal crate). Consumers never depend on `sc-extract-generated` directly — they go through `sc-extract`, which re-exports the traits and the resulting `RecordStore` / `DataPools` / `Handle<T>`. See `docs/feature-gating-v2.md` for the feature classification design.

## Purpose

`tools/sc-generator` is the offline tool that mirrors the Star Citizen DataCore schema into Rust. It reads a `Data.p4k`, walks the DCB via svarog, and emits the complete set of Rust types plus their `Extract` implementations into `sc-extract-generated/src/generated/`. The generated output is committed to the repository — developers do not need a local `Data.p4k` to build the workspace.

## Where it lives

```
sc-holotable/
├── tools/
│   └── sc-generator/              workspace member, binary crate
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs            CLI entry
│           ├── pipeline.rs        P4K open, DCB parse, orchestrate emit
│           ├── naming.rs          Identifier sanitization
│           ├── emit.rs            Struct/enum/poly-enum/metadata emission
│           ├── features.rs        Feature classification + path-based splitting
│           └── closure.rs         Data-driven type closure computation
└── crates/
    └── sc-extract-generated/
        └── src/
            └── generated/         ← committed output, written by the tool
                ├── core/          empty polymorphic bases (unconditional)
                ├── multi_feature/ types shared across 2+ features (per-type cfg)
                ├── dormant/       schema types never observed in data (opt-in)
                ├── <feature>/     245 leaf feature directories
                ├── enums.rs       all generated enums
                ├── poly_enums.rs  polymorphic pointer enums
                ├── data_pools.rs  DataPools struct
                ├── metadata.rs    version constants
                └── mod.rs         module declarations + re-exports
```

The generator is a workspace member so it's built with `cargo build --workspace`, but it is never invoked from `build.rs`. **No automatic codegen at build time.** Running the generator is an explicit developer action.

## Inputs and outputs

**Inputs:**
- `--p4k <path>` — path to `Data.p4k` (required)
- `--out-dir <path>` — where to write generated files; defaults to `crates/sc-extract-generated/src/generated/`
- `--check` — don't write files; parse-only sanity check
- `--dump-refs` — diagnostic; dump per-record `Reference` resolution stats and exit

**Outputs:**

Everything in `sc-extract-generated/src/generated/` is overwritten:

1. **Struct definitions** — one Rust `struct` per reachable DataCore struct, mirroring fields and types. No derives.
2. **Enum definitions** — one Rust `enum` per DataCore enum with `Debug + Clone + PartialEq + Eq + Hash` derives and a `from_dcb_str` associated function plus `Unrecognized(String)` fallback.
3. **Poly enum definitions** — one `{Base}Ptr` tagged enum per polymorphic base with variants for each observed subclass plus `Unknown { struct_index, instance_index }`.
4. **`Extract` + `Pooled` implementations** — generated parsing code that reads from svarog `Instance` and populates pool slots.
5. **`DataPools`** — per-type `Vec<Option<T>>` pool fields, feature-gated to match their types.
6. **`metadata.rs`** — constants describing the DCB this output was built from.
7. **Cargo.toml updates** — auto-detected `[features]` section written to both `sc-extract-generated/Cargo.toml` and `sc-extract/Cargo.toml`.

**No hand-written code in the generated directory.** If a file needs hand-written additions, it belongs elsewhere in `sc-extract-generated` or `sc-extract`.

## Generation rules

### Inheritance is flattened

DataCore structs inherit via `parent_type_index`. The generated Rust struct lists every field (inherited first, then own) as peers. The generator delegates to svarog's `get_struct_properties` which already walks the inheritance chain — no additional parent walk. Doc comments preserve the parent chain for traceability:

```rust
/// DCB type: SCItemWeaponComponentParams
/// Inherits: EntityComponentParams -> IComponentParams
pub struct SCItemWeaponComponentParams {
    // Inherited from IComponentParams:
    pub enabled: bool,
    // Own fields:
    pub connection_params: Option<Handle<SWeaponConnectionParams>>,
    pub fire_actions: Vec<Handle<SWeaponActionFireRapidParams>>,
    // ...
}
```

### Polymorphism via tagged enums

When a pointer field stores multiple concrete subclasses at runtime, the generator emits a tagged `{Base}Ptr` enum:

```rust
pub enum SWeaponActionParamsPtr {
    SWeaponActionFireRapidParams(Handle<SWeaponActionFireRapidParams>),
    SWeaponActionFireSingleParams(Handle<SWeaponActionFireSingleParams>),
    SWeaponActionSequenceParams(Handle<SWeaponActionSequenceParams>),
    Unknown { struct_index: u32, instance_index: u32 },
}
```

Polymorphism detection is **data-driven**: the generator walks actual DCB instances and records which concrete `struct_index` values appear at each pointer site. Schema-static closures over-fan-out catastrophically on bases like `DataForgeComponentParams` (899 subclasses). See `docs/feature-gating-v2.md` §Decision 1 for the rationale.

The `Unknown` variant carries both halves of the raw instance reference so consumers can fall back to `datacore.db().instance(struct_index, instance_index)`.

### Reference semantics — flat-pool model

Every generated struct type has its own `Vec<Option<T>>` inside `DataPools`, and references between structs become typed `Handle<T>` values. Top-level records additionally get a per-type `HashMap<CigGuid, Handle<T>>` in `RecordIndex`.

| DCB | Generated Rust | Notes |
|---|---|---|
| `Class` (inline struct field) | `Option<Handle<Foo>>` | handle into pool; materialised by `Builder::alloc_nested` |
| `Class` (array element) | `Vec<Handle<Foo>>` | each element routed through `alloc_nested` |
| `StrongPointer<Foo>` | `Option<Handle<Foo>>` | pool-backed; dedup on `(struct_index, instance_index)` |
| `WeakPointer<Foo>` | `Option<Handle<Foo>>` | same treatment as StrongPointer |
| `Reference<Foo>` (by GUID) | `Option<CigGuid>` | stays as GUID; cross-record, resolved via `records` |

**Why flat pools?** The earlier design used `Option<Box<T>>` and recursive `FromInstance` calls. DCB composition nests 50+ levels deep, overflowing the 1 MB Windows stack. The flat-pool model uses an iterative worklist inside `Builder`, keeping stack depth O(1).

**Why a single generic `Handle<T>`?** An intermediate draft emitted per-type `{Name}Id(u32)` newtypes (~20k extra items). At that scale rustc's cgu partitioner tipped into cross-cgu symbol-visibility failures. A single generic keeps type safety while halving generated item count.

### Typed value surfaces

- **`DataType::Locale` fields** emit as `LocaleKey` (a newtype over `String` in `sc-extract-generated`). Prevents accidental confusion with free-text strings.
- **`DataType::EnumChoice` fields** emit as the corresponding generated Rust enum. Every enum gets `from_dcb_str` and an `Unrecognized(String)` fallback for forward compat. (Named `Unrecognized`, not `Unknown`, because real DCB enums have values named `Unknown`.)

### Rust keyword collisions

Raw identifiers for most keywords (`type` → `r#type`). Keywords that can't be raw identifiers in type position (`self`, `Self`, `super`, `extern`, `crate`) get an underscore suffix (`Self_`).

### Derive strategy

**Generated structs derive nothing.** No `Debug`, `Clone`, `Serialize`, or `Deserialize`. Consumers access data through `handle.get(&pools)` for typed access and `Datacore::db()` for raw svarog escape-hatch. Removing all struct derives eliminated ~12,500 proc-macro expansions and reduced cold check time by ~85%.

**Generated enums derive `Debug + Clone + PartialEq + Eq + Hash`.** Enums are small and the extra traits are useful for filtering/lookup.

### Feature gating

Types are classified into four physical modules by the generator's feature classifier (see `docs/feature-gating-v2.md`):

| Module | Types | Gate | Purpose |
|---|---:|---|---|
| `core/` | 336 | unconditional | Empty polymorphic bases — zero compile cost |
| `multi_feature/` | 3,789 | per-type `#[cfg(any(...))]` | Types reachable from 2+ features |
| `dormant/` | 809 | `#[cfg(feature = "dormant")]` | Schema types never observed in real data |
| leaf dirs | ~1,300 | `#[cfg(feature = "…")]` | Types owned by a single leaf feature |

Feature names mirror DCB record paths (`entities-scitem-ships`, `ammoparams`). Parent features include all children. The `full` feature enables every leaf; `dormant` forwards to `full` and adds the unobserved schema types.

### Reachability pruning

`compute_reachable_struct_indices` does a transitive BFS from every record type; structs never referenced by any record are dropped. A generation-time self-check panics if any emitted field's target is outside the reachable set.

### Patch resilience

`Builder::seed_database` resolves record types by **name**, not index. A game patch that reorders struct types doesn't silently drop records. In debug builds, unknown record types trigger a panic with the full list of unrecognized names — a direct signal to regenerate. Release builds skip them silently for graceful degradation.

## Running the generator

### Developer workflow on patch day

```bash
cargo run -p sc-generator --release -- --p4k "C:\Games\StarCitizen\LIVE\Data.p4k"
git diff crates/sc-extract-generated/src/generated/
cargo build --workspace
cargo test --workspace
git commit -am "chore(generator): update for SC 4.x.x"
```

### When to bump `SCHEMA_VERSION`

- A generated struct field is removed or renamed
- A field's type changes incompatibly
- `DataPools` / `RecordIndex` internal structure changes

### When to bump `GENERATOR_VERSION`

- Generator logic changes that affect output even when the DCB schema hasn't (e.g., changed keyword-escape rules, changed inheritance flattening).

## Design decisions

### No build-time codegen

The generator is never invoked from `build.rs`. Developers without a `Data.p4k` can still build the workspace because output is committed.

### Committed generated output

The `generated/` directory is in git. Machine-generated code is legible, diff-friendly, and in its own directory.

### No config file for type selection

Earlier drafts had a `targets.sc-weapons = [...]` config. Dropped because maintaining it would break on every game patch. The generator auto-discovers features from DCB record paths.

### Inheritance flattened, not preserved as composition

Abstract bases in the DCB are almost always empty metadata containers. Flattening removes nesting for consumers without losing information (doc comments preserve parent chain).
