//! What an item looks like: the render recipe a renderer needs to draw an
//! item the way the game's inventory does, without hand-written tables.
//!
//! - [`Visuals`]: per item record, the geometry variants with their material
//!   and tint palette, the skin tag selecting among them, the item ports with
//!   their attach helpers, and the default loadout.
//! - [`Palettes`]: the `TintPaletteTree` records the recipes point at.
//! - [`InventoryView`]: how the inventory presents an item (orientation
//!   offset, light rig) and the size of its grid cell.
//! - [`Visuals::scene`]: an item assembled for drawing, with what is
//!   attached to it ([`Scene`]).
//!
//! Geometry files are paths into `Data.p4k`; reading them is
//! `sc_extract::{MeshGeometry, Material, CharacterDefinition, Skeleton}`.
//!
//! ```no_run
//! use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordCollection};
//! use sc_items::Items;
//! use sc_items_visuals::{Palettes, Visuals};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let data = AssetData::extract(&assets, &AssetConfig::standard())?;
//! let datacore = Datacore::parse(&assets, &data)?;
//! let items = Items::build(datacore.records());
//! let visuals = Visuals::build(&datacore, &items);
//! let palettes = Palettes::build(&datacore);
//! for (guid, v) in visuals.iter().take(3) {
//!     let look = v.look(&[]);
//!     println!("{guid}: {:?}", look.map(|l| l.geometry));
//! }
//! # let _ = palettes;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod inventory;
mod palette;
mod scene;

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EntityClassDefinition, Handle, QuatT, SGeometryNodeParams,
    SItemPortDefAttachmentImplementationBasePtr, SItemPortLoadoutBaseParamsPtr,
    SItemPortLoadoutEntryParams, SMaterialNodeParams, TintPaletteRef,
};
use sc_extract::{DataPools, Datacore, Guid};
use sc_items::Items;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

pub use inventory::{InventoryView, LightRig, RigLight};
pub use palette::{Palette, PaletteEntry, Palettes};
pub use sc_extract::RecordCollection;
pub use scene::{
    Attached, Attachment, Body, MAX_DEPTH, Offset, Part, Piece, Scene, SceneNote, ang3_quaternion,
};

/// One item's render recipe.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visual {
    /// The entity's tags (`EntityClassDefinition.tags`); inventory tables
    /// match on them (`ItemCategory.requiredTag`).
    pub tags: Vec<Guid>,
    /// `SGeometryResourceParams.Geometry`: the base geometry and its tagged
    /// variants.
    pub geometry: Option<GeometryNode>,
    /// `SGeometryResourceParams.Material`: a material / palette applied to
    /// the whole item, with tagged variants.
    pub material: Option<MaterialNode>,
    /// `SCItemWeaponComponentParams.geometryTags`: the tag a skin record
    /// selects its variant with (`Sf01`); empty for the base item.
    pub geometry_tags: String,
    /// `SItemPortContainerComponentParams.ports`.
    pub ports: Vec<Port>,
    /// `SEntityComponentDefaultLoadoutParams`: what the item ships with.
    pub default_loadout: Vec<LoadoutEntry>,
    /// `AttachDef.inventoryOccupancyFixedGridDimensionsUIV4`: the inventory
    /// cell in grid units (width, height).
    pub grid: Option<[u32; 2]>,
}

/// An `SGeometryNodeParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeometryNode {
    /// Space-separated tags this variant is selected by; empty on the base.
    pub tags: String,
    /// `.cdf` / `.cga` / `.cgf` / `.skin`.
    pub geometry: Option<String>,
    /// `.mtl`; `None` keeps the parent's (or the geometry's own `MtlName`).
    pub material: Option<String>,
    pub palette: Option<PaletteRef>,
    pub variants: Vec<GeometryNode>,
}

/// An `SMaterialNodeParams`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialNode {
    pub tags: String,
    pub material: Option<String>,
    pub palette: Option<PaletteRef>,
    pub variants: Vec<MaterialNode>,
}

/// A `TintPaletteRef`: a `TintPaletteTree` record, optionally one of its
/// children by name. (`SwizzleOverride` is not modelled: no item record
/// sets it.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteRef {
    pub record: Guid,
    /// Child palette name (`accent_a`); empty for the root.
    pub child: String,
}

