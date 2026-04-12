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

/// DCB type: `Burst`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Burst {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `minRateOfFire` (Single)
    #[serde(default)]
    pub min_rate_of_fire: f32,
    /// DCB field: `minBurstLength` (Single)
    #[serde(default)]
    pub min_burst_length: f32,
    /// DCB field: `minShotCount` (Single)
    #[serde(default)]
    pub min_shot_count: f32,
    /// DCB field: `gapLengthMultiplier` (Single)
    #[serde(default)]
    pub gap_length_multiplier: f32,
    /// DCB field: `burstVariationMultiplier` (Single)
    #[serde(default)]
    pub burst_variation_multiplier: f32,
}

impl Pooled for Burst {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.burst }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.burst }
}

impl<'a> Extract<'a> for Burst {
    const TYPE_NAME: &'static str = "Burst";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            min_rate_of_fire: inst.get_f32("minRateOfFire").unwrap_or_default(),
            min_burst_length: inst.get_f32("minBurstLength").unwrap_or_default(),
            min_shot_count: inst.get_f32("minShotCount").unwrap_or_default(),
            gap_length_multiplier: inst.get_f32("gapLengthMultiplier").unwrap_or_default(),
            burst_variation_multiplier: inst.get_f32("burstVariationMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuffDurationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffDurationBase {
}

impl Pooled for BuffDurationBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.buff_duration_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.buff_duration_base }
}

impl<'a> Extract<'a> for BuffDurationBase {
    const TYPE_NAME: &'static str = "BuffDurationBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuffValueOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffValueOverride {
    /// DCB field: `valueOverride` (Single)
    #[serde(default)]
    pub value_override: f32,
}

impl Pooled for BuffValueOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.buff_value_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.buff_value_override }
}

impl<'a> Extract<'a> for BuffValueOverride {
    const TYPE_NAME: &'static str = "BuffValueOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value_override: inst.get_f32("valueOverride").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Node`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Node {
}

impl Pooled for BuildingBlocks_Node {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_node }
}

impl<'a> Extract<'a> for BuildingBlocks_Node {
    const TYPE_NAME: &'static str = "BuildingBlocks_Node";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_LayoutPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LayoutPolicyBase {
}

impl Pooled for BuildingBlocks_LayoutPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_layout_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_layout_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_LayoutPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_LayoutPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_LayoutPolicyItemBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LayoutPolicyItemBase {
}

impl Pooled for BuildingBlocks_LayoutPolicyItemBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_layout_policy_item_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_layout_policy_item_base }
}

impl<'a> Extract<'a> for BuildingBlocks_LayoutPolicyItemBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_LayoutPolicyItemBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_ScrollPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ScrollPolicyBase {
}

impl Pooled for BuildingBlocks_ScrollPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_scroll_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_scroll_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ScrollPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ScrollPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_DropTargetPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_DropTargetPolicyBase {
    /// DCB field: `IdVariable` (Class)
    #[serde(default)]
    pub id_variable: Option<Handle<BuildingBlocks_BindingsVariableInput>>,
}

impl Pooled for BuildingBlocks_DropTargetPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_drop_target_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_drop_target_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_DropTargetPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_DropTargetPolicyBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id_variable: match inst.get("IdVariable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BindingsVariableInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BindingsVariableInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_DraggablePolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_DraggablePolicyBase {
    /// DCB field: `onDragStart` (StrongPointer (array))
    #[serde(default)]
    pub on_drag_start: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onDragEnd` (StrongPointer (array))
    #[serde(default)]
    pub on_drag_end: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `dragStartSoundTag` (Reference)
    #[serde(default)]
    pub drag_start_sound_tag: Option<CigGuid>,
    /// DCB field: `dragEndSoundTag` (Reference)
    #[serde(default)]
    pub drag_end_sound_tag: Option<CigGuid>,
    /// DCB field: `cursorSpeedSoundTag` (Reference)
    #[serde(default)]
    pub cursor_speed_sound_tag: Option<CigGuid>,
    /// DCB field: `dragXPosURL` (String)
    #[serde(default)]
    pub drag_xpos_url: String,
    /// DCB field: `dragYPosURL` (String)
    #[serde(default)]
    pub drag_ypos_url: String,
}

impl Pooled for BuildingBlocks_DraggablePolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_draggable_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_draggable_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_DraggablePolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_DraggablePolicyBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_drag_start: inst.get_array("onDragStart")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_drag_end: inst.get_array("onDragEnd")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            drag_start_sound_tag: inst.get("dragStartSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            drag_end_sound_tag: inst.get("dragEndSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            cursor_speed_sound_tag: inst.get("cursorSpeedSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            drag_xpos_url: inst.get_str("dragXPosURL").map(String::from).unwrap_or_default(),
            drag_ypos_url: inst.get_str("dragYPosURL").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsVariableInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsVariableInput {
    /// DCB field: `binding` (String)
    #[serde(default)]
    pub binding: String,
}

impl Pooled for BuildingBlocks_BindingsVariableInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_bindings_variable_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_bindings_variable_input }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsVariableInput {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsVariableInput";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            binding: inst.get_str("binding").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FixedOrRelativeValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FixedOrRelativeValue {
    /// DCB field: `value` (Single)
    #[serde(default)]
    pub value: f32,
    /// DCB field: `behavior` (EnumChoice)
    #[serde(default)]
    pub behavior: String,
}

impl Pooled for BuildingBlocks_FixedOrRelativeValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_fixed_or_relative_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_fixed_or_relative_value }
}

impl<'a> Extract<'a> for BuildingBlocks_FixedOrRelativeValue {
    const TYPE_NAME: &'static str = "BuildingBlocks_FixedOrRelativeValue";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_f32("value").unwrap_or_default(),
            behavior: inst.get_str("behavior").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TRBL`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TRBL {
    /// DCB field: `top` (Single)
    #[serde(default)]
    pub top: f32,
    /// DCB field: `right` (Single)
    #[serde(default)]
    pub right: f32,
    /// DCB field: `bottom` (Single)
    #[serde(default)]
    pub bottom: f32,
    /// DCB field: `left` (Single)
    #[serde(default)]
    pub left: f32,
    /// DCB field: `front` (Single)
    #[serde(default)]
    pub front: f32,
    /// DCB field: `back` (Single)
    #[serde(default)]
    pub back: f32,
}

impl Pooled for BuildingBlocks_TRBL {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_trbl }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_trbl }
}

impl<'a> Extract<'a> for BuildingBlocks_TRBL {
    const TYPE_NAME: &'static str = "BuildingBlocks_TRBL";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            top: inst.get_f32("top").unwrap_or_default(),
            right: inst.get_f32("right").unwrap_or_default(),
            bottom: inst.get_f32("bottom").unwrap_or_default(),
            left: inst.get_f32("left").unwrap_or_default(),
            front: inst.get_f32("front").unwrap_or_default(),
            back: inst.get_f32("back").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Size`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Size {
    /// DCB field: `width` (Class)
    #[serde(default)]
    pub width: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `height` (Class)
    #[serde(default)]
    pub height: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `depth` (Class)
    #[serde(default)]
    pub depth: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `minWidth` (Class)
    #[serde(default)]
    pub min_width: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `minHeight` (Class)
    #[serde(default)]
    pub min_height: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `maxWidth` (Class)
    #[serde(default)]
    pub max_width: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `maxHeight` (Class)
    #[serde(default)]
    pub max_height: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `enableMinWidth` (Boolean)
    #[serde(default)]
    pub enable_min_width: bool,
    /// DCB field: `enableMinHeight` (Boolean)
    #[serde(default)]
    pub enable_min_height: bool,
    /// DCB field: `enableMaxWidth` (Boolean)
    #[serde(default)]
    pub enable_max_width: bool,
    /// DCB field: `enableMaxHeight` (Boolean)
    #[serde(default)]
    pub enable_max_height: bool,
}

impl Pooled for BuildingBlocks_Size {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_size }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_size }
}

impl<'a> Extract<'a> for BuildingBlocks_Size {
    const TYPE_NAME: &'static str = "BuildingBlocks_Size";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            width: match inst.get("width") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            height: match inst.get("height") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            depth: match inst.get("depth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_width: match inst.get("minWidth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_height: match inst.get("minHeight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_width: match inst.get("maxWidth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_height: match inst.get("maxHeight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_min_width: inst.get_bool("enableMinWidth").unwrap_or_default(),
            enable_min_height: inst.get_bool("enableMinHeight").unwrap_or_default(),
            enable_max_width: inst.get_bool("enableMaxWidth").unwrap_or_default(),
            enable_max_height: inst.get_bool("enableMaxHeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_BorderSide`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BorderSide {
    /// DCB field: `width` (Single)
    #[serde(default)]
    pub width: f32,
    /// DCB field: `color` (StrongPointer)
    #[serde(default)]
    pub color: Option<Handle<BuildingBlocks_ColorBase>>,
}

impl Pooled for BuildingBlocks_BorderSide {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_border_side }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_border_side }
}

