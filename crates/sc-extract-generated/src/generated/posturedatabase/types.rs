// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `posturedatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PostureDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureDatabase {
    /// `Groups` (StrongPointer (array))
    #[serde(default)]
    pub groups: Vec<Handle<PostureGroup>>,
}

impl Pooled for PostureDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.posturedatabase.posture_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.posturedatabase.posture_database }
}

impl<'a> Extract<'a> for PostureDatabase {
    const TYPE_NAME: &'static str = "PostureDatabase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("Groups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PostureGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PostureGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PostureGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureGroup {
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `Stance` (EnumChoice)
    #[serde(default)]
    pub stance: String,
    /// `Postures` (StrongPointer (array))
    #[serde(default)]
    pub postures: Vec<Handle<PostureData>>,
}

impl Pooled for PostureGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.posturedatabase.posture_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.posturedatabase.posture_group }
}

impl<'a> Extract<'a> for PostureGroup {
    const TYPE_NAME: &'static str = "PostureGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            stance: inst.get_str("Stance").map(String::from).unwrap_or_default(),
            postures: inst.get_array("Postures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PostureData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PostureData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PostureData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureData {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `Priority` (Single)
    #[serde(default)]
    pub priority: f32,
    /// `BodyDirection` (EnumChoice)
    #[serde(default)]
    pub body_direction: String,
    /// `IsLean` (Boolean)
    #[serde(default)]
    pub is_lean: bool,
    /// `AnimationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
}

impl Pooled for PostureData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.posturedatabase.posture_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.posturedatabase.posture_data }
}

impl<'a> Extract<'a> for PostureData {
    const TYPE_NAME: &'static str = "PostureData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("Priority").unwrap_or_default(),
            body_direction: inst.get_str("BodyDirection").map(String::from).unwrap_or_default(),
            is_lean: inst.get_bool("IsLean").unwrap_or_default(),
            animation_tag: inst.get_str("AnimationTag").map(String::from).unwrap_or_default(),
        }
    }
}

