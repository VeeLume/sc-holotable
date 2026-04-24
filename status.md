# sc-holotable -- work status

> Current work state. Read `CLAUDE.md` first for workspace orientation;
> this file is the always-current "where we left off" snapshot.

## Next session -- start here

The low-level layer (`sc-installs` + `sc-extract` + `sc-extract-generated` + `sc-generator`) is **feature-complete**. `sc-weapons` v1 + v2 phase 1 (multi-mode + charge modifiers) + v2 phase 2 (sustain derivations) are shipped. v2 phase 3 (comparison stats) is specced in `docs/sc-weapons.md` but blocked on three Phase 2 model fixes listed under "What's next".

## Last worked on

**sc-contracts ContractIndex + consumer guide landed (2026-04-24)** — `src/index.rs` + `docs/sc-contracts-guide.md`. [`ContractIndex::build(datacore, locale)`] bundles the merged contract list plus every registry (ships, blueprints, currency, locations, localities) into a single `Clone + Debug` data struct. Accessors: `get(id)` (O(1) by canonical GUID), `get_by_variation(id)` (O(n) — matches sub-contract GUIDs too), `iter()`, `len()`, `is_empty()`. Fields are public by convention since it's a plain bundle, not an opaque handle. The `bp_title_conflicts` example is refactored to use the `ContractIndex` + `find_bp_conflicts` pattern — drops ~80 lines of hand-rolled grouping logic, now serves as the canonical consumer pattern for sc-langpatch migration. [`docs/sc-contracts-guide.md`] is the "how do I use this" companion to the `sc-contracts.md` design spec: quick-start, Cargo setup, Contract model field-by-field, the 7 main consumer workflows (title rendering with runtime-substitution awareness, BP annotation with conflict awareness, mission-span rendering, ship encounter display, cooldown formatting, SCMDB matching), escape hatches, gotchas, performance / version compatibility notes. 20 unit tests + 1 doctest pass.

**sc-contracts mission-span (locality) resolution landed (2026-04-24)** — `src/locality.rs`. Graduated the deferred v2 mission-span design into first-class support: [`LocationRegistry`] walks every `StarMapObject`, iteratively resolves `parent` chains to classify each location into a `SystemKey { Stanton, Pyro, Nyx, Other }` via root-name matching (`PyroStar` / `NyxSolarSystem` / bare `Stanton` all normalise correctly — stripping `Star` / `SolarSystem` suffixes). Body-level ancestor surfaces as `LocationRef.body` when the chain has an intermediate node between location and root. Every `StarMapObject.name` LocaleKey is resolved into `LocationRef.display_name` + `body_display_name` — the player-facing body labels (`Sirus`, `Hurston`, `Bloom`, `Terminus`, `New Babbage`). [`LocalityRegistry`] walks every `MissionLocality`, resolves `available_locations` through `LocationRegistry`, and precomputes `LocalityView.region_label: String` — a one-line human-readable summary like `"Pyro: Bloom, Rat's Nest, Starlight Service Station"` (capped at 5 bodies + `+N more`) or `"Stanton + Pyro"` for cross-system spans. `ExpandedContract.mission_span: Vec<Guid>` / `Contract.mission_span: Vec<Guid>` are GUID-valued (kept cheap; consumers look up via `LocalityRegistry::get`). On SC 4.7 LIVE: 805 contracts (49%) have mission_span, 2,451 locality references total, 359 contracts cross multiple systems. `bp_title_conflicts` now prints the resolved spans — the Pyro Region A/B vs C/D blueprint split is self-explaining: `_AB` pool = `span: RegionA (Pyro: Checkmate, Patch City); RegionB (Pyro: Bloom, Rat's Nest, Starlight Service Station)` (inner Pyro) vs `_CD` pool = `RegionC (Pyro: Gaslight, Pyro V); RegionD (Pyro: Terminus)` (outer Pyro). Stanton-vs-Pyro Shubin split shows `Stanton1 (Stanton: Hurston); Stanton2 (Stanton: Crusader); Stanton3 (Stanton: ArcCorp); Stanton4 (Stanton: microTech)` — every planet named. Data-driven: parent-chain traversal + typed `name` LocaleKey lookup, no geographic heuristics (workspace rule §5). 19 tests (+4 region_label). Clippy clean on sc-contracts.

