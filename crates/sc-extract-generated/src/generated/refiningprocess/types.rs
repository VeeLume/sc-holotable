// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `refiningprocess`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RefiningProcess`
pub struct RefiningProcess {
    /// `refiningSpeed` (EnumChoice)
    pub refining_speed: RefiningSpeed,
    /// `refiningQuality` (EnumChoice)
    pub refining_quality: RefiningQuality,
    /// `processName` (Locale)
    pub process_name: LocaleKey,
}

impl Pooled for RefiningProcess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.refiningprocess.refining_process }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.refiningprocess.refining_process }
}

impl<'a> Extract<'a> for RefiningProcess {
    const TYPE_NAME: &'static str = "RefiningProcess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            refining_speed: RefiningSpeed::from_dcb_str(inst.get_str("refiningSpeed").unwrap_or("")),
            refining_quality: RefiningQuality::from_dcb_str(inst.get_str("refiningQuality").unwrap_or("")),
            process_name: inst.get_str("processName").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

