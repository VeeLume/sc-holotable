# `sc-extract` — Phase 2a implementation notes

> **Historical record.** This document captures the initial Phase 2a implementation, back when the crate had a hand-written `FromInstance` trait and monolithic module structure. The `FromInstance` trait was superseded by the flat-pool `Extract` + `Pooled` pair (see `implementing/sc-generator.md`), and the whole entry-point surface was reworked on 2026-04-13 into staged `AssetSource::from_install` / `AssetData::extract` / `Datacore::parse` building blocks (see `docs/sc-extract.md`). The rationale for what was in Phase 2a remains useful context; the code shapes shown below no longer match the live crate.

This document records Phase 2a of the `sc-extract` implementation: the foundational layer that unblocks every subsequent phase. It's deliberately scoped to the minimum viable surface — error type, svarog re-exports, the `FromInstance` trait, and `LocaleMap` — so subsequent phases can land incrementally without blocking on the full crate.

## Status

**Complete.** First implementation of the crate. Phase 2a compiles, tests pass, clippy is clean. The rest of sc-extract (vehicle XML, reference graph, tags, manufacturers, snapshot, etc.) lands in phases 2b–2d, tracked separately.

## Scope of Phase 2a

| Area | Status |
|---|---|
| `Cargo.toml` — real deps | ✅ |
| `lib.rs` — crate root with modules + re-exports | ✅ |
| `error.rs` — unified `Error` enum | ✅ (locale variants only; placeholders for later phases) |
| svarog re-exports + `Guid` alias | ✅ |
| `FromInstance` trait | ✅ |
| `LocaleMap` parse + serialize | ✅ |
| Vehicle XML parsing | ❌ — phase 2b |
| `ReferenceGraph` | ❌ — phase 2c |
| `TagTree`, `ManufacturerRegistry`, `DisplayNameCache` | ❌ — phase 2c |
| Playable-content filters | ❌ — phase 2c |
| `ExtractedData` envelope + snapshot save/load | ❌ — phase 2d |
| `parse_from_p4k` orchestrator | ❌ — phase 2d |
| `generated/` module | ❌ — produced by `tools/sc-generator` (task 3) |

## File layout

```
crates/sc-extract/
├── Cargo.toml
└── src/
    ├── lib.rs            Crate root: docs, module declarations, re-exports
    ├── error.rs          Error enum (thiserror, #[non_exhaustive]) + Result alias
    ├── from_instance.rs  The FromInstance trait
    └── locale.rs         LocaleMap + LocKey — global.ini parse + serialize
```

Files in `src/` that will be added in later phases:

- `vehicle_xml.rs` or `vehicle_xml/` (phase 2b)
- `graph.rs` (phase 2c)
- `tags.rs` (phase 2c)
- `manufacturers.rs` (phase 2c)
- `display_names.rs` (phase 2c)
- `filters.rs` (phase 2c)
- `extracted.rs` or `snapshot.rs` (phase 2d)
- `generated/` (from `tools/sc-generator`)

## Dependencies

```toml
[dependencies]
thiserror  = { workspace = true }
serde      = { workspace = true }
serde_json = { workspace = true }
tracing    = { workspace = true }
encoding_rs = "0.8"
svarog-common   = { git = "https://github.com/19h/Svarog.git" }
svarog-datacore = { git = "https://github.com/19h/Svarog.git" }

[dev-dependencies]
tempfile = "3"
```

`encoding_rs` is a crate-local dep (only sc-extract needs UTF-16 LE decoding). If a future crate needs it too, it moves to `[workspace.dependencies]`.

### Svarog

svarog is pulled from **`https://github.com/19h/Svarog.git`** (the canonical upstream, now that the bulkhead-fixes branch was merged). Both sub-crates are declared as git deps — Cargo clones the repo once and uses both crates from the single checkout.

**Current pinned commit: `ce06ec67`** (automatically selected by Cargo on first `cargo check`, will stay in `Cargo.lock` until explicitly updated).

The `Cargo.toml` dep uses no explicit commit/tag/branch — it tracks the default branch. This is deliberately loose during early development. **Before the first real user consumes sc-holotable, this should be pinned** to an exact commit or tag via:

