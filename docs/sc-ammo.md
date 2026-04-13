# `sc-ammo` — design specification

> Status: **proposal, awaiting review.** Depends on `docs/sc-extract.md` and `docs/codegen.md`.

> **Flat-pool API update (post-refactor).** Since the spec was first drafted, `sc-extract-generated` moved to a flat-pool representation: nested `Class` / `StrongPointer` / `WeakPointer` fields are now `Option<TId>` handles into per-type `Vec<Option<T>>` pools inside `DataPools`, not `Option<Box<T>>`. Any code sketch below that shows direct dereference of a nested struct field will, in the real implementation, go through `id.get(pools)` or `pools[id].as_ref()` to resolve the handle. See `docs/codegen.md` §"Reference semantics — flat-pool model" for the API. The wrapper pattern (one `Ammo` struct, a trait, a few accessors) is unchanged — only the way nested fields are reached.

## Purpose

`sc-ammo` provides a hand-written domain wrapper around the generated `AmmoParams` record type from `sc-extract`. It exposes weapon-intrinsic ammo concerns — alpha damage, projectile vs explosion damage split, penetration, damage falloff — with ergonomic accessors, without duplicating any of the data that's already in the generated types.

It is a deliberately small crate: roughly one wrapper struct, a few traits, and a handful of accessor methods. The real data lives in `sc_extract::generated::AmmoParams` and related structs.

## Consumers

| Crate | What it uses |
|---|---|
| `sc-weapons` | Resolves a weapon's ammo reference through `sc-ammo::Ammo::from_raw`. Every weapon calculation that reads damage routes through this crate. |
| `bulkhead` (future) | Damage-pipeline inputs — alpha, damage-per-type split, penetration distance, falloff. |
| `sc-langpatch` (future) | Rarely needs ammo details directly, but uses the same wrapper if it does. |

## Scope

**What `sc-ammo` owns:**

- A single wrapper struct `Ammo` around the generated `AmmoParams`.
- Accessor methods for the fields consumers actually need (damage, speed, lifetime, penetration, radius, falloff).
- The canonical handling of the **two damage sources**: direct hit (`projectileParams.damage`) and explosion (`detonationParams.explosionParams.damage`). Both are always exposed; both are always summed for "total damage".
- A few sub-categorization helpers that answer "is this ballistic / energy / distortion / rocket-type ammo?"

**What `sc-ammo` does *not* own:**

- The raw `AmmoParams` struct (lives in `sc_extract::generated`)
- `DamageInfo` (also lives in `sc_extract::generated`)
- Weapon-level calculations like DPS or sustained DPS — those live in `sc-weapons`.
- Damage-pipeline computations (shield/armor/hull) — those live in `bulkhead` or a future `sc-ships`.
- Extraction or serialization — the raw type is already extracted by `sc-extract` and lives in `DatacoreSnapshot::records` (reachable either via a live `Datacore::snapshot()` or through `ExtractSnapshot::datacore`).

## The `Ammo` wrapper

```rust
// crates/components/sc-ammo/src/lib.rs
use sc_extract::generated::{AmmoParams, DamageInfo};

/// Newtype wrapper around the generated `AmmoParams` record.
/// All fields are reachable via `self.raw()` for direct access;
/// the methods below provide the ergonomic / derived values.
#[derive(Debug, Clone)]
pub struct Ammo(AmmoParams);

impl Ammo {
    pub fn from_raw(raw: AmmoParams) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> &AmmoParams {
        &self.0
    }

    pub fn into_raw(self) -> AmmoParams {
        self.0
    }
}
```

No validation at construction. `Ammo` is a zero-cost wrapper — it exists purely to hang methods on. Consumers who already have an `AmmoParams` reference can call `Ammo::from_raw(params.clone())` or access fields directly.

## Damage accessors — the two-source rule

Every ammo record has two independent damage sources. **Callers must always sum them for correct "total damage".** Reading only `projectile_params.damage` silently misses explosion damage, which is where distortion weapons and rockets deliver *all* of their damage.

