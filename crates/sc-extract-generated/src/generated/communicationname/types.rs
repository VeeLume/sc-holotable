// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `communicationname`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommunicationName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationName {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for CommunicationName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationname.communication_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationname.communication_name }
}

impl<'a> Extract<'a> for CommunicationName {
    const TYPE_NAME: &'static str = "CommunicationName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

