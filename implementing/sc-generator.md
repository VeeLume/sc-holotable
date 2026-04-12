# `sc-generator` — implementation notes

This document records the implementation of `tools/sc-generator`, the offline tool that walks a Star Citizen `Data.p4k`, reads the DataCore schema, and emits Rust type definitions into `crates/sc-extract-generated/src/generated/`.

> **Post-flat-pool refactor note.** The sections below describe the **original** generator design, which emitted `FromInstance` impls that recursively walked nested `Option<Box<T>>` fields. That design overflowed the Windows main-thread stack on deep DCB nesting and needed a 128 MB worker-thread stack workaround. The current generator emits a **flat-pool** model instead: every struct gets a per-type `TId(u32)` handle, nested Class / StrongPointer / WeakPointer fields become `Option<TId>` / `Vec<TId>` into `DataPools`, and materialisation is driven iteratively by `Builder::drain`. See `docs/codegen.md` §"Reference semantics — flat-pool model" and `crates/sc-extract-generated/src/{extract,builder}.rs` for the new design. The historical notes here are kept for context on what the compile-time bucketing, naming, and gotcha mitigations solve — all of those still apply. Anywhere this doc says `FromInstance`, substitute `Extract`; anywhere it says `Option<Box<T>>`, substitute `Option<TId>`.

## Status

**Complete and verified end-to-end.** Successfully extracts the full DCB schema from a real `Data.p4k` and produces Rust code that compiles cleanly as part of sc-extract.

Numbers from the reference run against Star Citizen 4.7:

- **6545 structs** emitted
- **760 enums** emitted
- **173 bucket files** (plus `enums.rs`, `metadata.rs`, `mod.rs`)
- **~270k total lines** of generated Rust
- **Generation time: ~1.6 seconds** (P4K open + DCB parse + all emission)
- **Cold cargo check of sc-extract: ~6 minutes**
- **Warm cargo check: 1 second** (incremental)
- **0 errors, 0 warnings** when compiled as part of sc-extract

## File layout

```
tools/sc-generator/
├── Cargo.toml
└── src/
    ├── main.rs       CLI entry, arg parsing
    ├── pipeline.rs   Open P4K, extract/parse DCB, orchestrate emit, write files
    ├── naming.rs     Identifier sanitization (snake_case, keyword escape)
    └── emit.rs       Codegen — struct defs, FromInstance impls, enums, metadata, mod.rs
```

## CLI

```
sc-generator --p4k <path> [--out-dir <path>] [--check]
```

- `--p4k`: path to `Data.p4k`. Required.
- `--out-dir`: where to write generated files. Defaults to `crates/sc-extract/src/generated`.
- `--check`: parse everything but don't write any files. For CI / sanity.

Typical developer invocation:

```bash
cargo run -p sc-generator --release -- --p4k "C:/Games/StarCitizen/LIVE/Data.p4k"
```

Release mode is strongly preferred — the DCB parse is tens of times faster than debug.

## Dependencies

```toml
[dependencies]
sc-installs        = { workspace = true }   # reads build_manifest.id for metadata
thiserror          = { workspace = true }
tracing            = { workspace = true }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
svarog-common      = { git = "https://github.com/19h/Svarog.git", features = ["serde"] }
svarog-datacore    = { git = "https://github.com/19h/Svarog.git" }
svarog-p4k         = { git = "https://github.com/19h/Svarog.git" }
```

## What it does

The pipeline:

1. Opens `Data.p4k` via `svarog-p4k`.
2. Finds `Game2.dcb` inside the archive by scanning entries (case-insensitive suffix match).
3. Reads and parses the DCB via `svarog-datacore::DataCoreDatabase::parse`. Output:
   - `struct_definitions()`: every struct type in the schema
   - `enum_definitions()`: every enum type
   - `struct_name()`, `property_name()`, `enum_name()` lookups
   - `get_struct_properties(idx)`: the list of fields for a struct
4. Reads `build_manifest.id` from next to the P4K (via `sc-installs::read_build_manifest`) to stamp version metadata into the generated output.
5. Deletes stale `types*.rs`, `enums.rs`, `metadata.rs`, `mod.rs` files from a previous run. Leaves any other files in the directory untouched.
6. Emits, then writes:
   - **`types_<bucket>.rs`** files (one per bucket) with struct definitions + `FromInstance` impls
   - **`enums.rs`** with a Rust enum per DCB enum plus an `Unrecognized(String)` fallback variant
   - **`metadata.rs`** with version / provenance constants
   - **`mod.rs`** that declares all sub-modules and re-exports every generated type flat

