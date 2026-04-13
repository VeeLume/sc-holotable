# `sc-weapons` — design specification

> Status: **proposal, awaiting review.** Depends on `docs/sc-extract.md`, `docs/codegen.md`, and `docs/sc-ammo.md`.

> **Flat-pool API update (post-refactor).** The generated types no longer store nested `Class` / `StrongPointer` / `WeakPointer` fields as `Option<Box<T>>`. They store `Option<TId>` / `Vec<TId>` handles into per-type `Vec<Option<T>>` pools living in `DataPools`. The code sketches below were written against the old recursive model and show `raw.components.iter()` / `attach.attach_def.as_ref()?` patterns — those still illustrate the intent, but the real implementation will thread a `&DataPools` through every accessor. Where the sketch shows `raw.components` (a materialised `Vec<DataForgeComponentParams>`), the real code will do `raw.components.iter().filter_map(|id| id.get(pools))` — same shape at the ergonomics layer. See `docs/codegen.md` §"Reference semantics — flat-pool model" for the generated API surface. When this crate actually lands, this header will be rewritten against the real types.

## Purpose

`sc-weapons` provides hand-written domain wrappers around `EntityClassDefinition` records that represent weapons — ship-mounted and FPS — plus weapon-intrinsic calculations (fire rate, alpha damage, DPS, sustained DPS, time-to-overheat). It is `bulkhead`'s forcing function for correctness: every field bulkhead's damage simulator reads must be accessible through this crate.

It is deliberately scoped **narrowly**. Weapons and ammo only. Ships, shields, armor, radars, and the damage pipeline are separate concerns, owned by other crates (or still in bulkhead for now).

## Consumers

| Crate / app | What it uses |
|---|---|
| `bulkhead` (future) | The complete weapon-intrinsic calc surface. Full weapon data for the damage-pipeline composition point. **Drives correctness — the forcing function.** |
| `sc-langpatch` (future) | A narrow subset — display names, class / size / grade, for the label-enrichment module. Reads a few fields off the same canonical wrapper. |
| `streamdeck-starcitizen` | Does **not** use `sc-weapons`. It only reads keybind definitions, not weapon stats. |

## Scope

**What `sc-weapons` owns:**

- `ShipWeapon` — wrapper around `EntityClassDefinition` records that have `SCItemWeaponComponentParams` in their `Components` array and are flagged as ship-mounted (`type = WeaponGun`, `subtype in {Gun, Rocket}`).
- `FpsWeapon` — wrapper around `EntityClassDefinition` records flagged as FPS (`type = WeaponPersonal`, `subtype != Gadget`).
- A `WeaponCore` trait implemented by both, providing the overlap: display name, class name, manufacturer ref, alpha damage, basic fire-rate access.
- Specialization traits for weapon sub-categories: `BallisticWeapon`, `EnergyWeapon` — implemented on `ShipWeapon` and returning `Option` for weapons that don't match the category.
- Weapon-intrinsic calculations: fire rate, alpha damage (delegated to `sc-ammo`), theoretical DPS, sustained DPS (ballistic-heat model + energy-capacitor model), time-to-overheat.
- `try_from_raw` constructors that validate the record shape before wrapping.

**What `sc-weapons` does *not* own:**

- The raw `EntityClassDefinition`, `SCItemWeaponComponentParams`, `SWeaponConnectionParams`, or any DCB structure — those live in `sc_extract::generated`.
- Ammo data or damage sums — those live in `sc-ammo`, and `sc-weapons` delegates to it.
- Damage-pipeline calculations (projectile → shield → armor → hull) — bulkhead composes these from weapon + target data.
- Ship loadout resolution or hardpoint assignment — that will be `sc-ships` later.
- Playable-content filtering — `sc-extract::is_playable_weapon` returns a predicate; `sc-weapons` doesn't filter during construction.

## Two wrapper types, one shared trait

Ship weapons and FPS weapons have enough divergence in their calc logic that forcing them into one type leads to branching at every call site. But they share a core of access patterns (display name, manufacturer, alpha damage) and we want consumers that only need the core to work generically. The pattern:

