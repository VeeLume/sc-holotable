// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `PooledLightData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PooledLightData {
    /// DCB field: `flareName` (String)
    #[serde(default)]
    pub flare_name: String,
    /// DCB field: `flareScale` (Single)
    #[serde(default)]
    pub flare_scale: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `diffuseColor` (Class)
    #[serde(default)]
    pub diffuse_color: Option<Handle<RGB>>,
    /// DCB field: `diffuseMultiplier` (Single)
    #[serde(default)]
    pub diffuse_multiplier: f32,
    /// DCB field: `specularMultiplier` (Single)
    #[serde(default)]
    pub specular_multiplier: f32,
    /// DCB field: `attenuationBulbSize` (Single)
    #[serde(default)]
    pub attenuation_bulb_size: f32,
    /// DCB field: `animSpeed` (Single)
    #[serde(default)]
    pub anim_speed: f32,
    /// DCB field: `rampTime` (Single)
    #[serde(default)]
    pub ramp_time: f32,
    /// DCB field: `fake` (Boolean)
    #[serde(default)]
    pub fake: bool,
    /// DCB field: `autoClip` (Boolean)
    #[serde(default)]
    pub auto_clip: bool,
    /// DCB field: `style` (Byte)
    #[serde(default)]
    pub style: u32,
    /// DCB field: `animPhase` (Byte)
    #[serde(default)]
    pub anim_phase: u32,
    /// DCB field: `flareLensOpticsFrustumAngle` (Byte)
    #[serde(default)]
    pub flare_lens_optics_frustum_angle: u32,
}

impl Pooled for PooledLightData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.pooled_light_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.pooled_light_data }
}

impl<'a> Extract<'a> for PooledLightData {
    const TYPE_NAME: &'static str = "PooledLightData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flare_name: inst.get_str("flareName").map(String::from).unwrap_or_default(),
            flare_scale: inst.get_f32("flareScale").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            diffuse_color: match inst.get("diffuseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            diffuse_multiplier: inst.get_f32("diffuseMultiplier").unwrap_or_default(),
            specular_multiplier: inst.get_f32("specularMultiplier").unwrap_or_default(),
            attenuation_bulb_size: inst.get_f32("attenuationBulbSize").unwrap_or_default(),
            anim_speed: inst.get_f32("animSpeed").unwrap_or_default(),
            ramp_time: inst.get_f32("rampTime").unwrap_or_default(),
            fake: inst.get_bool("fake").unwrap_or_default(),
            auto_clip: inst.get_bool("autoClip").unwrap_or_default(),
            style: inst.get_u32("style").unwrap_or_default(),
            anim_phase: inst.get_u32("animPhase").unwrap_or_default(),
            flare_lens_optics_frustum_angle: inst.get_u32("flareLensOpticsFrustumAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `PoolFilter_NoRef`
///
/// Inherits from: `PoolFilter_Base` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilter_NoRef {
}

impl Pooled for PoolFilter_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.pool_filter_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.pool_filter_no_ref }
}

impl<'a> Extract<'a> for PoolFilter_NoRef {
    const TYPE_NAME: &'static str = "PoolFilter_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PoolFilterRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilterRecord {
    /// DCB field: `filter` (StrongPointer)
    #[serde(default)]
    pub filter: Option<Handle<PoolFilter_NoRef>>,
}

impl Pooled for PoolFilterRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.pool_filter_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.pool_filter_record }
}

impl<'a> Extract<'a> for PoolFilterRecord {
    const TYPE_NAME: &'static str = "PoolFilterRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter: match inst.get("filter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PoolFilter_NoRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PoolFilter_NoRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PointOfInterestData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestData {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `displayInfoText` (Locale)
    #[serde(default)]
    pub display_info_text: String,
    /// DCB field: `imagePath` (String)
    #[serde(default)]
    pub image_path: String,
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec2>>,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for PointOfInterestData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.point_of_interest_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.point_of_interest_data }
}

