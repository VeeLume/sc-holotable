# Feature request — `sc-weapons` gaps surfaced by sc-langpatch migration

> **Consumer:** `sc-langpatch` (`weapon_enhancer` module)
> **Filed:** 2026-04-24
> **Status:** Proposed — not implemented
> **Driver:** Migration of `weapon_enhancer` from raw svarog walks to `sc-weapons` v1+v2.

## Background

`sc-langpatch`'s `weapon_enhancer` module annotates `global.ini` with weapon stats — size prefix on the name, damage / penetration / projectile speed / ammo / capacitor on the description. The module was migrated to consume `sc_weapons::iter_ship_weapons(&datacore)` so that weapon data extraction lives in one canonical crate (workspace rule §3 — one canonical model per domain).

The migration successfully moved ammo resolution, damage extraction, and capacitor sizing into `sc-weapons`. Three pieces still require raw-svarog walks in `sc-langpatch` and are tagged `TODO(sc-holotable)` in the consumer code. They are reasonable additions to `sc-weapons` and are filed here as a single bundle since they share the same migration story.

## 1. Localization keys on `ShipWeapon` / `FpsWeapon`

### What's missing

`ShipWeapon` exposes `record_name` and `entity_handle` but no localization-key fields. To patch a weapon's INI entry, sc-langpatch needs the `Name` / `Description` keys from `SAttachableComponentParams.AttachDef.Localization`. These are the strings ending up in `global.ini` (e.g. `item_NameGATS_BallisticGatling_S1`).

### Current consumer workaround

```rust
// sc-langpatch/src-tauri/src/modules/weapon_enhancer.rs
fn loc_keys_for(db: &DataCoreDatabase, guid: Guid) -> Option<LocKeys> {
    let record = db.record(&guid)?;
    let inst = record.as_instance();
    let components = inst.get_array("Components")?;
    for comp in components {
        let comp_inst = to_instance(db, &comp)?;
        if comp_inst.type_name() != Some("SAttachableComponentParams") { continue }
        let attach_def = comp_inst.get_instance("AttachDef")?;
        let loc = attach_def.get_instance("Localization")?;
        let name_key = loc.get_str("Name")?.strip_prefix('@')?.to_string();
        let desc_key = loc.get_str("Description")?.strip_prefix('@')?.to_string();
        return Some(LocKeys { name_key, desc_key });
    }
    None
}
```

Every consumer wanting to write or compare against localized text re-implements this walk.

### Proposed shape

Add to `ShipWeapon` and `FpsWeapon`:

```rust
pub struct ShipWeapon {
    // … existing fields …
    /// Localization key for the weapon name (e.g. `"item_NameGATS_BallisticGatling_S1"`).
    /// `@`-prefix already stripped. `None` when no `SAttachableComponentParams.Localization.Name` resolves.
    pub name_key: Option<LocaleKey>,
    /// Same for the description (`item_Desc…`).
    pub desc_key: Option<LocaleKey>,
}
```

`LocaleKey` is already the typed newtype emitted by the generator for `DataType::Locale` fields, so consumers can pass it straight into `LocaleMap::resolve` / `LocaleMap::contains_key`. Both fields default to `None` when the localization chain doesn't resolve — same shape as `ShipWeapon.damage`.

Alternative: surface a free-standing helper `sc_weapons::localization_keys(handle, pools) -> Option<LocKeys>` that consumers call when needed. Cheaper to land than touching the struct, but every consumer pays the lookup separately.

**Recommendation: add as struct fields.** They're cheap to materialize alongside the existing `SAttachableComponentParams` walk in `try_new`, and consumers shouldn't care that "loc keys live in the same component as item type/size."

### Bonus

Lifting this also lets `sc-contracts` resolve weapon names through the same pre-resolved `LocaleKey`s in mission encounter listings (`ShipCandidate.display_name` is already there for ships — consistency).

## 2. `ShipWeapon.penetration_m`

### What's missing

`ShipWeapon.damage` resolves through `AmmoParams → projectileParams → BulletProjectileParams → DamageInfo`. The same `projectileParams` instance carries `penetrationParams.basePenetrationDistance`, which is used in spviewer / Erkul to give players a sense of how thick a hull a weapon punches through. sc-langpatch surfaces it as `Penetration: 12.50m` in the description block.

### Current consumer workaround

```rust
fn legacy_penetration(db: &DataCoreDatabase, guid: Guid) -> Option<f32> {
    let record = db.record(&guid)?;
    let inst = record.as_instance();
    let components = inst.get_array("Components")?;
    for comp in components {
        let comp_inst = to_instance(db, &comp)?;
        if comp_inst.type_name() != Some("SAmmoContainerComponentParams") { continue }
        let Some(Value::Reference(Some(r))) = comp_inst.get("ammoParamsRecord") else { continue };
        let ammo_record = db.record(&r.guid)?;
        return ammo_record.as_instance()
            .get_instance("projectileParams")
            .and_then(|p| p.get_instance("penetrationParams"))
            .and_then(|pen| pen.get_f32("basePenetrationDistance"));
    }
    None
}
```

This duplicates roughly half of `damage::resolve_ammo` for one extra field.

### Proposed shape

