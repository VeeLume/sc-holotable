//! How a player comes by an item or a vehicle, as far as the client data
//! shows — a domain crate.
//!
//! [`Acquisitions`] holds one [`Acquisition`] per item or vehicle record:
//! the [`Source`]s that make it obtainable, the [`Mount`]s where a vehicle's
//! stock carries it, and for a vehicle its [`Listing`]. Nothing is filtered
//! out; [`Acquisition::status`] classifies.
//!
//! # Why three states
//!
//! Shop stock never reaches the client data (kiosks get it from the game
//! server), so a part sold only in shops shows no source here. No part can
//! be proven unobtainable from the data: everything without a source is
//! [`Status::Unknown`], so shop-only parts stay visible. Only a vehicle can
//! read [`Status::Unobtainable`]: an AI, mission or display variant
//! ([`Listing::Variant`]).
//!
//! Where a part is mounted is no proof either. [`Acquisition::locked_only`]
//! (every stock mount a locked port) holds for the Bengal turret's Slayer
//! cannon, but also for the M4A laser cannon (`BEHR_LaserCannon_S2`), which
//! shops sell and whose only stock mounts are orbital sentry turrets. And
//! `SCItemPurchasableParams` sits on NPC-only guns too (all 13 Bengal,
//! Vanduul and sentry guns on LIVE 4.10), so it marks no shop item
//! (`acquisitions_probe`).
//!
//! # Sources
//!
//! - [`Source::Listed`]: a player vehicle itself ([`Listing::Base`],
//!   [`Listing::Edition`] or [`Listing::Reward`]; see [`Listing`] for how
//!   the data tells them from AI variants).
//! - [`Source::Stock`]: carried by a player vehicle's stock loadout, at any
//!   depth (a gun on a gimbal on a turret, a docked snub).
//! - [`Source::Craft`]: made by a blueprint a character starts with
//!   ([`Unlock::Default`], `CraftingGlobalParams`' whitelist) or one a
//!   mission's reward pool can grant ([`Unlock::Missions`], any mission
//!   whose blueprint reward has a chance above 0; the data does not say
//!   which missions the servers run).
//! - [`Source::Reward`]: handed out by missions ([`Grant::Missions`]: a
//!   `ContractResult_Item`, scrip included, or a group of a
//!   `ContractResult_ItemsWeighting` with a weight above 0) or by a reward
//!   terminal ([`Grant::Generator`]: an entity whose
//!   `SRewardGeneratorComponentParams.rewardPool` holds it with a weight
//!   above 0). On LIVE 4.10 that covers every record a reward struct hands
//!   out, except the 15 of two `ItemAwardWeightingsRecord`s no record
//!   references (`IAWR_hangerFlairs`, `IAWR_HighTierShips`). Event ladder
//!   tiers grant badges (`AwardService_Award` holds no item), so items an
//!   event hands out for its badges, such as the Iasi event's Helios
//!   IP-20, are granted server-side and read [`Status::Unknown`].
//!
//! - [`Source::Loot`]: a loot table some lootable carries can drop it
//!   ([`sc_loot`]: corpses, NPC actors, world containers), and that
//!   lootable's pool filter admits it. Spawn-with picks count; secondary
//!   choices only weight the picks. Weights and chances are not weighed: a
//!   pick of weight 0.001 counts.
//!
//! NPC spawns are not modelled yet; parts that only they supply read
//! [`Status::Unknown`].
//!
//! LIVE 4.10: 298 vehicles are obtainable (221 base, 25 editions, 52
//! rewards) and 849 variants unobtainable (`acquisitions_probe`).
//!
//! ```no_run
//! use sc_acquisitions::{Acquisitions, RecordCollection, Status};
//! use sc_crafting::Blueprints;
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
//! use sc_items::Items;
//! use sc_loot::{LootTables, Lootables};
//! use sc_missions::Missions;
//! use sc_tags::Tags;
//! use sc_vehicles::{PortContainers, Vehicles};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let containers = PortContainers::build(&datacore, &assets);
//! let vehicles = Vehicles::build(&datacore, &items, &containers, &assets);
//! let blueprints = Blueprints::build(&datacore, &items);
//! let missions = Missions::build(&datacore);
//! let tables = LootTables::build(&datacore, &items, &Tags::build(datacore.records()));
//! let lootables = Lootables::build(&datacore, &tables);
//! let acquisitions = Acquisitions::build(
//!     &datacore, &vehicles, &containers, &blueprints, &missions, &tables, &lootables,
//! );
//! for guid in items.guids().take(5) {
//!     if acquisitions.status(guid) == Status::Obtainable {
//!         println!("{guid}: {:?}", acquisitions.get(guid).map(|a| &a.sources));
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::{HashMap, HashSet};

