// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `jumppoints`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SJumpTunnelVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelVisualParams {
    /// `quantumEffect` (Boolean)
    #[serde(default)]
    pub quantum_effect: bool,
    /// `portalExternalViewDistRatio` (Single)
    #[serde(default)]
    pub portal_external_view_dist_ratio: f32,
    /// `portalInternalViewDistRatio` (Single)
    #[serde(default)]
    pub portal_internal_view_dist_ratio: f32,
    /// `visAreaViewDistRatio` (Single)
    #[serde(default)]
    pub vis_area_view_dist_ratio: f32,
    /// `probeIntensity` (Single)
    #[serde(default)]
    pub probe_intensity: f32,
    /// `internalDrawDistance` (Single)
    #[serde(default)]
    pub internal_draw_distance: f32,
    /// `externalDrawDistance` (Single)
    #[serde(default)]
    pub external_draw_distance: f32,
    /// `openingHiddenTime` (Single)
    #[serde(default)]
    pub opening_hidden_time: f32,
    /// `closingHiddenTime` (Single)
    #[serde(default)]
    pub closing_hidden_time: f32,
    /// `exitPortalCullingDistance` (Single)
    #[serde(default)]
    pub exit_portal_culling_distance: f32,
}

impl Pooled for SJumpTunnelVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_visual_params }
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

/// DCB type: `SSpreadMisfireEffect`
/// Inherits from: `SMisfireEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpreadMisfireEffect {
    /// `effectTrigger` (Reference)
    #[serde(default)]
    pub effect_trigger: Option<CigGuid>,
    /// `effectTag` (Reference)
    #[serde(default)]
    pub effect_tag: Option<CigGuid>,
}

impl Pooled for SSpreadMisfireEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sspread_misfire_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sspread_misfire_effect }
}

impl<'a> Extract<'a> for SSpreadMisfireEffect {
    const TYPE_NAME: &'static str = "SSpreadMisfireEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_trigger: inst.get("effectTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            effect_tag: inst.get("effectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `GlobalJumpPointTuningParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointTuningParams {
    /// `requiredTuningAmount` (Single)
    #[serde(default)]
    pub required_tuning_amount: f32,
}

impl Pooled for GlobalJumpPointTuningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_point_tuning_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_point_tuning_params }
}

impl<'a> Extract<'a> for GlobalJumpPointTuningParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointTuningParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_tuning_amount: inst.get_f32("requiredTuningAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointOpeningParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointOpeningParams {
    /// `projectileTime` (Single)
    #[serde(default)]
    pub projectile_time: f32,
    /// `tuningCollapseTime` (Single)
    #[serde(default)]
    pub tuning_collapse_time: f32,
    /// `openEffectStartDelay` (Single)
    #[serde(default)]
    pub open_effect_start_delay: f32,
    /// `revealTime` (Single)
    #[serde(default)]
    pub reveal_time: f32,
    /// `revealAnimCurve` (Class)
    #[serde(default)]
    pub reveal_anim_curve: Option<Handle<BezierCurve>>,
    /// `revealFadeDelay` (Single)
    #[serde(default)]
    pub reveal_fade_delay: f32,
    /// `openingTime` (Single)
    #[serde(default)]
    pub opening_time: f32,
    /// `openingEndDelay` (Single)
    #[serde(default)]
    pub opening_end_delay: f32,
    /// `apertureTimeScaleRange` (Class)
    #[serde(default)]
    pub aperture_time_scale_range: Option<Handle<Range>>,
}

impl Pooled for GlobalJumpPointOpeningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_point_opening_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_point_opening_params }
}

impl<'a> Extract<'a> for GlobalJumpPointOpeningParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointOpeningParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            projectile_time: inst.get_f32("projectileTime").unwrap_or_default(),
            tuning_collapse_time: inst.get_f32("tuningCollapseTime").unwrap_or_default(),
            open_effect_start_delay: inst.get_f32("openEffectStartDelay").unwrap_or_default(),
            reveal_time: inst.get_f32("revealTime").unwrap_or_default(),
            reveal_anim_curve: match inst.get("revealAnimCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            reveal_fade_delay: inst.get_f32("revealFadeDelay").unwrap_or_default(),
            opening_time: inst.get_f32("openingTime").unwrap_or_default(),
            opening_end_delay: inst.get_f32("openingEndDelay").unwrap_or_default(),
            aperture_time_scale_range: match inst.get("apertureTimeScaleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpPointClosingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointClosingParams {
    /// `closingTime` (Single)
    #[serde(default)]
    pub closing_time: f32,
    /// `shrinkTime` (Single)
    #[serde(default)]
    pub shrink_time: f32,
    /// `closingTriggerDelay` (Single)
    #[serde(default)]
    pub closing_trigger_delay: f32,
}

impl Pooled for GlobalJumpPointClosingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_point_closing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_point_closing_params }
}

impl<'a> Extract<'a> for GlobalJumpPointClosingParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointClosingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            closing_time: inst.get_f32("closingTime").unwrap_or_default(),
            shrink_time: inst.get_f32("shrinkTime").unwrap_or_default(),
            closing_trigger_delay: inst.get_f32("closingTriggerDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointEffectParams {
    /// `tuningParams` (Class)
    #[serde(default)]
    pub tuning_params: Option<Handle<GlobalJumpPointTuningParams>>,
    /// `openingParams` (Class)
    #[serde(default)]
    pub opening_params: Option<Handle<GlobalJumpPointOpeningParams>>,
    /// `closingParams` (Class)
    #[serde(default)]
    pub closing_params: Option<Handle<GlobalJumpPointClosingParams>>,
}

impl Pooled for GlobalJumpPointEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_point_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_point_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpPointEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tuning_params: match inst.get("tuningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointTuningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            opening_params: match inst.get("openingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointOpeningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            closing_params: match inst.get("closingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointClosingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveUIConeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveUIConeParams {
    /// `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// `angle` (Single)
    #[serde(default)]
    pub angle: f32,
    /// `lookAtAngle` (Single)
    #[serde(default)]
    pub look_at_angle: f32,
}

impl Pooled for JumpDriveUIConeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_uicone_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_uicone_params }
}

