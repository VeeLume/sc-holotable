//! Ships and ground vehicles as a loadout sees them — a domain crate, the
//! backbone of Starlume's loadout calculator.
//!
//! - [`Vehicles`]: every entity with `VehicleComponentParams`: its hull
//!   (parts with HP and mass, from the vehicle XML via
//!   [`sc_extract::VehicleDefinition`]), its ports (the hull's hardpoints
//!   plus the entity's own), and its [`stock`](Vehicle::stock) loadout as a
//!   tree.
//! - [`PortContainers`]: every entity's own ports and default loadout, so
//!   the items below a port (gimbals, turrets, racks) bring their ports.
//! - [`fits`] / [`is_editable`]: whether an item goes into a port, and
//!   whether the player can change it.
//!
//! Read from LIVE 4.10 (DCB, vehicle XML, client): ports come from the hull
//! XML and the entity's DCB port container (no name in both); a stock
//! loadout entry names its item by reference or record name, and nests the
//! loadout of the item's own ports; an entry without nesting leaves the
//! item's own default loadout in place, a port without an entry holds its
//! `defaultItem`. The RSI Constellation Mk V Centaurus resolves to every
//! line spviewer lists (`vehicles_probe`).
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_items::Items;
//! use sc_vehicles::{PortContainers, Vehicles};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let containers = PortContainers::build(&datacore);
//! let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
//! for (guid, v) in vehicles.iter().take(3) {
//!     println!("{guid}: {} hull HP, {} ports", v.hull_health(), v.ports.len());
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod container;
mod defense;
mod distribution;
mod flight;
mod loadout;
mod network;
mod port;
mod power;
mod resource;
mod vehicle;

pub use container::{LoadoutEntry, PortContainer, PortContainers};
pub use defense::{Defense, Shield};
pub use distribution::{
    Allocation, Consumer, ItemReadout, Plant, Pool, PowerNetwork, Readout, Signals,
};
pub use flight::{Accelerations, Flight, port_axes};
pub use loadout::{Loadout, Refusal};
pub use network::{DistributionDefaults, NetworkParams, PoolSize, PowerPool};
pub use port::{Misfit, Port, PortSource, fits, has_flag, is_editable};
pub use power::{em_signature, plant_output, signature_ratio};
pub use resource::{
    DeltaKind, DynamicAmount, PowerRange, ResourceAmount, ResourceDelta, ResourceProfile,
    ResourceProfiles, ResourceState, ResourceUnits, Signature,
};
pub use vehicle::{Insurance, Mounted, Vehicle, Vehicles, network_items};

/// Re-export the canonical accessor trait so consumers can bring the
/// `get` / `iter` / `len` / `values` surface into scope alongside the collections.
pub use sc_extract::RecordCollection;