**sc-contracts ShipRegistry landed + intent-based resolver (2026-04-24)** — `src/ships.rs` in sc-contracts. Data-derived two-pass construction: collect `Tag` GUIDs from every `MissionPropertyValue_ShipSpawnDescriptions` spawn query across the contract graph, then include every `EntityClassDefinition` whose tag set intersects. Against SC 4.7 LIVE: 3,218 entities, 223 spawn-referenced tags, 913 ship-relevant tags.

Resolver uses `TagTree` hierarchy to classify tags (workspace rule §5 — no magic thresholds):
- **Ship-selective**: descendants of `Ship` root (832 tags).
- **Carried non-ship**: under `AI`/`Missions`, attached to ≥1 ship (Criminal, PU, VeryHard).
- **Pure context**: non-ship, 0 carriers (ArriveViaQT, HumanPilot70, Target).

Dispatch on *intent* in the original positive set:
- Ship-targeted query (has ≥1 Ship-root tag) → drop pure context; if ship tag survives filter, strict ALL-OF match; if it doesn't (broken/typo'd selector) return empty, not a generic over-match.
- Broad query (no Ship-root tag) → drop pure context, strict-match survivors. Handles intentional "any VeryHard Criminal ship" queries.

Validation against SC 4.7 LIVE: Gilly Combat Gauntlet Mission01 Wave1=Drake Cutter, Wave2=C8 Pisces variants, Wave3=∅ (broken `Relient_Tana` tag — SCMDB exhibits same bug); Mission07 first slot=Hammerhead; Mission08=Polaris+Gladius+Corsair. Matches user ground truth. Coverage **94.1% of spawn queries (5,920 of 6,291)**; residual 5.9% are pure-context queries awaiting runtime location merge, or broken-selector queries returning honest empty. Also fixed `sc_extract::TagTree::from_database` to derive parents from children (DCB has no `parent` field on Tag records; 65 root tags, 18,314 total).

Spec updated: `TagRegistry` dropped in favour of existing `sc_extract::TagTree`; `ReferenceGraph` noted as useful for v2 reverse-lookup queries but not needed by v1.

**sc-contracts merge API refactor (2026-04-24)** — replaced the `RewardCertainty` enum with two cleaner primitives:
- `Contract.title_siblings: Vec<Guid>` — GUIDs of other contracts sharing this contract's `(title, description)`. Computed once in the post-merge sweep; empty when title is unique.
- `find_bp_conflicts(&[Contract]) -> Vec<BpConflictGroup>` — purpose-built helper that returns every title with divergent blueprint pools across siblings, with `has_mixed_presence: bool` flagging the critical "some variants have BP, some don't" case sc-langpatch renders as `[BP]*`.

Rationale: the old `RewardCertainty::SharedUniform` was provably unreachable under strict merging (matching rewards always collapsed into one Contract). `SharedDiffering` was a binary "inspect further" hint that consumers couldn't act on without walking the data themselves. The pointer + helper combination gives consumers the actionable primitive directly. Analogous `find_scrip_conflicts` / `find_rep_conflicts` helpers are future work on the same shape.

Live numbers unchanged: 1,642 merged contracts, 366 (22.3%) with `title_siblings`. `find_bp_conflicts` returns 39 conflict groups globally, 8 with mixed BP presence.

