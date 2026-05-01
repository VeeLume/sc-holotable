# sc-holotable -- work status

> Current work state. Read `CLAUDE.md` first for workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Next session -- start here

The whole workspace is feature-complete for v2. `sc-installs`,
`sc-extract`, `sc-extract-generated`, `sc-generator`, `sc-weapons`
(v1 + v2 phases 1-3), and `sc-contracts` (v2 all 7 phases + resolver
fix) are all shipped and on green tests. `sc-explorer` is the live
TUI for browsing the data; pair it with the `cargo run … --example
<dig>` investigation tools when something looks off.

## Last worked on

**`ShipRegistry::resolve_spawn` — three-bug fix (2026-05-01)** — `93cf1cb`. Salvage-target spawns reported `candidates=0` despite working in-game. Three coupled bugs identified via `salvage_pool` + `resolve_debug` digs:

1. **Tag-tree subsumption missing.** Avenger Titan Renegade carries the leaf `AvailableToSalvage > Small`; the slot demands the bare parent `AvailableToSalvage`. Strict GUID equality fails. Fix: precompute `tags_expanded` per entity at build time (literal tags + all ancestors) and match against that.
2. **Spawn-state tags treated as identity filters.** 10 of the slot's 12 positive tags live under `AI > Ship > SpawnFlags > *` or `AI > CargoManifest > *` — runtime state, not entity selection. They have non-zero carriers among non-salvage civilians, so the existing zero-carrier filter doesn't drop them. Fix: classify spawn-state subtrees at build, drop them from positive-tag matching.
3. **`has_ship_intent` over-fired.** The tag tree has *two* `Ship` nodes (canonical model root + intermediate node under `AI`). The intent gate caught `AI > Ship > SpawnFlags` tags as ship intent, then the post-state-filter set had no surviving Ship tag, returning empty early. Fix: compute intent on the state-filtered set.

