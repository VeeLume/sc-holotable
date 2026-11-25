// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aimotive`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `Condition_AudioIntensityHeard`
/// Inherits from: `AIMotiveCondition`
///
/// New in 5.0.0-ptu.13273298; absent from 4.10.3-live.13220821.
pub struct Condition_AudioIntensityHeard {
    /// `id` (Guid)
    pub id: CigGuid,
    /// `minSatisfactionDuration` (Single)
    pub min_satisfaction_duration: f32,
    /// `threshold` (Single)
    pub threshold: f32,
    /// `maxAllowedPause` (Single)
    pub max_allowed_pause: f32,
    /// `resetAfterTrigger` (Boolean)
    pub reset_after_trigger: bool,
}

impl Pooled for Condition_AudioIntensityHeard {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aimotive.condition_audio_intensity_heard
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aimotive.condition_audio_intensity_heard
    }
}

impl<'a> Extract<'a> for Condition_AudioIntensityHeard {
    const TYPE_NAME: &'static str = "Condition_AudioIntensityHeard";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            min_satisfaction_duration: inst.get_f32("minSatisfactionDuration").unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
            max_allowed_pause: inst.get_f32("maxAllowedPause").unwrap_or_default(),
            reset_after_trigger: inst.get_bool("resetAfterTrigger").unwrap_or_default(),
        }
    }
}

/// DCB type: `Condition_Stance`
/// Inherits from: `AIMotiveCondition`
///
/// New in 5.0.0-ptu.13273298; absent from 4.10.3-live.13220821.
pub struct Condition_Stance {
    /// `id` (Guid)
    pub id: CigGuid,
    /// `minSatisfactionDuration` (Single)
    pub min_satisfaction_duration: f32,
    /// `stance` (String)
    pub stance: String,
    /// `matchStance` (Boolean)
    pub match_stance: bool,
}

impl Pooled for Condition_Stance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aimotive.condition_stance
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aimotive.condition_stance
    }
}

impl<'a> Extract<'a> for Condition_Stance {
    const TYPE_NAME: &'static str = "Condition_Stance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            min_satisfaction_duration: inst.get_f32("minSatisfactionDuration").unwrap_or_default(),
            stance: inst.get_str("stance").map(String::from).unwrap_or_default(),
            match_stance: inst.get_bool("matchStance").unwrap_or_default(),
        }
    }
}

/// DCB type: `Condition_TargetIsShip`
/// Inherits from: `AIMotiveCondition`
pub struct Condition_TargetIsShip {
    /// `id` (Guid)
    pub id: CigGuid,
    /// `minSatisfactionDuration` (Single)
    pub min_satisfaction_duration: f32,
    /// `isShip` (Boolean)
    pub is_ship: bool,
}

impl Pooled for Condition_TargetIsShip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.aimotive.condition_target_is_ship
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.aimotive.condition_target_is_ship
    }
}

impl<'a> Extract<'a> for Condition_TargetIsShip {
    const TYPE_NAME: &'static str = "Condition_TargetIsShip";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            min_satisfaction_duration: inst.get_f32("minSatisfactionDuration").unwrap_or_default(),
            is_ship: inst.get_bool("isShip").unwrap_or_default(),
        }
    }
}
