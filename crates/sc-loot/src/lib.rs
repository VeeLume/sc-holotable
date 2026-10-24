//! Loot tables, what their entries pick, and what carries them — a domain
//! crate.
//!
//! - [`LootTables`]: every loot table of both generations the DCB holds
//!   (`LootTable` and `LootTableV3Record`), normalised into one model: a
//!   table holds weighted [`TableArchetype`]s, an [`Archetype`] weighted
//!   [`LootEntry`]s, and each entry a [`Selector`] with the items it picks
//!   resolved. [`LootTables::by_item`] answers "which tables can drop this".
//! - [`Lootables`]: every record a loot table is attached to — entities and
//!   NPC actors with a `LootGenerationComponentParams` (corpses, actors), and
//!   `SubHarvestableConfigRecord`s and `SubHarvestableMultiConfigRecord`s
//!   whose slots carry a loot config (world containers such as weapon racks
//!   and crates) — each with its
//!   [`LootConfig`]s (table, pool filter, chance to generate, secondary
//!   choices).
//!
//! # What can be picked (client 4.10.197)
//!
//! The pool is the client's loot item database
//! (`CLootGenerationDatabase::InitializeItemDatabase`): every entity class
//! carrying the tag `LootGeneration.CanGenerateAsLoot` (or one below it).
//! The client also leaves out a class whose occupancy volume it cannot
//! determine; that check is not modelled. Items are the records
//! [`sc_items::Items`] holds.
//!
//! A tag selector ([`TagTerm`]) picks an item when every positive tag is the
//! item's own tag or an ancestor of one (`EntityClassDefinition.tags`, the
//! [`sc_tags::Tags`] tree) and no negative tag is
//! (`ReverseTagLookupTable::FindItems`: per positive tag the union over its
//! subtree, intersected, the negative tags' items subtracted). The data
//! agrees: 65 selector terms match no item exactly but do through the tree,
//! e.g. "Sustenance" (`Item/Sustenance`) (`tag_match_dig`). A v2 entry's
//! term is its tag plus `additionalTags`, less the archetype's
//! `excludedTags`.
//!
//! A pool filter ([`PoolFilter`]) narrows what a lootable's tables can pick.
//! A sequence starts from the whole pool; a `Cumulative` step keeps the items
//! it admits, an `Additive` step adds the items it admits back (the ASD
//! delving filter drops faction and special-event items, then adds the
//! location's event items). That reading comes from the filters' names and
//! shapes (INFERRED): where the client applies them was not found. Nor are
//! container capacity limits (an item must fit the container) or weights,
//! counts and chances modelled; those are carried as authored.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
//! use sc_items::Items;
//! use sc_loot::{Lootables, LootTables, RecordCollection};
//! use sc_tags::Tags;
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let tags = Tags::build(datacore.records());
//! let tables = LootTables::build(&datacore, &items, &tags);
//! let lootables = Lootables::build(&datacore, &tables);
//! for (guid, lootable) in lootables.iter().take(3) {
//!     for config in &lootable.configs {
//!         let picks = config.tables().filter_map(|t| tables.get(&t)).count();
//!         println!("{guid}: {picks} tables");
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::{HashMap, HashSet};

pub use sc_extract::RecordCollection;
use sc_extract::generated as dcb;
use sc_extract::generated::*;
use sc_extract::{DataPools, Datacore, Guid};
use sc_tags::Tags;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// A `TagsDNFTerm`: every positive tag, no negative one. See the crate docs
/// for how it matches.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagTerm {
    pub positive: Vec<Guid>,
    pub negative: Vec<Guid>,
}

/// What a loot entry picks from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Selector {
    /// Items matching a tag term.
    Tags(TagTerm),
    /// These entity classes (`EntityClassList_Manual`).
    Entities(Vec<Guid>),
    /// A selector kind not modelled.
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// Which loot system a table belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Generation {
    /// `LootTable` / `LootArchetype`.
    V2,
    /// `LootTableV3Record` / `LootArchetypeV3Record`.
    V3,
}

/// One weighted pick of an [`Archetype`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LootEntry {
    pub name: String,
    pub weight: f32,
    pub selector: Selector,
    /// The items the selector picks.
    pub items: Vec<Guid>,
    /// The stack size range (`StackSize`), when authored.
    pub stack: Option<(i32, i32)>,
    /// What spawns along with a pick (v3 `ArchetypeOptionalDataV3_SpawnWith`).
    pub spawn_with: Vec<SpawnWith>,
}

