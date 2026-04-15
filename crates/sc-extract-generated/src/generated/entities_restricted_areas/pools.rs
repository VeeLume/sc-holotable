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

/// Pool storage for the `entities-restricted_areas` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesRestricted_areasPools {
    #[serde(default)]
    pub restricted_area_component_params: Vec<Option<RestrictedAreaComponentParams>>,
}
