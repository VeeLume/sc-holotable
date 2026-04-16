# `sc-installs` — implementation notes

**Status**: Complete. 51 tests + 1 doctest. No planned changes.

## Port lineage

Ported primarily from **streamdeck-starcitizen's `src/discovery.rs`** (494 lines, the most complete prior implementation). Two additions from **sc-langpatch's `src-tauri/src/discovery.rs`**:

- Strict validation — require `Data.p4k` to exist, not just the install directory.
- Localization override path — `Installation::localization_override(Language)`.

The launcher-visible version format (`4.7.1-live.11592622`) is also from sc-langpatch.

## Consumer switch-over plan

1. **streamdeck-starcitizen** switches first. Its `src/discovery.rs` becomes a thin wrapper around `sc_installs::discover`. Strongest validation since it's the source of the reference implementation.
2. **sc-langpatch** follows. Its `src-tauri/src/discovery.rs` becomes an import with `specta` feature enabled.
3. **bulkhead** uses `sc_installs::discover_last_launched` directly (greenfield).

## Non-obvious decisions

- **Single regex for both launcher log formats** (legacy plain-text + JSON v2.x). Uses substring extraction, not format-specific parsers. Handles mixed-format logs from launcher upgrades.
- **`Language` enum is English-only, `#[non_exhaustive]`.** SC uses regional folder variants (`chinese_(simplified)`) that aren't safe to guess. Each language must be verified against a real install before adding.
- **`specta = "=2.0.0-rc.21"`** exact pin — `^2` resolves to `rc.24` which requires nightly. Matches sc-langpatch's pin.