impl<'a> Extract<'a> for BuildingBlocks_BorderSide {
    const TYPE_NAME: &'static str = "BuildingBlocks_BorderSide";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            width: inst.get_f32("width").unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_BorderRadiusCorner`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BorderRadiusCorner {
    /// DCB field: `radius` (Class)
    #[serde(default)]
    pub radius: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `chamfer` (Boolean)
    #[serde(default)]
    pub chamfer: bool,
}

impl Pooled for BuildingBlocks_BorderRadiusCorner {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_border_radius_corner }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_border_radius_corner }
}

impl<'a> Extract<'a> for BuildingBlocks_BorderRadiusCorner {
    const TYPE_NAME: &'static str = "BuildingBlocks_BorderRadiusCorner";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            radius: match inst.get("radius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chamfer: inst.get_bool("chamfer").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Border`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Border {
    /// DCB field: `top` (Class)
    #[serde(default)]
    pub top: Option<Handle<BuildingBlocks_BorderSide>>,
    /// DCB field: `right` (Class)
    #[serde(default)]
    pub right: Option<Handle<BuildingBlocks_BorderSide>>,
    /// DCB field: `bottom` (Class)
    #[serde(default)]
    pub bottom: Option<Handle<BuildingBlocks_BorderSide>>,
    /// DCB field: `left` (Class)
    #[serde(default)]
    pub left: Option<Handle<BuildingBlocks_BorderSide>>,
    /// DCB field: `topLeftRadius` (Class)
    #[serde(default)]
    pub top_left_radius: Option<Handle<BuildingBlocks_BorderRadiusCorner>>,
    /// DCB field: `topRightRadius` (Class)
    #[serde(default)]
    pub top_right_radius: Option<Handle<BuildingBlocks_BorderRadiusCorner>>,
    /// DCB field: `bottomLeftRadius` (Class)
    #[serde(default)]
    pub bottom_left_radius: Option<Handle<BuildingBlocks_BorderRadiusCorner>>,
    /// DCB field: `bottomRightRadius` (Class)
    #[serde(default)]
    pub bottom_right_radius: Option<Handle<BuildingBlocks_BorderRadiusCorner>>,
}

impl Pooled for BuildingBlocks_Border {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_border }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_border }
}

impl<'a> Extract<'a> for BuildingBlocks_Border {
    const TYPE_NAME: &'static str = "BuildingBlocks_Border";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            top: match inst.get("top") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            right: match inst.get("right") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom: match inst.get("bottom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left: match inst.get("left") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderSide>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_left_radius: match inst.get("topLeftRadius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_right_radius: match inst.get("topRightRadius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_left_radius: match inst.get("bottomLeftRadius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_right_radius: match inst.get("bottomRightRadius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_BorderRadiusCorner>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_Background`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Background {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `color` (StrongPointer)
    #[serde(default)]
    pub color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `topLeftColor` (StrongPointer)
    #[serde(default)]
    pub top_left_color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `topRightColor` (StrongPointer)
    #[serde(default)]
    pub top_right_color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `bottomLeftColor` (StrongPointer)
    #[serde(default)]
    pub bottom_left_color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `bottomRightCColor` (StrongPointer)
    #[serde(default)]
    pub bottom_right_ccolor: Option<Handle<BuildingBlocks_ColorBase>>,
}

impl Pooled for BuildingBlocks_Background {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_background }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_background }
}

impl<'a> Extract<'a> for BuildingBlocks_Background {
    const TYPE_NAME: &'static str = "BuildingBlocks_Background";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_left_color: match inst.get("topLeftColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_right_color: match inst.get("topRightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_left_color: match inst.get("bottomLeftColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_right_ccolor: match inst.get("bottomRightCColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_LayoutItemCommon`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LayoutItemCommon {
    /// DCB field: `order` (Int64)
    #[serde(default)]
    pub order: i64,
    /// DCB field: `transitionDuration` (Single)
    #[serde(default)]
    pub transition_duration: f32,
    /// DCB field: `transitionDelay` (Single)
    #[serde(default)]
    pub transition_delay: f32,
    /// DCB field: `timingFunction` (StrongPointer)
    #[serde(default)]
    pub timing_function: Option<Handle<BuildingBlocks_TimingFunctionBase>>,
}

impl Pooled for BuildingBlocks_LayoutItemCommon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_layout_item_common }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_layout_item_common }
}

impl<'a> Extract<'a> for BuildingBlocks_LayoutItemCommon {
    const TYPE_NAME: &'static str = "BuildingBlocks_LayoutItemCommon";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            order: inst.get_i64("order").unwrap_or_default(),
            transition_duration: inst.get_f32("transitionDuration").unwrap_or_default(),
            transition_delay: inst.get_f32("transitionDelay").unwrap_or_default(),
            timing_function: match inst.get("timingFunction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_SegmentedFill`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_SegmentedFill {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `segmentSize` (Class)
    #[serde(default)]
    pub segment_size: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `spaceSize` (Class)
    #[serde(default)]
    pub space_size: Option<Handle<BuildingBlocks_FixedOrRelativeValue>>,
    /// DCB field: `angle` (Single)
    #[serde(default)]
    pub angle: f32,
    /// DCB field: `xOffset` (Single)
    #[serde(default)]
    pub x_offset: f32,
    /// DCB field: `segmentColor` (StrongPointer)
    #[serde(default)]
    pub segment_color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `barFill` (Boolean)
    #[serde(default)]
    pub bar_fill: bool,
}

impl Pooled for BuildingBlocks_SegmentedFill {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_segmented_fill }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_segmented_fill }
}

impl<'a> Extract<'a> for BuildingBlocks_SegmentedFill {
    const TYPE_NAME: &'static str = "BuildingBlocks_SegmentedFill";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            segment_size: match inst.get("segmentSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            space_size: match inst.get("spaceSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FixedOrRelativeValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle: inst.get_f32("angle").unwrap_or_default(),
            x_offset: inst.get_f32("xOffset").unwrap_or_default(),
            segment_color: match inst.get("segmentColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bar_fill: inst.get_bool("barFill").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_SvgFill`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_SvgFill {
    /// DCB field: `svgPath` (String)
    #[serde(default)]
    pub svg_path: String,
    /// DCB field: `nineSliceRect` (Class)
    #[serde(default)]
    pub nine_slice_rect: Option<Handle<BuildingBlocks_TRBL>>,
    /// DCB field: `renderShape` (Boolean)
    #[serde(default)]
    pub render_shape: bool,
    /// DCB field: `flipHorizontal` (Boolean)
    #[serde(default)]
    pub flip_horizontal: bool,
    /// DCB field: `flipVertical` (Boolean)
    #[serde(default)]
    pub flip_vertical: bool,
    /// DCB field: `enableColorOverlay` (Boolean)
    #[serde(default)]
    pub enable_color_overlay: bool,
    /// DCB field: `color` (StrongPointer)
    #[serde(default)]
    pub color: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `scalingBehavior` (EnumChoice)
    #[serde(default)]
    pub scaling_behavior: String,
    /// DCB field: `containPositionX` (Single)
    #[serde(default)]
    pub contain_position_x: f32,
    /// DCB field: `containPositionY` (Single)
    #[serde(default)]
    pub contain_position_y: f32,
    /// DCB field: `enableNineSliceRect` (Boolean)
    #[serde(default)]
    pub enable_nine_slice_rect: bool,
    /// DCB field: `nineSliceScale` (Single)
    #[serde(default)]
    pub nine_slice_scale: f32,
    /// DCB field: `strokeExtent` (Single)
    #[serde(default)]
    pub stroke_extent: f32,
    /// DCB field: `playhead` (Single)
    #[serde(default)]
    pub playhead: f32,
}

impl Pooled for BuildingBlocks_SvgFill {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_svg_fill }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_svg_fill }
}

