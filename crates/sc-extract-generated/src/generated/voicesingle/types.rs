// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `voicesingle`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `VoiceSingle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSingle {
    /// `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
}

impl Pooled for VoiceSingle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.voicesingle.voice_single }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.voicesingle.voice_single }
}

impl<'a> Extract<'a> for VoiceSingle {
    const TYPE_NAME: &'static str = "VoiceSingle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
        }
    }
}

