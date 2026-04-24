// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-frontend`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `LoadoutDummyComponentParams`
pub struct LoadoutDummyComponentParams {
    /// `playerTagPoint` (String)
    pub player_tag_point: String,
    /// `playerIdleAnim` (String)
    pub player_idle_anim: String,
    /// `playerDisplayParams` (Class)
    pub player_display_params: Option<Handle<UIWorldDisplay3DParams>>,
    /// `vehicleTagPoint` (String)
    pub vehicle_tag_point: String,
    /// `vehicleBoundingBox` (Class)
    pub vehicle_bounding_box: Option<Handle<Vec3>>,
    /// `vehicleAngle` (Class)
    pub vehicle_angle: Option<Handle<Ang3>>,
    /// `vehicleDisplayParams` (Class)
    pub vehicle_display_params: Option<Handle<UIWorldDisplay3DParams>>,
}

impl Pooled for LoadoutDummyComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_frontend.loadout_dummy_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_frontend.loadout_dummy_component_params
    }
}

impl<'a> Extract<'a> for LoadoutDummyComponentParams {
    const TYPE_NAME: &'static str = "LoadoutDummyComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_tag_point: inst
                .get_str("playerTagPoint")
                .map(String::from)
                .unwrap_or_default(),
            player_idle_anim: inst
                .get_str("playerIdleAnim")
                .map(String::from)
                .unwrap_or_default(),
            player_display_params: match inst.get("playerDisplayParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<UIWorldDisplay3DParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            vehicle_tag_point: inst
                .get_str("vehicleTagPoint")
                .map(String::from)
                .unwrap_or_default(),
            vehicle_bounding_box: match inst.get("vehicleBoundingBox") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            vehicle_angle: match inst.get("vehicleAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            vehicle_display_params: match inst.get("vehicleDisplayParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<UIWorldDisplay3DParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
