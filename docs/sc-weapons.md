# `sc-weapons` — design specification

> Status: **v1 implementing.** Based on data exploration against 4.7 LIVE, verified against spviewer. See `D:\Obsidian\Star Citizen\Mechanics\Weapons.md` for the full exploration findings.

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

`SCItemWeaponComponentParams.fireActions` is a `Vec<SWeaponActionParamsPtr>` (poly enum). v1 extracts the **primary** fire action (`[0]`). 97 weapons have multiple fire actions (fire mode switching) — secondary modes are deferred to v2.

```rust
// fire_action.rs
pub enum FireActionKind {
    Rapid { fire_rate: i32, heat_per_shot: f32, spin_up: f32, spin_down: f32 },
    Single { fire_rate: i32, heat_per_shot: f32 },
    Sequence { effective_rpm: f32, barrel_count: usize },
    Burst { fire_rate: i32, shot_count: u32, cooldown: f32 },
    Charged { charge_time: f32, overcharge_time: f32, cooldown: f32, auto_fire: bool, full_only: bool },
    Beam { dps: DamageSummary, full_range: f32, zero_range: f32, heat_per_sec: f32 },
    Unknown,
}
```

### Extraction rules per variant

- **FireRapid** → copy `fire_rate`, `heat_per_shot`, `spin_up_time`, `spin_down_time`.
- **FireSingle** → copy `fire_rate`, `heat_per_shot`.
- **Sequence** → walk `sequence_entries`, compute `effective_rpm = min(delay, inner_rpm)` (verified with spviewer on 12 weapons). `barrel_count` = number of entries.
- **Burst** → copy `fire_rate`, `shot_count`, `cooldown_time`.
- **Charged** → copy `charge_time`, `overcharge_time`, `cooldown_time`, `fire_automatically_on_full_charge`, `fire_only_on_full_charge`. Inner action details deferred.
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
    pub primary_fire_action: FireActionKind,
    pub fire_action_count: usize,
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

## Deferred to v2

- **Sustained DPS calculations** — ballistic heat-cycle model (overheat + recovery), energy burst+regen model. Formulas from the original spec need validation against real data.
- **Time-to-overheat calculations** — derived from heat_per_shot × fire_rate vs cooling_per_second.
- **FPS sustain models** — Volt heat-ramp mechanic (DynamicCondition + rangedHeatStats), Kastak charged secondary modes, K&W FPS heat behavior.
- **WeaponCore trait** — shared trait for polymorphism over ShipWeapon/FpsWeapon. Add when a consumer needs it.
- **BallisticWeapon / EnergyWeapon specialization traits** — the Option-returning pattern. SustainKind enum is sufficient for v1.
- **sc-ammo as separate crate** — extract damage.rs when a second consumer needs ammo logic independently.
- **Display name resolution** — consumers use `DisplayNameCache::get(&weapon.guid)` directly.
- **Manufacturer name resolution** — consumers use `ManufacturerRegistry::get(&weapon.manufacturer_guid)` directly.
- **Power state multipliers** — all 1.0 on combat weapons in 4.7. Overclock was active in a past patch but is currently disabled. Infrastructure exists but unused.
- **Secondary fire modes** — v1 extracts primary fire action only. 97 weapons have multiple actions (mode switching). Expose secondary modes in v2.
- **Weapon attachment sub-ports** — SItemPortContainerComponentParams + SEntityComponentDefaultLoadoutParams for optics, barrels, underbarrels on FPS weapons.
- **Charge modifier details** — `SWeaponActionFireChargedParams.maxChargeModifier` carries `SWeaponStats` with damage/speed/pellet multipliers. v1 stores charge timing only.
