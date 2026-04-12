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

/// Pool storage for the `globalquantumdriveparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalquantumdriveparamsPools {
    #[serde(default)]
    pub quantum_drive_spline_rollback_params: Vec<Option<QuantumDriveSplineRollbackParams>>,
    #[serde(default)]
    pub quantum_drive_spline_fxnode: Vec<Option<QuantumDriveSplineFXNode>>,
    #[serde(default)]
    pub quantum_drive_spline_fxparams: Vec<Option<QuantumDriveSplineFXParams>>,
    #[serde(default)]
    pub quantum_drive_spline_traversal_params: Vec<Option<QuantumDriveSplineTraversalParams>>,
    #[serde(default)]
    pub quantum_drive_notification: Vec<Option<QuantumDriveNotification>>,
    #[serde(default)]
    pub quantum_drive_notifications: Vec<Option<QuantumDriveNotifications>>,
    #[serde(default)]
    pub quantum_drive_audio_params: Vec<Option<QuantumDriveAudioParams>>,
    #[serde(default)]
    pub quantum_drive_global_params: Vec<Option<QuantumDriveGlobalParams>>,
    #[serde(default)]
    pub quantum_music_event_base: Vec<Option<QuantumMusicEventBase>>,
    #[serde(default)]
    pub quantum_drive_state_music_map: Vec<Option<QuantumDriveStateMusicMap>>,
    #[serde(default)]
    pub quantum_state_music_map: Vec<Option<QuantumStateMusicMap>>,
    #[serde(default)]
    pub quantum_drive_loc_type_music_map: Vec<Option<QuantumDriveLocTypeMusicMap>>,
    #[serde(default)]
    pub quantum_effect_music: Vec<Option<QuantumEffectMusic>>,
    #[serde(default)]
    pub quantum_music_trip_category: Vec<Option<QuantumMusicTripCategory>>,
    #[serde(default)]
    pub quantum_music_params: Vec<Option<QuantumMusicParams>>,
}
