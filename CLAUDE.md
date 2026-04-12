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
│   ├── sc-ammo.md                 ammo crate spec (not implemented yet)
│   └── sc-weapons.md              weapons crate spec (not implemented yet)
├── implementing/                  phase-by-phase implementation notes
│   ├── sc-installs.md
│   ├── sc-extract-2a.md           sc-extract phase 2a (foundation)
│   ├── sc-extract-2c.md           sc-extract phase 2c (graph/tags/manufacturers/etc.)
│   └── sc-generator.md
├── crates/
│   ├── sc-installs/               ✅ implemented, 51 tests passing
│   ├── sc-extract/                ✅ phase 2a + 2c + 2d done (hand-written half)
│   ├── sc-extract-generated/      ✅ workspace-internal — generator output + FromInstance trait
│   │   └── src/generated/         174 files (types_*, enums, record_store, metadata)
│   ├── sc-contracts/              📦 stub only (lib.rs docs, no code)
│   └── sc-weapons/                📦 stub only (lib.rs docs, no code)
└── tools/
    └── sc-generator/              ✅ implemented — offline DCB schema → Rust codegen
```

Note: the spec proposed a `crates/components/` subdirectory for future `sc-ammo`, `sc-shields`, etc. That grouping hasn't been applied to the filesystem yet — `sc-weapons` lives at `crates/sc-weapons`, not `crates/components/sc-weapons`. Defer until we actually add a sibling component crate.

### The sc-extract / sc-extract-generated split

`sc-extract-generated` is a **workspace-internal crate** that holds everything the generator emits plus the `FromInstance` trait. `sc-extract` depends on it and re-exports its public surface, so consumers still only see `sc_extract::RecordStore`, `sc_extract::FromInstance`, etc.

Why split: the generated code is ~270k lines. At `opt-level = 3` the LLVM passes on the giant match statements in `record_store.rs` dominate release build time (first cold release build was 64 minutes). Splitting into its own crate gives us two wins:

1. **Build cache stability** — edits to hand-written sc-extract modules no longer invalidate the LLVM object files for the generated code. Iteration on the hand-written half is fast even in release mode.
2. **Per-crate profile override** — the workspace `Cargo.toml` sets `[profile.release.package.sc-extract-generated]` to `opt-level = 1` with `codegen-units = 256`, which skips the expensive inlining passes and parallelizes aggressively. Runtime cost is negligible (the hot path is load-once-walk-everything, serde and hashmap-bound, not the match dispatch).

Three release-tier profiles trade compile time against runtime speed. All three keep `sc-extract-generated` at `opt-level = 1 / codegen-units = 256` (serde/hashmap-bound, higher opt-levels just burn LLVM time):

| Profile | Compile | Runtime | Use case |
|---|---|---|---|
| `dev-opt` | fast | decent | `cargo build --profile dev-opt` — smoke tests, quick iteration |
| `release` | moderate | good | `cargo build --release` — CI, snapshot generation, consumer builds |
| `release-max` | slow | maximum | `cargo build --profile release-max` — shipped builds, benchmarks |

`release` uses `opt-level = 2` globally (3 gains nothing on serde/hashmap workloads). `release-max` adds `opt-level = 3`, `lto = "thin"`, `codegen-units = 1` for maximum cross-crate inlining on deps.

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

### Feature gates and version pins that bite

- **`specta = "=2.0.0-rc.21"`** — not `"2"`. Cargo resolves `^2` to `rc.24` which requires nightly `debug_closure_helpers`. Matches sc-langpatch's pin.
- **Rust edition 2024**, `rust-version = "1.94.1"`. Let-chains (`if let Some(x) = foo && condition`) are fine — stable since 1.88.

## Running the generator

When a new SC patch lands, regenerate the DataCore bindings:

```bash
cargo run -p sc-generator --release -- --p4k "C:/Games/StarCitizen/LIVE/Data.p4k"
```

This writes ~173 files into `crates/sc-extract/src/generated/`. Review the diff (`git diff crates/sc-extract/src/generated/`), fix any call sites that broke, commit the generated output alongside the hand-written fixes.

Release mode is strongly preferred over debug — DCB parse is tens of times faster. The whole run takes ~1.6 seconds.

Available flags:

- `--p4k <path>` (required)
- `--out-dir <path>` — defaults to `crates/sc-extract-generated/src/generated`
- `--check` — parse everything but don't write files

## Build / test / lint

```bash
# Fast iteration (warm cache ~1s)
cargo check -p sc-extract
cargo check -p sc-installs

