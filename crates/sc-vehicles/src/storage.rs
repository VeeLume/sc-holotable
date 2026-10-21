//! What a vehicle stores: cargo grids, storage lockers, suit lockers and
//! weapon racks, from its loadout and the entities its interior places.
//!
//! Read from LIVE 4.10. Cargo grids and storage are items with an
//! inventory ([`sc_items::Inventory`]): an open container is a grid, a
//! closed one a locker. Grids mostly sit in the loadout (some behind a
//! door's XML loadout, as on the Hammerhead); personal storage, suit
//! lockers and weapon racks mostly sit in the interior
//! ([`Interior::entities`](crate::Interior::entities)). Racks and suit
//! lockers carry no marker of their own: they are the entities whose
//! ports take personal weapons (`WeaponPersonal`) or armour
//! (`Char_Armor_*`).

use std::collections::{BTreeMap, BTreeSet};

use sc_extract::generated::{EItemSubType, EItemType};
use sc_extract::{Guid, PortType, RecordCollection};
use sc_items::{Inventory, InventoryKind, Items};
use serde::{Deserialize, Serialize};

use crate::{Mounted, PlacedEntity, Port, PortContainers, Transform, Vehicle, Vehicles};

/// What a vehicle stores.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Storage {
    /// The ship's own inventory ([`Vehicle::inventory`]), SCU.
    pub ship_inventory: Option<f64>,
    /// Open containers.
    pub cargo: Vec<CargoGrid>,
    /// Closed containers: personal storage, seat and bed storage, lockers.
    pub lockers: Vec<Locker>,
    /// Entities whose ports take armour.
    pub suit_lockers: Vec<Rack>,
    /// Entities whose ports take personal weapons (fire-extinguisher
    /// cabinets among them: their one slot takes a gadget).
    pub weapon_racks: Vec<Rack>,
}

/// Where a stored thing sits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Place {
    /// In the loadout, at this port path (`hardpoint_cargo_lift/
    /// hardpoint_cargo_grid`).
    Port(String),
    /// Placed by the interior, or mounted below such an entity (`path`,
    /// empty for the entity itself).
    Interior {
        /// Index into [`Vehicle::object_containers`].
        container: usize,
        /// The entity's instance name.
        name: String,
        /// The entity's frame in the ship.
        transform: Transform,
        path: String,
    },
}

/// An open container: a cargo grid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CargoGrid {
    pub place: Place,
    pub item: Guid,
    /// [`Inventory::scu`]: whole 1.25 m cells.
    pub scu: f64,
    /// `interiorDimensions`, m.
    pub dimensions: Option<[f32; 3]>,
    /// Outside the hull (`isExternalContainer`, the Hull C's struts).
    pub external: bool,
}

/// A closed container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locker {
    pub place: Place,
    pub item: Guid,
    /// SCU.
    pub capacity: f64,
}

/// A suit locker or weapon rack and its slots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rack {
    pub place: Place,
    pub item: Guid,
    pub slots: Vec<Slot>,
}

impl Rack {
    /// One of its slots takes items of `item_type` with `sub_type` (a weapon
    /// rack that takes `WeaponPersonal` `Medium` holds rifles; a
    /// fire-extinguisher cabinet only a `Gadget`).
    pub fn takes(&self, item_type: &EItemType, sub_type: &EItemSubType) -> bool {
        self.slots.iter().any(|s| s.takes(item_type, sub_type))
    }
}

/// One port of a [`Rack`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub port: String,
    pub min_size: i32,
    pub max_size: i32,
    /// What goes in: per item type, the sub-types of the items that match
    /// the port's types and size range (a weapon rack's rifle slot takes
    /// `WeaponPersonal` `Medium`; a slot listing no sub-type takes every
    /// sub-type an item of a fitting size has). Tags are not checked.
    pub accepts: Vec<PortType>,
}

impl Slot {
    /// It takes items of `item_type` with `sub_type`.
    pub fn takes(&self, item_type: &EItemType, sub_type: &EItemSubType) -> bool {
        self.accepts
            .iter()
            .any(|t| t.item_type == *item_type && t.sub_types.contains(sub_type))
    }
}

