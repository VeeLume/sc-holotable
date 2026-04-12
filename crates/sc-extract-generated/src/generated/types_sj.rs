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

/// DCB type: `SJumpTunnelVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelVisualParams {
    /// DCB field: `quantumEffect` (Boolean)
    #[serde(default)]
    pub quantum_effect: bool,
    /// DCB field: `portalExternalViewDistRatio` (Single)
    #[serde(default)]
    pub portal_external_view_dist_ratio: f32,
    /// DCB field: `portalInternalViewDistRatio` (Single)
    #[serde(default)]
    pub portal_internal_view_dist_ratio: f32,
    /// DCB field: `visAreaViewDistRatio` (Single)
    #[serde(default)]
    pub vis_area_view_dist_ratio: f32,
    /// DCB field: `probeIntensity` (Single)
    #[serde(default)]
    pub probe_intensity: f32,
    /// DCB field: `internalDrawDistance` (Single)
    #[serde(default)]
    pub internal_draw_distance: f32,
    /// DCB field: `externalDrawDistance` (Single)
    #[serde(default)]
    pub external_draw_distance: f32,
    /// DCB field: `openingHiddenTime` (Single)
    #[serde(default)]
    pub opening_hidden_time: f32,
    /// DCB field: `closingHiddenTime` (Single)
    #[serde(default)]
    pub closing_hidden_time: f32,
    /// DCB field: `exitPortalCullingDistance` (Single)
    #[serde(default)]
    pub exit_portal_culling_distance: f32,
}

impl Pooled for SJumpTunnelVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_visual_params }
}

impl<'a> Extract<'a> for SJumpTunnelVisualParams {
    const TYPE_NAME: &'static str = "SJumpTunnelVisualParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            quantum_effect: inst.get_bool("quantumEffect").unwrap_or_default(),
            portal_external_view_dist_ratio: inst.get_f32("portalExternalViewDistRatio").unwrap_or_default(),
            portal_internal_view_dist_ratio: inst.get_f32("portalInternalViewDistRatio").unwrap_or_default(),
            vis_area_view_dist_ratio: inst.get_f32("visAreaViewDistRatio").unwrap_or_default(),
            probe_intensity: inst.get_f32("probeIntensity").unwrap_or_default(),
            internal_draw_distance: inst.get_f32("internalDrawDistance").unwrap_or_default(),
            external_draw_distance: inst.get_f32("externalDrawDistance").unwrap_or_default(),
            opening_hidden_time: inst.get_f32("openingHiddenTime").unwrap_or_default(),
            closing_hidden_time: inst.get_f32("closingHiddenTime").unwrap_or_default(),
            exit_portal_culling_distance: inst.get_f32("exitPortalCullingDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpPointPushAreaParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpPointPushAreaParams {
    /// DCB field: `areaRadius` (Single)
    #[serde(default)]
    pub area_radius: f32,
    /// DCB field: `softPushAreaDistance` (Single)
    #[serde(default)]
    pub soft_push_area_distance: f32,
    /// DCB field: `strength` (Single)
    #[serde(default)]
    pub strength: f32,
    /// DCB field: `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// DCB field: `falloffRatio` (Single)
    #[serde(default)]
    pub falloff_ratio: f32,
}

impl Pooled for SJumpPointPushAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_point_push_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_point_push_area_params }
}

impl<'a> Extract<'a> for SJumpPointPushAreaParams {
    const TYPE_NAME: &'static str = "SJumpPointPushAreaParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            area_radius: inst.get_f32("areaRadius").unwrap_or_default(),
            soft_push_area_distance: inst.get_f32("softPushAreaDistance").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            falloff_ratio: inst.get_f32("falloffRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelSectionProbabilityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionProbabilityParams {
    /// DCB field: `section` (WeakPointer)
    #[serde(default)]
    pub section: Option<Handle<SJumpTunnelSectionGenerationParams>>,
    /// DCB field: `probability` (Single)
    #[serde(default)]
    pub probability: f32,
}

impl Pooled for SJumpTunnelSectionProbabilityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_section_probability_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_section_probability_params }
}

impl<'a> Extract<'a> for SJumpTunnelSectionProbabilityParams {
    const TYPE_NAME: &'static str = "SJumpTunnelSectionProbabilityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            section: match inst.get("section") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            probability: inst.get_f32("probability").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelSectionControlPointGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionControlPointGenerationParams {
    /// DCB field: `sectionOffset` (Class)
    #[serde(default)]
    pub section_offset: Option<Handle<Range>>,
    /// DCB field: `sectionLength` (Class)
    #[serde(default)]
    pub section_length: Option<Handle<Range>>,
    /// DCB field: `angleOffset` (Class)
    #[serde(default)]
    pub angle_offset: Option<Handle<Range>>,
}

