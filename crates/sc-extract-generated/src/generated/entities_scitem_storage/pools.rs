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

/// Pool storage for the `entities-scitem-storage` feature.
#[derive(Default)]
pub struct EntitiesScitemStoragePools {
    pub strack_view_outfit_interaction_swap_data: Vec<Option<STrackViewOutfitInteractionSwapData>>,
    pub sentity_component_outfit_hanger_params: Vec<Option<SEntityComponentOutfitHangerParams>>,
    pub sflightsuit_hanger_group: Vec<Option<SFlightsuitHangerGroup>>,
    pub smannequin_hanger_group: Vec<Option<SMannequinHangerGroup>>,
}
