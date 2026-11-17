//! Where each item drops: every lootable config that can yield it, the
//! container it fills, the places that container sits and the chance a fill
//! yields the item.

use std::collections::HashMap;

use sc_extract::{Guid, RecordCollection};
use sc_items::Items;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{LootPlacements, LootTables, Lootables, Placed};

/// One container fill: one config of one lootable, filling one kind of
/// container. A config placed in containers of different sizes is one
/// source per container, since the chances depend on the room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropSource {
    /// The lootable ([`Lootables`] key).
    pub lootable: Guid,
    /// The config's index in [`Lootable::configs`](crate::Lootable::configs).
    pub config: usize,
    /// The entity whose inventory the fill goes into: the config's own
    /// container, else the entity placing the slot config, else the
    /// lootable entity itself. `None` when none of them is known.
    pub container: Option<Guid>,
    /// That container's capacity in SCU; `None` (unknown) fills unbounded.
    pub capacity: Option<f64>,
    /// The `StarMapObject`s it sits at, each with how many such containers
    /// are placed there, most first. Empty when no placement reaches a
    /// place (NPC loot, mission and event content, underground facilities;
    /// see [`LootPlacements`]).
    pub places: Vec<(Guid, usize)>,
}

/// One way an item drops: a [`DropSource`] ([`Drops::source`]) and the
/// item's odds in one of its fills.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Drop {
    /// The source's index; read it with [`Drops::source`].
    pub source: usize,
    /// The chance one fill yields at least one of the item.
    pub chance: f64,
    /// How many of the item one fill yields on average.
    pub expected: f64,
}

/// Every item's [`Drop`]s, keyed by the item's GUID, most likely first.
///
/// Built by simulating every lootable config once per container it fills
/// ([`LootTables::simulate`], a fixed seed, so a build is deterministic):
/// the cost is the number of configs times `runs`, spread over the CPU's
/// cores (LIVE 4.10: about 16 s at 2,000 fills, `drops_probe`). An item no
/// config yields in `runs` fills has no entry; a chance below about
/// `1 / runs` may be missed.
///
/// ```no_run
/// use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
/// use sc_items::Items;
/// use sc_loot::{Drops, LootPlacements, LootTables, Lootables, RecordCollection};
/// use sc_tags::Tags;
///
/// let install = sc_discovery::discover_primary()?;
/// let assets = AssetSource::from_install(&install)?;
/// let data = AssetData::extract(&assets, &AssetConfig::standard())?;
/// let datacore = Datacore::parse(&assets, &data)?;
/// let items = Items::build(datacore.records());
/// let tags = Tags::build(datacore.records());
/// let tables = LootTables::build(&datacore, &items, &tags);
/// let lootables = Lootables::build(&datacore, &tables);
/// let placements = LootPlacements::cook(&assets, &lootables)?;
/// let drops = Drops::build(&tables, &lootables, &placements, &items, 2_000);
/// for (item, list) in drops.iter().take(3) {
///     // The most likely drop in a container placed on the star map.
///     let best = list.iter().find(|d| !drops.source(d).places.is_empty());
///     if let Some(d) = best {
///         let places = drops.source(d).places.len();
///         println!("{item}: {:.1} % at {places} places", d.chance * 100.0);
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Drops {
    sources: Vec<DropSource>,
    by_item: HashMap<Guid, Vec<Drop>>,
}

impl Drops {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Simulate `runs` fills of every config of every lootable, per
    /// container it fills, and index the results by item.
    pub fn build(
        tables: &LootTables,
        lootables: &Lootables,
        placements: &LootPlacements,
        items: &Items,
        runs: u32,
    ) -> Self {
        let capacity = |g: &Guid| {
            items
                .get(g)
                .and_then(|i| i.inventory.as_ref())
                .and_then(|inv| inv.scu())
        };
        let mut lootable_list: Vec<(&Guid, &crate::Lootable)> = lootables.iter().collect();
        lootable_list.sort_by_key(|(g, _)| g.to_string());
        let mut sources: Vec<DropSource> = Vec::new();
        for (guid, lootable) in lootable_list {
            let placed = placements.get(guid).map_or(&[][..], Vec::as_slice);
            for (index, config) in lootable.configs.iter().enumerate() {
                // Group the placements by the container they fill.
                let mut groups: HashMap<Option<Guid>, HashMap<Guid, usize>> = HashMap::new();
                for p in placed {
                    let host = match p.placed {
                        Placed::Entity => Some(*guid),
                        Placed::Container { entity } => entity,
                    };
                    let places = groups.entry(config.container.or(host)).or_default();
                    for (place, n) in &p.places {
                        *places.entry(*place).or_default() += n * p.count;
                    }
                }
                if groups.is_empty() {
                    // Placed nowhere: the lootable fills itself when it is
                    // an entity with an inventory.
                    let own = capacity(guid).map(|_| *guid);
                    groups.insert(config.container.or(own), HashMap::new());
                }
                let mut groups: Vec<_> = groups.into_iter().collect();
                groups.sort_by_key(|(c, _)| c.map(|g| g.to_string()));
                for (container, places) in groups {
                    let mut places: Vec<(Guid, usize)> = places.into_iter().collect();
                    places
                        .sort_by(|a, b| b.1.cmp(&a.1).then(a.0.to_string().cmp(&b.0.to_string())));
                    sources.push(DropSource {
                        lootable: *guid,
                        config: index,
                        capacity: container.as_ref().and_then(capacity),
                        container,
                        places,
                    });
                }
            }
        }

        let volume = |g: &Guid| items.get(g).and_then(|i| i.occupancy);
        let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
        let chunk = sources.len().div_ceil(threads).max(1);
        let results: Vec<Vec<crate::DropChance>> = std::thread::scope(|scope| {
            let handles: Vec<_> = sources
                .chunks(chunk)
                .map(|part| {
                    scope.spawn(move || {
                        part.iter()
                            .map(|s| {
                                let config = &lootables
                                    .get(&s.lootable)
                                    .expect("source from lootables")
                                    .configs[s.config];
                                tables.simulate(config, s.capacity, volume, runs, 42).drops
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().expect("drop simulation thread"))
                .collect()
        });

        let mut by_item: HashMap<Guid, Vec<Drop>> = HashMap::new();
        for (source, drops) in results.into_iter().enumerate() {
            for d in drops {
                by_item.entry(d.item).or_default().push(Drop {
                    source,
                    chance: d.chance,
                    expected: d.expected,
                });
            }
        }
        for list in by_item.values_mut() {
            list.sort_by(|a, b| b.chance.total_cmp(&a.chance).then(a.source.cmp(&b.source)));
        }
        info!(
            items = by_item.len(),
            sources = sources.len(),
            "loot drops built"
        );
        Self { sources, by_item }
    }

    /// The container fill a [`Drop`] comes from.
    pub fn source(&self, drop: &Drop) -> &DropSource {
        &self.sources[drop.source]
    }
}

impl RecordCollection for Drops {
    type Item = Vec<Drop>;

    fn get(&self, guid: &Guid) -> Option<&Vec<Drop>> {
        self.by_item.get(guid)
    }

    fn len(&self) -> usize {
        self.by_item.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Vec<Drop>)> + '_ {
        self.by_item.iter()
    }
}
