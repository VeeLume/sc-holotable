# sc-holotable — work status

> Current work state. Read `CLAUDE.md` first for the workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Last worked on

**Flat-pool refactor of `sc-extract-generated` — replaces the recursive `FromInstance` materialiser that required a 128 MB worker-thread stack, plus a set of compile-time optimisations to keep the generated crate tractable.**

Why it was needed: the old generator emitted nested `Option<Box<T>>` fields populated by recursive `FromInstance::from_instance` calls. DCB composition nests 50+ levels deep, overflowing the Windows 1 MB main-thread stack; a 128 MB worker-thread stack was used as a workaround. A game patch reindexing struct types would also silently drop records because the dispatch match baked `struct_index` literals.

Shipped:

1. **`Extract` trait + `Pooled` trait + `Builder`** (hand-written in `crates/sc-extract-generated/src/{extract,handle,builder}.rs`). Replaces `FromInstance`. Each generated struct implements both `Extract` (how to decode one from an `Instance`) and `Pooled` (where its `Vec<Option<T>>` pool lives in `DataPools`). `Builder<'a>` owns `DataPools`, `RecordIndex`, a `VecDeque<PendingSlot>` worklist, and a `(struct_idx, inst_idx) → u32` dedup map. `alloc_nested::<T>(inst, from_pool)` reserves a pool slot, enqueues the instance, and returns a typed `Handle<T>` immediately — no recursion through the call stack.
2. **Generic `Handle<T>`**: one hand-written `Handle<T>(u32, PhantomData<fn() -> T>)` replaces the ~6,500 per-type `{Name}Id(u32)` newtypes an intermediate draft emitted. A single blanket `impl<T: Pooled> Index<Handle<T>> for DataPools` + `Handle<T>::get` covers every pool type. Dropped ~20,000 generated items (struct defs + Index impls + inherent gets) that were pure ergonomic decoration.
3. **Reachability pruning in the generator.** `compute_reachable_struct_indices` does a transitive BFS from every record type through Class / StrongPointer / WeakPointer fields; struct definitions that no record transitively references are dropped at generation time. Drops emission from **6,545 → 1,935 types** (~70%). A generation-time self-check walks every emitted field and panics if any target is outside the reachable set — if it ever fires it means the BFS has a hole and the pruning would be unsafe.
4. **Debug-build staleness panic.** `Builder::seed_database` tracks, in `cfg(debug_assertions)` only, every record struct_index whose type isn't in the generated dispatch table. At the end of seeding, if any were seen, it panics with the full list of unknown type names — direct signal that the generator is out of date relative to the runtime DCB. Release builds dead-code-eliminate the whole collection + check; unknown records are silently skipped and the app keeps working with whatever subset it understands.
5. **Name-based `seed_database` dispatch.** The dispatch table is resolved per-run against the live DCB by `Extract::TYPE_NAME`, not by baked struct_index. A game patch that reorders struct types doesn't silently drop existing record types.
6. **128 MB stack hack removed.** `parse_from_p4k` runs on the main thread, calls `Builder::new(&db).consume_database().finish()`. The iterative drain loop keeps stack depth O(1) regardless of DCB nesting depth.
7. **`SCHEMA_VERSION` bumped 1 → 2.** Old snapshots fail `load_snapshot` cleanly.
8. **Workspace profile fixes.** `.cargo/config.toml` sets `RUST_MIN_STACK = 67108864` (64 MB) so rustc itself doesn't stack-overflow on the giant generated crate. A `[profile.dev.package.sc-extract-generated]` override sets `opt-level = 1, codegen-units = 256` (matches the release override) — without it the dev profile's default `opt-level = 0` produces ~40k `link.exe` LNK2019 errors on serde-derive visitor helpers.
9. **Dedup win.** Pool-backed instances referenced from multiple records (or multiple fields inside a record) share one slot; snapshots deduplicate transparently.

### Final numbers (real `Data.p4k`, 4.7 branch)

| Metric | Before refactor | After refactor |
|---|---|---|
| Raw DCB struct types | 6,545 | 6,545 (unchanged) |
| Emitted struct types | ~6,500 | **1,935** (pruned) |
| Generated directory size | ~23 MB | **5.4 MB** |
| Generator run | ~1.6 s | ~1.8 s |
| Cold `cargo check -p sc-extract-generated` | ~6 min | **~1m40s** |
| Main-thread stack needed at runtime | 128 MB worker thread | default 1 MB, no worker thread |
| Tests | 89 pass | 89 pass |
| Clippy `-D warnings` | clean | clean |