/// An item port and where an attached item goes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    /// `SItemPortDef.Name` (`magazine_attach`, `$slot_rifle_1`). Inventory
    /// dumps identify a child's port by crc32c of this name.
    pub name: String,
    /// The helper the child attaches to (`SItemPortDefAttachmentImplementationBone`):
    /// a skeleton bone or a geometry node of this item (`magAttach`).
    pub helper: Option<String>,
    /// The helper's `Offset`.
    pub offset: Transform,
    /// `ItemOffsetHelperName`: the attached item's own helper (bone or node)
    /// that meets this port's helper, instead of the item's origin. A
    /// backpack's stocked-weapon ports name `attach_offset_right_01` /
    /// `attach_offset_left_01` on the weapon.
    pub item_offset_helper: Option<String>,
    /// `SItemPortDefAttachmentImplementationSkin`: the attached item is
    /// skinned to this item's skeleton, so in bind pose it sits at this
    /// item's origin (a body's head, a head's hair, eyes and piercings).
    pub skinned: bool,
    /// `Flags`, space-separated as authored. Besides engine flags
    /// (`uneditable`, `invisible`) they name geometry variants for the
    /// attached item: a player head's `Eyes_ItemPort` says `Protos_Head`,
    /// which picks the eyes' `Protos_head` sub-geometry.
    pub flags: String,
}

/// A `QuatT` as authored: position, then `Ang3` rotation in degrees.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

/// One `SItemPortLoadoutEntryParams` of a default loadout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadoutEntry {
    pub port: String,
    /// The attached item's record; `None` when the entry is empty or names
    /// no record that exists.
    pub item: Option<Guid>,
    /// The entry's own nested loadout (for the attached item's ports).
    pub children: Vec<LoadoutEntry>,
}

/// A resolved look: one geometry with the material and palette that go with
/// it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Look {
    pub geometry: String,
    pub material: Option<String>,
    pub palette: Option<PaletteRef>,
}

impl Visual {
    /// The look for the item's own [`Visual::geometry_tags`] plus `extra`
    /// tags (e.g. `inventoryStoredEntity`).
    ///
    /// Starting at the base geometry node, the first variant whose tags are
    /// all requested is entered, repeatedly; each entered node overrides the
    /// geometry, material and palette it sets. A palette still unset then
    /// comes from the material node, walked the same way. Verified on the
    /// P4-AR "Star Kitten" (`Sf01`: own material and palette), LH86
    /// "Takahashi Racing" (base material, own palette) and the Bosco rack
    /// (palette only on the material node).
    pub fn look(&self, extra: &[&str]) -> Option<Look> {
        let mut wanted: Vec<String> = self
            .geometry_tags
            .split_whitespace()
            .chain(extra.iter().flat_map(|t| t.split_whitespace()))
            .map(str::to_ascii_lowercase)
            .collect();
        wanted.dedup();
        let selected = |tags: &str| {
            let tags: Vec<_> = tags.split_whitespace().collect();
            !tags.is_empty()
                && tags
                    .iter()
                    .all(|t| wanted.iter().any(|w| w.eq_ignore_ascii_case(t)))
        };

        let mut node = self.geometry.as_ref()?;
        let mut geometry = node.geometry.clone();
        let mut material = node.material.clone();
        let mut palette = node.palette.clone();
        while let Some(v) = node.variants.iter().find(|v| selected(&v.tags)) {
            node = v;
            geometry = node.geometry.clone().or(geometry);
            material = node.material.clone().or(material);
            palette = node.palette.clone().or(palette);
        }
        if let Some(mut m) = self.material.as_ref() {
            let mut m_material = m.material.clone();
            let mut m_palette = m.palette.clone();
            while let Some(v) = m.variants.iter().find(|v| selected(&v.tags)) {
                m = v;
                m_material = m.material.clone().or(m_material);
                m_palette = m.palette.clone().or(m_palette);
            }
            material = material.or(m_material);
            palette = palette.or(m_palette);
        }
        Some(Look {
            geometry: geometry?,
            material,
            palette,
        })
    }

    /// The port named `name` (case-insensitive, as the game matches them).
    pub fn port(&self, name: &str) -> Option<&Port> {
        self.ports
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
    }
}

/// Every item's [`Visual`], keyed by `EntityClassDefinition` GUID. Covers the
/// [`Items`] that carry `SGeometryResourceParams`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Visuals {
    by_guid: HashMap<Guid, Visual>,
}

impl Visuals {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let ecds = &store.records.multi_feature.entity_class_definition;
        let names = class_names(datacore);
        let mut by_guid = HashMap::new();
        for (&guid, &handle) in ecds {
            if items.get(&guid).is_none() {
                continue;
            }
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            if let Some(v) = visual(ecd, pools, &names) {
                debug!(%guid, "visual");
                by_guid.insert(guid, v);
            }
        }
        Self { by_guid }
    }
}