/// A v3 `SpawnWith`: extra items that come with an entry's pick.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnWith {
    pub name: String,
    /// `chanceToSpawnWith`.
    pub chance: f32,
    /// `amountToSpawn`, when authored.
    pub amount: Option<(i32, i32)>,
    pub selector: Selector,
    pub items: Vec<Guid>,
}

/// A secondary group of a pick: a v2 archetype's `secondaryOrGroups` (on
/// LIVE 4.10 all rarity bands, `LootGeneration/LootRarity/Common` …
/// `Legendary`) or one layer of a lootable's v3 secondary choices
/// (manufacturer, rarity and faction weights, `V3ManufacturerWeights_*` /
/// `V3SecondaryChoices_*`; the manufacturer ones end in an "Anything else"
/// entry with no positive tag). It adds no items of its own. In v2 the
/// client keeps only the entry picks that match one of every group's
/// entries, and [`LootEntry::items`] is already narrowed so; how v3 layers
/// act was not read (they are taken as weights only). Test an item against
/// an entry with [`LootTables::selects`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondaryGroup {
    pub name: String,
    pub entries: Vec<SecondaryEntry>,
}

/// One weighted band of a [`SecondaryGroup`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondaryEntry {
    pub name: String,
    pub weight: f32,
    pub selector: Selector,
}

/// A loot archetype: weighted entries (and, in v2, the weightings of their
/// picks).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Archetype {
    /// The archetype record; `None` when authored inline in a v3 table.
    pub record: Option<Guid>,
    pub entries: Vec<LootEntry>,
    pub secondary: Vec<SecondaryGroup>,
}

/// One weighted archetype of a [`LootTable`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableArchetype {
    /// The v3 entry's `name`; empty in v2.
    pub name: String,
    pub weight: f32,
    pub archetype: Archetype,
    /// v2 `numberOfResultsConstraints` (min, max).
    pub results: Option<(i32, i32)>,
    /// v3 `LootTableOptionalDataV3_ChanceToExist`.
    pub chance_to_exist: Option<f32>,
    /// v3 `LootTableOptionalDataV3_ChoiceLimit`.
    pub choice_limit: Option<i32>,
    /// v3 `LootTableOptionalDataV3_DupeLimit`.
    pub dupe_limit: Option<i32>,
    /// v3 `LootTableOptionalDataV3_PriorityChoice.priorityCount`.
    pub priority_count: Option<i32>,
}

/// A loot table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LootTable {
    pub generation: Generation,
    pub archetypes: Vec<TableArchetype>,
}

impl LootTable {
    /// Every item it can drop: the entries' picks and what spawns with
    /// them. May repeat.
    pub fn items(&self) -> impl Iterator<Item = &Guid> + '_ {
        self.archetypes.iter().flat_map(|a| {
            a.archetype.entries.iter().flat_map(|e| {
                e.items
                    .iter()
                    .chain(e.spawn_with.iter().flat_map(|s| &s.items))
            })
        })
    }
}

/// Every loot table of both generations, keyed by its record GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LootTables {
    by_guid: HashMap<Guid, LootTable>,
    /// Item → the tables that can drop it.
    by_item: HashMap<Guid, Vec<Guid>>,
    /// Item → its tags and all their ancestors: what a tag term tests.
    closures: HashMap<Guid, HashSet<Guid>>,
}