use sc_crafting::{Blueprints, GlobalParams};
pub use sc_extract::RecordCollection;
use sc_extract::generated::DataForgeComponentParamsPtr;
use sc_extract::{Datacore, Guid};
use sc_loot::{LootConfig, LootTables, Lootables};
use sc_missions::Missions;
pub use sc_vehicles::Listing;
use sc_vehicles::{Mounted, Port, PortContainers, Vehicles, is_editable};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Why a record is obtainable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    /// A player vehicle itself: [`Listing::Base`], [`Listing::Edition`] or
    /// [`Listing::Reward`].
    Listed(Listing),
    /// Carried by the stock loadout of this player vehicle.
    Stock { vehicle: Guid },
    /// Made by this `CraftingBlueprintRecord`, unlocked as `unlock` says.
    Craft { blueprint: Guid, unlock: Unlock },
    /// Handed out as a reward.
    Reward(Grant),
    /// Dropped by this loot table (a `LootTable` or `LootTableV3Record`,
    /// [`sc_loot::LootTables`]) of some lootable.
    Loot { table: Guid },
}

/// What hands out a reward.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grant {
    /// These missions: an item reward, or a weighted item reward group
    /// with a weight above 0. In contract order.
    Missions(Vec<Guid>),
    /// The reward terminal entity whose `SRewardGeneratorComponentParams`
    /// reward pool holds it with a weight above 0.
    Generator(Guid),
}

/// How a character gets a blueprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Unlock {
    /// Every character starts with it (`CraftingGlobalParams`'
    /// `defaultBlueprintSelection` whitelist).
    Default,
    /// These missions' rewards can grant it: a blueprint reward with a
    /// chance above 0 whose pool holds it. In contract order.
    Missions(Vec<Guid>),
}

/// One place a vehicle's stock loadout carries a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mount {
    /// The vehicle whose stock it is.
    pub vehicle: Guid,
    /// The port path from the vehicle down, `turret/gimbal/gun`.
    pub path: String,
    /// Whether the player can change the port ([`is_editable`] on its port
    /// chain). A port the data does not define counts as editable, so it
    /// never makes a part [`locked_only`](Acquisition::locked_only).
    pub editable: bool,
}

/// How a player comes by one record. See [`Acquisition::status`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Acquisition {
    /// Why it is obtainable; empty when no modelled source supplies it.
    pub sources: Vec<Source>,
    /// Every place a vehicle's stock carries it (any vehicle, AI variants
    /// included).
    pub mounts: Vec<Mount>,
    /// The vehicle's [`Listing`]; `None` for an item.
    pub listing: Option<Listing>,
}

/// What [`Acquisition::status`] concludes. See the crate docs for why there
/// are three.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    /// At least one [`Source`].
    Obtainable,
    /// An AI, mission or display vehicle ([`Listing::Variant`]) that no
    /// source supplies, such as a player ship's stock docking it.
    Unobtainable,
    /// No source: nothing known either way. Shop-only parts read this, and
    /// so do parts only NPC spawns supply, until those are modelled.
    /// [`Acquisition::mounts`] is the evidence left.
    Unknown,
}

