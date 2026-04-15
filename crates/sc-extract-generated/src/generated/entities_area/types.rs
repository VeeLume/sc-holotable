// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-area`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DesignerNavigationVolumeParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignerNavigationVolumeParams {
}

impl Pooled for DesignerNavigationVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.designer_navigation_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.designer_navigation_volume_params }
}

impl<'a> Extract<'a> for DesignerNavigationVolumeParams {
    const TYPE_NAME: &'static str = "DesignerNavigationVolumeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlanetInclusionVolumeParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetInclusionVolumeParams {
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
}

impl Pooled for PlanetInclusionVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.planet_inclusion_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.planet_inclusion_volume_params }
}

impl<'a> Extract<'a> for PlanetInclusionVolumeParams {
    const TYPE_NAME: &'static str = "PlanetInclusionVolumeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AreaTriggerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaTriggerParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `triggerOnce` (Boolean)
    #[serde(default)]
    pub trigger_once: bool,
    /// `onlyPlayers` (Boolean)
    #[serde(default)]
    pub only_players: bool,
    /// `onlyLocalPlayer` (Boolean)
    #[serde(default)]
    pub only_local_player: bool,
    /// `onlyAI` (Boolean)
    #[serde(default)]
    pub only_ai: bool,
    /// `inVehicleOnly` (Boolean)
    #[serde(default)]
    pub in_vehicle_only: bool,
    /// `removeEntityOnTrigger` (Boolean)
    #[serde(default)]
    pub remove_entity_on_trigger: bool,
    /// `perPlayer` (Boolean)
    #[serde(default)]
    pub per_player: bool,
    /// `enterDelay` (Single)
    #[serde(default)]
    pub enter_delay: f32,
    /// `exitDelay` (Single)
    #[serde(default)]
    pub exit_delay: f32,
}

impl Pooled for AreaTriggerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.area_trigger_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.area_trigger_params }
}

impl<'a> Extract<'a> for AreaTriggerParams {
    const TYPE_NAME: &'static str = "AreaTriggerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            trigger_once: inst.get_bool("triggerOnce").unwrap_or_default(),
            only_players: inst.get_bool("onlyPlayers").unwrap_or_default(),
            only_local_player: inst.get_bool("onlyLocalPlayer").unwrap_or_default(),
            only_ai: inst.get_bool("onlyAI").unwrap_or_default(),
            in_vehicle_only: inst.get_bool("inVehicleOnly").unwrap_or_default(),
            remove_entity_on_trigger: inst.get_bool("removeEntityOnTrigger").unwrap_or_default(),
            per_player: inst.get_bool("perPlayer").unwrap_or_default(),
            enter_delay: inst.get_f32("enterDelay").unwrap_or_default(),
            exit_delay: inst.get_f32("exitDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `GravityBoxParams`
/// Inherits from: `GravityBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityBoxParams {
    /// `active` (Boolean)
    #[serde(default)]
    pub active: bool,
    /// `uniform` (Boolean)
    #[serde(default)]
    pub uniform: bool,
    /// `fallOffInner` (Single)
    #[serde(default)]
    pub fall_off_inner: f32,
    /// `gravityMagnitude` (Single)
    #[serde(default)]
    pub gravity_magnitude: f32,
    /// `gravityDirection` (Class)
    #[serde(default)]
    pub gravity_direction: Option<Handle<Vec3>>,
    /// `filled` (Boolean)
    #[serde(default)]
    pub filled: bool,
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
}

impl Pooled for GravityBoxParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.gravity_box_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.gravity_box_params }
}