impl<'a> Extract<'a> for JumpDriveUIConeParams {
    const TYPE_NAME: &'static str = "JumpDriveUIConeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            range: inst.get_f32("range").unwrap_or_default(),
            angle: inst.get_f32("angle").unwrap_or_default(),
            look_at_angle: inst.get_f32("lookAtAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointParams {
    /// `alignmentAngle` (Single)
    #[serde(default)]
    pub alignment_angle: f32,
    /// `alignmentTolerance` (Single)
    #[serde(default)]
    pub alignment_tolerance: f32,
    /// `alignmentRange` (Single)
    #[serde(default)]
    pub alignment_range: f32,
    /// `startClosingTime` (Single)
    #[serde(default)]
    pub start_closing_time: f32,
    /// `shipPullInDelayTime` (Single)
    #[serde(default)]
    pub ship_pull_in_delay_time: f32,
    /// `shipPullInBufferTime` (Single)
    #[serde(default)]
    pub ship_pull_in_buffer_time: f32,
    /// `debrisPushOutAcceleration` (Single)
    #[serde(default)]
    pub debris_push_out_acceleration: f32,
    /// `debrisPushOutMaximumSpeed` (Single)
    #[serde(default)]
    pub debris_push_out_maximum_speed: f32,
    /// `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpPointEffectParams>>,
    /// `uiConeParams` (Class)
    #[serde(default)]
    pub ui_cone_params: Option<Handle<JumpDriveUIConeParams>>,
}

impl Pooled for GlobalJumpPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_point_params }
}

impl<'a> Extract<'a> for GlobalJumpPointParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            alignment_angle: inst.get_f32("alignmentAngle").unwrap_or_default(),
            alignment_tolerance: inst.get_f32("alignmentTolerance").unwrap_or_default(),
            alignment_range: inst.get_f32("alignmentRange").unwrap_or_default(),
            start_closing_time: inst.get_f32("startClosingTime").unwrap_or_default(),
            ship_pull_in_delay_time: inst.get_f32("shipPullInDelayTime").unwrap_or_default(),
            ship_pull_in_buffer_time: inst.get_f32("shipPullInBufferTime").unwrap_or_default(),
            debris_push_out_acceleration: inst.get_f32("debrisPushOutAcceleration").unwrap_or_default(),
            debris_push_out_maximum_speed: inst.get_f32("debrisPushOutMaximumSpeed").unwrap_or_default(),
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ui_cone_params: match inst.get("uiConeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveUIConeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelSectionProbabilityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionProbabilityParams {
    /// `section` (WeakPointer)
    #[serde(default)]
    pub section: Option<Handle<SJumpTunnelSectionGenerationParams>>,
    /// `probability` (Single)
    #[serde(default)]
    pub probability: f32,
}

impl Pooled for SJumpTunnelSectionProbabilityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_section_probability_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_section_probability_params }
}

impl<'a> Extract<'a> for SJumpTunnelSectionProbabilityParams {
    const TYPE_NAME: &'static str = "SJumpTunnelSectionProbabilityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            section: match inst.get("section") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            probability: inst.get_f32("probability").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelSectionControlPointGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionControlPointGenerationParams {
    /// `sectionOffset` (Class)
    #[serde(default)]
    pub section_offset: Option<Handle<Range>>,
    /// `sectionLength` (Class)
    #[serde(default)]
    pub section_length: Option<Handle<Range>>,
    /// `angleOffset` (Class)
    #[serde(default)]
    pub angle_offset: Option<Handle<Range>>,
}

impl Pooled for SJumpTunnelSectionControlPointGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_section_control_point_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_section_control_point_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelSectionControlPointGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelSectionControlPointGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            section_offset: match inst.get("sectionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            section_length: match inst.get("sectionLength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            angle_offset: match inst.get("angleOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelObstacleGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelObstacleGenerationParams {
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Range>>,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Range>>,
    /// `angle` (Class)
    #[serde(default)]
    pub angle: Option<Handle<Range>>,
}

impl Pooled for SJumpTunnelObstacleGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_obstacle_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_obstacle_generation_params }
}

impl<'a> Extract<'a> for SJumpTunnelObstacleGenerationParams {
    const TYPE_NAME: &'static str = "SJumpTunnelObstacleGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            angle: match inst.get("angle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelEllipticalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelEllipticalParams {
    /// `minRadius` (Single)
    #[serde(default)]
    pub min_radius: f32,
    /// `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// `radiusMultiplier` (Class)
    #[serde(default)]
    pub radius_multiplier: Option<Handle<BezierCurve>>,
    /// `minAspectRatio` (Single)
    #[serde(default)]
    pub min_aspect_ratio: f32,
    /// `maxAspectRatio` (Single)
    #[serde(default)]
    pub max_aspect_ratio: f32,
    /// `aspectRatioMultiplier` (Class)
    #[serde(default)]
    pub aspect_ratio_multiplier: Option<Handle<BezierCurve>>,
    /// `chanceToFollowCurvature` (Single)
    #[serde(default)]
    pub chance_to_follow_curvature: f32,
}

impl Pooled for SJumpTunnelEllipticalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_elliptical_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_elliptical_params }
}