impl Acquisition {
    /// Classifies the record: see [`Status`].
    pub fn status(&self) -> Status {
        if !self.sources.is_empty() {
            Status::Obtainable
        } else if self.listing == Some(Listing::Variant) {
            Status::Unobtainable
        } else {
            Status::Unknown
        }
    }

    /// Whether some vehicle's stock carries it and every such mount is a
    /// port the player cannot change: a hint for parts NPCs are built
    /// with, not proof (see the crate docs for the M4A, which shops sell).
    pub fn locked_only(&self) -> bool {
        !self.mounts.is_empty() && self.mounts.iter().all(|m| !m.editable)
    }
}

/// Every vehicle, and every item some vehicle's stock carries, an unlockable
/// blueprint makes, a reward hands out or a loot table drops, keyed by
/// `EntityClassDefinition` GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Acquisitions {
    by_guid: HashMap<Guid, Acquisition>,
}

impl Acquisitions {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Classify every vehicle by its [`Listing`] and walk every vehicle's
    /// stock loadout: each item becomes a [`Mount`], and a
    /// [`Source::Stock`] when the vehicle is a player one. `containers`
    /// supplies the ports of the items below the hull, so each mount knows
    /// whether its port can be changed. Then every blueprint a character
    /// starts with (`CraftingGlobalParams`' whitelist, read from
    /// `datacore`) or a mission's reward can grant (`missions`) gives what
    /// it makes a [`Source::Craft`]; and every item a mission's reward or a
    /// reward terminal (`datacore`'s `SRewardGeneratorComponentParams`)
    /// hands out gets a [`Source::Reward`]; every item a lootable's table
    /// can drop past its pool filter (`tables`, `lootables`) a
    /// [`Source::Loot`].
    pub fn build(
        datacore: &Datacore,
        vehicles: &Vehicles,
        containers: &PortContainers,
        blueprints: &Blueprints,
        missions: &Missions,
        tables: &LootTables,
        lootables: &Lootables,
    ) -> Self {
        let mut by_guid: HashMap<Guid, Acquisition> = HashMap::new();
        let mut undefined_ports = 0usize;
        for (&guid, vehicle) in vehicles.iter() {
            let player = vehicle.listing != Listing::Variant;
            let own = by_guid.entry(guid).or_default();
            own.listing = Some(vehicle.listing);
            if player {
                own.sources.push(Source::Listed(vehicle.listing));
            }
            // Depth first: (mounted, path, port chain from the hull down).
            let mut stack: Vec<(&Mounted, String, Vec<Option<&Port>>)> = vehicle
                .stock
                .iter()
                .map(|m| (m, m.port.clone(), vec![vehicle.port(&m.port)]))
                .collect();
            while let Some((m, path, chain)) = stack.pop() {
                let Some(item) = m.item else {
                    continue;
                };
                let editable = match chain.iter().copied().collect::<Option<Vec<&Port>>>() {
                    Some(ports) => is_editable(&ports),
                    None => {
                        undefined_ports += 1;
                        true
                    }
                };
                let entry = by_guid.entry(item).or_default();
                entry.mounts.push(Mount {
                    vehicle: guid,
                    path: path.clone(),
                    editable,
                });
                let source = Source::Stock { vehicle: guid };
                if player && !entry.sources.contains(&source) {
                    entry.sources.push(source);
                }
                let ports = ports_of(item, vehicles, containers);
                for child in &m.children {
                    let mut chain = chain.clone();
                    chain.push(
                        ports
                            .iter()
                            .find(|p| p.name.eq_ignore_ascii_case(&child.port)),
                    );
                    stack.push((child, format!("{path}/{}", child.port), chain));
                }
            }
        }
        debug!(
            undefined_ports,
            "stock mounts on ports the data does not define"
        );
        let crafting = GlobalParams::build(datacore).unwrap_or_default();
        add_crafts(&mut by_guid, blueprints, &crafting, missions);
        add_rewards(&mut by_guid, datacore, missions);
        add_loot(&mut by_guid, tables, lootables);
        info!(records = by_guid.len(), "acquisitions built");
        Self { by_guid }
    }

