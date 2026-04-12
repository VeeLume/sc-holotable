// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `aiprofile` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiprofilePools {
    #[serde(default)]
    pub activity_behavior_request_condition: Vec<Option<ActivityBehaviorRequestCondition>>,
    #[serde(default)]
    pub activity_behavior_request: Vec<Option<ActivityBehaviorRequest>>,
    #[serde(default)]
    pub activity_data_record: Vec<Option<ActivityDataRecord>>,
    #[serde(default)]
    pub activity_data: Vec<Option<ActivityData>>,
    #[serde(default)]
    pub saiperception_disturbance_type_settings: Vec<Option<SAIPerceptionDisturbanceTypeSettings>>,
    #[serde(default)]
    pub saiperception_meter_settings: Vec<Option<SAIPerceptionMeterSettings>>,
    #[serde(default)]
    pub saiperception_audio_type_settings: Vec<Option<SAIPerceptionAudioTypeSettings>>,
    #[serde(default)]
    pub saiperception_audio_settings: Vec<Option<SAIPerceptionAudioSettings>>,
    #[serde(default)]
    pub saiperception_visual_settings: Vec<Option<SAIPerceptionVisualSettings>>,
    #[serde(default)]
    pub aiperception_profile: Vec<Option<AIPerceptionProfile>>,
    #[serde(default)]
    pub aimercy_timer_settings: Vec<Option<AIMercyTimerSettings>>,
    #[serde(default)]
    pub aivisual_field_params: Vec<Option<AIVisualFieldParams>>,
    #[serde(default)]
    pub aicontextual_visual_field_profile: Vec<Option<AIContextualVisualFieldProfile>>,
    #[serde(default)]
    pub aivisual_field_profile: Vec<Option<AIVisualFieldProfile>>,
    #[serde(default)]
    pub aiobservable_filter_flags: Vec<Option<AIObservableFilterFlags>>,
    #[serde(default)]
    pub aiobservable_filters: Vec<Option<AIObservableFilters>>,
    #[serde(default)]
    pub aiobservable_filters_profile: Vec<Option<AIObservableFiltersProfile>>,
    #[serde(default)]
    pub movement_system_additional_params: Vec<Option<MovementSystemAdditionalParams>>,
    #[serde(default)]
    pub movement_system_additional_params_record: Vec<Option<MovementSystemAdditionalParamsRecord>>,
    #[serde(default)]
    pub aispecial_ranged_attack_config: Vec<Option<AISpecialRangedAttackConfig>>,
    #[serde(default)]
    pub aiavailable_special_ranged_attacks_config: Vec<Option<AIAvailableSpecialRangedAttacksConfig>>,
    #[serde(default)]
    pub aiprofile: Vec<Option<AIProfile>>,
    #[serde(default)]
    pub aiming: Vec<Option<Aiming>>,
    #[serde(default)]
    pub burst: Vec<Option<Burst>>,
    #[serde(default)]
    pub aitime_since_target_seen: Vec<Option<AITimeSinceTargetSeen>>,
    #[serde(default)]
    pub character_accuracy_modifiers: Vec<Option<CharacterAccuracyModifiers>>,
    #[serde(default)]
    pub character_skills: Vec<Option<CharacterSkills>>,
    #[serde(default)]
    pub seat_operator_skills: Vec<Option<SeatOperatorSkills>>,
    #[serde(default)]
    pub common_targeting_same_target_score: Vec<Option<CommonTargetingSameTargetScore>>,
    #[serde(default)]
    pub common_target_visibility_score: Vec<Option<CommonTargetVisibilityScore>>,
    #[serde(default)]
    pub common_current_target_distance_score: Vec<Option<CommonCurrentTargetDistanceScore>>,
    #[serde(default)]
    pub common_tactic_scores: Vec<Option<CommonTacticScores>>,
    #[serde(default)]
    pub tactic_scoring_profile: Vec<Option<TacticScoringProfile>>,
    #[serde(default)]
    pub shared_tactic_params: Vec<Option<SharedTacticParams>>,
    #[serde(default)]
    pub shooting_params: Vec<Option<ShootingParams>>,
    #[serde(default)]
    pub tactic_player_distance: Vec<Option<TacticPlayerDistance>>,
    #[serde(default)]
    pub skill_definitions: Vec<Option<SkillDefinitions>>,
    #[serde(default)]
    pub skill: Vec<Option<Skill>>,
    #[serde(default)]
    pub stat_definitions: Vec<Option<StatDefinitions>>,
    #[serde(default)]
    pub stat: Vec<Option<Stat>>,
    #[serde(default)]
    pub stat_influence: Vec<Option<StatInfluence>>,
}
