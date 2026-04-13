# sc-holotable — work status

> Current work state. Read `CLAUDE.md` first for the workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Last worked on

**sc-extract API rework (2026-04-13)** — split the old `ExtractedData` god-struct into three orthogonal layers: live runtime handles, serializable asset data, and serializable datacore data. Removed the five overlapping `parse_*` entry points in favor of three staged building blocks. [`Datacore`] now owns the live `DataCoreDatabase` so raw svarog queries remain available after high-level parsing — a capability the old `parse_inner` flow dropped on the floor.

### What shipped (2026-04-13) — API rework

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

4. **8 MB Windows stack** for binaries (fixes snapshot deserialization stack overflow).

5. **Three release profiles** — `dev-opt` (fast compile), `release` (balanced), `release-max` (max runtime).

## Benchmarks (real Data.p4k, 4.7 branch, 2026-04-12)

### Compile times (cold dev-opt build)

| Features enabled | Compile time | vs Full |
|---|---|---|
| Core only (default) | **4m 19s** | **4.8x faster** |
| Core + ships | 4m 38s | 4.5x faster |
| Full (all 1,935 types) | 20m 56s | baseline |
| Old monolithic (before feature gating) | ~21 min | — |

### Cold `cargo check` times (dev profile)

| Features | Check time |
|---|---|
| Core only | **26s** |
| Core + ammoparams | 28s |
| Full | 1m 41s |
| Old monolithic | ~6 min |

### Runtime (parse_real_p4k with DatacoreConfig::standard(), graph off)

> Numbers from 2026-04-12 (pre-rework). The rework doesn't meaningfully change parse cost — same underlying Builder + iterative walkers — but these should be re-measured after the next `cargo run --example parse_real_p4k` against the 4.7 P4K.

| Features | Records | Parse time | Snapshot size | Load time |
|---|---|---|---|---|
| Core only | 60,669 | 17.0s | 12.35 MB | 1.35s |
| Core + ships | 60,836 | 16.4s | 12.42 MB | 1.34s |
| Full | 111,928 | 18.9s | 16.79 MB | 2.33s |

### With reference graph (DatacoreConfig::all(), full features)

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
| `sc-extract` phase 2d | ✅ AssetSource, `Datacore`, `DatacoreSnapshot`, `AssetData`, `ExtractSnapshot` (rework 2026-04-13 replaced the old `ExtractedData` + `parse_*` family) |
| `sc-extract` configs | ✅ `DatacoreConfig` + `AssetConfig` (replaced flat `ExtractConfig`); builders + presets (all/standard/minimal) |
| `sc-extract-generated` | ✅ flat-pool model, feature-gated per-feature directories |
| `sc-generator` | ✅ codegen + feature classification + Cargo.toml generation |
| `sc-ammo` | 📦 spec only |
| `sc-weapons` | 📦 stub only |
| `sc-contracts` | 📦 stub only |

### Test counts

- `sc-installs`: 51 tests
- `sc-extract`: 90 tests (88 pre-rework + 6 new snapshot round-trip tests − 4 from the deleted `extracted.rs`)
- `sc-generator`: 3 tests
- **Total: 144 tests, all passing**

## Feature classification details

The generator's feature classification algorithm:
- Walks the DCB record path tree top-down from root
- At each node, computes compile cost = Σ field_count(type) for all types in the BFS closure
- Splits when cost > 500 fields, keeps leaf features when cost < 50 fields
- Types shared by 5+ features are promoted to core (always compiled)
- Feature names mirror the path: `entities-scitem-ships`, `audio`, `ammoparams`
- Parent features automatically include all children

Current classification (4.7 branch):
- Core: 214 types, 967 fields (11% of total)
- 227 leaf features, 5 parent features
- Total: 1,935 types, 9,141 fields

## Open questions / unresolved decisions

### Answered

- ~~**svarog commit pinning**~~ — Done: `ce06ec67` as workspace dep.
- ~~**game_version / build_id hardcoded**~~ — Done: `snapshot_meta_from_install` fills from manifest.
- ~~**Reference graph stack overflow**~~ — Done: iterative walker + instance dedup.
- ~~**Compile time**~~ — Done: feature-gated code, 5x improvement.
- ~~**Runtime escape hatch for raw svarog queries after parse**~~ — Done: `Datacore` owns the live `DataCoreDatabase`.
- ~~**`parse_from_*` sprawl and tuple return**~~ — Done: three staged entry points, one snapshot type.

### Still open

- **`TagTree::from_database` field names are speculative.** Assumes `tagName` / `parent` / `children` / `legacyGUID`. Smoke test shows 18,313 tags — the fields are at least partially correct. Needs manual validation.
- **`ManufacturerRegistry::from_database` assumptions.** Shows 1,084 manufacturers — working, but field names unverified.
- **`DisplayNameCache::from_database`** — returns 0 entries because `global.ini` was not found in the archive. The path pattern `english/global.ini` may have changed in this build.
- **`Language` enum** — only English. Add others when needed.
- **Polymorphic enum generation** — not implemented. Needed for `sc-weapons` fire actions.
- **Locale path** — `global.ini` not found in Data.p4k. Needs investigation (path may have changed).

## What's next

1. **Start `sc-weapons`** — first real domain crate consuming `RecordStore`. Will validate the feature-gated type system works ergonomically for consumers.
2. **Investigate locale path** — find where `global.ini` lives in the current Data.p4k build.
3. **Add manual feature aliases** — e.g., `weapons = ["ammoparams", "entities-scitem-ships", "damage"]` for ergonomic consumer usage.

## Numbers

| Metric | Value |
|---|---|
| DCB structs extracted | 6,545 |
| DCB enums extracted | 760 |
| Emitted struct types | 1,935 (pruned from 6,545) |
| Core types (always compiled) | 214 |
| Leaf features | 227 |
| Parent features | 5 |
| Total compile cost | 9,141 fields |
| Generated feature directories | 227 |
| Generator run time | ~3 seconds |
| Cold dev-opt build (core only) | 4m 19s |
| Cold dev-opt build (full) | 20m 56s |
| Cold cargo check (core only) | 26s |
| Total tests passing | 144 |
| svarog commit | `ce06ec67` (pinned) |

## Fresh-session checklist

1. Read `CLAUDE.md` for orientation and gotchas.
2. Read this file (`status.md`) for current work state.
3. `cargo check -p sc-extract` to verify warm build.
4. Run smoke test: `cargo run -p sc-extract --profile dev-opt --example parse_real_p4k`
5. For full extraction: `cargo run -p sc-extract --profile dev-opt --features full --example parse_real_p4k`