**Still pending**: end-to-end smoke test against real `Data.p4k` — run `parse_real_p4k` example to capture actual parse time, pool sizes, and snapshot size, and verify that the pruned set of emitted types covers everything real records touch at runtime (which the generator-time self-check already proves structurally).

### Post-refactor fix: iterative reference graph walker

First real-data run (2026-04-11) confirmed record store builds successfully (111,928 records in ~12s) but **stack-overflowed** during `ReferenceGraph::from_database`. Root cause: the recursive `collect_refs_from_instance` / `collect_refs_from_value` functions walked DCB composition nesting (50+ levels) on the call stack — same class of bug the flat-pool refactor fixed in the Builder. Windows main-thread stack is 1 MB; the recursive frame accumulated path strings + Instance refs exceeded it.

Fix: replaced the two recursive functions with an iterative worklist inside `from_database`. Worklist items are `(Instance<'_>, Guid, String)` — `Instance` is `Copy` (two `u32`s + two refs), so storing it directly is cheap. Array elements that are classes/pointers are pushed onto the worklist rather than recursed into. Stack depth is now O(1) regardless of DCB nesting.

The other `from_database` methods (`TagTree`, `ManufacturerRegistry`, `DisplayNameCache`) don't recurse deeply — they only read top-level record properties — so they're unaffected.

## Tasks

| # | Task | Status |
|---|---|---|
| 1 | Implement sc-installs (port + tests) | ✅ completed |
| 2 | Implement sc-extract hand-written foundation | ✅ completed (2a + 2c + 2d done; 2b skipped) |
| 3 | Implement sc-generator (tools/sc-generator) | ✅ completed |
| 4 | End-to-end wire-up and evaluation | 🔄 in_progress (implementation done, real-data smoke test pending) |

## Implementation status per crate

| Crate / Tool | Spec | Implementing notes | Status |
|---|---|---|---|
| `sc-installs` | `docs/sc-installs.md` | `implementing/sc-installs.md` | ✅ **done** — 51 tests, 0 clippy warnings |
| `sc-extract` phase 2a | `docs/sc-extract.md` | `implementing/sc-extract-2a.md` | ✅ error type, LocaleMap, svarog re-exports |
| `sc-extract` phase 2b | — | — | ⏭️ **skipped** — superseded by the `AssetSource` layer; vehicle XML parsing deferred to a consumer-driven need |
| `sc-extract` phase 2c | `docs/sc-extract.md` | `implementing/sc-extract-2c.md` | ✅ ReferenceGraph, TagTree, ManufacturerRegistry, DisplayNameCache, playable filters |
| `sc-extract` phase 2d | `docs/sc-extract.md` | — | ✅ **implementation done** — ExtractedData, AssetSource, parse_from_p4k, save/load snapshot; smoke test pending |
| `sc-extract-generated` | `docs/codegen.md` | `implementing/sc-generator.md` | ✅ flat-pool model; `DataPools` + `RecordIndex` + per-type `TId` handles; iterative `Builder::drain` replaces recursive `FromInstance` |
| `sc-generator` | `docs/codegen.md` | `implementing/sc-generator.md` | ✅ **done** — ~1.6s on real Data.p4k, outputs to `sc-extract-generated/src/generated` |
| `sc-ammo` | `docs/sc-ammo.md` | — | 📦 **not scaffolded** — spec only |
| `sc-weapons` | `docs/sc-weapons.md` | — | 📦 **stub only** |
| `sc-contracts` | — | — | 📦 **stub only** |

### Test counts

- `sc-installs`: 51 tests passing
- `sc-extract`: 89 tests passing (85 from phase 2a+2c + 4 new in `extracted::tests` for snapshot round-trip, version mismatch, atomic write)
- `sc-extract-generated`: 0 tests (pure generated code + trait def)
- `sc-generator`: 3 naming tests passing
- **Total: 143 tests**

## What's next

Once the dev-opt build finishes, run:

```bash
cargo run -p sc-extract --profile dev-opt --example parse_real_p4k -- "C:/Games/StarCitizen/LIVE/Data.p4k"
```

