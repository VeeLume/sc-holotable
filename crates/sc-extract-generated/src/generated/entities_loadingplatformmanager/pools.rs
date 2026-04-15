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

/// Pool storage for the `entities-loadingplatformmanager` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesLoadingplatformmanagerPools {
    #[serde(default)]
    pub scloading_platform_entity_references: Vec<Option<SCLoadingPlatformEntityReferences>>,
    #[serde(default)]
    pub scloading_platform_light_group_params: Vec<Option<SCLoadingPlatformLightGroupParams>>,
    #[serde(default)]
    pub scloading_platform_effect_params: Vec<Option<SCLoadingPlatformEffectParams>>,
    #[serde(default)]
    pub scloading_platform_trackview_params: Vec<Option<SCLoadingPlatformTrackviewParams>>,
    #[serde(default)]
    pub scloading_platform_manager_params: Vec<Option<SCLoadingPlatformManagerParams>>,
}