impl<'a> Extract<'a> for GravityBoxParams {
    const TYPE_NAME: &'static str = "GravityBoxParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            uniform: inst.get_bool("uniform").unwrap_or_default(),
            fall_off_inner: inst.get_f32("fallOffInner").unwrap_or_default(),
            gravity_magnitude: inst.get_f32("gravityMagnitude").unwrap_or_default(),
            gravity_direction: match inst.get("gravityDirection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            filled: inst.get_bool("filled").unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HarvestableOverrideAreaParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableOverrideAreaParams {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: HarvestableOverrideAreaType,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for HarvestableOverrideAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.harvestable_override_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.harvestable_override_area_params }
}

impl<'a> Extract<'a> for HarvestableOverrideAreaParams {
    const TYPE_NAME: &'static str = "HarvestableOverrideAreaParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: HarvestableOverrideAreaType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `SLockCameraViewAreaParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLockCameraViewAreaParams {
    /// `lockFirstPerson` (Boolean)
    #[serde(default)]
    pub lock_first_person: bool,
    /// `lockThirdPerson` (Boolean)
    #[serde(default)]
    pub lock_third_person: bool,
    /// `lockToDefaultView` (Boolean)
    #[serde(default)]
    pub lock_to_default_view: bool,
    /// `lockToEnterExitView` (Boolean)
    #[serde(default)]
    pub lock_to_enter_exit_view: bool,
    /// `cameraEnterToggleSecondsDelay` (Single)
    #[serde(default)]
    pub camera_enter_toggle_seconds_delay: f32,
    /// `cameraExitToggleSecondsDelay` (Single)
    #[serde(default)]
    pub camera_exit_toggle_seconds_delay: f32,
    /// `exitAreaTag` (Reference)
    #[serde(default)]
    pub exit_area_tag: Option<CigGuid>,
}

impl Pooled for SLockCameraViewAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.slock_camera_view_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.slock_camera_view_area_params }
}

impl<'a> Extract<'a> for SLockCameraViewAreaParams {
    const TYPE_NAME: &'static str = "SLockCameraViewAreaParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lock_first_person: inst.get_bool("lockFirstPerson").unwrap_or_default(),
            lock_third_person: inst.get_bool("lockThirdPerson").unwrap_or_default(),
            lock_to_default_view: inst.get_bool("lockToDefaultView").unwrap_or_default(),
            lock_to_enter_exit_view: inst.get_bool("lockToEnterExitView").unwrap_or_default(),
            camera_enter_toggle_seconds_delay: inst.get_f32("cameraEnterToggleSecondsDelay").unwrap_or_default(),
            camera_exit_toggle_seconds_delay: inst.get_f32("cameraExitToggleSecondsDelay").unwrap_or_default(),
            exit_area_tag: inst.get("exitAreaTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AreaEllipsoidComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaEllipsoidComponentParams {
}

impl Pooled for AreaEllipsoidComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.area_ellipsoid_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.area_ellipsoid_component_params }
}

impl<'a> Extract<'a> for AreaEllipsoidComponentParams {
    const TYPE_NAME: &'static str = "AreaEllipsoidComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GravityShapeComponentParams`
/// Inherits from: `GameShapeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityShapeComponentParams {
}

impl Pooled for GravityShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.gravity_shape_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.gravity_shape_component_params }
}

impl<'a> Extract<'a> for GravityShapeComponentParams {
    const TYPE_NAME: &'static str = "GravityShapeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `NavigationShapeComponentParams`
/// Inherits from: `AreaShapeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationShapeComponentParams {
}

impl Pooled for NavigationShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.navigation_shape_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.navigation_shape_component_params }
}

impl<'a> Extract<'a> for NavigationShapeComponentParams {
    const TYPE_NAME: &'static str = "NavigationShapeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `NavigationCostShapeComponentParams`
/// Inherits from: `AreaShapeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationCostShapeComponentParams {
}

impl Pooled for NavigationCostShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.navigation_cost_shape_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.navigation_cost_shape_component_params }
}

impl<'a> Extract<'a> for NavigationCostShapeComponentParams {
    const TYPE_NAME: &'static str = "NavigationCostShapeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SLandingCommunicationParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLandingCommunicationParams {
}

impl Pooled for SLandingCommunicationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.slanding_communication_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.slanding_communication_params }
}

impl<'a> Extract<'a> for SLandingCommunicationParams {
    const TYPE_NAME: &'static str = "SLandingCommunicationParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LandingAreaGroupParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingAreaGroupParams {
    /// `dimensions` (Class)
    #[serde(default)]
    pub dimensions: Option<Handle<Vec3>>,
}

impl Pooled for LandingAreaGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.landing_area_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.landing_area_group_params }
}

impl<'a> Extract<'a> for LandingAreaGroupParams {
    const TYPE_NAME: &'static str = "LandingAreaGroupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HangarAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HangarAreaComponentParams {
    /// `xDimension` (Single)
    #[serde(default)]
    pub x_dimension: f32,
    /// `yDimension` (Single)
    #[serde(default)]
    pub y_dimension: f32,
    /// `zDimension` (Single)
    #[serde(default)]
    pub z_dimension: f32,
}

