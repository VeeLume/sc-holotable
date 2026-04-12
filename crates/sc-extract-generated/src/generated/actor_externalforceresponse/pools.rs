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

/// Pool storage for the `actor-externalforceresponse` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorExternalforceresponsePools {
    #[serde(default)]
    pub sactor_force_reaction_filter_range_override_def: Vec<Option<SActorForceReactionFilterRangeOverrideDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filter_item_def: Vec<Option<SActorForceReactionFilterItemDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_filter_item_def: Vec<Option<SActorForceReactionLeanFilterItemDef>>,
    #[serde(default)]
    pub sactor_force_reaction_limit_def: Vec<Option<SActorForceReactionLimitDef>>,
    #[serde(default)]
    pub sactor_force_reaction_state_config: Vec<Option<SActorForceReactionStateConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_filter_def: Vec<Option<SActorForceReactionFilterDef>>,
    #[serde(default)]
    pub sactor_force_reaction_envelope: Vec<Option<SActorForceReactionEnvelope>>,
    #[serde(default)]
    pub sactor_force_reaction_curve: Vec<Option<SActorForceReactionCurve>>,
    #[serde(default)]
    pub sactor_force_reaction_curve_config: Vec<Option<SActorForceReactionCurveConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_effect_defaults: Vec<Option<SActorForceReactionEffectDefaults>>,
    #[serde(default)]
    pub sactor_force_reaction_global_effect_config: Vec<Option<SActorForceReactionGlobalEffectConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_aim_punch_config: Vec<Option<SActorForceReactionAimPunchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_weapon_twitch_config: Vec<Option<SActorForceReactionWeaponTwitchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_head_recoil_config: Vec<Option<SActorForceReactionHeadRecoilConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_fovscale_config: Vec<Option<SActorForceReactionFOVScaleConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_twitch_config: Vec<Option<SActorForceReactionAnimationTwitchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_flinch_config: Vec<Option<SActorForceReactionAnimationFlinchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_stagger_config: Vec<Option<SActorForceReactionAnimationStaggerConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_stagger_tag_config: Vec<Option<SActorForceReactionStaggerTagConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_global_stagger_config: Vec<Option<SActorForceReactionGlobalStaggerConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_impulse_accumulation_config: Vec<Option<SActorForceReactionImpulseAccumulationConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_flight_duration_config: Vec<Option<SActorForceReactionFlightDurationConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_movement_launch_config: Vec<Option<SActorForceReactionMovementLaunchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_block_adsconfig: Vec<Option<SActorForceReactionBlockADSConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_block_config: Vec<Option<SActorForceReactionBlockConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_twitch_range_def: Vec<Option<SActorForceReactionTwitchRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_twitch_config_def: Vec<Option<SActorForceReactionTwitchConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_flinch_range_def: Vec<Option<SActorForceReactionFlinchRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_flinch_config_def: Vec<Option<SActorForceReactionFlinchConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_stagger_range_def: Vec<Option<SActorForceReactionStaggerRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filtered_stagger_range_def: Vec<Option<SActorForceReactionFilteredStaggerRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filtered_stagger_config_def: Vec<Option<SActorForceReactionFilteredStaggerConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_unfiltered_stagger_config_def: Vec<Option<SActorForceReactionUnfilteredStaggerConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_knockdown_range_def: Vec<Option<SActorForceReactionKnockdownRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_knockdown_config_def: Vec<Option<SActorForceReactionKnockdownConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_angle_limits_def: Vec<Option<SActorForceReactionLeanAngleLimitsDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_filter_def: Vec<Option<SActorForceReactionLeanFilterDef>>,
    #[serde(default)]
    pub sactor_force_reaction_procedural_lean_pose_list: Vec<Option<SActorForceReactionProceduralLeanPoseList>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_config_def: Vec<Option<SActorForceReactionLeanConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_stumble_config_def: Vec<Option<SActorForceReactionStumbleConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_sustained_impulse_def: Vec<Option<SActorForceReactionSustainedImpulseDef>>,
    #[serde(default)]
    pub sactor_force_reaction_external_impulse_def: Vec<Option<SActorForceReactionExternalImpulseDef>>,
    #[serde(default)]
    pub sactor_force_reaction_actor_bump_def: Vec<Option<SActorForceReactionActorBumpDef>>,
    #[serde(default)]
    pub sactor_force_reaction_sustained_force_def: Vec<Option<SActorForceReactionSustainedForceDef>>,
    #[serde(default)]
    pub sactor_force_reactions_stun_def: Vec<Option<SActorForceReactionsStunDef>>,
    #[serde(default)]
    pub sactor_force_reactions_vehicle_force_dampening_def: Vec<Option<SActorForceReactionsVehicleForceDampeningDef>>,
    #[serde(default)]
    pub sactor_force_reactions_def: Vec<Option<SActorForceReactionsDef>>,
    #[serde(default)]
    pub sactor_hit_reactions_def: Vec<Option<SActorHitReactionsDef>>,
    #[serde(default)]
    pub splayer_role_shake_multipliers: Vec<Option<SPlayerRoleShakeMultipliers>>,
    #[serde(default)]
    pub sactor_external_force_response_vibration_entry: Vec<Option<SActorExternalForceResponseVibrationEntry>>,
    #[serde(default)]
    pub sactor_external_force_response_camera_shake_def: Vec<Option<SActorExternalForceResponseCameraShakeDef>>,
    #[serde(default)]
    pub sactor_force_reactions_procedural_lean_override: Vec<Option<SActorForceReactionsProceduralLeanOverride>>,
    #[serde(default)]
    pub sactor_force_reactions_preset_record: Vec<Option<SActorForceReactionsPresetRecord>>,
    #[serde(default)]
    pub camera_actor_vibration_shake_config: Vec<Option<CameraActorVibrationShakeConfig>>,
}