## Bucketing strategy

Early drafts put all struct definitions in a single `types.rs`. That file came out at **266,846 lines** and produced a cargo check time of **>10 minutes** — unworkable for iteration.

The current strategy splits the output into per-prefix bucket files:

1. **First pass**: group structs by a two-letter prefix. The prefix is derived from the first two ASCII-alphabetic characters of the sanitized Rust struct name (lowercased). So `SCItemWeaponComponentParams` → `"sc"`, `BulletImpulseFalloffParams` → `"bu"`, `X` → `"x"` (single-letter fallback), `_123` → `"other"` (no alphabetic chars).
2. **Second pass**: if a prefix has more than `MAX_STRUCTS_PER_BUCKET` (currently **300**) structs, split the list into numbered sub-buckets — `bu1`, `bu2`, `bu3`, etc. — each with up to 300 structs.

This produces ~173 files for the current DCB. The largest file (`types_bu2.rs`) is ~28,000 lines containing ~300 `BuildingBlocks_*` types.

### Why this specific strategy

- **rustc parallelizes type-check across modules.** Many smaller files mean more work in parallel.
- **serde proc-macro expansion is per-item**, and each bucket file handles its own macro invocations independently.
- **Alphabetical grouping minimizes cross-file references.** Types within the same `SCItem*` family tend to reference each other, keeping most dependencies intra-bucket.
- **Numbered sub-buckets are stable.** `bu1` always gets the first 300 `Bu*` structs in DCB order, `bu2` the next 300. Regeneration produces deterministic output.

### Cross-bucket references

Every bucket file has `use super::*;` at the top. `mod.rs` has `pub use types_<bucket>::*;` for every bucket, which re-exports all generated types flat into the `generated` namespace. Combined, that means any bucket file can reference any other bucket's types by bare name without explicit paths.

Rust's name resolution handles this without cycles: the `use super::*` imports are resolved lazily, and the generator guarantees no identifier collisions (duplicate struct names are deduplicated in the first pass).

## Type mapping

DCB `DataType` → Rust type:

| DataType | Rust (scalar) | Rust (array) |
|---|---|---|
| `Boolean` | `bool` | `Vec<bool>` |
| `SByte` / `Int16` / `Int32` | `i32` | `Vec<i32>` |
| `Int64` | `i64` | `Vec<i64>` |
| `Byte` / `UInt16` / `UInt32` | `u32` | `Vec<u32>` |
| `UInt64` | `u64` | `Vec<u64>` |
| `Single` | `f32` | `Vec<f32>` |
| `Double` | `f64` | `Vec<f64>` |
| `String` / `Locale` / `EnumChoice` | `String` | `Vec<String>` |
| `Guid` | `CigGuid` | `Vec<CigGuid>` |
| `Class` / `StrongPointer` / `WeakPointer` | `Option<Box<T>>` | `Vec<T>` |
| `Reference` | `Option<CigGuid>` | `Vec<CigGuid>` |

### Design choices for type mapping

- **Small integer types widen.** `i8`/`i16` both become `i32`, and `u8`/`u16` both become `u32`. This matches svarog's `get_i32` / `get_u32` accessors, which return widened values. Loses precise bit-width but is practical — the extracted values fit in the wider type without truncation.
- **Strings are owned (`String`), not borrowed.** Generated structs are self-contained and serializable; they can't hold borrowed references into the DCB bytes after extraction.
- **`Class` / `StrongPointer` / `WeakPointer` become `Option<Box<T>>`** (scalar form) or `Vec<T>` (array form). `Option` because these fields can be null; `Box` because a struct can transitively contain itself via these fields, which would produce an infinitely-sized type without indirection.
- **`Reference` becomes `Option<CigGuid>`, not `Option<RecordRef>`.** svarog's `RecordRef` is a transparent wrapper around `CigGuid` but doesn't derive `serde::{Serialize, Deserialize}`. Using the raw guid avoids a 1000+-error cascade and keeps the serialized form compact. Consumers resolve cross-record references through sc-extract's `RecordStore` by guid.

## FromInstance implementation

Every generated struct gets a `FromInstance` impl. The trait signature is:

```rust
pub trait FromInstance: Sized {
    fn from_instance(inst: &Instance<'_>, db: &DataCoreDatabase) -> Option<Self>;
}
```

The `db` parameter is required because **array elements that are `Value::Class` values** need `Instance::from_inline_data(db, struct_index, data)` to be materialized into nested instances. svarog's `Instance` type doesn't expose its internal database handle, so callers have to thread `db` through.

