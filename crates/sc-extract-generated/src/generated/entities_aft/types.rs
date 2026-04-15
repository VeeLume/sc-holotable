// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-aft`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EntityEventCallbackComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityEventCallbackComponentParams {
}

impl Pooled for EntityEventCallbackComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_aft.entity_event_callback_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_aft.entity_event_callback_component_params }
}

impl<'a> Extract<'a> for EntityEventCallbackComponentParams {
    const TYPE_NAME: &'static str = "EntityEventCallbackComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AreaEventCallbackComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaEventCallbackComponentParams {
}

impl Pooled for AreaEventCallbackComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_aft.area_event_callback_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_aft.area_event_callback_component_params }
}

impl<'a> Extract<'a> for AreaEventCallbackComponentParams {
    const TYPE_NAME: &'static str = "AreaEventCallbackComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

