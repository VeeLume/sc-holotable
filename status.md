# sc-holotable — work status

> Current work state. Read `CLAUDE.md` first for the workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Last worked on

**Feature-gated generated code** — the generator now produces per-feature directories with `#[cfg(feature = "...")]` gates, reducing cold compile times by ~5x for consumers that only need a subset of DCB types.

### What shipped (2026-04-12)

1. **`ExtractConfig`** — configurable extraction pipeline. Consumers can skip expensive indices (reference graph, tag tree, manufacturers, etc.) via `ExtractConfig::standard()` / `ExtractConfig::minimal()` / `ExtractConfig::builder()`.

2. **`parse_from_install`** — ergonomic entry point that auto-fills `game_version` / `build_id` from the `Installation`'s manifest. `parse_from_p4k` still works for raw paths.

3. **Iterative reference graph walker** — replaced recursive `collect_refs_from_instance` with an iterative worklist + per-record instance dedup. Simplified `ReferenceGraph` to `Guid → Vec<Guid>` (no field_path strings). SCHEMA_VERSION bumped to 3.

4. **Feature-gated generated code** — the generator:
   - Classifies types into features based on DCB record file paths
   - Uses compile cost (total field count in BFS type closure) as the split metric
   - Emits per-feature directories (`core/`, `audio/`, `entities-scitem-ships/`, etc.)
   - Writes `[features]` sections to both `sc-extract-generated/Cargo.toml` and `sc-extract/Cargo.toml`
   - Features mirror the path hierarchy; parent features enable all children

5. **Svarog pinned** to `ce06ec67` as workspace dependencies.

6. **8 MB Windows stack** for binaries (fixes snapshot deserialization stack overflow).

7. **Three release profiles** — `dev-opt` (fast compile), `release` (balanced), `release-max` (max runtime).

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

### Runtime (parse_real_p4k with ExtractConfig::standard(), graph off)

| Features | Records | Parse time | Snapshot size | Load time |
|---|---|---|---|---|
| Core only | 60,669 | 17.0s | 12.35 MB | 1.35s |
| Core + ships | 60,836 | 16.4s | 12.42 MB | 1.34s |
| Full | 111,928 | 18.9s | 16.79 MB | 2.33s |

### With reference graph (ExtractConfig::all(), full features)

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
| `sc-extract` phase 2d | ✅ ExtractedData, AssetSource, parse_from_p4k/install, snapshot save/load |
| `sc-extract` ExtractConfig | ✅ configurable pipeline (standard/all/minimal/builder) |
| `sc-extract-generated` | ✅ flat-pool model, feature-gated per-feature directories |
| `sc-generator` | ✅ codegen + feature classification + Cargo.toml generation |
| `sc-ammo` | 📦 spec only |
| `sc-weapons` | 📦 stub only |
| `sc-contracts` | 📦 stub only |

### Test counts

- `sc-installs`: 51 tests
- `sc-extract`: 88 tests
- `sc-generator`: 3 tests
- **Total: 142 tests, all passing**

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
- ~~**game_version / build_id hardcoded**~~ — Done: `parse_from_install` fills from manifest.
- ~~**Reference graph stack overflow**~~ — Done: iterative walker + instance dedup.
- ~~**Compile time**~~ — Done: feature-gated code, 5x improvement.

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
| Total tests passing | 142 |
| svarog commit | `ce06ec67` (pinned) |

## Fresh-session checklist

1. Read `CLAUDE.md` for orientation and gotchas.
2. Read this file (`status.md`) for current work state.
3. `cargo check -p sc-extract` to verify warm build.
4. Run smoke test: `cargo run -p sc-extract --profile dev-opt --example parse_real_p4k`
5. For full extraction: `cargo run -p sc-extract --profile dev-opt --features full --example parse_real_p4k`
