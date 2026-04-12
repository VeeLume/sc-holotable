// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `ResistanceWeightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResistanceWeightParams {
    /// DCB field: `headWeighting` (Single)
    #[serde(default)]
    pub head_weighting: f32,
    /// DCB field: `bodyWeighting` (Single)
    #[serde(default)]
    pub body_weighting: f32,
    /// DCB field: `armsWeighting` (Single)
    #[serde(default)]
    pub arms_weighting: f32,
    /// DCB field: `handsWeighting` (Single)
    #[serde(default)]
    pub hands_weighting: f32,
    /// DCB field: `legsWeighting` (Single)
    #[serde(default)]
    pub legs_weighting: f32,
    /// DCB field: `feetWeighting` (Single)
    #[serde(default)]
    pub feet_weighting: f32,
}

impl Pooled for ResistanceWeightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resistance_weight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resistance_weight_params }
}

impl<'a> Extract<'a> for ResistanceWeightParams {
    const TYPE_NAME: &'static str = "ResistanceWeightParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            head_weighting: inst.get_f32("headWeighting").unwrap_or_default(),
            body_weighting: inst.get_f32("bodyWeighting").unwrap_or_default(),
            arms_weighting: inst.get_f32("armsWeighting").unwrap_or_default(),
            hands_weighting: inst.get_f32("handsWeighting").unwrap_or_default(),
            legs_weighting: inst.get_f32("legsWeighting").unwrap_or_default(),
            feet_weighting: inst.get_f32("feetWeighting").unwrap_or_default(),
        }
    }
}

/// DCB type: `RevivalFadeInParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevivalFadeInParams {
    /// DCB field: `fadeinTime` (Single)
    #[serde(default)]
    pub fadein_time: f32,
    /// DCB field: `fadeinBlur` (Single)
    #[serde(default)]
    pub fadein_blur: f32,
}

impl Pooled for RevivalFadeInParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.revival_fade_in_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.revival_fade_in_params }
}

impl<'a> Extract<'a> for RevivalFadeInParams {
    const TYPE_NAME: &'static str = "RevivalFadeInParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fadein_time: inst.get_f32("fadeinTime").unwrap_or_default(),
            fadein_blur: inst.get_f32("fadeinBlur").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReverseTrailsSetting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseTrailsSetting {
    /// DCB field: `disabledAngle` (Single)
    #[serde(default)]
    pub disabled_angle: f32,
    /// DCB field: `disabledFadeAngle` (Single)
    #[serde(default)]
    pub disabled_fade_angle: f32,
}

impl Pooled for ReverseTrailsSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reverse_trails_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reverse_trails_setting }
}

impl<'a> Extract<'a> for ReverseTrailsSetting {
    const TYPE_NAME: &'static str = "ReverseTrailsSetting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            disabled_angle: inst.get_f32("disabledAngle").unwrap_or_default(),
            disabled_fade_angle: inst.get_f32("disabledFadeAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationPrerequisiteRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationPrerequisiteRange {
    /// DCB field: `minValue` (Single)
    #[serde(default)]
    pub min_value: f32,
    /// DCB field: `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
}

impl Pooled for ReputationPrerequisiteRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_prerequisite_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_prerequisite_range }
}

impl<'a> Extract<'a> for ReputationPrerequisiteRange {
    const TYPE_NAME: &'static str = "ReputationPrerequisiteRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationPrerequisites`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationPrerequisites {
    /// DCB field: `wantedLevelJurisdictionOverride` (Reference)
    #[serde(default)]
    pub wanted_level_jurisdiction_override: Option<CigGuid>,
    /// DCB field: `wantedLevel` (Class)
    #[serde(default)]
    pub wanted_level: Option<Handle<ReputationPrerequisiteRange>>,
}

impl Pooled for ReputationPrerequisites {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_prerequisites }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_prerequisites }
}

impl<'a> Extract<'a> for ReputationPrerequisites {
    const TYPE_NAME: &'static str = "ReputationPrerequisites";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wanted_level_jurisdiction_override: inst.get("wantedLevelJurisdictionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            wanted_level: match inst.get("wantedLevel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ReputationPrerequisiteRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReputationPrerequisiteRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RelationMarkerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationMarkerParams {
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for RelationMarkerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.relation_marker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.relation_marker_params }
}

impl<'a> Extract<'a> for RelationMarkerParams {
    const TYPE_NAME: &'static str = "RelationMarkerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RelationStandingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationStandingParams {
    /// DCB field: `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// DCB field: `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// DCB field: `markerParams` (Class)
    #[serde(default)]
    pub marker_params: Option<Handle<RelationMarkerParams>>,
}