impl Pooled for HangarAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.hangar_area_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.hangar_area_component_params }
}

impl<'a> Extract<'a> for HangarAreaComponentParams {
    const TYPE_NAME: &'static str = "HangarAreaComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_dimension: inst.get_f32("xDimension").unwrap_or_default(),
            y_dimension: inst.get_f32("yDimension").unwrap_or_default(),
            z_dimension: inst.get_f32("zDimension").unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaRedoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaRedoutParams {
    /// `redoutInGroundVehicle` (Boolean)
    #[serde(default)]
    pub redout_in_ground_vehicle: bool,
    /// `redoutInShip` (Boolean)
    #[serde(default)]
    pub redout_in_ship: bool,
    /// `timeToMaxRedout` (Single)
    #[serde(default)]
    pub time_to_max_redout: f32,
    /// `maxRedout` (Single)
    #[serde(default)]
    pub max_redout: f32,
}

impl Pooled for RestrictedAreaRedoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_redout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_redout_params }
}

impl<'a> Extract<'a> for RestrictedAreaRedoutParams {
    const TYPE_NAME: &'static str = "RestrictedAreaRedoutParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            redout_in_ground_vehicle: inst.get_bool("redoutInGroundVehicle").unwrap_or_default(),
            redout_in_ship: inst.get_bool("redoutInShip").unwrap_or_default(),
            time_to_max_redout: inst.get_f32("timeToMaxRedout").unwrap_or_default(),
            max_redout: inst.get_f32("maxRedout").unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaAutopilotParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaAutopilotParams {
    /// `minimumSpeed` (Single)
    #[serde(default)]
    pub minimum_speed: f32,
}

impl Pooled for RestrictedAreaAutopilotParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_autopilot_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_autopilot_params }
}

impl<'a> Extract<'a> for RestrictedAreaAutopilotParams {
    const TYPE_NAME: &'static str = "RestrictedAreaAutopilotParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_speed: inst.get_f32("minimumSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaKillParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaKillParams {
    /// `killActorsOnFoot` (Boolean)
    #[serde(default)]
    pub kill_actors_on_foot: bool,
    /// `killGroundVehicles` (Boolean)
    #[serde(default)]
    pub kill_ground_vehicles: bool,
    /// `killShips` (Boolean)
    #[serde(default)]
    pub kill_ships: bool,
    /// `despawnObjects` (Boolean)
    #[serde(default)]
    pub despawn_objects: bool,
    /// `despawnDelay` (Single)
    #[serde(default)]
    pub despawn_delay: f32,
    /// `allowTagsOnLoad` (Reference (array))
    #[serde(default)]
    pub allow_tags_on_load: Vec<CigGuid>,
}

impl Pooled for RestrictedAreaKillParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_kill_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_kill_params }
}

impl<'a> Extract<'a> for RestrictedAreaKillParams {
    const TYPE_NAME: &'static str = "RestrictedAreaKillParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            kill_actors_on_foot: inst.get_bool("killActorsOnFoot").unwrap_or_default(),
            kill_ground_vehicles: inst.get_bool("killGroundVehicles").unwrap_or_default(),
            kill_ships: inst.get_bool("killShips").unwrap_or_default(),
            despawn_objects: inst.get_bool("despawnObjects").unwrap_or_default(),
            despawn_delay: inst.get_f32("despawnDelay").unwrap_or_default(),
            allow_tags_on_load: inst.get_array("allowTagsOnLoad")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaHUDMessageParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaHUDMessageParams {
    /// `proximityWarningMinDistance` (Single)
    #[serde(default)]
    pub proximity_warning_min_distance: f32,
    /// `proximityWarningMaxTime` (Single)
    #[serde(default)]
    pub proximity_warning_max_time: f32,
    /// `proximityWarningDetectionConeAngle` (Single)
    #[serde(default)]
    pub proximity_warning_detection_cone_angle: f32,
    /// `proximityWarningMessages` (Locale)
    #[serde(default)]
    pub proximity_warning_messages: LocaleKey,
    /// `autopilotMessage` (Locale)
    #[serde(default)]
    pub autopilot_message: LocaleKey,
    /// `proximityMessageTimer` (Single)
    #[serde(default)]
    pub proximity_message_timer: f32,
    /// `nearWarningMessage` (Locale)
    #[serde(default)]
    pub near_warning_message: LocaleKey,
    /// `leaveWarningMessage` (Locale)
    #[serde(default)]
    pub leave_warning_message: LocaleKey,
}

impl Pooled for RestrictedAreaHUDMessageParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_hudmessage_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_hudmessage_params }
}