Extend `ResolvedAmmo` (private) and surface on `ShipWeapon`:

```rust
pub struct ShipWeapon {
    // … existing fields …
    /// Ammo penetration distance in metres (`AmmoParams → projectileParams → penetrationParams.basePenetrationDistance`).
    /// `None` for weapons with no penetration data (energy beams, projectiles without a `BulletProjectileParams` chain).
    pub penetration_m: Option<f32>,
}
```

Cost: one extra `f32` per weapon in the materialized `Vec<ShipWeapon>`, one extra field read in `extract_damage`. Neither moves the needle on parse time.

### Open question

Penetration semantics may differ between `BulletProjectileParams` and other projectile-params subclasses (`TachyonProjectileParams`, etc., which `extract_damage` already comments out). If they have their own penetration model, the field should remain `None` for those rather than silently zero. Worth confirming before landing.

## 3. Missile / torpedo coverage

### What's missing

`sc-weapons` v1 covers `EItemType::WeaponGun` (ship guns) and `EItemType::WeaponPersonal` (FPS). `EItemType::Missile` is unsupported — `iter_ship_weapons` and `iter_fps_weapons` both filter it out via `WeaponCategory::classify`.

sc-langpatch's `weapon_enhancer` keeps the entire missile path in raw svarog — extraction of explosion damage, GCS speed, lock time / angle / range / tracking signal type, arm time. ~150 lines of code that look very much like the old ship-weapon extraction the migration just retired.

### Proposed shape

A new `Missile` model paralleling `ShipWeapon`, plus an iterator. Two open questions:

**Crate placement.** `sc-weapons` already lives in the workspace and "weapon-shaped data" is the natural fit — but `sc-ammo` is specced (`docs/sc-ammo.md`) for ammo-side extraction, and missiles are arguably ammo-with-a-bus. Pick one:

- A. New `Missile` struct in `sc-weapons` next to `ShipWeapon` / `FpsWeapon`. Consistent with the "weapons crate covers anything you fire."
- B. Land it in `sc-ammo` whenever that crate gets scaffolded. Cleaner taxonomy but blocks on `sc-ammo` work.

**Recommendation: A.** Missiles are weapons end-to-end (size class, manufacturer, mountable) — they fit the `sc-weapons` mental model. `sc-ammo` can model the ammunition side later (penetration, projectile types, splash) without owning the equippable.

### Sketch

```rust
pub struct Missile {
    pub guid: Guid,
    pub record_name: String,
    pub size: i32,
    pub item_sub_type: EItemSubType,            // distinguishes Missile vs Torpedo
    pub manufacturer_guid: Option<Guid>,
    pub name_key: Option<LocaleKey>,
    pub desc_key: Option<LocaleKey>,

    pub damage: Option<DamageSummary>,           // explosion damage, all 6 types
    pub speed: Option<f32>,                      // GCSParams.linearSpeed
    pub arm_time: Option<f32>,                   // armTime

    pub tracking: Option<TrackingProfile>,       // None for unguided
    pub entity_handle: Handle<EntityClassDefinition>,
}

pub struct TrackingProfile {
    pub signal: TrackingSignal,                  // Infrared / Electromagnetic / CrossSection
    pub lock_time: f32,
    pub lock_angle_deg: f32,
    pub lock_range_min_m: f32,
    pub lock_range_max_m: f32,
}

pub enum TrackingSignal {
    Infrared,
    Electromagnetic,
    CrossSection,
    Unrecognized(String),                        // forward compat per workspace convention
}

pub fn iter_missiles(datacore: &Datacore) -> impl Iterator<Item = Missile> + '_ { … }
```

Reusing `DamageSummary` keeps the consumer-side stats-formatting code identical between ships and missiles. `TrackingProfile` is missile-specific and shouldn't bloat `ShipWeapon`.

### Validation targets

- 4.7 LIVE missile + torpedo entity counts (`Missile` + `Torpedo` sub_type) match `sc-langpatch`'s current count.
- Damage / speed / arm time match the values currently shown in-game by sc-langpatch's annotations.
- Tracking signal enum covers every value observed (`Infrared` / `Electromagnetic` / `CrossSection`) without falling through to `Unrecognized` on clean data.

## Priority

In sc-langpatch terms: **all three are nice-to-have, none are blocking the migration.** The raw-svarog workarounds work and are isolated behind clearly-marked `TODO` comments. Landing any of them lets us delete code without changing module behaviour.

Loose ranking by lines-of-code savings:

1. **Missile coverage** — biggest win (~150 lines), largest crate-side work.
2. **Localization keys** — small, but every weapon-touching consumer needs them.
3. **Penetration** — smallest, isolated to one helper.

A reasonable bundling: land 1+2 together (missiles need loc keys anyway), defer 3 to whenever someone touches `damage::resolve_ammo` next.

## Cross-references

- Consumer code with `TODO(sc-holotable)` markers: `E:\vscode\rust\sc-langpatch\src-tauri\src\modules\weapon_enhancer.rs`
- `sc-weapons` model: `crates/sc-weapons/src/ship.rs`, `crates/sc-weapons/src/damage.rs`
- Classification gate that excludes missiles today: `crates/sc-weapons/src/classify.rs`
