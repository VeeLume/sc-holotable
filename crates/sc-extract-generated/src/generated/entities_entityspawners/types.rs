// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityspawners`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SSpawnOnlyOnRequest`
/// Inherits from: `SSpawnRules`
pub struct SSpawnOnlyOnRequest {
    /// `maxEntities` (Int32)
    pub max_entities: i32,
}

impl Pooled for SSpawnOnlyOnRequest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_entityspawners.sspawn_only_on_request
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_entityspawners.sspawn_only_on_request
    }
}

impl<'a> Extract<'a> for SSpawnOnlyOnRequest {
    const TYPE_NAME: &'static str = "SSpawnOnlyOnRequest";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_entities: inst.get_i32("maxEntities").unwrap_or_default(),
        }
    }
}

/// DCB type: `SSequencerSpawnInUseableSpawnerTaskParams`
/// Inherits from: `SSequencerSpawnerTaskParams`
pub struct SSequencerSpawnInUseableSpawnerTaskParams {
    /// `entityToSpawn` (Reference)
    pub entity_to_spawn: Option<CigGuid>,
    /// `useChannel` (Reference)
    pub use_channel: Option<CigGuid>,
    /// `triggerAreas` (Boolean)
    pub trigger_areas: bool,
}

impl Pooled for SSequencerSpawnInUseableSpawnerTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_entityspawners
            .ssequencer_spawn_in_useable_spawner_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_entityspawners
            .ssequencer_spawn_in_useable_spawner_task_params
    }
}

impl<'a> Extract<'a> for SSequencerSpawnInUseableSpawnerTaskParams {
    const TYPE_NAME: &'static str = "SSequencerSpawnInUseableSpawnerTaskParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_to_spawn: inst
                .get("entityToSpawn")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            use_channel: inst
                .get("useChannel")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            trigger_areas: inst.get_bool("triggerAreas").unwrap_or_default(),
        }
    }
}