impl<'a> Extract<'a> for RestrictedAreaHUDMessageParams {
    const TYPE_NAME: &'static str = "RestrictedAreaHUDMessageParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            proximity_warning_min_distance: inst.get_f32("proximityWarningMinDistance").unwrap_or_default(),
            proximity_warning_max_time: inst.get_f32("proximityWarningMaxTime").unwrap_or_default(),
            proximity_warning_detection_cone_angle: inst.get_f32("proximityWarningDetectionConeAngle").unwrap_or_default(),
            proximity_warning_messages: inst.get_str("proximityWarningMessages").map(LocaleKey::from).unwrap_or_default(),
            autopilot_message: inst.get_str("autopilotMessage").map(LocaleKey::from).unwrap_or_default(),
            proximity_message_timer: inst.get_f32("proximityMessageTimer").unwrap_or_default(),
            near_warning_message: inst.get_str("nearWarningMessage").map(LocaleKey::from).unwrap_or_default(),
            leave_warning_message: inst.get_str("leaveWarningMessage").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RestrictedAreaParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaParams {
    /// `redoutParams` (Class)
    #[serde(default)]
    pub redout_params: Option<Handle<RestrictedAreaRedoutParams>>,
    /// `autopilotParams` (Class)
    #[serde(default)]
    pub autopilot_params: Option<Handle<RestrictedAreaAutopilotParams>>,
    /// `killParams` (Class)
    #[serde(default)]
    pub kill_params: Option<Handle<RestrictedAreaKillParams>>,
    /// `hudMessageParams` (Class)
    #[serde(default)]
    pub hud_message_params: Option<Handle<RestrictedAreaHUDMessageParams>>,
}

impl Pooled for RestrictedAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_params }
}

impl<'a> Extract<'a> for RestrictedAreaParams {
    const TYPE_NAME: &'static str = "RestrictedAreaParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            redout_params: match inst.get("redoutParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RestrictedAreaRedoutParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            autopilot_params: match inst.get("autopilotParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RestrictedAreaAutopilotParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            kill_params: match inst.get("killParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RestrictedAreaKillParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hud_message_params: match inst.get("hudMessageParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RestrictedAreaHUDMessageParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RestrictedAreaSphereParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictedAreaSphereParams {
}

impl Pooled for RestrictedAreaSphereParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.restricted_area_sphere_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.restricted_area_sphere_params }
}

impl<'a> Extract<'a> for RestrictedAreaSphereParams {
    const TYPE_NAME: &'static str = "RestrictedAreaSphereParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ShipRecallBannedAreaParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipRecallBannedAreaParams {
}

impl Pooled for ShipRecallBannedAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.ship_recall_banned_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.ship_recall_banned_area_params }
}

impl<'a> Extract<'a> for ShipRecallBannedAreaParams {
    const TYPE_NAME: &'static str = "ShipRecallBannedAreaParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EAMessageTriggerComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAMessageTriggerComponentParams {
    /// `triggerFrequency` (EnumChoice)
    #[serde(default)]
    pub trigger_frequency: EEAMessageTriggerFrequency,
    /// `phase` (Int32)
    #[serde(default)]
    pub phase: i32,
    /// `faction` (Reference)
    #[serde(default)]
    pub faction: Option<CigGuid>,
    /// `triggerMarkerType` (EnumChoice)
    #[serde(default)]
    pub trigger_marker_type: EEntityMarkerType,
    /// `triggerNotification` (Class)
    #[serde(default)]
    pub trigger_notification: Option<Handle<SEACriticalMessageDef>>,
}

impl Pooled for EAMessageTriggerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.eamessage_trigger_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.eamessage_trigger_component_params }
}

