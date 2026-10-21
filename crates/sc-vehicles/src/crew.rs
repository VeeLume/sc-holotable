//! Who crews a vehicle: its seats and what each one controls.
//!
//! Read from LIVE 4.10, where a seat's port says what its occupant may
//! control: a list of [`PriorityGroup`]s by item type, each with a default
//! priority and priorities for items whose port carries a given
//! controllable tag ([`Port::user_control`]). Of the seats claiming an item,
//! the highest priority controls it. Controller items pass control on the
//! same way ([`Port::usable_control`]): the Constellation Mk5's gunner seat
//! claims the weapon controller tagged `gunnerSeatLeft`, which claims the
//! lower turret. Nothing here matches a port or seat name.
//!
//! Against the crew size each ship record states, the roles below give the
//! same count on 108 of the 149 vehicle XMLs. Where they differ, the seat
//! controls nothing a [`Role`] covers (engineering and scanning consoles,
//! the Freelancer's copilot: the pilot outranks it on everything), or the
//! ship's seats sit in modules or in the interior rather than its ports.

use sc_extract::generated::{EItemSubType, EItemType};
use sc_extract::{Guid, RecordCollection};
use sc_items::Items;
use serde::{Deserialize, Serialize};

use crate::{Mounted, Port, PortContainers, Vehicle, Vehicles};

/// A vehicle's seats.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Crew {
    /// Every seat in the loadout, crewed or not, in loadout order.
    pub stations: Vec<Station>,
}

/// A seat: a `Seat` item, or a manned turret (the turret is the seat).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    /// Port path (`hardpoint_seat_pilot`).
    pub port: String,
    pub item: Guid,
    /// Empty for a passenger seat.
    pub roles: Vec<Role>,
}

/// What a seat's occupant does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// Holds the highest claim on the flight controller (a ground vehicle's
    /// wheeled controller).
    Pilot,
    /// Claims the flight controller below the pilot: takes the helm when
    /// the pilot's seat is empty.
    Copilot,
    /// Sits in the turret.
    MannedTurret,
    /// Controls turrets it does not sit in (port paths), through a weapon
    /// controller or directly. Never set on the pilot.
    RemoteTurret { turrets: Vec<String> },
    /// Controls a mining controller.
    Mining,
    /// Controls a salvage controller.
    Salvage,
}

impl Station {
    /// It has a [`Role`]: crew, not a passenger.
    pub fn is_crewed(&self) -> bool {
        !self.roles.is_empty()
    }

    pub fn has(&self, role: &Role) -> bool {
        self.roles
            .iter()
            .any(|r| std::mem::discriminant(r) == std::mem::discriminant(role))
    }
}

/// An item in the loadout.
struct Node<'a> {
    path: String,
    item: Guid,
    item_type: EItemType,
    port: &'a Port,
}

