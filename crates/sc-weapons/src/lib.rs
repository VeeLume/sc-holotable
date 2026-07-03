//! Canonical weapon data model for Star Citizen.
//!
//! Wraps generated `EntityClassDefinition` records from [`sc_extract`] into
//! ergonomic, owned structs with all weapon data materialized at construction
//! time. No `&DataPools` needed after construction — every accessor is a
//! plain field read.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use sc_weapons::{iter_ship_weapons, Items};
//!
//! let datacore: sc_extract::Datacore = /* ... */;
//! let items = Items::build(datacore.records());   // build once, share by reference
//! for weapon in iter_ship_weapons(&datacore, &items) {
//!     println!("{}: S{} {:?}", weapon.record_name, weapon.size, weapon.primary_fire_action);
//! }
//! ```
//!
//! # Scope
//!
//! v1 is **data accessors only**. Sustained DPS calculations, fire-mode
//! switching, and FPS sustain models are deferred to v2 — see
//! `docs/sc-weapons.md` for the full deferred list.

mod classify;
mod damage;
mod fire_action;
mod fps;
mod missile;
mod pools;
mod ship;
mod sustain;

#[cfg(feature = "tui")]
pub mod tui;

pub use classify::WeaponCategory;
pub use damage::DamageSummary;
pub use fire_action::{ChargeModifier, FireActionKind};
pub use fps::FpsWeapon;
pub use missile::{Missile, TrackingProfile};
pub use pools::WeaponPools;
pub use ship::{LoadoutContext, ShipWeapon};
pub use sustain::{EnergyModel, HeatModel, SustainKind};

// ── Narrow-consumer re-exports ──────────────────────────────────────────────
//
// Lets a consumer depend on `sc-weapons` alone and still construct the
// arguments `iter_*_weapons` takes, without adding a direct
// `sc-extract` dep. Type identity is preserved because every aggregation
// crate pulls the same `sc-extract` rev.
pub use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, ExtractSnapshot, Guid, LocaleKey, LocaleMap,
    SnapshotMeta,
};
// Item envelope now lives in sc-items; re-export so single-crate consumers
// can name the cache type without a direct sc-items dep.
pub use sc_items::{Item, Items};

/// Escape hatch for raw DCB queries when the typed model doesn't cover
/// a case. Reach for these only as a last resort.
pub mod raw {
    pub use sc_extract::svarog_datacore;
    pub use sc_extract::{DataCoreDatabase, Instance, Value};
}

use std::collections::HashMap;

/// Iterate over all ship weapons in the datacore.
///
/// Walks every `EntityClassDefinition` record, attempts to construct a
/// [`ShipWeapon`], and yields those that succeed. Records that aren't ship
/// weapons (FPS, CMLs, mining, creatures) are silently skipped.
pub fn iter_ship_weapons<'a>(
    datacore: &'a Datacore,
    items: &'a Items,
) -> impl Iterator<Item = ShipWeapon> + 'a {
    let store = datacore.records();
    let db = datacore.db();
    let pools = &store.pools;
    let ecd_map = &store.records.multi_feature.entity_class_definition;
    let ammo_map = &store.records.multi_feature.ammo_params;

    // Pre-build GUID → record name map
    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    ecd_map.iter().filter_map(move |(&guid, &handle)| {
        ShipWeapon::try_new(handle, guid, pools, ecd_map, ammo_map, &record_names, items)
    })
}

/// Every materialized weapon family plus the cross-family collision index —
/// the domain entry point for sc-weapons (workspace rule 6).
///
/// Build once via [`Weapons::build`]; the per-family [`iter_ship_weapons`] /
/// [`iter_fps_weapons`] / [`iter_missiles`] functions remain (rule 7) for
/// streaming one family without materializing the whole set.
pub struct Weapons {
    /// Ship-mounted guns.
    pub ships: Vec<ShipWeapon>,
    /// FPS / personal weapons.
    pub fps: Vec<FpsWeapon>,
    /// Missiles + torpedoes.
    pub missiles: Vec<Missile>,
    /// `LocaleKey → Vec<Guid>` collision index across all three families.
    /// More grouping axes (by manufacturer, size, tag) land as sibling fields
    /// — non-breaking.
    pub pools: WeaponPools,
}

impl Weapons {
    /// Materialize every weapon family (ship guns, FPS, missiles) and the
    /// collision pools, sharing one `&Items` across the three walks.
    ///
    /// (sc-langpatch's `weapon_enhancer` is the motivating consumer — it needs
    /// both the per-weapon lists and the `LocaleKey → Vec<Guid>` pool.)
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let ships: Vec<ShipWeapon> = iter_ship_weapons(datacore, items).collect();
        let fps: Vec<FpsWeapon> = iter_fps_weapons(datacore, items).collect();
        let missiles: Vec<Missile> = iter_missiles(datacore, items).collect();
        let pools = WeaponPools::build(&ships, &fps, &missiles);
        Self {
            ships,
            fps,
            missiles,
            pools,
        }
    }
}

/// Iterate over all FPS weapons in the datacore.
///
/// Same pattern as [`iter_ship_weapons`] but yields [`FpsWeapon`] instead.
pub fn iter_fps_weapons<'a>(
    datacore: &'a Datacore,
    items: &'a Items,
) -> impl Iterator<Item = FpsWeapon> + 'a {
    let store = datacore.records();
    let db = datacore.db();
    let pools = &store.pools;
    let ecd_map = &store.records.multi_feature.entity_class_definition;
    let ammo_map = &store.records.multi_feature.ammo_params;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    ecd_map.iter().filter_map(move |(&guid, &handle)| {
        FpsWeapon::try_new(handle, guid, pools, ecd_map, ammo_map, &record_names, items)
    })
}

/// Iterate over all ship missiles + torpedoes in the datacore.
///
/// Walks every `EntityClassDefinition`, attempts to construct a
/// [`Missile`], and yields those that succeed. Records that aren't
/// missile/torpedo-classified ordnance are silently skipped.
pub fn iter_missiles<'a>(
    datacore: &'a Datacore,
    items: &'a Items,
) -> impl Iterator<Item = Missile> + 'a {
    let store = datacore.records();
    let db = datacore.db();
    let pools = &store.pools;
    let ecd_map = &store.records.multi_feature.entity_class_definition;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    ecd_map.iter().filter_map(move |(&guid, &handle)| {
        Missile::try_new(handle, guid, pools, ecd_map, &record_names, items)
    })
}
