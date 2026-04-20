# `sc-weapons` — design specification

> Status: **v1 shipped, v2 phases 1 + 2 shipped.** Multi-mode fire actions, charge modifiers, ballistic heat-cycle sustained DPS, energy regen-gated sustained DPS, time-to-overheat. Based on data exploration against 4.7 LIVE, verified against spviewer. See `D:\Obsidian\Star Citizen\Mechanics\Weapons.md` for the full exploration findings.

## Purpose

`sc-weapons` provides hand-written domain wrappers around `EntityClassDefinition` records that represent weapons — ship-mounted and FPS. It materializes weapon data from the flat-pool model into ergonomic, owned structs that consumers can use without threading `&DataPools` through every call.

v1 is **data accessors only** — no derived calculations (sustained DPS, time-to-overheat). Those formulas need validation against real data before shipping, per the "go slow" principle.

## Consumers

| Crate / app | What it uses |
|---|---|
| `bulkhead` (future) | Full weapon data for the damage-pipeline composition point. **Drives correctness.** |
| `sc-langpatch` (future) | Display names, class / size / grade for label enrichment. Reads a few fields off the same struct. |
| `streamdeck-starcitizen` | Does **not** use `sc-weapons`. |

## Scope

**What `sc-weapons` v1 owns:**

- `ShipWeapon` — materialized struct for ship-mounted weapons (`EItemType::WeaponGun`, `EItemSubType::Gun | Rocket`). 182 entities, ~164 unique bases in 4.7.
- `FpsWeapon` — materialized struct for FPS weapons (`EItemType::WeaponPersonal`, `SubType != Gadget`). 331 entities, ~44 unique bases in 4.7.
- `FireActionKind` — enum covering all 9 fire action types with extracted scalars.
- `DamageSummary` — all 6 damage types summed from direct + explosion ammo chain.
- `HeatModel` / `EnergyModel` / `SustainKind` — sustain params as data, no derived calcs.
- Ammo chain resolution (two-path, 100% for combat weapons).
- Iteration helpers: `iter_ship_weapons`, `iter_fps_weapons`.

**What `sc-weapons` v1 does NOT own:**

- Sustained DPS / time-to-overheat calculations (deferred to v2).
- `WeaponCore` trait / `BallisticWeapon` / `EnergyWeapon` traits (deferred to v2).
- sc-ammo as a separate crate (ammo logic is inline in a private `damage` module).
- Damage pipeline (shield/armor/hull) — consumer responsibility.
- Ship loadout / hardpoint resolution — future sc-ships.
- Display name / manufacturer name resolution — consumers use `DisplayNameCache` / `ManufacturerRegistry` directly.
- Power state multiplier application (all 1.0 on combat weapons in 4.7).

## Architecture: materialized owned structs

Generated types don't derive `Clone` or `Debug`. Storing `Handle<T>` would require `&DataPools` for every accessor. Instead, `try_new` constructors resolve all handles once, copy ~300 bytes of scalars per weapon into owned structs. ~500 weapons × 300 bytes = negligible memory.

After construction, every accessor is a plain field read. No pool references needed.

The `entity_handle: Handle<EntityClassDefinition>` field is kept as an escape hatch — consumers with access to `DataPools` can reach the raw generated type if needed.

## Classification

Weapons are classified by `SAttachableComponentParams.AttachDef` → `SItemDefinition`:

```rust
// classify.rs
match (item_def.r#type, item_def.sub_type) {
    (EItemType::WeaponGun, EItemSubType::Gun | EItemSubType::Rocket) => Ship,
    (EItemType::WeaponPersonal, sub) if sub != EItemSubType::Gadget => Fps,
    _ => None, // CMLs, gadgets, mining, creatures — out of scope
}
```

Uses proper enum variants, not string matching. The spec's original `"WeaponGun"` string comparisons were wrong — the generated code uses `EItemType` / `EItemSubType` enums.

## Fire action types

`SCItemWeaponComponentParams.fireActions` is a `Vec<SWeaponActionParamsPtr>` (poly enum). Both `ShipWeapon` and `FpsWeapon` expose the full `Vec<FireActionKind>` in declaration order (`fire_actions[0]` is the primary mode). 97 of 331 FPS weapons in 4.7 expose >1 mode (Karna burst→charge, Kastak burst→burst, beam-rifle wrappers, etc.); ship weapons in 4.7 are uniformly single-mode.