```rust
impl Ammo {
    /// Direct-hit damage from `projectileParams.damage`.
    /// For distortion weapons this is essentially zero (~0.00009).
    /// For ballistics/lasers this is the primary damage source.
    pub fn direct_damage(&self) -> DamageInfo {
        self.0.projectile_params.damage.clone()
    }

    /// Explosion damage from `detonationParams.explosionParams.damage`.
    /// Returns `DamageInfo::default()` if the ammo has no detonation/explosion.
    /// For distortion weapons and rockets this is the primary damage source.
    pub fn explosion_damage(&self) -> DamageInfo {
        self.0
            .detonation_params
            .as_ref()
            .and_then(|d| d.explosion_params.as_ref())
            .map(|e| e.damage.clone())
            .unwrap_or_default()
    }

    /// Total alpha damage = direct + explosion, per damage type.
    /// **Always use this** instead of reading `direct_damage` alone,
    /// unless you specifically need to distinguish the two sources.
    pub fn total_damage(&self) -> DamageInfo {
        let mut total = self.direct_damage();
        let explosion = self.explosion_damage();
        total.damage_physical    += explosion.damage_physical;
        total.damage_energy      += explosion.damage_energy;
        total.damage_distortion  += explosion.damage_distortion;
        total.damage_thermal     += explosion.damage_thermal;
        total.damage_biochemical += explosion.damage_biochemical;
        total.damage_stun        += explosion.damage_stun;
        total
    }

    /// Scalar total alpha damage — sum of every damage type in `total_damage`.
    pub fn alpha_damage_scalar(&self) -> f32 {
        let d = self.total_damage();
        d.damage_physical
            + d.damage_energy
            + d.damage_distortion
            + d.damage_thermal
            + d.damage_biochemical
            + d.damage_stun
    }

    /// True if this ammo is primarily explosion-based
    /// (direct damage is negligible, all damage comes from explosion).
    pub fn is_explosion_primary(&self) -> bool {
        let direct = self.direct_damage();
        let direct_total = direct.damage_physical
            + direct.damage_energy
            + direct.damage_distortion
            + direct.damage_thermal
            + direct.damage_biochemical
            + direct.damage_stun;
        direct_total < 1.0 && self.explosion_damage().damage_physical > 0.0
            || self.explosion_damage().damage_distortion > 0.0
    }
}
```

### The distortion weapon trap

**This is a required test case.** A distortion weapon's `direct_damage` is essentially zero (`~0.00009` in practice); all meaningful damage lives in `explosion_damage`. Bulkhead's old code had this bug. Our `total_damage` and `alpha_damage_scalar` methods fix it by construction.

```rust
#[cfg(test)]
mod tests {
    // Required test: verify a known distortion weapon's total damage is non-zero.
    // Uses a committed snapshot fixture or a constructed test AmmoParams.
    #[test]
    fn distortion_weapon_damage_includes_explosion() {
        let ammo = fixture::apocalypse_launchpad_distortion_ammo();
        assert!(
            ammo.direct_damage().damage_distortion < 1.0,
            "distortion direct damage should be ~0"
        );
        assert!(
            ammo.explosion_damage().damage_distortion > 0.0,
            "distortion explosion damage should be > 0"
        );
        assert!(
            ammo.total_damage().damage_distortion > 0.0,
            "total damage should be > 0"
        );
    }
}
```

## Kinematic accessors

```rust
impl Ammo {
    /// Projectile muzzle velocity in m/s.
    pub fn speed(&self) -> f32 {
        self.0.speed
    }

    /// Time-to-live in seconds before the projectile despawns.
    pub fn lifetime(&self) -> f32 {
        self.0.lifetime
    }

    /// Effective range = speed × lifetime (before damage drop-off).
    pub fn max_range(&self) -> f32 {
        self.speed() * self.lifetime()
    }
}
```

## Penetration accessors

```rust
impl Ammo {
    /// Base penetration distance in meters (from `projectileParams.penetrationParams`).
    /// Returns `None` if the ammo is not meant to penetrate
    /// (e.g., distortion weapons — `penetrationAbsorptionForType[Distortion] = 0`).
    pub fn base_penetration_distance(&self) -> Option<f32> {
        self.0
            .projectile_params
            .penetration_params
            .as_ref()
            .map(|p| p.base_penetration_distance)
    }

    /// Penetration cone radius near / far (for component damage calc).
    pub fn penetration_cone(&self) -> Option<(f32, f32)> {
        self.0
            .projectile_params
            .penetration_params
            .as_ref()
            .map(|p| (p.near_radius, p.far_radius))
    }
}
```

## Explosion-radius accessors

```rust
impl Ammo {
    /// Explosion inner / outer radius for AoE damage calculation.
    /// Returns `None` if the ammo has no detonation params.
    pub fn explosion_radius(&self) -> Option<(f32, f32)> {
        self.0
            .detonation_params
            .as_ref()
            .and_then(|d| d.explosion_params.as_ref())
            .map(|e| (e.min_radius, e.max_radius))
    }
}
```

## Sub-categorization helpers

```rust
impl Ammo {
    /// Rough classification based on where the damage lives.
    pub fn kind(&self) -> AmmoKind {
        if self.is_explosion_primary() {
            if self.direct_damage().damage_physical > 0.0 {
                AmmoKind::Rocket
            } else {
                AmmoKind::Distortion
            }
        } else if self.direct_damage().damage_physical > 0.0 {
            AmmoKind::Ballistic
        } else {
            AmmoKind::Energy
        }
    }
}

/// Coarse classification of an ammo type. Derived from where the damage
/// lives, not from a DataCore field — this is an opinionated categorization
/// meant for display and filtering, not for engine-correct behavior.
pub enum AmmoKind {
    /// Direct physical damage, no explosion.
    Ballistic,
    /// Direct energy damage, no explosion.
    Energy,
    /// Explosion-only distortion damage.
    Distortion,
    /// Explosion-only physical damage (rockets, missiles).
    Rocket,
}
```

