# `sc-extract` — Phase 2c implementation notes

> **Historical record.** Phase 2c shipped these modules inside the pre-rework `ExtractedData` god-struct with `from_database(&db, ...)` construction called from `parse_inner`. The 2026-04-13 rework moved the same `from_database` builders under `Datacore::parse` and split the results across `DatacoreSnapshot` (records, graph, tag_tree, manufacturers, display_names) and `AssetData` (locale). The individual module internals (`graph.rs`, `tags.rs`, `manufacturers.rs`, `display_names.rs`, `locale.rs`, `filters.rs`) were *not* changed by the rework and still match this doc. See `docs/sc-extract.md` for the current plumbing on top.

This document records Phase 2c of the `sc-extract` implementation: the cross-cutting services (reference graph, tag tree, manufacturer registry, display-name cache, playable-content filters). Phase 2b (vehicle XML) was deliberately skipped per user decision — that part of the design is less settled and will be revisited when a consumer actually needs it.

## Status

**Complete.** Phase 2c lands five new modules on top of Phase 2a's foundation. The data structures are fully implemented and unit-tested. The `from_database` construction paths compile against svarog but have not been integration-tested against a real `Data.p4k` yet — that's what Phase 2d / Task 4 (end-to-end wire-up) will verify.

## Scope

| Module | Purpose | Status |
|---|---|---|
| `graph.rs` | `ReferenceGraph` — forward + reverse lookup of cross-record references | ✅ |
| `tags.rs` | `TagTree` — hierarchical navigation over DCB tag records | ✅ |
| `manufacturers.rs` | `ManufacturerRegistry` — flat lookup of manufacturer records | ✅ |
| `display_names.rs` | `DisplayNameCache` — pre-computed entity display names | ✅ |
| `filters.rs` | Playable-content predicates (pure functions) | ✅ |

**Not in Phase 2c:**

- `parse_from_p4k` orchestrator — Phase 2d
- `ExtractedData` envelope + snapshot save/load — Phase 2d
- Vehicle XML parser — deferred indefinitely (Phase 2b skipped)
- Generated types module — produced by `sc-generator` (Task 3)

## File layout additions

```
crates/sc-extract/src/
├── error.rs              (unchanged from Phase 2a)
├── from_instance.rs      (unchanged from Phase 2a)
├── locale.rs             (unchanged from Phase 2a)
├── graph.rs              NEW — ReferenceGraph + construction
├── tags.rs               NEW — TagTree + TagNode
├── manufacturers.rs      NEW — ManufacturerRegistry + Manufacturer
├── display_names.rs      NEW — DisplayNameCache + resolver helper
└── filters.rs            NEW — is_playable_weapon, is_playable_ship
```

Plus `lib.rs` updates to declare the new modules and re-export their public types.

## Module notes

### `graph.rs` — ReferenceGraph

The reference graph stores `Value::Reference` edges only. `StrongPointer` and `WeakPointer` are deliberately **not** tracked as edges — they're internal structural pointers into the struct instance pool, not cross-cutting references consumers would reverse-lookup. The walker still *recurses* through them (and through `Class`, `ClassRef`, and arrays) to find nested `Reference` values — for example, a weapon's `Components` array contains a `SCItemWeaponComponentParams` class whose `ammoContainer.ammoParamsRecord` is a GUID reference; that nested reference gets recorded as an edge from the weapon to the ammo record.

Key API:

- `ReferenceGraph::new()` — empty
- `insert_edge(Edge)` — public so callers can build graphs incrementally; maintains forward + reverse indices atomically
- `outgoing(&Guid) -> &[Edge]` — forward lookup
- `incoming(&Guid) -> &[Guid]` — reverse lookup (answers "who references this?")
- `all_edges()`, `edge_count()`, `source_count()`, `target_count()`, `is_empty()`
- `ReferenceGraph::from_database(&DataCoreDatabase)` — builds by walking every record

The walker has two mutually recursive helpers:

- `collect_refs_from_instance` — iterates `inst.properties()` and handles each `Value` variant
- `collect_refs_from_value` — processes a single value (used for array elements, which come from `inst.get_array(name)` and arrive as `Value`s rather than properties)

The split exists because `Instance::properties()` yields properties with their name, whereas array iteration via `get_array(name)` yields bare `Value`s without property names. Both paths handle the same set of variants but access them differently.

**Edge path format:** `"Components[2].ammoContainer.ammoParamsRecord"` — dotted names for nested fields, `[index]` suffixes for array elements. No leading separator at the root of a record.

### `tags.rs` — TagTree

Hand-written navigation over the DCB's tag records. `TagNode` fields: `guid`, `name`, `parent: Option<Guid>`, `children: Vec<Guid>`, `legacy_guid: Option<i32>`. The tree stores a `HashMap<Guid, TagNode>` plus a `HashMap<String, Vec<Guid>>` for name lookups (name collisions exist in the real DCB).

