//! How the inventory presents an item: the orientation offset and light rig
//! of its category (`PlayerChoice_PITConfig.inventoryParams`), and the light
//! rigs themselves (the `inventorylightgroup_*` entities).
//!
//! The cell scene (`inventory_panel_item_itemwindow`) is Y-up with the camera
//! on −Z and screen-up −Y; the item turns by Euler XYZ of
//! [`InventoryView::default_orientation`] plus its category's offset; the rig
//! sits at the cell centre turned by (90, 180, 0), each light a projector
//! shining along its local +X. Those scene facts live in the renderer.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EItemSubType, EItemType, Handle, ItemCategory,
    PersonalThoughtInventoryParams, SAttachmentImplementationBasePtr, SLightItem,
};
use sc_extract::{DataPools, Datacore, Guid};
use sc_items::Item;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::Visual;

/// The inventory's presentation tables and light rigs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventoryView {
    /// `defaultOrientation`, degrees (90, 0, 0 in 4.x).
    pub default_orientation: [f32; 3],
    orientations: Vec<(Vec<Category>, [f32; 3])>,
    light_groups: Vec<(Vec<Category>, Guid)>,
    /// `defaultLightGroupEntityClass`: the rig for items no entry matches.
    pub default_light_group: Option<Guid>,
    rigs: HashMap<Guid, LightRig>,
}

/// One `ItemCategory`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Category {
    #[serde(with = "enum_serde::item_type")]
    item_type: EItemType,
    #[serde(with = "enum_serde::item_sub_type")]
    sub_type: EItemSubType,
    required_tag: Option<Guid>,
}

/// A light group entity's lights.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LightRig {
    pub lights: Vec<RigLight>,
}

/// One `SLightItem`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RigLight {
    /// `SLightItem_key`, `SLightItem_rim`, …
    pub name: String,
    /// `lightType` as authored (`Projector`, `Omni`, …).
    pub kind: String,
    pub active: bool,
    /// Position in the rig's frame.
    pub position: [f32; 3],
    /// `Ang3` degrees; a projector shines along its local +X.
    pub rotation: [f32; 3],
    /// Default-state colour, linear RGB.
    pub color: [f32; 3],
    pub intensity: f32,
    /// Kelvin, used instead of `color` when `use_temperature`.
    pub temperature: f32,
    pub use_temperature: bool,
    /// `lightRadius`: the range.
    pub radius: f32,
    pub bulb_radius: f32,
    /// Projector cone, degrees.
    pub fov: f32,
    pub casts_shadows: bool,
}

impl InventoryView {
    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let configs = &store.records.multi_feature.player_choice_pitconfig;
        if configs.len() > 1 {
            warn!(
                count = configs.len(),
                "several PlayerChoice_PITConfig records; using an arbitrary one"
            );
        }
        let Some(params) = configs
            .values()
            .filter_map(|h| h.get(pools))
            .find_map(|c| c.inventory_params.and_then(|p| p.get(pools)))
        else {
            warn!("no PlayerChoice_PITConfig inventory params");
            return Self::default();
        };
        let mut view = from_params(params, pools);
        let ecds = &store.records.multi_feature.entity_class_definition;
        let wanted: Vec<Guid> = view
            .light_groups
            .iter()
            .map(|(_, g)| *g)
            .chain(view.default_light_group)
            .collect();
        for guid in wanted {
            if view.rigs.contains_key(&guid) {
                continue;
            }
            match ecds.get(&guid).and_then(|h| h.get(pools)) {
                Some(ecd) => {
                    view.rigs.insert(guid, rig(ecd, pools));
                }
                None => warn!(%guid, "inventory light group entity not found"),
            }
        }
        view
    }

    /// The category orientation offset for an item, degrees; `None` when no
    /// entry matches (the game then turns the item by
    /// [`InventoryView::default_orientation`] alone).
    pub fn orientation(&self, item: &Item, visual: &Visual) -> Option<[f32; 3]> {
        best(&self.orientations, item, &visual.tags).copied()
    }

    /// The light group entity lighting an item's cell.
    pub fn light_group(&self, item: &Item, visual: &Visual) -> Option<Guid> {
        best(&self.light_groups, item, &visual.tags)
            .copied()
            .or(self.default_light_group)
    }

    /// The light rig of a light group entity.
    pub fn rig(&self, light_group: &Guid) -> Option<&LightRig> {
        self.rigs.get(light_group)
    }
}

/// The best-matching entry. A category matches on the item type, its sub
/// type or `UNDEFINED`, and its `requiredTag` (when set) among the item's
/// tags. An exact sub type outranks `UNDEFINED`, a required tag outranks
/// none; ties keep table order. Verified: LH86 (tag `cdef6808`) gets the
/// sidearm rig and P4-AR the primary rig, both `WeaponPersonal/UNDEFINED`
/// entries; the Bosco rack (`Misc/Weapon`, no tags) matches neither tagged
/// `Misc/UNDEFINED` orientation, and the game shows it unturned.
fn best<'a, T>(table: &'a [(Vec<Category>, T)], item: &Item, tags: &[Guid]) -> Option<&'a T> {
    let score = |c: &Category| -> Option<u8> {
        if c.item_type != item.item_type {
            return None;
        }
        let exact = c.sub_type == item.item_sub_type;
        if !exact && c.sub_type != EItemSubType::UNDEFINED {
            return None;
        }
        let tagged = match c.required_tag {
            Some(t) if !tags.contains(&t) => return None,
            Some(_) => 1,
            None => 0,
        };
        Some(u8::from(exact) * 2 + tagged)
    };
    let mut best: Option<(u8, &T)> = None;
    for (cats, value) in table {
        if let Some(s) = cats.iter().filter_map(score).max()
            && best.is_none_or(|(b, _)| s > b)
        {
            best = Some((s, value));
        }
    }
    best.map(|(_, v)| v)
}

