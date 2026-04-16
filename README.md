# sc-holotable

Shared Rust utility workspace for Star Citizen tooling. Provides install discovery, game-file extraction primitives, and semantic domain models that multiple consumer apps share instead of each reimplementing their own.

## Status

Early stub. Scaffolded structure and crate-level docs only — no logic yet. This repo is being grown deliberately slowly: incorrect assumptions about Star Citizen's data formats are easy to make and expensive to remove once consumers depend on them.

## Crates

| Crate | Purpose | Depends on |
|---|---|---|
| `sc-installs`  | Discover installed SC channels (LIVE / Hotfix / PTU / EPTU / Tech Preview), resolve paths to `Data.p4k`, `global.ini`, `user.cfg`, etc. Reads the RSI launcher log and `build_manifest.id`. | nothing domain-ish |
| `sc-extract`   | svarog adapters, DCB traversal, GUID / tag / cross-reference resolution, foundational types (`Guid`, `Reference`, `Tag`), and localization (`global.ini` parse + serialize, `LocaleMap`, `LocKey`, convenience extractor). Re-exports svarog as an escape hatch. | svarog |
| `sc-weapons`   | Canonical weapon + damage model. `bulkhead` drives correctness; `sc-langpatch` consumes a subset. | `sc-extract` |
| `sc-contracts` | Raw layer (generators, objectives, references) and a model layer that grows iteratively as the generator system is understood. | `sc-extract` |

## Consumer apps

- **bulkhead** — SC combat / damage calculator. Primary driver of `sc-weapons`.
- **sc-langpatch** — `global.ini` localization patcher. Primary driver of `sc-contracts`.
- **streamdeck-starcitizen** — Stream Deck keybind plugin. Consumes only `sc-installs`.

## Layering

```
       sc-weapons      sc-contracts
             \             /
              \           /
               sc-extract  ──►  svarog (re-exported as escape hatch)
                /
       sc-installs  (standalone — no domain deps, no svarog)
```

Rules the layering enforces:

- **`sc-installs` is completely standalone.** Consumers that only need install discovery don't pay for svarog. `streamdeck-starcitizen` relies on this.
- **Domain crates go through `sc-extract`, never directly through svarog.** Cross-reference resolution is centralized. When a weapon references an ammo record, or a contract references a reward table, `sc-extract` handles the chase once.
- **svarog is re-exported from `sc-extract` as an escape hatch, not the preferred interface.** Apps should prefer `sc-extract`'s own helpers when they exist, and only reach for raw svarog when the abstraction doesn't cover their case yet. That "yet" is load-bearing — reaching for raw svarog repeatedly for the same thing is a signal to lift the helper into `sc-extract`.
- **`sc-extract` deals in bytes and types, not filesystem side effects.** It reads `Data.p4k` and parses/serializes localization content, but it does not write patched files to the install override path — that is the consumer's call (sc-langpatch, in practice), using a path helper from `sc-installs`. This keeps `sc-extract` free of any `sc-installs` dependency and preserves the acyclic layering.

## Design principles

1. **Go slow.** Verify game-mechanics assumptions against real data before encoding them as types. A raw layer is the safe fallback while understanding grows. Prefer "we don't model this yet" over a wrong model.
2. **Real utility lib.** Don't contort the API to match a specific consumer's current needs. Consumers adapt to the lib, not the other way round. Awkwardness during integration is a signal about the consumer, not the lib.
3. **One canonical model per domain.** When two consumers need overlapping data, they share a single type — the most-demanding consumer drives correctness, others read a subset.

## Conventions

- **Logging: `tracing`.** Every crate in the workspace that emits logs uses the `tracing` crate and its macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`). Consuming apps install whatever subscriber they want — `tracing-subscriber` for CLI / desktop apps, the Tauri logger for sc-langpatch, the existing subscriber in streamdeck-starcitizen — and get unified, structured output across both the lib and the app. No `println!`, no `eprintln!`, and no ad-hoc `log` crate usage in lib code.

  `tracing` is pinned at the workspace level in `[workspace.dependencies]`; member crates opt in with `tracing = { workspace = true }` in their own `Cargo.toml`.

## Integration

Git-dep only. Not published to crates.io.

```toml
[dependencies]
sc-installs = { git = "https://github.com/<user>/sc-holotable.git", tag = "vX.Y.Z" }
```

During heavy iteration — especially while the `sc-contracts` model layer is being discovered — consumers may use a `[patch]` section to point at a local checkout of this workspace.