impl<'a> Extract<'a> for EAMessageTriggerComponentParams {
    const TYPE_NAME: &'static str = "EAMessageTriggerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_frequency: EEAMessageTriggerFrequency::from_dcb_str(inst.get_str("triggerFrequency").unwrap_or("")),
            phase: inst.get_i32("phase").unwrap_or_default(),
            faction: inst.get("faction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trigger_marker_type: EEntityMarkerType::from_dcb_str(inst.get_str("triggerMarkerType").unwrap_or("")),
            trigger_notification: match inst.get("triggerNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEACriticalMessageDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EAPlayableAreaMovementTypeParamsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAPlayableAreaMovementTypeParamsDef {
    /// `onExit` (EnumChoice)
    #[serde(default)]
    pub on_exit: EEAPlayableAreaOnExit,
    /// `onExitDelay` (Single)
    #[serde(default)]
    pub on_exit_delay: f32,
    /// `onExitDelayPhaseChange` (Single)
    #[serde(default)]
    pub on_exit_delay_phase_change: f32,
    /// `maxRedoutPercent` (Single)
    #[serde(default)]
    pub max_redout_percent: f32,
    /// `timeToMaxRedout` (Single)
    #[serde(default)]
    pub time_to_max_redout: f32,
    /// `timeToMaxRedoutPhaseChange` (Single)
    #[serde(default)]
    pub time_to_max_redout_phase_change: f32,
    /// `ignoreAI` (Boolean)
    #[serde(default)]
    pub ignore_ai: bool,
}

impl Pooled for EAPlayableAreaMovementTypeParamsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.eaplayable_area_movement_type_params_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.eaplayable_area_movement_type_params_def }
}

impl<'a> Extract<'a> for EAPlayableAreaMovementTypeParamsDef {
    const TYPE_NAME: &'static str = "EAPlayableAreaMovementTypeParamsDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            on_exit: EEAPlayableAreaOnExit::from_dcb_str(inst.get_str("onExit").unwrap_or("")),
            on_exit_delay: inst.get_f32("onExitDelay").unwrap_or_default(),
            on_exit_delay_phase_change: inst.get_f32("onExitDelayPhaseChange").unwrap_or_default(),
            max_redout_percent: inst.get_f32("maxRedoutPercent").unwrap_or_default(),
            time_to_max_redout: inst.get_f32("timeToMaxRedout").unwrap_or_default(),
            time_to_max_redout_phase_change: inst.get_f32("timeToMaxRedoutPhaseChange").unwrap_or_default(),
            ignore_ai: inst.get_bool("ignoreAI").unwrap_or_default(),
        }
    }
}

/// DCB type: `EAPlayableAreaControllerComponentParamsDef`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAPlayableAreaControllerComponentParamsDef {
    /// `paramsOnFoot` (StrongPointer)
    #[serde(default)]
    pub params_on_foot: Option<Handle<EAPlayableAreaMovementTypeParamsDef>>,
    /// `paramsGroundVehicle` (StrongPointer)
    #[serde(default)]
    pub params_ground_vehicle: Option<Handle<EAPlayableAreaMovementTypeParamsDef>>,
    /// `paramsSpaceship` (StrongPointer)
    #[serde(default)]
    pub params_spaceship: Option<Handle<EAPlayableAreaMovementTypeParamsDef>>,
    /// `despawnDelay` (Single)
    #[serde(default)]
    pub despawn_delay: f32,
    /// `OOBAttackingTeamMessage` (Locale)
    #[serde(default)]
    pub oobattacking_team_message: LocaleKey,
    /// `OOBDefendingTeamMessage` (Locale)
    #[serde(default)]
    pub oobdefending_team_message: LocaleKey,
    /// `OOBGeneralMessage` (Locale)
    #[serde(default)]
    pub oobgeneral_message: LocaleKey,
    /// `audiotriggerOutOfBounds` (Class)
    #[serde(default)]
    pub audiotrigger_out_of_bounds: Option<Handle<GlobalResourceAudio>>,
    /// `dangerLevelRtpc` (Class)
    #[serde(default)]
    pub danger_level_rtpc: Option<Handle<AudioRtpc>>,
    /// `outOfBoundsTimeRemainingRtpc` (Class)
    #[serde(default)]
    pub out_of_bounds_time_remaining_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for EAPlayableAreaControllerComponentParamsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.eaplayable_area_controller_component_params_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.eaplayable_area_controller_component_params_def }
}

