// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-worlddisplay`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SSCSignatureDummyParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SSCSignatureDummyParams {
    /// `enable` (Boolean)
    pub enable: bool,
    /// `type` (EnumChoice)
    pub r#type: ESignatureType,
    /// `value` (Single)
    pub value: f32,
}

impl Pooled for SSCSignatureDummyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_worlddisplay.sscsignature_dummy_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_worlddisplay.sscsignature_dummy_params
    }
}

impl<'a> Extract<'a> for SSCSignatureDummyParams {
    const TYPE_NAME: &'static str = "SSCSignatureDummyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            r#type: ESignatureType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}