```rust
// crates/components/sc-weapons/src/lib.rs
use sc_extract::{
    Guid,
    generated::{EntityClassDefinition, SCItemWeaponComponentParams, DataForgeComponentParams,
                SAttachableComponentParams},
};
use sc_ammo::Ammo;

// ── Shared trait ────────────────────────────────────────────────────────

/// Operations that apply to any weapon — ship or FPS.
pub trait WeaponCore {
    fn class_name(&self) -> &str;
    fn display_name<'a>(&self, cache: &'a DisplayNameCache) -> Option<&'a str>;
    fn manufacturer_ref(&self) -> Option<Guid>;
    fn size(&self) -> u32;
    fn grade(&self) -> Option<u32>;

    /// Alpha damage per shot, summed across direct + explosion.
    fn alpha_damage(&self, ammo: &Ammo) -> f32;

    /// Shots per minute.
    fn fire_rate_rpm(&self) -> f32;

    /// Theoretical DPS ignoring heat, energy, and ammo limits.
    /// `fire_rate_rpm / 60 * alpha_damage`.
    fn theoretical_dps(&self, ammo: &Ammo) -> f32 {
        (self.fire_rate_rpm() / 60.0) * self.alpha_damage(ammo)
    }
}

// ── Ship weapon wrapper ────────────────────────────────────────────────

/// A ship-mounted weapon. Constructed from an `EntityClassDefinition`
/// record that has been verified to contain `SCItemWeaponComponentParams`
/// and to match the ship-weapon type/subtype classification.
#[derive(Debug, Clone)]
pub struct ShipWeapon(EntityClassDefinition);

impl ShipWeapon {
    /// Attempt to wrap a raw record as a ship weapon.
    /// Returns `None` if the record doesn't have weapon components, or if
    /// its type/subtype doesn't match the ship-weapon classification.
    pub fn try_from_raw(raw: EntityClassDefinition) -> Option<Self> {
        // Must have SCItemWeaponComponentParams in Components
        let has_weapon_params = raw.components.iter().any(|c| {
            matches!(c, DataForgeComponentParams::SCItemWeaponComponentParams(_))
        });
        if !has_weapon_params {
            return None;
        }

        // Must have SAttachableComponentParams with Type="WeaponGun" and
        // SubType in {Gun, Rocket}
        let attach = Self::find_attachable(&raw)?;
        let attach_def = attach.attach_def.as_ref()?;
        let matches_ship = attach_def.r#type == "WeaponGun"
            && matches!(attach_def.sub_type.as_str(), "Gun" | "Rocket");
        if !matches_ship {
            return None;
        }

        Some(ShipWeapon(raw))
    }

    pub fn raw(&self) -> &EntityClassDefinition {
        &self.0
    }

    pub fn into_raw(self) -> EntityClassDefinition {
        self.0
    }

    // ── Private accessors into the components array ──

    fn weapon_params(&self) -> &SCItemWeaponComponentParams {
        self.0.components.iter().find_map(|c| match c {
            DataForgeComponentParams::SCItemWeaponComponentParams(w) => Some(w),
            _ => None,
        }).expect("verified at construction")
    }

    fn find_attachable(raw: &EntityClassDefinition) -> Option<&SAttachableComponentParams> {
        raw.components.iter().find_map(|c| match c {
            DataForgeComponentParams::SAttachableComponentParams(a) => Some(a),
            _ => None,
        })
    }

    fn attachable(&self) -> &SAttachableComponentParams {
        Self::find_attachable(&self.0).expect("verified at construction")
    }
}

impl WeaponCore for ShipWeapon {
    fn class_name(&self) -> &str {
        &self.0.class_name
    }

    fn display_name<'a>(&self, cache: &'a sc_extract::DisplayNameCache) -> Option<&'a str> {
        cache.get(&self.0.record_id)
    }

    fn manufacturer_ref(&self) -> Option<Guid> {
        self.attachable()
            .attach_def
            .as_ref()?
            .manufacturer
            .as_ref()
            .map(|r| r.guid)
    }

    fn size(&self) -> u32 {
        self.attachable()
            .attach_def
            .as_ref()
            .map(|a| a.size)
            .unwrap_or(0)
    }

    fn grade(&self) -> Option<u32> {
        self.attachable().attach_def.as_ref().map(|a| a.grade)
    }

    fn alpha_damage(&self, ammo: &Ammo) -> f32 {
        ammo.alpha_damage_scalar()
    }

    fn fire_rate_rpm(&self) -> f32 {
        // Walks fire_actions[0] → depends on variant (Rapid / Single / Sequence)
        // See the fire_actions helper below.
        self.primary_fire_action_rpm()
    }
}
```

