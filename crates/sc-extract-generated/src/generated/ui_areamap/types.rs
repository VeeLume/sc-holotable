// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-areamap`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `AreaMapCameraParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaMapCameraParams {
    /// `cameraDist` (Single)
    #[serde(default)]
    pub camera_dist: f32,
    /// `cameraPitch` (Single)
    #[serde(default)]
    pub camera_pitch: f32,
    /// `cameraYaw` (Single)
    #[serde(default)]
    pub camera_yaw: f32,
    /// `cameraZoomMax` (Single)
    #[serde(default)]
    pub camera_zoom_max: f32,
    /// `cameraZoomMin` (Single)
    #[serde(default)]
    pub camera_zoom_min: f32,
    /// `cameraPanCurve` (Class)
    #[serde(default)]
    pub camera_pan_curve: Option<Handle<BezierCurve>>,
    /// `cameraPanMultiplier` (Single)
    #[serde(default)]
    pub camera_pan_multiplier: f32,
    /// `cameraZoomCurve` (Class)
    #[serde(default)]
    pub camera_zoom_curve: Option<Handle<BezierCurve>>,
    /// `cameraZoomMultiplier` (Single)
    #[serde(default)]
    pub camera_zoom_multiplier: f32,
}

impl Pooled for AreaMapCameraParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_areamap.area_map_camera_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_areamap.area_map_camera_params }
}

impl<'a> Extract<'a> for AreaMapCameraParams {
    const TYPE_NAME: &'static str = "AreaMapCameraParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_dist: inst.get_f32("cameraDist").unwrap_or_default(),
            camera_pitch: inst.get_f32("cameraPitch").unwrap_or_default(),
            camera_yaw: inst.get_f32("cameraYaw").unwrap_or_default(),
            camera_zoom_max: inst.get_f32("cameraZoomMax").unwrap_or_default(),
            camera_zoom_min: inst.get_f32("cameraZoomMin").unwrap_or_default(),
            camera_pan_curve: match inst.get("cameraPanCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            camera_pan_multiplier: inst.get_f32("cameraPanMultiplier").unwrap_or_default(),
            camera_zoom_curve: match inst.get("cameraZoomCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            camera_zoom_multiplier: inst.get_f32("cameraZoomMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `AreaMapParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaMapParams {
    /// `groundMaterial` (String)
    #[serde(default)]
    pub ground_material: String,
    /// `wallMaterial` (String)
    #[serde(default)]
    pub wall_material: String,
    /// `groundMaterialFaded` (String)
    #[serde(default)]
    pub ground_material_faded: String,
    /// `wallMaterialFaded` (String)
    #[serde(default)]
    pub wall_material_faded: String,
    /// `doorMaterial` (String)
    #[serde(default)]
    pub door_material: String,
    /// `markerMaterial` (String)
    #[serde(default)]
    pub marker_material: String,
    /// `markerGeomName` (String)
    #[serde(default)]
    pub marker_geom_name: String,
    /// `doorGeomName` (String)
    #[serde(default)]
    pub door_geom_name: String,
    /// `rttGeomMaterial` (String)
    #[serde(default)]
    pub rtt_geom_material: String,
    /// `entityClipName` (String)
    #[serde(default)]
    pub entity_clip_name: String,
    /// `rttoName` (String)
    #[serde(default)]
    pub rtto_name: String,
    /// `outlineMaterial` (String)
    #[serde(default)]
    pub outline_material: String,
    /// `entityComponentRoomTag` (Reference)
    #[serde(default)]
    pub entity_component_room_tag: Option<CigGuid>,
    /// `entityComponentRoomConnectorTag` (Reference)
    #[serde(default)]
    pub entity_component_room_connector_tag: Option<CigGuid>,
    /// `entityComponentRoomGroupTag` (Reference)
    #[serde(default)]
    pub entity_component_room_group_tag: Option<CigGuid>,
    /// `cameraParams` (Class)
    #[serde(default)]
    pub camera_params: Option<Handle<AreaMapCameraParams>>,
    /// `inputParams` (Class)
    #[serde(default)]
    pub input_params: Option<Handle<UI3DDisplayInputParams>>,
    /// `rttGeomDimensions` (Class)
    #[serde(default)]
    pub rtt_geom_dimensions: Option<Handle<Vec2>>,
}

impl Pooled for AreaMapParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_areamap.area_map_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_areamap.area_map_params }
}

impl<'a> Extract<'a> for AreaMapParams {
    const TYPE_NAME: &'static str = "AreaMapParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ground_material: inst.get_str("groundMaterial").map(String::from).unwrap_or_default(),
            wall_material: inst.get_str("wallMaterial").map(String::from).unwrap_or_default(),
            ground_material_faded: inst.get_str("groundMaterialFaded").map(String::from).unwrap_or_default(),
            wall_material_faded: inst.get_str("wallMaterialFaded").map(String::from).unwrap_or_default(),
            door_material: inst.get_str("doorMaterial").map(String::from).unwrap_or_default(),
            marker_material: inst.get_str("markerMaterial").map(String::from).unwrap_or_default(),
            marker_geom_name: inst.get_str("markerGeomName").map(String::from).unwrap_or_default(),
            door_geom_name: inst.get_str("doorGeomName").map(String::from).unwrap_or_default(),
            rtt_geom_material: inst.get_str("rttGeomMaterial").map(String::from).unwrap_or_default(),
            entity_clip_name: inst.get_str("entityClipName").map(String::from).unwrap_or_default(),
            rtto_name: inst.get_str("rttoName").map(String::from).unwrap_or_default(),
            outline_material: inst.get_str("outlineMaterial").map(String::from).unwrap_or_default(),
            entity_component_room_tag: inst.get("entityComponentRoomTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entity_component_room_connector_tag: inst.get("entityComponentRoomConnectorTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entity_component_room_group_tag: inst.get("entityComponentRoomGroupTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            camera_params: match inst.get("cameraParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AreaMapCameraParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            input_params: match inst.get("inputParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UI3DDisplayInputParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtt_geom_dimensions: match inst.get("rttGeomDimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FontSupportParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSupportParams {
    /// `letterMaterial` (String)
    #[serde(default)]
    pub letter_material: String,
    /// `fontParams` (Class)
    #[serde(default)]
    pub font_params: Option<Handle<InnerThought_Config>>,
}

impl Pooled for FontSupportParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_areamap.font_support_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_areamap.font_support_params }
}

impl<'a> Extract<'a> for FontSupportParams {
    const TYPE_NAME: &'static str = "FontSupportParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            letter_material: inst.get_str("letterMaterial").map(String::from).unwrap_or_default(),
            font_params: match inst.get("fontParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InnerThought_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityComponentLocationDataParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityComponentLocationDataParams {
    /// `DisplayMesh` (String)
    #[serde(default)]
    pub display_mesh: String,
}

impl Pooled for SEntityComponentLocationDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_areamap.sentity_component_location_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_areamap.sentity_component_location_data_params }
}

impl<'a> Extract<'a> for SEntityComponentLocationDataParams {
    const TYPE_NAME: &'static str = "SEntityComponentLocationDataParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_mesh: inst.get_str("DisplayMesh").map(String::from).unwrap_or_default(),
        }
    }
}

