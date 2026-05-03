# Feature request — `sc-weapons` gaps surfaced by sc-langpatch migration

> **Consumer:** `sc-langpatch` (`weapon_enhancer` module)
> **Filed:** 2026-04-24
> **Updated:** 2026-05-03 — added §0 (multi-entity-per-loc-key collision) and §4 (FPS coverage status), tightened §1/§2/§3 with evidence from a 4.7-LIVE patched `global.ini`.
> **Status:** Implemented (Phases A–C landed). Localization restructure (prerequisite) landed first per the Resolution plan.
> **Driver:** Migration of `weapon_enhancer` from raw svarog walks to `sc-weapons` v1+v2.

## Background

`sc-langpatch`'s `weapon_enhancer` module annotates `global.ini` with weapon stats — size prefix on the name, damage / penetration / projectile speed / ammo / capacitor on the description. The module was migrated to consume `sc_weapons::iter_ship_weapons(&datacore)` so that weapon data extraction lives in one canonical crate (workspace rule §3 — one canonical model per domain).

The migration successfully moved ammo resolution, damage extraction, and capacitor sizing into `sc-weapons`. The items below are the remaining gaps. **Section 0 is new and now the highest-priority item** — without it, the consumer produces visibly broken output.

## 0. Multi-entity-per-loc-key collision (NEW, blocking quality)

### What's broken in production today

Multiple `EntityClassDefinition` records frequently share a single INI localization key. The `weapon_enhancer` module iterates entities and pushes a `Prefix("S{size} ")` op per entity, plus a `Suffix(stats_block)` op per entity. Because the patcher stacks ops per key (see `apply_ops` in `sc-langpatch/src-tauri/src/merge.rs`), the rendered string accumulates one prefix and one stats block **per colliding entity**. Verified in the 4.7 LIVE build `11715810` patched output (`global_4.7.0-live.11715810_fe3f0f6f_20260502155413_modified.ini`):

```
item_NameAMRS_ScatterGun_S3              = S3 S3 PyroBurst Scattergun
item_NameAPAR_BallisticGatling_S4        = S3 S4 S4 S6 Revenant Gatling
item_NameBEHR_LaserCannon_S6             = S6 S6 S6 M8A Cannon
item_NameBEHR_LaserCannon_S7             = S7 S9 S7 S7 S7 S7 S7 S7 S7 S12 S8 S9 M9A Cannon   ← 12 entities
item_NameGATS_BallisticCannon_S3         = S3 S7 S8 Tarantula GT-870 Mark 3 Cannon
item_NameGATS_BallisticGatling_S1        = S1 S1 S1 S1 S1 YellowJacket GT-210 Gatling
item_NameKLWE_LaserRepeater_S5           = S5 S5 S5 S5 CF-557 Galdereen Repeater
item_NameMISL_S02_CS_FSKI_Tempest        = S2 [CS] S2 [CS] Tempest II Missile
```

The matching `item_Desc…` keys are equally affected — `item_DescBEHR_LaserCannon_S7` ends up with **twelve** consecutive `<EM4>Weapon Stats</EM4>` blocks, each with a different size / damage / ammo profile because each colliding entity contributes its own.

### Root cause

`sc_weapons::iter_ship_weapons` returns one `ShipWeapon` per `EntityClassDefinition` record. INI keys are owned by the *item display name*, not the entity, and CIG ships several variants (different mount points, different sizes, different ammo containers) under one display name. Without a way to group/dedupe by display name the consumer can't decide what to write.

This is the single largest correctness issue blocking the migration from being shippable as-is. The pre-migration code had the same bug — the migration didn't introduce it — but the migration is the right time to address it because the fix needs to live next to the iterator that produces the duplicates.

### Possible shapes (pick one)

**A. Iterator-level dedup with a collision policy.**

