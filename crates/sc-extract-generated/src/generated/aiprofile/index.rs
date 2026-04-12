// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `aiprofile` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiprofileIndex {
    #[serde(default)]
    pub activity_data: HashMap<CigGuid, Handle<ActivityData>>,
    #[serde(default)]
    pub aiperception_profile: HashMap<CigGuid, Handle<AIPerceptionProfile>>,
    #[serde(default)]
    pub aimercy_timer_settings: HashMap<CigGuid, Handle<AIMercyTimerSettings>>,
    #[serde(default)]
    pub aivisual_field_params: HashMap<CigGuid, Handle<AIVisualFieldParams>>,
    #[serde(default)]
    pub aivisual_field_profile: HashMap<CigGuid, Handle<AIVisualFieldProfile>>,
    #[serde(default)]
    pub aiobservable_filter_flags: HashMap<CigGuid, Handle<AIObservableFilterFlags>>,
    #[serde(default)]
    pub aiobservable_filters_profile: HashMap<CigGuid, Handle<AIObservableFiltersProfile>>,
    #[serde(default)]
    pub movement_system_additional_params_record: HashMap<CigGuid, Handle<MovementSystemAdditionalParamsRecord>>,
    #[serde(default)]
    pub aispecial_ranged_attack_config: HashMap<CigGuid, Handle<AISpecialRangedAttackConfig>>,
    #[serde(default)]
    pub aiavailable_special_ranged_attacks_config: HashMap<CigGuid, Handle<AIAvailableSpecialRangedAttacksConfig>>,
    #[serde(default)]
    pub aiprofile: HashMap<CigGuid, Handle<AIProfile>>,
    #[serde(default)]
    pub skill_definitions: HashMap<CigGuid, Handle<SkillDefinitions>>,
    #[serde(default)]
    pub stat_definitions: HashMap<CigGuid, Handle<StatDefinitions>>,
}

impl AiprofileIndex {
    pub fn len(&self) -> usize {
        self.activity_data.len()
            + self.aiperception_profile.len()
            + self.aimercy_timer_settings.len()
            + self.aivisual_field_params.len()
            + self.aivisual_field_profile.len()
            + self.aiobservable_filter_flags.len()
            + self.aiobservable_filters_profile.len()
            + self.movement_system_additional_params_record.len()
            + self.aispecial_ranged_attack_config.len()
            + self.aiavailable_special_ranged_attacks_config.len()
            + self.aiprofile.len()
            + self.skill_definitions.len()
            + self.stat_definitions.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
