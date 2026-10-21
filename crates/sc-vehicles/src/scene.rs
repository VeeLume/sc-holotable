//! A loadout assembled for drawing: the hull and every installed item, as
//! an [`sc_items_visuals::Scene`] a renderer walks like any item's.

use sc_extract::{CharacterDefinition, Guid};
use sc_items_visuals::{Attached, Palettes, Scene, Visuals};

use crate::{Mounted, Vehicle};

/// `vehicle` (record `guid`) with `loadout` installed (usually
/// [`Vehicle::stock`]), for drawing. Items sit on their ports' helpers: an
/// item's own ports as its record defines them; a vehicle's port that names
/// no helper (every hull-XML hardpoint, and entity ports like the Mk V's
/// `hardpoint_tail`) on the hull node of the same name, the engine's
/// attach rule for vehicle ports. Docked vehicles are drawn too, on their
/// ports. `read_cdf` reads a `.cdf` assembly by path. `None` when the
/// vehicle has no geometry.
///
/// ```no_run
/// use sc_extract::{AssetConfig, AssetData, AssetSource, CharacterDefinition, Datacore, RecordCollection};
/// use sc_items::Items;
/// use sc_items_visuals::{Palettes, Visuals};
/// use sc_vehicles::{PortContainers, Vehicles};
///
/// let install = sc_discovery::discover_primary()?;
/// let assets = AssetSource::from_install(&install)?;
/// let data = AssetData::extract(&assets, &AssetConfig::standard())?;
/// let datacore = Datacore::parse(&assets, &data)?;
/// let items = Items::build(datacore.records());
/// let containers = PortContainers::build(&datacore, &assets);
/// let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
/// let (visuals, palettes) = (Visuals::build(&datacore, &items), Palettes::build(&datacore));
/// let (guid, ship) = vehicles.iter().next().ok_or("no vehicles")?;
/// let scene = sc_vehicles::scene(&visuals, &palettes, guid, ship, &ship.stock, |p| {
///     CharacterDefinition::read(&assets, p).map_err(|e| e.to_string())
/// });
/// println!("{:?}", scene.map(|s| s.root.attachments.len()));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn scene(
    visuals: &Visuals,
    palettes: &Palettes,
    guid: &Guid,
    vehicle: &Vehicle,
    loadout: &[Mounted],
    read_cdf: impl FnMut(&str) -> Result<CharacterDefinition, String>,
) -> Option<Scene> {
    let ports: Vec<sc_items_visuals::Port> = vehicle
        .ports
        .iter()
        .map(|p| sc_items_visuals::Port {
            name: p.name.clone(),
            helper: Some(p.name.clone()),
            offset: sc_items_visuals::Transform::default(),
            item_offset_helper: None,
            skinned: false,
            flags: p.flags.join(" "),
        })
        .collect();
    visuals.mounted_scene(palettes, guid, &ports, &attached(loadout), read_cdf)
}

/// A resolved loadout tree as [`Attached`] entries: every level explicit.
fn attached(level: &[Mounted]) -> Vec<Attached> {
    level
        .iter()
        .filter_map(|m| {
            Some(Attached {
                port: m.port.clone(),
                item: m.item?,
                children: Some(attached(&m.children)),
            })
        })
        .collect()
}
