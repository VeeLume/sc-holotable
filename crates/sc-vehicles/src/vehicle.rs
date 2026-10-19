//! Vehicles: hull, ports and stock loadout.

use std::collections::{HashMap, HashSet};

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EAEntityInclusionMode, EItemSubType, EntityClassDefinition,
    EntityClassStaticDataParamsPtr, SSCSignatureSystemCrossSectionParamsPtr,
};
use sc_extract::{
    AssetSource, DataPools, Datacore, Guid, LocaleKey, RecordCollection, Value, VehicleDefinition,
    VehiclePart,
};
use sc_items::Items;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::{LoadoutEntry, Port, PortContainers};

/// How deep a stock loadout is followed (vehicle → turret → gimbal → gun →
/// attachment is 4).
const MAX_DEPTH: usize = 8;

/// A ship or ground vehicle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    /// `VehicleComponentParams.vehicleName`.
    pub name_key: Option<LocaleKey>,
    /// `VehicleComponentParams.vehicleDefinition`: the hull XML.
    pub definition: String,
    /// `VehicleComponentParams.modification` applied to it (empty: none).
    pub modification: String,
    /// `VehicleComponentParams.crewSize`.
    pub crew_size: i32,
    /// `VehicleComponentParams.maxBoundingBoxSize`, m.
    pub bounding_box: Option<[f32; 3]>,
    /// `SSCSignatureSystemParams.radarProperties.crossSectionParams`: the
    /// radar cross-section seen along x / y / z (erkul and spviewer show the
    /// largest). `None` when it comes from the geometry (an SDF
    /// cross-section, e.g. the Golem) rather than the data.
    #[serde(default)]
    pub cross_section: Option<[f32; 3]>,
    /// `SGeometryResourceParams.Geometry.Geometry.Geometry.path`: the hull
    /// geometry (`.cga`), whose helper nodes place the item ports.
    #[serde(default)]
    pub geometry: Option<String>,
    /// `VehicleComponentParams.objectContainers`: the interior and other
    /// object containers placed in the ship (see [`crate::Interior`]).
    #[serde(default)]
    pub object_containers: Vec<crate::ObjectContainer>,
    /// `SEntityInsuranceProperties.shipInsuranceParams`: what an insurance
    /// claim costs in time and money; `None` for vehicles that cannot be
    /// insured.
    #[serde(default)]
    pub insurance: Option<Insurance>,
    /// The base hull its insurance names
    /// (`shipInsuranceParams.shipEntityClassName`): the record a claim
    /// returns. `None` when empty or stale.
    #[serde(default)]
    pub insured_as: Option<Guid>,
    /// Its crew manifest (`SEntityComponentCrewManifestParams`) lists AI
    /// crew: an NPC-flown record.
    #[serde(default)]
    pub ai_crew: bool,
    /// `EAEntityDataParams.inclusionMode` is `ReadyToInclude`.
    #[serde(default)]
    pub ea_ready: bool,
    /// A reward struct (`ContractResult_Item.entityClass`,
    /// `ItemAwardEntityClass.entityClass`, `SWeightedRewardEntry.
    /// rewardEntityRecord`) hands it out: a Wikelo, PYAM Exec or BTALA
    /// reward ship. Rentals are not rewards.
    #[serde(default)]
    pub reward: bool,
    /// `SEntityComponentHostilityCompositeParams.useOnlyPrimaryChildForHostility`:
    /// set on mission and AI ships only (hijacked / drug-run hulls).
    #[serde(default)]
    pub scenario: bool,
    /// `VehicleComponentParams.unmoveable`: a display hull.
    #[serde(default)]
    pub display: bool,
    /// `aiEngineeringOptions.canExplodeFromHullDamage`.
    #[serde(default)]
    pub explodes_from_hull_damage: bool,
    /// `AttachDef.SubType` is `Vehicle_GroundVehicle`.
    #[serde(default)]
    pub ground: bool,
    /// `VehicleComponentParams.isGravlevVehicle`.
    #[serde(default)]
    pub gravlev: bool,
    /// What a calculator should list it as.
    #[serde(default)]
    pub listing: Listing,
    /// How penetrating hits treat its components.
    #[serde(default)]
    pub penetration: PenetrationRules,
    /// The hull's parts (vehicle XML), each with its HP and mass.
    pub hull: Vec<VehiclePart>,
    /// The vehicle XML's `itemPortTags`.
    pub item_port_tags: Vec<String>,
    /// The vehicle entity's own `AttachDef.Tags`.
    pub tags: Vec<String>,
    /// The hull's ports (vehicle XML) then the entity's own (DCB).
    pub ports: Vec<Port>,
    /// What every port holds as the vehicle ships.
    pub stock: Vec<Mounted>,
}

