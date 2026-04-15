# sc-holotable

Shared Rust utility workspace for Star Citizen tooling. Provides install discovery, DataCore extraction, reference-graph indexing, localization, and generated type bindings that multiple consumer apps share instead of re-implementing.

> **Read this first when starting any new session on this workspace.** The design specs in `docs/` and the implementation notes in `implementing/` have the full history; this file is the fast orientation.

## Consumer apps

Three downstream apps depend (or will depend) on this workspace as a git dep. They live outside the repo:

| App | What it uses | Location |
|---|---|---|
| `streamdeck-starcitizen` | `sc-installs` only (for install discovery) | `E:\vscode\streamdeck\streamdeck-starcitizen` |
| `sc-langpatch` | `sc-installs` + `sc-extract` (localization, contracts) | `E:\vscode\rust\sc-langpatch` |
| `bulkhead` | `sc-installs` + `sc-extract` + (future) `sc-weapons` | `E:\vscode\rust\bulkhead` |

**Design rule:** sc-holotable crates are *real utility libs*, not bent to any one consumer's current shape. Consumers adapt to the lib, not the other way round.

## Workspace layout

```
sc-holotable/
├── Cargo.toml                     workspace — see members list for active crates
├── README.md                      public-facing overview
├── CLAUDE.md                      this file
├── status.md                      current work state — read alongside this file
├── docs/                          design specs (stable reference)
│   ├── datacore.md                DCB binary format reference
│   ├── sc-installs.md             install discovery crate spec
│   ├── sc-extract.md              foundational extraction crate spec
│   ├── codegen.md                 sc-generator design
│   ├── feature-gating.md          feature-gating v1 (superseded)
│   ├── feature-gating-v2.md       feature-gating v2 — data-driven scoping + poly (implemented)
│   ├── benchmarks.md              canonical benchmark numbers — re-run via tools/bench/bench.ps1
│   ├── sc-ammo.md                 ammo crate spec (not implemented yet)
│   └── sc-weapons.md              weapons crate spec (not implemented yet)
├── implementing/                  phase-by-phase implementation notes
│   ├── sc-installs.md
│   ├── sc-extract-2a.md           sc-extract phase 2a (foundation)
│   ├── sc-extract-2c.md           sc-extract phase 2c (graph/tags/manufacturers/etc.)
│   └── sc-generator.md
├── crates/
│   ├── sc-installs/               ✅ implemented, 51 tests passing
│   ├── sc-extract/                ✅ phase 2a + 2c + 2d done; API reworked 2026-04-13 (staged entry points)
│   ├── sc-extract-generated/      ✅ workspace-internal — core/dormant/multi_feature + leaf feature dirs
│   │   └── src/generated/         245 leaf feature dirs + core/ + dormant/ + multi_feature/
│   ├── sc-contracts/              📦 stub only (lib.rs docs, no code)
│   └── sc-weapons/                📦 stub only (lib.rs docs, no code)
└── tools/
    ├── sc-generator/              ✅ implemented — offline DCB schema → Rust codegen
    └── bench/                     PowerShell benchmark script + README — writes to docs/benchmarks.md
```

Note 1: `sc-ammo` has a spec but **no crate directory** — it's not in the workspace members list. Scaffold when we start implementing it.

Note 2: the spec proposed a `crates/components/` subdirectory for future `sc-ammo`, `sc-shields`, etc. That grouping hasn't been applied to the filesystem yet — `sc-weapons` lives at `crates/sc-weapons`, not `crates/components/sc-weapons`. Defer until we actually add a sibling component crate.

### The sc-extract / sc-extract-generated split

`sc-extract-generated` is a **workspace-internal crate** that holds everything the generator emits plus the `Extract` / `Pooled` traits, the `Builder`, and the `LocaleKey` newtype. `sc-extract` depends on it and re-exports its public surface, so consumers see `sc_extract::RecordStore`, `sc_extract::Builder`, `sc_extract::LocaleKey`, etc.

