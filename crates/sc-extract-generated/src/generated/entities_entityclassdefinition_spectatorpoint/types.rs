// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_spectatorpoint`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SpectatorPointParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectatorPointParams {
    /// `faction` (Reference)
    #[serde(default)]
    pub faction: Option<CigGuid>,
}

impl Pooled for SpectatorPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_spectatorpoint.spectator_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_spectatorpoint.spectator_point_params }
}

impl<'a> Extract<'a> for SpectatorPointParams {
    const TYPE_NAME: &'static str = "SpectatorPointParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            faction: inst.get("faction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

