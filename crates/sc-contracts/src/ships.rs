//! Ship-entity registry — data-derived from the contract graph.
//!
//! Star Citizen's contract system selects which NPC ships to spawn by
//! referencing **tag queries** (`positive_tags ∩ !negative_tags`) inside
//! `MissionPropertyValue_ShipSpawnDescriptions` values. Resolving those
//! queries at display time requires a pool of candidate ship entities
//! with their tag sets and display names indexed.
//!
//! Construction is two-pass and purely data-derived — no
//! `EntityClassDefinition` record-name filtering, per workspace design
//! principle §5 (no string matching where data-derived alternatives
//! exist):
//!
//! 1. Walk every `MissionPropertyValue_ShipSpawnDescriptions` materialized
//!    in the pools. Collect every `Tag` GUID referenced in positive or
//!    negative spawn-query slots.
//! 2. Walk every `EntityClassDefinition`. Keep any whose `tags` set
//!    intersects the spawn-referenced tags from step 1. Resolve its
//!    display name via the `DisplayNameCache` the snapshot already holds,
//!    and its size via its `SAttachableComponentParams.AttachDef.Size`.
//!
//! The resulting pool is exactly the set of entities contracts can
//! possibly spawn, and nothing they cannot. sc-langpatch's existing
//! `build_ship_tag_index` under-matches because it filters on
//! `_PU_AI` / `_pu_ai` record-name substrings and drops entities outside
//! that filter even when contracts reference them.

use std::collections::{HashMap, HashSet};

use sc_extract::generated::{
    DataForgeComponentParamsPtr, DataPools, EntityClassDefinition, TagList,
};
use sc_extract::{Datacore, Guid};

/// An AI ship entity indexed for tag-query spawn resolution.
#[derive(Debug, Clone)]
pub struct ShipEntity {
    /// The `EntityClassDefinition` record's GUID.
    pub entity_guid: Guid,
    /// Localized display name as resolved by [`DisplayNameCache`]. May be
    /// empty for entities whose localization isn't in `global.ini` yet.
    pub display_name: String,
    /// The full tag set on the entity.
    pub tags: HashSet<Guid>,
    /// Size class from `SAttachableComponentParams.AttachDef.Size` —
    /// 1 = small, 2 = medium, 3 = large, up through capital ships.
    /// `0` if the entity has no attach definition.
    pub size: i32,
}

/// A resolved spawn-query candidate. Consumers display these; the full
/// [`ShipEntity`] stays inside the registry for match logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShipCandidate {
    pub display_name: String,
    pub size: i32,
    /// Back-reference to the underlying ship entity in case the consumer
    /// needs the full tag set or raw GUID.
    pub entity_guid: Guid,
}

/// Registry of AI ship entities, keyed for tag-query resolution.
///
/// Built once per `Datacore` via [`ShipRegistry::build`]. Holds roughly
/// one entry per possible NPC ship + variant in the live DCB.
/// Name of the top-level `Tag` record under which every ship-selector
/// tag lives (e.g. `Cutter`, `Pisces`, `Gladius`, `Idris_M`). Derived
/// from the tag tree at registry build time — not a string match on
/// entity names, in line with workspace design principle §5.
const SHIP_TAG_ROOT_NAME: &str = "Ship";

#[derive(Debug, Clone, Default)]
pub struct ShipRegistry {
    entities: Vec<ShipEntity>,
    /// Union of every tag present on at least one entity in the registry.
    ship_relevant_tags: HashSet<Guid>,
    /// Every tag the contract graph ever queries (positive or negative)
    /// inside a ship-spawn description.
    spawn_referenced_tags: HashSet<Guid>,
    /// For each tag, how many pool entities carry it.
    tag_carrier_count: HashMap<Guid, usize>,
    /// Every tag that is a (transitive) descendant of the tag tree's
    /// `Ship` root. These are the ship-selective tags — model identifiers
    /// and variants. A spawn query that doesn't include at least one
    /// surviving (non-zero-carrier) ship-selective tag is ambiguous and
    /// resolves to empty.
    ship_tags: HashSet<Guid>,
}

