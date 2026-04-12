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

/// Pool storage for the `radarsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RadarsystemPools {
    #[serde(default)]
    pub radar_system_global_params: Vec<Option<RadarSystemGlobalParams>>,
    #[serde(default)]
    pub master_mode_switch_delta_signature_types: Vec<Option<MasterModeSwitchDeltaSignatureTypes>>,
    #[serde(default)]
    pub signature_system_global_params: Vec<Option<SignatureSystemGlobalParams>>,
    #[serde(default)]
    pub contact_state_global_params: Vec<Option<ContactStateGlobalParams>>,
    #[serde(default)]
    pub signature_uiglobal_params: Vec<Option<SignatureUIGlobalParams>>,
    #[serde(default)]
    pub actor_signature_multiplier_global_params: Vec<Option<ActorSignatureMultiplierGlobalParams>>,
    #[serde(default)]
    pub cross_section_global_params: Vec<Option<CrossSectionGlobalParams>>,
    #[serde(default)]
    pub signature_type_global_params: Vec<Option<SignatureTypeGlobalParams>>,
    #[serde(default)]
    pub scan_wave_detection_params: Vec<Option<ScanWaveDetectionParams>>,
    #[serde(default)]
    pub delta_signature_sensitivity_params: Vec<Option<DeltaSignatureSensitivityParams>>,
    #[serde(default)]
    pub radar_delta_signature_detection_params: Vec<Option<RadarDeltaSignatureDetectionParams>>,
    #[serde(default)]
    pub delta_signature_spike_params: Vec<Option<DeltaSignatureSpikeParams>>,
    #[serde(default)]
    pub room_traversal_operations_params: Vec<Option<RoomTraversalOperationsParams>>,
    #[serde(default)]
    pub room_traversal_contact_type_entry: Vec<Option<RoomTraversalContactTypeEntry>>,
    #[serde(default)]
    pub room_traversal_params: Vec<Option<RoomTraversalParams>>,
    #[serde(default)]
    pub room_shared_params: Vec<Option<RoomSharedParams>>,
    #[serde(default)]
    pub radar_system_shared_params: Vec<Option<RadarSystemSharedParams>>,
    #[serde(default)]
    pub radar_shared_params: Vec<Option<RadarSharedParams>>,
    #[serde(default)]
    pub radar_jammer_shared_params: Vec<Option<RadarJammerSharedParams>>,
    #[serde(default)]
    pub scan_shared_params: Vec<Option<ScanSharedParams>>,
    #[serde(default)]
    pub scan_sfxshared_params: Vec<Option<ScanSFXSharedParams>>,
    #[serde(default)]
    pub ping_shared_params: Vec<Option<PingSharedParams>>,
    #[serde(default)]
    pub ping_blob_life_time: Vec<Option<PingBlobLifeTime>>,
    #[serde(default)]
    pub ping_extended_life_time_params: Vec<Option<PingExtendedLifeTimeParams>>,
    #[serde(default)]
    pub ping_contact_life_time: Vec<Option<PingContactLifeTime>>,
    #[serde(default)]
    pub blob_vfxshared_params: Vec<Option<BlobVFXSharedParams>>,
    #[serde(default)]
    pub blob_vfxdistance_params: Vec<Option<BlobVFXDistanceParams>>,
    #[serde(default)]
    pub ping_sfxshared_params: Vec<Option<PingSFXSharedParams>>,
    #[serde(default)]
    pub ping_wave_vfxparams: Vec<Option<PingWaveVFXParams>>,
    #[serde(default)]
    pub ping_vfxshared_params: Vec<Option<PingVFXSharedParams>>,
    #[serde(default)]
    pub ping_type_params: Vec<Option<PingTypeParams>>,
    #[serde(default)]
    pub contact_highlight_layers_params: Vec<Option<ContactHighlightLayersParams>>,
    #[serde(default)]
    pub contact_highlight_shared_params: Vec<Option<ContactHighlightSharedParams>>,
    #[serde(default)]
    pub contact_highlight_visual_base_params: Vec<Option<ContactHighlightVisualBaseParams>>,
    #[serde(default)]
    pub contact_highlight_state_params: Vec<Option<ContactHighlightStateParams>>,
    #[serde(default)]
    pub contact_tagging_shared_params: Vec<Option<ContactTaggingSharedParams>>,
    #[serde(default)]
    pub occlusion_check_shared_params: Vec<Option<OcclusionCheckSharedParams>>,
    #[serde(default)]
    pub scan_information_def: Vec<Option<ScanInformationDef>>,
    #[serde(default)]
    pub scan_information_table: Vec<Option<ScanInformationTable>>,
    #[serde(default)]
    pub scan_boxout_icon_def: Vec<Option<ScanBoxoutIconDef>>,
    #[serde(default)]
    pub scan_custom_data_def: Vec<Option<ScanCustomDataDef>>,
    #[serde(default)]
    pub scan_display_section_params: Vec<Option<ScanDisplaySectionParams>>,
    #[serde(default)]
    pub scan_display_layout_params: Vec<Option<ScanDisplayLayoutParams>>,
    #[serde(default)]
    pub radar_contact_game_play_properties: Vec<Option<RadarContactGamePlayProperties>>,
    #[serde(default)]
    pub radar_signature_category_definition: Vec<Option<RadarSignatureCategoryDefinition>>,
    #[serde(default)]
    pub radar_signature_category_entry: Vec<Option<RadarSignatureCategoryEntry>>,
    #[serde(default)]
    pub radar_contact_type_definition: Vec<Option<RadarContactTypeDefinition>>,
    #[serde(default)]
    pub radar_contact_type_entry: Vec<Option<RadarContactTypeEntry>>,
    #[serde(default)]
    pub radar_contact_group_definition: Vec<Option<RadarContactGroupDefinition>>,
    #[serde(default)]
    pub radar_contact_group_entry: Vec<Option<RadarContactGroupEntry>>,
    #[serde(default)]
    pub radar_contact_sub_group_entry: Vec<Option<RadarContactSubGroupEntry>>,
    #[serde(default)]
    pub radar_delta_signature_notification_params: Vec<Option<RadarDeltaSignatureNotificationParams>>,
    #[serde(default)]
    pub radar_delta_signature_definition: Vec<Option<RadarDeltaSignatureDefinition>>,
    #[serde(default)]
    pub radar_delta_signature_entry: Vec<Option<RadarDeltaSignatureEntry>>,
    #[serde(default)]
    pub scan_procedure_params: Vec<Option<ScanProcedureParams>>,
}