impl Crew {
    /// The seats of `vehicle` with `loadout` (its [`stock`](Vehicle::stock)
    /// tree or an edited one). Docked vehicles are left out, as they are
    /// from storage and the power network, unless no seat of `vehicle`
    /// flies it: the Caterpillar is flown from its command module, a
    /// vehicle docked to it, whose seats are then listed below its port.
    pub fn compute(
        vehicle: &Vehicle,
        loadout: &[Mounted],
        vehicles: &Vehicles,
        items: &Items,
        containers: &PortContainers,
    ) -> Self {
        let (nodes, docked) = nodes(vehicle, loadout, vehicles, items, containers);
        let stations: Vec<usize> = (0..nodes.len())
            .filter(|&i| is_station(&nodes[i], items))
            .collect();
        let mut roles: Vec<Vec<Role>> = vec![Vec::new(); stations.len()];
        let mut add = |s: usize, role: Role| {
            if !roles[s].contains(&role) {
                roles[s].push(role);
            }
        };

        // Each station's claim on node `n`, as a rank.
        let claims = |n: &Node| -> Vec<Option<u64>> {
            stations
                .iter()
                .map(|&s| claim(&nodes[s].port.user_control, n))
                .collect()
        };
        let top = |ranks: &[Option<u64>]| -> Vec<usize> {
            let best = ranks.iter().flatten().max();
            (0..ranks.len())
                .filter(|&i| best.is_some() && ranks[i].as_ref() == best)
                .collect()
        };

        let mut remote: Vec<(usize, String)> = Vec::new();
        for (i, n) in nodes.iter().enumerate() {
            if stations.contains(&i) {
                continue;
            }
            let ranks = claims(n);
            let winners = top(&ranks);
            match n.item_type {
                EItemType::FlightController | EItemType::WheeledController => {
                    for (s, r) in ranks.iter().enumerate() {
                        match r {
                            Some(_) if winners.contains(&s) => add(s, Role::Pilot),
                            Some(_) => add(s, Role::Copilot),
                            None => {}
                        }
                    }
                }
                EItemType::MiningController => winners.iter().for_each(|&s| add(s, Role::Mining)),
                EItemType::SalvageController => winners.iter().for_each(|&s| add(s, Role::Salvage)),
                // A manned turret's own guns are its seat's.
                _ if is_turret(&n.item_type)
                    && !stations
                        .iter()
                        .any(|&s| n.path.starts_with(&format!("{}/", nodes[s].path))) =>
                {
                    // Seats claiming the turret directly, else the seats
                    // on top of the controllers claiming it.
                    let mut seats = winners.clone();
                    if seats.is_empty() {
                        let controllers: Vec<Option<u64>> = nodes
                            .iter()
                            .enumerate()
                            .map(|(c, ctl)| {
                                (!stations.contains(&c))
                                    .then(|| claim(&ctl.port.usable_control, n))
                                    .flatten()
                            })
                            .collect();
                        for c in top(&controllers) {
                            seats.extend(top(&claims(&nodes[c])));
                        }
                    }
                    remote.extend(seats.into_iter().map(|s| (s, n.path.clone())));
                }
                _ => {}
            }
        }
        for (s, &i) in stations.iter().enumerate() {
            if items
                .get(&nodes[i].item)
                .is_some_and(|it| it.item_sub_type == EItemSubType::MannedTurret)
            {
                add(s, Role::MannedTurret);
            }
        }
        for (s, r) in roles.iter_mut().enumerate() {
            let pilot = r.contains(&Role::Pilot);
            if pilot {
                r.retain(|x| *x != Role::Copilot);
                continue;
            }
            let turrets: Vec<String> = remote
                .iter()
                .filter(|(x, _)| *x == s)
                .map(|(_, p)| p.clone())
                .collect();
            if !turrets.is_empty() {
                r.push(Role::RemoteTurret { turrets });
            }
        }

        let flown = roles.iter().any(|r| r.contains(&Role::Pilot));
        let mut out = Self {
            stations: stations
                .iter()
                .zip(roles)
                .map(|(&i, roles)| Station {
                    port: nodes[i].path.clone(),
                    item: nodes[i].item,
                    roles,
                })
                .collect(),
        };
        if !flown {
            for (path, m) in docked {
                let Some(v) = m.item.and_then(|g| vehicles.get(&g)) else {
                    continue;
                };
                let inner = Self::compute(v, &m.children, vehicles, items, containers);
                out.stations
                    .extend(inner.stations.into_iter().map(|s| Station {
                        port: format!("{path}/{}", s.port),
                        ..s
                    }));
            }
        }
        out
    }

    /// The seats with a [`Role`].
    pub fn crewed(&self) -> impl Iterator<Item = &Station> + '_ {
        self.stations.iter().filter(|s| s.is_crewed())
    }

    /// How many seats have a [`Role`]: the crew the vehicle can use.
    pub fn size(&self) -> usize {
        self.crewed().count()
    }

    /// How many seats have `role` (its data ignored: any
    /// [`Role::RemoteTurret`] counts).
    pub fn count(&self, role: &Role) -> usize {
        self.stations.iter().filter(|s| s.has(role)).count()
    }
}

/// Every item in the loadout with the port it sits in, docked vehicles
/// and what they carry left out (returned apart, with their paths).
fn nodes<'a, 'm>(
    vehicle: &'a Vehicle,
    loadout: &'m [Mounted],
    vehicles: &'a Vehicles,
    items: &Items,
    containers: &'a PortContainers,
) -> (Vec<Node<'a>>, Vec<(String, &'m Mounted)>) {
    let mut out = Vec::new();
    let mut docked = Vec::new();
    let mut stack: Vec<(String, &Mounted, &'a [Port])> = loadout
        .iter()
        .rev()
        .map(|m| (m.port.clone(), m, &vehicle.ports[..]))
        .collect();
    while let Some((path, m, ports)) = stack.pop() {
        let Some(item) = m.item else { continue };
        if vehicles.get(&item).is_some() {
            docked.push((path, m));
            continue;
        }
        let Some(port) = ports.iter().find(|p| p.name.eq_ignore_ascii_case(&m.port)) else {
            continue;
        };
        let item_type = items
            .get(&item)
            .map_or(EItemType::Unrecognized(String::new()), |i| {
                i.item_type.clone()
            });
        out.push(Node {
            path: path.clone(),
            item,
            item_type,
            port,
        });
        let below = vehicles.ports_of(&item, containers);
        stack.extend(
            m.children
                .iter()
                .rev()
                .map(|c| (format!("{path}/{}", c.port), c, below)),
        );
    }
    (out, docked)
}

fn is_station(n: &Node, items: &Items) -> bool {
    n.item_type == EItemType::Seat
        || items
            .get(&n.item)
            .is_some_and(|i| i.item_sub_type == EItemSubType::MannedTurret)
}

fn is_turret(t: &EItemType) -> bool {
    matches!(
        t,
        EItemType::Turret | EItemType::TurretBase | EItemType::UtilityTurret
    )
}

/// The highest claim `groups` make on the item at node `n`.
fn claim(groups: &[sc_extract::PriorityGroup], n: &Node) -> Option<u64> {
    groups
        .iter()
        .filter(|g| g.item_type == n.item_type)
        .filter_map(|g| g.claim(&n.port.controllable_tags).rank())
        .max()
}