impl ShipRegistry {
    /// Build the registry from the current [`Datacore`].
    ///
    /// Walks every materialized `MissionPropertyValue_ShipSpawnDescriptions`
    /// to collect spawn-referenced tags, then every `EntityClassDefinition`
    /// to include entities whose tag set intersects those tags.
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let display_names = &datacore.snapshot().display_names;

        // Pass 1 — collect every tag referenced in ship-spawn queries.
        let mut spawn_referenced_tags: HashSet<Guid> = HashSet::new();
        for desc in pools
            .multi_feature
            .mission_property_value_ship_spawn_descriptions
            .iter()
            .flatten()
        {
            for group_h in &desc.spawn_descriptions {
                let Some(group) = group_h.get(pools) else { continue };
                for options_h in &group.ships {
                    let Some(options) = options_h.get(pools) else { continue };
                    for option_h in &options.options {
                        let Some(option) = option_h.get(pools) else { continue };
                        collect_tags(pools, option.tags.as_ref(), &mut spawn_referenced_tags);
                        collect_tags(
                            pools,
                            option.negative_tags.as_ref(),
                            &mut spawn_referenced_tags,
                        );
                    }
                }
            }
        }

        // Pass 2 — include every EntityClassDefinition whose tags intersect
        // the spawn-referenced set.
        let mut entities = Vec::new();
        let mut ship_relevant_tags: HashSet<Guid> = HashSet::new();

        // The RecordIndex stores guid → handle for root records. Walk it so
        // we have both the GUID and the typed record in hand.
        for (guid, handle) in &datacore.records().records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else { continue };

            let entity_tags: HashSet<Guid> = ecd.tags.iter().copied().collect();
            if entity_tags.is_disjoint(&spawn_referenced_tags) {
                continue;
            }

            let display_name = display_names.get(guid).unwrap_or("").to_string();
            let size = extract_size(ecd, pools);