impl<'a> Extract<'a> for BuildingBlocks_SvgFill {
    const TYPE_NAME: &'static str = "BuildingBlocks_SvgFill";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            svg_path: inst.get_str("svgPath").map(String::from).unwrap_or_default(),
            nine_slice_rect: match inst.get("nineSliceRect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_shape: inst.get_bool("renderShape").unwrap_or_default(),
            flip_horizontal: inst.get_bool("flipHorizontal").unwrap_or_default(),
            flip_vertical: inst.get_bool("flipVertical").unwrap_or_default(),
            enable_color_overlay: inst.get_bool("enableColorOverlay").unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scaling_behavior: inst.get_str("scalingBehavior").map(String::from).unwrap_or_default(),
            contain_position_x: inst.get_f32("containPositionX").unwrap_or_default(),
            contain_position_y: inst.get_f32("containPositionY").unwrap_or_default(),
            enable_nine_slice_rect: inst.get_bool("enableNineSliceRect").unwrap_or_default(),
            nine_slice_scale: inst.get_f32("nineSliceScale").unwrap_or_default(),
            stroke_extent: inst.get_f32("strokeExtent").unwrap_or_default(),
            playhead: inst.get_f32("playhead").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Overflow`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Overflow {
    /// DCB field: `overflow` (EnumChoice)
    #[serde(default)]
    pub overflow: String,
    /// DCB field: `widthFadeThreshold` (Single)
    #[serde(default)]
    pub width_fade_threshold: f32,
    /// DCB field: `heightFadeThreshold` (Single)
    #[serde(default)]
    pub height_fade_threshold: f32,
    /// DCB field: `depthFadeThreshold` (Single)
    #[serde(default)]
    pub depth_fade_threshold: f32,
    /// DCB field: `fadeXAxis` (Boolean)
    #[serde(default)]
    pub fade_xaxis: bool,
    /// DCB field: `fadeYAxis` (Boolean)
    #[serde(default)]
    pub fade_yaxis: bool,
    /// DCB field: `fadeZAxis` (Boolean)
    #[serde(default)]
    pub fade_zaxis: bool,
}

impl Pooled for BuildingBlocks_Overflow {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_overflow }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_overflow }
}

impl<'a> Extract<'a> for BuildingBlocks_Overflow {
    const TYPE_NAME: &'static str = "BuildingBlocks_Overflow";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            overflow: inst.get_str("overflow").map(String::from).unwrap_or_default(),
            width_fade_threshold: inst.get_f32("widthFadeThreshold").unwrap_or_default(),
            height_fade_threshold: inst.get_f32("heightFadeThreshold").unwrap_or_default(),
            depth_fade_threshold: inst.get_f32("depthFadeThreshold").unwrap_or_default(),
            fade_xaxis: inst.get_bool("fadeXAxis").unwrap_or_default(),
            fade_yaxis: inst.get_bool("fadeYAxis").unwrap_or_default(),
            fade_zaxis: inst.get_bool("fadeZAxis").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_RadialTransform`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_RadialTransform {
    /// DCB field: `transformMultiplier` (Single)
    #[serde(default)]
    pub transform_multiplier: f32,
    /// DCB field: `curvatureAxis` (EnumChoice)
    #[serde(default)]
    pub curvature_axis: String,
    /// DCB field: `mouseTestShape` (Boolean)
    #[serde(default)]
    pub mouse_test_shape: bool,
    /// DCB field: `polyResolution` (Byte)
    #[serde(default)]
    pub poly_resolution: u32,
    /// DCB field: `conicSlantFactor` (Single)
    #[serde(default)]
    pub conic_slant_factor: f32,
}

impl Pooled for BuildingBlocks_RadialTransform {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_radial_transform }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_radial_transform }
}

impl<'a> Extract<'a> for BuildingBlocks_RadialTransform {
    const TYPE_NAME: &'static str = "BuildingBlocks_RadialTransform";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            transform_multiplier: inst.get_f32("transformMultiplier").unwrap_or_default(),
            curvature_axis: inst.get_str("curvatureAxis").map(String::from).unwrap_or_default(),
            mouse_test_shape: inst.get_bool("mouseTestShape").unwrap_or_default(),
            poly_resolution: inst.get_u32("polyResolution").unwrap_or_default(),
            conic_slant_factor: inst.get_f32("conicSlantFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_RadialTransformChild`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_RadialTransformChild {
    /// DCB field: `inheritRotation` (Boolean)
    #[serde(default)]
    pub inherit_rotation: bool,
    /// DCB field: `inheritShapeWarp` (Boolean)
    #[serde(default)]
    pub inherit_shape_warp: bool,
    /// DCB field: `maintainGapLength` (Boolean)
    #[serde(default)]
    pub maintain_gap_length: bool,
}

impl Pooled for BuildingBlocks_RadialTransformChild {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_radial_transform_child }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_radial_transform_child }
}

impl<'a> Extract<'a> for BuildingBlocks_RadialTransformChild {
    const TYPE_NAME: &'static str = "BuildingBlocks_RadialTransformChild";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            inherit_rotation: inst.get_bool("inheritRotation").unwrap_or_default(),
            inherit_shape_warp: inst.get_bool("inheritShapeWarp").unwrap_or_default(),
            maintain_gap_length: inst.get_bool("maintainGapLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Animation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Animation {
    /// DCB field: `animationTimeline` (StrongPointer)
    #[serde(default)]
    pub animation_timeline: Option<Handle<BuildingBlocks_TimelineTypeBase>>,
    /// DCB field: `dynamicStartTimeVariableName` (String)
    #[serde(default)]
    pub dynamic_start_time_variable_name: String,
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
    /// DCB field: `delayRandomRange` (Single)
    #[serde(default)]
    pub delay_random_range: f32,
    /// DCB field: `direction` (EnumChoice)
    #[serde(default)]
    pub direction: String,
    /// DCB field: `fillMode` (EnumChoice)
    #[serde(default)]
    pub fill_mode: String,
    /// DCB field: `iterationCount` (Byte)
    #[serde(default)]
    pub iteration_count: u32,
    /// DCB field: `loopIndefinitely` (Boolean)
    #[serde(default)]
    pub loop_indefinitely: bool,
    /// DCB field: `timeSync` (Boolean)
    #[serde(default)]
    pub time_sync: bool,
    /// DCB field: `additive` (Boolean)
    #[serde(default)]
    pub additive: bool,
    /// DCB field: `timescaleIndependent` (Boolean)
    #[serde(default)]
    pub timescale_independent: bool,
}

impl Pooled for BuildingBlocks_Animation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_animation }
}

impl<'a> Extract<'a> for BuildingBlocks_Animation {
    const TYPE_NAME: &'static str = "BuildingBlocks_Animation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_timeline: match inst.get("animationTimeline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dynamic_start_time_variable_name: inst.get_str("dynamicStartTimeVariableName").map(String::from).unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
            delay_random_range: inst.get_f32("delayRandomRange").unwrap_or_default(),
            direction: inst.get_str("direction").map(String::from).unwrap_or_default(),
            fill_mode: inst.get_str("fillMode").map(String::from).unwrap_or_default(),
            iteration_count: inst.get_u32("iterationCount").unwrap_or_default(),
            loop_indefinitely: inst.get_bool("loopIndefinitely").unwrap_or_default(),
            time_sync: inst.get_bool("timeSync").unwrap_or_default(),
            additive: inst.get_bool("additive").unwrap_or_default(),
            timescale_independent: inst.get_bool("timescaleIndependent").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_PreviewScreenBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PreviewScreenBase {
    /// DCB field: `renderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
}

impl Pooled for BuildingBlocks_PreviewScreenBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_preview_screen_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_preview_screen_base }
}

impl<'a> Extract<'a> for BuildingBlocks_PreviewScreenBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_PreviewScreenBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            render_layer: inst.get_str("renderLayer").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_PreviewSceneEntityRoot`
///
/// Inherits from: `BuildingBlocks_PreviewScreenBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PreviewSceneEntityRoot {
    /// DCB field: `renderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
}

impl Pooled for BuildingBlocks_PreviewSceneEntityRoot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_preview_scene_entity_root }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_preview_scene_entity_root }
}

impl<'a> Extract<'a> for BuildingBlocks_PreviewSceneEntityRoot {
    const TYPE_NAME: &'static str = "BuildingBlocks_PreviewSceneEntityRoot";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            render_layer: inst.get_str("renderLayer").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_PreviewSceneRttRoot`
///
/// Inherits from: `BuildingBlocks_PreviewScreenBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PreviewSceneRttRoot {
    /// DCB field: `renderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
    /// DCB field: `entityScale` (Single)
    #[serde(default)]
    pub entity_scale: f32,
    /// DCB field: `entityOrientation` (Class)
    #[serde(default)]
    pub entity_orientation: Option<Handle<Deg3>>,
}

impl Pooled for BuildingBlocks_PreviewSceneRttRoot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_preview_scene_rtt_root }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_preview_scene_rtt_root }
}

impl<'a> Extract<'a> for BuildingBlocks_PreviewSceneRttRoot {
    const TYPE_NAME: &'static str = "BuildingBlocks_PreviewSceneRttRoot";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            render_layer: inst.get_str("renderLayer").map(String::from).unwrap_or_default(),
            entity_scale: inst.get_f32("entityScale").unwrap_or_default(),
            entity_orientation: match inst.get("entityOrientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_PreviewSceneAugmentedRealityRtt`
///
/// Inherits from: `BuildingBlocks_PreviewScreenWorldRoot` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PreviewSceneAugmentedRealityRtt {
    /// DCB field: `renderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
    /// DCB field: `cardsUseStageRadius` (Boolean)
    #[serde(default)]
    pub cards_use_stage_radius: bool,
}

impl Pooled for BuildingBlocks_PreviewSceneAugmentedRealityRtt {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_preview_scene_augmented_reality_rtt }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_preview_scene_augmented_reality_rtt }
}