impl<'a> Extract<'a> for EAPlayableAreaControllerComponentParamsDef {
    const TYPE_NAME: &'static str = "EAPlayableAreaControllerComponentParamsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params_on_foot: match inst.get("paramsOnFoot") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EAPlayableAreaMovementTypeParamsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            params_ground_vehicle: match inst.get("paramsGroundVehicle") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EAPlayableAreaMovementTypeParamsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            params_spaceship: match inst.get("paramsSpaceship") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EAPlayableAreaMovementTypeParamsDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            despawn_delay: inst.get_f32("despawnDelay").unwrap_or_default(),
            oobattacking_team_message: inst.get_str("OOBAttackingTeamMessage").map(LocaleKey::from).unwrap_or_default(),
            oobdefending_team_message: inst.get_str("OOBDefendingTeamMessage").map(LocaleKey::from).unwrap_or_default(),
            oobgeneral_message: inst.get_str("OOBGeneralMessage").map(LocaleKey::from).unwrap_or_default(),
            audiotrigger_out_of_bounds: match inst.get("audiotriggerOutOfBounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            danger_level_rtpc: match inst.get("dangerLevelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            out_of_bounds_time_remaining_rtpc: match inst.get("outOfBoundsTimeRemainingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SLandingAreaObjectMetadataParams`
/// Inherits from: `SObjectMetadataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLandingAreaObjectMetadataParams {
}

impl Pooled for SLandingAreaObjectMetadataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.slanding_area_object_metadata_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.slanding_area_object_metadata_params }
}

impl<'a> Extract<'a> for SLandingAreaObjectMetadataParams {
    const TYPE_NAME: &'static str = "SLandingAreaObjectMetadataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ExtraLegalBounds`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtraLegalBounds {
    /// `minX` (Single)
    #[serde(default)]
    pub min_x: f32,
    /// `maxX` (Single)
    #[serde(default)]
    pub max_x: f32,
    /// `minY` (Single)
    #[serde(default)]
    pub min_y: f32,
    /// `maxY` (Single)
    #[serde(default)]
    pub max_y: f32,
    /// `minZ` (Single)
    #[serde(default)]
    pub min_z: f32,
    /// `maxZ` (Single)
    #[serde(default)]
    pub max_z: f32,
}

impl Pooled for ExtraLegalBounds {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.extra_legal_bounds }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.extra_legal_bounds }
}

impl<'a> Extract<'a> for ExtraLegalBounds {
    const TYPE_NAME: &'static str = "ExtraLegalBounds";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_x: inst.get_f32("minX").unwrap_or_default(),
            max_x: inst.get_f32("maxX").unwrap_or_default(),
            min_y: inst.get_f32("minY").unwrap_or_default(),
            max_y: inst.get_f32("maxY").unwrap_or_default(),
            min_z: inst.get_f32("minZ").unwrap_or_default(),
            max_z: inst.get_f32("maxZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingAreaComponentParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `dimensions` (Class)
    #[serde(default)]
    pub dimensions: Option<Handle<Vec3>>,
    /// `extraLegalBounds` (Class)
    #[serde(default)]
    pub extra_legal_bounds: Option<Handle<ExtraLegalBounds>>,
    /// `HUDOffset` (Single)
    #[serde(default)]
    pub hudoffset: f32,
    /// `HUDDisplayName` (Locale)
    #[serde(default)]
    pub huddisplay_name: LocaleKey,
    /// `alignDirection` (Boolean)
    #[serde(default)]
    pub align_direction: bool,
    /// `autopilotEnabled` (Boolean)
    #[serde(default)]
    pub autopilot_enabled: bool,
    /// `approachDistance` (Single)
    #[serde(default)]
    pub approach_distance: f32,
    /// `touchdownHoverAltitude` (Single)
    #[serde(default)]
    pub touchdown_hover_altitude: f32,
    /// `dockingClassOverride` (Int32)
    #[serde(default)]
    pub docking_class_override: i32,
    /// `allowGroundVehicles` (Boolean)
    #[serde(default)]
    pub allow_ground_vehicles: bool,
    /// `allowSpaceships` (Boolean)
    #[serde(default)]
    pub allow_spaceships: bool,
    /// `autoRegisterWithATC` (Boolean)
    #[serde(default)]
    pub auto_register_with_atc: bool,
    /// `canBeUsedBy` (EnumChoice)
    #[serde(default)]
    pub can_be_used_by: ELandingAreaCanBeUsedBy,
    /// `ATCPriority` (Int32)
    #[serde(default)]
    pub atcpriority: i32,
    /// `autoGenerateUI` (Boolean)
    #[serde(default)]
    pub auto_generate_ui: bool,
    /// `baseUIScale` (Single)
    #[serde(default)]
    pub base_uiscale: f32,
    /// `skipObstructionCheck` (Boolean)
    #[serde(default)]
    pub skip_obstruction_check: bool,
    /// `alwaysLowerPlatformBeforeSpawn` (Boolean)
    #[serde(default)]
    pub always_lower_platform_before_spawn: bool,
}

impl Pooled for LandingAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.landing_area_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.landing_area_component_params }
}

