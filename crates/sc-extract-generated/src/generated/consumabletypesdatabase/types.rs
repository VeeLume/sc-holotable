// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `consumabletypesdatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ConsumableType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableType {
    /// `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// `subtypes` (Reference (array))
    #[serde(default)]
    pub subtypes: Vec<CigGuid>,
}

impl Pooled for ConsumableType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.consumabletypesdatabase.consumable_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.consumabletypesdatabase.consumable_type }
}

impl<'a> Extract<'a> for ConsumableType {
    const TYPE_NAME: &'static str = "ConsumableType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            subtypes: inst.get_array("subtypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConsumableSubtype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableSubtype {
    /// `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// `consumableName` (Locale)
    #[serde(default)]
    pub consumable_name: String,
    /// `effectsPerMicroSCU` (StrongPointer (array))
    #[serde(default)]
    pub effects_per_micro_scu: Vec<Handle<ConsumableEffect>>,
    /// `tintOverride` (StrongPointer)
    #[serde(default)]
    pub tint_override: Option<Handle<RGBA>>,
}

impl Pooled for ConsumableSubtype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.consumabletypesdatabase.consumable_subtype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.consumabletypesdatabase.consumable_subtype }
}

impl<'a> Extract<'a> for ConsumableSubtype {
    const TYPE_NAME: &'static str = "ConsumableSubtype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            consumable_name: inst.get_str("consumableName").map(String::from).unwrap_or_default(),
            effects_per_micro_scu: inst.get_array("effectsPerMicroSCU")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ConsumableEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ConsumableEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            tint_override: match inst.get("tintOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ConsumableTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTypeDatabase {
    /// `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for ConsumableTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.consumabletypesdatabase.consumable_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.consumabletypesdatabase.consumable_type_database }
}

impl<'a> Extract<'a> for ConsumableTypeDatabase {
    const TYPE_NAME: &'static str = "ConsumableTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConsumableEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableEffect {
    /// `effectDescription` (Locale)
    #[serde(default)]
    pub effect_description: String,
}

impl Pooled for ConsumableEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.consumabletypesdatabase.consumable_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.consumabletypesdatabase.consumable_effect }
}

impl<'a> Extract<'a> for ConsumableEffect {
    const TYPE_NAME: &'static str = "ConsumableEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_description: inst.get_str("effectDescription").map(String::from).unwrap_or_default(),
        }
    }
}

