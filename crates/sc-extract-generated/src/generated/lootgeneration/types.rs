// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `lootgeneration`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `LootGenerationSpecialEventArchetype`
pub struct LootGenerationSpecialEventArchetype {
    /// `eventString` (String)
    pub event_string: String,
    /// `probabilityPerContainer` (Single)
    pub probability_per_container: f32,
    /// `minEntriesPerContainer` (Int32)
    pub min_entries_per_container: i32,
    /// `maxEntriesPerContainer` (Int32)
    pub max_entries_per_container: i32,
    /// `archetype` (Reference)
    pub archetype: Option<CigGuid>,
    /// `archetypeV3` (StrongPointer)
    pub archetype_v3: Option<LootArchetypeV3_BasePtr>,
}

impl Pooled for LootGenerationSpecialEventArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_generation_special_event_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_generation_special_event_archetype }
}

impl<'a> Extract<'a> for LootGenerationSpecialEventArchetype {
    const TYPE_NAME: &'static str = "LootGenerationSpecialEventArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            event_string: inst.get_str("eventString").map(String::from).unwrap_or_default(),
            probability_per_container: inst.get_f32("probabilityPerContainer").unwrap_or_default(),
            min_entries_per_container: inst.get_i32("minEntriesPerContainer").unwrap_or_default(),
            max_entries_per_container: inst.get_i32("maxEntriesPerContainer").unwrap_or_default(),
            archetype: inst.get("archetype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            archetype_v3: match inst.get("archetypeV3") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(LootArchetypeV3_BasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LootGenerationGlobalParams`
pub struct LootGenerationGlobalParams {
    /// `specialEventArchetypes` (Class (array))
    pub special_event_archetypes: Vec<Handle<LootGenerationSpecialEventArchetype>>,
}

impl Pooled for LootGenerationGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lootgeneration.loot_generation_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lootgeneration.loot_generation_global_params }
}

impl<'a> Extract<'a> for LootGenerationGlobalParams {
    const TYPE_NAME: &'static str = "LootGenerationGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            special_event_archetypes: inst.get_array("specialEventArchetypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootGenerationSpecialEventArchetype>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LootGenerationSpecialEventArchetype>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

