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

/// Pool storage for the `entities-haulingentityclass` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesHaulingentityclassPools {
    #[serde(default)]
    pub hauling_entity_class_list_base: Vec<Option<Hauling_EntityClassListBase>>,
    #[serde(default)]
    pub hauling_entity_classes: Vec<Option<Hauling_EntityClasses>>,
}
