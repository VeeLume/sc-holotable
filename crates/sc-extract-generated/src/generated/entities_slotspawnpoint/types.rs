// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-slotspawnpoint`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EntityComponentSlotSpawnPointParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentSlotSpawnPointParams {
    /// `previewEntityClass` (Reference)
    #[serde(default)]
    pub preview_entity_class: Option<CigGuid>,
}

impl Pooled for EntityComponentSlotSpawnPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_slotspawnpoint.entity_component_slot_spawn_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_slotspawnpoint.entity_component_slot_spawn_point_params }
}

impl<'a> Extract<'a> for EntityComponentSlotSpawnPointParams {
    const TYPE_NAME: &'static str = "EntityComponentSlotSpawnPointParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            preview_entity_class: inst.get("previewEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

