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

/// Pool storage for the `entities-scitem-default_lensdisplay_pu` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemDefault_lensdisplay_puPools {
    #[serde(default)]
    pub schat_channel_black_list: Vec<Option<SChatChannelBlackList>>,
    #[serde(default)]
    pub entity_component_rtt_aspect_boxout_params: Vec<Option<EntityComponentRttAspectBoxoutParams>>,
}
