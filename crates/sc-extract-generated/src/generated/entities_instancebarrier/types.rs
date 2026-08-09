// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-instancebarrier`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `InstanceBarrierParams`
/// Inherits from: `DataForgeComponentParams`
pub struct InstanceBarrierParams {
    /// `barrierType` (EnumChoice)
    pub barrier_type: EInstanceBarrierType,
}

impl Pooled for InstanceBarrierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_instancebarrier.instance_barrier_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_instancebarrier.instance_barrier_params
    }
}

impl<'a> Extract<'a> for InstanceBarrierParams {
    const TYPE_NAME: &'static str = "InstanceBarrierParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            barrier_type: EInstanceBarrierType::from_dcb_str(
                inst.get_str("barrierType").unwrap_or(""),
            ),
        }
    }
}