impl Storage {
    /// What `vehicle` stores with `loadout` (its [`stock`](Vehicle::stock)
    /// tree or an edited one) and the `entities` its interior places
    /// ([`Interior::read`](crate::Interior::read)). Docked vehicles are
    /// left out, as they are from the power network.
    pub fn compute(
        vehicle: &Vehicle,
        loadout: &[Mounted],
        entities: &[PlacedEntity],
        vehicles: &Vehicles,
        items: &Items,
        containers: &PortContainers,
    ) -> Self {
        let mut out = Self {
            ship_inventory: vehicle.inventory.as_ref().and_then(Inventory::scu),
            ..Self::default()
        };
        let kinds = Kinds::new(items);
        let mut add = |place: Place, item: Guid| out.add(place, item, items, containers, &kinds);

        let mut stack: Vec<(String, &Mounted)> =
            loadout.iter().map(|m| (m.port.clone(), m)).collect();
        let mut found = Vec::new();
        while let Some((path, m)) = stack.pop() {
            let Some(item) = m.item else { continue };
            if vehicles.get(&item).is_some() {
                continue;
            }
            found.push((Place::Port(path.clone()), item));
            stack.extend(m.children.iter().map(|c| (format!("{path}/{}", c.port), c)));
        }
        for e in entities {
            let place = |path: String| Place::Interior {
                container: e.container,
                name: e.name.clone(),
                transform: e.transform,
                path,
            };
            found.push((place(String::new()), e.class));
            let children = vehicles.default_children(e.class, containers);
            let mut stack: Vec<(String, &Mounted)> =
                children.iter().map(|m| (m.port.clone(), m)).collect();
            while let Some((path, m)) = stack.pop() {
                if let Some(item) = m.item {
                    found.push((place(path.clone()), item));
                }
                stack.extend(m.children.iter().map(|c| (format!("{path}/{}", c.port), c)));
            }
        }
        for (place, item) in found {
            add(place, item);
        }
        out
    }

    fn add(
        &mut self,
        place: Place,
        item: Guid,
        items: &Items,
        containers: &PortContainers,
        kinds: &Kinds,
    ) {
        if let Some(inv) = items.get(&item).and_then(|i| i.inventory.as_ref()) {
            match inv.kind {
                InventoryKind::Open { external } => self.cargo.push(CargoGrid {
                    place: place.clone(),
                    item,
                    scu: inv.scu().unwrap_or(0.0),
                    dimensions: inv.dimensions,
                    external,
                }),
                InventoryKind::OpenAlways => self.cargo.push(CargoGrid {
                    place: place.clone(),
                    item,
                    scu: inv.scu().unwrap_or(0.0),
                    dimensions: inv.dimensions,
                    external: false,
                }),
                InventoryKind::Closed => self.lockers.push(Locker {
                    place: place.clone(),
                    item,
                    capacity: inv.capacity.unwrap_or(0.0),
                }),
                InventoryKind::Other => {}
            }
        }
        let Some(container) = containers.get(&item) else {
            return;
        };
        let slots: Vec<Slot> = container
            .ports
            .iter()
            .map(|p| Slot {
                port: p.name.clone(),
                min_size: p.min_size,
                max_size: p.max_size,
                accepts: kinds.accepted(p),
            })
            .filter(|s| !s.accepts.is_empty())
            .collect();
        let takes = |pred: fn(&EItemType) -> bool| {
            slots
                .iter()
                .any(|s| s.accepts.iter().any(|t| pred(&t.item_type)))
        };
        let suits = takes(is_armour);
        let weapons = takes(|t| *t == EItemType::WeaponPersonal);
        if suits {
            self.suit_lockers.push(Rack {
                place: place.clone(),
                item,
                slots: slots.clone(),
            });
        }
        if weapons {
            self.weapon_racks.push(Rack { place, item, slots });
        }
    }

    /// Every grid's SCU, internal and external.
    pub fn cargo_scu(&self) -> f64 {
        self.cargo.iter().fold(0.0, |sum, g| sum + g.scu)
    }

    /// Every locker's capacity, SCU.
    pub fn locker_scu(&self) -> f64 {
        self.lockers.iter().fold(0.0, |sum, l| sum + l.capacity)
    }
}

fn is_armour(t: &EItemType) -> bool {
    matches!(
        t,
        EItemType::Char_Armor_Helmet
            | EItemType::Char_Armor_Torso
            | EItemType::Char_Armor_Arms
            | EItemType::Char_Armor_Legs
            | EItemType::Char_Armor_Undersuit
            | EItemType::Char_Armor_Backpack
    )
}

/// Every (type, sub-type, size) an item has: what a slot can take.
struct Kinds(BTreeSet<(String, String, i32)>);

impl Kinds {
    fn new(items: &Items) -> Self {
        Self(
            items
                .values()
                .map(|i| {
                    (
                        i.item_type.as_dcb_str().to_string(),
                        i.item_sub_type.as_dcb_str().to_string(),
                        i.size,
                    )
                })
                .collect(),
        )
    }

    /// Per item type, the sub-types of items that match `port`'s types and
    /// size range.
    fn accepted(&self, port: &Port) -> Vec<PortType> {
        let mut by_type: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for (t, sub, size) in &self.0 {
            if *size < port.min_size || *size > port.max_size {
                continue;
            }
            let matches = port.types.iter().any(|pt| {
                pt.item_type.as_dcb_str() == t
                    && (pt.sub_types.is_empty()
                        || pt.sub_types.iter().any(|s| s.as_dcb_str() == sub))
            });
            if matches {
                by_type.entry(t).or_default().insert(sub);
            }
        }
        by_type
            .into_iter()
            .map(|(t, subs)| PortType {
                item_type: EItemType::from_dcb_str(t),
                sub_types: subs.into_iter().map(EItemSubType::from_dcb_str).collect(),
            })
            .collect()
    }
}
