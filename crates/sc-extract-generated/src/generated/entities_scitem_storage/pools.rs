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

/// Pool storage for the `entities-scitem-storage` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemStoragePools {
    #[serde(default)]
    pub strack_view_outfit_interaction_swap_data: Vec<Option<STrackViewOutfitInteractionSwapData>>,
    #[serde(default)]
    pub sentity_component_outfit_hanger_params: Vec<Option<SEntityComponentOutfitHangerParams>>,
    #[serde(default)]
    pub sflightsuit_hanger_group: Vec<Option<SFlightsuitHangerGroup>>,
    #[serde(default)]
    pub smannequin_hanger_group: Vec<Option<SMannequinHangerGroup>>,
}