impl Pooled for RelationStandingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.relation_standing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.relation_standing_params }
}

impl<'a> Extract<'a> for RelationStandingParams {
    const TYPE_NAME: &'static str = "RelationStandingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            marker_params: match inst.get("markerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RelationMarkerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RelationMarkerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ReputationRewardBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationRewardBaseDef {
    /// DCB field: `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// DCB field: `reward` (Reference)
    #[serde(default)]
    pub reward: Option<CigGuid>,
}

impl Pooled for ReputationRewardBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_reward_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_reward_base_def }
}

impl<'a> Extract<'a> for ReputationRewardBaseDef {
    const TYPE_NAME: &'static str = "ReputationRewardBaseDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward: inst.get("reward").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `RefiningProcess`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefiningProcess {
    /// DCB field: `refiningSpeed` (EnumChoice)
    #[serde(default)]
    pub refining_speed: String,
    /// DCB field: `refiningQuality` (EnumChoice)
    #[serde(default)]
    pub refining_quality: String,
    /// DCB field: `processName` (Locale)
    #[serde(default)]
    pub process_name: String,
}

impl Pooled for RefiningProcess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.refining_process }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.refining_process }
}

impl<'a> Extract<'a> for RefiningProcess {
    const TYPE_NAME: &'static str = "RefiningProcess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            refining_speed: inst.get_str("refiningSpeed").map(String::from).unwrap_or_default(),
            refining_quality: inst.get_str("refiningQuality").map(String::from).unwrap_or_default(),
            process_name: inst.get_str("processName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RefineryNotificationConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefineryNotificationConfiguration {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `textMultiple` (Locale)
    #[serde(default)]
    pub text_multiple: String,
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `refineryServiceError` (Locale)
    #[serde(default)]
    pub refinery_service_error: String,
    /// DCB field: `refineryDeliveryFailed` (Locale)
    #[serde(default)]
    pub refinery_delivery_failed: String,
    /// DCB field: `refineryJobCreationFailed` (Locale)
    #[serde(default)]
    pub refinery_job_creation_failed: String,
    /// DCB field: `refinerySetupError` (Locale)
    #[serde(default)]
    pub refinery_setup_error: String,
}

impl Pooled for RefineryNotificationConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.refinery_notification_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.refinery_notification_configuration }
}

impl<'a> Extract<'a> for RefineryNotificationConfiguration {
    const TYPE_NAME: &'static str = "RefineryNotificationConfiguration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            text_multiple: inst.get_str("textMultiple").map(String::from).unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            refinery_service_error: inst.get_str("refineryServiceError").map(String::from).unwrap_or_default(),
            refinery_delivery_failed: inst.get_str("refineryDeliveryFailed").map(String::from).unwrap_or_default(),
            refinery_job_creation_failed: inst.get_str("refineryJobCreationFailed").map(String::from).unwrap_or_default(),
            refinery_setup_error: inst.get_str("refinerySetupError").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RentalNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalNotificationParams {
    /// DCB field: `timeToRentalExpireMessage` (Locale)
    #[serde(default)]
    pub time_to_rental_expire_message: String,
    /// DCB field: `rentedItemNameToken` (String)
    #[serde(default)]
    pub rented_item_name_token: String,
    /// DCB field: `rentalDurationToken` (String)
    #[serde(default)]
    pub rental_duration_token: String,
}

impl Pooled for RentalNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.rental_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.rental_notification_params }
}

impl<'a> Extract<'a> for RentalNotificationParams {
    const TYPE_NAME: &'static str = "RentalNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            time_to_rental_expire_message: inst.get_str("timeToRentalExpireMessage").map(String::from).unwrap_or_default(),
            rented_item_name_token: inst.get_str("rentedItemNameToken").map(String::from).unwrap_or_default(),
            rental_duration_token: inst.get_str("rentalDurationToken").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationValueSetting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationValueSetting {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `value` (Single)
    #[serde(default)]
    pub value: f32,
}

impl Pooled for ReputationValueSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_value_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_value_setting }
}

impl<'a> Extract<'a> for ReputationValueSetting {
    const TYPE_NAME: &'static str = "ReputationValueSetting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationComparisonRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationComparisonRange {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
}

impl Pooled for ReputationComparisonRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_comparison_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_comparison_range }
}

