# sc-holotable

Shared Rust utility workspace for Star Citizen tooling. Provides install
discovery, DataCore (`Game2.dcb`) extraction, generated type bindings, and
curated domain models — items, tags, manufacturers, resources, locations,
gathering, crafting, weapons, missions — that multiple consumer apps share
instead of each reimplementing their own.

## Status

Released on two independent tag axes — `sc-holotable/v*` for the library API
and `datacore/*` for game-patch binding snapshots. The workspace is grown
deliberately slowly — incorrect assumptions about Star Citizen's data formats
are easy to make and expensive to remove once consumers depend on them — so each
domain is verified against live DCB data before it is modelled.

API documentation is rustdoc (`cargo doc --open -p sc-holotable --features full`);
[`CHANGELOG.md`](CHANGELOG.md) tracks the public surface per release. There is
deliberately no `docs/` directory.

## Crates

Every crate has one of three roles, which fixes its construction signature and
error policy.

**I/O-boundary** — read the filesystem / `Data.p4k`; fallible:

| Crate | Purpose |
|---|---|
| `sc-discovery` | Discover installed SC channels (LIVE / Hotfix / PTU / EPTU / Tech Preview); resolve paths to `Data.p4k`, `global.ini`, `user.cfg`. Standalone — no svarog. |
| `sc-extract` | DCB traversal + localization, GUID / reference resolution, the generated type surface, byte-level snapshots. Re-exports svarog as an escape hatch. |

**Foundational** — GUID-keyed indices built from a parsed `RecordStore`:

| Crate | Purpose |
|---|---|
| `sc-items` | Universal item envelope (AttachDef metadata, typed type/subtype). |
| `sc-tags` | Tag tree (roots / ancestors / descendants). |
| `sc-manufacturers` | Manufacturer registry (by GUID / by code). |
| `sc-resources` | `ResourceType` catalog + refining graph + per-resource quality bridge. |
| `sc-locations` | Typed `StarMapObject` surface + universe hierarchy. |
| `sc-gathering` | Resource-gathering providers (mining / salvage / plants). |

**Domain** — models built from a parsed `Datacore` (+ `Items`):

| Crate | Purpose |
|---|---|
| `sc-crafting` | Blueprints + product-stat integrator (recipe quality model). |
| `sc-items-armor`, `sc-items-fps-weapons`, `sc-items-ship-components`, `sc-items-ship-weapons` | Per-domain base-stat sheets. |
| `sc-weapons` | Weapon + damage model. Legacy — superseded by the `sc-items-*` sheets for product stats. |
| `sc-missions` | Contracts — `Mission` / `Missions` / `MissionPools` / `Encounter`. |

`sc-holotable` is the umbrella crate: `build_foundations` fuses every
foundational index into a single record walk, and `HolotableSnapshot` persists
them for fast load. `sc-extract-generated` is workspace-internal codegen output
(consume it through `sc-extract`, never directly).

## Consumer apps

- **starlume**, **hearth** — crafting, missions and payout surfaces.
- **sc-cargo-planner** — hauling legs, mission locations, cargo grids.
- **sc-fleetsync** — install discovery + items.
- **sc-langpatch** — `global.ini` localization patcher. Drives the localization rules and `sc-missions`.
- **bulkhead** — SC combat / damage calculator. Drives the weapon maths.

## Layering

```
  domain          sc-crafting   sc-weapons   sc-missions   sc-items-*
                        \             \           /           /
  foundational    sc-items  sc-tags  sc-manufacturers  sc-resources  sc-locations  sc-gathering
                        \                      |                          /
  I/O-boundary                          sc-extract  ──►  svarog (re-exported escape hatch)

  standalone                            sc-discovery   (no domain deps, no svarog)
```

Rules the layering enforces:

- **`sc-discovery` is completely standalone.** Consumers that only need install discovery don't pay for svarog.
- **Domain and foundational crates go through `sc-extract`, never directly through svarog.** Cross-reference resolution is centralized.
- **svarog is re-exported from `sc-extract` as an escape hatch, not the preferred interface.** Prefer `sc-extract`'s own helpers; reach for raw svarog only when the abstraction doesn't cover a case yet. Reaching for it repeatedly for the same thing is a signal to lift a helper into `sc-extract`.
- **`sc-extract` deals in bytes and types, not filesystem side effects.** It reads `Data.p4k` and parses/serializes localization, but does not write patched files — that is the consumer's call (sc-langpatch, using a path helper from `sc-discovery`). This keeps `sc-extract` free of any `sc-discovery` dependency and preserves the acyclic layering.

## Design principles

1. **Go slow.** Verify game-mechanics assumptions against real data before encoding them as types. A raw layer is the safe fallback while understanding grows. Prefer "we don't model this yet" over a wrong model.
2. **Real utility lib.** Don't contort the API to a specific consumer's current needs. Consumers adapt to the lib, not the other way round. Awkwardness during integration is a signal about the consumer, not the lib.
3. **One canonical model per domain.** When two consumers need overlapping data, they share a single type — the most-demanding consumer drives correctness, others read a subset.

The full set (including "layering is on data source, not format" and "no string matching where typed alternatives exist") and the per-crate API contract live in `CLAUDE.md`.

## Conventions

- **Logging: `tracing`.** Every crate that emits logs uses `tracing` and its macros (`trace!` / `debug!` / `info!` / `warn!` / `error!`). Consuming apps install whatever subscriber they want and get unified, structured output across both the lib and the app. No `println!`, no `eprintln!`, no ad-hoc `log` usage in lib code.

  `tracing` is pinned at the workspace level in `[workspace.dependencies]`; member crates opt in with `tracing = { workspace = true }`.

## Integration

Git-dep only. Not published to crates.io. Depend on the umbrella crate and pick
capabilities with features — one pin, no svarog rev to match, no `sc-extract`
leaf flags to remember:

```toml
[dependencies]
sc-holotable = { git = "https://github.com/VeeLume/sc-holotable.git", tag = "sc-holotable/vX.Y.Z", features = ["installs", "missions"] }
```

The feature table is in the `sc-holotable` crate docs. During heavy iteration,
consumers may use a `[patch]` section to point at a local checkout of this
workspace — not a path dependency, which breaks on every other machine.
