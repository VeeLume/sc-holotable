// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `tagdatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// `legacyGUID` (UInt32)
    #[serde(default)]
    pub legacy_guid: u32,
    /// `tagName` (String)
    #[serde(default)]
    pub tag_name: String,
    /// `children` (Reference (array))
    #[serde(default)]
    pub children: Vec<CigGuid>,
}

impl Pooled for Tag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tagdatabase.tag }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tagdatabase.tag }
}

impl<'a> Extract<'a> for Tag {
    const TYPE_NAME: &'static str = "Tag";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            legacy_guid: inst.get_u32("legacyGUID").unwrap_or_default(),
            tag_name: inst.get_str("tagName").map(String::from).unwrap_or_default(),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDatabase {
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for TagDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tagdatabase.tag_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tagdatabase.tag_database }
}

impl<'a> Extract<'a> for TagDatabase {
    const TYPE_NAME: &'static str = "TagDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

