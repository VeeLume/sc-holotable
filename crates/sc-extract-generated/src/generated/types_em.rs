// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `EmotionDescription`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionDescription {
    /// DCB field: `emotionName` (String)
    #[serde(default)]
    pub emotion_name: String,
    /// DCB field: `facialEmotionTag` (String)
    #[serde(default)]
    pub facial_emotion_tag: String,
}

impl Pooled for EmotionDescription {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_em.emotion_description }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_em.emotion_description }
}

impl<'a> Extract<'a> for EmotionDescription {
    const TYPE_NAME: &'static str = "EmotionDescription";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            emotion_name: inst.get_str("emotionName").map(String::from).unwrap_or_default(),
            facial_emotion_tag: inst.get_str("facialEmotionTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EmotionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionList {
    /// DCB field: `emotions` (Class (array))
    #[serde(default)]
    pub emotions: Vec<Handle<EmotionDescription>>,
}

impl Pooled for EmotionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_em.emotion_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_em.emotion_list }
}

impl<'a> Extract<'a> for EmotionList {
    const TYPE_NAME: &'static str = "EmotionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            emotions: inst.get_array("emotions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EmotionDescription>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EmotionDescription>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

