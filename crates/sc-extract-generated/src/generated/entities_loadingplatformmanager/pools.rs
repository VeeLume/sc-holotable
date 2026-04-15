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

/// Pool storage for the `entities-loadingplatformmanager` feature.
#[derive(Default)]
pub struct EntitiesLoadingplatformmanagerPools {
    pub scloading_platform_entity_references: Vec<Option<SCLoadingPlatformEntityReferences>>,
    pub scloading_platform_light_group_params: Vec<Option<SCLoadingPlatformLightGroupParams>>,
    pub scloading_platform_effect_params: Vec<Option<SCLoadingPlatformEffectParams>>,
    pub scloading_platform_trackview_params: Vec<Option<SCLoadingPlatformTrackviewParams>>,
    pub scloading_platform_manager_params: Vec<Option<SCLoadingPlatformManagerParams>>,
}
