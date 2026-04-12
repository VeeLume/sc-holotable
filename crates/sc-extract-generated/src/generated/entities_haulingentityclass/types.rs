// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-haulingentityclass`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Hauling_EntityClassListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hauling_EntityClassListBase {
}

impl Pooled for Hauling_EntityClassListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_haulingentityclass.hauling_entity_class_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_haulingentityclass.hauling_entity_class_list_base }
}

impl<'a> Extract<'a> for Hauling_EntityClassListBase {
    const TYPE_NAME: &'static str = "Hauling_EntityClassListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Hauling_EntityClasses`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hauling_EntityClasses {
    /// `entityClasses` (StrongPointer)
    #[serde(default)]
    pub entity_classes: Option<Handle<Hauling_EntityClassListBase>>,
    /// `orderDisplayName` (Locale)
    #[serde(default)]
    pub order_display_name: String,
}

impl Pooled for Hauling_EntityClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_haulingentityclass.hauling_entity_classes }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_haulingentityclass.hauling_entity_classes }
}

impl<'a> Extract<'a> for Hauling_EntityClasses {
    const TYPE_NAME: &'static str = "Hauling_EntityClasses";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_classes: match inst.get("entityClasses") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Hauling_EntityClassListBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Hauling_EntityClassListBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            order_display_name: inst.get_str("orderDisplayName").map(String::from).unwrap_or_default(),
        }
    }
}