```rust
// fire_action.rs
pub enum FireActionKind {
    Rapid { fire_rate: i32, heat_per_shot: f32, spin_up: f32, spin_down: f32 },
    Single { fire_rate: i32, heat_per_shot: f32 },
    Sequence { effective_rpm: f32, barrel_count: usize },
    Burst { fire_rate: i32, shot_count: u32, cooldown: f32 },
    Charged {
        charge_time: f32, overcharge_time: f32, cooldown: f32,
        auto_fire: bool, full_only: bool,
        max_modifier: Option<ChargeModifier>,  // damage/speed/pellet/etc.
    },
    Beam { dps: DamageSummary, full_range: f32, zero_range: f32, heat_per_sec: f32 },
    Unknown,
}

pub struct ChargeModifier {
    pub damage_multiplier: f32,
    pub projectile_speed_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub charge_time_multiplier: f32,
    pub heat_generation_multiplier: f32,
    pub ammo_cost_multiplier: f32,
    pub pellets: i32,        // 0 = no override
    pub burst_shots: i32,    // 0 = no override
}
```

Verified at full-charge: KLWE Atlas S10 (capital ship railgun) reports `dmg×2.00, heat×2.00`; Karna alt-mode reports `dmg×1.20, burst=8`.

### Extraction rules per variant

- **FireRapid** → copy `fire_rate`, `heat_per_shot`, `spin_up_time`, `spin_down_time`.
- **FireSingle** → copy `fire_rate`, `heat_per_shot`.
- **Sequence** → walk `sequence_entries`, compute `effective_rpm = min(delay, inner_rpm)` (verified with spviewer on 12 weapons). `barrel_count` = number of entries.
- **Burst** → copy `fire_rate`, `shot_count`, `cooldown_time`.
- **Charged** → copy `charge_time`, `overcharge_time`, `cooldown_time`, `fire_automatically_on_full_charge`, `fire_only_on_full_charge`. Resolves `max_charge_modifier` → `SWeaponStats` and exposes the multiplier scalars as `Option<ChargeModifier>`.
- **Beam** → extract `damage_per_second` (resolves DamageBasePtr → DamageSummary), copy `full_damage_range`, `zero_damage_range`, `heat_per_second`.
- **DynamicCondition** → unwrap to `default_weapon_action` and extract that. This is the Volt heat-ramp wrapper.
- **HealingBeam / Parallel** → `Unknown` (out of scope for combat).

### Sequence RPM rule

Verified against spviewer: `effective_rpm = min(sequence_delay_rpm, inner_action_rpm)`. The sequence delay is the intended rate, the inner action's RPM is a per-shot cap. For burst inner actions, effective shots/min = sequence_rate × burst_count.

## Sustain models

Ship weapons have exactly one of: Heat (65), Energy (108), or None (9 RPODs). No weapon has both.

```rust
// sustain.rs
pub struct HeatModel {
    pub overheat_temperature: f32,
    pub cooling_per_second: f32,
    pub overheat_fix_time: f32,
    pub temperature_after_overheat_fix: f32,
    pub time_till_cooling_starts: f32,
}

pub struct EnergyModel {
    pub max_ammo_load: f32,
    pub max_regen_per_sec: f32,
    pub regeneration_cooldown: f32,
    pub regeneration_cost_per_bullet: f32,
    pub requested_regen_per_sec: f32,
    pub requested_ammo_load: f32,
}

pub enum SustainKind {
    Heat(HeatModel),
    Energy(EnergyModel),
    None,
}
```

**Critical correction**: `heat_per_shot` is on the **fire action** (`SWeaponActionFireRapidParams.heatPerShot`), NOT on `SWeaponSimplifiedHeatParams`. The heat model struct carries the cooling/overheat params; the fire action carries the per-shot heat cost. They must be combined for sustain calculations (deferred to v2).

## Ammo chain resolution

Two paths, 100% resolution for combat weapons (433 resolved, 27 unresolved are mining/creature/dummy):