```rust
// ── FPS weapon wrapper ──────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FpsWeapon(EntityClassDefinition);

impl FpsWeapon {
    pub fn try_from_raw(raw: EntityClassDefinition) -> Option<Self> {
        let has_weapon_params = raw.components.iter().any(|c| {
            matches!(c, DataForgeComponentParams::SCItemWeaponComponentParams(_))
        });
        if !has_weapon_params {
            return None;
        }

        let attach = Self::find_attachable(&raw)?;
        let attach_def = attach.attach_def.as_ref()?;
        let matches_fps = attach_def.r#type == "WeaponPersonal"
            && attach_def.sub_type != "Gadget";
        if !matches_fps {
            return None;
        }

        Some(FpsWeapon(raw))
    }

    // ... same structure as ShipWeapon, with FPS-specific accessors
    //     (magazine size, reload time, etc.) instead of heat/regen.
}

impl WeaponCore for FpsWeapon {
    // ... parallel impl
}
```

### Why two types, not one

The calc logic for ship and FPS weapons genuinely diverges at the sustain-model level:

- Ship weapons have **heat models** (ballistic) or **energy capacitor models** (lasers) for sustain.
- FPS weapons have **magazines** and **reload time**. No heat. No capacitor.

Forcing them into one type means every sustain calculation starts with `if self.is_ship_weapon() { ... } else { ... }`. Splitting the types moves that dispatch to the type level, so each `impl` block only deals with the fields it knows exist.

The shared `WeaponCore` trait covers the intersection — display name, alpha damage, raw fire-rate access — and lets consumers that don't care about sub-category (e.g., sc-langpatch's label enrichment) work generically via `fn enrich_label<W: WeaponCore>(w: &W, ...)`.

## Sub-categorization traits for ship weapons

Ship weapons come in two sustain flavors and one special case:

- **Ballistic** — have a `simplified_heat_params` field. Fire until overheat, lockout during cooling, repeat.
- **Energy (capacitor)** — have a `weapon_regen_consumer_params` field. Fire a burst (`max_ammo_load` shots), wait for capacitor regen, repeat.
- **Distortion** — damage lives in explosion params, not direct hit. Handled at the `sc-ammo` layer (`Ammo::is_explosion_primary()`), not as a weapon-level category, because distortion-ness is really an ammo property.

Two specialization traits, one per sustain model:

```rust
/// Ship weapons that use a heat model for sustain. Implemented on `ShipWeapon`
/// but all methods return `Option` because a given weapon may be energy-based
/// instead of ballistic.
pub trait BallisticWeapon {
    fn heat_per_shot(&self) -> Option<f32>;
    fn cooling_per_second(&self) -> Option<f32>;
    fn overheat_threshold(&self) -> Option<f32>;
    fn overheat_recovery_seconds(&self) -> Option<f32>;

    /// Sustained DPS under the ballistic heat model — computed from
    /// fire_rate, heat_per_shot, cooling_per_second, and overheat_recovery.
    /// Picks the better of two firing strategies (overheat-cycle vs
    /// managed-cooling) and returns the higher DPS.
    /// Returns `None` if this weapon doesn't use a heat model.
    fn sustained_dps_ballistic(&self, ammo: &Ammo) -> Option<f32>;

    /// Time until the weapon overheats from cold, firing continuously.
    /// `None` if no heat model.
    fn time_to_overheat(&self) -> Option<Duration>;
}

/// Ship weapons that use a capacitor regen model for sustain.
pub trait EnergyWeapon {
    fn energy_per_shot(&self) -> Option<f32>;
    fn max_burst_shots(&self) -> Option<u32>;
    fn regen_cooldown_seconds(&self) -> Option<f32>;
    fn max_regen_per_second(&self) -> Option<f32>;

    /// Sustained DPS under the energy capacitor model — computed from
    /// burst shots, regen cooldown, and max regen per second.
    /// Returns `None` if this weapon doesn't use a capacitor model.
    fn sustained_dps_energy(&self, ammo: &Ammo) -> Option<f32>;
}

impl BallisticWeapon for ShipWeapon {
    fn heat_per_shot(&self) -> Option<f32> {
        self.weapon_params()
            .connection_params
            .simplified_heat_params
            .as_ref()
            .map(|h| h.heat_per_shot)
    }

    // ... and similar for the other fields
    // All return None if simplified_heat_params is absent.
}

impl EnergyWeapon for ShipWeapon {
    fn energy_per_shot(&self) -> Option<f32> {
        self.weapon_params()
            .weapon_regen_consumer_params
            .as_ref()
            .map(|r| r.regeneration_cost_per_bullet)
    }

    // ... and similar
}
```

### Why `Option` returns instead of conditional trait impls

