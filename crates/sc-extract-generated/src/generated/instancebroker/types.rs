// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `instancebroker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `InstanceBrokerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct InstanceBrokerParams {
    /// `lostEligibilityTitleLocString` (Locale)
    pub lost_eligibility_title_loc_string: LocaleKey,
    /// `lostEligibilityBodyLocString` (Locale)
    pub lost_eligibility_body_loc_string: LocaleKey,
    /// `enteredWithoutEligibilityTitleLocString` (Locale)
    pub entered_without_eligibility_title_loc_string: LocaleKey,
    /// `enteredWithoutEligibilityBodyLocString` (Locale)
    pub entered_without_eligibility_body_loc_string: LocaleKey,
    /// `travellingToInstanceWithoutEligibilityTitleLocString` (Locale)
    pub travelling_to_instance_without_eligibility_title_loc_string: LocaleKey,
    /// `travellingToInstanceWithoutEligibilityBodyLocString` (Locale)
    pub travelling_to_instance_without_eligibility_body_loc_string: LocaleKey,
    /// `playerEjectTimerLength` (Single)
    pub player_eject_timer_length: f32,
}

impl Pooled for InstanceBrokerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.instancebroker.instance_broker_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.instancebroker.instance_broker_params
    }
}

impl<'a> Extract<'a> for InstanceBrokerParams {
    const TYPE_NAME: &'static str = "InstanceBrokerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lost_eligibility_title_loc_string: inst
                .get_str("lostEligibilityTitleLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            lost_eligibility_body_loc_string: inst
                .get_str("lostEligibilityBodyLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            entered_without_eligibility_title_loc_string: inst
                .get_str("enteredWithoutEligibilityTitleLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            entered_without_eligibility_body_loc_string: inst
                .get_str("enteredWithoutEligibilityBodyLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            travelling_to_instance_without_eligibility_title_loc_string: inst
                .get_str("travellingToInstanceWithoutEligibilityTitleLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            travelling_to_instance_without_eligibility_body_loc_string: inst
                .get_str("travellingToInstanceWithoutEligibilityBodyLocString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            player_eject_timer_length: inst.get_f32("playerEjectTimerLength").unwrap_or_default(),
        }
    }
}
