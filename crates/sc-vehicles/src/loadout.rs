//! An editable loadout: what each port of a vehicle holds, as a calculator
//! changes it.

use sc_extract::{Guid, RecordCollection};
use sc_items::{Item, Items};
use serde::{Deserialize, Serialize};

use crate::{Misfit, Mounted, Port, PortContainers, Vehicle, Vehicles, fits, is_editable};

/// A vehicle's loadout: its [`stock`](Vehicle::stock) tree, then edited.
///
/// A port is addressed by its path of port names from the hull down
/// (`["hardpoint_gun_s5_front_left", "hardpoint_class_2"]`: the gun on a
/// gimbal), matched without case as the game matches them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loadout {
    /// The vehicle's `EntityClassDefinition`.
    pub vehicle: Guid,
    pub mounted: Vec<Mounted>,
}

/// Why an item cannot go into a port.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refusal {
    /// The path names no port.
    NoPort,
    /// The port cannot be changed by the player ([`is_editable`]).
    Locked,
    /// The item does not fit the port ([`fits`]).
    Misfit(Misfit),
}

impl Loadout {
    /// The vehicle's stock loadout.
    pub fn stock(vehicles: &Vehicles, vehicle: &Guid) -> Option<Self> {
        Some(Self {
            vehicle: *vehicle,
            mounted: vehicles.get(vehicle)?.stock.clone(),
        })
    }

    /// What the port at `path` holds.
    pub fn at(&self, path: &[&str]) -> Option<&Mounted> {
        let mut level = &self.mounted;
        let mut found = None;
        for p in path {
            let m = level.iter().find(|m| m.port.eq_ignore_ascii_case(p))?;
            level = &m.children;
            found = Some(m);
        }
        found
    }

    fn at_mut(&mut self, path: &[&str]) -> Option<&mut Mounted> {
        let (first, rest) = path.split_first()?;
        let mut m = self
            .mounted
            .iter_mut()
            .find(|m| m.port.eq_ignore_ascii_case(first))?;
        for p in rest {
            m = m
                .children
                .iter_mut()
                .find(|c| c.port.eq_ignore_ascii_case(p))?;
        }
        Some(m)
    }

    /// The ports along `path`, outermost first, and the items holding each
    /// port after the first: what [`is_editable`] and
    /// [`Vehicle::offered_tags`] take.
    pub fn chain<'a>(
        &self,
        path: &[&str],
        vehicles: &'a Vehicles,
        containers: &'a PortContainers,
    ) -> Option<(Vec<&'a Port>, Vec<Guid>)> {
        let vehicle = vehicles.get(&self.vehicle)?;
        let mut ports: &'a [Port] = &vehicle.ports;
        let mut chain = Vec::new();
        let mut holders = Vec::new();
        let mut level = &self.mounted;
        for (i, p) in path.iter().enumerate() {
            let port = ports.iter().find(|q| q.name.eq_ignore_ascii_case(p))?;
            chain.push(port);
            if i + 1 == path.len() {
                break;
            }
            let m = level.iter().find(|m| m.port.eq_ignore_ascii_case(p))?;
            let item = m.item?;
            holders.push(item);
            ports = vehicles.ports_of(&item, containers);
            level = &m.children;
        }
        Some((chain, holders))
    }

    /// Whether `item` can go into the port at `path`: the port exists, the
    /// player may change it, and the item fits it.
    pub fn check(
        &self,
        path: &[&str],
        item: &Item,
        items: &Items,
        vehicles: &Vehicles,
        containers: &PortContainers,
    ) -> Result<(), Refusal> {
        let vehicle = vehicles.get(&self.vehicle).ok_or(Refusal::NoPort)?;
        let (chain, holders) = self
            .chain(path, vehicles, containers)
            .ok_or(Refusal::NoPort)?;
        if !is_editable(&chain) {
            return Err(Refusal::Locked);
        }
        let port = chain.last().ok_or(Refusal::NoPort)?;
        let offered = vehicle.offered_tags(items, &chain, &holders);
        fits(port, item, &offered).map_err(Refusal::Misfit)
    }

    /// Every item that can go into the port at `path` ([`check`](Self::check)),
    /// empty for a locked or missing port. Template and placeholder records
    /// (`POWR_S02_Template`) fit too; they have no player-facing name, which
    /// is how a consumer tells them apart once it resolves names.
    pub fn candidates(
        &self,
        path: &[&str],
        items: &Items,
        vehicles: &Vehicles,
        containers: &PortContainers,
    ) -> Vec<Guid> {
        let (Some(vehicle), Some((chain, holders))) = (
            vehicles.get(&self.vehicle),
            self.chain(path, vehicles, containers),
        ) else {
            return Vec::new();
        };
        let Some(port) = chain.last() else {
            return Vec::new();
        };
        if !is_editable(&chain) {
            return Vec::new();
        }
        let offered = vehicle.offered_tags(items, &chain, &holders);
        items
            .iter()
            .filter(|(_, item)| fits(port, item, &offered).is_ok())
            .map(|(g, _)| *g)
            .collect()
    }

    /// Put `item` into the port at `path` (`None` empties it). What the old
    /// item carried goes with it; the new one comes with its own default
    /// loadout, as when bought. Does not [`check`](Self::check) the item;
    /// `false` when the path names no port.
    pub fn equip(
        &mut self,
        path: &[&str],
        item: Option<Guid>,
        vehicles: &Vehicles,
        containers: &PortContainers,
    ) -> bool {
        let Some(m) = self.at_mut(path) else {
            return false;
        };
        m.item = item;
        m.children = item
            .map(|i| vehicles.default_children(i, containers))
            .unwrap_or_default();
        true
    }

    /// The vehicle this loadout is for.
    pub fn vehicle<'a>(&self, vehicles: &'a Vehicles) -> Option<&'a Vehicle> {
        vehicles.get(&self.vehicle)
    }
}
