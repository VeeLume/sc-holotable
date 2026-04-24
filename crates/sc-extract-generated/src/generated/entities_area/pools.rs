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

/// Pool storage for the `entities-area` feature.
#[derive(Default)]
pub struct EntitiesAreaPools {
    pub designer_navigation_volume_params: Vec<Option<DesignerNavigationVolumeParams>>,
    pub planet_inclusion_volume_params: Vec<Option<PlanetInclusionVolumeParams>>,
    pub area_trigger_params: Vec<Option<AreaTriggerParams>>,
    pub gravity_box_params: Vec<Option<GravityBoxParams>>,
    pub harvestable_override_area_params: Vec<Option<HarvestableOverrideAreaParams>>,
    pub slock_camera_view_area_params: Vec<Option<SLockCameraViewAreaParams>>,
    pub area_ellipsoid_component_params: Vec<Option<AreaEllipsoidComponentParams>>,
    pub gravity_shape_component_params: Vec<Option<GravityShapeComponentParams>>,
    pub navigation_shape_component_params: Vec<Option<NavigationShapeComponentParams>>,
    pub navigation_cost_shape_component_params: Vec<Option<NavigationCostShapeComponentParams>>,
    pub slanding_communication_params: Vec<Option<SLandingCommunicationParams>>,
    pub landing_area_group_params: Vec<Option<LandingAreaGroupParams>>,
    pub hangar_area_component_params: Vec<Option<HangarAreaComponentParams>>,
    pub restricted_area_redout_params: Vec<Option<RestrictedAreaRedoutParams>>,
    pub restricted_area_autopilot_params: Vec<Option<RestrictedAreaAutopilotParams>>,
    pub restricted_area_kill_params: Vec<Option<RestrictedAreaKillParams>>,
    pub restricted_area_hudmessage_params: Vec<Option<RestrictedAreaHUDMessageParams>>,
    pub restricted_area_params: Vec<Option<RestrictedAreaParams>>,
    pub restricted_area_sphere_params: Vec<Option<RestrictedAreaSphereParams>>,
    pub ship_recall_banned_area_params: Vec<Option<ShipRecallBannedAreaParams>>,
    pub eamessage_trigger_component_params: Vec<Option<EAMessageTriggerComponentParams>>,
    pub eaplayable_area_movement_type_params_def: Vec<Option<EAPlayableAreaMovementTypeParamsDef>>,
    pub eaplayable_area_controller_component_params_def:
        Vec<Option<EAPlayableAreaControllerComponentParamsDef>>,
    pub slanding_area_object_metadata_params: Vec<Option<SLandingAreaObjectMetadataParams>>,
    pub extra_legal_bounds: Vec<Option<ExtraLegalBounds>>,
    pub landing_area_component_params: Vec<Option<LandingAreaComponentParams>>,
    pub landing_area_param_override_params: Vec<Option<LandingAreaParamOverrideParams>>,
    pub sloading_area_params: Vec<Option<SLoadingAreaParams>>,
    pub trespass_area_component_params: Vec<Option<TrespassAreaComponentParams>>,
    pub slanding_area_entry_tracker_params: Vec<Option<SLandingAreaEntryTrackerParams>>,
    pub sccomms_channel_area_component_params: Vec<Option<SCCommsChannelAreaComponentParams>>,
}