impl<'a> Extract<'a> for ReputationComparisonRange {
    const TYPE_NAME: &'static str = "ReputationComparisonRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationValueSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationValueSettings {
    /// DCB field: `rangeMin` (Single)
    #[serde(default)]
    pub range_min: f32,
    /// DCB field: `rangeMax` (Single)
    #[serde(default)]
    pub range_max: f32,
    /// DCB field: `modificationValues` (Reference (array))
    #[serde(default)]
    pub modification_values: Vec<CigGuid>,
    /// DCB field: `comparisonValues` (Class (array))
    #[serde(default)]
    pub comparison_values: Vec<Handle<ReputationComparisonRange>>,
}

impl Pooled for ReputationValueSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.reputation_value_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.reputation_value_settings }
}

impl<'a> Extract<'a> for ReputationValueSettings {
    const TYPE_NAME: &'static str = "ReputationValueSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            range_min: inst.get_f32("rangeMin").unwrap_or_default(),
            range_max: inst.get_f32("rangeMax").unwrap_or_default(),
            modification_values: inst.get_array("modificationValues")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            comparison_values: inst.get_array("comparisonValues")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ReputationComparisonRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ReputationComparisonRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ResourceTypeProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeProperties {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for ResourceTypeProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resource_type_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resource_type_properties }
}

impl<'a> Extract<'a> for ResourceTypeProperties {
    const TYPE_NAME: &'static str = "ResourceTypeProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ResourceTypeDensityType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDensityType {
}

impl Pooled for ResourceTypeDensityType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resource_type_density_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resource_type_density_type }
}

impl<'a> Extract<'a> for ResourceTypeDensityType {
    const TYPE_NAME: &'static str = "ResourceTypeDensityType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ResourceType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceType {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `densityType` (StrongPointer)
    #[serde(default)]
    pub density_type: Option<Handle<ResourceTypeDensityType>>,
    /// DCB field: `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
    /// DCB field: `defaultThumbnailPathSVG` (String)
    #[serde(default)]
    pub default_thumbnail_path_svg: String,
    /// DCB field: `rttThumbnailEntityClass` (Reference)
    #[serde(default)]
    pub rtt_thumbnail_entity_class: Option<CigGuid>,
    /// DCB field: `properties` (StrongPointer (array))
    #[serde(default)]
    pub properties: Vec<Handle<ResourceTypeProperties>>,
    /// DCB field: `refinedVersion` (Reference)
    #[serde(default)]
    pub refined_version: Option<CigGuid>,
    /// DCB field: `validateDefaultCargoBox` (Boolean)
    #[serde(default)]
    pub validate_default_cargo_box: bool,
    /// DCB field: `defaultCargoContainers` (StrongPointer)
    #[serde(default)]
    pub default_cargo_containers: Option<Handle<SResourceTypeDefaultCargoContainers>>,
}

impl Pooled for ResourceType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resource_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resource_type }
}

impl<'a> Extract<'a> for ResourceType {
    const TYPE_NAME: &'static str = "ResourceType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            density_type: match inst.get("densityType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ResourceTypeDensityType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ResourceTypeDensityType>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
            default_thumbnail_path_svg: inst.get_str("defaultThumbnailPathSVG").map(String::from).unwrap_or_default(),
            rtt_thumbnail_entity_class: inst.get("rttThumbnailEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ResourceTypeProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ResourceTypeProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            refined_version: inst.get("refinedVersion").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            validate_default_cargo_box: inst.get_bool("validateDefaultCargoBox").unwrap_or_default(),
            default_cargo_containers: match inst.get("defaultCargoContainers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ResourceTypeGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeGroup {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
    /// DCB field: `defaultThumbnailPathSVG` (String)
    #[serde(default)]
    pub default_thumbnail_path_svg: String,
    /// DCB field: `groups` (Reference (array))
    #[serde(default)]
    pub groups: Vec<CigGuid>,
    /// DCB field: `resources` (Reference (array))
    #[serde(default)]
    pub resources: Vec<CigGuid>,
}

impl Pooled for ResourceTypeGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resource_type_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resource_type_group }
}

impl<'a> Extract<'a> for ResourceTypeGroup {
    const TYPE_NAME: &'static str = "ResourceTypeGroup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
            default_thumbnail_path_svg: inst.get_str("defaultThumbnailPathSVG").map(String::from).unwrap_or_default(),
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            resources: inst.get_array("resources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ResourceTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDatabase {
    /// DCB field: `groups` (Reference (array))
    #[serde(default)]
    pub groups: Vec<CigGuid>,
}

impl Pooled for ResourceTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_re.resource_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_re.resource_type_database }
}

impl<'a> Extract<'a> for ResourceTypeDatabase {
    const TYPE_NAME: &'static str = "ResourceTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