impl<'a> Extract<'a> for SJumpTunnelEllipticalParams {
    const TYPE_NAME: &'static str = "SJumpTunnelEllipticalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_radius: inst.get_f32("minRadius").unwrap_or_default(),
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
            radius_multiplier: match inst.get("radiusMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            min_aspect_ratio: inst.get_f32("minAspectRatio").unwrap_or_default(),
            max_aspect_ratio: inst.get_f32("maxAspectRatio").unwrap_or_default(),
            aspect_ratio_multiplier: match inst.get("aspectRatioMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chance_to_follow_curvature: inst.get_f32("chanceToFollowCurvature").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelSectionGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelSectionGenerationParams {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `defaultSectionLength` (Single)
    #[serde(default)]
    pub default_section_length: f32,
    /// `nextSectionProbabilities` (Class (array))
    #[serde(default)]
    pub next_section_probabilities: Vec<Handle<SJumpTunnelSectionProbabilityParams>>,
    /// `controlPoints` (Class (array))
    #[serde(default)]
    pub control_points: Vec<Handle<SJumpTunnelSectionControlPointGenerationParams>>,
    /// `chanceOfObstacles` (Single)
    #[serde(default)]
    pub chance_of_obstacles: f32,
    /// `numberOfObstacles` (Class)
    #[serde(default)]
    pub number_of_obstacles: Option<Handle<Range>>,
    /// `obstacleGeneration` (Class)
    #[serde(default)]
    pub obstacle_generation: Option<Handle<SJumpTunnelObstacleGenerationParams>>,
    /// `ellipticalParams` (Class)
    #[serde(default)]
    pub elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
}

impl Pooled for SJumpTunnelSectionGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_section_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_section_generation_params }
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            control_points: inst.get_array("controlPoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionControlPointGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SJumpTunnelSectionControlPointGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            chance_of_obstacles: inst.get_f32("chanceOfObstacles").unwrap_or_default(),
            number_of_obstacles: match inst.get("numberOfObstacles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            obstacle_generation: match inst.get("obstacleGeneration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelObstacleGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            elliptical_params: match inst.get("ellipticalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelGenerationParams {
    /// `tunnelRadius` (Single)
    #[serde(default)]
    pub tunnel_radius: f32,
    /// `tunnelRadiusGameplayBuffer` (Single)
    #[serde(default)]
    pub tunnel_radius_gameplay_buffer: f32,
    /// `tunnelLength` (Single)
    #[serde(default)]
    pub tunnel_length: f32,
    /// `entranceLength` (Single)
    #[serde(default)]
    pub entrance_length: f32,
    /// `exitLength` (Single)
    #[serde(default)]
    pub exit_length: f32,
    /// `entranceEllipticalParams` (Class)
    #[serde(default)]
    pub entrance_elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
    /// `exitEllipticalParams` (Class)
    #[serde(default)]
    pub exit_elliptical_params: Option<Handle<SJumpTunnelEllipticalParams>>,
    /// `firstSectionProbabilities` (Class (array))
    #[serde(default)]
    pub first_section_probabilities: Vec<Handle<SJumpTunnelSectionProbabilityParams>>,
    /// `genParams` (Class (array))
    #[serde(default)]
    pub gen_params: Vec<Handle<SJumpTunnelSectionGenerationParams>>,
}

impl Pooled for SJumpTunnelGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_generation_params }
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
                _ => None,
            },
            exit_elliptical_params: match inst.get("exitEllipticalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelEllipticalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            first_section_probabilities: inst.get_array("firstSectionProbabilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SJumpTunnelSectionProbabilityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            gen_params: inst.get_array("genParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SJumpTunnelSectionGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelLightParams {
    /// `distanceAhead` (Single)
    #[serde(default)]
    pub distance_ahead: f32,
    /// `intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `bulbRadius` (Single)
    #[serde(default)]
    pub bulb_radius: f32,
    /// `animSpeed` (Single)
    #[serde(default)]
    pub anim_speed: f32,
    /// `lightStyle` (Byte)
    #[serde(default)]
    pub light_style: u32,
}

impl Pooled for GlobalJumpTunnelLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_light_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelLightParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelLightParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            distance_ahead: inst.get_f32("distanceAhead").unwrap_or_default(),
            intensity: inst.get_f32("intensity").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            bulb_radius: inst.get_f32("bulbRadius").unwrap_or_default(),
            anim_speed: inst.get_f32("animSpeed").unwrap_or_default(),
            light_style: inst.get_u32("lightStyle").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelFogParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelFogParams {
    /// `fogStartOffset` (Single)
    #[serde(default)]
    pub fog_start_offset: f32,
    /// `fogEndOffset` (Single)
    #[serde(default)]
    pub fog_end_offset: f32,
    /// `fogEndIntensityDistanceRange` (Class)
    #[serde(default)]
    pub fog_end_intensity_distance_range: Option<Handle<Range>>,
    /// `fogAnimatedIntensityScaleRange` (Class)
    #[serde(default)]
    pub fog_animated_intensity_scale_range: Option<Handle<Range>>,
}

impl Pooled for GlobalJumpTunnelFogParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_fog_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_fog_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelFogParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelFogParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fog_start_offset: inst.get_f32("fogStartOffset").unwrap_or_default(),
            fog_end_offset: inst.get_f32("fogEndOffset").unwrap_or_default(),
            fog_end_intensity_distance_range: match inst.get("fogEndIntensityDistanceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fog_animated_intensity_scale_range: match inst.get("fogAnimatedIntensityScaleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpTunnelPassByLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelPassByLightParams {
    /// `intensityRange` (Class)
    #[serde(default)]
    pub intensity_range: Option<Handle<Range>>,
    /// `radiusRange` (Class)
    #[serde(default)]
    pub radius_range: Option<Handle<Range>>,
    /// `bulbRange` (Class)
    #[serde(default)]
    pub bulb_range: Option<Handle<Range>>,
    /// `spacingRange` (Class)
    #[serde(default)]
    pub spacing_range: Option<Handle<Range>>,
    /// `distanceFromSpline` (Class)
    #[serde(default)]
    pub distance_from_spline: Option<Handle<Range>>,
    /// `speedRange` (Class)
    #[serde(default)]
    pub speed_range: Option<Handle<Range>>,
    /// `colorRandomOffsetRange` (Class)
    #[serde(default)]
    pub color_random_offset_range: Option<Handle<Range>>,
    /// `entranceOffset` (Single)
    #[serde(default)]
    pub entrance_offset: f32,
    /// `spawnChance` (Single)
    #[serde(default)]
    pub spawn_chance: f32,
    /// `maxRange` (Single)
    #[serde(default)]
    pub max_range: f32,
    /// `fadeOutDistStart` (Single)
    #[serde(default)]
    pub fade_out_dist_start: f32,
}

impl Pooled for GlobalJumpTunnelPassByLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_pass_by_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_pass_by_light_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelPassByLightParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelPassByLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            intensity_range: match inst.get("intensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radius_range: match inst.get("radiusRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bulb_range: match inst.get("bulbRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spacing_range: match inst.get("spacingRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_from_spline: match inst.get("distanceFromSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_range: match inst.get("speedRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            color_random_offset_range: match inst.get("colorRandomOffsetRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entrance_offset: inst.get_f32("entranceOffset").unwrap_or_default(),
            spawn_chance: inst.get_f32("spawnChance").unwrap_or_default(),
            max_range: inst.get_f32("maxRange").unwrap_or_default(),
            fade_out_dist_start: inst.get_f32("fadeOutDistStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelProbeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelProbeParams {
    /// `probeRadius` (Single)
    #[serde(default)]
    pub probe_radius: f32,
}

impl Pooled for GlobalJumpTunnelProbeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_probe_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_probe_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelProbeParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelProbeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            probe_radius: inst.get_f32("probeRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelEffectParams {
    /// `failEffect` (Class)
    #[serde(default)]
    pub fail_effect: Option<Handle<GlobalResourceParticle>>,
    /// `interiorExitEffect` (Class)
    #[serde(default)]
    pub interior_exit_effect: Option<Handle<GlobalResourceParticle>>,
    /// `spaceloopEffect` (Class)
    #[serde(default)]
    pub spaceloop_effect: Option<Handle<GlobalResourceParticle>>,
    /// `centralSplineEffect` (Class)
    #[serde(default)]
    pub central_spline_effect: Option<Handle<GlobalResourceParticle>>,
    /// `spaceFillingSplineEffect` (Class)
    #[serde(default)]
    pub space_filling_spline_effect: Option<Handle<GlobalResourceParticle>>,
    /// `sunFlareEffect` (Class)
    #[serde(default)]
    pub sun_flare_effect: Option<Handle<GlobalResourceParticle>>,
    /// `sunLightParams` (Class)
    #[serde(default)]
    pub sun_light_params: Option<Handle<GlobalJumpTunnelLightParams>>,
    /// `probeParams` (Class)
    #[serde(default)]
    pub probe_params: Option<Handle<GlobalJumpTunnelProbeParams>>,
    /// `fogParams` (Class)
    #[serde(default)]
    pub fog_params: Option<Handle<GlobalJumpTunnelFogParams>>,
    /// `passByLightParams` (Class)
    #[serde(default)]
    pub pass_by_light_params: Option<Handle<GlobalJumpTunnelPassByLightParams>>,
    /// `splineLength` (Single)
    #[serde(default)]
    pub spline_length: f32,
}

impl Pooled for GlobalJumpTunnelEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fail_effect: match inst.get("failEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interior_exit_effect: match inst.get("interiorExitEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spaceloop_effect: match inst.get("spaceloopEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            central_spline_effect: match inst.get("centralSplineEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            space_filling_spline_effect: match inst.get("spaceFillingSplineEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sun_flare_effect: match inst.get("sunFlareEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sun_light_params: match inst.get("sunLightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            probe_params: match inst.get("probeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelProbeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fog_params: match inst.get("fogParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelFogParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            pass_by_light_params: match inst.get("passByLightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelPassByLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spline_length: inst.get_f32("splineLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelCameraEffectParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelCameraEffectParam {
    /// `referenceValue` (Single)
    #[serde(default)]
    pub reference_value: f32,
}

impl Pooled for JumpTunnelCameraEffectParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_tunnel_camera_effect_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_tunnel_camera_effect_param }
}

impl<'a> Extract<'a> for JumpTunnelCameraEffectParam {
    const TYPE_NAME: &'static str = "JumpTunnelCameraEffectParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reference_value: inst.get_f32("referenceValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelCameraEffects {
    /// `blur` (Class)
    #[serde(default)]
    pub blur: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// `bloom` (Class)
    #[serde(default)]
    pub bloom: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// `chromaticAberation` (Class)
    #[serde(default)]
    pub chromatic_aberation: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// `shutterSpeed` (Class)
    #[serde(default)]
    pub shutter_speed: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// `fov` (Class)
    #[serde(default)]
    pub fov: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// `fovScale` (Class)
    #[serde(default)]
    pub fov_scale: Option<Handle<JumpTunnelCameraEffectParam>>,
}

impl Pooled for JumpTunnelCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_tunnel_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_tunnel_camera_effects }
}

impl<'a> Extract<'a> for JumpTunnelCameraEffects {
    const TYPE_NAME: &'static str = "JumpTunnelCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blur: match inst.get("blur") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bloom: match inst.get("bloom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            chromatic_aberation: match inst.get("chromaticAberation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            shutter_speed: match inst.get("shutterSpeed") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fov: match inst.get("fov") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fov_scale: match inst.get("fovScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveVelocityStrengthParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveVelocityStrengthParams {
    /// `maxStrengthAngle` (Single)
    #[serde(default)]
    pub max_strength_angle: f32,
    /// `minStrengthAngle` (Single)
    #[serde(default)]
    pub min_strength_angle: f32,
}

impl Pooled for JumpDriveVelocityStrengthParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_velocity_strength_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_velocity_strength_params }
}

impl<'a> Extract<'a> for JumpDriveVelocityStrengthParams {
    const TYPE_NAME: &'static str = "JumpDriveVelocityStrengthParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_strength_angle: inst.get_f32("maxStrengthAngle").unwrap_or_default(),
            min_strength_angle: inst.get_f32("minStrengthAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelCameraEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelCameraEffectParams {
    /// `alignmentToSpline` (Class)
    #[serde(default)]
    pub alignment_to_spline: Option<Handle<JumpTunnelCameraEffects>>,
    /// `distortionRatio` (Class)
    #[serde(default)]
    pub distortion_ratio: Option<Handle<JumpTunnelCameraEffects>>,
    /// `openingProximity` (Class)
    #[serde(default)]
    pub opening_proximity: Option<Handle<JumpTunnelCameraEffects>>,
    /// `wallProximity` (Class)
    #[serde(default)]
    pub wall_proximity: Option<Handle<JumpTunnelCameraEffects>>,
    /// `failureState` (Class)
    #[serde(default)]
    pub failure_state: Option<Handle<JumpTunnelCameraEffects>>,
    /// `velocityStrength` (Class)
    #[serde(default)]
    pub velocity_strength: Option<Handle<JumpTunnelCameraEffects>>,
    /// `velocityStrengthParams` (Class)
    #[serde(default)]
    pub velocity_strength_params: Option<Handle<JumpDriveVelocityStrengthParams>>,
}

impl Pooled for GlobalJumpTunnelCameraEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_camera_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_camera_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelCameraEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelCameraEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            alignment_to_spline: match inst.get("alignmentToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distortion_ratio: match inst.get("distortionRatio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            opening_proximity: match inst.get("openingProximity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            wall_proximity: match inst.get("wallProximity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            failure_state: match inst.get("failureState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            velocity_strength: match inst.get("velocityStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            velocity_strength_params: match inst.get("velocityStrengthParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveVelocityStrengthParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpTunnelDistortionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelDistortionParams {
    /// `safeAreaRatio` (Single)
    #[serde(default)]
    pub safe_area_ratio: f32,
    /// `distortionDamageRate` (Single)
    #[serde(default)]
    pub distortion_damage_rate: f32,
}

impl Pooled for SJumpTunnelDistortionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_distortion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_distortion_params }
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
    /// `exitAcceleration` (Single)
    #[serde(default)]
    pub exit_acceleration: f32,
    /// `exitMaximumSpeed` (Single)
    #[serde(default)]
    pub exit_maximum_speed: f32,
    /// `exitRollAcceleration` (Single)
    #[serde(default)]
    pub exit_roll_acceleration: f32,
    /// `exitPitchAcceleration` (Single)
    #[serde(default)]
    pub exit_pitch_acceleration: f32,
    /// `exitYawAcceleration` (Single)
    #[serde(default)]
    pub exit_yaw_acceleration: f32,
    /// `exitAngularVelocityMaximum` (Single)
    #[serde(default)]
    pub exit_angular_velocity_maximum: f32,
    /// `ratioOfMaxDistortionDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_distortion_damage: f32,
    /// `ratioOfMaxJDRVWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_jdrvwear_damage: f32,
    /// `ratioOfMaxQDRVWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_qdrvwear_damage: f32,
    /// `ratioOfMaxHullWearDamage` (Single)
    #[serde(default)]
    pub ratio_of_max_hull_wear_damage: f32,
    /// `teleportRangeOffset` (Class)
    #[serde(default)]
    pub teleport_range_offset: Option<Handle<Range>>,
    /// `teleportMaxHeight` (Single)
    #[serde(default)]
    pub teleport_max_height: f32,
}

impl Pooled for SJumpTunnelFailureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_failure_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_failure_params }
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
                _ => None,
            },
            teleport_max_height: inst.get_f32("teleportMaxHeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpTunnelExitParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelExitParams {
    /// `defaultDistanceRange` (Class)
    #[serde(default)]
    pub default_distance_range: Option<Handle<Range>>,
    /// `defaultMaxHeight` (Single)
    #[serde(default)]
    pub default_max_height: f32,
    /// `exitPushArea` (Class)
    #[serde(default)]
    pub exit_push_area: Option<Handle<SJumpPointPushAreaParams>>,
}

impl Pooled for SJumpTunnelExitParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.sjump_tunnel_exit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.sjump_tunnel_exit_params }
}

impl<'a> Extract<'a> for SJumpTunnelExitParams {
    const TYPE_NAME: &'static str = "SJumpTunnelExitParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_distance_range: match inst.get("defaultDistanceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_max_height: inst.get_f32("defaultMaxHeight").unwrap_or_default(),
            exit_push_area: match inst.get("exitPushArea") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpPointPushAreaParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpTunnelHostParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelHostParams {
    /// `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// `tunnelGenerationParams` (Class)
    #[serde(default)]
    pub tunnel_generation_params: Option<Handle<SJumpTunnelGenerationParams>>,
    /// `tunnelDistortionParams` (Class)
    #[serde(default)]
    pub tunnel_distortion_params: Option<Handle<SJumpTunnelDistortionParams>>,
    /// `tunnelFailureParams` (Class)
    #[serde(default)]
    pub tunnel_failure_params: Option<Handle<SJumpTunnelFailureParams>>,
    /// `tunnelExitParams` (Class)
    #[serde(default)]
    pub tunnel_exit_params: Option<Handle<SJumpTunnelExitParams>>,
    /// `visualParams` (Class)
    #[serde(default)]
    pub visual_params: Option<Handle<SJumpTunnelVisualParams>>,
    /// `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpTunnelEffectParams>>,
    /// `entityPullInAcceleration` (Single)
    #[serde(default)]
    pub entity_pull_in_acceleration: f32,
}

impl Pooled for GlobalJumpTunnelHostParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_tunnel_host_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_tunnel_host_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelHostParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelHostParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_generation_params: match inst.get("tunnelGenerationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_distortion_params: match inst.get("tunnelDistortionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelDistortionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_failure_params: match inst.get("tunnelFailureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelFailureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_exit_params: match inst.get("tunnelExitParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelExitParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            visual_params: match inst.get("visualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entity_pull_in_acceleration: inst.get_f32("entityPullInAcceleration").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveStateAudioMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveStateAudioMap {
    /// `enterStateLoop` (Class)
    #[serde(default)]
    pub enter_state_loop: Option<Handle<GlobalResourceAudio>>,
    /// `enterStateOneShot` (Class)
    #[serde(default)]
    pub enter_state_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// `exitStateLoop` (Class)
    #[serde(default)]
    pub exit_state_loop: Option<Handle<GlobalResourceAudio>>,
    /// `jumpDriveState` (EnumChoice)
    #[serde(default)]
    pub jump_drive_state: ItemJumpDriveState,
}

impl Pooled for JumpDriveStateAudioMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_state_audio_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_state_audio_map }
}

impl<'a> Extract<'a> for JumpDriveStateAudioMap {
    const TYPE_NAME: &'static str = "JumpDriveStateAudioMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enter_state_loop: match inst.get("enterStateLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enter_state_one_shot: match inst.get("enterStateOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exit_state_loop: match inst.get("exitStateLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            jump_drive_state: ItemJumpDriveState::from_dcb_str(inst.get_str("jumpDriveState").unwrap_or("")),
        }
    }
}

/// DCB type: `JumpDriveAudioMovementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveAudioMovementParams {
    /// `shipLinearAccelerationRL` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_rl: Option<Handle<AudioRtpc>>,
    /// `shipLinearAccelerationFB` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_fb: Option<Handle<AudioRtpc>>,
    /// `shipLinearAccelerationUD` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_ud: Option<Handle<AudioRtpc>>,
    /// `shipAngularAccelerationPitch` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_pitch: Option<Handle<AudioRtpc>>,
    /// `shipAngularAccelerationRoll` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_roll: Option<Handle<AudioRtpc>>,
    /// `shipAngularAccelerationYaw` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_yaw: Option<Handle<AudioRtpc>>,
    /// `shipAngularTurbulencePitch` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_pitch: Option<Handle<AudioRtpc>>,
    /// `shipAngularTurbulenceRoll` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_roll: Option<Handle<AudioRtpc>>,
    /// `shipAngularTurbulenceYaw` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_yaw: Option<Handle<AudioRtpc>>,
    /// `shipLinearVelocityRL` (Class)
    #[serde(default)]
    pub ship_linear_velocity_rl: Option<Handle<AudioRtpc>>,
    /// `shipLinearVelocityFB` (Class)
    #[serde(default)]
    pub ship_linear_velocity_fb: Option<Handle<AudioRtpc>>,
    /// `shipLinearVelocityUD` (Class)
    #[serde(default)]
    pub ship_linear_velocity_ud: Option<Handle<AudioRtpc>>,
    /// `tunnelLinearAccelerationRL` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_rl: Option<Handle<AudioRtpc>>,
    /// `tunnelLinearAccelerationFB` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_fb: Option<Handle<AudioRtpc>>,
    /// `tunnelLinearAccelerationUD` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_ud: Option<Handle<AudioRtpc>>,
    /// `tunnelAngularAccelerationPitch` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_pitch: Option<Handle<AudioRtpc>>,
    /// `tunnelAngularAccelerationRoll` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_roll: Option<Handle<AudioRtpc>>,
    /// `tunnelAngularAccelerationYaw` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_yaw: Option<Handle<AudioRtpc>>,
    /// `playerInputPitch` (Class)
    #[serde(default)]
    pub player_input_pitch: Option<Handle<AudioRtpc>>,
    /// `playerInputRoll` (Class)
    #[serde(default)]
    pub player_input_roll: Option<Handle<AudioRtpc>>,
    /// `playerInputYaw` (Class)
    #[serde(default)]
    pub player_input_yaw: Option<Handle<AudioRtpc>>,
    /// `playerInputStrafeRL` (Class)
    #[serde(default)]
    pub player_input_strafe_rl: Option<Handle<AudioRtpc>>,
    /// `playerInputStrafeFB` (Class)
    #[serde(default)]
    pub player_input_strafe_fb: Option<Handle<AudioRtpc>>,
    /// `playerInputStrafeUD` (Class)
    #[serde(default)]
    pub player_input_strafe_ud: Option<Handle<AudioRtpc>>,
    /// `afterburnerRequestedRtpc` (Class)
    #[serde(default)]
    pub afterburner_requested_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for JumpDriveAudioMovementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_audio_movement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_audio_movement_params }
}

impl<'a> Extract<'a> for JumpDriveAudioMovementParams {
    const TYPE_NAME: &'static str = "JumpDriveAudioMovementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ship_linear_acceleration_rl: match inst.get("shipLinearAccelerationRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_linear_acceleration_fb: match inst.get("shipLinearAccelerationFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_linear_acceleration_ud: match inst.get("shipLinearAccelerationUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_acceleration_pitch: match inst.get("shipAngularAccelerationPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_acceleration_roll: match inst.get("shipAngularAccelerationRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_acceleration_yaw: match inst.get("shipAngularAccelerationYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_turbulence_pitch: match inst.get("shipAngularTurbulencePitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_turbulence_roll: match inst.get("shipAngularTurbulenceRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_angular_turbulence_yaw: match inst.get("shipAngularTurbulenceYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_linear_velocity_rl: match inst.get("shipLinearVelocityRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_linear_velocity_fb: match inst.get("shipLinearVelocityFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_linear_velocity_ud: match inst.get("shipLinearVelocityUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_linear_acceleration_rl: match inst.get("tunnelLinearAccelerationRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_linear_acceleration_fb: match inst.get("tunnelLinearAccelerationFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_linear_acceleration_ud: match inst.get("tunnelLinearAccelerationUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_angular_acceleration_pitch: match inst.get("tunnelAngularAccelerationPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_angular_acceleration_roll: match inst.get("tunnelAngularAccelerationRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_angular_acceleration_yaw: match inst.get("tunnelAngularAccelerationYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_pitch: match inst.get("playerInputPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_roll: match inst.get("playerInputRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_yaw: match inst.get("playerInputYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_strafe_rl: match inst.get("playerInputStrafeRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_strafe_fb: match inst.get("playerInputStrafeFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_input_strafe_ud: match inst.get("playerInputStrafeUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            afterburner_requested_rtpc: match inst.get("afterburnerRequestedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveAudioParams {
    /// `stateMap` (Class)
    #[serde(default)]
    pub state_map: Option<Handle<JumpDriveStateAudioMap>>,
    /// `tunnelProgressRtpc` (Class)
    #[serde(default)]
    pub tunnel_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// `inJumpTunnelRtpc` (Class)
    #[serde(default)]
    pub in_jump_tunnel_rtpc: Option<Handle<AudioRtpc>>,
    /// `distanceFromSplineRtpc` (Class)
    #[serde(default)]
    pub distance_from_spline_rtpc: Option<Handle<AudioRtpc>>,
    /// `wallImpactSpeedThreshold` (Single)
    #[serde(default)]
    pub wall_impact_speed_threshold: f32,
    /// `wallImpactDistanceThreshold` (Single)
    #[serde(default)]
    pub wall_impact_distance_threshold: f32,
    /// `passByLightDistanceThreshold` (Single)
    #[serde(default)]
    pub pass_by_light_distance_threshold: f32,
    /// `passByLightDistanceNormRtpc` (Class)
    #[serde(default)]
    pub pass_by_light_distance_norm_rtpc: Option<Handle<AudioRtpc>>,
    /// `passByLightDotRtpc` (Class)
    #[serde(default)]
    pub pass_by_light_dot_rtpc: Option<Handle<AudioRtpc>>,
    /// `tunnelWallImpactSpeedRtpc` (Class)
    #[serde(default)]
    pub tunnel_wall_impact_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `tunnelWallImpactOneShot` (Class)
    #[serde(default)]
    pub tunnel_wall_impact_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// `startTunnelWallContactLoop` (Class)
    #[serde(default)]
    pub start_tunnel_wall_contact_loop: Option<Handle<GlobalResourceAudio>>,
    /// `stopTunnelWallContactLoop` (Class)
    #[serde(default)]
    pub stop_tunnel_wall_contact_loop: Option<Handle<GlobalResourceAudio>>,
    /// `splineVelRtpc` (Class)
    #[serde(default)]
    pub spline_vel_rtpc: Option<Handle<AudioRtpc>>,
    /// `distortionRtpc` (Class)
    #[serde(default)]
    pub distortion_rtpc: Option<Handle<AudioRtpc>>,
    /// `startSplineCenterLoop` (Class)
    #[serde(default)]
    pub start_spline_center_loop: Option<Handle<GlobalResourceAudio>>,
    /// `stopSplineCenterLoop` (Class)
    #[serde(default)]
    pub stop_spline_center_loop: Option<Handle<GlobalResourceAudio>>,
    /// `startShipTunnelMidpointLoop` (Class)
    #[serde(default)]
    pub start_ship_tunnel_midpoint_loop: Option<Handle<GlobalResourceAudio>>,
    /// `stopShipTunnelMidpointLoop` (Class)
    #[serde(default)]
    pub stop_ship_tunnel_midpoint_loop: Option<Handle<GlobalResourceAudio>>,
    /// `movementParams` (Class)
    #[serde(default)]
    pub movement_params: Option<Handle<JumpDriveAudioMovementParams>>,
}

impl Pooled for JumpDriveAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_audio_params }
}

