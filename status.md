# sc-holotable — work status

> Current work state. Read `CLAUDE.md` first for the workspace orientation;
> this file is the always-current "where we left off" snapshot.

## ⚠ Next session — start here

**Test harness has a methodology bug that was patched but not yet verified end-to-end.** Before resuming the compile-time iteration cycle, the [`tools/bench/bench.ps1`](tools/bench/bench.ps1) `Invoke-ColdBuild` change needs validation. The script now runs `cargo build -p sc-extract --release --example parse_real_p4k` instead of the old lib-only command, so the cold-build measurement actually includes the fat LTO link pass (which only fires when linking a final binary, not when producing an rlib). Until this is verified working and a fresh baseline is captured, **do not draw conclusions from any compile-time numbers in [`docs/benchmarks.md`](docs/benchmarks.md) before 2026-04-16 01:28** — they were measured against the lib-only build and underreport real binary build cost by 2-3× (observed: ships lib=50s vs binary=2m 37s under fat LTO + cu=256). Runtime numbers are unaffected throughout.

**To resume:**

1. Run `.\tools\bench\bench.ps1 -Mode all -Features entities-scitem-ships,full -KillRa` against the current `Cargo.toml` (fat LTO + `sc-extract-generated.opt-level = 1` + `release.codegen-units = 256`). Confirm that the build numbers reported by the script now match the binary builds reported by the cargo trace inside the runtime step (i.e. the gap is gone).
2. Establish a clean post-fix baseline by reverting per-package overrides one at a time and re-running, so the LTO + opt-level + cgu sweeps have a comparable starting point under the corrected methodology.
3. Re-run the load-bearing decisions with valid numbers: thin vs fat LTO at the binary link is the most likely conclusion to flip, since fat's LTO link is exactly the thing the old measurement was hiding.
4. Continue the iteration log in [`docs/benchmarks.md`](docs/benchmarks.md) §iteration-log from the marked methodology-change line.

The state in `Cargo.toml` and `.cargo/config.toml` at session end is the partial iteration result, **not a validated answer**. It's a defensible starting point but should be rebaselined under the new harness before any further commits change the profile.

## Last worked on

**Compile-time iteration cycle (2026-04-15 evening → 2026-04-16 early morning)** — multi-step exploration to bring sc-extract cold compile times down. Three coupled phases:

1. **Generator derive-drop (commit `06e0d2e`).** Stripped `Serialize`/`Deserialize`/`Debug`/`Clone` from generated structs, `Serialize`/`Deserialize` and the manual `Default` impl from generated enums, and `serde` entirely from `sc-extract-generated`'s dep graph. ~12,500 proc-macro expansions eliminated. Cold full check 4m 19s → 37.66s (-85%); cold full dev-opt build 7m 10s → 2m 53s (-60%); peak full RAM 13.63 GB → 4.41 GB. Runtime parse times unchanged, as expected.

2. **Profile + `.cargo/config.toml` reset (commit `e538c27`).** Deleted `dev-opt` and `release-max` custom profiles, dropped every per-package and per-profile override in `Cargo.toml`, removed the `RUST_MIN_STACK` env var and Windows linker `/STACK` override from `.cargo/config.toml`. All of this customization was serde-derive workaround that no longer applied. Default `cargo check` works at opt-level 0 with no stack bump, test suite still 94/94. Updated `bench.ps1` + `tools/bench/README.md` to use `--release` instead of the now-deleted `dev-opt` profile.