```toml
svarog-datacore = { git = "https://github.com/19h/Svarog.git", rev = "ce06ec67" }
```

Tracked as a follow-up.

## Module notes

### `lib.rs`

- Declares three private modules (`error`, `from_instance`, `locale`) and re-exports the public types.
- Re-exports `svarog_common` and `svarog_datacore` as full namespaces (escape hatch) in addition to cherry-picking the most-used types at the crate root (`DataCoreDatabase`, `Record`, `Instance`, `Value`, `Query`, `RecordRef`, `InstanceRef`, `ArrayRef`, `ArrayElementType`, `DataType`).
- `CigGuid` from `svarog-common` is re-exported as `Guid` for brevity.
- Module-level docs explicitly describe Phase 2a's scope and list what's planned for later phases so future-me / future-agents know the crate is deliberately partial.

### `error.rs`

- Single `Error` enum, `#[derive(thiserror::Error)]`, `#[non_exhaustive]`.
- Current variants: `LocaleIoError`, `LocaleDecodeFailed`, `LocaleMalformedLine`.
- Comment blocks reserve space for variants that'll be added in phases 2b/2c/2d (vehicle XML, DCB parse, snapshot). Left out of the current enum so the build stays warning-clean.

### `from_instance.rs`

- Single trait definition: `FromInstance::from_instance(&Instance<'_>) -> Option<Self>`.
- Doc comments make the implementor contract explicit: `None` only for fundamental type mismatch (polymorphic dispatch failure), not for missing individual fields.
- Includes a `ignore`-marked doctest showing the typical generated implementation shape, so consumers reading the docs can see what generated code will look like.
- Depends on `svarog_datacore::Instance` — that's the only svarog type the trait signature touches.

### `locale.rs`

The most substantial module in Phase 2a. Two public types:

**`LocKey(String)`** — newtype around a localization key. Has both `as_str()` (returns the raw key with any `@` prefix) and `stripped()` (returns the key without the `@` prefix). Separate type so APIs that take localization references can say so explicitly without having to document the `@`-stripping convention.

**`LocaleMap`** — backing `HashMap<String, String>`, plus methods:

- **Parsing**: `parse(&[u8])` for UTF-16 LE with BOM (the format shipped inside `Data.p4k`), `parse_utf8_bom(&[u8])` for UTF-8 with BOM (what sc-langpatch writes as override files), `parse_file(&Path)` for convenience.
- **Lookup**: `get(key)` for raw keys, `resolve(key)` that handles the `@` prefix transparently, plus `contains_key`, `len`, `is_empty`, `iter`.
- **Mutation**: `set(key, value)`, `remove(key)`.
- **Serialization**: `serialize() -> Vec<u8>` produces **UTF-8 with BOM** — deterministic output (keys sorted alphabetically), `\r\n` line endings, UTF-8 BOM prefix.

### Serialization format decision

`LocaleMap::serialize` produces **UTF-8 with BOM**, not UTF-16 LE. This is a deliberate choice matching what sc-langpatch actually writes today:

- **Star Citizen accepts both encodings** for override files at `<install>/data/Localization/<lang>/global.ini`.
- **UTF-8 is easier to inspect and diff** than UTF-16 LE.
- **sc-langpatch already uses UTF-8 + BOM**, so this is zero-surprise for the primary consumer.
- **Round-trip via `parse_utf8_bom`** gives a consistent test story.

The spec originally said UTF-16 LE on the serialize side; I updated the approach here during implementation because matching sc-langpatch was the pragmatic call. If a future consumer actually needs UTF-16 LE output (e.g., patching global.ini in-place inside a P4K archive), we'll add `serialize_utf16_le()` as a second method at that time.

### Parse is lenient about BOM

Both `parse` and `parse_utf8_bom` strip the BOM if present but don't require it. This accommodates:

- Files shipped inside `Data.p4k` (always have BOM)
- Files previously written by sc-langpatch (always have BOM)
- Files exported by other tools that may or may not include BOM
- Hand-crafted test fixtures (easier to write without the BOM)