/// What a vehicle record is to a player. No DCB field marks a player
/// ship (LIVE 4.10: insurance, entitlement, purchasable, inclusion and tags
/// all also sit on AI records), so this combines signals, in order:
///
/// 1. a record a reward struct hands out is a [`Reward`](Self::Reward);
/// 2. a scenario hull ([`Vehicle::scenario`]) or a display hull
///    ([`Vehicle::display`]) is a variant;
/// 3. a record some vehicle's insurance names (`shipEntityClassName`, the
///    only variant-to-base link) is a [`Base`](Self::Base), unless it is
///    not ready for inclusion and no loadout docks it;
/// 4. otherwise, a record with its own `vehicleName`, no AI crew, ready
///    for inclusion and not exploding from hull damage (the AI templates)
///    is an [`Edition`](Self::Edition);
/// 5. the rest are variants.
///
/// LIVE 4.10: 246 listed records (240 names), every one of erkul's 222
/// (joined by GUID) plus 24 the data cannot tell apart: event editions
/// (BIS 2949 Showdowns, CitizenCon), Teach's Specials, five old hulls
/// with a newer `_GS` twin of the same name, one TEMP record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Listing {
    /// A base hull: some vehicle's insurance names it.
    Base,
    /// An edition of a base: its own `vehicleName`, no AI crew, ready for
    /// inclusion.
    Edition,
    /// A ship a contract or reward terminal hands out.
    Reward,
    /// An AI, mission, display or same-named variant.
    #[default]
    Variant,
}

/// `VehicleComponentParams` penetration multipliers (client `CVehicle::
/// OnHit`, LIVE 4.10).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PenetrationRules {
    /// `fusePenetrationDamageMultiplier`: on a relay, the damage goes to
    /// its first live fuse times this.
    pub fuse_multiplier: f32,
    /// `componentPenetrationDamageMultiplier`: every other component.
    pub component_multiplier: f32,
    /// `aiEngineeringOptions.penetrationCanDamagePowerPlants`: only matters
    /// when an AI flies the ship (players' plants are always hit).
    pub ai_power_plants: bool,
}

impl Default for PenetrationRules {
    fn default() -> Self {
        Self {
            fuse_multiplier: 1.0,
            component_multiplier: 1.0,
            ai_power_plants: true,
        }
    }
}

/// `ShipInsuranceParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Insurance {
    /// `baseWaitTimeMinutes` / `mandatoryWaitTimeMinutes`.
    pub base_wait_minutes: f32,
    pub mandatory_wait_minutes: f32,
    /// `baseExpeditingFee`, aUEC.
    pub expediting_fee: u32,
}

/// A port of a loadout with what it holds, and what that holds in turn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mounted {
    pub port: String,
    pub item: Option<Guid>,
    pub children: Vec<Mounted>,
}

impl Vehicle {
    /// The hull's hit points: every part's `damageMax`.
    pub fn hull_health(&self) -> f32 {
        self.hull.iter().map(|p| p.damage_max).sum()
    }

    /// The hull's mass, kg, without the loadout: every part's mass (the
    /// root body plus detachable pieces, e.g. the Skylark's 11 panels of
    /// 10.5 kg).
    pub fn hull_mass(&self) -> Option<f32> {
        self.hull.iter().filter_map(|p| p.mass).reduce(|a, b| a + b)
    }

    /// The port named `name` (case-insensitive, as the game matches them).
    pub fn port(&self, name: &str) -> Option<&Port> {
        self.ports
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }

