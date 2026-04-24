// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalchargedrainbeamparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SChargeDrainHighlightOutlineValues`
pub struct SChargeDrainHighlightOutlineValues {
    /// `color` (Class)
    pub color: Option<Handle<RGB>>,
    /// `occludedAlpha` (Single)
    pub occluded_alpha: f32,
    /// `outlineWidth` (Single)
    pub outline_width: f32,
    /// `outlineOnly` (Boolean)
    pub outline_only: bool,
}

impl Pooled for SChargeDrainHighlightOutlineValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalchargedrainbeamparams
            .scharge_drain_highlight_outline_values
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalchargedrainbeamparams
            .scharge_drain_highlight_outline_values
    }
}

impl<'a> Extract<'a> for SChargeDrainHighlightOutlineValues {
    const TYPE_NAME: &'static str = "SChargeDrainHighlightOutlineValues";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            occluded_alpha: inst.get_f32("occludedAlpha").unwrap_or_default(),
            outline_width: inst.get_f32("outlineWidth").unwrap_or_default(),
            outline_only: inst.get_bool("outlineOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SChargeDrainTargetStateOutlineParams`
pub struct SChargeDrainTargetStateOutlineParams {
    /// `inoperableOutlineValues` (Class)
    pub inoperable_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// `jumpstartRequiredOutlineValues` (Class)
    pub jumpstart_required_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// `jumpstartPossibleOutlineValues` (Class)
    pub jumpstart_possible_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// `validTargetOutlineValues` (Class)
    pub valid_target_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
}

impl Pooled for SChargeDrainTargetStateOutlineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalchargedrainbeamparams
            .scharge_drain_target_state_outline_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalchargedrainbeamparams
            .scharge_drain_target_state_outline_params
    }
}

impl<'a> Extract<'a> for SChargeDrainTargetStateOutlineParams {
    const TYPE_NAME: &'static str = "SChargeDrainTargetStateOutlineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            inoperable_outline_values: match inst.get("inoperableOutlineValues") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            jumpstart_required_outline_values: match inst.get("jumpstartRequiredOutlineValues") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            jumpstart_possible_outline_values: match inst.get("jumpstartPossibleOutlineValues") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            valid_target_outline_values: match inst.get("validTargetOutlineValues") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SChargeDrainCardParams`
pub struct SChargeDrainCardParams {
    /// `cardLerpSpeed` (Single)
    pub card_lerp_speed: f32,
    /// `attachPointLerpSpeed` (Single)
    pub attach_point_lerp_speed: f32,
    /// `closingDelay` (Single)
    pub closing_delay: f32,
    /// `closingTransitionTime` (Single)
    pub closing_transition_time: f32,
    /// `nearDistance` (Single)
    pub near_distance: f32,
    /// `defaultScreenPos` (Class)
    pub default_screen_pos: Option<Handle<Vec2>>,
    /// `maxDistScreenPosScale` (Single)
    pub max_dist_screen_pos_scale: f32,
}

impl Pooled for SChargeDrainCardParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.sglobalchargedrainbeamparams.scharge_drain_card_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.sglobalchargedrainbeamparams.scharge_drain_card_params
    }
}

impl<'a> Extract<'a> for SChargeDrainCardParams {
    const TYPE_NAME: &'static str = "SChargeDrainCardParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            card_lerp_speed: inst.get_f32("cardLerpSpeed").unwrap_or_default(),
            attach_point_lerp_speed: inst.get_f32("attachPointLerpSpeed").unwrap_or_default(),
            closing_delay: inst.get_f32("closingDelay").unwrap_or_default(),
            closing_transition_time: inst.get_f32("closingTransitionTime").unwrap_or_default(),
            near_distance: inst.get_f32("nearDistance").unwrap_or_default(),
            default_screen_pos: match inst.get("defaultScreenPos") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            max_dist_screen_pos_scale: inst.get_f32("maxDistScreenPosScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalChargeDrainBeamParams`
pub struct SGlobalChargeDrainBeamParams {
    /// `targetStateOutlineParams` (Class)
    pub target_state_outline_params: Option<Handle<SChargeDrainTargetStateOutlineParams>>,
    /// `targetCardParams` (Class)
    pub target_card_params: Option<Handle<SChargeDrainCardParams>>,
    /// `chargeCardParams` (Class)
    pub charge_card_params: Option<Handle<SChargeDrainCardParams>>,
}

impl Pooled for SGlobalChargeDrainBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalchargedrainbeamparams
            .sglobal_charge_drain_beam_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalchargedrainbeamparams
            .sglobal_charge_drain_beam_params
    }
}

impl<'a> Extract<'a> for SGlobalChargeDrainBeamParams {
    const TYPE_NAME: &'static str = "SGlobalChargeDrainBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target_state_outline_params: match inst.get("targetStateOutlineParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainTargetStateOutlineParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            target_card_params: match inst.get("targetCardParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainCardParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            charge_card_params: match inst.get("chargeCardParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SChargeDrainCardParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