impl<'a> Extract<'a> for BuildingBlocks_PreviewSceneAugmentedRealityRtt {
    const TYPE_NAME: &'static str = "BuildingBlocks_PreviewSceneAugmentedRealityRtt";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            render_layer: inst.get_str("renderLayer").map(String::from).unwrap_or_default(),
            cards_use_stage_radius: inst.get_bool("cardsUseStageRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_PrimitiveVisualState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PrimitiveVisualState {
    /// DCB field: `interference` (Single)
    #[serde(default)]
    pub interference: f32,
}

impl Pooled for BuildingBlocks_PrimitiveVisualState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_primitive_visual_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_primitive_visual_state }
}

impl<'a> Extract<'a> for BuildingBlocks_PrimitiveVisualState {
    const TYPE_NAME: &'static str = "BuildingBlocks_PrimitiveVisualState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interference: inst.get_f32("interference").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_PrimitiveSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_PrimitiveSettings {
    /// DCB field: `primitiveMaterialPath` (String)
    #[serde(default)]
    pub primitive_material_path: String,
    /// DCB field: `UVStart` (Class)
    #[serde(default)]
    pub uvstart: Option<Handle<Vec2>>,
    /// DCB field: `UVSize` (Class)
    #[serde(default)]
    pub uvsize: Option<Handle<Vec2>>,
    /// DCB field: `isUValuePerPixel` (Boolean)
    #[serde(default)]
    pub is_uvalue_per_pixel: bool,
    /// DCB field: `isVValuePerPixel` (Boolean)
    #[serde(default)]
    pub is_vvalue_per_pixel: bool,
    /// DCB field: `isGrouped` (Boolean)
    #[serde(default)]
    pub is_grouped: bool,
    /// DCB field: `visualState` (StrongPointer)
    #[serde(default)]
    pub visual_state: Option<Handle<BuildingBlocks_PrimitiveVisualState>>,
}

impl Pooled for BuildingBlocks_PrimitiveSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_primitive_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_primitive_settings }
}

impl<'a> Extract<'a> for BuildingBlocks_PrimitiveSettings {
    const TYPE_NAME: &'static str = "BuildingBlocks_PrimitiveSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            primitive_material_path: inst.get_str("primitiveMaterialPath").map(String::from).unwrap_or_default(),
            uvstart: match inst.get("UVStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            uvsize: match inst.get("UVSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_uvalue_per_pixel: inst.get_bool("isUValuePerPixel").unwrap_or_default(),
            is_vvalue_per_pixel: inst.get_bool("isVValuePerPixel").unwrap_or_default(),
            is_grouped: inst.get_bool("isGrouped").unwrap_or_default(),
            visual_state: match inst.get("visualState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PrimitiveVisualState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PrimitiveVisualState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_CalloutSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_CalloutSettings {
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
    /// DCB field: `activationVariableURL` (String)
    #[serde(default)]
    pub activation_variable_url: String,
}

impl Pooled for BuildingBlocks_CalloutSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_callout_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_callout_settings }
}

impl<'a> Extract<'a> for BuildingBlocks_CalloutSettings {
    const TYPE_NAME: &'static str = "BuildingBlocks_CalloutSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            activation_variable_url: inst.get_str("activationVariableURL").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_VirtualCursorPolicy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_VirtualCursorPolicy {
    /// DCB field: `hoverFrictionEnabled` (Boolean)
    #[serde(default)]
    pub hover_friction_enabled: bool,
    /// DCB field: `hoverStateEnabled` (Boolean)
    #[serde(default)]
    pub hover_state_enabled: bool,
}

impl Pooled for BuildingBlocks_VirtualCursorPolicy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_virtual_cursor_policy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_virtual_cursor_policy }
}

impl<'a> Extract<'a> for BuildingBlocks_VirtualCursorPolicy {
    const TYPE_NAME: &'static str = "BuildingBlocks_VirtualCursorPolicy";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hover_friction_enabled: inst.get_bool("hoverFrictionEnabled").unwrap_or_default(),
            hover_state_enabled: inst.get_bool("hoverStateEnabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_WidgetBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `styleTags` (Reference (array))
    #[serde(default)]
    pub style_tags: Vec<CigGuid>,
    /// DCB field: `rendererType` (EnumChoice)
    #[serde(default)]
    pub renderer_type: String,
    /// DCB field: `rendererPolicy` (StrongPointer)
    #[serde(default)]
    pub renderer_policy: Option<Handle<BuildingBlocks_RendererPolicyBase>>,
    /// DCB field: `primitiveSettings` (Class)
    #[serde(default)]
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// DCB field: `parent` (WeakPointer)
    #[serde(default)]
    pub parent: Option<Handle<BuildingBlocks_WidgetBase>>,
    /// DCB field: `previewScene` (WeakPointer)
    #[serde(default)]
    pub preview_scene: Option<Handle<BuildingBlocks_PreviewScreenBase>>,
    /// DCB field: `previewSceneFlattened` (WeakPointer)
    #[serde(default)]
    pub preview_scene_flattened: Option<Handle<BuildingBlocks_PreviewScreenBase>>,
    /// DCB field: `cullingLevel` (EnumChoice)
    #[serde(default)]
    pub culling_level: String,
    /// DCB field: `isActive` (Boolean)
    #[serde(default)]
    pub is_active: bool,
    /// DCB field: `affectsLayout` (Boolean)
    #[serde(default)]
    pub affects_layout: bool,
    /// DCB field: `affectsAutosize` (Boolean)
    #[serde(default)]
    pub affects_autosize: bool,
    /// DCB field: `exportNode` (Boolean)
    #[serde(default)]
    pub export_node: bool,
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// DCB field: `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<Deg3>>,
    /// DCB field: `orientationOffset` (Class)
    #[serde(default)]
    pub orientation_offset: Option<Handle<Deg3>>,
    /// DCB field: `scale` (Class)
    #[serde(default)]
    pub scale: Option<Handle<Vec3>>,
    /// DCB field: `sizing` (Class)
    #[serde(default)]
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// DCB field: `autoScalingMethod` (EnumChoice)
    #[serde(default)]
    pub auto_scaling_method: String,
    /// DCB field: `padding` (Class)
    #[serde(default)]
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// DCB field: `margin` (Class)
    #[serde(default)]
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// DCB field: `pivot` (Class)
    #[serde(default)]
    pub pivot: Option<Handle<Vec3>>,
    /// DCB field: `anchor` (Class)
    #[serde(default)]
    pub anchor: Option<Handle<Vec3>>,
    /// DCB field: `background` (Class)
    #[serde(default)]
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// DCB field: `segmentedFill` (Class)
    #[serde(default)]
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// DCB field: `svgFill` (Class)
    #[serde(default)]
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// DCB field: `border` (Class)
    #[serde(default)]
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// DCB field: `layoutPolicy` (StrongPointer)
    #[serde(default)]
    pub layout_policy: Option<Handle<BuildingBlocks_LayoutPolicyBase>>,
    /// DCB field: `layoutPolicyItem` (StrongPointer)
    #[serde(default)]
    pub layout_policy_item: Option<Handle<BuildingBlocks_LayoutPolicyItemBase>>,
    /// DCB field: `layoutItemCommon` (StrongPointer)
    #[serde(default)]
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// DCB field: `dropTargetPolicy` (StrongPointer)
    #[serde(default)]
    pub drop_target_policy: Option<Handle<BuildingBlocks_DropTargetPolicyBase>>,
    /// DCB field: `draggablePolicy` (StrongPointer)
    #[serde(default)]
    pub draggable_policy: Option<Handle<BuildingBlocks_DraggablePolicyBase>>,
    /// DCB field: `tooltipPolicy` (StrongPointer)
    #[serde(default)]
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// DCB field: `contextMenuPolicy` (StrongPointer)
    #[serde(default)]
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// DCB field: `grabControlsPolicy` (StrongPointer)
    #[serde(default)]
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// DCB field: `calloutSettings` (StrongPointer)
    #[serde(default)]
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// DCB field: `virtualCursorPolicy` (StrongPointer)
    #[serde(default)]
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// DCB field: `overflow` (Class)
    #[serde(default)]
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// DCB field: `scrollPolicy` (StrongPointer)
    #[serde(default)]
    pub scroll_policy: Option<Handle<BuildingBlocks_ScrollPolicyBase>>,
    /// DCB field: `radialTransform` (Class)
    #[serde(default)]
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// DCB field: `radialTransformChild` (Class)
    #[serde(default)]
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// DCB field: `animation` (Class)
    #[serde(default)]
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// DCB field: `interactions` (Class)
    #[serde(default)]
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// DCB field: `inheritsScale` (Boolean)
    #[serde(default)]
    pub inherits_scale: bool,
    /// DCB field: `inheritsRotation` (Boolean)
    #[serde(default)]
    pub inherits_rotation: bool,
    /// DCB field: `inheritsTranslation` (Boolean)
    #[serde(default)]
    pub inherits_translation: bool,
    /// DCB field: `inheritsAlpha` (Boolean)
    #[serde(default)]
    pub inherits_alpha: bool,
    /// DCB field: `inheritsOverflow` (Boolean)
    #[serde(default)]
    pub inherits_overflow: bool,
    /// DCB field: `alpha` (Single)
    #[serde(default)]
    pub alpha: f32,
    /// DCB field: `layer` (Byte)
    #[serde(default)]
    pub layer: u32,
    /// DCB field: `aspectRatioLibrary` (Reference)
    #[serde(default)]
    pub aspect_ratio_library: Option<CigGuid>,
    /// DCB field: `focusIndex` (Int16)
    #[serde(default)]
    pub focus_index: i32,
    /// DCB field: `inlineStyles` (Class (array))
    #[serde(default)]
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// DCB field: `hoverCursor` (EnumChoice)
    #[serde(default)]
    pub hover_cursor: String,
    /// DCB field: `enableHeldCursor` (Boolean)
    #[serde(default)]
    pub enable_held_cursor: bool,
    /// DCB field: `heldCursor` (EnumChoice)
    #[serde(default)]
    pub held_cursor: String,
}

impl Pooled for BuildingBlocks_WidgetBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_widget_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_widget_base }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst.get_array("styleTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            renderer_type: inst.get_str("rendererType").map(String::from).unwrap_or_default(),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_RendererPolicyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_RendererPolicyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            culling_level: inst.get_str("cullingLevel").map(String::from).unwrap_or_default(),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Size>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Size>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_scaling_method: inst.get_str("autoScalingMethod").map(String::from).unwrap_or_default(),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TRBL>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Background>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Background>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_SvgFill>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_SvgFill>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Border>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Border>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_LayoutPolicyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_LayoutPolicyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_LayoutPolicyItemBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_LayoutPolicyItemBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_DropTargetPolicyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_DropTargetPolicyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_DraggablePolicyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_DraggablePolicyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Overflow>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Overflow>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ScrollPolicyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ScrollPolicyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Animation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Animation>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Interactions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Interactions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst.get("aspectRatioLibrary").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst.get_array("inlineStyles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hover_cursor: inst.get_str("hoverCursor").map(String::from).unwrap_or_default(),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: inst.get_str("heldCursor").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FieldModifierBase {
}

impl Pooled for BuildingBlocks_FieldModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_field_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_field_modifier_base }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_RendererPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_RendererPolicyBase {
}