impl RecordCollection for Visuals {
    type Item = Visual;

    fn get(&self, guid: &Guid) -> Option<&Visual> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Visual)> + '_ {
        self.by_guid.iter()
    }
}

// ── extraction ──────────────────────────────────────────────────────────────

fn visual(
    ecd: &EntityClassDefinition,
    pools: &DataPools,
    names: &HashMap<String, Guid>,
) -> Option<Visual> {
    let mut geometry_params = None;
    let mut ports = Vec::new();
    let mut default_loadout = Vec::new();
    let mut geometry_tags = String::new();
    let mut grid = None;
    for c in &ecd.components {
        match c {
            DataForgeComponentParamsPtr::SGeometryResourceParams(h) => {
                geometry_params = geometry_params.or(h.get(pools));
            }
            DataForgeComponentParamsPtr::SItemPortContainerComponentParams(h) => {
                if let Some(pc) = h.get(pools) {
                    ports.extend(pc.ports.iter().filter_map(|p| port(*p, pools)));
                }
            }
            DataForgeComponentParamsPtr::SEntityComponentDefaultLoadoutParams(h) => {
                if let Some(l) = h.get(pools) {
                    default_loadout = loadout(l.loadout.as_ref(), pools, names);
                }
            }
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => {
                if let Some(w) = h.get(pools) {
                    geometry_tags = w.geometry_tags.clone();
                }
            }
            DataForgeComponentParamsPtr::SAttachableComponentParams(h) => {
                grid = h
                    .get(pools)
                    .and_then(|a| a.attach_def)
                    .and_then(|d| d.get(pools))
                    .and_then(|d| d.inventory_occupancy_fixed_grid_dimensions_uiv4)
                    .and_then(|v| v.get(pools))
                    .filter(|v| v.x >= 1.0 && v.y >= 1.0)
                    .map(|v| [v.x as u32, v.y as u32]);
            }
            _ => {}
        }
    }
    let params = geometry_params?;
    Some(Visual {
        tags: ecd.tags.clone(),
        geometry: params.geometry.and_then(|h| geometry_node(h, pools)),
        material: params.material.and_then(|h| material_node(h, pools)),
        geometry_tags,
        ports,
        default_loadout,
        grid,
    })
}

fn non_empty(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| s.to_string())
}

fn geometry_node(h: Handle<SGeometryNodeParams>, pools: &DataPools) -> Option<GeometryNode> {
    let n = h.get(pools)?;
    let data = n.geometry.and_then(|d| d.get(pools));
    Some(GeometryNode {
        tags: n.tags.clone(),
        geometry: data
            .and_then(|d| d.geometry)
            .and_then(|g| g.get(pools))
            .and_then(|g| non_empty(&g.path)),
        material: data
            .and_then(|d| d.material)
            .and_then(|m| m.get(pools))
            .and_then(|m| non_empty(&m.path)),
        palette: data
            .and_then(|d| d.palette)
            .and_then(|p| palette_ref(p, pools)),
        variants: n
            .sub_geometry
            .iter()
            .filter_map(|v| geometry_node(*v, pools))
            .collect(),
    })
}

fn material_node(h: Handle<SMaterialNodeParams>, pools: &DataPools) -> Option<MaterialNode> {
    let n = h.get(pools)?;
    Some(MaterialNode {
        tags: n.tags.clone(),
        material: n
            .material
            .and_then(|m| m.get(pools))
            .and_then(|m| non_empty(&m.path)),
        palette: n.palette.and_then(|p| palette_ref(p, pools)),
        variants: n
            .material_variants
            .iter()
            .filter_map(|v| material_node(*v, pools))
            .collect(),
    })
}

fn palette_ref(h: Handle<TintPaletteRef>, pools: &DataPools) -> Option<PaletteRef> {
    let r = h.get(pools)?;
    Some(PaletteRef {
        record: r.root_record?,
        child: r.child_path.clone(),
    })
}

fn transform(h: Option<Handle<QuatT>>, pools: &DataPools) -> Transform {
    let Some(q) = h.and_then(|h| h.get(pools)) else {
        return Transform::default();
    };
    Transform {
        position: q
            .position
            .and_then(|p| p.get(pools))
            .map_or([0.0; 3], |p| [p.x, p.y, p.z]),
        rotation: q
            .rotation
            .and_then(|r| r.get(pools))
            .map_or([0.0; 3], |r| [r.x, r.y, r.z]),
    }
}

