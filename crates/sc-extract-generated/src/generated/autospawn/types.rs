// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `autospawn`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `AutoSpawnTargetClass`
/// Inherits from: `AutoSpawnClass`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct AutoSpawnTargetClass {
    /// `npcClass` (Reference)
    pub npc_class: Option<CigGuid>,
}

impl Pooled for AutoSpawnTargetClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.autospawn.auto_spawn_target_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.autospawn.auto_spawn_target_class
    }
}

impl<'a> Extract<'a> for AutoSpawnTargetClass {
    const TYPE_NAME: &'static str = "AutoSpawnTargetClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            npc_class: inst
                .get("npcClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ScheduleAreaAssignment`
/// Inherits from: `OnSpawnAssignment`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct ScheduleAreaAssignment {
    /// `areaPositiveTags` (Class)
    pub area_positive_tags: Option<Handle<TagList>>,
    /// `areaNegativeTags` (Class)
    pub area_negative_tags: Option<Handle<TagList>>,
}

impl Pooled for ScheduleAreaAssignment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.autospawn.schedule_area_assignment
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.autospawn.schedule_area_assignment
    }
}

impl<'a> Extract<'a> for ScheduleAreaAssignment {
    const TYPE_NAME: &'static str = "ScheduleAreaAssignment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            area_positive_tags: match inst.get("areaPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            area_negative_tags: match inst.get("areaNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `EncounterIndexEntry`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct EncounterIndexEntry {
    /// `encounterPreset` (EnumChoice)
    pub encounter_preset: EncounterPreset,
    /// `autoSpawns` (Reference (array))
    pub auto_spawns: Vec<CigGuid>,
}

impl Pooled for EncounterIndexEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.autospawn.encounter_index_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.autospawn.encounter_index_entry
    }
}

impl<'a> Extract<'a> for EncounterIndexEntry {
    const TYPE_NAME: &'static str = "EncounterIndexEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            encounter_preset: EncounterPreset::from_dcb_str(
                inst.get_str("encounterPreset").unwrap_or(""),
            ),
            auto_spawns: inst
                .get_array("autoSpawns")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EncounterIndex`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct EncounterIndex {
    /// `encounters` (Class (array))
    pub encounters: Vec<Handle<EncounterIndexEntry>>,
}

impl Pooled for EncounterIndex {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.autospawn.encounter_index
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.autospawn.encounter_index
    }
}

impl<'a> Extract<'a> for EncounterIndex {
    const TYPE_NAME: &'static str = "EncounterIndex";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            encounters: inst
                .get_array("encounters")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<EncounterIndexEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<EncounterIndexEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