impl LootTables {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read every `LootTable` and `LootTableV3Record` with the archetypes
    /// they name, and resolve each selector against `items` (their
    /// `EntityClassDefinition.tags` under the `tags` tree).
    pub fn build(datacore: &Datacore, items: &sc_items::Items, tags: &Tags) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let ecds = &store.records.multi_feature.entity_class_definition;
        // The client's loot item database holds the entity classes tagged
        // `LootGeneration.CanGenerateAsLoot` (or a tag below it).
        let lootable = tags
            .by_name("CanGenerateAsLoot")
            .iter()
            .copied()
            .find(|g| tags.path(g).join("/") == "LootGeneration/CanGenerateAsLoot");
        if lootable.is_none() {
            warn!("no LootGeneration/CanGenerateAsLoot tag: no item can be picked");
        }
        let mut closures: HashMap<Guid, HashSet<Guid>> = HashMap::new();
        for guid in items.guids() {
            let Some(ecd) = ecds.get(guid).and_then(|h| h.get(pools)) else {
                continue;
            };
            let mut closure = HashSet::new();
            for tag in &ecd.tags {
                closure.insert(*tag);
                closure.extend(tags.ancestors(tag).map(|t| t.guid));
            }
            if lootable.is_some_and(|l| closure.contains(&l)) {
                closures.insert(*guid, closure);
            }
        }
        let mut tables = Self {
            closures,
            ..Self::default()
        };
        let reader = Reader {
            datacore,
            store,
            pools,
        };
        let mut by_guid = HashMap::new();
        for (guid, h) in &store.records.multi_feature.loot_table {
            if let Some(t) = h.get(pools) {
                by_guid.insert(*guid, reader.v2_table(t, &tables));
            }
        }
        for (guid, h) in &store.records.multi_feature.loot_table_v3_record {
            if let Some(LootTableV3_NoRefPtr::LootTableV3(t)) =
                h.get(pools).and_then(|r| r.loot_table.as_ref())
                && let Some(t) = t.get(pools)
            {
                by_guid.insert(*guid, reader.v3_table(t, &tables));
            }
        }
        let mut by_item: HashMap<Guid, Vec<Guid>> = HashMap::new();
        let mut sorted: Vec<(&Guid, &LootTable)> = by_guid.iter().collect();
        sorted.sort_by_key(|(g, _)| g.to_string());
        for (guid, table) in sorted {
            for item in table.items() {
                let list = by_item.entry(*item).or_default();
                if !list.contains(guid) {
                    list.push(*guid);
                }
            }
        }
        info!(
            tables = by_guid.len(),
            items = by_item.len(),
            "loot tables built"
        );
        tables.by_guid = by_guid;
        tables.by_item = by_item;
        tables
    }

    /// The tables that can drop `item` (an entry's pick or a spawn-with;
    /// pool filters not applied).
    pub fn by_item(&self, item: &Guid) -> &[Guid] {
        self.by_item.get(item).map_or(&[], Vec::as_slice)
    }

    /// Whether `term` picks `item`.
    pub fn selects(&self, term: &TagTerm, item: &Guid) -> bool {
        let Some(closure) = self.closures.get(item) else {
            return term.positive.is_empty();
        };
        term.positive.iter().all(|t| closure.contains(t))
            && !term.negative.iter().any(|t| closure.contains(t))
    }

    /// Whether `filter` lets `item` into the pool (see the crate docs).
    /// Filter kinds not modelled let everything through.
    pub fn admits(&self, filter: &PoolFilter, item: &Guid) -> bool {
        match filter {
            PoolFilter::Tags(term) => self.selects(term, item),
            PoolFilter::Sequence(steps) => steps.iter().fold(true, |admitted, step| {
                let pass = self.admits(&step.filter, item);
                match step.mode {
                    Combine::Cumulative => admitted && pass,
                    Combine::Additive => admitted || pass,
                }
            }),
            PoolFilter::Other { .. } => true,
        }
    }

    fn resolve(&self, selector: &Selector) -> Vec<Guid> {
        match selector {
            Selector::Tags(term) => {
                let mut items: Vec<Guid> = self
                    .closures
                    .keys()
                    .filter(|g| self.selects(term, g))
                    .copied()
                    .collect();
                items.sort_by_key(|g| g.to_string());
                items
            }
            Selector::Entities(list) => list.clone(),
            Selector::Other { .. } => Vec::new(),
        }
    }
}

impl RecordCollection for LootTables {
    type Item = LootTable;

    fn get(&self, guid: &Guid) -> Option<&LootTable> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &LootTable)> + '_ {
        self.by_guid.iter()
    }
}

/// How a step of a [`PoolFilter::Sequence`] combines with the pool so far.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Combine {
    /// Keep only the items the step admits.
    Cumulative,
    /// Add the items the step admits.
    Additive,
}

/// One step of a [`PoolFilter::Sequence`] (`PoolFilterInstance`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterStep {
    pub name: String,
    pub mode: Combine,
    pub filter: PoolFilter,
}

/// A `PoolFilterRecord`'s filter, with record references followed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolFilter {
    /// Items matching a tag term.
    Tags(TagTerm),
    /// Steps applied in order to the whole pool.
    Sequence(Vec<FilterStep>),
    /// A filter kind not modelled (it admits everything).
    Other {
        type_name: String,
        struct_index: u32,
    },
}

