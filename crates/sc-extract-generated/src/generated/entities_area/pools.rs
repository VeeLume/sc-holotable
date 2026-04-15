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

/// Pool storage for the `entities-area` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesAreaPools {
    #[serde(default)]
    pub designer_navigation_volume_params: Vec<Option<DesignerNavigationVolumeParams>>,
    #[serde(default)]
    pub planet_inclusion_volume_params: Vec<Option<PlanetInclusionVolumeParams>>,
    #[serde(default)]
    pub area_trigger_params: Vec<Option<AreaTriggerParams>>,
    #[serde(default)]
    pub gravity_box_params: Vec<Option<GravityBoxParams>>,
    #[serde(default)]
    pub harvestable_override_area_params: Vec<Option<HarvestableOverrideAreaParams>>,
    #[serde(default)]
    pub slock_camera_view_area_params: Vec<Option<SLockCameraViewAreaParams>>,
    #[serde(default)]
    pub area_ellipsoid_component_params: Vec<Option<AreaEllipsoidComponentParams>>,
    #[serde(default)]
    pub gravity_shape_component_params: Vec<Option<GravityShapeComponentParams>>,
    #[serde(default)]
    pub navigation_shape_component_params: Vec<Option<NavigationShapeComponentParams>>,
    #[serde(default)]
    pub navigation_cost_shape_component_params: Vec<Option<NavigationCostShapeComponentParams>>,
    #[serde(default)]
    pub slanding_communication_params: Vec<Option<SLandingCommunicationParams>>,
    #[serde(default)]
    pub landing_area_group_params: Vec<Option<LandingAreaGroupParams>>,
    #[serde(default)]
    pub hangar_area_component_params: Vec<Option<HangarAreaComponentParams>>,
    #[serde(default)]
    pub restricted_area_redout_params: Vec<Option<RestrictedAreaRedoutParams>>,
    #[serde(default)]
    pub restricted_area_autopilot_params: Vec<Option<RestrictedAreaAutopilotParams>>,
    #[serde(default)]
    pub restricted_area_kill_params: Vec<Option<RestrictedAreaKillParams>>,
    #[serde(default)]
    pub restricted_area_hudmessage_params: Vec<Option<RestrictedAreaHUDMessageParams>>,
    #[serde(default)]
    pub restricted_area_params: Vec<Option<RestrictedAreaParams>>,
    #[serde(default)]
    pub restricted_area_sphere_params: Vec<Option<RestrictedAreaSphereParams>>,
    #[serde(default)]
    pub ship_recall_banned_area_params: Vec<Option<ShipRecallBannedAreaParams>>,
    #[serde(default)]
    pub eamessage_trigger_component_params: Vec<Option<EAMessageTriggerComponentParams>>,
    #[serde(default)]
    pub eaplayable_area_movement_type_params_def: Vec<Option<EAPlayableAreaMovementTypeParamsDef>>,
    #[serde(default)]
    pub eaplayable_area_controller_component_params_def: Vec<Option<EAPlayableAreaControllerComponentParamsDef>>,
    #[serde(default)]
    pub slanding_area_object_metadata_params: Vec<Option<SLandingAreaObjectMetadataParams>>,
    #[serde(default)]
    pub extra_legal_bounds: Vec<Option<ExtraLegalBounds>>,
    #[serde(default)]
    pub landing_area_component_params: Vec<Option<LandingAreaComponentParams>>,
    #[serde(default)]
    pub landing_area_param_override_params: Vec<Option<LandingAreaParamOverrideParams>>,
    #[serde(default)]
    pub sloading_area_params: Vec<Option<SLoadingAreaParams>>,
    #[serde(default)]
    pub trespass_area_component_params: Vec<Option<TrespassAreaComponentParams>>,
    #[serde(default)]
    pub slanding_area_entry_tracker_params: Vec<Option<SLandingAreaEntryTrackerParams>>,
    #[serde(default)]
    pub sccomms_channel_area_component_params: Vec<Option<SCCommsChannelAreaComponentParams>>,
}