impl Pooled for SJumpTunnelSectionControlPointGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_section_control_point_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_section_control_point_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelSectionControlPointGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelSectionControlPointGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            section_offset: match inst.get("sectionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            section_length: match inst.get("sectionLength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle_offset: match inst.get("angleOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelObstacleGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelObstacleGenerationParams {
    /// DCB field: `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Range>>,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Range>>,
    /// DCB field: `angle` (Class)
    #[serde(default)]
    pub angle: Option<Handle<Range>>,
}

impl Pooled for SJumpTunnelObstacleGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_obstacle_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_obstacle_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelObstacleGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelObstacleGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle: match inst.get("angle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelEllipticalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelEllipticalParams {
    /// DCB field: `minRadius` (Single)
    #[serde(default)]
    pub min_radius: f32,
    /// DCB field: `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// DCB field: `radiusMultiplier` (Class)
    #[serde(default)]
    pub radius_multiplier: Option<Handle<BezierCurve>>,
    /// DCB field: `minAspectRatio` (Single)
    #[serde(default)]
    pub min_aspect_ratio: f32,
    /// DCB field: `maxAspectRatio` (Single)
    #[serde(default)]
    pub max_aspect_ratio: f32,
    /// DCB field: `aspectRatioMultiplier` (Class)
    #[serde(default)]
    pub aspect_ratio_multiplier: Option<Handle<BezierCurve>>,
    /// DCB field: `chanceToFollowCurvature` (Single)
    #[serde(default)]
    pub chance_to_follow_curvature: f32,
}

impl Pooled for SJumpTunnelEllipticalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_elliptical_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_elliptical_params }
}