    /// The tags an item sees from above when it goes into the last port of
    /// `chain` (outermost port first) whose holders are `holders` (the items
    /// carrying each port after the first): every port's `portTags`, every
    /// holder's tags, the vehicle XML's `itemPortTags` and the vehicle
    /// entity's tags. Pass it to [`fits`](crate::fits).
    pub fn offered_tags<'a>(
        &'a self,
        items: &'a Items,
        chain: &[&'a Port],
        holders: &[Guid],
    ) -> Vec<&'a str> {
        chain
            .iter()
            .flat_map(|p| p.port_tags.iter())
            .chain(
                holders
                    .iter()
                    .filter_map(|g| items.get(g))
                    .flat_map(|i| i.tags.iter()),
            )
            .chain(&self.item_port_tags)
            .chain(&self.tags)
            .map(String::as_str)
            .collect()
    }
}

/// Every vehicle (ships and ground vehicles; AI and edition records
/// included), keyed by `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vehicles {
    by_guid: HashMap<Guid, Vehicle>,
}

impl Vehicles {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read every entity with `VehicleComponentParams`, its hull XML (with
    /// its modification) and its stock loadout. A vehicle whose XML cannot
    /// be read keeps its DCB ports and an empty hull, with a warning.
    pub fn build(
        datacore: &Datacore,
        items: &Items,
        containers: &PortContainers,
        assets: &AssetSource,
    ) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let class_names = crate::container::class_names(datacore);
        let mut hulls: HashMap<(String, String), Option<VehicleDefinition>> = HashMap::new();
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let Some(params) = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::VehicleComponentParams(h) => h.get(pools),
                _ => None,
            }) else {
                continue;
            };
            if params.vehicle_definition.is_empty() {
                continue;
            }
            let key = (
                params.vehicle_definition.to_ascii_lowercase(),
                params.modification.clone(),
            );
            let hull = hulls.entry(key).or_insert_with(|| {
                VehicleDefinition::read(assets, &params.vehicle_definition, &params.modification)
                    .inspect_err(|error| {
                        warn!(%guid, path = params.vehicle_definition, %error, "vehicle XML unreadable");
                    })
                    .ok()
            });
            let mut ports: Vec<Port> = hull
                .iter()
                .flat_map(|h| &h.parts)
                .filter_map(|p| Some(Port::from_hull(&p.name, p.port.as_ref()?)))
                .collect();
            if let Some(c) = containers.get(&guid) {
                ports.extend(c.ports.iter().cloned());
            }
            debug!(%guid, ports = ports.len(), "vehicle");
            by_guid.insert(
                guid,
                Vehicle {
                    name_key: (!params.vehicle_name.is_empty())
                        .then(|| params.vehicle_name.clone()),
                    definition: params.vehicle_definition.clone(),
                    modification: params.modification.clone(),
                    crew_size: params.crew_size,
                    bounding_box: params
                        .max_bounding_box_size
                        .and_then(|h| h.get(pools))
                        .map(|v| [v.x, v.y, v.z]),
                    cross_section: cross_section(ecd, pools),
                    geometry: geometry(ecd, pools),
                    object_containers: params
                        .object_containers
                        .iter()
                        .filter_map(|h| h.get(pools))
                        .filter(|oc| !oc.file_name.is_empty())
                        .map(|oc| {
                            let offset = oc.offset.and_then(|h| h.get(pools));
                            crate::ObjectContainer {
                                file: oc.file_name.clone(),
                                position: offset
                                    .and_then(|q| q.position)
                                    .and_then(|h| h.get(pools))
                                    .map_or([0.0; 3], |v| [v.x, v.y, v.z]),
                                rotation: offset
                                    .and_then(|q| q.rotation)
                                    .and_then(|h| h.get(pools))
                                    .map_or([0.0; 3], |a| [a.x, a.y, a.z]),
                                bone: (!oc.bone_name.is_empty()).then(|| oc.bone_name.clone()),
                            }
                        })
                        .collect(),
                    insurance: insurance(ecd, pools),
                    insured_as: insured_class(ecd, pools)
                        .filter(|n| !n.is_empty())
                        .and_then(|n| {
                            let hit = class_names.get(&n.to_ascii_lowercase()).copied();
                            if hit.is_none() {
                                warn!(%guid, class = n, "insurance names no vehicle record");
                            }
                            hit
                        }),
                    ai_crew: ecd
                        .components
                        .iter()
                        .find_map(|c| match c {
                            DataForgeComponentParamsPtr::SEntityComponentCrewManifestParams(h) => {
                                h.get(pools)?.crew_manifest
                            }
                            _ => None,
                        })
                        .and_then(|m| store.records.multi_feature.crew_manifest.get(&m))
                        .and_then(|h| h.get(pools))
                        .is_some_and(|m| !m.crew.is_empty()),
                    ea_ready: ecd.static_entity_class_data.iter().any(|d| {
                        matches!(d, EntityClassStaticDataParamsPtr::EAEntityDataParams(h)
                            if h.get(pools).is_some_and(|e| e.inclusion_mode == EAEntityInclusionMode::ReadyToInclude))
                    }),
                    reward: false,
                    scenario: ecd.components.iter().any(|c| {
                        matches!(c, DataForgeComponentParamsPtr::SEntityComponentHostilityCompositeParams(h)
                            if h.get(pools).is_some_and(|p| p.use_only_primary_child_for_hostility))
                    }),
                    display: params.unmoveable,
                    explodes_from_hull_damage: params
                        .ai_engineering_options
                        .and_then(|h| h.get(pools))
                        .is_some_and(|o| o.can_explode_from_hull_damage),
                    ground: items
                        .get(&guid)
                        .is_some_and(|i| i.item_sub_type == EItemSubType::Vehicle_GroundVehicle),
                    gravlev: params.is_gravlev_vehicle,
                    penetration: PenetrationRules {
                        fuse_multiplier: params.fuse_penetration_damage_multiplier,
                        component_multiplier: params.component_penetration_damage_multiplier,
                        ai_power_plants: params
                            .ai_engineering_options
                            .and_then(|h| h.get(pools))
                            .is_none_or(|o| o.penetration_can_damage_power_plants),
                    },
                    listing: Listing::Variant,
                    hull: hull.as_ref().map(|h| h.parts.clone()).unwrap_or_default(),
                    item_port_tags: hull
                        .as_ref()
                        .map(|h| h.item_port_tags.clone())
                        .unwrap_or_default(),
                    tags: items.get(&guid).map(|i| i.tags.clone()).unwrap_or_default(),
                    ports,
                    stock: Vec::new(),
                },
            );
        }

        // Stock loadouts, once every vehicle's ports are known (a docked
        // vehicle's ports come from its own hull).
        let stocks: Vec<(Guid, Vec<Mounted>)> = by_guid
            .iter()
            .map(|(guid, v)| {
                let entries = containers
                    .get(guid)
                    .map_or(&[][..], |c| &c.default_loadout[..]);
                (*guid, mount(&v.ports, entries, &by_guid, containers, 0))
            })
            .collect();
        for (guid, stock) in stocks {
            if let Some(v) = by_guid.get_mut(&guid) {
                v.stock = stock;
            }
        }

        // Listing (see `Listing`).
        let rewards = reward_classes(datacore);
        let bases: HashSet<Guid> = by_guid.values().filter_map(|v| v.insured_as).collect();
        let docked: HashSet<Guid> = by_guid
            .values()
            .flat_map(|v| carried(&v.stock))
            .filter(|g| by_guid.contains_key(g))
            .collect();
        let names: HashMap<Guid, Option<LocaleKey>> = by_guid
            .iter()
            .map(|(g, v)| (*g, v.name_key.clone()))
            .collect();
        for (guid, v) in &mut by_guid {
            v.reward = rewards.contains(guid);
            v.listing = if v.reward {
                Listing::Reward
            } else if v.scenario || v.display {
                Listing::Variant
            } else if bases.contains(guid) {
                if v.ea_ready || docked.contains(guid) {
                    Listing::Base
                } else {
                    Listing::Variant
                }
            } else {
                let own_name = v
                    .insured_as
                    .and_then(|b| names.get(&b))
                    .is_none_or(|base| *base != v.name_key);
                if own_name && !v.ai_crew && v.ea_ready && !v.explodes_from_hull_damage {
                    Listing::Edition
                } else {
                    Listing::Variant
                }
            };
        }
        Self { by_guid }
    }

    /// The vehicles a calculator lists: [`Listing::Base`] and
    /// [`Listing::Edition`] ([`Listing::Reward`] ships are separate).
    pub fn listed(&self) -> impl Iterator<Item = (&Guid, &Vehicle)> + '_ {
        self.by_guid
            .iter()
            .filter(|(_, v)| matches!(v.listing, Listing::Base | Listing::Edition))
    }

    /// What `item` brings below it when mounted: its ports (a docked
    /// vehicle's from its hull), filled from its own default loadout.
    pub(crate) fn default_children(&self, item: Guid, containers: &PortContainers) -> Vec<Mounted> {
        let container = containers.get(&item);
        let ports = match self.by_guid.get(&item) {
            Some(v) => &v.ports[..],
            None => container.map_or(&[][..], |c| &c.ports[..]),
        };
        let entries = container.map_or(&[][..], |c| &c.default_loadout[..]);
        mount(ports, entries, &self.by_guid, containers, 1)
    }

    /// The ports an item mounted in a loadout offers: a docked vehicle's
    /// hull ports, else the item's own.
    pub(crate) fn ports_of<'a>(
        &'a self,
        item: &Guid,
        containers: &'a PortContainers,
    ) -> &'a [Port] {
        match self.by_guid.get(item) {
            Some(v) => &v.ports,
            None => containers.get(item).map_or(&[][..], |c| &c.ports[..]),
        }
    }
}