    /// The [`Status`] of any record: [`Status::Unknown`] for one the
    /// collection does not hold (no vehicle, carried by no stock loadout).
    pub fn status(&self, guid: &Guid) -> Status {
        self.by_guid
            .get(guid)
            .map_or(Status::Unknown, Acquisition::status)
    }
}

/// A [`Source::Loot`] for every item a lootable's table can drop that its
/// pool filter admits.
fn add_loot(by_guid: &mut HashMap<Guid, Acquisition>, tables: &LootTables, lootables: &Lootables) {
    let admitted = |config: &LootConfig, item: &Guid| {
        config
            .pool_filter
            .as_ref()
            .is_none_or(|f| tables.admits(f, item))
    };
    let mut drops: HashSet<(Guid, Guid)> = HashSet::new();
    for lootable in lootables.values() {
        for config in &lootable.configs {
            for table in config.tables() {
                let Some(t) = tables.get(&table) else {
                    continue;
                };
                for item in t.items().filter(|i| admitted(config, i)) {
                    drops.insert((*item, table));
                }
            }
        }
    }
    let mut drops: Vec<(Guid, Guid)> = drops.into_iter().collect();
    drops.sort_by_key(|(item, table)| (item.to_string(), table.to_string()));
    debug!(drops = drops.len(), "item / loot table pairs");
    for (item, table) in drops {
        by_guid
            .entry(item)
            .or_default()
            .sources
            .push(Source::Loot { table });
    }
}

/// A [`Source::Reward`] for every item missions or reward terminals hand
/// out.
fn add_rewards(by_guid: &mut HashMap<Guid, Acquisition>, datacore: &Datacore, missions: &Missions) {
    // Item → the missions that hand it out, in contract order.
    let mut granted: Vec<(Guid, Vec<Guid>)> = Vec::new();
    let mut index: HashMap<Guid, usize> = HashMap::new();
    for mission in &missions.contracts {
        let rewards = &mission.rewards;
        let weighted = rewards
            .weighted_items
            .iter()
            .flat_map(|w| &w.groups)
            .filter(|g| g.weight > 0.0)
            .flat_map(|g| &g.items)
            .map(|r| r.entity_class);
        let handed = (rewards.items.iter().map(|r| r.entity_class))
            .chain(weighted)
            .chain(rewards.scrip.iter().map(|s| s.currency_guid));
        for item in handed {
            let i = *index.entry(item).or_insert_with(|| {
                granted.push((item, Vec::new()));
                granted.len() - 1
            });
            let ids = &mut granted[i].1;
            if !ids.contains(&mission.id) {
                ids.push(mission.id);
            }
        }
    }
    debug!(items = granted.len(), "items missions hand out");
    for (item, ids) in granted {
        by_guid
            .entry(item)
            .or_default()
            .sources
            .push(Source::Reward(Grant::Missions(ids)));
    }

    // Reward terminals, in GUID order so sources do not follow map order.
    let store = datacore.records();
    let pools = &store.pools;
    let mut generators: Vec<(Guid, Vec<Guid>)> = store
        .records
        .multi_feature
        .entity_class_definition
        .iter()
        .filter_map(|(guid, handle)| {
            let ecd = handle.get(pools)?;
            let params = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::SRewardGeneratorComponentParams(h) => h.get(pools),
                _ => None,
            })?;
            let items = params
                .reward_pool
                .iter()
                .filter_map(|h| h.get(pools))
                .filter(|e| e.weight > 0.0)
                .filter_map(|e| e.reward_entity_record)
                .collect();
            Some((*guid, items))
        })
        .collect();
    generators.sort_by_key(|(guid, _)| guid.to_string());
    debug!(generators = generators.len(), "reward terminals");
    for (generator, items) in generators {
        for item in items {
            let source = Source::Reward(Grant::Generator(generator));
            let entry = by_guid.entry(item).or_default();
            if !entry.sources.contains(&source) {
                entry.sources.push(source);
            }
        }
    }
}