```rust
// damage.rs — ammo resolution algorithm
fn resolve_ammo(ecd, weapon_params, pools, ecd_map, ammo_map) -> Option<&AmmoParams> {
    // Path 1: weapon entity's own SAmmoContainerComponentParams (377 weapons)
    // Path 2: weapon_params.ammo_container_record → intermediate entity → SAmmoContainerComponentParams (56 weapons)
    // ammo_container_record NEVER points directly at AmmoParams — always at an entity
}
```

Damage extraction walks: `AmmoParams.projectile_params` (poly enum → `BulletProjectileParams`) → `damage` (poly enum → `DamageInfo`) + `detonation_params.explosion_params.damage`. All 6 fields summed.

```rust
pub struct DamageSummary {
    pub physical: f32,
    pub energy: f32,
    pub distortion: f32,
    pub thermal: f32,
    pub biochemical: f32,
    pub stun: f32,
}
```

## ShipWeapon struct

```rust
pub struct ShipWeapon {
    pub guid: CigGuid,
    pub record_name: String,
    pub size: i32,
    pub grade: i32,
    pub item_type: EItemType,
    pub item_sub_type: EItemSubType,
    pub manufacturer_guid: Option<CigGuid>,
    pub fire_actions: Vec<FireActionKind>,
    pub sustain: SustainKind,
    pub damage: Option<DamageSummary>,
    pub ammo_speed: Option<f32>,
    pub ammo_lifetime: Option<f32>,
    pub pellet_count: Option<i32>,
    pub magazine_size: Option<i32>,
    pub power_draw: Option<f32>,
    pub health: Option<f32>,
    pub entity_handle: Handle<EntityClassDefinition>,
}
```

Constructor returns `Option<Self>` — `None` if classification doesn't match or required components missing. Construction failure is normal during iteration over all entities.

## FpsWeapon struct

