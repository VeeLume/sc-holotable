# Crafting product stats — orientation

> Short orientation for the `sc-holotable/v0.14.0` feature: crafted-item
> **product stats** = an item's *base* stats reshaped by a recipe's per-material
> quality modifiers. Read this before touching the `sc-items-*` stat crates or
> `sc-crafting`'s product-stats surface.

## The shape of the problem

A crafting recipe's per-material effects (modelled in `sc-crafting` since
v0.13.0) modify **gameplay properties** (`CraftingGameplayPropertyDef`, "GPP")
by quality. To show a final stat you need the item's **base value** for that
property and a way to join the two. Two halves:

1. **Base stats** — read from the crafted `EntityClassDefinition`'s typed
   component structure. One pure-data **T1 crate per item domain**.
2. **The join** — GPP → which base stat. **This link is NOT in the p4k**
   (proven 3 ways: GPP defs are display-only; `ReferenceGraph` shows GPPs
   referenced *only* by `CraftingBlueprintRecord`; entity trees never reference
   a GPP). So it's a curated mapping — anchored on the GPP's **record name**
   (e.g. `GPP_Weapon_FireRate`), **never its GUID** (GUIDs churn between
   patches and fail silently), build-validated with a `warn!` on drift. This is
   the project rule's "string match genuinely unavoidable → scope, comment,
   alarm" carve-out.

## New crates (T1, pure data, keyed by entity GUID)

| Crate | Item types | Stats |
|---|---|---|
| `sc-items-fps-weapons` | WeaponPersonal | `FpsWeapons`/`FpsWeaponStats`: fire_rate, damage, spread, recoil (pitch/yaw/smooth), mag |
| `sc-items-armor` | Char_Armor_* | `Armor`/`ArmorStats`: temp_resistance, radiation, per-type `DamageResistance` |
| `sc-items-ship-components` | Cooler/PowerPlant/QuantumDrive/Shield/Radar | `ShipComponents`/`ShipComponentStats`: integrity + quantum speed/fuel, shield HP/regen, coolant/power, radar aim-assist |
| `sc-items-ship-weapons` | WeaponGun/WeaponMining | `ShipWeapons`/`ShipWeaponStats`: integrity + per-shot (gun) / beam-DPS (mining) damage |

Each is `<Index>::build(&datacore, &items)`. They depend only on `sc-extract` +
`sc-items`; classification via `EItemType`, stats read through the **raw
`Datacore::db()` layer** (the stat components aren't under one clean sc-extract
feature, and several are non-pooled `Reference` targets). `sc-weapons` is now
**legacy** — these are the focused, crafting-facing replacements.

## API (`sc-crafting`)

```rust
pub enum GameplayStat { WeaponFireRate, WeaponDamage, WeaponRecoil{Kick,Handling,Smoothness},
    WeaponSpread, ArmorDamageMitigation, ArmorTemperature{Min,Max}, ArmorRadiationDissipation,
    Integrity, QuantumSpeed, QuantumFuelRequirement, ShieldMaxHealth,
    CoolantGeneration, PowerGeneration, Radar{Min,Max}AimAssist, Unknown(String) }

impl GameplayProperty { pub fn stat(&self) -> GameplayStat; pub record_name: String }
impl GameplayStat { pub fn from_gpp_name(record_name: &str) -> GameplayStat }

pub trait ProductStatSource { fn base_value(&self, entity: &Guid, stat: &GameplayStat) -> Option<f32>; }
// impl'd in sc-crafting for FpsWeapons / Armor / ShipComponents / ShipWeapons

pub struct ProductStat { gameplay_property: Guid, stat: GameplayStat, base: Option<f32>,
    modified: Option<f32>, factor: f32, additive: f32 }   // .pct_change()

impl Blueprints {
    pub fn product_stats<S: ProductStatSource>(&self, entity: Guid, gp: &GameplayProperties,
        bases: &S, quality: i32) -> Vec<ProductStat>;
}
```

Usage: build the domain index for the crafted item's type, then
`blueprints.product_stats(entity_guid, &gp, &fps_weapons /* or armor/ship_* */, quality)`.
`base = None` → percent-only stat (the consumer shows `pct_change()`).

**Aggregation is additive-delta** across slots: `factor = 1 + Σ(factorᵢ − 1)`
(two −10% slots → −20%, verified vs scmdb — *not* multiplicative). The quality
curve crosses 1.0 at Base-500: **below Q500 every crafted stat is worse**.

## Umbrella (`sc-holotable`)

Features `fps-weapons` / `armor` / `ship-components` / `ship-weapons` (all
pulled by `crafting`, in `all-t1`). Prelude re-exports the index/stats types +
`GameplayStat` / `ProductStat` / `ProductStatSource`. Modules:
`sc_holotable::{fps_weapons, armor, ship_components, ship_weapons}`.

## Adding a new domain (mechanical, 4 steps)

1. New `sc-items-<x>` crate: `<Index>::build` + per-item stats (raw or typed).
2. Register: workspace `members` + `[workspace.dependencies]` + sc-crafting dep.
3. In sc-crafting: any new `GameplayStat` variants + `KNOWN` `(suffix, variant)`
   rows + `impl ProductStatSource for <Index>`.
4. Umbrella: optional dep + `<x>` feature (+ `crafting` enables it) + module +
   prelude export.

## Gotchas (verified field paths)

- **FPS recoil** lives at `fireActions[0].recoil → WeaponProceduralRecoilConfigDef.weaponProceduralAimRecoil.curveAimRecoil.{pitchMaxDegrees,yawMaxDegrees,recoilSmoothTime}` — *not* the sibling `max` Vec2 (often 0) or `actorProceduralRecoilConfig` (null).
- **Armor Damage Mitigation** = `1 − physical_multiplier` (mitigation fraction; the modifier scales the mitigation, not the multiplier).
- **`SCItemCoolerParams`/`SCItemPowerPlantParams` are dormant** — coolant/power live in `ItemResourceComponentParams.states[].deltas[].generation` (resource enum Coolant/Power); power is an integer `units` (pips), coolant a float `standardResourceUnits`.
- **Quantum drive speed** = `params.driveSpeed` ÷ 1e6 (m/s → Mm/s).
- **Mining laser** is a beam: damage = `fireActions[0](SWeaponActionFireBeamParams).damagePerSecond`; guns use the FPS ammo chain.

## Not covered

- **Tractor / hull-scraping** GPPs → percent-only (no absolute base in static data; scmdb shows the same).
- **SalvageModifier** absolute Speed/Radius/Efficiency (deep `weaponStats.salvageModifier.*` path) — deferred, niche.
- Context stats not modified by recipes (EM sig, power draw, repair time, ship-gun fire rate/spread/DPS-derived) — extracted where trivial, not wired as product stats.

## Tools

- `cargo run -p sc-crafting --release --example product_stats` — end-to-end scmdb reproduction (weapons/armor/ship).
- `cargo run -p sc-crafting --release --example craft_landscape` — every craftable item type → its GPPs + base components.
- `examples/fps_gpp_dig.rs` — the GPP-binding investigation (the "not in the p4k" proof).
