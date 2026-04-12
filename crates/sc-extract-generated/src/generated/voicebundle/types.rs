// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `voicebundle`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `VoiceBundle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceBundle {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `voices` (Reference (array))
    #[serde(default)]
    pub voices: Vec<CigGuid>,
}

impl Pooled for VoiceBundle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.voicebundle.voice_bundle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.voicebundle.voice_bundle }
}

impl<'a> Extract<'a> for VoiceBundle {
    const TYPE_NAME: &'static str = "VoiceBundle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            voices: inst.get_array("voices")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

