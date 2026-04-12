// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `starmap`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `StarMapObjectType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObjectType {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `classification` (Locale)
    #[serde(default)]
    pub classification: String,
    /// `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// `selectable` (Boolean)
    #[serde(default)]
    pub selectable: bool,
    /// `fadeBehindParent` (Boolean)
    #[serde(default)]
    pub fade_behind_parent: bool,
    /// `onParentSurface` (Boolean)
    #[serde(default)]
    pub on_parent_surface: bool,
    /// `spawnNavPoints` (Boolean)
    #[serde(default)]
    pub spawn_nav_points: bool,
    /// `showAsNeighbor` (Boolean)
    #[serde(default)]
    pub show_as_neighbor: bool,
    /// `innerCulling` (Boolean)
    #[serde(default)]
    pub inner_culling: bool,
    /// `showInMapSelectList` (Boolean)
    #[serde(default)]
    pub show_in_map_select_list: bool,
    /// `markerConfig` (Reference)
    #[serde(default)]
    pub marker_config: Option<CigGuid>,
    /// `validQuantumTravelDestination` (Boolean)
    #[serde(default)]
    pub valid_quantum_travel_destination: bool,
    /// `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapObjectType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.starmap.star_map_object_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.starmap.star_map_object_type }
}

impl<'a> Extract<'a> for StarMapObjectType {
    const TYPE_NAME: &'static str = "StarMapObjectType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            classification: inst.get_str("classification").map(String::from).unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            selectable: inst.get_bool("selectable").unwrap_or_default(),
            fade_behind_parent: inst.get_bool("fadeBehindParent").unwrap_or_default(),
            on_parent_surface: inst.get_bool("onParentSurface").unwrap_or_default(),
            spawn_nav_points: inst.get_bool("spawnNavPoints").unwrap_or_default(),
            show_as_neighbor: inst.get_bool("showAsNeighbor").unwrap_or_default(),
            inner_culling: inst.get_bool("innerCulling").unwrap_or_default(),
            show_in_map_select_list: inst.get_bool("showInMapSelectList").unwrap_or_default(),
            marker_config: inst.get("markerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            valid_quantum_travel_destination: inst.get_bool("validQuantumTravelDestination").unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StarMapObjectTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObjectTypes {
    /// `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for StarMapObjectTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.starmap.star_map_object_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.starmap.star_map_object_types }
}

impl<'a> Extract<'a> for StarMapObjectTypes {
    const TYPE_NAME: &'static str = "StarMapObjectTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapMissionObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapMissionObject {
    /// `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapMissionObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.starmap.star_map_mission_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.starmap.star_map_mission_object }
}

impl<'a> Extract<'a> for StarMapMissionObject {
    const TYPE_NAME: &'static str = "StarMapMissionObject";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StarMapPartyMemberObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapPartyMemberObject {
    /// `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapPartyMemberObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.starmap.star_map_party_member_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.starmap.star_map_party_member_object }
}

impl<'a> Extract<'a> for StarMapPartyMemberObject {
    const TYPE_NAME: &'static str = "StarMapPartyMemberObject";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