impl Pooled for BuildingBlocks_RendererPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_renderer_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_renderer_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_RendererPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_RendererPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TimingFunctionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimingFunctionBase {
}

impl Pooled for BuildingBlocks_TimingFunctionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_timing_function_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_timing_function_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TimingFunctionBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimingFunctionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_FieldTransitionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FieldTransitionBase {
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
    /// DCB field: `delayRandomRange` (Single)
    #[serde(default)]
    pub delay_random_range: f32,
    /// DCB field: `timescaleIndependent` (Boolean)
    #[serde(default)]
    pub timescale_independent: bool,
    /// DCB field: `timingFunction` (StrongPointer)
    #[serde(default)]
    pub timing_function: Option<Handle<BuildingBlocks_TimingFunctionBase>>,
}

impl Pooled for BuildingBlocks_FieldTransitionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_field_transition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_field_transition_base }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldTransitionBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldTransitionBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
            delay_random_range: inst.get_f32("delayRandomRange").unwrap_or_default(),
            timescale_independent: inst.get_bool("timescaleIndependent").unwrap_or_default(),
            timing_function: match inst.get("timingFunction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_TimelineTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimelineTypeBase {
}

impl Pooled for BuildingBlocks_TimelineTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_timeline_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_timeline_type_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TimelineTypeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimelineTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TimelineTypeEmbedded`
///
/// Inherits from: `BuildingBlocks_TimelineTypeBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimelineTypeEmbedded {
    /// DCB field: `keyframes` (Class (array))
    #[serde(default)]
    pub keyframes: Vec<Handle<BuildingBlocks_Keyframe>>,
}

impl Pooled for BuildingBlocks_TimelineTypeEmbedded {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_timeline_type_embedded }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_timeline_type_embedded }
}

impl<'a> Extract<'a> for BuildingBlocks_TimelineTypeEmbedded {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimelineTypeEmbedded";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            keyframes: inst.get_array("keyframes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_Keyframe>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_Keyframe>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Timeline`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Timeline {
    /// DCB field: `timeline` (Class)
    #[serde(default)]
    pub timeline: Option<Handle<BuildingBlocks_TimelineTypeEmbedded>>,
}

impl Pooled for BuildingBlocks_Timeline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_timeline }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_timeline }
}

impl<'a> Extract<'a> for BuildingBlocks_Timeline {
    const TYPE_NAME: &'static str = "BuildingBlocks_Timeline";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timeline: match inst.get("timeline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeEmbedded>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeEmbedded>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_Keyframe`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Keyframe {
    /// DCB field: `percent` (Single)
    #[serde(default)]
    pub percent: f32,
    /// DCB field: `modifiers` (Class (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BuildingBlocks_KeyframeModifierData>>,
}

impl Pooled for BuildingBlocks_Keyframe {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_keyframe }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_keyframe }
}

impl<'a> Extract<'a> for BuildingBlocks_Keyframe {
    const TYPE_NAME: &'static str = "BuildingBlocks_Keyframe";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            percent: inst.get_f32("percent").unwrap_or_default(),
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_KeyframeModifierData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_KeyframeModifierData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_KeyframeModifierData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_KeyframeModifierData {
    /// DCB field: `timingFunction` (StrongPointer)
    #[serde(default)]
    pub timing_function: Option<Handle<BuildingBlocks_TimingFunctionBase>>,
    /// DCB field: `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<BuildingBlocks_FieldModifierBase>>,
}

impl Pooled for BuildingBlocks_KeyframeModifierData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_keyframe_modifier_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_keyframe_modifier_data }
}

