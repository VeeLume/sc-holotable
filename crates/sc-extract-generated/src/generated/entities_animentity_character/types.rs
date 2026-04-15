// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-animentity_character`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SItemPortActorRecordParams`
/// Inherits from: `SItemPortLoadoutBaseParams`
pub struct SItemPortActorRecordParams {
    /// `WearRange` (StrongPointer)
    pub wear_range: Option<Handle<Range>>,
    /// `DirtRange` (StrongPointer)
    pub dirt_range: Option<Handle<Range>>,
    /// `SkipInventoryItemsOnMissionEntities` (Boolean)
    pub skip_inventory_items_on_mission_entities: bool,
    /// `InventoryItems` (Class (array))
    pub inventory_items: Vec<Handle<SLoadoutInventoryItem>>,
    /// `actorRecord` (Reference)
    pub actor_record: Option<CigGuid>,
    /// `OutfitNameTags` (Reference (array))
    pub outfit_name_tags: Vec<CigGuid>,
}

impl Pooled for SItemPortActorRecordParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_animentity_character.sitem_port_actor_record_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_animentity_character.sitem_port_actor_record_params }
}

impl<'a> Extract<'a> for SItemPortActorRecordParams {
    const TYPE_NAME: &'static str = "SItemPortActorRecordParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wear_range: match inst.get("WearRange") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dirt_range: match inst.get("DirtRange") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skip_inventory_items_on_mission_entities: inst.get_bool("SkipInventoryItemsOnMissionEntities").unwrap_or_default(),
            inventory_items: inst.get_array("InventoryItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SLoadoutInventoryItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SLoadoutInventoryItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            actor_record: inst.get("actorRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            outfit_name_tags: inst.get_array("OutfitNameTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

