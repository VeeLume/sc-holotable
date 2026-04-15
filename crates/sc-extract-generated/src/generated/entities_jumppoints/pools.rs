// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-jumppoints` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesJumppointsPools {
    #[serde(default)]
    pub jump_tunnel_effect_tinting_params: Vec<Option<JumpTunnelEffectTintingParams>>,
    #[serde(default)]
    pub jump_tunnel_cube_map_params: Vec<Option<JumpTunnelCubeMapParams>>,
    #[serde(default)]
    pub jump_point_effect_params: Vec<Option<JumpPointEffectParams>>,
    #[serde(default)]
    pub sjump_point_params: Vec<Option<SJumpPointParams>>,
    #[serde(default)]
    pub sjump_point_atcparams: Vec<Option<SJumpPointATCParams>>,
    #[serde(default)]
    pub sjump_tunnel_host_params: Vec<Option<SJumpTunnelHostParams>>,
    #[serde(default)]
    pub global_resource_texture_dds: Vec<Option<GlobalResourceTextureDDS>>,
}