impl<'a> Extract<'a> for LandingAreaComponentParams {
    const TYPE_NAME: &'static str = "LandingAreaComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            extra_legal_bounds: match inst.get("extraLegalBounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExtraLegalBounds>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hudoffset: inst.get_f32("HUDOffset").unwrap_or_default(),
            huddisplay_name: inst.get_str("HUDDisplayName").map(LocaleKey::from).unwrap_or_default(),
            align_direction: inst.get_bool("alignDirection").unwrap_or_default(),
            autopilot_enabled: inst.get_bool("autopilotEnabled").unwrap_or_default(),
            approach_distance: inst.get_f32("approachDistance").unwrap_or_default(),
            touchdown_hover_altitude: inst.get_f32("touchdownHoverAltitude").unwrap_or_default(),
            docking_class_override: inst.get_i32("dockingClassOverride").unwrap_or_default(),
            allow_ground_vehicles: inst.get_bool("allowGroundVehicles").unwrap_or_default(),
            allow_spaceships: inst.get_bool("allowSpaceships").unwrap_or_default(),
            auto_register_with_atc: inst.get_bool("autoRegisterWithATC").unwrap_or_default(),
            can_be_used_by: ELandingAreaCanBeUsedBy::from_dcb_str(inst.get_str("canBeUsedBy").unwrap_or("")),
            atcpriority: inst.get_i32("ATCPriority").unwrap_or_default(),
            auto_generate_ui: inst.get_bool("autoGenerateUI").unwrap_or_default(),
            base_uiscale: inst.get_f32("baseUIScale").unwrap_or_default(),
            skip_obstruction_check: inst.get_bool("skipObstructionCheck").unwrap_or_default(),
            always_lower_platform_before_spawn: inst.get_bool("alwaysLowerPlatformBeforeSpawn").unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingAreaParamOverrideParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingAreaParamOverrideParams {
    /// `landingAreaOverride` (String)
    #[serde(default)]
    pub landing_area_override: String,
    /// `HUDDisplayName` (Locale)
    #[serde(default)]
    pub huddisplay_name: LocaleKey,
}

impl Pooled for LandingAreaParamOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.landing_area_param_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.landing_area_param_override_params }
}

impl<'a> Extract<'a> for LandingAreaParamOverrideParams {
    const TYPE_NAME: &'static str = "LandingAreaParamOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            landing_area_override: inst.get_str("landingAreaOverride").map(String::from).unwrap_or_default(),
            huddisplay_name: inst.get_str("HUDDisplayName").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SLoadingAreaParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadingAreaParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `dimensions` (Class)
    #[serde(default)]
    pub dimensions: Option<Handle<Vec3>>,
    /// `autoRegisterWithATC` (Boolean)
    #[serde(default)]
    pub auto_register_with_atc: bool,
    /// `dockingClassOverride` (Int32)
    #[serde(default)]
    pub docking_class_override: i32,
    /// `allowGroundVehicles` (Boolean)
    #[serde(default)]
    pub allow_ground_vehicles: bool,
    /// `allowSpaceships` (Boolean)
    #[serde(default)]
    pub allow_spaceships: bool,
    /// `canBeUsedBy` (EnumChoice)
    #[serde(default)]
    pub can_be_used_by: ELandingAreaCanBeUsedBy,
    /// `HUDDisplayName` (Locale)
    #[serde(default)]
    pub huddisplay_name: LocaleKey,
    /// `ATCPriority` (Int32)
    #[serde(default)]
    pub atcpriority: i32,
    /// `autoGenerateUI` (Boolean)
    #[serde(default)]
    pub auto_generate_ui: bool,
}

