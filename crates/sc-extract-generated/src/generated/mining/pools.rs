// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `mining` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiningPools {
    #[serde(default)]
    pub hit_consistency_params: Vec<Option<HitConsistencyParams>>,
    #[serde(default)]
    pub mineable_explosion_params: Vec<Option<MineableExplosionParams>>,
    #[serde(default)]
    pub mineable_instability_params: Vec<Option<MineableInstabilityParams>>,
    #[serde(default)]
    pub mining_global_params: Vec<Option<MiningGlobalParams>>,
    #[serde(default)]
    pub mineable_element: Vec<Option<MineableElement>>,
    #[serde(default)]
    pub mineable_composition_part: Vec<Option<MineableCompositionPart>>,
    #[serde(default)]
    pub mineable_composition: Vec<Option<MineableComposition>>,
    #[serde(default)]
    pub mining_laser_global_params: Vec<Option<MiningLaserGlobalParams>>,
    #[serde(default)]
    pub mining_camera_shake_config: Vec<Option<MiningCameraShakeConfig>>,
    #[serde(default)]
    pub mining_controller_global_params: Vec<Option<MiningControllerGlobalParams>>,
}