Same shape minus ship-specific fields:
- No `sustain` (FPS sustain is complex: Volt heat-ramp, Kastak charged modes — deferred)
- No `power_draw` (FPS weapons don't participate in ship power network)
- No `health` (FPS weapons have no `SHealthComponentParams`)

## Iteration helpers

```rust
pub fn iter_ship_weapons(datacore: &Datacore) -> impl Iterator<Item = ShipWeapon> + '_
pub fn iter_fps_weapons(datacore: &Datacore) -> impl Iterator<Item = FpsWeapon> + '_
```

Takes `&Datacore` (not just snapshot) because record names require `DataCoreDatabase` access. Walks `entity_class_definition` map, attempts construction, filters `None`.

## Dependencies

```toml
[dependencies]
sc-extract = { workspace = true, features = ["entities-scitem-weapons", "ammoparams"] }
thiserror = { workspace = true }
tracing = { workspace = true }
svarog-common = { workspace = true }
```

No consumer-facing feature flags. Batteries included.

## Derived sustain numbers (v2 phase 2)

Materialised `ShipWeapon` structs carry the raw `HeatModel` / `EnergyModel`. The following derivation methods compute common sustain-relevant numbers on top:

```rust
impl HeatModel {
    fn time_to_overheat(&self, heat_per_shot: f32, rpm: f32) -> Option<f32>;
    fn duty_cycle(&self, heat_per_shot: f32, rpm: f32) -> Option<f32>;
}
impl EnergyModel {
    fn burst_shot_count(&self) -> Option<f32>;
    fn sustained_rpm(&self) -> Option<f32>;
}
impl ShipWeapon {
    fn alpha_dps(&self) -> Option<f32>;              // burst-rate DPS
    fn time_to_overheat(&self) -> Option<f32>;       // Rapid/Single + Heat only
    fn overheat_lockout_time(&self) -> Option<f32>;
    fn sustained_dps(&self) -> Option<f32>;
}
```

**Validated against `Weapons.md` reference values** (pinned in unit tests):

| Weapon | Metric | Expected | Computed |
|---|---|---|---|
| GATS_BallisticGatling_S1 | time_to_overheat | 11.72 s | ✓ 11.72 s |
| GATS_BallisticGatling_S1 | duty_cycle | 0.842 | ✓ 0.842 (84%) |
| HRST_LaserRepeater_S3 | burst_shot_count | 0.89 | ✓ 0.89 |
| HRST_LaserRepeater_S3 | sustained_rpm | 10.71 rpm | ✓ 10.7 rpm |

**Known caveats** (documented in method docstrings):

- **`alpha_dps` returns `None` for Burst / Charged / Unknown.** Burst cooldown timing and charge user-pacing need richer models.
- **Energy `sustained_dps` is a power-starved floor.** Uses `max_regen_per_sec` (the hard rate cap); the `requested_regen_per_sec` gap (often 50-300×) suggests real in-game DPS with full ship power is much closer to `alpha_dps`. sc-ships will eventually compute the power-network-constrained rate from raw fields.
- **Beam weapons' capacitor depletion** is not modelled in `sustained_dps` — it returns the instantaneous beam DPS. Exodus-10 style capacitor drain vs. regen is a consumer responsibility for now.
- **Sequence + Heat** assumes the weapon never overheats (fire-action `heat_per_shot` isn't on Sequence variants — heat may come from `SWeaponStats` on the connection params; not yet modelled).

## Planned v2 phase 3 — comparison stats (designed, not yet implemented)

Designed during the 2026-04-20 design session. Blocked on the model fixes listed below — numbers from the current code are wrong for Sequence/scattergun/energy weapons, so we write the spec, fix the data, then implement.

### Tier 1 — directly comparable (no rate/time dependency)

Already exposed via existing fields on `ShipWeapon`: `alpha` (= `damage.total()`), `damage_by_type` (full `DamageSummary`), `pellet_count`, `ammo_speed`, `ammo_lifetime`, projectile range (= `speed × lifetime`), `power_draw`, `health`. Two weapons' per-shot characteristics compare directly regardless of fire rate.

### Tier 2 — burst stats (rpm-coupled, compare within same RPM bucket)

```rust
impl ShipWeapon {
    fn burst_rpm(&self) -> Option<f32>;             // fire_rate or effective_rpm
    fn burst_dps(&self) -> Option<f32>;             // alpha × rpm / 60 × pellets
    fn burst_seconds(&self) -> Option<f32>;         // time until first forced stop
    fn volley_damage(&self) -> Option<f32>;         // burst_dps × burst_seconds
    fn recovery_seconds(&self) -> Option<f32>;      // lockout / regen-to-ready
    fn cycle_seconds(&self) -> Option<f32>;         // burst_seconds + recovery_seconds
}
```

Rename note: current `alpha_dps` becomes `burst_dps`; current `time_to_overheat` generalises to `burst_seconds` (covers both heat exhaustion and capacitor depletion under one name).

### Tier 3 — normalised (cross-RPM comparable)

```rust
impl ShipWeapon {
    /// % of burst_dps retained averaged over window T, starting from full/cold state.
    fn dps_retention_pct(&self, window_seconds: f32) -> Option<f32>;

    /// % of wall-clock time firing in long-run engagement (asymptotic).
    fn firing_time_pct(&self) -> Option<f32>;

    /// Asymptotic DPS as % of burst_dps (T→∞ limit of dps_retention_pct).
    fn long_run_dps_pct(&self) -> Option<f32>;

    /// Damage per heat unit consumed (heat weapons) — strips RPM entirely.
    fn thermal_efficiency(&self) -> Option<f32>;

    /// Burst DPS per unit of power draw (energy/mass-driver weapons).
    fn power_efficiency(&self) -> Option<f32>;
}
```

All return unitless ratios or domain-normalised numbers so weapons of different size/class compare directly.

### Single composite score for default sort

```rust
pub struct LoadoutContext {
    /// Engagement window. Caller-provided — typically wired to a UI slider
    /// so users can fine-tune per use-case.
    pub window_seconds: f32,

    /// Power units allocated to this weapon's slot. Caller-provided —
    /// `sc-ships` will compute actual values from loadout + power plant
    /// capacity + distribution. `None` = unconstrained (peak performance).
    pub power_per_slot: Option<f32>,
}

impl ShipWeapon {
    fn effective_dps(&self, ctx: &LoadoutContext) -> Option<f32>;
}
```

**Formula**:

```
power_factor = {
    ballistic: 1.0                                             // power-free
    energy:    min(1.0, power_per_slot / requested_power_for_peak)
    charged:   1.0                                             // user-paced
    none:      1.0
}
sustain_factor = dps_retention_pct(window_seconds) / 100
effective_dps  = burst_dps × power_factor × sustain_factor
```

**No default values baked in.** Both `window_seconds` and `power_per_slot` are caller-provided. The consumer app (`sc-ships`, future loadout UI) is responsible for supplying them from runtime state — a UI slider for the window, the currently-selected ship's power plant for the power share. This crate doesn't decide what "a reasonable window" is; the scenario decides.

`effective_dps` is the single scalar for the default sort. Tiebreaking / display columns in a UI: `volley_damage`, `burst_seconds`, `dps_retention_pct(window_seconds)`.

### Model fixes that must land first

The Phase 3 stats are only honest once these three bugs in Phase 2's code are resolved:

1. **Sequence / scattergun heat extraction.** Current `HeatModel::time_to_overheat` only reads `heat_per_shot` from Rapid/Single fire actions. For Sequence weapons (Deadbolt III, Tarantula, Shredder, etc.) and Single-with-hps=0 scatterguns (Predator), heat-per-shot lives on `SWeaponStats` attached to `connectionParams` (likely the `noPower` / `heatStats` slot per `D:/Obsidian/Star Citizen/Mechanics/Weapons.md` §Power state modifiers). Fix: extend `sustain.rs::extract_sustain` to carry a `heat_per_shot` field on `HeatModel` populated from `SWeaponStats` when the fire action doesn't carry it.

2. **Warm-restart duty cycle.** `HeatModel::duty_cycle` currently uses cold-start `time_to_overheat` for every cycle. Weapons with `temperature_after_overheat_fix > 0` (e.g. BEHR Revenant S6 Gatling with `after_fix=99`) have drastically shorter *warm* bursts than cold. Fix: add `time_to_overheat_warm()` using `(overheat_temperature - temperature_after_overheat_fix) / heat_rate`, and use the warm value in `duty_cycle` / `long_run_dps_pct` (asymptotic) while keeping cold for the first burst in `dps_retention_pct`.

3. **Energy model semantics.** HRST Attrition-3 S3 current impl reports 3% of burst DPS sustained vs spviewer's 71.4%. Ambiguity: whether `max_regen_per_sec` is the hard cap (current assumption) or the starvation floor, with `requested_regen_per_sec` being the effective rate when power is adequate. Needs reverse-engineering against spviewer values for 5-10 energy weapons to pin the real formula.

Once these land, re-run the 5-weapon comparison (Deadbolt III, Tarantula GT-870 Mk 3, Revenant Gatling, Shredder, Predator Scattergun) with real sustain numbers and validate the `effective_dps` rankings against intuition.

## Deferred to v2/v3

- **FPS sustain models** — Volt heat-ramp mechanic (DynamicCondition + rangedHeatStats), Kastak charged secondary modes, K&W FPS heat behavior.
- **Burst fire sustained DPS** — `SWeaponActionFireBurstParams.cooldown_time` needs a burst-cycle model to produce an honest average rate.
- **`WeaponCore` trait** — shared trait for polymorphism over ShipWeapon/FpsWeapon. Add when a consumer needs it.
- **BallisticWeapon / EnergyWeapon specialization traits** — the Option-returning pattern. SustainKind enum is sufficient for v1.
- **sc-ammo as separate crate** — extract damage.rs when a second consumer needs ammo logic independently.
- **Display name resolution** — consumers use `DisplayNameCache::get(&weapon.guid)` directly.
- **Manufacturer name resolution** — consumers use `ManufacturerRegistry::get(&weapon.manufacturer_guid)` directly.
- **Power state multipliers** — all 1.0 on combat weapons in 4.7. Overclock was active in a past patch but is currently disabled. Infrastructure exists but unused.
- **Weapon attachment sub-ports** — SItemPortContainerComponentParams + SEntityComponentDefaultLoadoutParams for optics, barrels, underbarrels on FPS weapons.
