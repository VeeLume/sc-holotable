// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `entities-jumppoints` feature.
#[derive(Default)]
pub struct EntitiesJumppointsPools {
    pub jump_tunnel_effect_tinting_params: Vec<Option<JumpTunnelEffectTintingParams>>,
    pub jump_tunnel_cube_map_params: Vec<Option<JumpTunnelCubeMapParams>>,
    pub jump_point_effect_params: Vec<Option<JumpPointEffectParams>>,
    pub sjump_point_params: Vec<Option<SJumpPointParams>>,
    pub sjump_point_atcparams: Vec<Option<SJumpPointATCParams>>,
    pub sjump_tunnel_host_params: Vec<Option<SJumpTunnelHostParams>>,
    pub global_resource_texture_dds: Vec<Option<GlobalResourceTextureDDS>>,
}
