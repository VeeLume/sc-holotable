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

/// Pool storage for the `longtermpersistence` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LongtermpersistencePools {
    #[serde(default)]
    pub long_term_persistence_white_list_sub_type_entry: Vec<Option<LongTermPersistenceWhiteListSubTypeEntry>>,
    #[serde(default)]
    pub long_term_persistence_sub_type_all: Vec<Option<LongTermPersistenceSubTypeAll>>,
    #[serde(default)]
    pub long_term_persistence_sub_type_list: Vec<Option<LongTermPersistenceSubTypeList>>,
    #[serde(default)]
    pub long_term_persistence_white_list_entry: Vec<Option<LongTermPersistenceWhiteListEntry>>,
    #[serde(default)]
    pub long_term_persistence_global_params: Vec<Option<LongTermPersistenceGlobalParams>>,
}