/// One loot config of a [`Lootable`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LootConfig {
    /// The v2 table (`lootTable`).
    pub table: Option<Guid>,
    /// The v3 table (`lootTableV3`). On LIVE 4.10 one config of 3,402 sets
    /// both; which one the client uses then is not read.
    pub table_v3: Option<Guid>,
    /// `lootConstraints.poolFilter`, resolved.
    pub pool_filter: Option<PoolFilter>,
    /// The pool filter's record.
    pub pool_filter_record: Option<Guid>,
    /// `lootConstraints.chanceToGenerate`.
    pub chance_to_generate: f32,
    /// `lootConstraints.totalResultsLimit`.
    pub total_results_limit: i32,
    /// `lootConstraints.secondaryChoices`: weightings of the picks, one per
    /// layer (see [`SecondaryGroup`]).
    pub secondary: Vec<SecondaryGroup>,
    /// For a harvestable slot, the container entity it spawns
    /// (`harvestableEntityClass`).
    pub container: Option<Guid>,
}

impl LootConfig {
    /// The tables it names (v2 and v3).
    pub fn tables(&self) -> impl Iterator<Item = Guid> + '_ {
        self.table.into_iter().chain(self.table_v3)
    }
}

/// What a [`Lootable`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LootableKind {
    /// An `EntityClassDefinition` with a `LootGenerationComponentParams`
    /// (NPC actors, corpses, lootable carryables).
    Entity,
    /// A `SubHarvestableConfigRecord`, or a `SubHarvestableMultiConfigRecord`
    /// with its inline configs: one config per slot that has one.
    Harvestable,
}

/// A record loot tables are attached to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lootable {
    pub kind: LootableKind,
    /// Its loot configs that name a table.
    pub configs: Vec<LootConfig>,
}

/// Every record loot tables are attached to, keyed by its GUID (entity,
/// `SubHarvestableConfigRecord` or `SubHarvestableMultiConfigRecord`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lootables {
    by_guid: HashMap<Guid, Lootable>,
    by_table: HashMap<Guid, Vec<Guid>>,
}

impl Lootables {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read every `LootGenerationComponentParams` on an entity and every
    /// sub-harvestable slot's loot config; `tables` is what the configs
    /// name.
    pub fn build(datacore: &Datacore, tables: &LootTables) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let reader = Reader {
            datacore,
            store,
            pools,
        };
        let mut by_guid: HashMap<Guid, Lootable> = HashMap::new();
        for (guid, h) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = h.get(pools) else { continue };
            let configs: Vec<LootConfig> = ecd
                .components
                .iter()
                .filter_map(|c| match c {
                    DataForgeComponentParamsPtr::LootGenerationComponentParams(h) => h.get(pools),
                    _ => None,
                })
                .filter_map(|p| p.loot_config?.get(pools))
                .map(|c| reader.config(c, None))
                .filter(|c| c.tables().next().is_some())
                .collect();
            if !configs.is_empty() {
                by_guid.insert(
                    *guid,
                    Lootable {
                        kind: LootableKind::Entity,
                        configs,
                    },
                );
            }
        }
        let mut harvestable = |guid: &Guid, configs: Vec<LootConfig>| {
            if !configs.is_empty() {
                by_guid.insert(
                    *guid,
                    Lootable {
                        kind: LootableKind::Harvestable,
                        configs,
                    },
                );
            }
        };
        for (guid, h) in &store.records.multi_feature.sub_harvestable_config_record {
            let Some(config) = h.get(pools).and_then(|r| r.sub_config?.get(pools)) else {
                continue;
            };
            harvestable(guid, reader.slots(config));
        }
        // A multi config's default and tagged configs: inline ones here, the
        // ones that name a SubHarvestableConfigRecord are that record's.
        for (guid, h) in &store
            .records
            .harvestable
            .sub_harvestable_multi_config_record
        {
            let Some(multi) = h.get(pools).and_then(|r| r.multi_config?.get(pools)) else {
                continue;
            };
            let tagged = multi
                .tagged_configs
                .iter()
                .filter_map(|t| t.get(pools)?.sub_config.as_ref());
            let configs = multi
                .default_config
                .iter()
                .chain(tagged)
                .filter_map(|c| match c {
                    SubHarvestableConfigSingleBasePtr::SubHarvestableConfigSingleManual(h) => {
                        h.get(pools)?.sub_config_manual?.get(pools)
                    }
                    _ => None,
                })
                .flat_map(|c| reader.slots(c))
                .collect();
            harvestable(guid, configs);
        }
        let mut by_table: HashMap<Guid, Vec<Guid>> = HashMap::new();
        let mut sorted: Vec<(&Guid, &Lootable)> = by_guid.iter().collect();
        sorted.sort_by_key(|(g, _)| g.to_string());
        for (guid, lootable) in sorted {
            for table in lootable.configs.iter().flat_map(LootConfig::tables) {
                if tables.get(&table).is_none() {
                    warn!(%guid, %table, "loot config names a table that does not exist");
                }
                let list = by_table.entry(table).or_default();
                if !list.contains(guid) {
                    list.push(*guid);
                }
            }
        }
        info!(lootables = by_guid.len(), "lootables built");
        Self { by_guid, by_table }
    }

    /// The lootables whose configs name `table`.
    pub fn by_table(&self, table: &Guid) -> &[Guid] {
        self.by_table.get(table).map_or(&[], Vec::as_slice)
    }
}