/// A [`Source::Craft`] for what every unlockable blueprint makes.
fn add_crafts(
    by_guid: &mut HashMap<Guid, Acquisition>,
    blueprints: &Blueprints,
    crafting: &GlobalParams,
    missions: &Missions,
) {
    // Blueprint → the missions whose rewards can grant it.
    let mut granted: HashMap<Guid, Vec<Guid>> = HashMap::new();
    for mission in &missions.contracts {
        for reward in mission.rewards.blueprints.iter().filter(|r| r.chance > 0.0) {
            let Some(pool) = missions.blueprints.get(&reward.pool_guid) else {
                continue;
            };
            for entry in &pool.items {
                let ids = granted
                    .entry(entry.blueprint.blueprint_record_guid)
                    .or_default();
                if !ids.contains(&mission.id) {
                    ids.push(mission.id);
                }
            }
        }
    }
    let defaults: HashSet<&Guid> = crafting.default_blueprint_whitelist.iter().collect();
    // Blueprint GUID order, so an item's sources do not depend on map order.
    let mut crafts: Vec<(Guid, Guid, Unlock)> = blueprints
        .iter()
        .filter_map(|(guid, blueprint)| {
            let made = blueprint.crafted_entity_guid()?;
            let unlock = if defaults.contains(guid) {
                Unlock::Default
            } else {
                Unlock::Missions(granted.get(guid)?.clone())
            };
            Some((made, *guid, unlock))
        })
        .collect();
    crafts.sort_by_key(|(_, blueprint, _)| blueprint.to_string());
    debug!(
        crafts = crafts.len(),
        blueprints = blueprints.len(),
        "unlockable blueprints"
    );
    for (made, blueprint, unlock) in crafts {
        by_guid
            .entry(made)
            .or_default()
            .sources
            .push(Source::Craft { blueprint, unlock });
    }
}

/// The ports `item` brings below it: a docked vehicle's hull ports, else
/// its own.
fn ports_of<'a>(item: Guid, vehicles: &'a Vehicles, containers: &'a PortContainers) -> &'a [Port] {
    match vehicles.get(&item) {
        Some(v) => &v.ports,
        None => containers.get(&item).map_or(&[][..], |c| &c.ports[..]),
    }
}

impl RecordCollection for Acquisitions {
    type Item = Acquisition;

    fn get(&self, guid: &Guid) -> Option<&Acquisition> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Acquisition)> + '_ {
        self.by_guid.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mount(editable: bool) -> Mount {
        Mount {
            vehicle: Guid::default(),
            path: "hardpoint".into(),
            editable,
        }
    }

    #[test]
    fn status() {
        let mut a = Acquisition::default();
        assert_eq!(a.status(), Status::Unknown);
        assert!(!a.locked_only());
        // Locked mounts are a hint, not proof.
        a.mounts.push(mount(false));
        assert_eq!(a.status(), Status::Unknown);
        assert!(a.locked_only());
        a.mounts.push(mount(true));
        assert!(!a.locked_only());
        a.sources.push(Source::Stock {
            vehicle: Guid::default(),
        });
        assert_eq!(a.status(), Status::Obtainable);

        let ship = |listing| Acquisition {
            listing: Some(listing),
            ..Default::default()
        };
        assert_eq!(ship(Listing::Variant).status(), Status::Unobtainable);
        // A listed ship always carries its Listed source; without one it is
        // nothing known.
        assert_eq!(ship(Listing::Base).status(), Status::Unknown);
    }
}
