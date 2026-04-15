// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_tilesocket`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `STileSocketParams`
/// Inherits from: `DataForgeComponentParams`
pub struct STileSocketParams {
}

impl Pooled for STileSocketParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_tilesocket.stile_socket_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_tilesocket.stile_socket_params }
}

impl<'a> Extract<'a> for STileSocketParams {
    const TYPE_NAME: &'static str = "STileSocketParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

