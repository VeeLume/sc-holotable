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
//! use sc_weapons::iter_ship_weapons;
//!
//! let datacore: sc_extract::Datacore = /* ... */;
//! for weapon in iter_ship_weapons(&datacore) {
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
mod error;
mod fire_action;
mod fps;
mod ship;
mod sustain;

pub use classify::WeaponCategory;
pub use damage::DamageSummary;
pub use error::WeaponError;
pub use fire_action::{ChargeModifier, FireActionKind};
pub use fps::FpsWeapon;
pub use ship::{LoadoutContext, ShipWeapon};
pub use sustain::{EnergyModel, HeatModel, SustainKind};

use sc_extract::{Datacore, Guid};

use std::collections::HashMap;

/// Iterate over all ship weapons in the datacore.
///
/// Walks every `EntityClassDefinition` record, attempts to construct a
/// [`ShipWeapon`], and yields those that succeed. Records that aren't ship
/// weapons (FPS, CMLs, mining, creatures) are silently skipped.
pub fn iter_ship_weapons(datacore: &Datacore) -> impl Iterator<Item = ShipWeapon> + '_ {
    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;
    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    // Pre-build GUID → record name map
    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    ecd_map.iter().filter_map(move |(&guid, &handle)| {
        ShipWeapon::try_new(handle, guid, pools, ecd_map, ammo_map, &record_names)
    })
}

/// Iterate over all FPS weapons in the datacore.
///
/// Same pattern as [`iter_ship_weapons`] but yields [`FpsWeapon`] instead.
pub fn iter_fps_weapons(datacore: &Datacore) -> impl Iterator<Item = FpsWeapon> + '_ {
    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;
    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    ecd_map.iter().filter_map(move |(&guid, &handle)| {
        FpsWeapon::try_new(handle, guid, pools, ecd_map, ammo_map, &record_names)
    })
}