impl<'a> Extract<'a> for JumpDriveAudioParams {
    const TYPE_NAME: &'static str = "JumpDriveAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_map: match inst.get("stateMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveStateAudioMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_progress_rtpc: match inst.get("tunnelProgressRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            in_jump_tunnel_rtpc: match inst.get("inJumpTunnelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_from_spline_rtpc: match inst.get("distanceFromSplineRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            wall_impact_speed_threshold: inst.get_f32("wallImpactSpeedThreshold").unwrap_or_default(),
            wall_impact_distance_threshold: inst.get_f32("wallImpactDistanceThreshold").unwrap_or_default(),
            pass_by_light_distance_threshold: inst.get_f32("passByLightDistanceThreshold").unwrap_or_default(),
            pass_by_light_distance_norm_rtpc: match inst.get("passByLightDistanceNormRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            pass_by_light_dot_rtpc: match inst.get("passByLightDotRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_wall_impact_speed_rtpc: match inst.get("tunnelWallImpactSpeedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tunnel_wall_impact_one_shot: match inst.get("tunnelWallImpactOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_tunnel_wall_contact_loop: match inst.get("startTunnelWallContactLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_tunnel_wall_contact_loop: match inst.get("stopTunnelWallContactLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spline_vel_rtpc: match inst.get("splineVelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distortion_rtpc: match inst.get("distortionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_spline_center_loop: match inst.get("startSplineCenterLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_spline_center_loop: match inst.get("stopSplineCenterLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_ship_tunnel_midpoint_loop: match inst.get("startShipTunnelMidpointLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_ship_tunnel_midpoint_loop: match inst.get("stopShipTunnelMidpointLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            movement_params: match inst.get("movementParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveAudioMovementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveMusicEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveMusicEvent {
    /// `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for JumpDriveMusicEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_music_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_music_event }
}

impl<'a> Extract<'a> for JumpDriveMusicEvent {
    const TYPE_NAME: &'static str = "JumpDriveMusicEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveStateMusicMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveStateMusicMap {
    /// `musicEvent` (Class)
    #[serde(default)]
    pub music_event: Option<Handle<JumpDriveMusicEvent>>,
    /// `jumpDriveState` (EnumChoice)
    #[serde(default)]
    pub jump_drive_state: ItemJumpDriveState,
}

impl Pooled for JumpDriveStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_state_music_map }
}

impl<'a> Extract<'a> for JumpDriveStateMusicMap {
    const TYPE_NAME: &'static str = "JumpDriveStateMusicMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_event: match inst.get("musicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            jump_drive_state: ItemJumpDriveState::from_dcb_str(inst.get_str("jumpDriveState").unwrap_or("")),
        }
    }
}

/// DCB type: `JumpDriveMusicParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveMusicParams {
    /// `tunnelProgressRtpc` (Class)
    #[serde(default)]
    pub tunnel_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// `stateMap` (Class (array))
    #[serde(default)]
    pub state_map: Vec<Handle<JumpDriveStateMusicMap>>,
    /// `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
    /// `preArrivalMusicEvent` (Class)
    #[serde(default)]
    pub pre_arrival_music_event: Option<Handle<JumpDriveMusicEvent>>,
}

impl Pooled for JumpDriveMusicParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_music_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_music_params }
}