            ship_relevant_tags.extend(entity_tags.iter().copied());
            entities.push(ShipEntity {
                entity_guid: *guid,
                display_name,
                tags: entity_tags,
                size,
            });
        }

        // Stable ordering: by size, then by display name, then by guid.
        entities.sort_by(|a, b| {
            a.size
                .cmp(&b.size)
                .then_with(|| a.display_name.cmp(&b.display_name))
                .then_with(|| a.entity_guid.to_string().cmp(&b.entity_guid.to_string()))
        });

        let mut tag_carrier_count: HashMap<Guid, usize> = HashMap::new();
        for e in &entities {
            for t in &e.tags {
                *tag_carrier_count.entry(*t).or_default() += 1;
            }
        }

        // Classify tags: every (transitive) descendant of the `Ship` tag
        // root is a ship-selective tag. Everything else (AI, Missions,
        // Audio, …) is faction / context / flavour and can't select a
        // ship model on its own.
        let tag_tree = &datacore.snapshot().tag_tree;
        let mut ship_tags: HashSet<Guid> = HashSet::new();
        for root_guid in tag_tree.by_name(SHIP_TAG_ROOT_NAME) {
            ship_tags.insert(*root_guid);
            for desc in tag_tree.descendants(root_guid) {
                ship_tags.insert(desc.guid);
            }
        }

        Self {
            entities,
            ship_relevant_tags,
            spawn_referenced_tags,
            tag_carrier_count,
            ship_tags,
        }
    }

    /// Total number of indexed ship entities.
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShipEntity> + '_ {
        self.entities.iter()
    }

    /// Tags the contract graph queries inside ship-spawn descriptions.
    /// Useful for per-query filtering — consumers can intersect their
    /// input query tags with this set to drop anything the ship pool
    /// doesn't recognise.
    pub fn spawn_referenced_tags(&self) -> &HashSet<Guid> {
        &self.spawn_referenced_tags
    }

    /// Every tag present on at least one registered ship entity.
    pub fn ship_relevant_tags(&self) -> &HashSet<Guid> {
        &self.ship_relevant_tags
    }

    /// Resolve a spawn query into a list of candidate ships.
    ///
    /// Classification uses the `sc_extract::TagTree` hierarchy:
    ///
    /// - **Ship-selective** tags are descendants of the tree's `Ship`
    ///   root (`Cutter`, `Gladius`, `Idris_M`, …).
    /// - **Carried non-ship** tags live under other roots (`AI`,
    ///   `Missions`) but are attached to some pool entities
    ///   (`Criminal`, `PU`, `VeryHard`). They narrow the pool but
    ///   don't identify a specific model.
    /// - **Pure context** tags are non-ship and attached to no pool
    ///   entity (AI skill levels, runtime flags like `ArriveViaQT`,
    ///   role markers like `Target`). They describe spawn context
    ///   the engine resolves at runtime.
    ///
    /// Resolution has two modes, dispatched by the **original** query's
    /// intent:
    ///
    /// 1. **Ship-targeted query** — the positive set contains at least
    ///    one `Ship`-root tag. The contract wanted a specific model.
    ///    Drop pure-context tags. If the ship tag survived, strict
    ///    ALL-OF match on survivors + negative. If it didn't (zero
    ///    carriers — broken or typo'd selector) return empty rather
    ///    than over-match against the surviving broad tags. Example:
    ///    Gilly Mission01 Wave3 `{Relient_Tana, Criminal, …}` — the
    ///    Ship tag has 0 carriers in the DCB, so return empty instead
    ///    of matching every criminal ship.
    ///
    /// 2. **Broad query** — the positive set has no `Ship`-root tag.
    ///    The contract intentionally didn't pin a specific model.
    ///    Drop pure-context tags and strict-match the rest. Example:
    ///    TarPits Wave3 `{VeryHard, Criminal, PU}` → matches every
    ///    VeryHard Criminal PU ship.
    ///
    /// Returns candidates in the same stable order as [`Self::iter`].
    pub fn resolve_spawn(
        &self,
        positive: &HashSet<Guid>,
        negative: &HashSet<Guid>,
    ) -> Vec<ShipCandidate> {
        // Inspect intent on the *original* positive set — before any
        // filtering. Did the contract author attempt to select a
        // specific ship model?
        let has_ship_intent = positive.iter().any(|t| self.ship_tags.contains(t));

        // Drop tags with zero carriers in the pool (pure context, or
        // a ship selector the game data failed to attach to any
        // entity).
        let filtered: HashSet<Guid> = positive
            .iter()
            .filter(|t| self.tag_carrier_count.get(*t).copied().unwrap_or(0) > 0)
            .copied()
            .collect();
        if filtered.is_empty() {
            return Vec::new();
        }

        // Ship-targeted queries must retain at least one ship tag
        // post-filter, else the selector was broken and we refuse to
        // over-match against the surviving broad tags. Broad queries
        // (no ship-intent at all) are allowed to proceed on carried
        // non-ship tags alone.
        if has_ship_intent && !filtered.iter().any(|t| self.ship_tags.contains(t)) {
            return Vec::new();
        }

        self.entities
            .iter()
            .filter(|ship| {
                filtered.iter().all(|t| ship.tags.contains(t))
                    && negative.iter().all(|t| !ship.tags.contains(t))
            })
            .map(|ship| ShipCandidate {
                display_name: ship.display_name.clone(),
                size: ship.size,
                entity_guid: ship.entity_guid,
            })
            .collect()
    }

    /// The set of ship-selective tag GUIDs — every descendant of the
    /// tag tree's `Ship` root. Exposed so the audit example (and
    /// potentially other consumers) can classify individual tags.
    pub fn ship_tags(&self) -> &HashSet<Guid> {
        &self.ship_tags
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn collect_tags(
    pools: &DataPools,
    list: Option<&sc_extract::generated::Handle<TagList>>,
    out: &mut HashSet<Guid>,
) {
    let Some(h) = list else { return };
    let Some(tag_list) = h.get(pools) else { return };
    out.extend(tag_list.tags.iter().copied());
}

fn extract_size(ecd: &EntityClassDefinition, pools: &DataPools) -> i32 {
    for comp in &ecd.components {
        if let DataForgeComponentParamsPtr::SAttachableComponentParams(h) = comp
            && let Some(att) = h.get(pools)
            && let Some(def_h) = att.attach_def.as_ref()
            && let Some(item_def) = def_h.get(pools)
        {
            return item_def.size;
        }
    }
    0
}
