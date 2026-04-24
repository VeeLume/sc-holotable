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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `StarMapObjectTypes`
pub struct StarMapObjectTypes {
    /// `types` (Reference (array))
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
pub struct StarMapMissionObject {
    /// `minimumDisplaySize` (Single)
    pub minimum_display_size: f32,
    /// `rotationSpeed` (Single)
    pub rotation_speed: f32,
    /// `facingMode` (EnumChoice)
    pub facing_mode: WorldDisplayObjectFacingMode,
    /// `geometry` (Class)
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `material` (Class)
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
            facing_mode: WorldDisplayObjectFacingMode::from_dcb_str(inst.get_str("facingMode").unwrap_or("")),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StarMapPartyMemberObject`
pub struct StarMapPartyMemberObject {
    /// `minimumDisplaySize` (Single)
    pub minimum_display_size: f32,
    /// `rotationSpeed` (Single)
    pub rotation_speed: f32,
    /// `facingMode` (EnumChoice)
    pub facing_mode: WorldDisplayObjectFacingMode,
    /// `geometry` (Class)
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `material` (Class)
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
            facing_mode: WorldDisplayObjectFacingMode::from_dcb_str(inst.get_str("facingMode").unwrap_or("")),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