Why split: the generated code is ~6,234 struct types + ~1,252 enum types with serde derives. Splitting into its own crate gives build cache stability (edits to hand-written code don't invalidate generated code) and allows per-crate profile overrides.

### Feature gating (v2 — data-driven scoping + polymorphism)

Post-polymorphism, the generated types live in three physical modules plus per-feature leaf directories. See `docs/feature-gating-v2.md` for the full design rationale.

| Module / bucket | Types | Compile gate | Purpose |
|---|---:|---|---|
| `core/` | 336 | unconditional, always compiled | Empty polymorphic bases — zero own fields, zero compile cost, promoted to decouple base visibility from referring-type visibility. |
| `multi_feature/` | 3,789 | per-type `#[cfg(any(feature = "A", feature = "B", …))]` | Types reachable from ≥2 features. Always compiled when any of the union's features is enabled. **`multi-feature` is a physical module name, NOT a Cargo feature** — don't add it to the feature list. |
| `dormant/` | 809 | module-level `#[cfg(feature = "dormant")]` | Schema-reachable types never observed in real DCB data (test scaffolding, unused scripting, deprecated engine features). Emitted but compiled only when `dormant` is opted in. `dormant = ["full"]` so enabling it pulls in every observed type too. |
| leaf feature dirs | ~1,300 | per-dir `#[cfg(feature = "…")]` | Types owned by a single leaf feature. 245 dirs, named after DCB record paths (`audio/`, `entities-scitem-ships/`, `ammoparams/`, …). |

```toml
# Narrow consumer — one leaf feature:
sc-extract = { features = ["entities-scitem-ships"] }

# Parent feature — all entity sub-features:
sc-extract = { features = ["entities"] }

# Every observed type:
sc-extract = { features = ["full"] }

# Every schema-reachable type (CI validation, forward-compat):
sc-extract = { features = ["dormant"] }
```

The generator auto-classifies by walking the actual DCB record instance graph (data-driven scoping), splitting types across the four buckets by cfg width. It writes the `[features]` section to both `sc-extract-generated/Cargo.toml` and `sc-extract/Cargo.toml`.

### Typed value surfaces in generated code

Two recent refinements make the generated code more type-safe than a 1:1 schema reflection:

- **`DataType::Locale` fields** emit as `LocaleKey` (a newtype over `String`). `LocaleMap::get` / `resolve` / `contains_key` accept any `AsRef<str>` so `&LocaleKey` passes natively. Consumers can't accidentally confuse a localization-reference string with a free-text string at compile time.
- **`DataType::EnumChoice` fields** emit as the corresponding generated Rust enum (resolved through `prop.struct_index`, dual-used as the enum index for choice fields). Every enum gets a `from_dcb_str` associated function. Unknown values (including variants added in a game patch the generator didn't see) fall through to `Unrecognized(String)` for forward compat.
- **Polymorphic pointer fields** emit a poly enum (`generated/poly_enums.rs`) with a variant per observed subclass plus an `Unknown { struct_index: u32, instance_index: u32 }` fallback. The `Unknown` payload is the raw-layer escape hatch — a consumer can walk the instance directly via `datacore.db().instance(struct_index, instance_index)`.

Profiles are currently **at cargo defaults** (no workspace customization). Previous workspace profile overrides were driven by serde-derive monomorphization costs that no longer exist after derive-drop (commit `06e0d2e`); the profile config + `.cargo/config.toml` were reset to a clean slate so any new overrides can be justified by fresh measurement. Use `cargo build`, `cargo build --release`, and `cargo check` with no special flags.

**Rule of thumb**: never depend on `sc-extract-generated` directly from another workspace crate — go through `sc-extract`. That's the stable public boundary.

## Design principles

These are load-bearing — deviating from them should be a deliberate decision, not an accident.

1. **Go slow.** Verify game-mechanics assumptions against real DCB data before encoding them as types. Incorrect assumptions baked into a shared lib propagate wrong behaviour to every consumer. A "raw layer" is the safe fallback — prefer "we don't model this yet" over a wrong model.
2. **Real utility lib, not app-shaped.** When the lib and a consumer disagree, change the consumer. Awkwardness at integration time is a signal about the consumer, not the lib.
3. **One canonical model per domain.** When two consumers need overlapping data, they share a single type — the most-demanding consumer drives correctness, others read a subset.
4. **Layering is on data source, not format.** `sc-extract` owns both DCB (`Game2.dcb`) and non-DCB (vehicle XML, `global.ini`) access. The split between hand-written and generated code is internal, not a crate boundary.

## Key conventions

### Naming

- **Crate prefix: `sc-`** — `sc-installs`, `sc-extract`, `sc-weapons`, etc.
- **Domain wrappers**: when a spec calls for a "raw layer + curated trait", the curated struct takes the plain name (`Weapon`, `Ammo`) and the raw layer is either `EntityClassDefinition` (generic DCB type) or generated directly. No `RawFoo`/`Foo` split — see the sc-weapons spec for the wrapper-over-generated-type pattern.
- **Error variants**: `thiserror` enums with `#[non_exhaustive]` so adding variants is non-breaking.

### Dependencies and features

- **`thiserror`** for library errors, `anyhow` is banned in lib code
- **`serde`** with `derive` feature, always-on (no feature flag)
- **`specta`** behind a `specta` feature flag when sc-langpatch needs typed Tauri bindings
- **`tracing`** (not `log`, not `println!`) for all logging. `debug!` for per-record details, `info!` for phase transitions, `warn!` for recoverable issues, `error!` for terminal failures

### Svarog integration

- **svarog is a git dep from `https://github.com/19h/Svarog.git`** (the canonical upstream; an earlier bulkhead-fixes branch was merged there)
- **`svarog-common` needs the `serde` feature enabled** — its derives are feature-gated. Forgetting this produces ~1000 `CigGuid: Deserialize` errors.
- **svarog is re-exported** from `sc-extract` as both full namespace (`sc_extract::svarog_datacore`) and cherry-picked types (`sc_extract::Guid`, `sc_extract::Value`, etc.). Consumers should never depend on svarog directly — go through `sc-extract`. `sc-extract-generated` takes svarog as a direct dep because the generated code does `use svarog_common::CigGuid;` etc. — that's fine because `sc-extract-generated` is workspace-internal, not public API.
- **Pinned to commit `ce06ec67`.** All svarog crates are workspace dependencies in the root `Cargo.toml` with `rev = "ce06ec67"`. Individual crate Cargo.toml files use `{ workspace = true }`. To bump svarog, update the rev in one place.
- **Local `[patch]` override for `DataCoreReference::is_null`.** The root `Cargo.toml` carries a `[patch."https://github.com/19h/Svarog.git"]` section pointing at `E:/repros/Svarog` with a one-line fix: treat `instance_index == -1` as a null sentinel. Without this, svarog parses thousands of explicit-null scalar `Reference` fields as `Some(garbage_guid)`. **The path is absolute and only resolves on Valerie's machine** — `cargo build` will fail on any other environment until either (a) the fix is upstreamed and the pinned rev is bumped, or (b) the path is made relative / the override is removed. Drop it as soon as the upstream fix lands.

### Feature gates and version pins that bite

- **`specta = "=2.0.0-rc.21"`** — not `"2"`. Cargo resolves `^2` to `rc.24` which requires nightly `debug_closure_helpers`. Matches sc-langpatch's pin.
- **Rust edition 2024**, `rust-version = "1.94.1"`. Let-chains (`if let Some(x) = foo && condition`) are fine — stable since 1.88.

## Running the generator

When a new SC patch lands, regenerate the DataCore bindings:

```bash
cargo run -p sc-generator --release -- --p4k "C:/Games/StarCitizen/LIVE/Data.p4k"
```

This writes the `core/`, `dormant/`, `multi_feature/`, and ~245 leaf feature directories into `crates/sc-extract-generated/src/generated/`, plus top-level `enums.rs` / `poly_enums.rs` / `data_pools.rs`, and updates both `sc-extract-generated/Cargo.toml` and `sc-extract/Cargo.toml` with the auto-detected `[features]` section. Review the diff, fix any call sites that broke, commit everything.

Release mode is strongly preferred over debug — DCB parse is tens of times faster. The whole run takes ~3 seconds.

Available flags:

- `--p4k <path>` (required)
- `--out-dir <path>` — defaults to `crates/sc-extract-generated/src/generated`
- `--check` — parse everything but don't write files
- `--dump-refs` — diagnostic; dump per-record `Reference` resolution stats and exit

Note: the diagnostic flags from the feature-gating-v2 design session (`--dump-paths`, `--dump-features`, `--check-polymorphism`, `--analyze-poly-bases`, `--measure-dormancy`, `--feature-closure`, `--measure-cfg-spread`) were removed after the design was committed. If you need to re-measure, reintroduce them on a temporary branch.

## Build / test / lint

```bash
# Fast iteration (warm cache ~1s)
cargo check -p sc-extract
cargo check -p sc-installs

# Tests for the libs
cargo test -p sc-installs             # 51 tests
cargo test -p sc-extract              # 91 tests (+ 3 doctests)
cargo test -p sc-generator            # 3 naming tests

# Clippy (strict)
cargo clippy -p sc-installs --all-targets -- -D warnings
cargo clippy -p sc-extract --all-targets -- -D warnings
cargo clippy -p sc-generator --all-targets -- -D warnings

# Smoke test — core only (fastest)
cargo run -p sc-extract --release --example parse_real_p4k

# Smoke test — with specific features
cargo run -p sc-extract --release --features ammoparams --example parse_real_p4k

# Smoke test — full (all types, slow compile)
cargo run -p sc-extract --release --features full --example parse_real_p4k

# Full release
cargo build -p sc-extract --release
```

**Warm incremental check is ~1 second.** See `docs/benchmarks.md` for current cold compile numbers per feature set. Use feature gating to keep iteration fast — only enable the features your consumer needs.

## Compile-time gotchas (this will bite you)

The generated code is ~6,234 struct types + ~1,252 enum types across the `core/`, `dormant/`, `multi_feature/` modules plus 245 leaf feature directories. Feature gating keeps compile times tractable:

1. **core is tiny (336 types).** Empty polymorphic bases only — promoting them unconditionally costs essentially nothing (zero own fields).
2. **multi_feature is big (3,789 types) but cfg-gated per type.** Every struct carries a `#[cfg(any(feature = "A", feature = "B", …))]` listing every feature whose closure touches it. When you compile with only one leaf feature enabled, rustc strips most of `multi_feature/` during parsing. It's always "visible" in the source tree but only "real" to rustc when at least one member of its cfg union is active.
3. **dormant (809 types) is whole-module gated.** Completely free unless `dormant` is explicitly enabled.
4. **Feature-gated directories.** Each leaf feature dir (e.g., `audio/`, `entities-scitem-ships/`) is behind `#[cfg(feature = "...")]`. Disabled features are never parsed by rustc.
5. **Generated structs derive nothing.** No `Debug`, no `Clone`, no serde. Post-derive-drop (commit `06e0d2e`) the workspace does not serialize, clone, or debug-print generated records — consumers go through `handle.get(&pools)` for read access and the raw svarog layer (`Datacore::db()`) for escape-hatch inspection. Fewer derives = fewer proc-macro invocations = faster compile.
6. **Generated enums derive `Debug + Clone + PartialEq + Eq + Hash`** and carry a `from_dcb_str` associated function. No serde, no manual `Default`.
7. **`#![allow(non_camel_case_types, dead_code)]`** on enum files because DCB enum values use arbitrary casing.
8. **`#![allow(unused_imports)]` on every feature type file.** Uniform imports keep the emitter simple.

If compile time regresses, the first diagnostic step is regenerating and diffing the three module sizes. The classifier in `tools/sc-generator/src/features.rs` controls all three boundaries — if `core/` grows well past 400 types, or `dormant/` shrinks toward 0, something has changed in either the DCB or the classifier and wants investigation.

## Specific gotchas encountered during implementation

These are real bugs that were hit and fixed — future edits should not re-introduce them:

- **`CigGuid` doesn't implement serde by default.** svarog-common feature-gates it behind `serde`. Enable the feature in every crate that uses it in a serializable context.
- **`RecordRef` doesn't implement serde at all.** Map DCB `Reference` fields to `Option<CigGuid>` and extract the inner guid, not `Option<RecordRef>`. See the generator's `rust_type_for` function for the mapping.
- **`Instance::database()` doesn't exist.** svarog's `Instance` hides its db handle. Any code that needs to materialize array-of-Class elements or resolve pool instances must reach through the [`Builder`], which owns a `&'a DataCoreDatabase` in its `db` field — see `crates/sc-extract-generated/src/builder.rs`.
- **Flat-pool materialisation is iterative, not recursive.** The generator no longer emits `FromInstance` impls that call themselves through nested `Option<Box<T>>` fields. Instead every emitted struct implements `Extract` + `Pooled`, nested Class / StrongPointer / WeakPointer fields become `Option<Handle<T>>` / `Vec<Handle<T>>` into per-type `Vec<Option<T>>` pools inside `DataPools`, and `Builder::drain` walks a heap-allocated worklist. There is **no** `EXTRACT_THREAD_STACK_SIZE` any more — the main-thread stack is sufficient regardless of DCB nesting depth.
- **`ReferenceGraph::from_database` is also iterative.** The initial recursive walker (`collect_refs_from_instance` / `collect_refs_from_value`) stack-overflowed on real data — same root cause as the Builder before the flat-pool refactor. Replaced with a `Vec<(Instance, Guid, String)>` worklist. Don't re-introduce recursion through Value::Class / ClassRef / StrongPointer / WeakPointer nesting in graph.rs.
- **Typed slot handles are generic, not per-type.** A single hand-written `Handle<T>(u32, PhantomData<fn() -> T>)` covers every emitted type, with blanket `impl<T: Pooled> Index<Handle<T>> for DataPools` / `impl<T: Pooled> Handle<T> { fn get(…) }`. Earlier drafts emitted per-type `TId(u32)` newtypes, Index impls, and inherent `get` blocks — ~20k generated items — which tipped rustc/link.exe into per-type symbol-visibility problems. Moving them to a single generic keeps the compile cost linear in the struct count and got serde-derive symbol resolution back into a working regime.
- **Reachability pruning in the generator.** `compute_reachable_struct_indices` does a transitive BFS from every record type through Class / StrongPointer / WeakPointer fields; struct definitions that no record transitively references are dropped at generation time. Post-polymorphism this drops 6,545 → 6,234 types (a thin prune — polymorphism support pulls in every observed subclass of every polymorphic base, which is where most of the type budget goes). A **self-check** runs at generation time and panics if any emitted field's target is outside the reachable set — that would be a BFS bug and should never happen.
- **Data-driven closure, not schema-static.** Per-feature closures are computed by walking *actual DCB record instance graphs*, following `StrongPointer` / `WeakPointer` at the runtime `struct_index` (not at the declared type). Schema-static closures over-fan-out catastrophically on polymorphic bases like `DataForgeComponentParams` (899 subclasses), so the generator asks "which subclasses does this record actually point at?" rather than "which subclasses could theoretically be there?". See `docs/feature-gating-v2.md` §Decision 1 for the full rationale. If you see `core/types.rs` grow past ~1 MB or cfg unions with hundreds of entries, the walker has drifted back to schema-static and needs repair.
- **Double parent-property walk (fixed 2026-04-15).** `collect_full_properties` used to wrap svarog's `get_struct_properties` in a walker that *also* traversed `parent_type_index`. Svarog already walks parents internally. The double-walk collected every ancestor's properties twice and tripped a field-name collision panic. Don't re-introduce the outer walk.
- **Field-name collision fallback (fixed 2026-04-15).** On real DCB data, `TorusFieldGeom` has both `R` (major radius) and `r` (minor radius) — mathematically distinct but sanitize to the same snake_case identifier. `emit_struct`'s collision handler appends a numeric suffix (`r`, `r_2`, `r_3`, ...) rather than panicking. Don't revert to panic — this case is legitimate.
- **`seed_database` dispatch is name-based, not index-based.** `Builder::seed_database` resolves every known record type's `struct_index` against the **live** DCB by name (`db.struct_name(i)`) and builds a per-run `Vec<Option<fn>>` dispatch table. This means a game patch that adds or reorders struct types does not silently drop records — only *newly added* record types are unknown, and existing ones keep being recognised even if their index shifted. The trade is a one-time O(n_structs) HashMap build at parse start.
- **Debug builds panic on generator staleness.** In debug builds (`cfg(debug_assertions)`), `seed_database` tracks every record struct_index whose type isn't in the generated dispatch table, and panics at end-of-seed with the full list of unknown type names. Release builds dead-code-eliminate the check — unknown records are silently skipped and the app keeps working with whatever subset it understands. This means: run a debug build after a game patch and you'll immediately know if you need to regenerate; ship release builds and users get graceful degradation until you push an update.
- **rustc stack bump was a serde-derive workaround, now removed.** Pre-derive-drop, `rustc` could overflow its 8 MB thread stack while parsing/typechecking the generated crate due to serde-derive proc-macro expansion across ~6.5k types. `.cargo/config.toml` used to set `RUST_MIN_STACK=67108864`. With serde derives gone, the default stack is sufficient — no workaround in `.cargo/config.toml` anymore. If you hit `STATUS_STACK_OVERFLOW` inside rustc again, that's a regression: investigate what re-introduced the derive load rather than re-adding the env var.
- **`Self`, `self`, `super`, `extern`, `crate` can't be raw identifiers in type position.** Generator escapes these with a trailing underscore (`Self_`). Most other keywords become `r#keyword`.
- **DCB enums can have a value literally named `Unknown`.** The generator's catch-all fallback variant is named `Unrecognized(String)`, not `Unknown(String)`, to avoid collisions. Don't rename it back.
- **Poly enums use a different `Unknown` shape.** For polymorphic pointer fields, the fallback variant is `Unknown { struct_index: u32, instance_index: u32 }` — a two-field struct variant carrying the raw-layer escape hatch. Consumers holding `Datacore` can resolve an `Unknown` through `datacore.db().instance(struct_index, instance_index)`. Don't collapse this back to a single-`u32` `Unknown(u32)` — you'd lose the instance handle and break the escape hatch.
- **`LocaleKey` lives in `sc-extract-generated`, not `sc-extract`.** Generated struct fields reference `LocaleKey` for every `DataType::Locale` field. Putting the newtype in `sc-extract` would create a dep cycle (since `sc-extract` already depends on `sc-extract-generated`). `sc-extract` re-exports it unchanged via the `locale` module.
- **Let-chains are fine.** The workspace is on `rust-version = "1.94.1"`; let-chains have been stable since 1.88. Previously-nested `if let` blocks in `manifest.rs` and `tags.rs` were converted to let-chains after the version bump.
- **Launcher log has two formats** (legacy plain-text + JSON v2.x). `sc-installs::log_parser` uses a single regex that works for both — don't split into separate parsers.
- **sc-langpatch writes `global.ini` as UTF-8 with BOM, not UTF-16 LE.** `LocaleMap::serialize` produces UTF-8+BOM for compatibility. `LocaleMap::parse` handles UTF-16 LE (the format shipped in `Data.p4k`), and `LocaleMap::parse_utf8_bom` handles the override-file format.

## What's done and what isn't

See `status.md` for the always-current version. Brief snapshot:

- ✅ **`sc-installs`** — fully implemented, 51 tests
- ✅ **`sc-extract`** — phases 2a + 2c + 2d done. API reworked 2026-04-13 into three staged entry points (`AssetSource::from_install` → `AssetData::extract` → `Datacore::parse`). `Datacore` owns the live `DataCoreDatabase` so raw svarog queries stay available post-parse. **Snapshot format reworked 2026-04-15 (later) to v5**: `ExtractSnapshot` now archives raw DCB + locale bytes (`{meta, files: BTreeMap<String, Vec<u8>>}`) instead of the cooked `DatacoreSnapshot`; the quartet is `capture` / `save` / `load` / `hydrate`. `AssetSource` became dual-sourced (live P4K or memory-backed byte map from a snapshot). 94 tests + 1 ignored live-roundtrip integration test.
- ⏭️ **`sc-extract` phase 2b** — vehicle XML — **deliberately skipped**
- ✅ **`sc-extract-generated`** — core/dormant/multi_feature split + 245 leaf feature dirs, typed enums, typed `LocaleKey`, poly enums with raw escape-hatch `Unknown`.
- ✅ **`sc-generator`** — codegen + data-driven feature classification + Cargo.toml generation, typed enum/locale emission, ~3s run
- 📦 **`sc-ammo`** — spec exists (`docs/sc-ammo.md`), crate not scaffolded
- 📦 **`sc-weapons`** — spec exists (`docs/sc-weapons.md`), stub crate only
- 📦 **`sc-contracts`** — stub crate only

## Where to read more

- **Design history and rationale**: `docs/*.md` — the specs. Read `sc-extract.md` and `codegen.md` first; they cover the most ground.
- **Implementation details and gotchas**: `implementing/*.md` — phase notes. Read `sc-generator.md` for the compile-time story and the generator architecture.
- **Current work state**: `status.md`.
- **Reference for the DCB binary format**: `docs/datacore.md`.

## Reference to other repos

When working here, you may need to read code in sibling repos:

- **`E:\repros\Svarog`** — read-only local clone of the svarog source. Use this to look up svarog types and APIs (e.g. `crates/svarog-datacore/src/instance.rs` for `Instance` / `Record` methods). Do not modify.
- **`E:\vscode\rust\bulkhead`** — the combat simulator. Reference implementation for damage-pipeline logic and the DCB data model docs (`docs/damage-system.md`, `docs/data-model.md`). Mostly scaffold, not a working app.
- **`E:\vscode\rust\sc-damage-calculator`** — bulkhead's predecessor. Has a real working vehicle XML parser at `src/extract/hull.rs` (~490 lines). When sc-extract phase 2b is revived, port from this.
- **`E:\vscode\rust\sc-langpatch`** — contract-patching Tauri app. Has working `discovery.rs` (reference for sc-installs) and a real contract-enhancer module (`src-tauri/src/modules/mission_enhancer.rs`) that shows the DCB contract system's complexity.
- **`E:\vscode\streamdeck\streamdeck-starcitizen`** — Stream Deck plugin. `src/discovery.rs` is the canonical prior-art for launcher log parsing.
- **`C:\Games\StarCitizen\LIVE\Data.p4k`** — the game data file sc-generator consumes. Also extracted at `C:\Games\StarCitizen\Extracted\` for browsing.