## Construction from a record lookup

Typical consumer flow:

```rust
use sc_extract::{DatacoreSnapshot, Guid};
use sc_ammo::Ammo;

fn get_ammo_for_weapon(snap: &DatacoreSnapshot, ammo_ref: Guid) -> Option<Ammo> {
    // Ammo records in the DCB are keyed by GUID — the reference comes from
    // the weapon's `SAmmoContainerComponentParams.ammoParamsRecord` field.
    // The exact accessor on `RecordStore` depends on the typed-lookup helpers
    // that ship with the flat-pool model; consult the live crate for the
    // current idiom.
    let handle = snap.records.ammo_handle(&ammo_ref)?;
    let raw = handle.get(&snap.records).cloned()?;
    Some(Ammo::from_raw(raw))
}
```

`AmmoParams` is a top-level record type in the DCB and lives in the flat pools on `RecordStore`. See `docs/codegen.md` and `implementing/sc-generator.md` for how the generator exposes per-type handles.

## Design decisions

### Newtype wrapper, not a new struct

`Ammo` is literally `struct Ammo(AmmoParams)`. It doesn't rearrange fields, doesn't copy data, doesn't maintain its own state. It exists only to hang methods on, because you can't add inherent methods to a type defined in another crate in Rust.

Alternative would be an extension trait, but extension traits require an extra `use sc_ammo::AmmoExt;` at every call site and produce worse IDE hover/completion. Newtype is simpler.

### `total_damage` is the default, not `direct_damage`

Consumers who call `alpha_damage_scalar` or `total_damage` get the correct answer regardless of weapon type. Consumers who specifically want to distinguish direct from explosion (for the damage-pipeline layer where penetration cones vs AoE matters) can call `direct_damage` and `explosion_damage` separately. The API makes the safer default easy and the detailed split possible.

### No weapon-level calcs here

Fire rate, DPS, sustained DPS, time-to-overheat all live in `sc-weapons`. `sc-ammo` only knows about the ammo itself — what it does when it hits something, not how often a weapon launches it. The split mirrors the DCB split: `AmmoParams` vs `SCItemWeaponComponentParams` are separate records in separate files.

### `AmmoKind` is opinionated classification, not a DCB field

The DCB doesn't tag ammo as "ballistic" vs "energy" vs "distortion" — it just has damage values per type and sometimes explosion params. `AmmoKind` derives a coarse bucket from that data for display purposes. It is explicitly opinionated and documented as such.

## What `sc-ammo` does *not* provide

- Raw ammo types — `sc_extract::generated::AmmoParams` is the source of truth.
- Weapon calculations — in `sc-weapons`.
- Damage-pipeline calculations — in consumer apps.
- Snapshot I/O — `sc-extract` owns parse/load.
- Cross-record resolution — `DatacoreSnapshot::records` (reachable via `Datacore::snapshot()` or `ExtractSnapshot::datacore`) handles GUID → record lookup through its flat pools; `sc-ammo` takes already-resolved `AmmoParams` values.

## Open questions

1. **Should the damage sum live on `DamageInfo` itself** (as a method in `sc-extract`, since `DamageInfo` is a generated type)? The generator can't add methods to its own output without hand-written glue, but `sc-extract` can add an extension trait on `DamageInfo`. My lean: yes, add `DamageInfo::total() -> f32` in `sc-extract` via an extension trait, and `sc-ammo` uses it. Avoids duplicating the sum logic.

2. **Kinematic calculations — should `max_range` account for falloff**, or just be raw `speed × lifetime`? Bulkhead's damage calc distinguishes "effective" range (where damage is still meaningful) from "max" range (where the projectile despawns). For now I've kept `max_range` as the raw physics calculation and will add `effective_range` later when the damage-drop model is clearer.

3. **Should `AmmoKind` be extended with `Shotgun` (multi-pellet)?** Some ballistic guns fire multiple pellets per shot. That's a weapon property (`pellets_per_shot` on `SWeaponActionFireRapidParams`), not an ammo property, so it probably belongs in `sc-weapons::Weapon::fire_mode` not here. **Leaving out for now.**

## Out of scope for this document

- Weapon wrapper and weapon calculations — see `docs/sc-weapons.md`.
- Raw `AmmoParams` type definition — that's generated, see `docs/codegen.md`.
- Damage pipeline (shield → armor → hull) — stays in bulkhead for now.
