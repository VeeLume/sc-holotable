//! Every entity's own item ports and default loadout, from the DCB.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EntityClassDefinition, Handle, SItemPortDef,
    SItemPortLoadoutBaseParamsPtr, SItemPortLoadoutEntryParams,
};
use sc_extract::{DataPools, Datacore, Guid, PortType, RecordCollection};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::network::power_pools;
use crate::{Port, PortSource, PowerPool};

/// An entity's `SItemPortContainerComponentParams.Ports` and its default
/// loadout (`SEntityComponentDefaultLoadoutParams`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortContainer {
    pub ports: Vec<Port>,
    pub default_loadout: Vec<LoadoutEntry>,
    /// `resourceNetworkPowerPools.itemPools`: the item types that share a
    /// power budget on this entity's network (vehicles only).
    #[serde(default)]
    pub power_pools: Vec<PowerPool>,
    /// `resourceNetworkPowerPools.maxDefaultDistribution`: at power-on every
    /// pool gets its whole size rather than its default share.
    #[serde(default)]
    pub max_default_distribution: bool,
}

impl PortContainer {
    /// The port named `name` (case-insensitive, as the game matches them).
    pub fn port(&self, name: &str) -> Option<&Port> {
        self.ports
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }
}

/// One `SItemPortLoadoutEntryParams` of a default loadout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadoutEntry {
    pub port: String,
    /// The attached item's record; `None` when the entry is empty or names
    /// no record that exists.
    pub item: Option<Guid>,
    /// The entry's own nested loadout (for the attached item's ports);
    /// empty means the item's own default loadout applies.
    pub children: Vec<LoadoutEntry>,
}

/// Every entity's [`PortContainer`], keyed by `EntityClassDefinition` GUID:
/// the vehicles and the items in their loadouts (gimbals, turrets, racks,
/// quantum drives with a jump-drive port, …).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortContainers {
    by_guid: HashMap<Guid, PortContainer>,
}

impl PortContainers {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let names = class_names(datacore);
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let container = container(ecd, pools, &names);
            if !container.ports.is_empty()
                || !container.default_loadout.is_empty()
                || !container.power_pools.is_empty()
            {
                debug!(%guid, ports = container.ports.len(), "port container");
                by_guid.insert(guid, container);
            }
        }
        Self { by_guid }
    }
}

impl RecordCollection for PortContainers {
    type Item = PortContainer;

    fn get(&self, guid: &Guid) -> Option<&PortContainer> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &PortContainer)> + '_ {
        self.by_guid.iter()
    }
}

fn container(
    ecd: &EntityClassDefinition,
    pools: &DataPools,
    names: &HashMap<String, Guid>,
) -> PortContainer {
    let mut out = PortContainer::default();
    for c in &ecd.components {
        match c {
            DataForgeComponentParamsPtr::SItemPortContainerComponentParams(h) => {
                if let Some(pc) = h.get(pools) {
                    out.ports
                        .extend(pc.ports.iter().filter_map(|p| entity_port(*p, pools)));
                    out.power_pools = power_pools(pc.resource_network_power_pools, pools);
                    out.max_default_distribution = pc
                        .resource_network_power_pools
                        .and_then(|h| h.get(pools))
                        .is_some_and(|s| s.max_default_distribution);
                }
            }
            DataForgeComponentParamsPtr::SEntityComponentDefaultLoadoutParams(h) => {
                if let Some(l) = h.get(pools) {
                    out.default_loadout = loadout(l.loadout.as_ref(), pools, names);
                }
            }
            _ => {}
        }
    }
    out
}

fn entity_port(h: Handle<SItemPortDef>, pools: &DataPools) -> Option<Port> {
    let p = h.get(pools)?;
    let words = |s: &str| s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    Some(Port {
        name: p.name.clone(),
        display_name: (!p.display_name.is_empty()).then(|| p.display_name.clone()),
        source: PortSource::Entity,
        types: p
            .types
            .iter()
            .filter_map(|t| t.get(pools))
            .map(|t| PortType {
                item_type: t.r#type.clone(),
                sub_types: t.sub_types.clone(),
            })
            .collect(),
        min_size: p.min_size,
        max_size: p.max_size,
        flags: words(&p.flags),
        port_tags: words(&p.port_tags),
        required_tags: words(&p.required_port_tags),
        default_item: p
            .default_item
            .and_then(|d| d.get(pools))
            .and_then(|d| d.entity_class),
    })
}

fn loadout(
    l: Option<&SItemPortLoadoutBaseParamsPtr>,
    pools: &DataPools,
    names: &HashMap<String, Guid>,
) -> Vec<LoadoutEntry> {
    match l {
        Some(SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutManualParams(h)) => h
            .get(pools)
            .map(|m| {
                m.entries
                    .iter()
                    .filter_map(|e| e.get(pools))
                    .map(|e| loadout_entry(e, pools, names))
                    .collect()
            })
            .unwrap_or_default(),
        // No vehicle uses an XML or record loadout in LIVE 4.10 (8 empty-path
        // XML templates).
        Some(_) | None => Vec::new(),
    }
}

fn loadout_entry(
    e: &SItemPortLoadoutEntryParams,
    pools: &DataPools,
    names: &HashMap<String, Guid>,
) -> LoadoutEntry {
    // Typed reference first. Many entries name the class by record name
    // only (`entityClassName`, reference empty), so the name is the
    // fallback: an EntityClassDefinition record name, matched
    // case-insensitively as the game's class registry does.
    let item = e.entity_class_reference.or_else(|| {
        if e.entity_class_name.is_empty() {
            return None;
        }
        let hit = names
            .get(&e.entity_class_name.to_ascii_lowercase())
            .copied();
        if hit.is_none() {
            warn!(
                class = %e.entity_class_name,
                port = %e.item_port_name,
                "default loadout names a class with no EntityClassDefinition record"
            );
        }
        hit
    });
    LoadoutEntry {
        port: e.item_port_name.clone(),
        item,
        children: loadout(e.loadout.as_ref(), pools, names),
    }
}

/// Lower-cased `EntityClassDefinition` record name → GUID, for the loadout
/// entries that name their class instead of referencing it.
fn class_names(datacore: &Datacore) -> HashMap<String, Guid> {
    datacore
        .db()
        .all_records()
        .filter(|r| r.type_name() == Some("EntityClassDefinition"))
        .filter_map(|r| {
            let name = r.name()?;
            let short = name.split_once('.').map_or(name, |(_, n)| n);
            Some((short.to_ascii_lowercase(), r.id()))
        })
        .collect()
}
