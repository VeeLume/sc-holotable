// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-groundvehicles`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `TrackWheeledVehicleAudioComponentParams`
/// Inherits from: `GroundVehicleAudioComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackWheeledVehicleAudioComponentParams {
    /// `engineSoundPosition` (String)
    #[serde(default)]
    pub engine_sound_position: String,
    /// `engineStartupOneShot` (Class)
    #[serde(default)]
    pub engine_startup_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// `engineRunLoopStart` (Class)
    #[serde(default)]
    pub engine_run_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `engineRunLoopStop` (Class)
    #[serde(default)]
    pub engine_run_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `hornSoundPosition` (String)
    #[serde(default)]
    pub horn_sound_position: String,
    /// `hornLoopStart` (Class)
    #[serde(default)]
    pub horn_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `hornLoopStop` (Class)
    #[serde(default)]
    pub horn_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `suspensionBumpSound` (Class)
    #[serde(default)]
    pub suspension_bump_sound: Option<Handle<GlobalResourceAudio>>,
    /// `gearChangeSound` (Class)
    #[serde(default)]
    pub gear_change_sound: Option<Handle<GlobalResourceAudio>>,
    /// `throttleOnSound` (Class)
    #[serde(default)]
    pub throttle_on_sound: Option<Handle<GlobalResourceAudio>>,
    /// `throttleOffSound` (Class)
    #[serde(default)]
    pub throttle_off_sound: Option<Handle<GlobalResourceAudio>>,
    /// `damagedSound` (Class)
    #[serde(default)]
    pub damaged_sound: Option<Handle<GlobalResourceAudio>>,
    /// `destroyedSound` (Class)
    #[serde(default)]
    pub destroyed_sound: Option<Handle<GlobalResourceAudio>>,
    /// `boostStartTrigger` (Class)
    #[serde(default)]
    pub boost_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `boostStopTrigger` (Class)
    #[serde(default)]
    pub boost_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `speedRtpc` (Class)
    #[serde(default)]
    pub speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleRtpc` (Class)
    #[serde(default)]
    pub throttle_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleLoadRtpc` (Class)
    #[serde(default)]
    pub throttle_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleOnAtLoadRtpc` (Class)
    #[serde(default)]
    pub throttle_on_at_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleOffAtLoadRtpc` (Class)
    #[serde(default)]
    pub throttle_off_at_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmRtpc` (Class)
    #[serde(default)]
    pub rpm_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmLoadRtpc` (Class)
    #[serde(default)]
    pub rpm_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmAccelerationRtpc` (Class)
    #[serde(default)]
    pub rpm_acceleration_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipRtpc` (Class)
    #[serde(default)]
    pub slip_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipForwardsRtpc` (Class)
    #[serde(default)]
    pub slip_forwards_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipLateralRtpc` (Class)
    #[serde(default)]
    pub slip_lateral_rtpc: Option<Handle<AudioRtpc>>,
    /// `steerRtpc` (Class)
    #[serde(default)]
    pub steer_rtpc: Option<Handle<AudioRtpc>>,
    /// `steerStrainRtpc` (Class)
    #[serde(default)]
    pub steer_strain_rtpc: Option<Handle<AudioRtpc>>,
    /// `groundContactsRtpc` (Class)
    #[serde(default)]
    pub ground_contacts_rtpc: Option<Handle<AudioRtpc>>,
    /// `airTimeRtpc` (Class)
    #[serde(default)]
    pub air_time_rtpc: Option<Handle<AudioRtpc>>,
    /// `suspensionCompressionRtpc` (Class)
    #[serde(default)]
    pub suspension_compression_rtpc: Option<Handle<AudioRtpc>>,
    /// `suspensionStrokeRtpc` (Class)
    #[serde(default)]
    pub suspension_stroke_rtpc: Option<Handle<AudioRtpc>>,
    /// `bumpCompressionRtpc` (Class)
    #[serde(default)]
    pub bump_compression_rtpc: Option<Handle<AudioRtpc>>,
    /// `surfaceRoughnessRtpc` (Class)
    #[serde(default)]
    pub surface_roughness_rtpc: Option<Handle<AudioRtpc>>,
    /// `boostDurationRtpc` (Class)
    #[serde(default)]
    pub boost_duration_rtpc: Option<Handle<AudioRtpc>>,
    /// `remainingBoostRtpc` (Class)
    #[serde(default)]
    pub remaining_boost_rtpc: Option<Handle<AudioRtpc>>,
    /// `bumpMinSusp` (Single)
    #[serde(default)]
    pub bump_min_susp: f32,
    /// `bumpMinSpeed` (Single)
    #[serde(default)]
    pub bump_min_speed: f32,
    /// `bumpIntensityMult` (Single)
    #[serde(default)]
    pub bump_intensity_mult: f32,
    /// `bumpMinTime` (Single)
    #[serde(default)]
    pub bump_min_time: f32,
    /// `throttleLoadLerpUpTime` (Single)
    #[serde(default)]
    pub throttle_load_lerp_up_time: f32,
    /// `throttleLoadLerpDownTime` (Single)
    #[serde(default)]
    pub throttle_load_lerp_down_time: f32,
    /// `skidLerpSpeed` (Single)
    #[serde(default)]
    pub skid_lerp_speed: f32,
    /// `skidCentrifugalFactor` (Single)
    #[serde(default)]
    pub skid_centrifugal_factor: f32,
    /// `skidBrakeFactor` (Single)
    #[serde(default)]
    pub skid_brake_factor: f32,
    /// `skidPowerLockFactor` (Single)
    #[serde(default)]
    pub skid_power_lock_factor: f32,
    /// `skidLateralFactor` (Single)
    #[serde(default)]
    pub skid_lateral_factor: f32,
    /// `skidForwardFactor` (Single)
    #[serde(default)]
    pub skid_forward_factor: f32,
    /// `treadRotationSpeedRtpc` (Class)
    #[serde(default)]
    pub tread_rotation_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadGroundContactRtpc` (Class)
    #[serde(default)]
    pub tread_ground_contact_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadCountRtpc` (Class)
    #[serde(default)]
    pub tread_count_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadIndexRtpc` (Class)
    #[serde(default)]
    pub tread_index_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadAudioSurfaceMap` (Reference)
    #[serde(default)]
    pub tread_audio_surface_map: Option<CigGuid>,
}

impl Pooled for TrackWheeledVehicleAudioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_groundvehicles.track_wheeled_vehicle_audio_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_groundvehicles.track_wheeled_vehicle_audio_component_params }
}

impl<'a> Extract<'a> for TrackWheeledVehicleAudioComponentParams {
    const TYPE_NAME: &'static str = "TrackWheeledVehicleAudioComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            engine_sound_position: inst.get_str("engineSoundPosition").map(String::from).unwrap_or_default(),
            engine_startup_one_shot: match inst.get("engineStartupOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            engine_run_loop_start: match inst.get("engineRunLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            engine_run_loop_stop: match inst.get("engineRunLoopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            horn_sound_position: inst.get_str("hornSoundPosition").map(String::from).unwrap_or_default(),
            horn_loop_start: match inst.get("hornLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            horn_loop_stop: match inst.get("hornLoopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            suspension_bump_sound: match inst.get("suspensionBumpSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gear_change_sound: match inst.get("gearChangeSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_on_sound: match inst.get("throttleOnSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_off_sound: match inst.get("throttleOffSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            damaged_sound: match inst.get("damagedSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            destroyed_sound: match inst.get("destroyedSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            boost_start_trigger: match inst.get("boostStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            boost_stop_trigger: match inst.get("boostStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            speed_rtpc: match inst.get("speedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_rtpc: match inst.get("throttleRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_load_rtpc: match inst.get("throttleLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_on_at_load_rtpc: match inst.get("throttleOnAtLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            throttle_off_at_load_rtpc: match inst.get("throttleOffAtLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rpm_rtpc: match inst.get("rpmRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rpm_load_rtpc: match inst.get("rpmLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rpm_acceleration_rtpc: match inst.get("rpmAccelerationRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            slip_rtpc: match inst.get("slipRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            slip_forwards_rtpc: match inst.get("slipForwardsRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            slip_lateral_rtpc: match inst.get("slipLateralRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            steer_rtpc: match inst.get("steerRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            steer_strain_rtpc: match inst.get("steerStrainRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ground_contacts_rtpc: match inst.get("groundContactsRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            air_time_rtpc: match inst.get("airTimeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            suspension_compression_rtpc: match inst.get("suspensionCompressionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            suspension_stroke_rtpc: match inst.get("suspensionStrokeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bump_compression_rtpc: match inst.get("bumpCompressionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            surface_roughness_rtpc: match inst.get("surfaceRoughnessRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            boost_duration_rtpc: match inst.get("boostDurationRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            remaining_boost_rtpc: match inst.get("remainingBoostRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bump_min_susp: inst.get_f32("bumpMinSusp").unwrap_or_default(),
            bump_min_speed: inst.get_f32("bumpMinSpeed").unwrap_or_default(),
            bump_intensity_mult: inst.get_f32("bumpIntensityMult").unwrap_or_default(),
            bump_min_time: inst.get_f32("bumpMinTime").unwrap_or_default(),
            throttle_load_lerp_up_time: inst.get_f32("throttleLoadLerpUpTime").unwrap_or_default(),
            throttle_load_lerp_down_time: inst.get_f32("throttleLoadLerpDownTime").unwrap_or_default(),
            skid_lerp_speed: inst.get_f32("skidLerpSpeed").unwrap_or_default(),
            skid_centrifugal_factor: inst.get_f32("skidCentrifugalFactor").unwrap_or_default(),
            skid_brake_factor: inst.get_f32("skidBrakeFactor").unwrap_or_default(),
            skid_power_lock_factor: inst.get_f32("skidPowerLockFactor").unwrap_or_default(),
            skid_lateral_factor: inst.get_f32("skidLateralFactor").unwrap_or_default(),
            skid_forward_factor: inst.get_f32("skidForwardFactor").unwrap_or_default(),
            tread_rotation_speed_rtpc: match inst.get("treadRotationSpeedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tread_ground_contact_rtpc: match inst.get("treadGroundContactRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tread_count_rtpc: match inst.get("treadCountRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tread_index_rtpc: match inst.get("treadIndexRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tread_audio_surface_map: inst.get("treadAudioSurfaceMap").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