Key navigation API:

- `get(&Guid) -> Option<&TagNode>` — lookup by GUID
- `by_name(&str) -> &[Guid]` — returns multiple matches due to name collisions
- `roots()` — iterator over top-level nodes (no parent)
- `ancestors(&Guid)` — walk upward to root, excluding the start node
- `descendants(&Guid)` — depth-first walk downward, excluding the start node
- `is_descendant_of(&Guid, ancestor: &Guid) -> bool` — transitive check
- `path(&Guid) -> Vec<&str>` — dotted path from root, e.g. `["Global", "Manufacturer", "Aegis"]`

Descendants iteration uses an explicit stack (no recursion) because depth-first with a generator pattern is cleaner and handles deep trees without stack overflow.

**Caveat about `from_database`:** this implementation assumes Tag records have `tagName` (string), `parent` (Reference), `children` (array of References), and `legacyGUID` (integer) fields. The exact DCB field names haven't been verified against real data yet — sc-langpatch's `build_tag_name_map` only reads `tagName`, so the parent/children handling is speculative. If integration testing reveals a different structure (e.g., tags are inline instances nested under a TagDatabase record rather than top-level records), this function will need adjustment. Documented as a follow-up.

### `manufacturers.rs` — ManufacturerRegistry

Simpler than TagTree. `Manufacturer` fields: `guid`, `code`, `name_key: Option<String>`, `description_key: Option<String>`. Two indices: `HashMap<Guid, Manufacturer>` and `HashMap<String, Guid>` (code → guid).

The `code` is derived from the record name by stripping the `"SCItemManufacturer."` type prefix, so `"SCItemManufacturer.GATS"` becomes `"GATS"`. If the prefix is absent for whatever reason, the full name is used as-is.

`name_key` and `description_key` are extracted from the `Localization` nested instance on each manufacturer record. These are the raw `@`-prefixed keys (e.g. `"@manufacturer_NameGATS"`) that consumers pass to `LocaleMap::resolve()` to get the display name.

### `display_names.rs` — DisplayNameCache

The most complex walker in Phase 2c. Resolves an entity's display name by chasing:

```
EntityClassDefinition.Components
  → each element (Class/StrongPointer)
  → match on type_name == "SAttachableComponentParams"
  → AttachDef (nested instance)
  → Localization (nested instance)
  → Name (string, @-prefixed locale key)
  → strip @ → lookup in LocaleMap
```

Stored as `HashMap<Guid, String>`. Constructed during parse, serialized in the snapshot, consumed lazily by UI layers.

**Design detail: `resolve_entity_display_name` takes an explicit `&DataCoreDatabase`** — not just an `Instance`. This is because svarog's `Instance` doesn't expose its internal database reference publicly, so there's no way to materialize array elements into `Instance` values without the `db` in hand. `DisplayNameCache::from_database` already has the db, so it just passes it through. A consumer resolving a single entity's name outside the cache path also has to pass the db. Minor ergonomic cost; documented in the function signature.

The helper `value_to_instance(&Value, &DataCoreDatabase)` converts a `Value::Class`, `Value::ClassRef`, `Value::StrongPointer(Some)`, or `Value::WeakPointer(Some)` into an `Instance<'_>`. This pattern appears in both `display_names.rs` and (informally) in `graph.rs`; if a third consumer needs it, it can be lifted into a shared helper.

### `filters.rs` — Playable-content predicates

**Pure functions over pre-extracted strings.** The filter logic doesn't touch svarog at all — callers extract the relevant fields via `inst.get_str("AttachDef.Type")` etc. and pass them in. This trade keeps the filters fully unit-testable without needing a real DCB fixture.

Two public functions:

- `is_playable_weapon(type_name, sub_type, display_name, class_name, size)` — enforces the weapon rules from bulkhead's `docs/damage-system.md`
- `is_playable_ship(class_name, display_name, has_vehicle_component)` — enforces the ship rules

Private helpers:

- `has_valid_display_name(Option<&str>)` — common check (non-empty, not `"placeholder"`)
- `has_internal_class_pattern(&str)` — matches `_LowPoly`, `_Dummy`, `_FakeHologram`, `vlk_` prefix

More filter functions (shields, armor, radars, etc.) land as those item categories become in scope.

## Tests

**61 new unit tests** in Phase 2c, across all five modules. Total sc-extract test count is now **85** (24 from Phase 2a + 61 new).