impl<'a> Extract<'a> for SJumpTunnelEllipticalParams {
    const TYPE_NAME: &'static str = "SJumpTunnelEllipticalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_radius: inst.get_f32("minRadius").unwrap_or_default(),
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
            radius_multiplier: match inst.get("radiusMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_aspect_ratio: inst.get_f32("minAspectRatio").unwrap_or_default(),
            max_aspect_ratio: inst.get_f32("maxAspectRatio").unwrap_or_default(),
            aspect_ratio_multiplier: match inst.get("aspectRatioMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chance_to_follow_curvature: inst.get_f32("chanceToFollowCurvature").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelSectionGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionGenerationParams {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `defaultSectionLength` (Single)
    #[serde(default)]
    pub default_section_length: f32,
    /// DCB field: `nextSectionProbabilities` (Class (array))
    #[serde(default)]
    pub next_section_probabilities: Vec<Handle<SJumpTunnelSectionProbabilityParams>>,
    /// DCB field: `controlPoints` (Class (array))
    #[serde(default)]
    pub control_points: Vec<Handle<SJumpTunnelSectionControlPointGenerationParams>>,
    /// DCB field: `chanceOfObstacles` (Single)
    #[serde(default)]
    pub chance_of_obstacles: f32,
    /// DCB field: `numberOfObstacles` (Class)
    #[serde(default)]
    pub number_of_obstacles: Option<Handle<Range>>,
    /// DCB field: `obstacleGeneration` (Class)
    #[serde(default)]
    pub obstacle_generation: Option<Handle<SJumpTunnelObstacleGenerationParams>>,
    /// DCB field: `ellipticalParams` (Class)
    #[serde(default)]
    pub elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
}

impl Pooled for SJumpTunnelSectionGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_section_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_section_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelSectionGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelSectionGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            default_section_length: inst.get_f32("defaultSectionLength").unwrap_or_default(),
            next_section_probabilities: inst.get_array("nextSectionProbabilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            control_points: inst.get_array("controlPoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionControlPointGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SJumpTunnelSectionControlPointGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            chance_of_obstacles: inst.get_f32("chanceOfObstacles").unwrap_or_default(),
            number_of_obstacles: match inst.get("numberOfObstacles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            obstacle_generation: match inst.get("obstacleGeneration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelObstacleGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelObstacleGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            elliptical_params: match inst.get("ellipticalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelGenerationParams {
    /// DCB field: `tunnelRadius` (Single)
    #[serde(default)]
    pub tunnel_radius: f32,
    /// DCB field: `tunnelRadiusGameplayBuffer` (Single)
    #[serde(default)]
    pub tunnel_radius_gameplay_buffer: f32,
    /// DCB field: `tunnelLength` (Single)
    #[serde(default)]
    pub tunnel_length: f32,
    /// DCB field: `entranceLength` (Single)
    #[serde(default)]
    pub entrance_length: f32,
    /// DCB field: `exitLength` (Single)
    #[serde(default)]
    pub exit_length: f32,
    /// DCB field: `entranceEllipticalParams` (Class)
    #[serde(default)]
    pub entrance_elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
    /// DCB field: `exitEllipticalParams` (Class)
    #[serde(default)]
    pub exit_elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
    /// DCB field: `firstSectionProbabilities` (Class (array))
    #[serde(default)]
    pub first_section_probabilities: Vec<Handle<SJumpTunnelSectionProbabilityParams>>,
    /// DCB field: `genParams` (Class (array))
    #[serde(default)]
    pub gen_params: Vec<Handle<SJumpTunnelSectionGenerationParams>>,
}

impl Pooled for SJumpTunnelGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tunnel_radius: inst.get_f32("tunnelRadius").unwrap_or_default(),
            tunnel_radius_gameplay_buffer: inst.get_f32("tunnelRadiusGameplayBuffer").unwrap_or_default(),
            tunnel_length: inst.get_f32("tunnelLength").unwrap_or_default(),
            entrance_length: inst.get_f32("entranceLength").unwrap_or_default(),
            exit_length: inst.get_f32("exitLength").unwrap_or_default(),
            entrance_elliptical_params: match inst.get("entranceEllipticalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exit_elliptical_params: match inst.get("exitEllipticalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            first_section_probabilities: inst.get_array("firstSectionProbabilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            gen_params: inst.get_array("genParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelDistortionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelDistortionParams {
    /// DCB field: `safeAreaRatio` (Single)
    #[serde(default)]
    pub safe_area_ratio: f32,
    /// DCB field: `distortionDamageRate` (Single)
    #[serde(default)]
    pub distortion_damage_rate: f32,
}

impl Pooled for SJumpTunnelDistortionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_distortion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_distortion_params }
}

impl<'a> Extract<'a> for SJumpTunnelDistortionParams {
    const TYPE_NAME: &'static str = "SJumpTunnelDistortionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            safe_area_ratio: inst.get_f32("safeAreaRatio").unwrap_or_default(),
            distortion_damage_rate: inst.get_f32("distortionDamageRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelFailureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelFailureParams {
    /// DCB field: `exitAcceleration` (Single)
    #[serde(default)]
    pub exit_acceleration: f32,
    /// DCB field: `exitMaximumSpeed` (Single)
    #[serde(default)]
    pub exit_maximum_speed: f32,
    /// DCB field: `exitRollAcceleration` (Single)
    #[serde(default)]
    pub exit_roll_acceleration: f32,
    /// DCB field: `exitPitchAcceleration` (Single)
    #[serde(default)]
    pub exit_pitch_acceleration: f32,
    /// DCB field: `exitYawAcceleration` (Single)
    #[serde(default)]
    pub exit_yaw_acceleration: f32,
    /// DCB field: `exitAngularVelocityMaximum` (Single)
    #[serde(default)]
    pub exit_angular_velocity_maximum: f32,
    /// DCB field: `ratioOfMaxDistortionDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_distortion_damage: f32,
    /// DCB field: `ratioOfMaxJDRVWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_jdrvwear_damage: f32,
    /// DCB field: `ratioOfMaxQDRVWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_qdrvwear_damage: f32,
    /// DCB field: `ratioOfMaxHullWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_hull_wear_damage: f32,
    /// DCB field: `teleportRangeOffset` (Class)
    #[serde(default)]
    pub teleport_range_offset: Option<Handle<Range>>,
    /// DCB field: `teleportMaxHeight` (Single)
    #[serde(default)]
    pub teleport_max_height: f32,
}

impl Pooled for SJumpTunnelFailureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_failure_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_failure_params }
}

impl<'a> Extract<'a> for SJumpTunnelFailureParams {
    const TYPE_NAME: &'static str = "SJumpTunnelFailureParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exit_acceleration: inst.get_f32("exitAcceleration").unwrap_or_default(),
            exit_maximum_speed: inst.get_f32("exitMaximumSpeed").unwrap_or_default(),
            exit_roll_acceleration: inst.get_f32("exitRollAcceleration").unwrap_or_default(),
            exit_pitch_acceleration: inst.get_f32("exitPitchAcceleration").unwrap_or_default(),
            exit_yaw_acceleration: inst.get_f32("exitYawAcceleration").unwrap_or_default(),
            exit_angular_velocity_maximum: inst.get_f32("exitAngularVelocityMaximum").unwrap_or_default(),
            ratio_of_max_distortion_damage: inst.get_f32("ratioOfMaxDistortionDamage").unwrap_or_default(),
            ratio_of_max_jdrvwear_damage: inst.get_f32("ratioOfMaxJDRVWearDamage").unwrap_or_default(),
            ratio_of_max_qdrvwear_damage: inst.get_f32("ratioOfMaxQDRVWearDamage").unwrap_or_default(),
            ratio_of_max_hull_wear_damage: inst.get_f32("ratioOfMaxHullWearDamage").unwrap_or_default(),
            teleport_range_offset: match inst.get("teleportRangeOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            teleport_max_height: inst.get_f32("teleportMaxHeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelExitParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelExitParams {
    /// DCB field: `defaultDistanceRange` (Class)
    #[serde(default)]
    pub default_distance_range: Option<Handle<Range>>,
    /// DCB field: `defaultMaxHeight` (Single)
    #[serde(default)]
    pub default_max_height: f32,
    /// DCB field: `exitPushArea` (Class)
    #[serde(default)]
    pub exit_push_area: Option<Handle<SJumpPointPushAreaParams>>,
}

impl Pooled for SJumpTunnelExitParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_tunnel_exit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_tunnel_exit_params }
}

impl<'a> Extract<'a> for SJumpTunnelExitParams {
    const TYPE_NAME: &'static str = "SJumpTunnelExitParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_distance_range: match inst.get("defaultDistanceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_max_height: inst.get_f32("defaultMaxHeight").unwrap_or_default(),
            exit_push_area: match inst.get("exitPushArea") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpPointPushAreaParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpPointPushAreaParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpDriveFlightRotationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightRotationParams {
    /// DCB field: `pitchYawLimiterType` (EnumChoice)
    #[serde(default)]
    pub pitch_yaw_limiter_type: String,
    /// DCB field: `maxAngularVelocity` (Class)
    #[serde(default)]
    pub max_angular_velocity: Option<Handle<Vec3>>,
    /// DCB field: `maxAngularAccelerationPositive` (Class)
    #[serde(default)]
    pub max_angular_acceleration_positive: Option<Handle<Vec3>>,
    /// DCB field: `maxAngularAccelerationNegative` (Class)
    #[serde(default)]
    pub max_angular_acceleration_negative: Option<Handle<Vec3>>,
    /// DCB field: `angularAccelerationDecay` (Single)
    #[serde(default)]
    pub angular_acceleration_decay: f32,
    /// DCB field: `useAngularInputs` (Boolean)
    #[serde(default)]
    pub use_angular_inputs: bool,
    /// DCB field: `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
}

impl Pooled for SJumpDriveFlightRotationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_drive_flight_rotation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_drive_flight_rotation_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightRotationParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightRotationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pitch_yaw_limiter_type: inst.get_str("pitchYawLimiterType").map(String::from).unwrap_or_default(),
            max_angular_velocity: match inst.get("maxAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration_positive: match inst.get("maxAngularAccelerationPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration_negative: match inst.get("maxAngularAccelerationNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angular_acceleration_decay: inst.get_f32("angularAccelerationDecay").unwrap_or_default(),
            use_angular_inputs: inst.get_bool("useAngularInputs").unwrap_or_default(),
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightLinearParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightLinearParams {
    /// DCB field: `linearLimiterType` (EnumChoice)
    #[serde(default)]
    pub linear_limiter_type: String,
    /// DCB field: `linearAccelerationPositive` (Class)
    #[serde(default)]
    pub linear_acceleration_positive: Option<Handle<Vec3>>,
    /// DCB field: `linearAccelerationNegative` (Class)
    #[serde(default)]
    pub linear_acceleration_negative: Option<Handle<Vec3>>,
    /// DCB field: `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
    /// DCB field: `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
    /// DCB field: `maxStrafeInputs` (Single)
    #[serde(default)]
    pub max_strafe_inputs: f32,
    /// DCB field: `linearAccelDecay` (Single)
    #[serde(default)]
    pub linear_accel_decay: f32,
    /// DCB field: `useLinearInputs` (Boolean)
    #[serde(default)]
    pub use_linear_inputs: bool,
}

impl Pooled for SJumpDriveFlightLinearParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_drive_flight_linear_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_drive_flight_linear_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightLinearParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightLinearParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            linear_limiter_type: inst.get_str("linearLimiterType").map(String::from).unwrap_or_default(),
            linear_acceleration_positive: match inst.get("linearAccelerationPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_acceleration_negative: match inst.get("linearAccelerationNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            max_strafe_inputs: inst.get_f32("maxStrafeInputs").unwrap_or_default(),
            linear_accel_decay: inst.get_f32("linearAccelDecay").unwrap_or_default(),
            use_linear_inputs: inst.get_bool("useLinearInputs").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightSteeringParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightSteeringParams {
    /// DCB field: `maxAcceleration` (Single)
    #[serde(default)]
    pub max_acceleration: f32,
    /// DCB field: `modifierCurve` (Class)
    #[serde(default)]
    pub modifier_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `maxAccelerationBoost` (Single)
    #[serde(default)]
    pub max_acceleration_boost: f32,
    /// DCB field: `modifierCurveBoost` (Class)
    #[serde(default)]
    pub modifier_curve_boost: Option<Handle<BezierCurve>>,
    /// DCB field: `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
    /// DCB field: `useSteeringParams` (Boolean)
    #[serde(default)]
    pub use_steering_params: bool,
}

impl Pooled for SJumpDriveFlightSteeringParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_drive_flight_steering_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_drive_flight_steering_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightSteeringParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightSteeringParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_acceleration: inst.get_f32("maxAcceleration").unwrap_or_default(),
            modifier_curve: match inst.get("modifierCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_acceleration_boost: inst.get_f32("maxAccelerationBoost").unwrap_or_default(),
            modifier_curve_boost: match inst.get("modifierCurveBoost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
            use_steering_params: inst.get_bool("useSteeringParams").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightTurbulenceNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightTurbulenceNoiseParams {
    /// DCB field: `frequencies` (Class)
    #[serde(default)]
    pub frequencies: Option<Handle<Vec3>>,
    /// DCB field: `multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
    /// DCB field: `hurstIndex` (Single)
    #[serde(default)]
    pub hurst_index: f32,
    /// DCB field: `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
}

impl Pooled for SJumpDriveFlightTurbulenceNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_drive_flight_turbulence_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_drive_flight_turbulence_noise_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightTurbulenceNoiseParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightTurbulenceNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequencies: match inst.get("frequencies") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            multiplier: inst.get_f32("multiplier").unwrap_or_default(),
            hurst_index: inst.get_f32("hurstIndex").unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightTurbulenceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightTurbulenceParams {
    /// DCB field: `noiseTurbulence` (Class)
    #[serde(default)]
    pub noise_turbulence: Option<Handle<SJumpDriveFlightTurbulenceNoiseParams>>,
    /// DCB field: `noiseGust` (Class)
    #[serde(default)]
    pub noise_gust: Option<Handle<SJumpDriveFlightTurbulenceNoiseParams>>,
    /// DCB field: `turbulenceByDistanceToSpline` (Class)
    #[serde(default)]
    pub turbulence_by_distance_to_spline: Option<Handle<BezierCurve>>,
    /// DCB field: `gustByDistanceToSpline` (Class)
    #[serde(default)]
    pub gust_by_distance_to_spline: Option<Handle<BezierCurve>>,
    /// DCB field: `pitchYawLimiterType` (EnumChoice)
    #[serde(default)]
    pub pitch_yaw_limiter_type: String,
    /// DCB field: `maxAngularVelocity` (Class)
    #[serde(default)]
    pub max_angular_velocity: Option<Handle<Vec3>>,
    /// DCB field: `maxAngularAcceleration` (Class)
    #[serde(default)]
    pub max_angular_acceleration: Option<Handle<Vec3>>,
    /// DCB field: `useTurbulenceParams` (Boolean)
    #[serde(default)]
    pub use_turbulence_params: bool,
    /// DCB field: `useGustParams` (Boolean)
    #[serde(default)]
    pub use_gust_params: bool,
}

impl Pooled for SJumpDriveFlightTurbulenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sj.sjump_drive_flight_turbulence_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sj.sjump_drive_flight_turbulence_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightTurbulenceParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightTurbulenceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            noise_turbulence: match inst.get("noiseTurbulence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_gust: match inst.get("noiseGust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence_by_distance_to_spline: match inst.get("turbulenceByDistanceToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gust_by_distance_to_spline: match inst.get("gustByDistanceToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch_yaw_limiter_type: inst.get_str("pitchYawLimiterType").map(String::from).unwrap_or_default(),
            max_angular_velocity: match inst.get("maxAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration: match inst.get("maxAngularAcceleration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_turbulence_params: inst.get_bool("useTurbulenceParams").unwrap_or_default(),
            use_gust_params: inst.get_bool("useGustParams").unwrap_or_default(),
        }
    }
}

