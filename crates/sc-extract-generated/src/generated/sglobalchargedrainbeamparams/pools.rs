// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `sglobalchargedrainbeamparams` feature.
#[derive(Default)]
pub struct SglobalchargedrainbeamparamsPools {
    pub scharge_drain_highlight_outline_values: Vec<Option<SChargeDrainHighlightOutlineValues>>,
    pub scharge_drain_target_state_outline_params:
        Vec<Option<SChargeDrainTargetStateOutlineParams>>,
    pub scharge_drain_card_params: Vec<Option<SChargeDrainCardParams>>,
    pub sglobal_charge_drain_beam_params: Vec<Option<SGlobalChargeDrainBeamParams>>,
}
