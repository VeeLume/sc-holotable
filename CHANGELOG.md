# Changelog

Versioning is tracked in two orthogonal axes, both monotonic and immutable:

- **`sc-holotable/vX.Y.Z`** — library API releases, tracked in this file.
  Pre-1.0 convention: `0.X.0` bumps for any public-surface change (additive
  *or* breaking); `0.x.Y` bumps for bugfixes and internal-only changes.
- **`datacore/<sc_version>`** — DataCore regeneration snapshots, cut by
  `tools/regenerate.ps1 -Publish` after a Star Citizen patch. Not tracked
  here.

Consumers pin against whichever axis they care about; tags point at
separate commits and advance independently.

## [Unreleased]

### Added

- `sc_contracts::Contract` and `sc_contracts::Variation` now carry
  `title_key: Option<LocaleKey>` and `description_key: Option<LocaleKey>`
  alongside the resolved `title` / `description` strings. Consumers
  patching `global.ini` (sc-langpatch, translation-extraction tooling)
  no longer need to re-walk the contract inheritance chain to recover
  the raw INI key the displayed text was resolved from.
- `sc_contracts::ResolvedText` gained the same `title_key` /
  `description_key` fields; `resolve_contract_text` fills them during
  the existing inheritance walk at no extra cost.
- `sc_contracts::ExpandedContract` propagates the keys so
  pre-merge consumers see them too.

### Changed

- `ResolvedText` is still constructible via field-init but now requires
  four fields instead of two. Call sites that built it manually should
  use `..Default::default()` to stay forward-compatible.