# Tests for the libs
cargo test -p sc-installs             # 51 tests
cargo test -p sc-extract              # 89 tests (Phase 2d added 4)
cargo test -p sc-generator            # 3 naming tests

# Clippy (strict)
cargo clippy -p sc-installs --all-targets -- -D warnings
cargo clippy -p sc-extract --all-targets -- -D warnings
cargo clippy -p sc-extract-generated --all-targets -- -D warnings
cargo clippy -p sc-generator --all-targets -- -D warnings

# Cold dev build of sc-extract with all generated code (~6 minutes)
cargo build -p sc-extract

# Smoke tests — use dev-opt for fast compile + decent runtime
cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- "C:/Games/StarCitizen/LIVE/Data.p4k"

# Full release (opt-level 2 for deps, 1 for generated code)
cargo build -p sc-extract --release

# Maximum runtime perf (opt-level 3 + thin LTO — slow to compile)
cargo build -p sc-extract --profile release-max
```

**Warm incremental check is ~1 second.** Cold dev check is ~6 minutes because rustc has to type-check the 170+ generated modules. Use `--profile dev-opt` for quick smoke tests, `--release` as the normal release, and `--profile release-max` only when you need maximum runtime speed and can tolerate a long build. Avoid triggering cold rebuilds when iterating.

## Compile-time gotchas (this will bite you)

The generated code is big — ~6,500 structs, ~760 enums, ~270,000 lines of Rust split across 173 bucket files. A few things keep the compile bearable:

1. **Two-letter bucketing with a 300-struct cap.** `SCItem*` types go to `types_sc.rs`, `Bu*` types to `types_bu1.rs`/`types_bu2.rs`, etc. Never put everything in one file — we measured >10 min compile times before the split.
2. **Generated structs derive only what's needed**: `Debug`, `Clone`, `Serialize`, `Deserialize`. No `Default`, no `PartialEq`/`Eq`/`Hash`. Fewer derives = fewer proc-macro invocations = faster compile.
3. **`#![allow(unused_imports)]` on every bucket file.** Uniform imports keep the emitter simple; the allow silences the noise without per-bucket tracking.
4. **Cold cargo check ≈ 6 minutes.** Warm incremental ≈ 1 second. The warm case is what matters for iteration.

If compile time regresses substantially, the first thing to check is the bucket-size distribution:

```bash
for f in crates/sc-extract-generated/src/generated/types_*.rs; do wc -l "$f"; done | sort -rn | head -10
```

No bucket file should be >30k lines. If one is, the `MAX_STRUCTS_PER_BUCKET` in `tools/sc-generator/src/emit.rs` needs tuning down.

## Specific gotchas encountered during implementation

These are real bugs that were hit and fixed — future edits should not re-introduce them:

