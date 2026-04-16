# `sc-extract` — Phase 2c implementation notes

**Status**: Complete. Modules are stable and integration-tested.

Phase 2c added `graph.rs`, `tags.rs`, `manufacturers.rs`, `display_names.rs`, `filters.rs`. Module APIs are documented in the code; this file captures only the non-obvious context.

## Open caveats

- **`TagTree::from_database`** assumes `tagName` / `parent` / `children` / `legacyGUID` fields. Smoke test shows 18,313 tags — `tagName` works. Parent/children/legacyGUID field names are unverified at the individual-field level. If the DCB structure is different (e.g., tags nested under a TagDatabase record rather than top-level), the tree will need adjustment.

- **`ManufacturerRegistry::from_database`** shows 1,084 manufacturers. Working, but `Localization` nesting and `name_key`/`description_key` extraction unverified at field level.

- **`DisplayNameCache::from_database`** returns 0 entries — `global.ini` was not found in the archive. The path pattern `english/global.ini` may have changed in this build. Needs investigation.

- **`value_to_instance` helper** is duplicated between `display_names.rs` and the graph walker. Lift to shared helper if a third consumer appears.

## Design constraints (don't change these)

- **`ReferenceGraph` walker is iterative** (worklist-based), not recursive. The original recursive implementation stack-overflowed on real data. Same root cause as the `Builder` before flat-pool. Don't re-introduce recursion through Value nesting in `graph.rs`.

- **`ReferenceGraph` stores `Value::Reference` edges only.** StrongPointer/WeakPointer are internal structural pointers, not cross-record references. The walker follows them to find nested References, but doesn't record them as edges.

- **Filters are pure functions over pre-extracted strings.** They don't touch svarog — callers extract fields and pass them in. Keeps filters unit-testable without DCB fixtures.