A single `ShipWeapon` type can be either ballistic or energy at runtime. Conditional trait impls (only implement `BallisticWeapon` if the heat-params field is present) aren't expressible in Rust — trait impls are decided at the type level, not per-instance.

The alternative would be separate types (`BallisticShipWeapon`, `EnergyShipWeapon`) constructed via separate `try_from_raw` methods that check the distinguishing field. This is cleaner per-type but creates the same problem we wanted to avoid: consumers have to dispatch on weapon kind at every call site. "One `ShipWeapon` type, traits return `Option`" is the pragmatic middle.

Consumers that need to filter to just ballistic weapons do:

```rust
for w in data.all_ship_weapons() {
    if w.heat_per_shot().is_some() {
        // ballistic
    }
}
```

## Convenience discovery

Walking `RecordStore` and wrapping each weapon is a common operation. `sc-weapons` provides helpers:

```rust
impl ShipWeapon {
    /// Iterator over all ship weapons in the snapshot.
    pub fn all_in<'a>(snap: &'a sc_extract::DatacoreSnapshot) -> impl Iterator<Item = Self> + 'a {
        snap.records
            .all_entity_classes()
            .filter_map(|(_, ecd)| Self::try_from_raw(ecd.clone()))
    }
}

impl FpsWeapon {
    pub fn all_in<'a>(snap: &'a sc_extract::DatacoreSnapshot) -> impl Iterator<Item = Self> + 'a {
        snap.records
            .all_entity_classes()
            .filter_map(|(_, ecd)| Self::try_from_raw(ecd.clone()))
    }
}
```

(`all_entity_classes` is spec shorthand for whichever typed-iteration helper the flat-pool `RecordStore` ends up exposing; the point is that the input is a `DatacoreSnapshot`, not the old `ExtractedData` god-struct.)

A consumer that only wants ballistic ship weapons:

```rust
for weapon in ShipWeapon::all_in(&data) {
    if weapon.heat_per_shot().is_some() {
        let dps = weapon.sustained_dps_ballistic(&ammo).unwrap();
        println!("{}: {} sustained DPS", weapon.class_name(), dps);
    }
}
```

## The calc functions in detail

The sustain calculations are the real value this crate adds over raw field access. They live in hand-written `impl` blocks on `ShipWeapon`:

### Ballistic sustain model

```rust
impl ShipWeapon {
    fn sustained_dps_ballistic_impl(&self, ammo: &Ammo) -> Option<f32> {
        let heat = self.weapon_params().connection_params.simplified_heat_params.as_ref()?;
        let rpm = self.fire_rate_rpm();
        let alpha = self.alpha_damage(ammo);

        let shots_per_sec = rpm / 60.0;
        let heat_rate = shots_per_sec * heat.heat_per_shot;

        // Overheat cycle: fire until overheat, wait for cooling, repeat.
        // Strategy A: fire continuously, hit overheat, wait `overheat_fix_time`.
        let time_to_overheat = heat.overheat_threshold / (heat_rate - heat.cooling_per_second);
        let shots_before_overheat = time_to_overheat * shots_per_sec;
        let cycle_duration = time_to_overheat + heat.overheat_fix_time;
        let dps_a = (shots_before_overheat * alpha) / cycle_duration;

        // Strategy B: fire at a rate that matches cooling exactly (no overheat).
        let managed_rate = heat.cooling_per_second / heat.heat_per_shot;
        let dps_b = managed_rate * alpha;

        Some(dps_a.max(dps_b))
    }
}
```

### Energy sustain model

```rust
impl ShipWeapon {
    fn sustained_dps_energy_impl(&self, ammo: &Ammo) -> Option<f32> {
        let regen = self.weapon_params().weapon_regen_consumer_params.as_ref()?;
        let rpm = self.fire_rate_rpm();
        let alpha = self.alpha_damage(ammo);

        let burst_shots = regen.max_ammo_load as f32;
        let burst_duration = burst_shots / (rpm / 60.0);
        let refill_duration = regen.regeneration_cooldown
            + burst_shots / regen.max_regen_per_sec;

        let total_cycle = burst_duration + refill_duration;
        Some((burst_shots * alpha) / total_cycle)
    }
}
```

Both of these are adapted from bulkhead's `docs/damage-system.md` and will be verified against real data before being treated as correct. **The "go slow" principle applies here above all else** — incorrect sustain formulas in a shared lib would propagate wrong numbers to every consumer.

## Design decisions

### Two wrapper types, not one