Per-field extraction expressions are computed by `from_instance_expr` in `emit.rs`:

- Primitives use `inst.get_bool("fieldName").unwrap_or_default()` style accessors.
- Strings use `inst.get_str("fieldName").map(String::from).unwrap_or_default()`.
- Class / StrongPointer / WeakPointer use `inst.get_instance("fieldName").and_then(|i| Type::from_instance(&i, db)).map(Box::new)`.
- Reference uses `inst.get("fieldName").and_then(|v| v.as_record_ref()).map(|r| r.guid)`.
- Arrays iterate via `inst.get_array("fieldName")` and map elements through matching `v.as_*()` helpers, filtering out `None` on unknowns.

For empty structs (no properties), the `inst` and `db` parameters are renamed to `_inst` and `_db` to avoid unused-variable warnings. For non-empty structs with no reference-resolving fields, an explicit `let _ = db;` silences the warning.

## Inheritance handling

DCB structs can inherit from parent structs (`parent_type_index` field). The generator **flattens** inheritance: a child struct's generated definition contains its parent's fields inlined as peers, followed by the child's own fields. The `collect_full_properties` helper walks the inheritance chain depth-first, accumulating all ancestor properties first.

Flattened output is simpler to consume (no `.parent.foo` nesting) and more compatible with serde round-tripping. The doc comment on each generated struct lists the immediate parent so traceability is preserved:

```rust
/// DCB type: `SCItemWeaponComponentParams`
///
/// Inherits from: `IComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemWeaponComponentParams { /* ... */ }
```

## Identifier sanitization

DCB names can contain characters that aren't valid Rust identifiers (dots, dashes) or collide with Rust keywords. The `naming` module handles this:

- **`to_snake_case`**: converts camelCase / PascalCase to snake_case. Consecutive uppercase letters collapse as a unit (acronyms like `SC` stay together, not `s_c`), and digits attach to the preceding run. This is a deliberate simplification — the goal is valid unique identifiers, not perfect case conversion.
- **`sanitize_field_name`**: applies `to_snake_case`, ensures the result is non-empty, and escapes Rust keywords. Most keywords become raw identifiers (`type` → `r#type`). The keywords that can't be used as raw identifiers (`self`, `Self`, `super`, `extern`, `crate`) get an underscore suffix instead (`self_`).
- **`sanitize_struct_name`**: replaces non-alphanumeric characters with underscores, ensures the name starts with a letter or underscore, collapses consecutive underscores. Type-position keywords get the underscore-suffix treatment (no raw identifiers for types).
- **`sanitize_variant_name`**: same rules as struct names. Enum variants share the same keyword restrictions.

## Derive strategy

Generated types derive:

- **`Debug`** — needed for diagnostics
- **`Clone`** — consumers may clone record entries when building views
- **`Serialize` + `Deserialize`** — required for the msgpack snapshot format

Generated types explicitly **do not** derive:

- **`Default`** — was an early choice, dropped for compile-time reasons. `FromInstance::from_instance` builds structs via explicit field initialization, so `Self::default()` is never called. Dropping `Default` removed ~6,545 auto-generated impls, a measurable compile-time saving.
- **`PartialEq` / `Eq` / `Hash`** — not needed for the snapshot format, adds per-struct derive work.

