# sc-holotable -- work status

> Current work state. Read `CLAUDE.md` first for workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Next session -- start here

The low-level layer (`sc-installs` + `sc-extract` + `sc-extract-generated` + `sc-generator`) is **feature-complete**. Release profiles are established. The next milestone is `sc-weapons` -- the first real domain crate.

## Last worked on

**Documentation cleanup (2026-04-16)** -- compacted implementing/ docs (5 files -> 2), replaced duplicated datacore spec with Obsidian vault redirect, updated codegen spec to match current state, trimmed iteration logs from benchmarks. Obsidian vault `sc-holotable.md` updated.

**Release profiles established (2026-04-16)** -- per-dimension sweeps across LTO modes, opt-levels, codegen-units, and panic strategies. Two profiles locked in:

| Profile | LTO | panic | gen opt | Use case |
|---|---|---|---|---|
| `release` | thin | unwind | 1 | Day-to-day builds |
| `release-max` | fat | abort | 1 | Shipped binaries / CI |

See `docs/benchmarks.md` for sweep findings and current numbers.

**Prior milestones** (2026-04-12 -> 2026-04-16):
- Generator derive-drop (-85% cold check, -60% cold build)
- Snapshot v5 byte-bundle rework (eliminated serde monomorphization cliff)
- Feature-gating v2: data-driven scoping, core/dormant/multi_feature split, 245 leaf features
- Typed enums + `LocaleKey` newtype + poly enums
- API rework to staged entry points (`AssetSource` -> `AssetData` -> `Datacore`)
- Iterative reference graph walker + flat-pool `Builder`
- svarog pinned to `ce06ec67`

## Implementation status

| Crate / Tool | Status |
|---|---|
| `sc-installs` | Complete. 51 tests + 1 doctest. |
| `sc-extract` | Complete (2a + 2c + 2d). 91 tests + 3 doctests. |
| `sc-extract-generated` | Complete. Flat-pool, 4-module split, typed enums/locale, poly enums. |
| `sc-generator` | Complete. Codegen + feature classification + Cargo.toml generation. |
| `sc-bench` | Complete. Runtime benchmark binary. |
| `sc-ammo` | Spec only (`docs/sc-ammo.md`), no crate. |
| `sc-weapons` | Stub only. |
| `sc-contracts` | Stub only. |

**Total: 145 tests + 4 doctests, all passing.**

## Open issues

- **`DisplayNameCache` returns 0 entries.** `global.ini` not found in `Data.p4k` -- the path pattern may have changed. Needs investigation.
- **`TagTree` / `ManufacturerRegistry` field names speculative.** Working (18,313 tags, 1,084 manufacturers) but individual field names unverified.
- **`Language` enum** -- only English. Add others when needed.
- **Svarog `[patch]` override is machine-local.** Remove once the `DataCoreReference::is_null` fix lands upstream.

## What's next

1. **Start `sc-weapons`** -- first real domain crate. Tests whether typed-enums, LocaleKey, poly-enum, and feature-gating hold up under a real consumer.
2. **Investigate locale path** -- find where `global.ini` lives in the current `Data.p4k`; fix `DisplayNameCache` + `LocaleMap` loading.
3. **Validate `TagTree` / `ManufacturerRegistry` field names** against real DCB records with typed field access.
4. **Upstream the `is_null` fix** to Svarog, bump pinned rev, drop `[patch]` override.

## Numbers

| Metric | Value |
|---|---|
| Emitted struct types | 6,234 (pruned from 6,545) |
| Emitted enum types | 1,252 |
| `core/` / `multi_feature/` / `dormant/` / leaf | 336 / 3,789 / 809 / ~1,300 |
| Leaf feature directories | 245 |
| Generator run time | ~3 seconds (release) |
| Cold check (ships) | ~18s |
| Cold release build (ships) | ~1m 44s |
| Runtime parse (ships, standard) | ~27s |
| svarog commit | `ce06ec67` + local `[patch]` |

## Fresh-session checklist

1. Read `CLAUDE.md` for orientation and gotchas.
2. Read this file for current work state.
3. `cargo check -p sc-extract` to verify warm build.
4. Smoke test: `cargo run -p sc-extract --release --example parse_real_p4k`
5. Full: `cargo run -p sc-extract --release --features full --example parse_real_p4k`