Ship and FPS weapons have fundamentally different sustain models (heat/energy vs magazine/reload). Forcing them into one type means every calc starts with a branch. Splitting at the type level is cleaner.

### `WeaponCore` trait for the overlap

Consumers that only care about display-name and alpha damage can work generically via `W: WeaponCore`. Consumers that care about specific sustain models reach for `BallisticWeapon` or `EnergyWeapon` explicitly, and get `Option` semantics for weapons that don't match.

### Delegation to `sc-ammo` for damage

`sc-weapons` never sums damage values itself. It always goes through `Ammo::alpha_damage_scalar()` or `Ammo::total_damage()`. This keeps the "distortion-weapons-have-~0-direct-damage" trap isolated to one place — the `sc-ammo` crate.

### Inherent methods + specialization traits, no trait for the base struct

`ShipWeapon` has a mix of inherent methods (for construction and private component access) and trait implementations (for the public `WeaponCore` + `BallisticWeapon` + `EnergyWeapon` APIs). There is no `Weapon` trait implemented on `ShipWeapon` — the *struct* is named `ShipWeapon`, and the general-purpose trait is `WeaponCore`. No name collision.

### `try_from_raw` validates at construction

The validation logic (checking that `SCItemWeaponComponentParams` is present, that `type` and `subtype` match) is the single canonical place for "is this a ship weapon?". Once `ShipWeapon::try_from_raw` returns `Some(_)`, every accessor can `expect` the relevant components exist without re-checking. No defensive `unwrap_or` inside accessors.

### No ownership of ammo data

`ShipWeapon` doesn't hold an `Ammo`. It holds a raw `EntityClassDefinition`, and the `SAmmoContainerComponentParams.ammo_params_record` GUID inside tells the consumer which ammo record to look up. Consumers call `data.records.get_ammo(&weapon.ammo_ref())` and pass the result into `alpha_damage(&ammo)`. This keeps the weapon wrapper small and avoids eagerly cloning ammo into every weapon.

Trade-off: every DPS calculation requires passing the ammo explicitly. For UI rendering that shows DPS for 500 weapons at once, consumers can pre-fetch the ammo map and reuse it.

## What `sc-weapons` does *not* provide

- Raw DCB types — those are in `sc_extract::generated`.
- Ammo damage summing — that's `sc-ammo`.
- Damage pipeline calculations (shield → armor → hull) — that stays in `bulkhead` until `sc-ships` exists.
- Snapshot I/O — that's `sc-extract`.
- Ship hardpoint resolution — that will be `sc-ships`.
- Loadout management — that will be `sc-ships` or a consumer.
- Weapon filtering rules — that's `sc_extract::is_playable_weapon`.
- Localization resolution — consumers pass a `&DisplayNameCache` into `display_name()`.

## Open questions

1. **Does `ShipWeapon::all_in` belong on the type, or as a free function in `sc-weapons`?** I've sketched it as an associated function, but a free `sc_weapons::iter_ship_weapons(data)` reads more naturally at call sites. Either works.

2. **Should the calc methods take `&Ammo` or `&Ammo::raw()`?** Taking `&Ammo` forces consumers to wrap the record first (one extra line). Taking `&AmmoParams` skips the wrapper but loses the damage-sum safety (someone could pass `projectile_params.damage` instead of going through `alpha_damage_scalar`). My lean is `&Ammo` for the safety.

3. **Weapon auto-classification on construction — `BallisticShipWeapon` / `EnergyShipWeapon` as separate types instead?** I argued against this above (branching problem at the consumer level), but it's worth revisiting if the `Option`-returning trait methods feel clunky in practice. Can always be added later.

4. **Should sustained DPS return `Result<f32, SustainError>` instead of `Option<f32>`?** The None case currently conflates "not applicable" (wrong weapon type) with "required fields missing from the DCB" (data bug). An enum would distinguish. Minor refinement; defer.

5. **Bulkhead's damage system doc has formulas that I'm using here almost verbatim. Should we re-verify them against real data before shipping this crate**, or trust bulkhead's derivation? Per the "go slow" principle — re-verify. The implementing doc for sc-weapons should specify a real-data validation step.

## Out of scope for this document

- Raw type definitions (generated) — see `docs/codegen.md`.
- Ammo wrapper and damage accessors — see `docs/sc-ammo.md`.
- Reference graph, tag tree, manufacturer registry — see `docs/sc-extract.md`.
- Damage pipeline — stays in bulkhead until `sc-ships`.
- Ship assembly — future `sc-ships` crate.