```rust
pub enum LocKeyCollision {
    /// First entity wins (current implicit behaviour if consumers naively dedup).
    First,
    /// Entity with the lowest record-name lexicographic order (deterministic).
    Canonical,
    /// Entity whose size matches the size implied by the loc-key suffix
    /// (e.g. `_S1` → size 1). Falls back to `Canonical` if no suffix.
    SizeMatchingSuffix,
}

pub fn iter_ship_weapons_by_loc_key(
    datacore: &Datacore,
    policy: LocKeyCollision,
) -> impl Iterator<Item = ShipWeapon> + '_ { … }
```

Pros: consumer code stays one-liner-clean. Cons: bakes a policy choice into the crate; consumers wanting "show all variants in one description" can't.

**B. Group-by helper.**

```rust
pub fn ship_weapons_by_loc_key(
    datacore: &Datacore,
) -> HashMap<LocaleKey, Vec<ShipWeapon>> { … }
```

Pros: consumer keeps full information and decides the collapse rule per use case. sc-langpatch can pick `vec[0]` for naive dedup, or render `S1-S3` ranges, or skip keys with conflicting damage. Cons: HashMap allocation cost on every call; consumers without a dedup need still pay it.

**C. Just expose loc keys (do nothing else).** Section 1 below already requests this. With it landed, sc-langpatch can dedup on its own: build a `HashMap<LocaleKey, ShipWeapon>` keeping the first entity per key. This is the **minimum viable fix**.