impl Pooled for SLoadingAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.sloading_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.sloading_area_params }
}

impl<'a> Extract<'a> for SLoadingAreaParams {
    const TYPE_NAME: &'static str = "SLoadingAreaParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            auto_register_with_atc: inst.get_bool("autoRegisterWithATC").unwrap_or_default(),
            docking_class_override: inst.get_i32("dockingClassOverride").unwrap_or_default(),
            allow_ground_vehicles: inst.get_bool("allowGroundVehicles").unwrap_or_default(),
            allow_spaceships: inst.get_bool("allowSpaceships").unwrap_or_default(),
            can_be_used_by: ELandingAreaCanBeUsedBy::from_dcb_str(inst.get_str("canBeUsedBy").unwrap_or("")),
            huddisplay_name: inst.get_str("HUDDisplayName").map(LocaleKey::from).unwrap_or_default(),
            atcpriority: inst.get_i32("ATCPriority").unwrap_or_default(),
            auto_generate_ui: inst.get_bool("autoGenerateUI").unwrap_or_default(),
        }
    }
}

/// DCB type: `TrespassAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrespassAreaComponentParams {
    /// `securityClearanceConditions` (Class)
    #[serde(default)]
    pub security_clearance_conditions: Option<Handle<SecurityClearanceConditions>>,
    /// `isFelony` (Boolean)
    #[serde(default)]
    pub is_felony: bool,
    /// `filterOnFootOrEVA` (Boolean)
    #[serde(default)]
    pub filter_on_foot_or_eva: bool,
    /// `filterInShip` (Boolean)
    #[serde(default)]
    pub filter_in_ship: bool,
    /// `filterInDockedShip` (Boolean)
    #[serde(default)]
    pub filter_in_docked_ship: bool,
    /// `filterInGroundVehicle` (Boolean)
    #[serde(default)]
    pub filter_in_ground_vehicle: bool,
}

impl Pooled for TrespassAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.trespass_area_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.trespass_area_component_params }
}

impl<'a> Extract<'a> for TrespassAreaComponentParams {
    const TYPE_NAME: &'static str = "TrespassAreaComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            security_clearance_conditions: match inst.get("securityClearanceConditions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityClearanceConditions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            is_felony: inst.get_bool("isFelony").unwrap_or_default(),
            filter_on_foot_or_eva: inst.get_bool("filterOnFootOrEVA").unwrap_or_default(),
            filter_in_ship: inst.get_bool("filterInShip").unwrap_or_default(),
            filter_in_docked_ship: inst.get_bool("filterInDockedShip").unwrap_or_default(),
            filter_in_ground_vehicle: inst.get_bool("filterInGroundVehicle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SLandingAreaEntryTrackerParams`
/// Inherits from: `SObjectDataBankEntryTrackerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLandingAreaEntryTrackerParams {
}

impl Pooled for SLandingAreaEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.slanding_area_entry_tracker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.slanding_area_entry_tracker_params }
}

impl<'a> Extract<'a> for SLandingAreaEntryTrackerParams {
    const TYPE_NAME: &'static str = "SLandingAreaEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCCommsChannelAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCCommsChannelAreaComponentParams {
    /// `commsChannels` (Reference (array))
    #[serde(default)]
    pub comms_channels: Vec<CigGuid>,
    /// `innerRadius` (Single)
    #[serde(default)]
    pub inner_radius: f32,
    /// `outerRadius` (Single)
    #[serde(default)]
    pub outer_radius: f32,
}

impl Pooled for SCCommsChannelAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_area.sccomms_channel_area_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_area.sccomms_channel_area_component_params }
}

impl<'a> Extract<'a> for SCCommsChannelAreaComponentParams {
    const TYPE_NAME: &'static str = "SCCommsChannelAreaComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            comms_channels: inst.get_array("commsChannels")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            inner_radius: inst.get_f32("innerRadius").unwrap_or_default(),
            outer_radius: inst.get_f32("outerRadius").unwrap_or_default(),
        }
    }
}