impl<'a> Extract<'a> for BuildingBlocks_KeyframeModifierData {
    const TYPE_NAME: &'static str = "BuildingBlocks_KeyframeModifierData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timing_function: match inst.get("timingFunction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifier: match inst.get("modifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_Canvas`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Canvas {
    /// DCB field: `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// DCB field: `coordinateMethod` (EnumChoice)
    #[serde(default)]
    pub coordinate_method: String,
    /// DCB field: `embeddedStyles` (Class (array))
    #[serde(default)]
    pub embedded_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// DCB field: `defaultStyles` (Class)
    #[serde(default)]
    pub default_styles: Option<Handle<BuildingBlocks_DefaultStyles>>,
    /// DCB field: `style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
    /// DCB field: `brandStyles` (Class (array))
    #[serde(default)]
    pub brand_styles: Vec<Handle<BuildingBlocks_BrandStyles>>,
    /// DCB field: `entityPreviewScene` (StrongPointer)
    #[serde(default)]
    pub entity_preview_scene: Option<Handle<BuildingBlocks_PreviewSceneEntityRoot>>,
    /// DCB field: `rttPreviewScene` (StrongPointer)
    #[serde(default)]
    pub rtt_preview_scene: Option<Handle<BuildingBlocks_PreviewSceneRttRoot>>,
    /// DCB field: `arPreviewScene` (StrongPointer)
    #[serde(default)]
    pub ar_preview_scene: Option<Handle<BuildingBlocks_PreviewSceneAugmentedRealityRtt>>,
    /// DCB field: `previewScenes` (StrongPointer (array))
    #[serde(default)]
    pub preview_scenes: Vec<Handle<BuildingBlocks_PreviewScreenBase>>,
    /// DCB field: `staticVariables` (StrongPointer (array))
    #[serde(default)]
    pub static_variables: Vec<Handle<BuildingBlocks_StaticVariableBase>>,
    /// DCB field: `proxyRootNode` (WeakPointer)
    #[serde(default)]
    pub proxy_root_node: Option<Handle<BuildingBlocks_WidgetBase>>,
    /// DCB field: `defaultButtonDownSoundTag` (Reference)
    #[serde(default)]
    pub default_button_down_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultRightButtonDownSoundTag` (Reference)
    #[serde(default)]
    pub default_right_button_down_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultLeftClickSoundTag` (Reference)
    #[serde(default)]
    pub default_left_click_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultRightClickSoundTag` (Reference)
    #[serde(default)]
    pub default_right_click_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultLeftDoubleClickSoundTag` (Reference)
    #[serde(default)]
    pub default_left_double_click_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultRollOverSoundTag` (Reference)
    #[serde(default)]
    pub default_roll_over_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultRollOffSoundTag` (Reference)
    #[serde(default)]
    pub default_roll_off_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultMovementStartSoundTag` (Reference)
    #[serde(default)]
    pub default_movement_start_sound_tag: Option<CigGuid>,
    /// DCB field: `defaultMovementStopSoundTag` (Reference)
    #[serde(default)]
    pub default_movement_stop_sound_tag: Option<CigGuid>,
    /// DCB field: `collisionType` (EnumChoice)
    #[serde(default)]
    pub collision_type: String,
    /// DCB field: `cacheAmount` (UInt16)
    #[serde(default)]
    pub cache_amount: u32,
    /// DCB field: `scene` (StrongPointer (array))
    #[serde(default)]
    pub scene: Vec<Handle<BuildingBlocks_WidgetBase>>,
    /// DCB field: `library` (StrongPointer (array))
    #[serde(default)]
    pub library: Vec<Handle<BuildingBlocks_WidgetBase>>,
    /// DCB field: `operations` (StrongPointer (array))
    #[serde(default)]
    pub operations: Vec<Handle<BuildingBlocks_Node>>,
    /// DCB field: `overrideDefaultTooltipCanvas` (Reference)
    #[serde(default)]
    pub override_default_tooltip_canvas: Option<CigGuid>,
    /// DCB field: `overrideDefaultCalloutCanvas` (Reference)
    #[serde(default)]
    pub override_default_callout_canvas: Option<CigGuid>,
}

impl Pooled for BuildingBlocks_Canvas {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_canvas }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_canvas }
}

impl<'a> Extract<'a> for BuildingBlocks_Canvas {
    const TYPE_NAME: &'static str = "BuildingBlocks_Canvas";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            coordinate_method: inst.get_str("coordinateMethod").map(String::from).unwrap_or_default(),
            embedded_styles: inst.get_array("embeddedStyles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_styles: match inst.get("defaultStyles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_DefaultStyles>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_DefaultStyles>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            style: inst.get("style").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            brand_styles: inst.get_array("brandStyles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_BrandStyles>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_BrandStyles>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            entity_preview_scene: match inst.get("entityPreviewScene") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneEntityRoot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneEntityRoot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtt_preview_scene: match inst.get("rttPreviewScene") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneRttRoot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneRttRoot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ar_preview_scene: match inst.get("arPreviewScene") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneAugmentedRealityRtt>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_PreviewSceneAugmentedRealityRtt>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            preview_scenes: inst.get_array("previewScenes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_PreviewScreenBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            static_variables: inst.get_array("staticVariables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StaticVariableBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StaticVariableBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            proxy_root_node: match inst.get("proxyRootNode") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_button_down_sound_tag: inst.get("defaultButtonDownSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_right_button_down_sound_tag: inst.get("defaultRightButtonDownSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_left_click_sound_tag: inst.get("defaultLeftClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_right_click_sound_tag: inst.get("defaultRightClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_left_double_click_sound_tag: inst.get("defaultLeftDoubleClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_roll_over_sound_tag: inst.get("defaultRollOverSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_roll_off_sound_tag: inst.get("defaultRollOffSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_movement_start_sound_tag: inst.get("defaultMovementStartSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_movement_stop_sound_tag: inst.get("defaultMovementStopSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            collision_type: inst.get_str("collisionType").map(String::from).unwrap_or_default(),
            cache_amount: inst.get_u32("cacheAmount").unwrap_or_default(),
            scene: inst.get_array("scene")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            library: inst.get_array("library")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            operations: inst.get_array("operations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_Node>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_Node>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            override_default_tooltip_canvas: inst.get("overrideDefaultTooltipCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            override_default_callout_canvas: inst.get("overrideDefaultCalloutCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `BuildingBlocks_FontStyle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FontStyle {
    /// DCB field: `font` (String)
    #[serde(default)]
    pub font: String,
    /// DCB field: `paintFile` (String)
    #[serde(default)]
    pub paint_file: String,
    /// DCB field: `isBold` (Boolean)
    #[serde(default)]
    pub is_bold: bool,
    /// DCB field: `imageSizePercent` (Single)
    #[serde(default)]
    pub image_size_percent: f32,
    /// DCB field: `scaleModifier` (Single)
    #[serde(default)]
    pub scale_modifier: f32,
    /// DCB field: `leadingModifier` (Single)
    #[serde(default)]
    pub leading_modifier: f32,
    /// DCB field: `topMarginModifier` (Single)
    #[serde(default)]
    pub top_margin_modifier: f32,
    /// DCB field: `leftMarginModifier` (Single)
    #[serde(default)]
    pub left_margin_modifier: f32,
    /// DCB field: `numImageReplacementSpaces` (Int32)
    #[serde(default)]
    pub num_image_replacement_spaces: i32,
}

impl Pooled for BuildingBlocks_FontStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_font_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_font_style }
}

impl<'a> Extract<'a> for BuildingBlocks_FontStyle {
    const TYPE_NAME: &'static str = "BuildingBlocks_FontStyle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            font: inst.get_str("font").map(String::from).unwrap_or_default(),
            paint_file: inst.get_str("paintFile").map(String::from).unwrap_or_default(),
            is_bold: inst.get_bool("isBold").unwrap_or_default(),
            image_size_percent: inst.get_f32("imageSizePercent").unwrap_or_default(),
            scale_modifier: inst.get_f32("scaleModifier").unwrap_or_default(),
            leading_modifier: inst.get_f32("leadingModifier").unwrap_or_default(),
            top_margin_modifier: inst.get_f32("topMarginModifier").unwrap_or_default(),
            left_margin_modifier: inst.get_f32("leftMarginModifier").unwrap_or_default(),
            num_image_replacement_spaces: inst.get_i32("numImageReplacementSpaces").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FontReplacementPair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FontReplacementPair {
    /// DCB field: `englishFont` (Reference)
    #[serde(default)]
    pub english_font: Option<CigGuid>,
    /// DCB field: `replacementFontName` (String)
    #[serde(default)]
    pub replacement_font_name: String,
    /// DCB field: `replacementFontPaintFile` (String)
    #[serde(default)]
    pub replacement_font_paint_file: String,
}

impl Pooled for BuildingBlocks_FontReplacementPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_font_replacement_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_font_replacement_pair }
}

impl<'a> Extract<'a> for BuildingBlocks_FontReplacementPair {
    const TYPE_NAME: &'static str = "BuildingBlocks_FontReplacementPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            english_font: inst.get("englishFont").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            replacement_font_name: inst.get_str("replacementFontName").map(String::from).unwrap_or_default(),
            replacement_font_paint_file: inst.get_str("replacementFontPaintFile").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_LanguageSpecificFontReplacement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LanguageSpecificFontReplacement {
    /// DCB field: `fontReplacementList` (Class (array))
    #[serde(default)]
    pub font_replacement_list: Vec<Handle<BuildingBlocks_FontReplacementPair>>,
}

impl Pooled for BuildingBlocks_LanguageSpecificFontReplacement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_language_specific_font_replacement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_language_specific_font_replacement }
}

impl<'a> Extract<'a> for BuildingBlocks_LanguageSpecificFontReplacement {
    const TYPE_NAME: &'static str = "BuildingBlocks_LanguageSpecificFontReplacement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            font_replacement_list: inst.get_array("fontReplacementList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TriggerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TriggerBase {
}