fn categories(list: &[Handle<ItemCategory>], pools: &DataPools) -> Vec<Category> {
    list.iter()
        .filter_map(|h| h.get(pools))
        .map(|c| Category {
            item_type: c.r#type.clone(),
            sub_type: c.sub_type.clone(),
            required_tag: c.required_tag,
        })
        .collect()
}

fn from_params(p: &PersonalThoughtInventoryParams, pools: &DataPools) -> InventoryView {
    let v3 = |h: Option<Handle<sc_extract::generated::Vec3>>| {
        h.and_then(|h| h.get(pools))
            .map_or([0.0; 3], |v| [v.x, v.y, v.z])
    };
    InventoryView {
        default_orientation: v3(p.default_orientation),
        orientations: p
            .list_of_items_orientation_offset
            .iter()
            .filter_map(|h| h.get(pools))
            .map(|o| {
                (
                    categories(&o.item_categories, pools),
                    v3(o.orientation_offset),
                )
            })
            .collect(),
        light_groups: p
            .list_of_item_light_groups
            .iter()
            .filter_map(|h| h.get(pools))
            .filter_map(|l| {
                Some((
                    categories(&l.item_categories, pools),
                    l.light_group_entity_class?,
                ))
            })
            .collect(),
        default_light_group: p.default_light_group_entity_class,
        rigs: HashMap::new(),
    }
}

fn rig(ecd: &sc_extract::generated::EntityClassDefinition, pools: &DataPools) -> LightRig {
    let lights = ecd
        .components
        .iter()
        .filter_map(|c| match c {
            DataForgeComponentParamsPtr::SLightGroupComponentParams(h) => h.get(pools),
            _ => None,
        })
        .flat_map(|g| g.light_items.iter().filter_map(|l| l.get(pools)))
        .filter_map(|l| light(l, pools))
        .collect();
    LightRig { lights }
}

fn light(l: &SLightItem, pools: &DataPools) -> Option<RigLight> {
    let p = l.light_params?.get(pools)?;
    // The inventory rigs place their lights with `…Entity`; `…Bone` carries
    // the same offset for lights on a skeleton.
    let offset = match &l.attachment_implementation {
        Some(SAttachmentImplementationBasePtr::SAttachmentImplementationEntity(e)) => {
            e.get(pools).and_then(|e| e.offset)
        }
        Some(SAttachmentImplementationBasePtr::SAttachmentImplementationBone(b)) => {
            b.get(pools).and_then(|b| b.offset)
        }
        _ => None,
    }
    .and_then(|o| o.get(pools));
    let position = offset
        .and_then(|o| o.position)
        .and_then(|v| v.get(pools))
        .map_or([0.0; 3], |v| [v.x, v.y, v.z]);
    let rotation = offset
        .and_then(|o| o.rotation)
        .and_then(|v| v.get(pools))
        .map_or([0.0; 3], |v| [v.x, v.y, v.z]);
    let state = p.default_state.and_then(|s| s.get(pools));
    let size = p.size_params.and_then(|s| s.get(pools));
    Some(RigLight {
        name: l.name.clone(),
        kind: p.light_type.as_dcb_str().to_string(),
        active: p.active,
        position,
        rotation,
        color: state
            .and_then(|s| s.color)
            .and_then(|c| c.get(pools))
            .map_or([1.0; 3], |c| [c.r, c.g, c.b]),
        intensity: state.map_or(0.0, |s| s.intensity),
        temperature: state.map_or(0.0, |s| s.temperature),
        use_temperature: p.use_temperature,
        radius: size.map_or(0.0, |s| s.light_radius),
        bulb_radius: size.map_or(0.0, |s| s.bulb_radius),
        fov: p
            .projector_params
            .and_then(|pp| pp.get(pools))
            .map_or(0.0, |pp| pp.fov),
        casts_shadows: p
            .shadow_params
            .and_then(|s| s.get(pools))
            .is_some_and(|s| s.shadow_casting),
    })
}

/// serde adapters for the generated enums (stored as their DCB strings), as
/// in sc-items.
mod enum_serde {
    use sc_extract::generated::{EItemSubType, EItemType};
    use serde::{Deserialize, Deserializer, Serializer};

    pub mod item_type {
        use super::*;
        pub fn serialize<S: Serializer>(v: &EItemType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemType, D::Error> {
            Ok(EItemType::from_dcb_str(&String::deserialize(d)?))
        }
    }

    pub mod item_sub_type {
        use super::*;
        pub fn serialize<S: Serializer>(v: &EItemSubType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemSubType, D::Error> {
            Ok(EItemSubType::from_dcb_str(&String::deserialize(d)?))
        }
    }
}
