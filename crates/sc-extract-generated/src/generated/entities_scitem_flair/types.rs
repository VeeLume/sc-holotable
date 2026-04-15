// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-flair`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SCItemFishParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemFishParams {
    /// `species` (String)
    #[serde(default)]
    pub species: String,
    /// `count` (Int32)
    #[serde(default)]
    pub count: i32,
}

impl Pooled for SCItemFishParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_flair.scitem_fish_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_flair.scitem_fish_params }
}

impl<'a> Extract<'a> for SCItemFishParams {
    const TYPE_NAME: &'static str = "SCItemFishParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            species: inst.get_str("species").map(String::from).unwrap_or_default(),
            count: inst.get_i32("count").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCJukeboxParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCJukeboxParams {
    /// `startTrigger` (Class)
    #[serde(default)]
    pub start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopTrigger` (Class)
    #[serde(default)]
    pub stop_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for SCJukeboxParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_flair.scjukebox_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_flair.scjukebox_params }
}

impl<'a> Extract<'a> for SCJukeboxParams {
    const TYPE_NAME: &'static str = "SCJukeboxParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_trigger: match inst.get("startTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_trigger: match inst.get("stopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