- **`CigGuid` doesn't implement serde by default.** svarog-common feature-gates it behind `serde`. Enable the feature in every crate that uses it in a serializable context.
- **`RecordRef` doesn't implement serde at all.** Map DCB `Reference` fields to `Option<CigGuid>` and extract the inner guid, not `Option<RecordRef>`. See the generator's `rust_type_for` function for the mapping.
- **`Instance::database()` doesn't exist.** svarog's `Instance` hides its db handle. Any code that needs to materialize array-of-Class elements or resolve pool instances must reach through the [`Builder`], which owns a `&'a DataCoreDatabase` in its `db` field — see `crates/sc-extract-generated/src/builder.rs`.
- **Flat-pool materialisation is iterative, not recursive.** The generator no longer emits `FromInstance` impls that call themselves through nested `Option<Box<T>>` fields. Instead every emitted struct implements `Extract` + `Pooled`, nested Class / StrongPointer / WeakPointer fields become `Option<Handle<T>>` / `Vec<Handle<T>>` into per-type `Vec<Option<T>>` pools inside `DataPools`, and `Builder::drain` walks a heap-allocated worklist. There is **no** `EXTRACT_THREAD_STACK_SIZE` any more — the main-thread stack is sufficient regardless of DCB nesting depth.
- **`ReferenceGraph::from_database` is also iterative.** The initial recursive walker (`collect_refs_from_instance` / `collect_refs_from_value`) stack-overflowed on real data — same root cause as the Builder before the flat-pool refactor. Replaced with a `Vec<(Instance, Guid, String)>` worklist. Don't re-introduce recursion through Value::Class / ClassRef / StrongPointer / WeakPointer nesting in graph.rs.
- **Typed slot handles are generic, not per-type.** A single hand-written `Handle<T>(u32, PhantomData<fn() -> T>)` covers every emitted type, with blanket `impl<T: Pooled> Index<Handle<T>> for DataPools` / `impl<T: Pooled> Handle<T> { fn get(…) }`. Earlier drafts emitted per-type `TId(u32)` newtypes, Index impls, and inherent `get` blocks — ~20k generated items — which tipped rustc/link.exe into per-type symbol-visibility problems. Moving them to a single generic keeps the compile cost linear in the struct count and got serde-derive symbol resolution back into a working regime.
- **Reachability pruning in the generator.** `compute_reachable_struct_indices` does a transitive BFS from every record type through Class / StrongPointer / WeakPointer fields; struct definitions that no record transitively references are dropped at generation time. Drops the raw 6,545 DCB struct types to ~1,935 emitted types (~70% reduction). A **self-check** runs at generation time and panics if any emitted field's target is outside the reachable set — that would be a BFS bug and should never happen.
- **`seed_database` dispatch is name-based, not index-based.** `Builder::seed_database` resolves every known record type's `struct_index` against the **live** DCB by name (`db.struct_name(i)`) and builds a per-run `Vec<Option<fn>>` dispatch table. This means a game patch that adds or reorders struct types does not silently drop records — only *newly added* record types are unknown, and existing ones keep being recognised even if their index shifted. The trade is a one-time O(n_structs) HashMap build at parse start.
- **Debug builds panic on generator staleness.** In debug builds (`cfg(debug_assertions)`), `seed_database` tracks every record struct_index whose type isn't in the generated dispatch table, and panics at end-of-seed with the full list of unknown type names. Release builds dead-code-eliminate the check — unknown records are silently skipped and the app keeps working with whatever subset it understands. This means: run a debug build after a game patch and you'll immediately know if you need to regenerate; ship release builds and users get graceful degradation until you push an update.
- **rustc needs a bigger stack for the generated crate.** With ~1.9k structs, thousands of `impl Extract` / `impl Pooled` items, and serde derives across them all, `rustc` itself can overflow its default 8 MB thread stack while parsing/typechecking. `.cargo/config.toml` sets `RUST_MIN_STACK=67108864` (64 MB); no per-user configuration needed. If you run rustc directly from a different environment and hit `STATUS_STACK_OVERFLOW` inside the rustc process, that's the fix.
- **`Self`, `self`, `super`, `extern`, `crate` can't be raw identifiers in type position.** Generator escapes these with a trailing underscore (`Self_`). Most other keywords become `r#keyword`.
- **DCB enums can have a value literally named `Unknown`.** The generator's catch-all fallback variant is named `Unrecognized(String)`, not `Unknown(String)`, to avoid collisions. Don't rename it back.
- **Let-chains are fine.** The workspace is on `rust-version = "1.94.1"`; let-chains have been stable since 1.88. Previously-nested `if let` blocks in `manifest.rs` and `tags.rs` were converted to let-chains after the version bump.
- **Launcher log has two formats** (legacy plain-text + JSON v2.x). `sc-installs::log_parser` uses a single regex that works for both — don't split into separate parsers.
- **sc-langpatch writes `global.ini` as UTF-8 with BOM, not UTF-16 LE.** `LocaleMap::serialize` produces UTF-8+BOM for compatibility. `LocaleMap::parse` handles UTF-16 LE (the format shipped in `Data.p4k`), and `LocaleMap::parse_utf8_bom` handles the override-file format.

## What's done and what isn't

See `status.md` for the always-current version. Brief snapshot:

- ✅ **`sc-installs`** — fully implemented, 52 tests passing, full spec + implementing doc
- ✅ **`sc-extract` phase 2a** — foundations: error, FromInstance trait, svarog re-exports, LocaleMap (24 tests)
- ⏭️ **`sc-extract` phase 2b** — vehicle XML — **deliberately skipped** (design not settled)
- ✅ **`sc-extract` phase 2c** — ReferenceGraph, TagTree, ManufacturerRegistry, DisplayNameCache, playable filters (61 tests)
- ✅ **`sc-generator`** — full DCB → Rust codegen, runs in ~1.6s, output compiles clean
- ⏳ **`sc-extract` phase 2d** — `ExtractedData` envelope, `parse_from_p4k` orchestrator, snapshot save/load — **not started**
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
