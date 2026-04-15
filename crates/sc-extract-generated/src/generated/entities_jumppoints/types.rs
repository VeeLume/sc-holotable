// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-jumppoints`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `JumpTunnelEffectTintingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelEffectTintingParams {
    /// `startColor` (Class)
    #[serde(default)]
    pub start_color: Option<Handle<RGB>>,
    /// `midColor` (Class)
    #[serde(default)]
    pub mid_color: Option<Handle<RGB>>,
    /// `endColor` (Class)
    #[serde(default)]
    pub end_color: Option<Handle<RGB>>,
    /// `startGlow` (Single)
    #[serde(default)]
    pub start_glow: f32,
    /// `endGlow` (Single)
    #[serde(default)]
    pub end_glow: f32,
    /// `distanceBeforeMidPoint` (Single)
    #[serde(default)]
    pub distance_before_mid_point: f32,
    /// `distanceAfterMidPoint` (Single)
    #[serde(default)]
    pub distance_after_mid_point: f32,
}

impl Pooled for JumpTunnelEffectTintingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.jump_tunnel_effect_tinting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.jump_tunnel_effect_tinting_params }
}

impl<'a> Extract<'a> for JumpTunnelEffectTintingParams {
    const TYPE_NAME: &'static str = "JumpTunnelEffectTintingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_color: match inst.get("startColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            mid_color: match inst.get("midColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            end_color: match inst.get("endColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_glow: inst.get_f32("startGlow").unwrap_or_default(),
            end_glow: inst.get_f32("endGlow").unwrap_or_default(),
            distance_before_mid_point: inst.get_f32("distanceBeforeMidPoint").unwrap_or_default(),
            distance_after_mid_point: inst.get_f32("distanceAfterMidPoint").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelCubeMapParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelCubeMapParams {
    /// `entrySpecProbe` (Class)
    #[serde(default)]
    pub entry_spec_probe: Option<Handle<GlobalResourceTextureDDS>>,
    /// `entryDiffProbe` (Class)
    #[serde(default)]
    pub entry_diff_probe: Option<Handle<GlobalResourceTextureDDS>>,
    /// `exitSpecProbe` (Class)
    #[serde(default)]
    pub exit_spec_probe: Option<Handle<GlobalResourceTextureDDS>>,
    /// `exitDiffProbe` (Class)
    #[serde(default)]
    pub exit_diff_probe: Option<Handle<GlobalResourceTextureDDS>>,
}

impl Pooled for JumpTunnelCubeMapParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.jump_tunnel_cube_map_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.jump_tunnel_cube_map_params }
}

impl<'a> Extract<'a> for JumpTunnelCubeMapParams {
    const TYPE_NAME: &'static str = "JumpTunnelCubeMapParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entry_spec_probe: match inst.get("entrySpecProbe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceTextureDDS>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entry_diff_probe: match inst.get("entryDiffProbe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceTextureDDS>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exit_spec_probe: match inst.get("exitSpecProbe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceTextureDDS>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            exit_diff_probe: match inst.get("exitDiffProbe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceTextureDDS>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpPointEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpPointEffectParams {
    /// `tunnelEffectTinting` (Class)
    #[serde(default)]
    pub tunnel_effect_tinting: Option<Handle<JumpTunnelEffectTintingParams>>,
    /// `cubeMapParams` (Class)
    #[serde(default)]
    pub cube_map_params: Option<Handle<JumpTunnelCubeMapParams>>,
}

impl Pooled for JumpPointEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.jump_point_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.jump_point_effect_params }
}

impl<'a> Extract<'a> for JumpPointEffectParams {
    const TYPE_NAME: &'static str = "JumpPointEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tunnel_effect_tinting: match inst.get("tunnelEffectTinting") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelEffectTintingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cube_map_params: match inst.get("cubeMapParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCubeMapParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpPointParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpPointParams {
    /// `jumpGateSuperGuid` (String)
    #[serde(default)]
    pub jump_gate_super_guid: String,
    /// `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<JumpPointEffectParams>>,
    /// `entrancePushAreaParams` (Class)
    #[serde(default)]
    pub entrance_push_area_params: Option<Handle<SJumpPointPushAreaParams>>,
    /// `pushAreaRadarGeometryName` (String)
    #[serde(default)]
    pub push_area_radar_geometry_name: String,
    /// `largestShipSize` (Reference)
    #[serde(default)]
    pub largest_ship_size: Option<CigGuid>,
    /// `linkingRange` (Single)
    #[serde(default)]
    pub linking_range: f32,
    /// `requiredFuel` (Single)
    #[serde(default)]
    pub required_fuel: f32,
}

impl Pooled for SJumpPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.sjump_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.sjump_point_params }
}

impl<'a> Extract<'a> for SJumpPointParams {
    const TYPE_NAME: &'static str = "SJumpPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            jump_gate_super_guid: inst.get_str("jumpGateSuperGuid").map(String::from).unwrap_or_default(),
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpPointEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entrance_push_area_params: match inst.get("entrancePushAreaParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpPointPushAreaParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            push_area_radar_geometry_name: inst.get_str("pushAreaRadarGeometryName").map(String::from).unwrap_or_default(),
            largest_ship_size: inst.get("largestShipSize").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            linking_range: inst.get_f32("linkingRange").unwrap_or_default(),
            required_fuel: inst.get_f32("requiredFuel").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpPointATCParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpPointATCParams {
}

impl Pooled for SJumpPointATCParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.sjump_point_atcparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.sjump_point_atcparams }
}

impl<'a> Extract<'a> for SJumpPointATCParams {
    const TYPE_NAME: &'static str = "SJumpPointATCParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SJumpTunnelHostParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpTunnelHostParams {
}

impl Pooled for SJumpTunnelHostParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.sjump_tunnel_host_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.sjump_tunnel_host_params }
}

impl<'a> Extract<'a> for SJumpTunnelHostParams {
    const TYPE_NAME: &'static str = "SJumpTunnelHostParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GlobalResourceTextureDDS`
/// Inherits from: `GlobalResourceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceTextureDDS {
    /// `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceTextureDDS {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_jumppoints.global_resource_texture_dds }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_jumppoints.global_resource_texture_dds }
}

impl<'a> Extract<'a> for GlobalResourceTextureDDS {
    const TYPE_NAME: &'static str = "GlobalResourceTextureDDS";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

