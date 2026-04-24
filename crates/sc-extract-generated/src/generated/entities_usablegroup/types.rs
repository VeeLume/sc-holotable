// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-usablegroup`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SUsableGroupParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SUsableGroupParams {
    /// `selectionMethod` (EnumChoice)
    pub selection_method: EUsableSelectionMethod,
    /// `condition` (EnumChoice)
    pub condition: EEndCondition,
    /// `resetOnCoordinator` (Boolean)
    pub reset_on_coordinator: bool,
    /// `maxNPCs` (Int32)
    pub max_npcs: i32,
    /// `numberOfUses` (Int32)
    pub number_of_uses: i32,
}

impl Pooled for SUsableGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_usablegroup.susable_group_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_usablegroup.susable_group_params
    }
}

impl<'a> Extract<'a> for SUsableGroupParams {
    const TYPE_NAME: &'static str = "SUsableGroupParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            selection_method: EUsableSelectionMethod::from_dcb_str(
                inst.get_str("selectionMethod").unwrap_or(""),
            ),
            condition: EEndCondition::from_dcb_str(inst.get_str("condition").unwrap_or("")),
            reset_on_coordinator: inst.get_bool("resetOnCoordinator").unwrap_or_default(),
            max_npcs: inst.get_i32("maxNPCs").unwrap_or_default(),
            number_of_uses: inst.get_i32("numberOfUses").unwrap_or_default(),
        }
    }
}
