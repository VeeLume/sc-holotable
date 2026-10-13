//! Mining: the rock side and its maths — a domain crate.
//!
//! What the game does when a laser works on a rock, read from the client
//! (LIVE 4.10) and checked against in-game scans:
//!
//! - [`Rulesets`] — the `MiningGlobalParams` records (ship, ground vehicle,
//!   FPS); a rock's `MineableParams.globalParams` picks one.
//! - [`Elements`] — the `MineableElement` records with their densities.
//! - [`Rock`] — a rock's own resistance, instability, window, mass and SCU
//!   from the composition the scanner lists.
//! - [`Setup`] / [`ModifiedRock`] — lasers (with modules) and gadgets applied
//!   to a rock: the values once the beams land, breakability
//!   ([`ModifiedRock::max_mass`]), the balance throttle, timings.
//! - [`simulate`] — the charge bar tick by tick, instability wave and breaking
//!   pools included; [`difficulty`] runs it many times with a [`Player`] model
//!   for our own difficulty (overcharges, danger at fracture, time).
//! - [`Laser::extraction_scu_per_s`] — extraction speed.
//! - [`RockTypes`] / [`Compositions`] / [`Shapes`] — what spawns: each
//!   `HarvestablePreset` rock's scale range, meshes and composition parts,
//!   and each mesh's fracture tree ([`Shape`]: the pieces it breaks into).
//!   [`Spawner`] rolls rocks as the client does and [`Survey`]s how a setup
//!   fares on them (breakable share, mass, SCU, cracks until collectable).
//!
//! Item stats come from `sc-items-ship-weapons` (lasers) and `sc-items-mining`
//! (modules, gadgets); rock spawns and compositions per location from
//! `sc-gathering`.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_mining::{Elements, Entry, Laser, ModifiedRock, Rock, Rulesets, Setup};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let resources = sc_resources::Resources::build(datacore.records());
//! let rulesets = Rulesets::build(&datacore, &resources);
//! let elements = Elements::build(&datacore, &resources);
//! let (ruleset, element) = (
//!     rulesets.values().next().ok_or("no ruleset")?,
//!     elements.values().next().ok_or("no element")?,
//! );
//! let entries = [Entry { element: element.guid, fraction: 0.6 }];
//! let rock = Rock::from_mass(ruleset, &elements, &entries, 9000.0).ok_or("no density")?;
//! let items = sc_items::Items::build(datacore.records());
//! let weapons = sc_items_ship_weapons::ShipWeapons::build(&datacore, &items);
//! let laser = weapons.values().find_map(|w| w.mining.clone()).ok_or("no mining laser")?;
//! let setup = Setup { lasers: vec![Laser::new(laser)], gadgets: vec![] };
//! let mined = ModifiedRock::new(ruleset, &rock, &setup);
//! println!(
//!     "breakable up to {:.0} t, balance throttle {:?}",
//!     mined.max_mass(ruleset),
//!     mined.balance_throttle(ruleset)
//! );
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod composition;
mod element;
mod rock;
mod rock_type;
mod ruleset;
mod setup;
mod shape;
mod sim;
mod spawn;
#[cfg(test)]
mod tests;

pub use composition::{Composition, Compositions, Part};
pub use element::{Element, Elements};
pub use rock::{Entry, Rock, Window};
pub use rock_type::{Geometry, RockType, RockTypes};
pub use ruleset::{HitConsistency, Ruleset, Rulesets};
pub use setup::{Combined, Laser, LaserPower, ModifiedRock, Setup};
pub use shape::{Piece, Shape};
pub use sim::{
    Difficulty, Fixed, Outcome, Player, Pool, Sample, SimConfig, Throttle, Trace, View, difficulty,
    hit_consistency, simulate,
};
pub use spawn::{Quantiles, Shapes, Spawn, Spawner, Survey};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collections.
pub use sc_extract::RecordCollection;