### Parse fails on malformed lines, skips blank/comment lines

Lines beginning with `#` or `;` are treated as comments and skipped. Blank lines (after trimming) are skipped. Any other non-blank line **must** contain an `=` separator — a missing `=` is `Error::LocaleMalformedLine` with the line number. This catches truncated files and encoding mismatches (wrong encoding would usually produce lines without `=` after decoding).

Values may contain `=` characters — we use `split_once('=')` so only the **first** `=` is the separator. Keys are trimmed; values are kept verbatim.

## Tests (24 total)

| Module | Test count | What's covered |
|---|---|---|
| `locale::tests` | 24 | All parse + lookup + mutation + serialize functionality |

### Test fixtures

Two helpers build the byte buffers needed for parser tests without committing binary fixtures:

```rust
fn utf16_le_with_bom(text: &str) -> Vec<u8>   // adds 0xFF 0xFE + UTF-16 LE bytes
fn utf8_with_bom(text: &str) -> Vec<u8>       // adds 0xEF 0xBB 0xBF + UTF-8 bytes
```

### Test coverage

- **`LocKey`** (3 tests): strips `@` prefix, passes through when no prefix, `From<&str>` impl.
- **UTF-16 LE parse** (7 tests): simple key=value, no-BOM variant, blank/comment lines skipped, values containing `=`, key whitespace trimmed, empty input, malformed line error (with line number assertion).
- **UTF-8 BOM parse** (2 tests): simple parse, parse without BOM.
- **Lookup** (4 tests): `resolve` handles both `@foo` and `foo`, missing key returns `None`, `contains_key`, `iter` yields all entries.
- **Mutation** (2 tests): `set` returns previous value, `remove` returns previous value.
- **Serialization** (4 tests): produces UTF-8 BOM prefix, sorts keys alphabetically for determinism, round-trip via `parse_utf8_bom`, preserves values containing `=`.
- **File I/O** (2 tests): `parse_file` reads from disk, missing file produces `LocaleIoError`.

## Validation

All commands run from the workspace root:

| Check | Result |
|---|---|
| `cargo check -p sc-extract` | ✅ clean (first run pulled svarog git dep + 19 transitive deps) |
| `cargo test -p sc-extract` | ✅ **24/24 unit tests pass**, 2 doc-tests ignored (trait + crate-level, both `ignore`-marked by design) |
| `cargo clippy -p sc-extract --all-targets -- -D warnings` | ✅ no warnings |

## Deferred to later phases

- **Vehicle XML parser.** Phase 2b ports `sc-damage-calculator/src/extract/hull.rs` into `sc-extract/src/vehicle_xml/`.
- **Reference graph, tag tree, manufacturer registry, display-name cache, playable filters.** Phase 2c.
- **`ExtractedData` envelope, snapshot save/load, `parse_from_p4k`.** Phase 2d. This is where `FromInstance` actually gets exercised (by the generated code from `tools/sc-generator`) and where the reference graph gets built.
- **`extract_locale(p4k_path, lang)` convenience.** Phase 2c or 2d — it needs `svarog-p4k` to read the locale file out of the archive. Not blocking for 2a because `LocaleMap::parse_file` handles files already on disk.

## Known follow-ups

- **Pin svarog to an exact commit** (`rev = "ce06ec67"` or a newer SHA) before the first external consumer uses sc-holotable. Currently tracking the default branch, which is a footgun for reproducible builds across developers.
- **Doc-test for `LocaleMap`** — consider adding a runnable doc-test that constructs a small map, serializes it, and re-parses it. Not a blocker; the unit test coverage is already solid.
- **Preserving key order on round-trip** — the `HashMap` backing store doesn't preserve insertion order, and `serialize` sorts alphabetically. If any consumer ever wants to round-trip a file preserving the original key order (for minimal diffs against an existing file), we'd need to swap to an `IndexMap` or similar. No current need.
- **`@` prefix constant.** The `@` character appears as a magic prefix in `LocKey::stripped` and `LocaleMap::resolve`. Probably worth a `const LOCALE_PREFIX: char = '@';` for discoverability, but deferred until the second use site exists.