impl Pooled for BuildingBlocks_TriggerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_trigger_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_trigger_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TriggerBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TriggerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_Interactions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Interactions {
    /// DCB field: `canInteract` (Boolean)
    #[serde(default)]
    pub can_interact: bool,
    /// DCB field: `disabled` (Boolean)
    #[serde(default)]
    pub disabled: bool,
    /// DCB field: `selected` (Boolean)
    #[serde(default)]
    pub selected: bool,
    /// DCB field: `collideLeftClick` (Boolean)
    #[serde(default)]
    pub collide_left_click: bool,
    /// DCB field: `collideRightClick` (Boolean)
    #[serde(default)]
    pub collide_right_click: bool,
    /// DCB field: `collideMouseWheel` (Boolean)
    #[serde(default)]
    pub collide_mouse_wheel: bool,
    /// DCB field: `hitDetectionOffset` (Single)
    #[serde(default)]
    pub hit_detection_offset: f32,
    /// DCB field: `propagateInteractionStates` (Boolean)
    #[serde(default)]
    pub propagate_interaction_states: bool,
    /// DCB field: `inheritHoverState` (Boolean)
    #[serde(default)]
    pub inherit_hover_state: bool,
    /// DCB field: `inheritDownState` (Boolean)
    #[serde(default)]
    pub inherit_down_state: bool,
    /// DCB field: `inheritSelectedState` (Boolean)
    #[serde(default)]
    pub inherit_selected_state: bool,
    /// DCB field: `inheritDisabledState` (Boolean)
    #[serde(default)]
    pub inherit_disabled_state: bool,
    /// DCB field: `blocksCursor` (Boolean)
    #[serde(default)]
    pub blocks_cursor: bool,
    /// DCB field: `blocksNavigation` (Boolean)
    #[serde(default)]
    pub blocks_navigation: bool,
    /// DCB field: `onLeftClick` (StrongPointer (array))
    #[serde(default)]
    pub on_left_click: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onRightClick` (StrongPointer (array))
    #[serde(default)]
    pub on_right_click: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onLeftDown` (StrongPointer (array))
    #[serde(default)]
    pub on_left_down: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onLeftDoubleClick` (StrongPointer (array))
    #[serde(default)]
    pub on_left_double_click: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onRollOver` (StrongPointer (array))
    #[serde(default)]
    pub on_roll_over: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `onRollOff` (StrongPointer (array))
    #[serde(default)]
    pub on_roll_off: Vec<Handle<BuildingBlocks_TriggerBase>>,
    /// DCB field: `boundRestrictionShape` (StrongPointer)
    #[serde(default)]
    pub bound_restriction_shape: Option<Handle<BuildingBlocks_ShapeBase>>,
    /// DCB field: `twoVariablePicker` (StrongPointer)
    #[serde(default)]
    pub two_variable_picker: Option<Handle<BuildingBlocks_TwoVariablePicker>>,
    /// DCB field: `buttonDownSoundTag` (Reference)
    #[serde(default)]
    pub button_down_sound_tag: Option<CigGuid>,
    /// DCB field: `rightButtonDownSoundTag` (Reference)
    #[serde(default)]
    pub right_button_down_sound_tag: Option<CigGuid>,
    /// DCB field: `leftClickSoundTag` (Reference)
    #[serde(default)]
    pub left_click_sound_tag: Option<CigGuid>,
    /// DCB field: `rightClickSoundTag` (Reference)
    #[serde(default)]
    pub right_click_sound_tag: Option<CigGuid>,
    /// DCB field: `leftDoubleClickSoundTag` (Reference)
    #[serde(default)]
    pub left_double_click_sound_tag: Option<CigGuid>,
    /// DCB field: `rollOverSoundTag` (Reference)
    #[serde(default)]
    pub roll_over_sound_tag: Option<CigGuid>,
    /// DCB field: `rollOffSoundTag` (Reference)
    #[serde(default)]
    pub roll_off_sound_tag: Option<CigGuid>,
    /// DCB field: `movementStartSoundTag` (Reference)
    #[serde(default)]
    pub movement_start_sound_tag: Option<CigGuid>,
    /// DCB field: `movementStopSoundTag` (Reference)
    #[serde(default)]
    pub movement_stop_sound_tag: Option<CigGuid>,
    /// DCB field: `movementStopDelayMs` (Single)
    #[serde(default)]
    pub movement_stop_delay_ms: f32,
}

impl Pooled for BuildingBlocks_Interactions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_interactions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_interactions }
}

impl<'a> Extract<'a> for BuildingBlocks_Interactions {
    const TYPE_NAME: &'static str = "BuildingBlocks_Interactions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            can_interact: inst.get_bool("canInteract").unwrap_or_default(),
            disabled: inst.get_bool("disabled").unwrap_or_default(),
            selected: inst.get_bool("selected").unwrap_or_default(),
            collide_left_click: inst.get_bool("collideLeftClick").unwrap_or_default(),
            collide_right_click: inst.get_bool("collideRightClick").unwrap_or_default(),
            collide_mouse_wheel: inst.get_bool("collideMouseWheel").unwrap_or_default(),
            hit_detection_offset: inst.get_f32("hitDetectionOffset").unwrap_or_default(),
            propagate_interaction_states: inst.get_bool("propagateInteractionStates").unwrap_or_default(),
            inherit_hover_state: inst.get_bool("inheritHoverState").unwrap_or_default(),
            inherit_down_state: inst.get_bool("inheritDownState").unwrap_or_default(),
            inherit_selected_state: inst.get_bool("inheritSelectedState").unwrap_or_default(),
            inherit_disabled_state: inst.get_bool("inheritDisabledState").unwrap_or_default(),
            blocks_cursor: inst.get_bool("blocksCursor").unwrap_or_default(),
            blocks_navigation: inst.get_bool("blocksNavigation").unwrap_or_default(),
            on_left_click: inst.get_array("onLeftClick")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_right_click: inst.get_array("onRightClick")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_left_down: inst.get_array("onLeftDown")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_left_double_click: inst.get_array("onLeftDoubleClick")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_roll_over: inst.get_array("onRollOver")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            on_roll_off: inst.get_array("onRollOff")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            bound_restriction_shape: match inst.get("boundRestrictionShape") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ShapeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ShapeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            two_variable_picker: match inst.get("twoVariablePicker") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TwoVariablePicker>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TwoVariablePicker>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            button_down_sound_tag: inst.get("buttonDownSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            right_button_down_sound_tag: inst.get("rightButtonDownSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            left_click_sound_tag: inst.get("leftClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            right_click_sound_tag: inst.get("rightClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            left_double_click_sound_tag: inst.get("leftDoubleClickSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            roll_over_sound_tag: inst.get("rollOverSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            roll_off_sound_tag: inst.get("rollOffSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_start_sound_tag: inst.get("movementStartSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_stop_sound_tag: inst.get("movementStopSoundTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_stop_delay_ms: inst.get_f32("movementStopDelayMs").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ShapeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ShapeBase {
    /// DCB field: `center` (Class)
    #[serde(default)]
    pub center: Option<Handle<Vec2>>,
    /// DCB field: `showBoundsDebug` (Boolean)
    #[serde(default)]
    pub show_bounds_debug: bool,
}

impl Pooled for BuildingBlocks_ShapeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_shape_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_shape_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ShapeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ShapeBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            show_bounds_debug: inst.get_bool("showBoundsDebug").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TwoVariablePicker`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TwoVariablePicker {
    /// DCB field: `variableXURL` (String)
    #[serde(default)]
    pub variable_xurl: String,
    /// DCB field: `variableYURL` (String)
    #[serde(default)]
    pub variable_yurl: String,
    /// DCB field: `markerNodeDef` (WeakPointer)
    #[serde(default)]
    pub marker_node_def: Option<Handle<BuildingBlocks_WidgetBase>>,
}

impl Pooled for BuildingBlocks_TwoVariablePicker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_two_variable_picker }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_two_variable_picker }
}

impl<'a> Extract<'a> for BuildingBlocks_TwoVariablePicker {
    const TYPE_NAME: &'static str = "BuildingBlocks_TwoVariablePicker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variable_xurl: inst.get_str("variableXURL").map(String::from).unwrap_or_default(),
            variable_yurl: inst.get_str("variableYURL").map(String::from).unwrap_or_default(),
            marker_node_def: match inst.get("markerNodeDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_WidgetBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_TooltipPolicy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TooltipPolicy {
    /// DCB field: `startDelay` (Single)
    #[serde(default)]
    pub start_delay: f32,
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec2>>,
    /// DCB field: `shouldUseParentOffsets` (Boolean)
    #[serde(default)]
    pub should_use_parent_offsets: bool,
    /// DCB field: `isLocal` (Boolean)
    #[serde(default)]
    pub is_local: bool,
}

impl Pooled for BuildingBlocks_TooltipPolicy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_tooltip_policy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_tooltip_policy }
}

impl<'a> Extract<'a> for BuildingBlocks_TooltipPolicy {
    const TYPE_NAME: &'static str = "BuildingBlocks_TooltipPolicy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_delay: inst.get_f32("startDelay").unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            should_use_parent_offsets: inst.get_bool("shouldUseParentOffsets").unwrap_or_default(),
            is_local: inst.get_bool("isLocal").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ContextMenuItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ContextMenuItem {
    /// DCB field: `locString` (Locale)
    #[serde(default)]
    pub loc_string: String,
    /// DCB field: `triggerURL` (String)
    #[serde(default)]
    pub trigger_url: String,
}

impl Pooled for BuildingBlocks_ContextMenuItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_context_menu_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_context_menu_item }
}

impl<'a> Extract<'a> for BuildingBlocks_ContextMenuItem {
    const TYPE_NAME: &'static str = "BuildingBlocks_ContextMenuItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loc_string: inst.get_str("locString").map(String::from).unwrap_or_default(),
            trigger_url: inst.get_str("triggerURL").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ContextMenuPolicy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ContextMenuPolicy {
    /// DCB field: `namespace` (String)
    #[serde(default)]
    pub namespace: String,
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
    /// DCB field: `menuItems` (Class (array))
    #[serde(default)]
    pub menu_items: Vec<Handle<BuildingBlocks_ContextMenuItem>>,
    /// DCB field: `closeDistance` (Single)
    #[serde(default)]
    pub close_distance: f32,
    /// DCB field: `isLocal` (Boolean)
    #[serde(default)]
    pub is_local: bool,
}

impl Pooled for BuildingBlocks_ContextMenuPolicy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_context_menu_policy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_context_menu_policy }
}

impl<'a> Extract<'a> for BuildingBlocks_ContextMenuPolicy {
    const TYPE_NAME: &'static str = "BuildingBlocks_ContextMenuPolicy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            namespace: inst.get_str("namespace").map(String::from).unwrap_or_default(),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            menu_items: inst.get_array("menuItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_ContextMenuItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_ContextMenuItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            close_distance: inst.get_f32("closeDistance").unwrap_or_default(),
            is_local: inst.get_bool("isLocal").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Range`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Range {
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
}

impl Pooled for BuildingBlocks_Range {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_range }
}

impl<'a> Extract<'a> for BuildingBlocks_Range {
    const TYPE_NAME: &'static str = "BuildingBlocks_Range";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_GrabControlsPolicy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_GrabControlsPolicy {
    /// DCB field: `grabBounds` (EnumChoice)
    #[serde(default)]
    pub grab_bounds: String,
    /// DCB field: `grabRotationSpeed` (Single)
    #[serde(default)]
    pub grab_rotation_speed: f32,
    /// DCB field: `grabRotationSlowdown` (Single)
    #[serde(default)]
    pub grab_rotation_slowdown: f32,
    /// DCB field: `grabResponsiveness` (Single)
    #[serde(default)]
    pub grab_responsiveness: f32,
    /// DCB field: `pitchRestrictions` (StrongPointer)
    #[serde(default)]
    pub pitch_restrictions: Option<Handle<BuildingBlocks_Range>>,
    /// DCB field: `yawRestrictions` (StrongPointer)
    #[serde(default)]
    pub yaw_restrictions: Option<Handle<BuildingBlocks_Range>>,
    /// DCB field: `grabRotationMode` (EnumChoice)
    #[serde(default)]
    pub grab_rotation_mode: String,
}

impl Pooled for BuildingBlocks_GrabControlsPolicy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_grab_controls_policy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_grab_controls_policy }
}

impl<'a> Extract<'a> for BuildingBlocks_GrabControlsPolicy {
    const TYPE_NAME: &'static str = "BuildingBlocks_GrabControlsPolicy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            grab_bounds: inst.get_str("grabBounds").map(String::from).unwrap_or_default(),
            grab_rotation_speed: inst.get_f32("grabRotationSpeed").unwrap_or_default(),
            grab_rotation_slowdown: inst.get_f32("grabRotationSlowdown").unwrap_or_default(),
            grab_responsiveness: inst.get_f32("grabResponsiveness").unwrap_or_default(),
            pitch_restrictions: match inst.get("pitchRestrictions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw_restrictions: match inst.get("yawRestrictions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            grab_rotation_mode: inst.get_str("grabRotationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_DefaultStyles`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_DefaultStyles {
    /// DCB field: `sharedStyles` (Reference)
    #[serde(default)]
    pub shared_styles: Option<CigGuid>,
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<BuildingBlocks_StyleEntry>>,
}

impl Pooled for BuildingBlocks_DefaultStyles {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_default_styles }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_default_styles }
}

impl<'a> Extract<'a> for BuildingBlocks_DefaultStyles {
    const TYPE_NAME: &'static str = "BuildingBlocks_DefaultStyles";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shared_styles: inst.get("sharedStyles").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_BrandStyles`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BrandStyles {
    /// DCB field: `brandIdentifier` (Reference)
    #[serde(default)]
    pub brand_identifier: Option<CigGuid>,
    /// DCB field: `sharedStyles` (Reference)
    #[serde(default)]
    pub shared_styles: Option<CigGuid>,
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<BuildingBlocks_StyleEntry>>,
}

impl Pooled for BuildingBlocks_BrandStyles {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_brand_styles }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_brand_styles }
}

