// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `jumppoints` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JumppointsPools {
    #[serde(default)]
    pub sjump_tunnel_visual_params: Vec<Option<SJumpTunnelVisualParams>>,
    #[serde(default)]
    pub sspread_misfire_effect: Vec<Option<SSpreadMisfireEffect>>,
    #[serde(default)]
    pub global_jump_point_tuning_params: Vec<Option<GlobalJumpPointTuningParams>>,
    #[serde(default)]
    pub global_jump_point_opening_params: Vec<Option<GlobalJumpPointOpeningParams>>,
    #[serde(default)]
    pub global_jump_point_closing_params: Vec<Option<GlobalJumpPointClosingParams>>,
    #[serde(default)]
    pub global_jump_point_effect_params: Vec<Option<GlobalJumpPointEffectParams>>,
    #[serde(default)]
    pub jump_drive_uicone_params: Vec<Option<JumpDriveUIConeParams>>,
    #[serde(default)]
    pub global_jump_point_params: Vec<Option<GlobalJumpPointParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_probability_params: Vec<Option<SJumpTunnelSectionProbabilityParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_control_point_generation_params: Vec<Option<SJumpTunnelSectionControlPointGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_obstacle_generation_params: Vec<Option<SJumpTunnelObstacleGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_elliptical_params: Vec<Option<SJumpTunnelEllipticalParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_generation_params: Vec<Option<SJumpTunnelSectionGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_generation_params: Vec<Option<SJumpTunnelGenerationParams>>,
    #[serde(default)]
    pub global_jump_tunnel_light_params: Vec<Option<GlobalJumpTunnelLightParams>>,
    #[serde(default)]
    pub global_jump_tunnel_fog_params: Vec<Option<GlobalJumpTunnelFogParams>>,
    #[serde(default)]
    pub global_jump_tunnel_pass_by_light_params: Vec<Option<GlobalJumpTunnelPassByLightParams>>,
    #[serde(default)]
    pub global_jump_tunnel_probe_params: Vec<Option<GlobalJumpTunnelProbeParams>>,
    #[serde(default)]
    pub global_jump_tunnel_effect_params: Vec<Option<GlobalJumpTunnelEffectParams>>,
    #[serde(default)]
    pub jump_tunnel_camera_effect_param: Vec<Option<JumpTunnelCameraEffectParam>>,
    #[serde(default)]
    pub jump_tunnel_camera_effects: Vec<Option<JumpTunnelCameraEffects>>,
    #[serde(default)]
    pub jump_drive_velocity_strength_params: Vec<Option<JumpDriveVelocityStrengthParams>>,
    #[serde(default)]
    pub global_jump_tunnel_camera_effect_params: Vec<Option<GlobalJumpTunnelCameraEffectParams>>,
    #[serde(default)]
    pub sjump_tunnel_distortion_params: Vec<Option<SJumpTunnelDistortionParams>>,
    #[serde(default)]
    pub sjump_tunnel_failure_params: Vec<Option<SJumpTunnelFailureParams>>,
    #[serde(default)]
    pub sjump_tunnel_exit_params: Vec<Option<SJumpTunnelExitParams>>,
    #[serde(default)]
    pub global_jump_tunnel_host_params: Vec<Option<GlobalJumpTunnelHostParams>>,
    #[serde(default)]
    pub jump_drive_state_audio_map: Vec<Option<JumpDriveStateAudioMap>>,
    #[serde(default)]
    pub jump_drive_audio_movement_params: Vec<Option<JumpDriveAudioMovementParams>>,
    #[serde(default)]
    pub jump_drive_audio_params: Vec<Option<JumpDriveAudioParams>>,
    #[serde(default)]
    pub jump_drive_music_event: Vec<Option<JumpDriveMusicEvent>>,
    #[serde(default)]
    pub jump_drive_state_music_map: Vec<Option<JumpDriveStateMusicMap>>,
    #[serde(default)]
    pub jump_drive_music_params: Vec<Option<JumpDriveMusicParams>>,
    #[serde(default)]
    pub global_jump_drive_tuning_effect_params: Vec<Option<GlobalJumpDriveTuningEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_entry_effect_params: Vec<Option<GlobalJumpDriveEntryEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_exit_effect_params: Vec<Option<GlobalJumpDriveExitEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_effect_params: Vec<Option<GlobalJumpDriveEffectParams>>,
    #[serde(default)]
    pub jump_drive_approach_rings_params: Vec<Option<JumpDriveApproachRingsParams>>,
    #[serde(default)]
    pub global_jump_drive_params: Vec<Option<GlobalJumpDriveParams>>,
    #[serde(default)]
    pub jump_travel_camera_params: Vec<Option<JumpTravelCameraParams>>,
}
