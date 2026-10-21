//! Every entity's own item ports and default loadout, from the DCB.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EntityClassDefinition, Handle, SCItemControlBaseParamsPtr,
    SCItemControllableParamsPtr, SItemPortDef, SItemPortLoadoutBaseParamsPtr,
    SItemPortLoadoutEntryParams,
};
use sc_extract::object_container::{self, XmlNode};
use sc_extract::{
    AssetSource, ControlPriority, DataPools, Datacore, Guid, PortType, PriorityGroup,
    RecordCollection,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::network::power_pools;
use crate::{Port, PortSource, PowerPool};

/// An entity's `SItemPortContainerComponentParams.Ports` and its default
/// loadout (`SEntityComponentDefaultLoadoutParams`): a manual loadout in
/// the DCB, or an XML loadout file it names (`loadoutPath`).
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

/// One entry of a default loadout: an `SItemPortLoadoutEntryParams`, or an
/// `<Item portName itemName>` of an XML loadout file.
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

    /// Read every entity's ports and default loadout; XML loadout files
    /// (doors that carry a cargo grid, beds, lockers) are read from
    /// `assets`.
    pub fn build(datacore: &Datacore, assets: &AssetSource) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut ctx = Ctx {
            pools,
            names: class_names(datacore),
            assets,
            files: HashMap::new(),
        };
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let container = container(ecd, &mut ctx);
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

/// What reading loadouts needs: the record pools, record names for the
/// entries that name their class, and the XML loadout files read so far
/// (by lower-cased path; many entities share one).
struct Ctx<'a> {
    pools: &'a DataPools,
    names: HashMap<String, Guid>,
    assets: &'a AssetSource,
    files: HashMap<String, Vec<LoadoutEntry>>,
}

fn container(ecd: &EntityClassDefinition, ctx: &mut Ctx) -> PortContainer {
    let pools = ctx.pools;
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
                    out.default_loadout = loadout(l.loadout.as_ref(), ctx);
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
    let (user, usable) = match &p.control {
        Some(SCItemControlBaseParamsPtr::SCItemControlSeatParams(h)) => h
            .get(pools)
            .map(|c| (c.user_control.as_ref(), c.usable_control.as_ref()))
            .unwrap_or_default(),
        Some(SCItemControlBaseParamsPtr::SCItemControlUsableParams(h)) => {
            (None, h.get(pools).and_then(|c| c.usable_control.as_ref()))
        }
        _ => (None, None),
    };
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
        controllable_tags: p
            .controllable_tag
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_string)
            .collect(),
        user_control: priority_groups(user, pools),
        usable_control: priority_groups(usable, pools),
    })
}

fn priority_groups(
    c: Option<&SCItemControllableParamsPtr>,
    pools: &DataPools,
) -> Vec<PriorityGroup> {
    let Some(SCItemControllableParamsPtr::SCItemControllableGroupParams(h)) = c else {
        return Vec::new();
    };
    let Some(g) = h.get(pools) else {
        return Vec::new();
    };
    g.priority_groups
        .iter()
        .filter_map(|h| h.get(pools))
        .map(|g| PriorityGroup {
            item_type: g.item_type.clone(),
            default: ControlPriority::from_dcb(g.default_priority),
            tags: g
                .tags
                .iter()
                .filter_map(|t| t.get(pools))
                .map(|t| (t.tag.clone(), ControlPriority::from_dcb(t.priority)))
                .collect(),
        })
        .collect()
}

fn loadout(l: Option<&SItemPortLoadoutBaseParamsPtr>, ctx: &mut Ctx) -> Vec<LoadoutEntry> {
    let pools = ctx.pools;
    match l {
        Some(SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutManualParams(h)) => {
            let Some(m) = h.get(pools) else {
                return Vec::new();
            };
            m.entries
                .iter()
                .filter_map(|e| e.get(pools))
                .map(|e| loadout_entry(e, ctx))
                .collect()
        }
        Some(SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutXMLParams(h)) => {
            match h.get(pools).map(|x| x.loadout_path.as_str()) {
                Some(path) if !path.is_empty() => xml_loadout(path, ctx),
                _ => Vec::new(),
            }
        }
        // Outfits and random loadouts dress characters; LIVE 4.10 uses no
        // record loadout.
        Some(_) | None => Vec::new(),
    }
}

fn loadout_entry(e: &SItemPortLoadoutEntryParams, ctx: &mut Ctx) -> LoadoutEntry {
    // Typed reference first. Many entries name the class by record name
    // only (`entityClassName`, reference empty), so the name is the
    // fallback.
    let item = e
        .entity_class_reference
        .or_else(|| class_by_name(&e.entity_class_name, &e.item_port_name, ctx));
    LoadoutEntry {
        port: e.item_port_name.clone(),
        item,
        children: loadout(e.loadout.as_ref(), ctx),
    }
}

/// An EntityClassDefinition record by name, matched case-insensitively as
/// the game's class registry does: loadout entries and XML loadout files
/// name their classes, with no typed reference.
fn class_by_name(name: &str, port: &str, ctx: &Ctx) -> Option<Guid> {
    if name.is_empty() {
        return None;
    }
    let hit = ctx.names.get(&name.to_ascii_lowercase()).copied();
    if hit.is_none() {
        warn!(
            class = %name,
            port,
            "default loadout names a class with no EntityClassDefinition record"
        );
    }
    hit
}

/// An XML loadout file (`<Loadout><Items><Item portName itemName>`, an
/// item's own `<Items>` nested in it), read once per path.
fn xml_loadout(path: &str, ctx: &mut Ctx) -> Vec<LoadoutEntry> {
    let key = path.to_ascii_lowercase();
    if let Some(entries) = ctx.files.get(&key) {
        return entries.clone();
    }
    let root = ctx
        .assets
        .read_data(path)
        .map_err(|e| e.to_string())
        .and_then(|b| object_container::decode(&b).map_err(|e| e.to_string()));
    let entries = match root {
        Ok(Some(root)) => xml_items(&root, ctx),
        Ok(None) => Vec::new(),
        Err(error) => {
            warn!(path, %error, "XML loadout unreadable");
            Vec::new()
        }
    };
    ctx.files.insert(key, entries.clone());
    entries
}

fn xml_items(node: &XmlNode, ctx: &Ctx) -> Vec<LoadoutEntry> {
    node.children_named("Items")
        .flat_map(|items| items.children_named("Item"))
        .filter_map(|item| {
            let port = item.attr("portName")?.to_string();
            Some(LoadoutEntry {
                item: class_by_name(item.attr("itemName").unwrap_or(""), &port, ctx),
                children: xml_items(item, ctx),
                port,
            })
        })
        .collect()
}

/// Lower-cased `EntityClassDefinition` record name → GUID, for the loadout
/// entries that name their class instead of referencing it.
pub(crate) fn class_names(datacore: &Datacore) -> HashMap<String, Guid> {
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
