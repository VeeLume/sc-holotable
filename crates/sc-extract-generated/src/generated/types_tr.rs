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

/// DCB type: `TraversalCostConditionTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalCostConditionTags {
    /// DCB field: `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagsDNF>>,
    /// DCB field: `costMultiplier` (Single)
    #[serde(default)]
    pub cost_multiplier: f32,
    /// DCB field: `blocksTraversability` (Boolean)
    #[serde(default)]
    pub blocks_traversability: bool,
    /// DCB field: `blocksStopping` (Boolean)
    #[serde(default)]
    pub blocks_stopping: bool,
}

impl Pooled for TraversalCostConditionTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.traversal_cost_condition_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.traversal_cost_condition_tags }
}

impl<'a> Extract<'a> for TraversalCostConditionTags {
    const TYPE_NAME: &'static str = "TraversalCostConditionTags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cost_multiplier: inst.get_f32("costMultiplier").unwrap_or_default(),
            blocks_traversability: inst.get_bool("blocksTraversability").unwrap_or_default(),
            blocks_stopping: inst.get_bool("blocksStopping").unwrap_or_default(),
        }
    }
}

/// DCB type: `TraversalCostConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalCostConfig {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `costs` (Class (array))
    #[serde(default)]
    pub costs: Vec<Handle<CostModifierPerAgentType>>,
}

impl Pooled for TraversalCostConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.traversal_cost_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.traversal_cost_config }
}

impl<'a> Extract<'a> for TraversalCostConfig {
    const TYPE_NAME: &'static str = "TraversalCostConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            costs: inst.get_array("costs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CostModifierPerAgentType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CostModifierPerAgentType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TrailFadingSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrailFadingSettings {
    /// DCB field: `minimumVisibleSpeed` (Single)
    #[serde(default)]
    pub minimum_visible_speed: f32,
    /// DCB field: `speedFadeRatio` (Single)
    #[serde(default)]
    pub speed_fade_ratio: f32,
    /// DCB field: `lowIdleBound` (Single)
    #[serde(default)]
    pub low_idle_bound: f32,
    /// DCB field: `idleThrustBound` (Single)
    #[serde(default)]
    pub idle_thrust_bound: f32,
    /// DCB field: `thrustAfterburnBound` (Single)
    #[serde(default)]
    pub thrust_afterburn_bound: f32,
}

impl Pooled for TrailFadingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.trail_fading_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.trail_fading_settings }
}

impl<'a> Extract<'a> for TrailFadingSettings {
    const TYPE_NAME: &'static str = "TrailFadingSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_visible_speed: inst.get_f32("minimumVisibleSpeed").unwrap_or_default(),
            speed_fade_ratio: inst.get_f32("speedFadeRatio").unwrap_or_default(),
            low_idle_bound: inst.get_f32("lowIdleBound").unwrap_or_default(),
            idle_thrust_bound: inst.get_f32("idleThrustBound").unwrap_or_default(),
            thrust_afterburn_bound: inst.get_f32("thrustAfterburnBound").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransformationInterpolatorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationInterpolatorParams {
    /// DCB field: `startOffsetValues` (Class)
    #[serde(default)]
    pub start_offset_values: Option<Handle<Vec3>>,
    /// DCB field: `endOffsetValues` (Class)
    #[serde(default)]
    pub end_offset_values: Option<Handle<Vec3>>,
    /// DCB field: `offsetInterpolationModes` (EnumChoice)
    #[serde(default)]
    pub offset_interpolation_modes: String,
    /// DCB field: `startRotationValues` (Class)
    #[serde(default)]
    pub start_rotation_values: Option<Handle<Ang3>>,
    /// DCB field: `endRotationValues` (Class)
    #[serde(default)]
    pub end_rotation_values: Option<Handle<Ang3>>,
    /// DCB field: `rotationInterpolationModes` (EnumChoice)
    #[serde(default)]
    pub rotation_interpolation_modes: String,
    /// DCB field: `startScaleValue` (Single)
    #[serde(default)]
    pub start_scale_value: f32,
    /// DCB field: `endScaleValue` (Single)
    #[serde(default)]
    pub end_scale_value: f32,
    /// DCB field: `scaleInterpolationMode` (EnumChoice)
    #[serde(default)]
    pub scale_interpolation_mode: String,
}

impl Pooled for TransformationInterpolatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.transformation_interpolator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.transformation_interpolator_params }
}

impl<'a> Extract<'a> for TransformationInterpolatorParams {
    const TYPE_NAME: &'static str = "TransformationInterpolatorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_offset_values: match inst.get("startOffsetValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_offset_values: match inst.get("endOffsetValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_interpolation_modes: inst.get_str("offsetInterpolationModes").map(String::from).unwrap_or_default(),
            start_rotation_values: match inst.get("startRotationValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_rotation_values: match inst.get("endRotationValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_interpolation_modes: inst.get_str("rotationInterpolationModes").map(String::from).unwrap_or_default(),
            start_scale_value: inst.get_f32("startScaleValue").unwrap_or_default(),
            end_scale_value: inst.get_f32("endScaleValue").unwrap_or_default(),
            scale_interpolation_mode: inst.get_str("scaleInterpolationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `TransformationInterpolator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationInterpolator {
    /// DCB field: `interpolationTime` (Single)
    #[serde(default)]
    pub interpolation_time: f32,
    /// DCB field: `transformationInterpolatorParams` (Class)
    #[serde(default)]
    pub transformation_interpolator_params: Option<Handle<TransformationInterpolatorParams>>,
}

impl Pooled for TransformationInterpolator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.transformation_interpolator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.transformation_interpolator }
}

impl<'a> Extract<'a> for TransformationInterpolator {
    const TYPE_NAME: &'static str = "TransformationInterpolator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interpolation_time: inst.get_f32("interpolationTime").unwrap_or_default(),
            transformation_interpolator_params: match inst.get("transformationInterpolatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TransitStationAnnouncements`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitStationAnnouncements {
    /// DCB field: `announcements` (Class (array))
    #[serde(default)]
    pub announcements: Vec<Handle<TransitStationAnnouncement>>,
}

impl Pooled for TransitStationAnnouncements {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.transit_station_announcements }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.transit_station_announcements }
}

impl<'a> Extract<'a> for TransitStationAnnouncements {
    const TYPE_NAME: &'static str = "TransitStationAnnouncements";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            announcements: inst.get_array("announcements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TransitStationAnnouncement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TransitStationAnnouncement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitStationAnnouncement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitStationAnnouncement {
    /// DCB field: `stationName` (Locale)
    #[serde(default)]
    pub station_name: String,
    /// DCB field: `preArrival` (Class)
    #[serde(default)]
    pub pre_arrival: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `postDeparture` (Class)
    #[serde(default)]
    pub post_departure: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for TransitStationAnnouncement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tr.transit_station_announcement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tr.transit_station_announcement }
}

impl<'a> Extract<'a> for TransitStationAnnouncement {
    const TYPE_NAME: &'static str = "TransitStationAnnouncement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            station_name: inst.get_str("stationName").map(String::from).unwrap_or_default(),
            pre_arrival: match inst.get("preArrival") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            post_departure: match inst.get("postDeparture") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