| Module | Test count | Coverage |
|---|---|---|
| `graph` | 9 | Empty graph, single edge insertion, outgoing + incoming lookup, multiple sources per target, single source per multiple targets, missing-guid lookup, all_edges iteration, serde round-trip |
| `tags` | 15 | Empty tree, insert + lookup by guid, lookup by name, name collisions, replace-insert updates name index, roots, ancestors walks to root, ancestors for root is empty, descendants walks subtree, descendants for leaf is empty, is_descendant_of, path from root to leaf, path for missing tag, iter yields all, serde round-trip |
| `manufacturers` | 7 | Empty registry, insert + guid lookup, code lookup, replace-insert updates code index, iter all, name/description keys stored, serde round-trip |
| `display_names` | 5 | Empty cache, insert + get, replace-insert, iter, serde round-trip |
| `filters` | 25 | Weapon: each type/subtype combo, missing/empty/placeholder display name, size zero, lowpoly/dummy/fakehologram/vlk_ patterns, size None is allowed. Ship: with/without vehicle component, missing display name, AI/Unmanned/Template variants, debris/wreck, BIS/Showdown event variants. |

### Test fixture helper

Each module that uses `Guid` for tests has a helper:

```rust
fn g(byte: u8) -> Guid {
    CigGuid::from_bytes([byte; 16])
}
```

Distinct GUIDs by filling all 16 bytes with the same test value. Not realistic (real GUIDs have mixed bytes) but sufficient for hash/equality tests.

For tags, a `build_sample_tree()` helper constructs a small tree (Global → Manufacturer → Aegis/Anvil, Global → Race → Human) that's reused across 8 navigation tests.

## Dependency additions

The `svarog-common` dep in `sc-extract/Cargo.toml` now enables its `serde` feature:

```toml
svarog-common = { git = "https://github.com/19h/Svarog.git", features = ["serde"] }
```

Without this, `CigGuid` doesn't implement `serde::Serialize` / `Deserialize` and every struct in Phase 2c that derives `Serialize` or `Deserialize` (all of them) fails to compile. This was a one-line fix once the error messages pointed at it.

## Validation

| Check | Result |
|---|---|
| `cargo check -p sc-extract` | ✅ clean |
| `cargo test -p sc-extract` | ✅ **85/85 unit tests pass** (24 Phase 2a + 61 Phase 2c), 2 doc-tests intentionally `ignore`-marked |
| `cargo clippy -p sc-extract --all-targets -- -D warnings` | ✅ no warnings |

## `from_database` methods are compile-verified, not run-verified

Important caveat: each module exposes a `from_database(db: &DataCoreDatabase) -> Self` method, and these compile cleanly against svarog's current API (`ce06ec67` commit). They have **not** been run against a real `Data.p4k` yet — that happens during Task 4 (end-to-end wire-up). The unit tests only exercise the data-structure operations (insert, lookup, navigation, iteration, serde).

Known risks in `from_database` implementations that integration testing will either confirm or break:

- **`TagTree::from_database`** assumes tags have `parent` / `children` fields. If the DCB structure is different (e.g., tags are inline instances nested under a `TagDatabase` record), the tree will come out empty or flat. sc-langpatch's code only reads `tagName`, so there's no precedent to crib from.
- **`ManufacturerRegistry::from_database`** assumes manufacturer records have a `Localization` instance with `Name` / `Description` string fields. If the actual field names are different (`DisplayName`, `Loc.Name`, etc.), the name/description keys will come out as `None`.
- **`DisplayNameCache::from_database`** assumes entity records have a `Components` array containing `SAttachableComponentParams` instances with nested `AttachDef → Localization → Name` chains. This matches bulkhead's documented DCB structure but hasn't been verified end-to-end here.

Each of these uses `records_by_type` (exact match) which avoids the substring-match footgun sc-langpatch runs into.

## Deferred to Phase 2d

- **`ExtractedData` envelope** that bundles `ReferenceGraph`, `TagTree`, `ManufacturerRegistry`, `DisplayNameCache`, `LocaleMap`, and (eventually) the generated record store
- **`parse_from_p4k` orchestrator** that opens a P4K, parses the DCB, extracts the locale from `Data/Localization/english/global.ini` via svarog-p4k, runs each `from_database` constructor, and returns a populated `ExtractedData`
- **`save_snapshot` / `load_snapshot`** over msgpack + zstd
- **Schema version checking** on load

Phase 2d is blocked on Task 3 (sc-generator) producing the generated record store — the envelope has a `records: RecordStore` field whose type is generated.

## Known follow-ups

- **Integration-test `from_database` methods** against a real `Data.p4k`. Until then, the builders are educated guesses based on sc-langpatch's patterns and bulkhead's docs.
- **Tag field names.** Verify `tagName` / `parent` / `children` / `legacyGUID` against the real DCB. If the structure is different, `TagTree::from_database` may need a rewrite.
- **Manufacturer `Localization` nesting.** Verify that `record.get_instance("Localization")` returns a valid instance — the XML I saw has `<Localization Type="SCItemLocalization">` which suggests the field is a typed class, which `get_instance` should handle.
- **The `value_to_instance` helper** is duplicated between `display_names.rs` and the graph walker. If a third consumer needs it, lift it into a shared helper (probably in a private `walker.rs` module).
- **Pin the svarog git dep** to an exact commit SHA (still tracking main branch). Same follow-up as Phase 2a.