3. **Iteration log on `entities-scitem-ships` + occasional `full` spot checks** in [`docs/benchmarks.md`](docs/benchmarks.md) §iteration-log:

   - **LTO sweep**: `lto = false` (default, the worst spot — runs local-thin per CGU at opt=3, duplicating full optimization 16x); `lto = "off"` (worst runtime, +20% parse); `lto = "thin"` (best of both); `lto = "fat"` (tied with thin on compile, marginally better runtime). Picked `fat`. Restored `link-args=/STACK:8388608` for Windows runtime stack — the 1 MB default overflows during full DCB extract dispatch, despite being a serde-unrelated cause (commit `fcf979d`).
   - **Opt-level on `sc-extract-generated`**: `opt=0` (-34% compile, +10% runtime); `opt=1` and `opt=3` essentially tied on both ships and full (fat LTO does the real optimization at link time). Picked `opt=1` over `opt=0` for release semantics — a compile-optimized profile can come later if needed.
   - **Opt-level on `sc-extract`**: `opt=1` here is a clear runtime regression (+50% parse) because the hand-written algorithms (graph BFS, tag tree, locale parse) need full LLVM optimization that LTO can't retroactively apply. Reverted.
   - **Codegen-units**: `cu=1` doubles compile time, `cu=256` (in progress) is the first config measured under the fixed harness.

4. **Methodology bug discovered (end of session).** All cold-build numbers prior to 2026-04-16 01:28 were lib-only (`cargo build -p sc-extract --release`), which doesn't run the fat LTO link pass — fat LTO only fires when linking an actual binary. The bench script's `Invoke-ColdBuild` was patched to use `--example parse_real_p4k` to force a real binary link, but the patched script needs validation and the prior comparisons need re-measurement before the LTO / opt-level / cgu conclusions are trustworthy. Runtime parse numbers throughout the session are valid because they came from binaries that did get LTO.

