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

/// Pool storage for the `jumppoints` feature.
#[derive(Default)]
pub struct JumppointsPools {
    pub sjump_tunnel_visual_params: Vec<Option<SJumpTunnelVisualParams>>,
    pub sspread_misfire_effect: Vec<Option<SSpreadMisfireEffect>>,
    pub global_jump_point_tuning_params: Vec<Option<GlobalJumpPointTuningParams>>,
    pub global_jump_point_opening_params: Vec<Option<GlobalJumpPointOpeningParams>>,
    pub global_jump_point_closing_params: Vec<Option<GlobalJumpPointClosingParams>>,
    pub global_jump_point_effect_params: Vec<Option<GlobalJumpPointEffectParams>>,
    pub jump_drive_uicone_params: Vec<Option<JumpDriveUIConeParams>>,
    pub global_jump_point_params: Vec<Option<GlobalJumpPointParams>>,
    pub sjump_tunnel_section_probability_params: Vec<Option<SJumpTunnelSectionProbabilityParams>>,
    pub sjump_tunnel_section_control_point_generation_params: Vec<Option<SJumpTunnelSectionControlPointGenerationParams>>,
    pub sjump_tunnel_obstacle_generation_params: Vec<Option<SJumpTunnelObstacleGenerationParams>>,
    pub sjump_tunnel_elliptical_params: Vec<Option<SJumpTunnelEllipticalParams>>,
    pub sjump_tunnel_section_generation_params: Vec<Option<SJumpTunnelSectionGenerationParams>>,
    pub sjump_tunnel_generation_params: Vec<Option<SJumpTunnelGenerationParams>>,
    pub global_jump_tunnel_light_params: Vec<Option<GlobalJumpTunnelLightParams>>,
    pub global_jump_tunnel_fog_params: Vec<Option<GlobalJumpTunnelFogParams>>,
    pub global_jump_tunnel_pass_by_light_params: Vec<Option<GlobalJumpTunnelPassByLightParams>>,
    pub global_jump_tunnel_probe_params: Vec<Option<GlobalJumpTunnelProbeParams>>,
    pub global_jump_tunnel_effect_params: Vec<Option<GlobalJumpTunnelEffectParams>>,
    pub jump_tunnel_camera_effect_param: Vec<Option<JumpTunnelCameraEffectParam>>,
    pub jump_tunnel_camera_effects: Vec<Option<JumpTunnelCameraEffects>>,
    pub jump_drive_velocity_strength_params: Vec<Option<JumpDriveVelocityStrengthParams>>,
    pub global_jump_tunnel_camera_effect_params: Vec<Option<GlobalJumpTunnelCameraEffectParams>>,
    pub sjump_tunnel_distortion_params: Vec<Option<SJumpTunnelDistortionParams>>,
    pub sjump_tunnel_failure_params: Vec<Option<SJumpTunnelFailureParams>>,
    pub sjump_tunnel_exit_params: Vec<Option<SJumpTunnelExitParams>>,
    pub global_jump_tunnel_host_params: Vec<Option<GlobalJumpTunnelHostParams>>,
    pub jump_drive_state_audio_map: Vec<Option<JumpDriveStateAudioMap>>,
    pub jump_drive_audio_movement_params: Vec<Option<JumpDriveAudioMovementParams>>,
    pub jump_drive_audio_params: Vec<Option<JumpDriveAudioParams>>,
    pub jump_drive_music_event: Vec<Option<JumpDriveMusicEvent>>,
    pub jump_drive_state_music_map: Vec<Option<JumpDriveStateMusicMap>>,
    pub jump_drive_music_params: Vec<Option<JumpDriveMusicParams>>,
    pub global_jump_drive_tuning_effect_params: Vec<Option<GlobalJumpDriveTuningEffectParams>>,
    pub global_jump_drive_entry_effect_params: Vec<Option<GlobalJumpDriveEntryEffectParams>>,
    pub global_jump_drive_exit_effect_params: Vec<Option<GlobalJumpDriveExitEffectParams>>,
    pub global_jump_drive_effect_params: Vec<Option<GlobalJumpDriveEffectParams>>,
    pub jump_drive_approach_rings_params: Vec<Option<JumpDriveApproachRingsParams>>,
    pub global_jump_drive_params: Vec<Option<GlobalJumpDriveParams>>,
    pub jump_travel_camera_params: Vec<Option<JumpTravelCameraParams>>,
}
