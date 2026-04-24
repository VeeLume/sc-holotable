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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `TrackWheeledVehicleAudioComponentParams`
/// Inherits from: `GroundVehicleAudioComponentParams`
pub struct TrackWheeledVehicleAudioComponentParams {
    /// `engineSoundPosition` (String)
    pub engine_sound_position: String,
    /// `engineStartupOneShot` (Class)
    pub engine_startup_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// `engineRunLoopStart` (Class)
    pub engine_run_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `engineRunLoopStop` (Class)
    pub engine_run_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `hornSoundPosition` (String)
    pub horn_sound_position: String,
    /// `hornLoopStart` (Class)
    pub horn_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `hornLoopStop` (Class)
    pub horn_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `suspensionBumpSound` (Class)
    pub suspension_bump_sound: Option<Handle<GlobalResourceAudio>>,
    /// `gearChangeSound` (Class)
    pub gear_change_sound: Option<Handle<GlobalResourceAudio>>,
    /// `throttleOnSound` (Class)
    pub throttle_on_sound: Option<Handle<GlobalResourceAudio>>,
    /// `throttleOffSound` (Class)
    pub throttle_off_sound: Option<Handle<GlobalResourceAudio>>,
    /// `damagedSound` (Class)
    pub damaged_sound: Option<Handle<GlobalResourceAudio>>,
    /// `destroyedSound` (Class)
    pub destroyed_sound: Option<Handle<GlobalResourceAudio>>,
    /// `boostStartTrigger` (Class)
    pub boost_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `boostStopTrigger` (Class)
    pub boost_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `speedRtpc` (Class)
    pub speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleRtpc` (Class)
    pub throttle_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleLoadRtpc` (Class)
    pub throttle_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleOnAtLoadRtpc` (Class)
    pub throttle_on_at_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `throttleOffAtLoadRtpc` (Class)
    pub throttle_off_at_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmRtpc` (Class)
    pub rpm_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmLoadRtpc` (Class)
    pub rpm_load_rtpc: Option<Handle<AudioRtpc>>,
    /// `rpmAccelerationRtpc` (Class)
    pub rpm_acceleration_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipRtpc` (Class)
    pub slip_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipForwardsRtpc` (Class)
    pub slip_forwards_rtpc: Option<Handle<AudioRtpc>>,
    /// `slipLateralRtpc` (Class)
    pub slip_lateral_rtpc: Option<Handle<AudioRtpc>>,
    /// `steerRtpc` (Class)
    pub steer_rtpc: Option<Handle<AudioRtpc>>,
    /// `steerStrainRtpc` (Class)
    pub steer_strain_rtpc: Option<Handle<AudioRtpc>>,
    /// `groundContactsRtpc` (Class)
    pub ground_contacts_rtpc: Option<Handle<AudioRtpc>>,
    /// `airTimeRtpc` (Class)
    pub air_time_rtpc: Option<Handle<AudioRtpc>>,
    /// `suspensionCompressionRtpc` (Class)
    pub suspension_compression_rtpc: Option<Handle<AudioRtpc>>,
    /// `suspensionStrokeRtpc` (Class)
    pub suspension_stroke_rtpc: Option<Handle<AudioRtpc>>,
    /// `bumpCompressionRtpc` (Class)
    pub bump_compression_rtpc: Option<Handle<AudioRtpc>>,
    /// `surfaceRoughnessRtpc` (Class)
    pub surface_roughness_rtpc: Option<Handle<AudioRtpc>>,
    /// `boostDurationRtpc` (Class)
    pub boost_duration_rtpc: Option<Handle<AudioRtpc>>,
    /// `remainingBoostRtpc` (Class)
    pub remaining_boost_rtpc: Option<Handle<AudioRtpc>>,
    /// `bumpMinSusp` (Single)
    pub bump_min_susp: f32,
    /// `bumpMinSpeed` (Single)
    pub bump_min_speed: f32,
    /// `bumpIntensityMult` (Single)
    pub bump_intensity_mult: f32,
    /// `bumpMinTime` (Single)
    pub bump_min_time: f32,
    /// `throttleLoadLerpUpTime` (Single)
    pub throttle_load_lerp_up_time: f32,
    /// `throttleLoadLerpDownTime` (Single)
    pub throttle_load_lerp_down_time: f32,
    /// `skidLerpSpeed` (Single)
    pub skid_lerp_speed: f32,
    /// `skidCentrifugalFactor` (Single)
    pub skid_centrifugal_factor: f32,
    /// `skidBrakeFactor` (Single)
    pub skid_brake_factor: f32,
    /// `skidPowerLockFactor` (Single)
    pub skid_power_lock_factor: f32,
    /// `skidLateralFactor` (Single)
    pub skid_lateral_factor: f32,
    /// `skidForwardFactor` (Single)
    pub skid_forward_factor: f32,
    /// `treadRotationSpeedRtpc` (Class)
    pub tread_rotation_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadGroundContactRtpc` (Class)
    pub tread_ground_contact_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadCountRtpc` (Class)
    pub tread_count_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadIndexRtpc` (Class)
    pub tread_index_rtpc: Option<Handle<AudioRtpc>>,
    /// `treadAudioSurfaceMap` (Reference)
    pub tread_audio_surface_map: Option<CigGuid>,
}

impl Pooled for TrackWheeledVehicleAudioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_groundvehicles
            .track_wheeled_vehicle_audio_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_groundvehicles
            .track_wheeled_vehicle_audio_component_params
    }
}

impl<'a> Extract<'a> for TrackWheeledVehicleAudioComponentParams {
    const TYPE_NAME: &'static str = "TrackWheeledVehicleAudioComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            engine_sound_position: inst
                .get_str("engineSoundPosition")
                .map(String::from)
                .unwrap_or_default(),
            engine_startup_one_shot: match inst.get("engineStartupOneShot") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            engine_run_loop_start: match inst.get("engineRunLoopStart") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            engine_run_loop_stop: match inst.get("engineRunLoopStop") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            horn_sound_position: inst
                .get_str("hornSoundPosition")
                .map(String::from)
                .unwrap_or_default(),
            horn_loop_start: match inst.get("hornLoopStart") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            horn_loop_stop: match inst.get("hornLoopStop") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            suspension_bump_sound: match inst.get("suspensionBumpSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            gear_change_sound: match inst.get("gearChangeSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            throttle_on_sound: match inst.get("throttleOnSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            throttle_off_sound: match inst.get("throttleOffSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            damaged_sound: match inst.get("damagedSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            destroyed_sound: match inst.get("destroyedSound") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            boost_start_trigger: match inst.get("boostStartTrigger") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            boost_stop_trigger: match inst.get("boostStopTrigger") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            speed_rtpc: match inst.get("speedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            throttle_rtpc: match inst.get("throttleRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            throttle_load_rtpc: match inst.get("throttleLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            throttle_on_at_load_rtpc: match inst.get("throttleOnAtLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            throttle_off_at_load_rtpc: match inst.get("throttleOffAtLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            rpm_rtpc: match inst.get("rpmRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            rpm_load_rtpc: match inst.get("rpmLoadRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            rpm_acceleration_rtpc: match inst.get("rpmAccelerationRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            slip_rtpc: match inst.get("slipRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            slip_forwards_rtpc: match inst.get("slipForwardsRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            slip_lateral_rtpc: match inst.get("slipLateralRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            steer_rtpc: match inst.get("steerRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            steer_strain_rtpc: match inst.get("steerStrainRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            ground_contacts_rtpc: match inst.get("groundContactsRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            air_time_rtpc: match inst.get("airTimeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            suspension_compression_rtpc: match inst.get("suspensionCompressionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            suspension_stroke_rtpc: match inst.get("suspensionStrokeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            bump_compression_rtpc: match inst.get("bumpCompressionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            surface_roughness_rtpc: match inst.get("surfaceRoughnessRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            boost_duration_rtpc: match inst.get("boostDurationRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            remaining_boost_rtpc: match inst.get("remainingBoostRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            bump_min_susp: inst.get_f32("bumpMinSusp").unwrap_or_default(),
            bump_min_speed: inst.get_f32("bumpMinSpeed").unwrap_or_default(),
            bump_intensity_mult: inst.get_f32("bumpIntensityMult").unwrap_or_default(),
            bump_min_time: inst.get_f32("bumpMinTime").unwrap_or_default(),
            throttle_load_lerp_up_time: inst.get_f32("throttleLoadLerpUpTime").unwrap_or_default(),
            throttle_load_lerp_down_time: inst
                .get_f32("throttleLoadLerpDownTime")
                .unwrap_or_default(),
            skid_lerp_speed: inst.get_f32("skidLerpSpeed").unwrap_or_default(),
            skid_centrifugal_factor: inst.get_f32("skidCentrifugalFactor").unwrap_or_default(),
            skid_brake_factor: inst.get_f32("skidBrakeFactor").unwrap_or_default(),
            skid_power_lock_factor: inst.get_f32("skidPowerLockFactor").unwrap_or_default(),
            skid_lateral_factor: inst.get_f32("skidLateralFactor").unwrap_or_default(),
            skid_forward_factor: inst.get_f32("skidForwardFactor").unwrap_or_default(),
            tread_rotation_speed_rtpc: match inst.get("treadRotationSpeedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            tread_ground_contact_rtpc: match inst.get("treadGroundContactRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            tread_count_rtpc: match inst.get("treadCountRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            tread_index_rtpc: match inst.get("treadIndexRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            tread_audio_surface_map: inst
                .get("treadAudioSurfaceMap")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