Generated enums derive: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`. Enums are much smaller than structs and the extra traits are useful for filtering / lookup.

## Handling the `Unknown` variant collision

Every generated enum needs a catch-all variant for forward compatibility (new DCB enum values that the generator hasn't been re-run for yet). Originally I used `Unknown(String)`, but several real DCB enums have a value literally named `Unknown`, producing an `E0428` "defined multiple times" error with a cascade of downstream E0277/E0532/E0618 errors (49 errors total from this one collision).

Fix: rename the fallback to **`Unrecognized(String)`**, which no DCB enum currently uses. The generator reserves this name in the variant de-duplication set before emitting DCB values, so a real enum value called `Unrecognized` (unlikely) would be dropped rather than cause a collision.

## Metadata emission

`metadata.rs` contains version constants baked at generation time:

```rust
pub const GENERATED_GAME_VERSION: &str = "...";
pub const GENERATED_GAME_BRANCH: &str = "...";
pub const GENERATED_BUILD_ID: &str = "...";
pub const GENERATED_CHANGELIST: Option<&str> = Some("...");
pub const GENERATED_AT: &str = "unix:1712750102";
pub const GENERATOR_VERSION: &str = "0.0.0";
pub const SCHEMA_VERSION: u32 = 1;
pub const STRUCT_COUNT: usize = 6545;
pub const ENUM_COUNT: usize = 760;
```

Version / branch / build_id / changelist come from the `build_manifest.id` file next to `Data.p4k`, read via `sc-installs::read_build_manifest`. If the manifest can't be read (file missing, parse error), the values default to `"unknown"` and generation still succeeds.

`SCHEMA_VERSION` is a manual constant baked into the generator itself. Bump it when the generator output shape changes in a way that would break old snapshot loaders.

## Validation

| Check | Result |
|---|---|
| `cargo build -p sc-generator --release` | ✅ 3 sec |
| `cargo test -p sc-generator` | ✅ 3 naming tests pass |
| `cargo clippy -p sc-generator --all-targets -- -D warnings` | ✅ no warnings |
| Run against real `Data.p4k` | ✅ 6545 structs + 760 enums in 1.6 sec |
| `cargo check -p sc-extract` (cold, with generated code) | ✅ **0 errors, 0 warnings, ~6 min** |
| `cargo check -p sc-extract` (warm, incremental) | ✅ **1 sec** |

## Compile-time journey

The compile-time optimization was an iterative process driven by real measurements. Earlier attempts:

| Attempt | Layout | Result |
|---|---|---|
| 1: single file | One `types.rs` at 266k lines | >10 min cargo check, unusable |
| 2: first-letter buckets | 26 files, types_s.rs at 93k lines | Still slow, dominant bucket too big |
| 3: two-letter buckets | ~170 files, types_bu.rs at 42k lines | Better, still 1 outlier |
| 4: two-letter + count cap | 173 files, largest ~28k lines | **~6 min cold, 1 sec warm** ✅ |

Additional savings along the way:

- Dropped `Default` derive (~6,545 fewer impls)
- Added `#![allow(unused_imports)]` to each bucket file (eliminated 123 warnings without filtering imports per bucket)
- Clean up stale generated files on each run (avoids orphans from layout changes)

Further speedups are possible (see follow-ups below) but the current numbers are acceptable for the iteration loop.

## Known follow-ups

- **Cold build time (~6 min) is still the bottleneck.** Potential further optimizations:
  - Move `generated/` into its own crate (`sc-generated` or similar) with `sc-extract` depending on it. Separate crates can parallelize more aggressively and incremental builds would touch less ground when editing hand-written sc-extract code.
  - Consider "expanding" serde derives ahead of time (via `cargo expand`) and committing the expanded code, removing proc-macro work from every build.
  - Skip `FromInstance` generation for structs that aren't reachable from any top-level record. The DCB has dead code.
- **Pin svarog to a specific commit.** Still tracking main branch; same follow-up as Phase 2a.
- **Validate generated code is semantically correct.** So far it *compiles*, but we haven't actually run a `parse_from_p4k` against the generated types end-to-end. That's Phase 2d / Task 4.
- **Polymorphic enum generation.** The spec mentioned emitting `FooVariant { A, B, Unknown }` enums for abstract base types with multiple concrete subclasses. Not implemented in the current generator — every struct is emitted as a plain concrete type. If polymorphic dispatch becomes necessary (likely for weapons' fire_actions array), add it here.
- **Better camel → snake conversion.** The current heuristic keeps acronyms together (`SCItem` → `scitem`). A lookup-aware converter could produce more idiomatic Rust names (`SCItem` → `sc_item`). Not urgent — the current names are valid and unique.
- **Cleanup of empty buckets.** Buckets like `types_j.rs` contain zero structs but still get emitted as (almost) empty files. Could be filtered out but the savings are negligible.

## What's next

With `sc-generator` producing valid compilable code, Task 4 is the final phase:

- Unblock Phase 2d of sc-extract: build the `ExtractedData` envelope, `parse_from_p4k` orchestrator, snapshot save/load. These depend on the generated `RecordStore` type, which the generator doesn't emit yet — that's still a gap.
- **Generator gap: `RecordStore` and `RecordView` are not yet emitted.** The spec calls for the generator to produce a typed record store keyed by top-level DCB record types. For MVP we skipped this; consumers iterate `db.all_records()` directly. Phase 2d will need either (a) the generator to produce `RecordStore`, or (b) hand-written aggregation logic in sc-extract.
- End-to-end validation: actually run `parse_from_p4k` → `FromInstance` materialization → `save_snapshot` → `load_snapshot` round-trip. That's when we find out if the generated `from_instance` expressions work against real data.
