// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `handholdgripdatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `HandholdGripType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandholdGripType {
    /// `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
    /// `handholdType` (EnumChoice)
    #[serde(default)]
    pub handhold_type: String,
}

impl Pooled for HandholdGripType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.handholdgripdatabase.handhold_grip_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.handholdgripdatabase.handhold_grip_type }
}

impl<'a> Extract<'a> for HandholdGripType {
    const TYPE_NAME: &'static str = "HandholdGripType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
            handhold_type: inst.get_str("handholdType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HandholdGripDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandholdGripDatabase {
    /// `gripTypes` (Reference (array))
    #[serde(default)]
    pub grip_types: Vec<CigGuid>,
}

impl Pooled for HandholdGripDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.handholdgripdatabase.handhold_grip_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.handholdgripdatabase.handhold_grip_database }
}

impl<'a> Extract<'a> for HandholdGripDatabase {
    const TYPE_NAME: &'static str = "HandholdGripDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grip_types: inst.get_array("gripTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