impl<'a> Extract<'a> for JumpDriveMusicParams {
    const TYPE_NAME: &'static str = "JumpDriveMusicParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tunnel_progress_rtpc: match inst.get("tunnelProgressRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            state_map: inst.get_array("stateMap")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<JumpDriveStateMusicMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<JumpDriveStateMusicMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            pre_arrival_duration_secs: inst.get_f32("preArrivalDurationSecs").unwrap_or_default(),
            pre_arrival_music_event: match inst.get("preArrivalMusicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpDriveTuningEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveTuningEffectParams {
    /// `interferenceStrength` (Single)
    #[serde(default)]
    pub interference_strength: f32,
    /// `midPointForInterferencePercentage` (Single)
    #[serde(default)]
    pub mid_point_for_interference_percentage: f32,
}

impl Pooled for GlobalJumpDriveTuningEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_drive_tuning_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_drive_tuning_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveTuningEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveTuningEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interference_strength: inst.get_f32("interferenceStrength").unwrap_or_default(),
            mid_point_for_interference_percentage: inst.get_f32("midPointForInterferencePercentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveEntryEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveEntryEffectParams {
    /// `trailStrengthMaxDistance` (Single)
    #[serde(default)]
    pub trail_strength_max_distance: f32,
}

impl Pooled for GlobalJumpDriveEntryEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_drive_entry_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_drive_entry_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveEntryEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveEntryEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trail_strength_max_distance: inst.get_f32("trailStrengthMaxDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveExitEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveExitEffectParams {
    /// `trailStrengthDelay` (Single)
    #[serde(default)]
    pub trail_strength_delay: f32,
}

impl Pooled for GlobalJumpDriveExitEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_drive_exit_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_drive_exit_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveExitEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveExitEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trail_strength_delay: inst.get_f32("trailStrengthDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveEffectParams {
    /// `tuningParams` (Class)
    #[serde(default)]
    pub tuning_params: Option<Handle<GlobalJumpDriveTuningEffectParams>>,
    /// `entryParams` (Class)
    #[serde(default)]
    pub entry_params: Option<Handle<GlobalJumpDriveEntryEffectParams>>,
    /// `exitParams` (Class)
    #[serde(default)]
    pub exit_params: Option<Handle<GlobalJumpDriveExitEffectParams>>,
    /// `failureBuildUpTime` (Single)
    #[serde(default)]
    pub failure_build_up_time: f32,
    /// `failureDissipationTime` (Single)
    #[serde(default)]
    pub failure_dissipation_time: f32,
}

impl Pooled for GlobalJumpDriveEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_drive_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_drive_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tuning_params: match inst.get("tuningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveTuningEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entry_params: match inst.get("entryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveEntryEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exit_params: match inst.get("exitParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveExitEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            failure_build_up_time: inst.get_f32("failureBuildUpTime").unwrap_or_default(),
            failure_dissipation_time: inst.get_f32("failureDissipationTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveApproachRingsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveApproachRingsParams {
    /// `firstRingDistance` (Single)
    #[serde(default)]
    pub first_ring_distance: f32,
    /// `lastRingDistance` (Single)
    #[serde(default)]
    pub last_ring_distance: f32,
    /// `largestRingDiameter` (Single)
    #[serde(default)]
    pub largest_ring_diameter: f32,
}

impl Pooled for JumpDriveApproachRingsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_drive_approach_rings_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_drive_approach_rings_params }
}

impl<'a> Extract<'a> for JumpDriveApproachRingsParams {
    const TYPE_NAME: &'static str = "JumpDriveApproachRingsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            first_ring_distance: inst.get_f32("firstRingDistance").unwrap_or_default(),
            last_ring_distance: inst.get_f32("lastRingDistance").unwrap_or_default(),
            largest_ring_diameter: inst.get_f32("largestRingDiameter").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveParams {
    /// `audioParams` (Class)
    #[serde(default)]
    pub audio_params: Option<Handle<JumpDriveAudioParams>>,
    /// `musicParams` (Class)
    #[serde(default)]
    pub music_params: Option<Handle<JumpDriveMusicParams>>,
    /// `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpDriveEffectParams>>,
    /// `approachRingParams` (Class)
    #[serde(default)]
    pub approach_ring_params: Option<Handle<JumpDriveApproachRingsParams>>,
    /// `malfunction` (StrongPointer)
    #[serde(default)]
    pub malfunction: Option<SMisfireEffectPtr>,
    /// `checksPassedDelay` (Single)
    #[serde(default)]
    pub checks_passed_delay: f32,
    /// `obstructionPaddingSize` (Single)
    #[serde(default)]
    pub obstruction_padding_size: f32,
    /// `wallRepelBounceVelocity` (Single)
    #[serde(default)]
    pub wall_repel_bounce_velocity: f32,
}

impl Pooled for GlobalJumpDriveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.global_jump_drive_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.global_jump_drive_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            music_params: match inst.get("musicParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            approach_ring_params: match inst.get("approachRingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveApproachRingsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            malfunction: match inst.get("malfunction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SMisfireEffectPtr::from_ref(b, r)),
                _ => None,
            },
            checks_passed_delay: inst.get_f32("checksPassedDelay").unwrap_or_default(),
            obstruction_padding_size: inst.get_f32("obstructionPaddingSize").unwrap_or_default(),
            wall_repel_bounce_velocity: inst.get_f32("wallRepelBounceVelocity").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTravelCameraParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTravelCameraParams {
    /// `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
    /// `cameraEffectParams` (Class)
    #[serde(default)]
    pub camera_effect_params: Option<Handle<GlobalJumpTunnelCameraEffectParams>>,
}

impl Pooled for JumpTravelCameraParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.jumppoints.jump_travel_camera_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.jumppoints.jump_travel_camera_params }
}

impl<'a> Extract<'a> for JumpTravelCameraParams {
    const TYPE_NAME: &'static str = "JumpTravelCameraParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            camera_effect_params: match inst.get("cameraEffectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelCameraEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