impl RecordCollection for Lootables {
    type Item = Lootable;

    fn get(&self, guid: &Guid) -> Option<&Lootable> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Lootable)> + '_ {
        self.by_guid.iter()
    }
}

/// Reads the loot structs of one DCB.
struct Reader<'a> {
    datacore: &'a Datacore,
    store: &'a sc_extract::RecordStore,
    pools: &'a DataPools,
}

/// Record references a pool filter may follow before giving up (a filter
/// naming itself).
const MAX_FILTER_DEPTH: usize = 8;

impl Reader<'_> {
    fn term(&self, h: Option<Handle<TagsDNFTerm>>) -> TagTerm {
        h.and_then(|h| h.get(self.pools))
            .map(|t| TagTerm {
                positive: t.positive_tags.clone(),
                negative: t.negative_tags.clone(),
            })
            .unwrap_or_default()
    }

    fn entity_list(&self, list: Option<&EntityClassList_BasePtr>) -> Selector {
        match list {
            Some(EntityClassList_BasePtr::EntityClassList_Manual(h)) => Selector::Entities(
                h.get(self.pools)
                    .map(|m| m.entity_classes.clone())
                    .unwrap_or_default(),
            ),
            Some(EntityClassList_BasePtr::Unknown { struct_index, .. }) => Selector::Other {
                type_name: format!("struct#{struct_index}"),
                struct_index: *struct_index,
            },
            other => Selector::Other {
                type_name: format!("{:?}", other.map(std::mem::discriminant)),
                struct_index: 0,
            },
        }
    }

    fn range(&self, q: Option<&QuantityRange_BasePtr>) -> Option<(i32, i32)> {
        match q? {
            QuantityRange_BasePtr::QuantityRange_Linear(h) => {
                h.get(self.pools).map(|r| (r.min, r.max))
            }
            _ => None,
        }
    }

    fn v2_table(&self, t: &dcb::LootTable, tables: &LootTables) -> LootTable {
        let archetypes = t
            .loot_archetypes
            .iter()
            .filter_map(|h| h.get(self.pools))
            .map(|w| TableArchetype {
                name: String::new(),
                weight: w.weight,
                archetype: w
                    .archetype
                    .map(|g| self.v2_archetype(g, tables))
                    .unwrap_or_default(),
                results: w
                    .number_of_results_constraints
                    .and_then(|h| h.get(self.pools))
                    .map(|n| (n.min_results, n.max_results)),
                chance_to_exist: None,
                choice_limit: None,
                dupe_limit: None,
                priority_count: None,
            })
            .collect();
        LootTable {
            generation: Generation::V2,
            archetypes,
        }
    }

    fn v2_archetype(&self, guid: Guid, tables: &LootTables) -> Archetype {
        let Some(a) = self
            .store
            .records
            .multi_feature
            .loot_archetype
            .get(&guid)
            .and_then(|h| h.get(self.pools))
        else {
            debug!(%guid, "v2 loot archetype not found");
            return Archetype {
                record: Some(guid),
                ..Archetype::default()
            };
        };
        let excluded: Vec<Guid> = a
            .excluded_tags
            .and_then(|t| t.get(self.pools))
            .map(|t| t.tags.clone())
            .unwrap_or_default();
        let entries = a
            .primary_or_group
            .and_then(|g| g.get(self.pools))
            .iter()
            .flat_map(|g| &g.entries)
            .filter_map(|h| h.get(self.pools))
            .map(|e| {
                let extra = self.term(e.additional_tags);
                let term = TagTerm {
                    positive: e.tag.into_iter().chain(extra.positive).collect(),
                    negative: excluded.iter().copied().chain(extra.negative).collect(),
                };
                let selector = Selector::Tags(term);
                let stack = e.optional_data.iter().find_map(|d| match d {
                    EntryOptionalData_BasePtr::EntryOptionalData_StackSize(h) => {
                        h.get(self.pools).map(|s| (s.min, s.max))
                    }
                    _ => None,
                });
                LootEntry {
                    name: e.name.clone(),
                    weight: e.weight,
                    items: tables.resolve(&selector),
                    selector,
                    stack,
                    spawn_with: Vec::new(),
                }
            })
            .collect();
        let secondary: Vec<SecondaryGroup> = a
            .secondary_or_groups
            .iter()
            .filter_map(|h| h.get(self.pools))
            .map(|g| SecondaryGroup {
                name: g.group_name.clone(),
                entries: g
                    .entries
                    .iter()
                    .filter_map(|h| h.get(self.pools))
                    .map(|e| {
                        let selector = Selector::Tags(TagTerm {
                            positive: e.tag.into_iter().collect(),
                            negative: Vec::new(),
                        });
                        SecondaryEntry {
                            name: String::new(),
                            weight: e.weight,
                            selector,
                        }
                    })
                    .collect(),
            })
            .collect();
        // The client intersects each entry's picks with every secondary
        // group: an item stays only if it matches one of the group's entries
        // (`GetLootSubsetHelperForArchetype` -> `LootSubsetHelper::
        // IntersectWith`).
        let mut entries: Vec<LootEntry> = entries;
        for group in &secondary {
            let allowed: HashSet<Guid> = group
                .entries
                .iter()
                .flat_map(|e| tables.resolve(&e.selector))
                .collect();
            for e in &mut entries {
                e.items.retain(|i| allowed.contains(i));
            }
        }
        Archetype {
            record: Some(guid),
            entries,
            secondary,
        }
    }

    fn v3_table(&self, t: &LootTableV3, tables: &LootTables) -> LootTable {
        let archetypes = t
            .loot_archetypes
            .iter()
            .filter_map(|h| h.get(self.pools))
            .map(|e| {
                let mut a = TableArchetype {
                    name: e.name.clone(),
                    weight: e.weight,
                    archetype: self.v3_archetype(e.archetype.as_ref(), tables),
                    results: None,
                    chance_to_exist: None,
                    choice_limit: None,
                    dupe_limit: None,
                    priority_count: None,
                };
                for d in &e.optional_data {
                    use LootTableOptionalDataV3_BasePtr as D;
                    match d {
                        D::LootTableOptionalDataV3_ChanceToExist(h) => {
                            a.chance_to_exist = h.get(self.pools).map(|d| d.chance_to_exist);
                        }
                        D::LootTableOptionalDataV3_ChoiceLimit(h) => {
                            a.choice_limit = h.get(self.pools).map(|d| d.choice_limit);
                        }
                        D::LootTableOptionalDataV3_DupeLimit(h) => {
                            a.dupe_limit = h.get(self.pools).map(|d| d.dupe_limit);
                        }
                        D::LootTableOptionalDataV3_PriorityChoice(h) => {
                            a.priority_count = h.get(self.pools).map(|d| d.priority_count);
                        }
                        _ => {}
                    }
                }
                a
            })
            .collect();
        LootTable {
            generation: Generation::V3,
            archetypes,
        }
    }

    fn v3_archetype(&self, a: Option<&LootArchetypeV3_BasePtr>, tables: &LootTables) -> Archetype {
        let (record, inline) = match a {
            Some(LootArchetypeV3_BasePtr::LootArchetypeV3(h)) => (None, Some(*h)),
            Some(LootArchetypeV3_BasePtr::LootArchetypeV3_RecordRef(h)) => {
                let guid = h.get(self.pools).and_then(|r| r.loot_archetype_record);
                let inline = guid
                    .and_then(|g| {
                        self.store
                            .records
                            .multi_feature
                            .loot_archetype_v3_record
                            .get(&g)
                    })
                    .and_then(|h| h.get(self.pools))
                    .and_then(|r| match r.loot_archetype.as_ref()? {
                        LootArchetypeV3_NoRefPtr::LootArchetypeV3(h) => Some(*h),
                        _ => None,
                    });
                (guid, inline)
            }
            _ => (None, None),
        };
        let entries = inline
            .and_then(|h| h.get(self.pools))
            .iter()
            .flat_map(|a| &a.entries)
            .filter_map(|h| h.get(self.pools))
            .map(|e| {
                let selector = self.v3_selector(e.selector.as_ref());
                let mut entry = LootEntry {
                    name: e.name.clone(),
                    weight: e.weight,
                    items: tables.resolve(&selector),
                    selector,
                    stack: None,
                    spawn_with: Vec::new(),
                };
                for d in &e.optional_data {
                    match d {
                        ArchetypeOptionalDataV3_BasePtr::ArchetypeOptionalDataV3_StackSize(h) => {
                            entry.stack = h
                                .get(self.pools)
                                .and_then(|s| self.range(s.stack_size.as_ref()));
                        }
                        ArchetypeOptionalDataV3_BasePtr::ArchetypeOptionalDataV3_SpawnWith(h) => {
                            let Some(s) = h.get(self.pools) else { continue };
                            let selector = match s.selector.as_ref() {
                                Some(SpawnWithV3Selector_BasePtr::SpawnWithV3Selector_Tags(t)) => {
                                    Selector::Tags(
                                        self.term(t.get(self.pools).and_then(|t| t.tags)),
                                    )
                                }
                                Some(SpawnWithV3Selector_BasePtr::Unknown {
                                    struct_index, ..
                                }) => Selector::Other {
                                    type_name: format!("struct#{struct_index}"),
                                    struct_index: *struct_index,
                                },
                                _ => Selector::Other {
                                    type_name: "SpawnWithV3Selector".into(),
                                    struct_index: 0,
                                },
                            };
                            entry.spawn_with.push(SpawnWith {
                                name: s.name.clone(),
                                chance: s.chance_to_spawn_with,
                                amount: self.range(s.amount_to_spawn.as_ref()),
                                items: tables.resolve(&selector),
                                selector,
                            });
                        }
                        _ => {}
                    }
                }
                entry
            })
            .collect();
        Archetype {
            record,
            entries,
            secondary: Vec::new(),
        }
    }

    fn v3_selector(&self, s: Option<&LootArchetypeV3Selector_BasePtr>) -> Selector {
        match s {
            Some(LootArchetypeV3Selector_BasePtr::LootArchetypeV3Selector_Tags(h)) => {
                Selector::Tags(self.term(h.get(self.pools).and_then(|t| t.tags)))
            }
            Some(LootArchetypeV3Selector_BasePtr::LootArchetypeV3Selector_EntityClasses(h)) => {
                self.entity_list(h.get(self.pools).and_then(|e| e.entity_classes.as_ref()))
            }
            Some(LootArchetypeV3Selector_BasePtr::Unknown { struct_index, .. }) => {
                Selector::Other {
                    type_name: format!("struct#{struct_index}"),
                    struct_index: *struct_index,
                }
            }
            _ => Selector::Other {
                type_name: "LootArchetypeV3Selector".into(),
                struct_index: 0,
            },
        }
    }

    fn config(&self, c: &dcb::LootConfig, container: Option<Guid>) -> LootConfig {
        let constraints = c.loot_constraints.and_then(|h| h.get(self.pools));
        let pool_filter_record = constraints.and_then(|k| k.pool_filter);
        LootConfig {
            table: c.loot_table,
            table_v3: c.loot_table_v3,
            pool_filter: pool_filter_record.and_then(|g| self.filter_record(g, 0)),
            pool_filter_record,
            chance_to_generate: constraints.map_or(1.0, |k| k.chance_to_generate),
            total_results_limit: constraints.map_or(0, |k| k.total_results_limit),
            secondary: constraints
                .and_then(|k| k.secondary_choices.as_ref())
                .map(|s| self.secondary_choices(s))
                .unwrap_or_default(),
            container,
        }
    }

    /// The loot configs of a sub-harvestable config's slots that name a
    /// table.
    fn slots(&self, config: &SubHarvestableConfig) -> Vec<LootConfig> {
        config
            .sub_harvestables
            .iter()
            .filter_map(|s| s.get(self.pools))
            .filter_map(|slot| {
                let c = slot.loot_config?.get(self.pools)?;
                Some(self.config(c, slot.harvestable_entity_class))
            })
            .filter(|c| c.tables().next().is_some())
            .collect()
    }

    fn filter_record(&self, guid: Guid, depth: usize) -> Option<PoolFilter> {
        if depth >= MAX_FILTER_DEPTH {
            warn!(%guid, "pool filter references nest too deep");
            return None;
        }
        let record = self
            .store
            .records
            .multi_feature
            .pool_filter_record
            .get(&guid)?
            .get(self.pools)?;
        match record.filter.as_ref()? {
            PoolFilter_NoRefPtr::PoolFilter_Tags(h) => Some(PoolFilter::Tags(
                self.term(h.get(self.pools).and_then(|t| t.tag_filter)),
            )),
            PoolFilter_NoRefPtr::PoolFilter_Sequence(h) => Some(PoolFilter::Sequence(
                h.get(self.pools)?
                    .filters
                    .iter()
                    .filter_map(|i| i.get(self.pools))
                    .filter_map(|i| {
                        let mode = match i.mode {
                            EPoolFilterCombineMode::Cumulative => Combine::Cumulative,
                            EPoolFilterCombineMode::Additive => Combine::Additive,
                            EPoolFilterCombineMode::Unrecognized(ref s) => {
                                warn!(mode = s, "unknown pool filter combine mode");
                                return None;
                            }
                        };
                        let filter = match i.filter.as_ref()? {
                            PoolFilter_BasePtr::PoolFilter_Tags(h) => PoolFilter::Tags(
                                self.term(h.get(self.pools).and_then(|t| t.tag_filter)),
                            ),
                            PoolFilter_BasePtr::PoolFilter_RecordRef(h) => {
                                self.filter_record(h.get(self.pools)?.filter_record?, depth + 1)?
                            }
                            PoolFilter_BasePtr::Unknown { struct_index, .. } => PoolFilter::Other {
                                type_name: format!("struct#{struct_index}"),
                                struct_index: *struct_index,
                            },
                            _ => PoolFilter::Other {
                                type_name: "PoolFilter".into(),
                                struct_index: 0,
                            },
                        };
                        Some(FilterStep {
                            name: i.name.trim().to_string(),
                            mode,
                            filter,
                        })
                    })
                    .collect(),
            )),
            PoolFilter_NoRefPtr::Unknown { struct_index, .. } => Some(PoolFilter::Other {
                type_name: format!("struct#{struct_index}"),
                struct_index: *struct_index,
            }),
            _ => Some(PoolFilter::Other {
                type_name: "PoolFilter".into(),
                struct_index: 0,
            }),
        }
    }

    fn secondary_choices(
        &self,
        s: &LootV3SecondaryChoicesRecordRef_BasePtr,
    ) -> Vec<SecondaryGroup> {
        let records = &self.store.records.multi_feature;
        let layers: Vec<Guid> = match s {
            LootV3SecondaryChoicesRecordRef_BasePtr::LootV3SecondaryChoicesRecordRef_SingleLayer(h) => {
                h.get(self.pools)
                    .and_then(|r| r.single_layer_record)
                    .into_iter()
                    .collect()
            }
            LootV3SecondaryChoicesRecordRef_BasePtr::LootV3SecondaryChoicesRecordRef_MultiLayer(h) => {
                h.get(self.pools)
                    .and_then(|r| r.multi_layer_record)
                    .and_then(|g| records.loot_v3_secondary_choices_multi_layer_record.get(&g))
                    .and_then(|h| h.get(self.pools))
                    .map(|m| m.layers.clone())
                    .unwrap_or_default()
            }
            _ => Vec::new(),
        };
        layers
            .iter()
            .filter_map(|g| {
                let layer = records
                    .loot_v3_secondary_choices_single_layer_record
                    .get(g)?
                    .get(self.pools)?;
                let name = self
                    .datacore
                    .db()
                    .record(g)
                    .and_then(|r| r.name())
                    .map(|n| n.split_once('.').map_or(n, |(_, n)| n).to_string())
                    .unwrap_or_default();
                Some(SecondaryGroup {
                    name,
                    entries: layer
                        .choices
                        .iter()
                        .filter_map(|h| h.get(self.pools))
                        .map(|c| {
                            let selector = match c.selector.as_ref() {
                                Some(
                                    LootV3SecondaryChoiceEntrySelector_BasePtr::LootV3SecondaryChoiceEntrySelector_Tags(h),
                                ) => Selector::Tags(self.term(h.get(self.pools).and_then(|t| t.tags))),
                                _ => Selector::Other {
                                    type_name: "LootV3SecondaryChoiceEntrySelector".into(),
                                    struct_index: 0,
                                },
                            };
                            SecondaryEntry {
                                name: c.name.clone(),
                                weight: c.weight,
                                selector,
                            }
                        })
                        .collect(),
                })
            })
            .collect()
    }
}