/// Every item a loadout tree carries.
fn carried(level: &[Mounted]) -> Vec<Guid> {
    let mut out = Vec::new();
    let mut stack: Vec<&Mounted> = level.iter().collect();
    while let Some(m) = stack.pop() {
        out.extend(m.item);
        stack.extend(&m.children);
    }
    out
}

/// The entity classes the reward structs hand out. Those structs sit in
/// contract and reward-terminal records outside this crate's features, so
/// their instances are walked on the raw layer by struct type and read by
/// their typed `Reference` field (no record names involved).
fn reward_classes(datacore: &Datacore) -> HashSet<Guid> {
    const REWARDS: [(&str, &str); 3] = [
        ("ContractResult_Item", "entityClass"),
        ("ItemAwardEntityClass", "entityClass"),
        ("SWeightedRewardEntry", "rewardEntityRecord"),
    ];
    let db = datacore.db();
    let mut out = HashSet::new();
    for m in db.data_mappings() {
        let Ok(index) = usize::try_from(m.struct_index) else {
            continue;
        };
        let Some(field) = db
            .struct_name(index)
            .and_then(|n| REWARDS.iter().find(|(s, _)| *s == n))
            .map(|(_, f)| *f)
        else {
            continue;
        };
        for i in 0..m.struct_count {
            if let Some(Value::Reference(Some(r))) = db.instance(index as u32, i).get(field) {
                out.insert(r.guid);
            }
        }
    }
    out
}

