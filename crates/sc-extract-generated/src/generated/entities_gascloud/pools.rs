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

/// Pool storage for the `entities-gascloud` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesGascloudPools {
    #[serde(default)]
    pub gas_cloud_light_noise_params: Vec<Option<GasCloudLightNoiseParams>>,
    #[serde(default)]
    pub gas_cloud_light_shadow_params: Vec<Option<GasCloudLightShadowParams>>,
    #[serde(default)]
    pub gas_cloud_light_fade_params: Vec<Option<GasCloudLightFadeParams>>,
    #[serde(default)]
    pub gas_cloud_light_audio_params: Vec<Option<GasCloudLightAudioParams>>,
    #[serde(default)]
    pub gas_cloud_light_params: Vec<Option<GasCloudLightParams>>,
    #[serde(default)]
    pub gas_cloud_override_sphere_volume_params: Vec<Option<GasCloudOverrideSphereVolumeParams>>,
    #[serde(default)]
    pub gas_cloud_override_cube_volume_params: Vec<Option<GasCloudOverrideCubeVolumeParams>>,
    #[serde(default)]
    pub gas_cloud_override_volume_params: Vec<Option<GasCloudOverrideVolumeParams>>,
    #[serde(default)]
    pub entity_link_targeting_params: Vec<Option<EntityLinkTargetingParams>>,
    #[serde(default)]
    pub lightning_region_lightning_params: Vec<Option<LightningRegionLightningParams>>,
    #[serde(default)]
    pub interference_params: Vec<Option<InterferenceParams>>,
    #[serde(default)]
    pub lightning_region_params: Vec<Option<LightningRegionParams>>,
}