**Recommendation: ship C now (it's already in scope as §1) and add B in a follow-up if a second consumer materializes.** Avoid A — collision policy is a UI concern, not a model concern.

### Validation targets for whichever shape lands

After the fix, the patched 4.7 LIVE output should show exactly one `S{n} ` prefix per `item_Name…` key and exactly one `<EM4>Weapon Stats</EM4>` block per `item_Desc…` key. The 8 keys listed above are the canary set — if any of them still doubles, the fix is incomplete.

## 1. Localization keys on `ShipWeapon` / `FpsWeapon` / `Missile`

### What's missing

`ShipWeapon` exposes `record_name` and `entity_handle` but no localization-key fields. To patch a weapon's INI entry, sc-langpatch needs the `Name` / `Description` keys from `SAttachableComponentParams.AttachDef.Localization`. These are the strings ending up in `global.ini` (e.g. `item_NameGATS_BallisticGatling_S1`).

Same applies to `FpsWeapon` (lower urgency — sc-langpatch doesn't currently annotate FPS weapons but will once §4 lands) and the proposed `Missile` model (§3).

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

Add to `ShipWeapon`, `FpsWeapon`, and `Missile`:

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

### Cross-benefit

- Lifting this also lets `sc-contracts` resolve weapon names through the same pre-resolved `LocaleKey`s in mission encounter listings (`ShipCandidate.display_name` is already there for ships — consistency).
- It's also the prerequisite for §0's minimum-viable fix.

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

### Note from production output

The raw-svarog implementation already produces the right data when there's no collision, e.g.:

```
item_NameGMISL_S05_IR_TALN_Valkyrie = S5 [IR] Valkyrie V-G Missile
```

But it suffers from the same multi-entity-per-key collision as ship weapons (see §0):

```
item_NameMISL_S02_CS_FSKI_Tempest = S2 [CS] S2 [CS] Tempest II Missile
```

And it surfaces an interesting data-quality observation worth preserving when the missile model lands: the entity name is sometimes inconsistent with the resolved `trackingSignalType`. In the 4.7 LIVE build:

```
item_NameGMISL_S03_EM_FSKI_Thunderbolt = S3 [CS] Thunderbolt III-G Missile
                              ^^                ^^
                              entity name says EM, DCB tracking signal says CS
```

The DCB value is authoritative (it's what the game actually uses for lock behaviour), but consumers should be aware that record-name parsing for tracking signal is unreliable.

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
    pub name_key: Option<LocaleKey>,            // §1
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
- Whichever §0 fix lands, also applies to `iter_missiles`.

## 4. FPS weapon annotation parity (status note)

`sc-langpatch`'s `weapon_enhancer` does not currently touch FPS weapons — only ship weapons and missiles. There's no immediate feature request here, but flagging for awareness:

- `FpsWeapon` lives next to `ShipWeapon` in `sc-weapons` and exposes very similar fields. When sc-langpatch grows an FPS-stats annotation (likely; the data is already accessible), it will hit §0 and §1 the same way ship weapons do. Whatever fix lands for §0/§1 should be uniformly applicable to `FpsWeapon`.

## Priority

Updated after 2026-05-03 review:

1. **§0 + §1** — bundled. §1 unblocks the minimum-viable §0 fix. Currently produces visibly broken in-game text; users notice. **Should land before sc-langpatch ships the migrated `weapon_enhancer`.**
2. **§3 (missiles)** — biggest LoC win in the consumer (~150 lines), needs §1 anyway. Also has §0-class collisions. Bundle with §0/§1 if convenient.
3. **§2 (penetration)** — smallest, isolated to one helper. Defer to whenever someone touches `damage::resolve_ammo` next.

The previous loose ranking ("missiles first") is superseded — §0 is now the most user-visible problem.

## Resolution plan (agreed 2026-05-03)

Reviewing the request alongside existing patterns in the workspace surfaced
that §0 and §1 are an instance of a more general localization-shape problem
already half-solved by `sc-extract`'s `DisplayNameCache` and `sc-contracts`'
`Mission.{title_key, description_key}` + `MissionPools` pair. Rather than
add ad-hoc `name_key` / `desc_key` fields to `ShipWeapon` in isolation,
the agreed plan lifts a small convention to the workspace level and lets
`sc-weapons` consume it. **The `sc-weapons` work blocks on the
localization restructure — that lands first, in a separate change.**

### Localization restructure (separate, lands first)

Lives in `sc-extract` + meaningful changes to `sc-contracts`. Tracked
separately from this feature request; summarized here so the dependency
is obvious.

**Driving constraint** — `sc-langpatch` overlays a community language
pack on top of base English *after* `Datacore::parse` has run, then
rebuilds a `LocaleMap` from the overlaid INI and hands it to modules
([sc-langpatch/src-tauri/src/lib.rs:316-336](file:///E:/vscode/rust/sc-langpatch/src-tauri/src/lib.rs)).
Anything pre-resolved at parse time is locked to base English and goes
silently wrong when the overlay differs. The acknowledged note in
`lib.rs` ("Acceptable until / unless we add a mechanism to rebuild the
cache against the post-overlay locale") is what this restructure
removes.

#### The rule

> **Stored data carries `LocaleKey`s. Strings are resolved at the
> call site through whatever `LocaleMap` is current.**

No pre-resolved string lives on a domain struct or in a workspace
cache. Resolution is always lazy.

#### 1. `DisplayNameCache` → `LocalizedItemCache` (keys only)

```rust
pub struct LocalizedItem {
    pub name_key: Option<LocaleKey>,        // raw — '@' kept
    pub short_name_key: Option<LocaleKey>,
    pub desc_key: Option<LocaleKey>,
}
```

- Walk happens once at parse — locale-independent.
- Lives at `datacore.snapshot().localized_items` (rename from
  `display_names`). `DisplayNameCache` is removed outright, no shim.
- `display_features` (`SCExtendedLocalizationLevelParams` — lore
  blurbs, marketing callouts, hero asset paths, hover audio) is
  **not** in the cache. It's sparse, heterogeneous, and only relevant
  to detail-card UIs. Reachable via the typed model
  (`SItemDefinition.localization → display_features`); a sibling
  cache can land later if a third consumer ever needs it bulked up.

#### 2. `LocaleKey` is raw, always

Generated code emits `LocaleKey` with the leading `@` intact;
downstream code keeps it that way. Resolution goes through
`LocaleMap::resolve(&LocaleKey)`, which handles `@` transparently.
`LocaleKey::stripped()` exists for the rare INI-write call site that
needs the bare key.

#### 3. Drop pre-resolved strings from `sc-contracts`

| Field | Action |
|---|---|
| `Mission.title: Option<String>` | drop |
| `Mission.description: Option<String>` | drop |
| `Mission.has_runtime_substitution: bool` | drop — derive at call site |
| `Mission.{title_key, description_key}` | keep, raw (`@` restored) |
| `ShipEntity.display_name: String` | drop |
| `ShipCandidate.display_name: String` | drop |
| `BlueprintPoolItem.display_name: String` | drop |
| `LocationRef.display_name: String` | drop |
| `LocationRef.body_display_name: String` | drop |
| `RewardCurrency.display_name: String` | drop |

`resolve_contract_text` keeps the four-level inheritance walk
(sub-contract → contract paramOverrides → handler contractParams →
template) but returns only keys. The expansion pass becomes
locale-independent — no `&LocaleMap` needed during build.

#### 4. Resolution is via methods on the domain types

```rust
impl Mission {
    pub fn title<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> { … }
    pub fn description<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> { … }
    pub fn has_runtime_substitution(&self, locale: &LocaleMap) -> bool { … }
}
impl ShipCandidate { pub fn display_name<'a>(&self, locale: &'a LocaleMap, cache: &'a LocalizedItemCache) -> Option<&'a str> { … } }
impl LocationRef    { pub fn display(&self, locale: &LocaleMap) -> String { … } }
// etc.
```

Methods discover well via IDE and read naturally at the call site.
`LocaleMap::resolve` returns `Option<&str>` — zero allocation on the
hot path.

#### 5. Sort order

Currently `ShipRegistry::build`, blueprint-pool sort, etc. sort by
the pre-resolved `display_name` at build time. After the restructure
they sort by a locale-independent key (record name / debug name /
size+guid). UIs that want a localized sort do it post-resolve at
render time.

#### 6. Pattern boundaries (intentional, not a gap)

- *Entity-localization* — single-walk, one `Localization` block per
  entity. Lives in `LocalizedItemCache`. Used by `sc-weapons`,
  future `bulkhead`, anything pointing at an item entity.
- *Domain-text resolution* — multi-level inheritance walks (contract
  four-level chain, future analogues). Stays per-domain;
  `sc-contracts` keeps `resolve_contract_text`. Both patterns
  produce **keys only**; resolution is the call site's job.

#### 7. Convention doc

A short `docs/localization.md` describes the rule, the cache shape,
the method-on-domain-type pattern, and the "why pre-resolved goes
stale" rationale so future domain crates land it consistently.

### sc-weapons work (this feature request, lands after the restructure)

#### Phase A — §0 + §1 collapse into a single shape

Once `LocalizedItemCache` is available, `ShipWeapon` / `FpsWeapon` /
(new) `Missile` add two key fields populated from the cache during
`try_new`:

```rust
pub name_key: Option<LocaleKey>,        // raw, with '@'
pub desc_key: Option<LocaleKey>,        // raw
```

No `display_name: String` — that violates the keys-only rule.
Consumers resolve via a method:

```rust
impl ShipWeapon {
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> { … }
}
```

Plus a per-domain pools struct mirroring `MissionPools`:

```rust
pub struct WeaponPools {
    pub name_key: HashMap<LocaleKey, Vec<Guid>>,
    pub desc_key: HashMap<LocaleKey, Vec<Guid>>,
}
pub fn weapon_pools(weapons: &[ShipWeapon]) -> WeaponPools { … }
```

This is the §0 fix: consumers dedup via `pools.name_key.iter().map(|(_, ids)| ids[0])`
or whatever policy they want. No collision-policy enum baked into the
crate.

#### Phase B — §3 missile model

Unchanged from the original proposal except for two refinements:

- Reuse `ESignatureType` directly for tracking signal — the generated
  enum already covers `Infrared` / `Electromagnetic` / `CrossSection`
  with an `Unrecognized` fallback. No parallel `TrackingSignal` enum.
- `find_component` (in `damage.rs`) gains a `SCItemMissileParams` arm.
- `Missile` consumes `LocalizedItemCache` for `name_key` / `desc_key`
  (keys only) and resolves via a `display_name(&locale)` method, same
  shape as ship and FPS weapons.

#### Phase C — §2 penetration

Unchanged: add `penetration_m: Option<f32>` to `ShipWeapon`, populated
from `BulletProjectileParams.penetration_params.base_penetration_distance`.
`None` for non-bullet projectiles (matches existing damage coverage).
`TachyonProjectileParams.penetration_params` carries the same field —
defer until that projectile family also gets damage modeled.

### Updated priority

1. **Localization restructure** — blocks everything else here. ✅ landed.
2. **Phase A (§0 + §1 via `WeaponPools` + cache fields)** — required for
   sc-langpatch's `weapon_enhancer` migration to ship. ✅ landed.
3. **Phase B (§3 missiles)** — biggest LoC win in the consumer.
   ✅ landed.
4. **Phase C (§2 penetration)** — small, isolated, defer. ✅ landed
   (also lifted onto `FpsWeapon` for symmetry — same `BulletProjectileParams`
   chain extracts identically).

### Landed shape

The implementation landed exactly along the lines proposed in the
Resolution plan. Notable details:

- `ShipWeapon` and `FpsWeapon` both carry
  `name_key: Option<LocaleKey>` / `desc_key: Option<LocaleKey>` (raw,
  `@`-prefixed). Methods `display_name(&LocaleMap)` /
  `description(&LocaleMap)` resolve at the call site.
- `Missile` model lives in `crates/sc-weapons/src/missile.rs`. Same
  field/method shape as ships and FPS, with explosion-damage
  reaching through `SCItemMissileParams.explosion_params` and an
  optional `TrackingProfile` (reusing the generated
  `ESignatureType` enum directly).
- `WeaponPools::build` is **unified** across all three families —
  `(ships, fps, missiles)` slices in, `LocaleKey → Vec<Guid>` pools
  out. sc-langpatch dedupes via `pools.name_key.iter().map(|(_, ids)| ids[0])`
  or whatever policy it wants.
- `iter_missiles` ships at the same level as `iter_ship_weapons` /
  `iter_fps_weapons`. A helper [`build_weapon_pools`] materializes
  all three lists + builds the unified pool in a single call.
- `penetration_m: Option<f32>` on `ShipWeapon` and `FpsWeapon`
  resolves through `BulletProjectileParams.penetration_params.basePenetrationDistance`.
  Tachyon and other projectile families stay `None` until their
  damage path also lands.
- `try_new` arg lists kept their original shape; `LocalizedItemCache`
  joined as one extra parameter rather than collapsing the existing
  maps into a context struct (deferrable).
- Missile `try_new` takes the same parameter shape as ship/FPS for
  symmetry, including `ecd_map` (currently unused — reserved for
  the same future revisions that touch ammo lookups).

## Cross-references

- Consumer code with `TODO(sc-holotable)` markers: `E:\vscode\rust\sc-langpatch\src-tauri\src\modules\weapon_enhancer.rs`
- Patched-output evidence for §0: `D:\Users\Valerie\AppData\Local\sc-langpatch\debug\global_4.7.0-live.11715810_fe3f0f6f_20260502155413_modified.ini`
- Patcher op-stacking semantics: `sc-langpatch/src-tauri/src/merge.rs` → `apply_ops`
- `sc-weapons` model: `crates/sc-weapons/src/ship.rs`, `crates/sc-weapons/src/damage.rs`
- Classification gate that excludes missiles today: `crates/sc-weapons/src/classify.rs`
- Existing pattern A reference: `crates/sc-extract/src/display_names.rs`
- Existing pattern B reference: `crates/sc-contracts/src/titles.rs`, `crates/sc-contracts/src/pools.rs`
