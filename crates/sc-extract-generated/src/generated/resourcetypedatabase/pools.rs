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

/// Pool storage for the `resourcetypedatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcetypedatabasePools {
    #[serde(default)]
    pub resource_type_properties: Vec<Option<ResourceTypeProperties>>,
    #[serde(default)]
    pub resource_type_density_type: Vec<Option<ResourceTypeDensityType>>,
    #[serde(default)]
    pub resource_type: Vec<Option<ResourceType>>,
    #[serde(default)]
    pub resource_type_group: Vec<Option<ResourceTypeGroup>>,
    #[serde(default)]
    pub resource_type_database: Vec<Option<ResourceTypeDatabase>>,
}