/// The class its insurance claims return (`shipEntityClassName`): an
/// `EntityClassDefinition` record name, matched without case as the game's
/// class registry does. The only link from a variant to its base hull; the
/// data has no GUID for it.
fn insured_class<'a>(ecd: &EntityClassDefinition, pools: &'a DataPools) -> Option<&'a str> {
    ecd.static_entity_class_data.iter().find_map(|d| match d {
        EntityClassStaticDataParamsPtr::SEntityInsuranceProperties(h) => Some(
            h.get(pools)?
                .ship_insurance_params?
                .get(pools)?
                .ship_entity_class_name
                .as_str(),
        ),
        _ => None,
    })
}

/// The entity's ship insurance terms.
fn insurance(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<Insurance> {
    ecd.static_entity_class_data.iter().find_map(|d| match d {
        EntityClassStaticDataParamsPtr::SEntityInsuranceProperties(h) => {
            let p = h.get(pools)?.ship_insurance_params?.get(pools)?;
            Some(Insurance {
                base_wait_minutes: p.base_wait_time_minutes,
                mandatory_wait_minutes: p.mandatory_wait_time_minutes,
                expediting_fee: p.base_expediting_fee,
            })
        }
        _ => None,
    })
}

/// The entity's root geometry path.
pub(crate) fn geometry(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<String> {
    let g = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SGeometryResourceParams(h) => h.get(pools),
        _ => None,
    })?;
    let path = &g
        .geometry?
        .get(pools)?
        .geometry?
        .get(pools)?
        .geometry?
        .get(pools)?
        .path;
    (!path.is_empty()).then(|| path.clone())
}

