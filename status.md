# sc-holotable -- work status

> Current work state. Read `CLAUDE.md` first for workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Next session -- start here

The low-level layer (`sc-installs` + `sc-extract` + `sc-extract-generated` + `sc-generator`) is **feature-complete**. `sc-weapons` v1 + v2 phase 1 (multi-mode + charge modifiers) + v2 phase 2 (sustain derivations) are shipped. v2 phase 3 (comparison stats) is specced in `docs/sc-weapons.md` but blocked on three Phase 2 model fixes listed under "What's next".

## Last worked on

**sc-weapons v2 phase 3 shipped (2026-04-20)** -- comparison stats implemented. Renamed `alpha_dps` -> `burst_dps`. Tier 2 adds `burst_rpm`/`burst_seconds`/`volley_damage`/`recovery_seconds`/`cycle_seconds`. Tier 3 adds `dps_retention_pct(T)`/`firing_time_pct`/`long_run_dps_pct`/`thermal_efficiency`/`power_efficiency`. Single composite `effective_dps(LoadoutContext { window_seconds, power_per_slot })` for default sort. `HeatModel::fire_time_in_window(T)` and `EnergyModel::shots_in_window(rpm, T)` are the core primitives. weapon_stats example now supports `--window`/`--power`/`--sort` for ranked views. 24 tests passing. Anvil Arrow S3-slot default sort (window=30s, power=1): Predator > Revenant > Attrition-3 > Shredder > Deadbolt > M5A > Panther > Tarantula > Singe.

**sc-weapons energy model fixed (2026-04-20)** -- reverse-engineered against 6 spviewer reference values. Corrected interpretation: `max_ammo_load` = shot capacity, `max_regen_per_sec` = shots/sec, `cost_per_bullet` = ship-level cost (irrelevant to weapon cycle). Matches spviewer sustain_60s within 2% for M5A, Attrition-3, Panther, XJ3, PyroBurst. Singe S3 (charged weapon) off by ~27% due to unaccounted per-shot interval in game engine.

**sc-weapons heat model fixes (2026-04-20)** -- unified `HeatModel::heat_rate_per_second` pulled from fire_action (Rapid/Single) or `SWeaponConnectionParams.heatRateOnline` (Sequence). Added `time_to_overheat_cold`/`time_to_overheat_warm`/`duty_cycle_long_run`. 5 target weapons now report honest sustain: Deadbolt III 55%, Tarantula Mk 3 45%, Shredder 71%, Revenant ANVL S3 78%, Predator no-overheat. BEHR Revenant S6 (after_fix=99) correctly collapses to 1% sustain.

**sc-weapons v2 phase 3 designed (2026-04-20)** -- three-tier comparison-stats spec committed to `docs/sc-weapons.md`. Tier 1 directly-comparable (already exposed), Tier 2 burst stats (rpm-coupled), Tier 3 normalised (cross-rpm). Single-scalar composite `effective_dps(LoadoutContext { window_seconds, power_per_slot })` for default sort -- no defaults baked in, caller (UI slider, sc-ships) supplies runtime values. Implementation blocked on energy model fix.

**sc-weapons v2 phase 2 (2026-04-20)** -- derived sustain numbers: `HeatModel::time_to_overheat/duty_cycle`, `EnergyModel::burst_shot_count/sustained_rpm`, `ShipWeapon::alpha_dps/time_to_overheat/overheat_lockout_time/sustained_dps`. Unit tests pin GATS S1 (11.72s to overheat, 84% duty) and HRST S3 (0.89 burst shots, 10.7 sustained rpm) against `Weapons.md` reference values. Energy sustained_dps is a power-starved floor — `requested_regen_per_sec` gap flagged for sc-ships to resolve. 171/183 ship weapons produce DPS numbers (12 Charged/Burst/Unknown abstain).

**sc-weapons v2 phase 1 (2026-04-20)** -- exposed full `fire_actions: Vec<FireActionKind>` (replaces `primary_fire_action` + `fire_action_count`). Charged variant now carries `Option<ChargeModifier>` from `SWeaponActionFireChargedParams.maxChargeModifier`. Verified KLWE Atlas S10 reports `dmg×2.00, heat×2.00`; Karna alt-mode reports `dmg×1.20, burst=8`. 97/331 FPS weapons surface multi-mode; ship weapons in 4.7 are uniformly single-mode.

**sc-weapons v1 (2026-04-16)** -- materialized `ShipWeapon`/`FpsWeapon` data accessors with damage, ammo, sustain, and fire-action extraction. Pre-Phase-1 baseline.

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
| `sc-weapons` | v1 + v2 phases 1-3 shipped. Materialized accessors, multi-mode fire actions, charge modifiers, unified heat/energy model, Tier 2/3 comparison stats, `effective_dps` composite. 24 unit tests. |
| `sc-contracts` | Stub only. |

**Total: 159 tests + 4 doctests, all passing.**

## Open issues

- **`TagTree` / `ManufacturerRegistry` field names speculative.** Working (18,313 tags, 1,084 manufacturers) but individual field names unverified.
- **`Language` enum** -- only English. Add others when needed.
- **Svarog `[patch]` override is machine-local.** Remove once the `DataCoreReference::is_null` fix lands upstream.

## What's next

1. **FPS sustain models** -- Volt heat-ramp, Kastak charged, K&W heat. Weapon attachment sub-ports.
2. **Charged-weapon per-shot interval** -- Singe S3 shows my `burst_dps` is 27% higher than spviewer due to an unaccounted ~0.675s cycle component. Needs game-engine investigation or spviewer source inspection.
3. **Burst-fire sustained DPS model** -- `SWeaponActionFireBurstParams.cooldown_time` needs a burst-cycle model to produce honest rate. Currently returns `None`.
4. **Validate `TagTree` / `ManufacturerRegistry` field names** against real DCB records with typed field access.
5. **Upstream the `is_null` fix** to Svarog, bump pinned rev, drop `[patch]` override.

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