impl<'a> Extract<'a> for PointOfInterestData {
    const TYPE_NAME: &'static str = "PointOfInterestData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_info_text: inst.get_str("displayInfoText").map(String::from).unwrap_or_default(),
            image_path: inst.get_str("imagePath").map(String::from).unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `PointOfInterestList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestList {
    /// DCB field: `pointsOfInterest` (Class (array))
    #[serde(default)]
    pub points_of_interest: Vec<Handle<PointOfInterestData>>,
}

impl Pooled for PointOfInterestList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.point_of_interest_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.point_of_interest_list }
}

impl<'a> Extract<'a> for PointOfInterestList {
    const TYPE_NAME: &'static str = "PointOfInterestList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            points_of_interest: inst.get_array("pointsOfInterest")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PointOfInterestData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PointOfInterestData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PostureDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureDatabase {
    /// DCB field: `Groups` (StrongPointer (array))
    #[serde(default)]
    pub groups: Vec<Handle<PostureGroup>>,
}

impl Pooled for PostureDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.posture_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.posture_database }
}

impl<'a> Extract<'a> for PostureDatabase {
    const TYPE_NAME: &'static str = "PostureDatabase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("Groups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PostureGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PostureGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PostureGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureGroup {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `Stance` (EnumChoice)
    #[serde(default)]
    pub stance: String,
    /// DCB field: `Postures` (StrongPointer (array))
    #[serde(default)]
    pub postures: Vec<Handle<PostureData>>,
}

impl Pooled for PostureGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.posture_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.posture_group }
}

impl<'a> Extract<'a> for PostureGroup {
    const TYPE_NAME: &'static str = "PostureGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            stance: inst.get_str("Stance").map(String::from).unwrap_or_default(),
            postures: inst.get_array("Postures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PostureData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PostureData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PostureData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureData {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `Priority` (Single)
    #[serde(default)]
    pub priority: f32,
    /// DCB field: `BodyDirection` (EnumChoice)
    #[serde(default)]
    pub body_direction: String,
    /// DCB field: `IsLean` (Boolean)
    #[serde(default)]
    pub is_lean: bool,
    /// DCB field: `AnimationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
}

impl Pooled for PostureData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.posture_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.posture_data }
}

impl<'a> Extract<'a> for PostureData {
    const TYPE_NAME: &'static str = "PostureData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("Priority").unwrap_or_default(),
            body_direction: inst.get_str("BodyDirection").map(String::from).unwrap_or_default(),
            is_lean: inst.get_bool("IsLean").unwrap_or_default(),
            animation_tag: inst.get_str("AnimationTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PopupDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupDef {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `message1` (Locale)
    #[serde(default)]
    pub message1: String,
    /// DCB field: `message2` (Locale)
    #[serde(default)]
    pub message2: String,
    /// DCB field: `message3` (Locale)
    #[serde(default)]
    pub message3: String,
    /// DCB field: `hasCancelButton` (Boolean)
    #[serde(default)]
    pub has_cancel_button: bool,
    /// DCB field: `hasConfirmButton` (Boolean)
    #[serde(default)]
    pub has_confirm_button: bool,
    /// DCB field: `cancelOverrideString` (Locale)
    #[serde(default)]
    pub cancel_override_string: String,
    /// DCB field: `confirmOverrideString` (Locale)
    #[serde(default)]
    pub confirm_override_string: String,
    /// DCB field: `popupFrame` (String)
    #[serde(default)]
    pub popup_frame: String,
    /// DCB field: `popupHeaderFrame` (String)
    #[serde(default)]
    pub popup_header_frame: String,
}

impl Pooled for PopupDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_po.popup_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_po.popup_def }
}

impl<'a> Extract<'a> for PopupDef {
    const TYPE_NAME: &'static str = "PopupDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            message1: inst.get_str("message1").map(String::from).unwrap_or_default(),
            message2: inst.get_str("message2").map(String::from).unwrap_or_default(),
            message3: inst.get_str("message3").map(String::from).unwrap_or_default(),
            has_cancel_button: inst.get_bool("hasCancelButton").unwrap_or_default(),
            has_confirm_button: inst.get_bool("hasConfirmButton").unwrap_or_default(),
            cancel_override_string: inst.get_str("cancelOverrideString").map(String::from).unwrap_or_default(),
            confirm_override_string: inst.get_str("confirmOverrideString").map(String::from).unwrap_or_default(),
            popup_frame: inst.get_str("popupFrame").map(String::from).unwrap_or_default(),
            popup_header_frame: inst.get_str("popupHeaderFrame").map(String::from).unwrap_or_default(),
        }
    }
}