**Snapshot byte-bundle rework v4 → v5 (2026-04-15, later)** — fixed the compile-time regression surfaced by the benchmarking run: sc-extract was spending ~90 % of its LLVM IR on `rmp_serde` monomorphizations of the cooked `DatacoreSnapshot`. Switched `ExtractSnapshot` to archive the raw `game2.dcb` + `english/global.ini` bytes instead of cooked typed pools, and added a `hydrate` step that re-parses at load time. Historical-snapshot comparison (bulkhead's weapon-diff feature) now works automatically because the DCB format is game-stable and field resolution happens at parse time. Plan file: [generic-marinating-plum.md](D:\Users\Valerie\.claude\plans\generic-marinating-plum.md).

### What shipped (2026-04-15, later) — v5 snapshot rework

1. **`AssetSource` is dual-sourced.** Internal enum over `Live(P4kArchive + PathBuf)` and `Memory(BTreeMap<String, Vec<u8>> + label)`. Public reading API (`read`, `try_read`, `find_and_read`, `default_profile_xml`, `vehicle_xml`) dispatches over both. New constructor `AssetSource::from_snapshot(files, label)`. `Datacore::parse` and `AssetData::extract` are unchanged — they consume either backing store through the same `find_and_read` predicate pattern. `archive()` and `source_path()` now return `Option` (both had zero callers in-tree).

2. **`ExtractSnapshot` reshape.** Replaced `{meta, asset_data: Option<AssetData>, datacore: Option<DatacoreSnapshot>}` with `{meta, files: BTreeMap<String, Vec<u8>>}`. `SCHEMA_VERSION` bumped **4 → 5**. The msgpack envelope's type graph is now entirely primitive (`u32`, `String`, `Vec<u8>`, `BTreeMap`), so `rmp_serde::to_vec_named(self)` monomorphizes once for trivial shapes instead of 1,756+ times for generated types.

3. **Three-phase API**:
   - `ExtractSnapshot::capture(assets, meta, config)` — read raw bytes from a live `AssetSource` and build the `files` map. Uses the same DCB / locale predicates currently in `Datacore::parse` / `AssetData::extract`.
   - `ExtractSnapshot::load(path)` — unchanged structurally: read + zstd decode + msgpack decode. Cheap; consumers can enumerate snapshots and inspect `meta` without paying parse cost.
   - `ExtractSnapshot::hydrate(asset_config, dc_config)` — construct a memory-backed `AssetSource` from `files` and re-run the full `AssetData::extract` + `Datacore::parse` pipeline. ~15–25 s of work.

4. **`SnapshotCaptureConfig`** — `{archive_dcb, archive_locale, archive_vehicle_xmls}` with `standard()` / `datacore_only()` / `minimal()` presets. `archive_vehicle_xmls` is wired as a placeholder for phase 2b — flipping it later won't require a format bump because the BTreeMap shape absorbs new file categories transparently.

5. **Derives cleaned up.** `Serialize`/`Deserialize` derives removed from `DatacoreSnapshot` (datacore.rs) and `AssetData` (asset_data.rs). Generated types in `sc-extract-generated` keep their derives (still present in source) but are never instantiated anywhere — no call site monomorphizes them, so rustc emits zero codegen for them. This is the load-bearing change that collapses sc-extract's IR.

6. **New `Error::SnapshotMissingDcb`** variant for the case where `hydrate` is called on a snapshot whose `files` map has no `game2.dcb` entry. `capture` fails fast with `Error::DcbNotFound` if `archive_dcb` is enabled and no DCB is found.

7. **Test rewrite.** Deleted `save_and_load_round_trip` / `assets_only_snapshot` / `datacore_only_snapshot` (the Option<_> taxonomy is gone). Added `round_trips_arbitrary_files_map`, `empty_files_map_round_trips`, `capture_requires_dcb`, `capture_captures_requested_files`, `capture_datacore_only_skips_locale`, `hydrate_rejects_missing_dcb`. Plus a new `#[ignore]`d end-to-end integration test at `crates/sc-extract/tests/snapshot_live_roundtrip.rs` that exercises the full capture → save → load → hydrate cycle against a real SC install and compares record/graph/tag/manufacturer/display-name counts between live parse and hydrated. 94 lib tests passing (was 91 pre-rework).

8. **`parse_real_p4k` example rewrite.** Times `capture`, `save`, `load`, `hydrate` separately so the per-phase cost split is visible in smoke-test output. Handles `--no-assets` via `SnapshotCaptureConfig { archive_locale: false, .. }` and `AssetConfig::minimal()` on hydrate.

9. **Docs updated**: `docs/sc-extract.md` snapshot format section fully rewritten (v5 envelope, capture/load/hydrate split, monomorphization rationale, historical-compatibility argument). Parsing-model "why eager" section simplified (the snapshot-requires-it rationale dropped since it no longer applies). References to "serialized in snapshot" / "cached in snapshot" throughout the doc updated to "rebuilt on hydrate".

**Expected compile-time win**: sc-extract IR drops from ~4.2 M → ~400 k LLVM lines for ammoparams; projected wall-clock `cargo build --profile dev-opt --features full` drops from 3h 26m → ~25–40 m, peak RAM from 25.67 GB → ~6 GB. To be confirmed by `bench.ps1 -Mode build -KillRa` after landing.

### What shipped earlier on 2026-04-15 — typed enums + core split

*Landed the generator half of the feature-gating v2 design: typed enums for DCB `EnumChoice` fields (with a forward-compat `Unrecognized(String)` fallback), a typed `LocaleKey` newtype for `Locale` fields, and a split of what used to be one wild `core` module into three physical modules (`core/`, `dormant/`, `multi_feature/`). Also fixed two generator bugs surfaced during regeneration (double parent-property walk; field-name collision panic on `TorusFieldGeom`'s `R`/`r` radii). See commit `bc9f899` for the full diff.*

1. **Typed `LocaleKey`** lives in `sc-extract-generated/src/locale_key.rs` and is re-exported from `sc-extract` unchanged. The generator routes `DataType::Locale` fields to `LocaleKey` instead of `String`, so consumers get a distinct newtype at compile time. `LocaleMap::get` / `resolve` / `contains_key` widened to `impl AsRef<str>` so `&LocaleKey` passes natively. `display_names.rs` uses `LocaleKey` as the typed intermediate.

2. **Typed enum fields.** `DataType::EnumChoice` now emits as the corresponding generated Rust enum (routed via `prop.struct_index`, which is dual-used as the enum index for choice fields). Every generated enum gets a `from_dcb_str` associated function and a manual `Default` impl so the `#[serde(default)]` on every struct field still resolves. Out-of-range enum indices fall back to `String` for safety.

3. **Core split into core / dormant / multi_feature** — three physical modules under `crates/sc-extract-generated/src/generated/`:

   | Module | Types | Gate | When compiled |
   |---|---:|---|---|
   | `core/` | **336** | unconditional | always — empty polymorphic bases promoted to core (Decision 4 from feature-gating-v2) |
   | `multi_feature/` | **3,789** | per-type `#[cfg(any(feature = "…"))]` | always, but individual types are cfg-gated by the union of features that need them |
   | `dormant/` | **809** | module-level `#[cfg(feature = "dormant")]` | only when the `dormant` Cargo feature is enabled; schema-reachable types never observed in real DCB data |
   | leaf feature dirs | ~1,300 | `#[cfg(feature = "…")]` | one dir per leaf feature (245 dirs) |

   Note: `multi-feature` is a **physical module name, not a Cargo feature**. It's excluded from the feature list written to both `Cargo.toml`s.

4. **Empty-feature-dir skip** — the emission loop now skips any feature whose assignment set is empty (except `core`, always emitted). Dropped 566 dead feature directories (809 → **245** leaf features).

5. **Svarog `[patch]` override** — root `Cargo.toml` carries a `[patch."https://github.com/19h/Svarog.git"]` section pointing at a local clone at `E:/repros/Svarog` that has a fix for `DataCoreReference::is_null` (treats `instance_index == -1` as a null sentinel). Without this, svarog parses thousands of explicit-null scalar `Reference` fields as `Some(garbage_guid)`. **Absolute path — only resolves on Valerie's machine.** Drop the patch once the fix lands upstream and the pinned `ce06ec67` rev is bumped.

6. **Generator bugs fixed during regeneration**:
   - `collect_full_properties` was wrapping svarog's `get_struct_properties` in a recursive walker that *also* walked `parent_type_index`. Svarog already walks parents internally, so every ancestor's properties were collected twice and caused field-name collision panics. Replaced with a direct `db.get_struct_properties` call.
   - `emit_struct`'s field-name collision handler now appends a numeric suffix (`r`, `r_2`, `r_3`, ...) instead of panicking. Needed for the genuine case collision on `TorusFieldGeom` where `R` (major radius) and `r` (minor radius) are mathematically distinct but sanitize to the same snake_case identifier.

7. **Generator flags cleaned up.** `--dump-paths`, `--dump-features`, `--check-polymorphism`, `--analyze-poly-bases`, `--measure-dormancy`, `--feature-closure`, `--measure-cfg-spread` — all removed from the shipped `sc-generator` CLI. Current flags: `--p4k`, `--out-dir`, `--check`, `--dump-refs`. The diagnostic tooling used during the feature-gating-v2 design session was retired once the design was committed; reintroduce it as temporary branches if future measurement is needed.

### What shipped previously (2026-04-13) — API rework

> The fresh-session chain is: open an install with `AssetSource::from_install`, then `AssetData::extract` for non-DCB assets, then `Datacore::parse` for the record store. All three are independent — consumers pick the ones they need.

1. **Staged entry points** replacing the old `parse_from_install{,_with}` / `parse_from_p4k` / `parse_with_config` / `parse_snapshot_only` family:
   - [`AssetSource::from_install`] — open the P4K from a discovered install
   - [`AssetData::extract`] — parse asset-sourced files (currently just `global.ini`)
   - [`Datacore::parse`] — build the record store + indices; returns a live session that owns the `DataCoreDatabase`

2. **Runtime / data split.** [`Datacore`] (live) owns the db + a [`DatacoreSnapshot`] (serde). [`AssetData`] (serde) holds the locale. Consumers reach for the runtime type when they need raw svarog queries and the snapshot type when they're reading cooked data.

3. **Single on-disk format.** [`ExtractSnapshot`] bundles `{ meta, asset_data: Option<…>, datacore: Option<…> }` into one msgpack+zstd file, covering all four consumer patterns (install-only / assets / datacore / both). `SCHEMA_VERSION` bumped to **4**.

4. **`SnapshotMeta`** holds provenance (schema version, game version, build id, `extracted_at` as RFC 3339). The pre-rework `generator_version` / `p4k_sha256` / `game_branch` fields were never implemented and are not in the new shape.

5. **`DatacoreConfig` + `AssetConfig`** replace the old flat `ExtractConfig`. Locale moved to `AssetConfig::build_locale` because it's asset-sourced, not DCB-derived.

6. **`current_timestamp()` + `snapshot_meta_from_install()`** free helpers, using `chrono` for real RFC 3339.

7. **`filters.rs` unchanged** — `is_playable_ship` / `is_playable_weapon` are pure predicates and stay as free functions until a concrete consumer shape justifies folding them into iteration helpers.

### What shipped previously (2026-04-12)

1. **Feature-gated generated code** — the generator classifies types into features based on DCB record file paths, uses compile cost (total field count in BFS type closure) as the split metric, emits per-feature directories (`core/`, `audio/`, `entities-scitem-ships/`, etc.), and writes `[features]` sections to both `sc-extract-generated/Cargo.toml` and `sc-extract/Cargo.toml`. Parent features automatically include all children.

2. **Iterative reference graph walker** — replaced recursive `collect_refs_from_instance` with an iterative worklist + per-record instance dedup. Simplified `ReferenceGraph` to `Guid → Vec<Guid>`. SCHEMA_VERSION was bumped to 3 (and now to 4 with the rework).

3. **Svarog pinned** to `ce06ec67` as workspace dependencies.

4. **Profiles and `.cargo/config.toml` reset to cargo defaults** (2026-04-15, post-derive-drop). Previous workspace customization was all serde-derive workarounds; clean slate now. New overrides get re-introduced only when measurement justifies them.

## Benchmarks

> **Canonical benchmark tables live in [`docs/benchmarks.md`](docs/benchmarks.md).** Re-run them via [`tools/bench/bench.ps1 -Mode all -KillRa`](tools/bench/README.md) after any generator-output change. The snapshot below is kept as a quick-reference summary; edit both places together.

> **Stale after 2026-04-15.** The numbers below were measured against the pre-polymorphism generator output (1,935 emitted types). The current output has 6,234 emitted types split into core/dormant/multi_feature/leaf dirs — compile times and snapshot sizes have changed and need re-measurement. Only `cargo check -p sc-extract --features full` has a fresh datapoint: **4m 15s cold** (from commit `bc9f899`'s verification block). Partial noisy numbers from a 2026-04-15 RA-contaminated run are preserved in `docs/benchmarks.md` under "History".

### Compile times (cold dev-opt build, PRE-2026-04-15, OUT OF DATE)

| Features enabled | Compile time | vs Full |
|---|---|---|
| Core only (default) | **4m 19s** | **4.8x faster** |
| Core + ships | 4m 38s | 4.5x faster |
| Full (all 1,935 types) | 20m 56s | baseline |
| Old monolithic (before feature gating) | ~21 min | — |

### Cold `cargo check` times (dev profile, PRE-2026-04-15, OUT OF DATE)

| Features | Check time |
|---|---|
| Core only | **26s** |
| Core + ammoparams | 28s |
| Full | 1m 41s (stale) → 4m 15s (2026-04-15) |
| Old monolithic | ~6 min |

The jump on `--features full` is expected: the generator now emits ~3.2× more types (6,234 vs 1,935) because polymorphism correctness pulls in every observed subclass. Default (core + multi_feature always-compiled, ~4,125 types) should land somewhere between the old 26 s and the new 4 m 15 s — measure and record.

### Runtime (parse_real_p4k with DatacoreConfig::standard(), graph off, PRE-2026-04-15)

| Features | Records | Parse time | Snapshot size | Load time |
|---|---|---|---|---|
| Core only | 60,669 | 17.0s | 12.35 MB | 1.35s |
| Core + ships | 60,836 | 16.4s | 12.42 MB | 1.34s |
| Full | 111,928 | 18.9s | 16.79 MB | 2.33s |

### With reference graph (DatacoreConfig::all(), full features, PRE-2026-04-15)

| Metric | Value |
|---|---|
| Total parse time | ~25s |
| Graph build time | ~7.4s |
| Graph edges | 1,427,349 |
| Snapshot size (with graph) | 34.88 MB |

## Tasks

| # | Task | Status |
|---|---|---|
| 1 | Implement sc-installs (port + tests) | ✅ completed |
| 2 | Implement sc-extract hand-written foundation | ✅ completed (2a + 2c + 2d) |
| 3 | Implement sc-generator (tools/sc-generator) | ✅ completed |
| 4 | End-to-end smoke test | ✅ completed — pipeline runs to completion |
| 5 | Feature-gated generated code | ✅ completed — 5x compile time reduction |

## Implementation status per crate

| Crate / Tool | Status |
|---|---|
| `sc-installs` | ✅ 51 tests, full spec |
| `sc-extract` phase 2a | ✅ error, LocaleMap, svarog re-exports |
| `sc-extract` phase 2b | ⏭️ skipped (vehicle XML deferred) |
| `sc-extract` phase 2c | ✅ ReferenceGraph, TagTree, ManufacturerRegistry, DisplayNameCache, filters |
| `sc-extract` phase 2d | ✅ AssetSource (dual-sourced), `Datacore`, `DatacoreSnapshot`, `AssetData`, `ExtractSnapshot` (v5 byte-bundle rework 2026-04-15 after the 2026-04-13 API rework) |
| `sc-extract` configs | ✅ `DatacoreConfig` + `AssetConfig` (replaced flat `ExtractConfig`); builders + presets (all/standard/minimal) |
| `sc-extract-generated` | ✅ flat-pool model, core/dormant/multi_feature + 245 leaf feature dirs, typed enums, typed `LocaleKey` |
| `sc-generator` | ✅ codegen + feature classification + Cargo.toml generation + typed enum/locale emission |
| `sc-ammo` | 📦 spec only (not in workspace members) |
| `sc-weapons` | 📦 stub only |
| `sc-contracts` | 📦 stub only |

### Test counts

- `sc-installs`: 51 tests (+ 1 doctest)
- `sc-extract`: 91 tests (+ 3 doctests) — added `resolve_accepts_lockey_newtype` on 2026-04-15
- `sc-generator`: 3 tests
- **Total: 145 tests + 4 doctests, all passing**

## Feature classification details

The generator's feature classification algorithm (post-2026-04-15, implementing the feature-gating-v2 design):
- Walks the DCB record path tree top-down from root to produce a large leaf-feature set, then splits types between three physical modules and per-feature leaf dirs based on cfg width.
- **Empty polymorphic bases** (zero own fields) are promoted unconditionally to `core/` — they're abstract markers that cost nothing to compile.
- **Types touched by ≥2 features** (cfg width ≥ 2) are routed to `multi_feature/` with a per-type `#[cfg(any(feature = "…"))]` listing every feature whose closure contains them.
- **Types reachable from a single leaf feature** live in that feature's leaf dir with a single `#[cfg(feature = "…")]`.
- **Types defined in the schema but never observed in real DCB data** are routed to `dormant/` under a whole-module `#[cfg(feature = "dormant")]` gate. They're emitted but compile only when a consumer opts in.
- Feature names mirror the DCB record path: `entities-scitem-ships`, `audio`, `ammoparams`. Parent features (`entities`, …) automatically include all children.

Current classification (SC 4.7 branch, 2026-04-15):

| Bucket | Types | % of total |
|---|---:|---:|
| `core/` (always compiled, unconditional) | 336 | 5.4% |
| `multi_feature/` (always compiled, per-type cfg) | 3,789 | 60.8% |
| `dormant/` (gated behind `dormant` feature) | 809 | 13.0% |
| leaf feature dirs (gated per leaf) | ~1,300 | 20.8% |
| **Total emitted** | **6,234** | — |

- DCB schema counts: 6,545 structs + 760 enums
- Reachability prune: 6,545 → 6,234 structs (311 unreachable)
- 245 leaf feature dirs (down from 811 pre-skip, 227 pre-polymorphism)
- Empty-feature-dir skip removed 566 dead dirs whose types had been reassigned to core / multi_feature / dormant

## Open questions / unresolved decisions

### Answered

- ~~**svarog commit pinning**~~ — Done: `ce06ec67` as workspace dep (+ local `[patch]` override for `is_null` fix, pending upstream).
- ~~**game_version / build_id hardcoded**~~ — Done: `snapshot_meta_from_install` fills from manifest.
- ~~**Reference graph stack overflow**~~ — Done: iterative walker + instance dedup.
- ~~**Compile time**~~ — Done: feature-gated code. Polymorphism reintroduced pressure but stays tolerable thanks to core/dormant/multi_feature split.
- ~~**Runtime escape hatch for raw svarog queries after parse**~~ — Done: `Datacore` owns the live `DataCoreDatabase`.
- ~~**`parse_from_*` sprawl and tuple return**~~ — Done: three staged entry points, one snapshot type.
- ~~**Polymorphic enum generation**~~ — Done: poly enums emitted (`generated/poly_enums.rs`), `Unknown { struct_index, instance_index }` fallback, empty bases promoted to `core/`.
- ~~**Typed locale and enum fields**~~ — Done: `LocaleKey` newtype + per-enum Rust types with `from_dcb_str`.

### Still open

- **Full dev-opt build regression (2026-04-15 morning, addressed by snapshot rework later that day).** Cold `cargo build -p sc-extract --profile dev-opt --features full` took **3h 25m 58s** and peaked at **25.67 GB** RAM. Investigation via `cargo llvm-lines` identified ~90% of sc-extract's IR as `rmp_serde` monomorphizations transitively reached from `rmp_serde::to_vec_named(&extract_snapshot)`. Fixed by the v5 snapshot byte-bundle rework above — the cooked `DatacoreSnapshot` is no longer serialized, so nothing in the workspace instantiates serde for generated types. Projected post-rework: full cold build ~25–40 m, peak RAM ~6 GB. **Awaiting confirmation via `bench.ps1 -Mode build -KillRa`.**
- **`entities-scitem-ships` hub leaf: 34m dev-opt build (pre-snapshot-rework measurement).** Single leaf feature pulls in most of `multi_feature`. The "hub leaf problem" from feature-gating-v2 is now measured, but the absolute number should drop substantially under v5 snapshot since the hub's monomorphization pressure was driven by the same `rmp_serde` cascade. Narrow ship consumers should still use a more specific sub-feature when we split this hub. Re-measure post-rework to see what remains.
- **~~16 GB machines cannot build `--features full`.~~** (Expected to be resolved by the v5 snapshot rework — peak RAM was 25.67 GB on a 32 GB box pre-rework; projected ~6 GB after. Confirm via `bench.ps1 -Mode build -Features full -KillRa` before closing.)
- **`TagTree::from_database` field names are speculative.** Assumes `tagName` / `parent` / `children` / `legacyGUID`. Smoke test shows 18,313 tags — the fields are at least partially correct. Needs manual validation.
- **`ManufacturerRegistry::from_database` assumptions.** Shows 1,084 manufacturers — working, but field names unverified.
- **`DisplayNameCache::from_database`** — returns 0 entries because `global.ini` was not found in the archive. The path pattern `english/global.ini` may have changed in this build.
- **`Language` enum** — only English. Add others when needed.
- **Locale path** — `global.ini` not found in Data.p4k. Needs investigation (path may have changed).
- **Svarog `[patch]` override is machine-local.** Remove once the `DataCoreReference::is_null` fix lands upstream and the pinned rev is bumped.
- **Benchmarks are stale.** Only `cargo check --features full` has a fresh number (4m 15s). Re-run cold builds, parse-time benchmarks, and snapshot sizes against the 2026-04-15 generator output.

## What's next

The low-level layer (`sc-installs` + `sc-extract` + `sc-extract-generated` + `sc-generator`) is **feature-complete enough to start building the high-level domain crates on top**. The remaining open items (locale path, tag/manufacturer field validation, benchmark refresh) are measurement and data-hygiene chores that don't block domain work.

Recommended order:

1. **Start `sc-weapons`** — first real domain crate. The sc-weapons spec at `docs/sc-weapons.md` describes a hand-curated wrapper over the generated `WeaponParams` + `AmmoParams` + `EntityClassDefinition` chain. This is the "low-level → high-level" boundary test: if the typed-enums, typed-LocaleKey, poly-enum, and feature-gating story holds up under a real consumer, we can roll the same pattern to `sc-ammo`, `sc-shields`, `sc-contracts`. If it doesn't, we surface the issue here before committing to the other domain crates.
2. **Refresh benchmarks** — cold compile times for `core only` / `full` / `dormant`, parse times against the 2026-04-15 generator output. Take a `cargo run --example parse_real_p4k` baseline before building sc-weapons so we can see what (if anything) consumer surface adds to compile time.
3. **Investigate locale path** — find where `global.ini` lives in the current `Data.p4k` build; fix `DisplayNameCache` + `LocaleMap` loading.
4. **Validate `TagTree` / `ManufacturerRegistry` field names** against real DCB records now that typed enums are in. These currently work by name-string heuristic and should be switched to typed field access.
5. **Upstream the `is_null` fix** to Svarog, bump the pinned rev, drop the `[patch]` override.

## Numbers

| Metric | Value |
|---|---|
| DCB structs extracted | 6,545 |
| DCB enums extracted | 760 |
| Emitted struct types | 6,234 (pruned from 6,545) |
| Emitted enum types (incl. poly enums) | 1,252 |
| `core/` types (unconditional) | 336 |
| `multi_feature/` types (always compiled, per-type cfg) | 3,789 |
| `dormant/` types (gated on `dormant` feature) | 809 |
| Leaf feature directories | 245 |
| Total types always compiled on default features | ~4,125 (core + multi_feature) |
| Total types on `--features full` | ~5,425 (adds leaf dirs) |
| Total types on `--features dormant` | 6,234 (adds dormant module) |
| Generator run time | ~3 seconds (release) |
| Cold cargo check (`--features full`) | 4m 15s (2026-04-15) |
| Total tests passing | 145 (+ 4 doctests) |
| svarog commit | `ce06ec67` (pinned) + local `[patch]` override for `is_null` |

## Fresh-session checklist

1. Read `CLAUDE.md` for orientation and gotchas.
2. Read this file (`status.md`) for current work state.
3. `cargo check -p sc-extract` to verify warm build.
4. Run smoke test: `cargo run -p sc-extract --release --example parse_real_p4k`
5. For full extraction: `cargo run -p sc-extract --release --features full --example parse_real_p4k`