**sc-contracts merge v1 landed (2026-04-24)** — `src/merge.rs`. `merge_expansions` groups `ExpandedContract`s by `(title, description, reward_signature)` (with `(handler_debug_name, debug_name)` fallback for text-less). Each `Contract` carries canonical fields + `variations: Vec<Variation>` with per-variation `extra_prerequisites` capturing the unique Locality GUIDs / gate prereqs that differ across tiers. Post-merge `compute_reward_certainty` flags sharing-title-with-divergent-rewards as `SharedDiffering`. On SC 4.7 LIVE: **1,642 Contracts from 4,590 expansions** (9.7% over SCMDB's 1,497 target, 2.8× collapse ratio). 366 `SharedDiffering` (22.3%) — the TarPits / Adaigo / CFP handyman salvage families where same runtime-sub title covers multiple reward tiers. 0 `SharedUniform` — unreachable under strict merging (matching rewards always collapse into the same Contract), documented in the enum. Gilly Combat Gauntlet collapses cleanly: 8 Contracts (one per scenario) each `variants: 4 (CSSS)` = parent+3subs with MG Scrip escalating 1→3→5→10→12→14→16→20. 12 unit tests. Future tuning lever: bucket scalar reward amounts before hashing if SCMDB alignment needs tightening.

**sc-contracts mission-span finding (2026-04-24)** — investigated what `Locality` prereq GUIDs actually resolve to. Traced via `expand_sample --detail`: `MissionLocality.* → StarMapObject.*` records. For Gilly Mission08 (Stanton training), the 4 expansions cover Hurston/Crusader/ArcCorp/microTech + moons. For Adaigo Pyro salvage, localities span Pyro I-VI with asteroid clusters — the "cross-system mission radius" pain players experience in Pyro/Nyx. 4,161 (91%) expansions have Locality prereqs; typical locality holds 15-30 locations. Documented as a planned v2 extension in `docs/sc-contracts.md §"Future features"` with proposed `mission_span: Vec<LocalityView>` field + `(System, Option<body>)` classifier. Raw GUIDs already captured in `PrereqView::Locality` — deferred field is a resolution convenience, not new data capture.

**sc-contracts expansion v2 landed (2026-04-24)** — extended `ExpandedContract` to the full contract surface: `Availability` struct (8 fields from handler's `ContractAvailability` overridden by typed `ContractBoolParamType` / `ContractIntParamType` params at contract + sub-contract levels) with `Cooldowns { completion, abandon, fail }`; `RewardAmount::{Calculated, Fixed, None}` for UEC; `ScripReward` / `RepReward` / `ItemReward` / `OtherReward` vecs with typed-currency dispatch via `RewardCurrencyCatalog`; rep amount resolution through `SReputationRewardAmount.reputationAmount`; `PrereqView` enum unifying handler+contract+sub-contract prereq chain into Locality/Location/LocationProperty/CrimeStat/Reputation/CompletedContractTags/Unknown. On SC 4.7 LIVE: 3,933 (86%) Calculated UEC / 0 Fixed (confirms aUEC is always engine-computed, no contract stores it as a `ContractResult_Item`), 513 with scrip, 3,603 with rep, 72 with items, 4,161 (91%) with prereqs, 4,471 (97%) with cooldown, 164 once-only. 10 unit tests.

**sc-contracts expansion v2 landed (2026-04-24)** — extended `ExpandedContract` with the full reward / availability / cooldown / prereq model, composing every registry built in stage 2:

- `Availability` — once_only / reaccept flags / hide_in_mobi_glas / max_players / etc., resolved through handler → contract → sub bool+int-param override inheritance (14 `ContractBoolParamType` + 6 `ContractIntParamType` variants).
- `Cooldowns { completion, abandon, fail }` with `DurationRange { mean, variation }`. Fail reuses abandon time gated by `can_reaccept_after_failing` (DCB has no separate fail field).
- `reward_uec: RewardAmount::{None, Calculated, Fixed(i32)}` — Calculated is the engine-computed marker from `ContractResult_CalculatedReward`.
- `reward_scrip: Vec<ScripReward>` via `RewardCurrencyCatalog`; `reward_rep: Vec<RepReward>` resolving `SReputationAmountParams.reward → SReputationRewardAmount.reputation_amount` (None for CalculatedReputation); `reward_items: Vec<ItemReward>` for non-currency ContractResult_Item; `reward_other: Vec<OtherReward>` for Badge / ScenarioProgress / JournalEntry / CompletionTags / CompletionBounty / ItemsWeighting / Reward / RefundBuyIn(f32) / Unknown-fallback.
- `prerequisites: Vec<PrereqView>` — typed enum (Locality / Location / LocationProperty / CrimeStat / Reputation / CompletedContractTags / Unknown) flattened from handler-availability + contract + sub-contract prereq sources.

Live 4.7 numbers: 3,933 CalculatedReward UEC (86%), 0 Fixed (confirms aUEC isn't a ContractResult_Item), 513 scrip, 3,603 rep, 72 items, 4,161 with prereqs (91%), 4,471 with cooldowns (97%). Gilly Mission08 renders end-to-end as "Combat Gauntlet - Scenario #8 | UEC calculated + MG Scrip ×20 + CompletionTags | cooldowns 5s/1s/1s | prereqs CompletedContractTags+CrimeStat+Locality | encounters Polaris+Gladius+Corsair". TarPits Stanton1 correctly flagged `illegal` with 60s abandon cooldown and the 9-ship salvage pool.

Remaining for step 4 (merge): group expansions by `(title, description, reward_signature)` → `Contract` with `variations: Vec<Variation>` + `reward_certainty`. The sub-contract duplication visible in v2 output will naturally collapse.

**sc-contracts expansion v1 landed (2026-04-24)** — `src/expand.rs`. [`expand_all`] walks every `ContractGenerator`, dispatches on handler kind (Legacy/Career/List/LinearSeries/Tutorial — PVPBounty + ServiceBeacon skipped in v1), and produces one `ExpandedContract` per (handler, contract, optional sub_contract) node. Each row composes `resolve_contract_text` (4-level title/desc inheritance), `ShipRegistry` (intent-based spawn resolution with `SpawnContext` non-ship tag classifier), `BlueprintPoolRegistry` (materialised pool items), + fields for `shareable` (template walk to `ActiveContractSettings.canBeShared`), `once_only` / `illegal_flag` (typed `ContractBoolParamType` with 3-level override inheritance). Output: 4,590 rows on SC 4.7 LIVE matching census expansion count. 85% resolved title, 77% runtime-substitution flagged, 1,052 with blueprint rewards, 885 illegal. Validated against Gilly Combat Gauntlet — the data correctly shows Mission03=Hornet, Mission04=Razor EX, Mission05=Freelancer+Sabre+Vanguard, Mission07=Hammerhead+Hornet+Arrow, Mission08=Polaris+Gladius+Corsair. Deferred to expansion v2: full reward model (UEC/scrip/rep/items), cooldowns, prerequisites, location resolution.

**sc-contracts RewardCurrencyCatalog landed (2026-04-24)** — `src/currency.rs`. Typed classifier via `SItemDefinition.type == EItemType::Currency` — no name matching. On SC 4.7 LIVE: 2 currency entities (MG Scrip 99 uses, Council Scrip 84 uses). Banu Wikelo favours are `EItemType::Cargo` (or similar), not Currency — intentionally classified as items, not scrip. Catalog exposes `is_currency(Guid) -> bool`, `get(Guid) -> Option<&CurrencyInfo>`, and iteration. 1 unit test.

**sc-contracts BlueprintPoolRegistry landed (2026-04-24)** — `src/blueprints.rs`. Walks every root `BlueprintPoolRecord` and resolves entries through `CraftingBlueprintRecord → CraftingBlueprint → CraftingProcess_Creation.entity_class → DisplayNameCache` for item display names, falling back to `CraftingBlueprint.blueprintName` LocaleMap lookup. Handles CIG's `<= PLACEHOLDER =>` localization placeholders (detected and treated as unresolved so the fallback path triggers). Against SC 4.7 LIVE: 45 pools, pool sizes 4-15 items, essentially 100% item-name resolution (only 2 unresolved blueprint refs + 1 missing locale key). Item names match SCMDB formatting: "Corbel Arms Halcyon", "Arrowhead Sniper Rifle Battery (16 cap)", "Pulse Laser Pistol", etc. Design guided by the `rjcncpt/StarCitizen-Deutsch-INI` blueprint-annotation feature: per-tier pools, `[BP]*` marking for mixed-reward siblings (already covered by our `RewardCertainty::SharedDiffering`), and the "CIG ships placeholder text, we ignore it" workaround. 2 unit tests.

**sc-contracts title/description resolver landed (2026-04-24)** — `src/titles.rs`. `resolve_contract_text` walks the full inheritance chain (sub_contract → contract.paramOverrides → handler.contractParams → template.stringParamOverrides) and resolves the final `LocaleKey`s to `String` via `LocaleMap`. Scope change from original spec: title/description elevated from "consumer concern" (LocaleKey passthrough) to "sc-contracts concern" (resolved String). Discovered via TarPits salvage investigation — those contracts carry text only on the handler or template, not `paramOverrides`, so naive resolvers silently returned empty strings. `ResolvedText::has_runtime_substitution` surfaces the `~mission(variable)` markers the engine fills at spawn time (TarPits titles resolve to `~mission(Contractor|SalvageContractorTitle)` → then at runtime `TarPits_ShipStrip_title_001` = `~mission(ship) clean up` → then spawn picks a specific ship). The two-layer runtime substitution + independent pool roll is the mechanism behind the in-game "description says Caterpillar but spawn is Starfarer" mismatch players see. 3 unit tests + live audit.

**sc-contracts SpawnContext landed (2026-04-24)** — `src/classify.rs`. Subtree-driven classifier of the non-ship positive tags on a spawn query. Fields: `ai_skill` (parsed from `HumanPilotN`), `ace_pilot`, `factions` (descendants of `AI ▸ Faction`), `cargo` (`AI ▸ CargoManifest`), `ai_traits` (other AI descendants), `mission_tags` (`Missions` root), `directives` (name prefix `Arrive` / `Ignore`), `other` (uncategorised — never silently dropped). Validated against Gilly Combat Gauntlet: Missions 01→08 show `ai_skill=10…90` with cargo progressing `Scraps ▸ Minimal ▸ Half ▸ Full` — the mission's intended difficulty curve surfaces directly. Consumers like sc-langpatch get wave enrichment for free. 4 unit tests + live audit verification.

**sc-contracts census landed (2026-04-24)** — `examples/contract_census.rs` walks the typed contract graph from `datacore.pools()`. First run against SC 4.7 LIVE produced: 106 generators, 248 handlers (71% Career, 25% List, rest <1%), 4,590 expansions, 1,800 sub-contracts, 929 distinct title keys across 2,377 title-bearing expansions. Zero unknown handler / prereq / reward kinds — typed enums fully cover the 4.7 surface. Major discovery: **`ContractResult_CalculatedReward` is the 2nd most common reward kind at 34%** (2,198 records) and carries no fields — the engine computes the amount at runtime. Most mission aUEC is *not* stored in the DCB. Spec updated: `reward_uec: RewardAmount::{Calculated, Fixed(i32), None}`. Currency catalog is small and populated: merc scrip (99 uses) + council scrip (84 uses) + Wikelo favours + sparse collector ship one-offs. aUEC never appears as a `ContractResult_Item.entity_class`, confirming the calculated hypothesis.

**sc-contracts spec drafted + revised (2026-04-24)** — `docs/sc-contracts.md` landed as a proposal, then restructured per design review. Single-layer model crate (no raw layer — escape hatch is `datacore.db()` / `.pools()`). Core pipeline: ingest registries (tag, ship-entity, blueprint-pool, reward-currency) → expand generators (handler × contract × sub_contract) → resolve GUIDs → similarity-merge to ~1,497 effective contracts (SCMDB calibration target). `TagRegistry` and `ShipRegistry` are owned here rather than deferred to future shared crates. Ship registry construction is data-derived from spawn-query tags (not sc-langpatch's under-matching name filter). Variation merging groups by `(title, description, reward_signature)`; post-merge `RewardCertainty::{Unique, SharedUniform, SharedDiffering}` surfaces the "reward info not guaranteed" signal sc-langpatch's existing mixed-BP logic computes today. Workspace design principle §5 added: no string matching where typed / data-derived alternatives exist.

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
| `sc-contracts` | Stub crate. Design spec `docs/sc-contracts.md` awaiting review. |

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