/// The manual radar cross-section of a vehicle entity.
fn cross_section(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<[f32; 3]> {
    let params = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SSCSignatureSystemParams(h) => h.get(pools),
        _ => None,
    })?;
    match params
        .radar_properties
        .and_then(|h| h.get(pools))?
        .cross_section_params
        .as_ref()?
    {
        SSCSignatureSystemCrossSectionParamsPtr::SSCSignatureSystemManualCrossSectionParams(h) => {
            let v = h.get(pools)?.cross_section?.get(pools)?;
            Some([v.x, v.y, v.z])
        }
        _ => None,
    }
}

/// Every item of `loadout` on the vehicle's own resource network, with its
/// port path (`turret/gimbal/gun`), depth first. A docked vehicle (an item
/// that is itself in `vehicles`, like the Centaurus's Skylark) runs its own
/// network, so it and what it carries are left out.
pub fn network_items(loadout: &[Mounted], vehicles: &Vehicles) -> Vec<(String, Guid)> {
    let mut out = Vec::new();
    let mut stack: Vec<(String, &Mounted)> =
        loadout.iter().rev().map(|m| (m.port.clone(), m)).collect();
    while let Some((path, m)) = stack.pop() {
        let Some(item) = m.item else {
            continue;
        };
        if vehicles.get(&item).is_some() {
            continue;
        }
        out.push((path.clone(), item));
        stack.extend(
            m.children
                .iter()
                .rev()
                .map(|c| (format!("{path}/{}", c.port), c)),
        );
    }
    out
}

/// What `ports` hold under the loadout `entries`: an entry's item, or else
/// the port's default item; below it the entry's nested loadout, or else the
/// item's own default loadout. Entries naming no listed port follow at the
/// end (the port is defined elsewhere, e.g. a docked vehicle's hull).
fn mount(
    ports: &[Port],
    entries: &[LoadoutEntry],
    vehicles: &HashMap<Guid, Vehicle>,
    containers: &PortContainers,
    depth: usize,
) -> Vec<Mounted> {
    let entry_for = |port: &str| entries.iter().find(|e| e.port.eq_ignore_ascii_case(port));
    let below = |item: Option<Guid>, nested: &[LoadoutEntry]| -> Vec<Mounted> {
        let Some(item) = item else {
            return Vec::new();
        };
        if depth + 1 >= MAX_DEPTH {
            return Vec::new();
        }
        let container = containers.get(&item);
        let child_ports: &[Port] = match vehicles.get(&item) {
            Some(v) => &v.ports,
            None => container.map_or(&[][..], |c| &c.ports[..]),
        };
        let child_entries = if nested.is_empty() {
            container.map_or(&[][..], |c| &c.default_loadout[..])
        } else {
            nested
        };
        mount(child_ports, child_entries, vehicles, containers, depth + 1)
    };
    let mut out: Vec<Mounted> = ports
        .iter()
        .map(|port| {
            let entry = entry_for(&port.name);
            let item = entry.and_then(|e| e.item).or(port.default_item);
            Mounted {
                port: port.name.clone(),
                item,
                children: below(item, entry.map_or(&[][..], |e| &e.children[..])),
            }
        })
        .collect();
    for e in entries {
        if !ports.iter().any(|p| p.name.eq_ignore_ascii_case(&e.port)) {
            out.push(Mounted {
                port: e.port.clone(),
                item: e.item,
                children: below(e.item, &e.children),
            });
        }
    }
    out
}

impl RecordCollection for Vehicles {
    type Item = Vehicle;

    fn get(&self, guid: &Guid) -> Option<&Vehicle> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Vehicle)> + '_ {
        self.by_guid.iter()
    }
}