The example ([crates/sc-extract/examples/parse_real_p4k.rs](crates/sc-extract/examples/parse_real_p4k.rs)) exercises `parse_from_p4k` → snapshot save → snapshot load → asset-source fetch and prints real numbers.

### Evaluation questions to answer

- Does `parse_from_p4k` run to completion against real data? (First bare-metal attempt crashed with a stack overflow during `build_from_database`; fixed by running on a 32 MB-stack worker thread, needs re-verification.)
- Do the speculative field names in `TagTree::from_database`, `ManufacturerRegistry::from_database`, and `DisplayNameCache::from_database` match reality? (`CLAUDE.md` flags these as guesses carried from earlier phases.)
- Snapshot size — target 5–30 MB zstd-compressed. If it's wildly off, re-check msgpack encoding / whether generated HashMap keys double-encode.
- Parse time — target tens of seconds. `RecordStore::build_from_database` dispatch on 597 match arms is the likely bottleneck; if it's >60s, revisit.
- `load_snapshot` time — target sub-second. Mostly serde + zstd.
- Snapshot round-trip byte-equivalence — not required but a good sanity check.

After answers are in, decide whether:
- The design in `docs/sc-extract.md` matches reality, or needs revision
- The design in `docs/sc-weapons.md` / `docs/sc-ammo.md` is still the right approach
- There are surprising gaps that should become follow-ups before the domain crates land

## Open questions / unresolved decisions

### Carried over from earlier phases

- ~~**svarog commit pinning.**~~ **Done** — pinned to `ce06ec67` as workspace deps in root `Cargo.toml`.
- **`TagTree::from_database` field names are speculative.** Assumes `tagName` / `parent` / `children` / `legacyGUID`. Phase 2d smoke test will reveal whether this is right.
- **`ManufacturerRegistry::from_database` assumes a `Localization` nested instance** with `Name` / `Description` string fields. Unverified.
- **`DisplayNameCache::from_database` assumes the Components chain** `Components → SAttachableComponentParams → AttachDef → Localization → Name`. Unverified.
- **`Language` enum currently has only English.** Add others when a consumer actually needs them.
- **Polymorphic enum generation for abstract base types.** The spec calls for tagged enums over concrete subclasses. Not implemented; probably needed for `sc-weapons` fire actions.

### New / still open

- **First dev-opt build timing unknown.** Measuring right now — the cold-build-after-split number is the headline number for deciding whether the profile split was worth the complexity.
- **`game_version` / `build_id` are filled automatically by `parse_from_install`**, which reads the `BuildManifest` from the `Installation`. `parse_from_p4k` still sets them to `"unknown"` (raw path, no manifest context).

### Deferred design questions

- Ship assembly layering when `sc-ships` eventually lands. Vehicle XML parsing is now covered generically by `AssetSource::vehicle_xml` — a hand-written CryXmlB parser will still be needed when a consumer demands it, but the access layer is in place.
- Damage pipeline stays in bulkhead until `sc-ships` exists.

## Numbers (as of last measurement)

| Metric | Value |
|---|---|
| DCB structs extracted | 6,545 |
| DCB enums extracted | 760 |
| Top-level record types (RecordStore fields) | 597 |
| Generated files | 174 (types_* + enums + metadata + record_store + mod) |
| Total generated lines | ~270,000 |
| Generator run time | ~1.6 seconds |
| sc-extract dev cold check | ~4 minutes |
| sc-extract dev warm check | <1 second |
| First release cold build (pre-split, opt-level 3) | **64 minutes** |
| dev-opt cold build (post-split) | _measuring_ |
| Total tests passing | 143 |
| svarog commit in use | `ce06ec67` (pinned via workspace dep) |

## Fresh-session checklist

If resuming work on this repo after a break:

1. Read `CLAUDE.md` for orientation and gotchas (including the `sc-extract-generated` split rationale and the stack-overflow gotcha).
2. Read this file (`status.md`) for current work state.
3. `cargo check -p sc-extract` to verify the workspace still builds warm.
4. If the smoke test hasn't been run yet, run `cargo run -p sc-extract --profile dev-opt --example parse_real_p4k` and update the numbers section with real data. Answer the evaluation questions above.
5. If the smoke test passed, the next move is **either** start `sc-weapons` (domain crate, first real consumer of `RecordStore`) **or** pin svarog to a commit SHA and regenerate.