impl<'a> Extract<'a> for BuildingBlocks_BrandStyles {
    const TYPE_NAME: &'static str = "BuildingBlocks_BrandStyles";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            brand_identifier: inst.get("brandIdentifier").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            shared_styles: inst.get("sharedStyles").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_StyleEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_StyleEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `conditionsList` (Class (array))
    #[serde(default)]
    pub conditions_list: Vec<Handle<BuildingBlocks_StyleConditionList>>,
    /// DCB field: `modifiers` (StrongPointer (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BuildingBlocks_FieldModifierBase>>,
    /// DCB field: `transitions` (StrongPointer (array))
    #[serde(default)]
    pub transitions: Vec<Handle<BuildingBlocks_FieldTransitionBase>>,
}

impl Pooled for BuildingBlocks_StyleEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_style_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_style_entry }
}

impl<'a> Extract<'a> for BuildingBlocks_StyleEntry {
    const TYPE_NAME: &'static str = "BuildingBlocks_StyleEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            conditions_list: inst.get_array("conditionsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleConditionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleConditionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            transitions: inst.get_array("transitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_FieldTransitionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_FieldTransitionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TextFormatModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TextFormatModifierBase {
}

impl Pooled for BuildingBlocks_TextFormatModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_text_format_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_text_format_modifier_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TextFormatModifierBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TextFormatModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TextEmphasisModifierList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TextEmphasisModifierList {
    /// DCB field: `textEmphasis` (StrongPointer (array))
    #[serde(default)]
    pub text_emphasis: Vec<Handle<BuildingBlocks_TextFormatModifierBase>>,
}

impl Pooled for BuildingBlocks_TextEmphasisModifierList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_text_emphasis_modifier_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_text_emphasis_modifier_list }
}

impl<'a> Extract<'a> for BuildingBlocks_TextEmphasisModifierList {
    const TYPE_NAME: &'static str = "BuildingBlocks_TextEmphasisModifierList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            text_emphasis: inst.get_array("textEmphasis")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TextFormatModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TextFormatModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Style`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Style {
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// DCB field: `colorStyles` (StrongPointer)
    #[serde(default)]
    pub color_styles: Option<Handle<BuildingBlocks_ColorBase>>,
    /// DCB field: `textFieldModifiers` (Class (array))
    #[serde(default)]
    pub text_field_modifiers: Vec<Handle<BuildingBlocks_TextEmphasisModifierList>>,
}

impl Pooled for BuildingBlocks_Style {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_style }
}

impl<'a> Extract<'a> for BuildingBlocks_Style {
    const TYPE_NAME: &'static str = "BuildingBlocks_Style";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            color_styles: match inst.get("colorStyles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            text_field_modifiers: inst.get_array("textFieldModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TextEmphasisModifierList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TextEmphasisModifierList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_StyleConditionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_StyleConditionList {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<BuildingBlocks_StyleSelectorConditionBase>>,
}

impl Pooled for BuildingBlocks_StyleConditionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_style_condition_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_style_condition_list }
}

impl<'a> Extract<'a> for BuildingBlocks_StyleConditionList {
    const TYPE_NAME: &'static str = "BuildingBlocks_StyleConditionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleSelectorConditionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleSelectorConditionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_StyleSelectorConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_StyleSelectorConditionBase {
}

impl Pooled for BuildingBlocks_StyleSelectorConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_style_selector_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_style_selector_condition_base }
}

impl<'a> Extract<'a> for BuildingBlocks_StyleSelectorConditionBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_StyleSelectorConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_StaticVariableBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_StaticVariableBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for BuildingBlocks_StaticVariableBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_static_variable_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_static_variable_base }
}

impl<'a> Extract<'a> for BuildingBlocks_StaticVariableBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_StaticVariableBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ColorBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ColorBase {
}

impl Pooled for BuildingBlocks_ColorBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_color_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_color_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ColorBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ColorBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_ExternalColorReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ExternalColorReference {
    /// DCB field: `color` (StrongPointer)
    #[serde(default)]
    pub color: Option<Handle<BuildingBlocks_ColorBase>>,
}

impl Pooled for BuildingBlocks_ExternalColorReference {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_external_color_reference }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_external_color_reference }
}

impl<'a> Extract<'a> for BuildingBlocks_ExternalColorReference {
    const TYPE_NAME: &'static str = "BuildingBlocks_ExternalColorReference";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_AspectRatioOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_AspectRatioOption {
    /// DCB field: `aspectRatio` (Single)
    #[serde(default)]
    pub aspect_ratio: f32,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for BuildingBlocks_AspectRatioOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_aspect_ratio_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_aspect_ratio_option }
}

impl<'a> Extract<'a> for BuildingBlocks_AspectRatioOption {
    const TYPE_NAME: &'static str = "BuildingBlocks_AspectRatioOption";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            aspect_ratio: inst.get_f32("aspectRatio").unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `BuildingBlocks_AspectRatioLibrary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_AspectRatioLibrary {
    /// DCB field: `aspectRatioOptions` (Class (array))
    #[serde(default)]
    pub aspect_ratio_options: Vec<Handle<BuildingBlocks_AspectRatioOption>>,
}

impl Pooled for BuildingBlocks_AspectRatioLibrary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bu.building_blocks_aspect_ratio_library }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bu.building_blocks_aspect_ratio_library }
}

impl<'a> Extract<'a> for BuildingBlocks_AspectRatioLibrary {
    const TYPE_NAME: &'static str = "BuildingBlocks_AspectRatioLibrary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aspect_ratio_options: inst.get_array("aspectRatioOptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_AspectRatioOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_AspectRatioOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

