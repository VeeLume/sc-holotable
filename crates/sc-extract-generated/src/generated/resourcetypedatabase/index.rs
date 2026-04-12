// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `resourcetypedatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcetypedatabaseIndex {
    #[serde(default)]
    pub resource_type: HashMap<CigGuid, Handle<ResourceType>>,
    #[serde(default)]
    pub resource_type_group: HashMap<CigGuid, Handle<ResourceTypeGroup>>,
    #[serde(default)]
    pub resource_type_database: HashMap<CigGuid, Handle<ResourceTypeDatabase>>,
}

impl ResourcetypedatabaseIndex {
    pub fn len(&self) -> usize {
        self.resource_type.len()
            + self.resource_type_group.len()
            + self.resource_type_database.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