fn port(h: Handle<sc_extract::generated::SItemPortDef>, pools: &DataPools) -> Option<Port> {
    let p = h.get(pools)?;
    let helper = match &p.attachment_implementation {
        Some(
            SItemPortDefAttachmentImplementationBasePtr::SItemPortDefAttachmentImplementationBone(
                b,
            ),
        ) => b
            .get(pools)
            .and_then(|b| b.helper)
            .and_then(|n| n.get(pools))
            .and_then(|n| n.helper)
            .and_then(|h| h.get(pools)),
        _ => None,
    };
    let skinned = matches!(
        p.attachment_implementation,
        Some(
            SItemPortDefAttachmentImplementationBasePtr::SItemPortDefAttachmentImplementationSkin(
                _
            )
        )
    );
    Some(Port {
        name: p.name.clone(),
        skinned,
        flags: p.flags.clone(),
        helper: helper.and_then(|h| non_empty(&h.name)),
        offset: helper.map_or_else(Transform::default, |h| transform(h.offset, pools)),
        item_offset_helper: helper.and_then(|h| non_empty(&h.item_offset_helper_name)),
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
        // Record / XML / outfit loadouts are not item icons' business yet.
        Some(_) | None => Vec::new(),
    }
}

fn loadout_entry(
    e: &SItemPortLoadoutEntryParams,
    pools: &DataPools,
    names: &HashMap<String, Guid>,
) -> LoadoutEntry {
    // Typed reference first. About half of all entries name the class by
    // record name only (`entityClassName`, reference empty), so the name is
    // the fallback; it is an EntityClassDefinition record name, matched
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

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        tags: &str,
        geometry: Option<&str>,
        material: Option<&str>,
        palette: Option<u8>,
    ) -> GeometryNode {
        GeometryNode {
            tags: tags.into(),
            geometry: geometry.map(Into::into),
            material: material.map(Into::into),
            palette: palette.map(|b| PaletteRef {
                record: Guid::from_bytes([b; 16]),
                child: String::new(),
            }),
            variants: Vec::new(),
        }
    }

    fn visual(geometry: GeometryNode, material: Option<MaterialNode>, tags: &str) -> Visual {
        Visual {
            tags: Vec::new(),
            geometry: Some(geometry),
            material,
            geometry_tags: tags.into(),
            ports: Vec::new(),
            default_loadout: Vec::new(),
            grid: None,
        }
    }

    #[test]
    fn skin_tag_selects_variant_and_inherits_unset_fields() {
        let mut base = node("", Some("w.cdf"), None, Some(1));
        base.variants
            .push(node("Sf01", Some("w.cdf"), Some("w_sf01.mtl"), Some(2)));
        base.variants.push(node("Pink01", None, None, Some(3)));
        let look = visual(base.clone(), None, "Sf01").look(&[]).unwrap();
        assert_eq!(look.material.as_deref(), Some("w_sf01.mtl"));
        assert_eq!(look.palette.unwrap().record, Guid::from_bytes([2; 16]));
        let look = visual(base.clone(), None, "pink01").look(&[]).unwrap();
        assert_eq!(look.geometry, "w.cdf");
        assert_eq!(look.material, None);
        assert_eq!(look.palette.unwrap().record, Guid::from_bytes([3; 16]));
        let look = visual(base, None, "").look(&[]).unwrap();
        assert_eq!(look.palette.unwrap().record, Guid::from_bytes([1; 16]));
    }

    #[test]
    fn material_node_palette_fills_in() {
        let m = MaterialNode {
            tags: String::new(),
            material: None,
            palette: Some(PaletteRef {
                record: Guid::from_bytes([9; 16]),
                child: String::new(),
            }),
            variants: Vec::new(),
        };
        let look = visual(node("", Some("rack.cga"), None, None), Some(m), "")
            .look(&[])
            .unwrap();
        assert_eq!(look.palette.unwrap().record, Guid::from_bytes([9; 16]));
    }

    #[test]
    fn multi_tag_variant_needs_every_tag() {
        let mut base = node("", Some("crate.cgf"), None, None);
        base.variants.push(node(
            "Male inventoryStoredEntity",
            Some("armor.skin"),
            None,
            None,
        ));
        let v = visual(base, None, "");
        assert_eq!(v.look(&["Male"]).unwrap().geometry, "crate.cgf");
        assert_eq!(
            v.look(&["Male", "inventoryStoredEntity"]).unwrap().geometry,
            "armor.skin"
        );
    }
}