Live recovery on SC 4.7: empty ship-encounter slots **335 → 255 (24% fewer)**. Adagio Lawful Salvage VeryEasy: **0 → 21 candidates** (Avenger Titan Renegade, Eclipse, Sabre, Terrapin, Origin 100i, San'tok.yāi, …). 91 `*_Unmanned_Salvage` ECDs now resolvable. `encounter_analytics` fingerprints structurally unchanged — fix recovers candidates without shifting tag distributions.

The remaining 255 still-empty slots are now likely real CIG data bugs (e.g., Foxwell Pyro `AlliedSpawnDescriptions_BP` querying `UEE_Navy` faction in a system UEE Navy doesn't operate in — confirmed via `ambush_dig`).

**sc-contracts v2 redesign — all 7 phases shipped (2026-04-30 → 2026-05-01)** — `57115c9` … `9513c4d`. Mission-centric API replacing the v1 generator-shaped surface. See `docs/sc-contracts-v2.md` for the full design.

| Phase | Commit | Headline |
|---|---|---|
| 1 | `a00a0d1` | TagBag symmetry on `EncounterSlot` (positive / negative / markup / entity) |
| 2 | `d2668a3` | Six reward fields collapsed into `MissionRewards` |
| 3 | `4d74d8f` | `MissionPools` precomputed on index + opt-in divergence helpers |
| 4 | `c636f51` | Implicit merge step removed; `Variation` / `title_siblings` / `find_bp_conflicts` deleted |
| 5 | `b271e82` | `Contract → Mission`, `ContractIndex → MissionIndex`, `MissionOrigin` consolidates handler fields |
| 6 | `9513c4d` | `Encounter` enum: ship + npc + entity widening, `mission_allied_marker` surfaced |
| 7 | `9513c4d` | `EncounterWave → EncounterPhase`, generic `EncounterPhase<S>` |

Live shape on SC 4.7: 4,590 missions, 11,599 ship-encounter slots + 3,749 NPC slots + 228 entity slots. NPC slots expose `mission_allied_marker` (48 slots), `is_critical` (0), `faction_override` (590).

**Investigation tooling (2026-05-01)** — committed as standalone examples in `crates/sc-contracts/examples/`:
- `contract_dump <substring>` — full per-mission dump (origin, rewards, prereqs, encounters with all four tag bags + tag-tree paths).
- `ambush_dig` — Foxwell ambush mission family deep dive (Locality resolution, AlliedSpawn pool census).
- `damage_dig` — every `initial_damage_settings` GUID resolved against the live DCB.
- `salvage_pool` — find every `*_Unmanned_Salvage` ECD; cross-reference with slot tag demands.
- `spawn_dig` — three-section: `name_property` distribution, F7C_Hornet broken-tag trace, empty-candidate census.
- `encounter_analytics` — six analytics sections on the encounter graph; baseline metrics for regression detection.
- `encounter_kinds` — variant census + NPC signal coverage.
- `role_investigation` / `tier_investigation` — earlier feature-request audit scripts.

These tools are kept committed; they're the canonical way to run a quick dig against a fresh DCB regen.

## Implementation status

| Crate / Tool | Status |
|---|---|
| `sc-installs` | Complete. 51 tests + 1 doctest. |
| `sc-extract` | Complete (2a + 2c + 2d). 94 tests + 3 doctests. |
| `sc-extract-generated` | Complete. Flat-pool, 4-module split, typed enums/locale, poly enums. |
| `sc-generator` | Complete. Codegen + feature classification + Cargo.toml generation. |
| `sc-bench` | Complete. Runtime benchmark binary. |
| `sc-ammo` | Spec only (`docs/sc-ammo.md`), no crate. |
| `sc-weapons` | v1 + v2 phases 1-3 shipped. 24 unit tests. |
| `sc-contracts` | **v2 complete (all 7 phases + resolver fix). 22 lib tests.** |
| `sc-explorer` | Tools binary — three tabs (Pools / Contracts / Weapons). Per-crate `tui` modules own their domain views. |

**Total: 191 tests + 4 doctests, all passing.**

## Open issues

- **`TagTree` / `ManufacturerRegistry` field names speculative.** Working (18,313 tags, 1,084 manufacturers) but individual field names unverified.
- **`Language` enum** -- only English. Add others when needed.
- **Svarog `[patch]` override is machine-local.** Remove once the `DataCoreReference::is_null` fix lands upstream.
- **255 ship-encounter slots still resolve to candidates=0** after the resolver fix — spawn_dig §3 lists them grouped by mission family. Top offenders include Foxwell / HeadHunters Stanton variants where the slot tags genuinely don't match any ECD (likely real CIG data bugs).

## What's next (open work, no current driver)

- **sc-langpatch migration to v2.** The langpatch feature-request docs (`docs/feature-request-sc-contracts-langpatch.md`, `-sc-weapons-langpatch.md`) were written against v1; sections need re-anchoring on v2 type names.
- **NPC encounter FPS-detail.** Closet/room/defendArea tag scopes on `AutoSpawnSettings` still dropped at extraction; v3 follow-up when an FPS consumer needs them.
- **`SpawnDescription_Ship.name_property` BP_SDs chains.** ~57 instances point to other `MissionPropertyValue_ShipSpawnDescriptions` for slot-chaining; not currently surfaced.
- **FPS sustain models** -- Volt heat-ramp, Kastak charged, K&W heat. Weapon attachment sub-ports.
- **Charged-weapon per-shot interval** -- Singe S3 shows my `burst_dps` is 27% higher than spviewer due to an unaccounted ~0.675s cycle component.
- **Burst-fire sustained DPS model** -- `SWeaponActionFireBurstParams.cooldown_time` needs a burst-cycle model to produce honest rate.
- **Validate `TagTree` / `ManufacturerRegistry` field names** against real DCB records.
- **Upstream the `is_null` fix** to Svarog, bump pinned rev, drop `[patch]` override.

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
| Missions on SC 4.7 LIVE | 4,590 |
| Ship-encounter slots / NPC slots / Entity slots | 11,599 / 3,749 / 228 |
| Empty ship-encounter slots after resolver fix | 255 (2.2%) |

## Fresh-session checklist

1. Read `CLAUDE.md` for orientation and gotchas.
2. Read this file for current work state.
3. `cargo check -p sc-contracts --all-targets --features tui` to verify warm build.
4. Smoke test: `cargo run -p sc-explorer --release` (interactive TUI).
5. Quick analytics: `cargo run -p sc-contracts --release --example encounter_analytics`.
