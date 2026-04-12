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

/// DCB type: `SMannequinActionDefRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMannequinActionDefRecord {
    /// DCB field: `actionDef` (StrongPointer)
    #[serde(default)]
    pub action_def: Option<Handle<IMannequinActionDef>>,
}

impl Pooled for SMannequinActionDefRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smannequin_action_def_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smannequin_action_def_record }
}

impl<'a> Extract<'a> for SMannequinActionDefRecord {
    const TYPE_NAME: &'static str = "SMannequinActionDefRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_def: match inst.get("actionDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<IMannequinActionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<IMannequinActionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMisfireEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMisfireEffect {
    /// DCB field: `effectTrigger` (Reference)
    #[serde(default)]
    pub effect_trigger: Option<CigGuid>,
    /// DCB field: `effectTag` (Reference)
    #[serde(default)]
    pub effect_tag: Option<CigGuid>,
}

impl Pooled for SMisfireEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smisfire_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smisfire_effect }
}

impl<'a> Extract<'a> for SMisfireEffect {
    const TYPE_NAME: &'static str = "SMisfireEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_trigger: inst.get("effectTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            effect_tag: inst.get("effectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMiningTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMiningTargeting {
    /// DCB field: `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SMiningTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smining_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smining_targeting }
}

impl<'a> Extract<'a> for SMiningTargeting {
    const TYPE_NAME: &'static str = "SMiningTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMasterModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMasterModeLabels {
    /// DCB field: `fullNames` (Locale)
    #[serde(default)]
    pub full_names: String,
    /// DCB field: `shortNames` (Locale)
    #[serde(default)]
    pub short_names: String,
}

impl Pooled for SMasterModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smaster_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smaster_mode_labels }
}

impl<'a> Extract<'a> for SMasterModeLabels {
    const TYPE_NAME: &'static str = "SMasterModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(String::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMaterialNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMaterialNodeParams {
    /// DCB field: `Tags` (String)
    #[serde(default)]
    pub tags: String,
    /// DCB field: `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `Palette` (Class)
    #[serde(default)]
    pub palette: Option<Handle<TintPaletteRef>>,
    /// DCB field: `materialVariants` (Class (array))
    #[serde(default)]
    pub material_variants: Vec<Handle<SMaterialNodeParams>>,
}

impl Pooled for SMaterialNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smaterial_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smaterial_node_params }
}

impl<'a> Extract<'a> for SMaterialNodeParams {
    const TYPE_NAME: &'static str = "SMaterialNodeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_str("Tags").map(String::from).unwrap_or_default(),
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            palette: match inst.get("Palette") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material_variants: inst.get_array("materialVariants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMaterialNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMaterialNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMegaMapSolarSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMegaMapSolarSystem {
    /// DCB field: `Record` (Reference)
    #[serde(default)]
    pub record: Option<CigGuid>,
    /// DCB field: `ObjectContainers` (String (array))
    #[serde(default)]
    pub object_containers: Vec<String>,
}

impl Pooled for SMegaMapSolarSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smega_map_solar_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smega_map_solar_system }
}

impl<'a> Extract<'a> for SMegaMapSolarSystem {
    const TYPE_NAME: &'static str = "SMegaMapSolarSystem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            record: inst.get("Record").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            object_containers: inst.get_array("ObjectContainers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMissionLocationModule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMissionLocationModule {
    /// DCB field: `isPersistent` (Boolean)
    #[serde(default)]
    pub is_persistent: bool,
    /// DCB field: `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// DCB field: `missionInputs` (StrongPointer (array))
    #[serde(default)]
    pub mission_inputs: Vec<Handle<AbstractMissionInitParam>>,
    /// DCB field: `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
}

impl Pooled for SMissionLocationModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smission_location_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smission_location_module }
}

impl<'a> Extract<'a> for SMissionLocationModule {
    const TYPE_NAME: &'static str = "SMissionLocationModule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_persistent: inst.get_bool("isPersistent").unwrap_or_default(),
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            mission_inputs: inst.get_array("missionInputs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AbstractMissionInitParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AbstractMissionInitParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMovableLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMovableLimits {
    /// DCB field: `veloctiyForwardClamp` (Single)
    #[serde(default)]
    pub veloctiy_forward_clamp: f32,
    /// DCB field: `veloctiyBackwardClamp` (Single)
    #[serde(default)]
    pub veloctiy_backward_clamp: f32,
    /// DCB field: `veloctiySideClamp` (Single)
    #[serde(default)]
    pub veloctiy_side_clamp: f32,
    /// DCB field: `maxYawSpeed` (Single)
    #[serde(default)]
    pub max_yaw_speed: f32,
    /// DCB field: `maxLinearAcceleration` (Single)
    #[serde(default)]
    pub max_linear_acceleration: f32,
    /// DCB field: `maxAngularAcceleration` (Single)
    #[serde(default)]
    pub max_angular_acceleration: f32,
    /// DCB field: `linearAccelerationEasingPower` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_power: f32,
    /// DCB field: `linearAccelerationEasingMinSpeed` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_min_speed: f32,
    /// DCB field: `linearAccelerationEasingMaxSpeed` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_max_speed: f32,
    /// DCB field: `angularAccelerationEasingPower` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_power: f32,
    /// DCB field: `angularAccelerationEasingMinSpeed` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_min_speed: f32,
    /// DCB field: `angularAccelerationEasingMaxSpeed` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_max_speed: f32,
    /// DCB field: `rotLinModifier` (Single)
    #[serde(default)]
    pub rot_lin_modifier: f32,
    /// DCB field: `lateralDamping` (Single)
    #[serde(default)]
    pub lateral_damping: f32,
    /// DCB field: `yawDamping` (Single)
    #[serde(default)]
    pub yaw_damping: f32,
    /// DCB field: `leanMaxSlopeAngle` (Single)
    #[serde(default)]
    pub lean_max_slope_angle: f32,
    /// DCB field: `leanMultiplier` (Single)
    #[serde(default)]
    pub lean_multiplier: f32,
    /// DCB field: `allowSprint` (Boolean)
    #[serde(default)]
    pub allow_sprint: bool,
    /// DCB field: `velocitySprintClamp` (Single)
    #[serde(default)]
    pub velocity_sprint_clamp: f32,
}

impl Pooled for SMovableLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smovable_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smovable_limits }
}

impl<'a> Extract<'a> for SMovableLimits {
    const TYPE_NAME: &'static str = "SMovableLimits";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            veloctiy_forward_clamp: inst.get_f32("veloctiyForwardClamp").unwrap_or_default(),
            veloctiy_backward_clamp: inst.get_f32("veloctiyBackwardClamp").unwrap_or_default(),
            veloctiy_side_clamp: inst.get_f32("veloctiySideClamp").unwrap_or_default(),
            max_yaw_speed: inst.get_f32("maxYawSpeed").unwrap_or_default(),
            max_linear_acceleration: inst.get_f32("maxLinearAcceleration").unwrap_or_default(),
            max_angular_acceleration: inst.get_f32("maxAngularAcceleration").unwrap_or_default(),
            linear_acceleration_easing_power: inst.get_f32("linearAccelerationEasingPower").unwrap_or_default(),
            linear_acceleration_easing_min_speed: inst.get_f32("linearAccelerationEasingMinSpeed").unwrap_or_default(),
            linear_acceleration_easing_max_speed: inst.get_f32("linearAccelerationEasingMaxSpeed").unwrap_or_default(),
            angular_acceleration_easing_power: inst.get_f32("angularAccelerationEasingPower").unwrap_or_default(),
            angular_acceleration_easing_min_speed: inst.get_f32("angularAccelerationEasingMinSpeed").unwrap_or_default(),
            angular_acceleration_easing_max_speed: inst.get_f32("angularAccelerationEasingMaxSpeed").unwrap_or_default(),
            rot_lin_modifier: inst.get_f32("rotLinModifier").unwrap_or_default(),
            lateral_damping: inst.get_f32("lateralDamping").unwrap_or_default(),
            yaw_damping: inst.get_f32("yawDamping").unwrap_or_default(),
            lean_max_slope_angle: inst.get_f32("leanMaxSlopeAngle").unwrap_or_default(),
            lean_multiplier: inst.get_f32("leanMultiplier").unwrap_or_default(),
            allow_sprint: inst.get_bool("allowSprint").unwrap_or_default(),
            velocity_sprint_clamp: inst.get_f32("velocitySprintClamp").unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDOperatorModeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDOperatorModeConfig {
    /// DCB field: `leftCastView` (EnumChoice)
    #[serde(default)]
    pub left_cast_view: String,
    /// DCB field: `rightCastView` (EnumChoice)
    #[serde(default)]
    pub right_cast_view: String,
    /// DCB field: `primaryMFDScreenView` (EnumChoice)
    #[serde(default)]
    pub primary_mfdscreen_view: String,
    /// DCB field: `secondaryMFDScreen1View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen1_view: String,
    /// DCB field: `secondaryMFDScreen2View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen2_view: String,
    /// DCB field: `secondaryMFDScreen3View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen3_view: String,
    /// DCB field: `secondaryMFDScreen4View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen4_view: String,
    /// DCB field: `secondaryMFDScreen5View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen5_view: String,
}

impl Pooled for SMFDOperatorModeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdoperator_mode_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdoperator_mode_config }
}

impl<'a> Extract<'a> for SMFDOperatorModeConfig {
    const TYPE_NAME: &'static str = "SMFDOperatorModeConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_cast_view: inst.get_str("leftCastView").map(String::from).unwrap_or_default(),
            right_cast_view: inst.get_str("rightCastView").map(String::from).unwrap_or_default(),
            primary_mfdscreen_view: inst.get_str("primaryMFDScreenView").map(String::from).unwrap_or_default(),
            secondary_mfdscreen1_view: inst.get_str("secondaryMFDScreen1View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen2_view: inst.get_str("secondaryMFDScreen2View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen3_view: inst.get_str("secondaryMFDScreen3View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen4_view: inst.get_str("secondaryMFDScreen4View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen5_view: inst.get_str("secondaryMFDScreen5View").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDViewException`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDViewException {
    /// DCB field: `viewToReplace` (EnumChoice)
    #[serde(default)]
    pub view_to_replace: String,
    /// DCB field: `replacementView` (EnumChoice)
    #[serde(default)]
    pub replacement_view: String,
}

impl Pooled for SMFDViewException {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdview_exception }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdview_exception }
}

impl<'a> Extract<'a> for SMFDViewException {
    const TYPE_NAME: &'static str = "SMFDViewException";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_to_replace: inst.get_str("viewToReplace").map(String::from).unwrap_or_default(),
            replacement_view: inst.get_str("replacementView").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDMasterModeViewExceptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDMasterModeViewExceptions {
    /// DCB field: `viewExceptions` (Class (array))
    #[serde(default)]
    pub view_exceptions: Vec<Handle<SMFDViewException>>,
}

impl Pooled for SMFDMasterModeViewExceptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdmaster_mode_view_exceptions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdmaster_mode_view_exceptions }
}

impl<'a> Extract<'a> for SMFDMasterModeViewExceptions {
    const TYPE_NAME: &'static str = "SMFDMasterModeViewExceptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            view_exceptions: inst.get_array("viewExceptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMFDViewException>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMFDViewException>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDModeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDModeConfig {
    /// DCB field: `defaultConfiguration` (Class)
    #[serde(default)]
    pub default_configuration: Option<Handle<SMFDOperatorModeConfig>>,
    /// DCB field: `operatorModeViewConfigurations` (Class)
    #[serde(default)]
    pub operator_mode_view_configurations: Option<Handle<SMFDOperatorModeConfig>>,
    /// DCB field: `operatorModeViewConfigurationsNoCasts` (Class)
    #[serde(default)]
    pub operator_mode_view_configurations_no_casts: Option<Handle<SMFDOperatorModeConfig>>,
    /// DCB field: `masterModeViewExceptions` (Class)
    #[serde(default)]
    pub master_mode_view_exceptions: Option<Handle<SMFDMasterModeViewExceptions>>,
}

impl Pooled for SMFDModeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdmode_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdmode_config }
}

impl<'a> Extract<'a> for SMFDModeConfig {
    const TYPE_NAME: &'static str = "SMFDModeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_configuration: match inst.get("defaultConfiguration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            operator_mode_view_configurations: match inst.get("operatorModeViewConfigurations") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            operator_mode_view_configurations_no_casts: match inst.get("operatorModeViewConfigurationsNoCasts") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            master_mode_view_exceptions: match inst.get("masterModeViewExceptions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDMasterModeViewExceptions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDMasterModeViewExceptions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SManufacturerMFDView`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SManufacturerMFDView {
    /// DCB field: `manufacturerStyle` (Reference)
    #[serde(default)]
    pub manufacturer_style: Option<CigGuid>,
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
}

impl Pooled for SManufacturerMFDView {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smanufacturer_mfdview }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smanufacturer_mfdview }
}

impl<'a> Extract<'a> for SManufacturerMFDView {
    const TYPE_NAME: &'static str = "SManufacturerMFDView";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            manufacturer_style: inst.get("manufacturerStyle").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMFDView`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDView {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `landscapeCanvas` (Reference)
    #[serde(default)]
    pub landscape_canvas: Option<CigGuid>,
    /// DCB field: `landscapeCanvasStyleOverride` (Class (array))
    #[serde(default)]
    pub landscape_canvas_style_override: Vec<Handle<SManufacturerMFDView>>,
    /// DCB field: `castsUsePortrait` (Boolean)
    #[serde(default)]
    pub casts_use_portrait: bool,
    /// DCB field: `portraitCanvas` (Reference)
    #[serde(default)]
    pub portrait_canvas: Option<CigGuid>,
    /// DCB field: `portraitCanvasStyleOverride` (Class (array))
    #[serde(default)]
    pub portrait_canvas_style_override: Vec<Handle<SManufacturerMFDView>>,
    /// DCB field: `viewType` (EnumChoice)
    #[serde(default)]
    pub view_type: String,
    /// DCB field: `urlpostfix` (String)
    #[serde(default)]
    pub urlpostfix: String,
}

impl Pooled for SMFDView {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdview }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdview }
}

impl<'a> Extract<'a> for SMFDView {
    const TYPE_NAME: &'static str = "SMFDView";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            landscape_canvas: inst.get("landscapeCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            landscape_canvas_style_override: inst.get_array("landscapeCanvasStyleOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SManufacturerMFDView>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SManufacturerMFDView>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            casts_use_portrait: inst.get_bool("castsUsePortrait").unwrap_or_default(),
            portrait_canvas: inst.get("portraitCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            portrait_canvas_style_override: inst.get_array("portraitCanvasStyleOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SManufacturerMFDView>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SManufacturerMFDView>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            view_type: inst.get_str("viewType").map(String::from).unwrap_or_default(),
            urlpostfix: inst.get_str("urlpostfix").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMasterModeMFDViewList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMasterModeMFDViewList {
    /// DCB field: `MasterModeViews` (Reference)
    #[serde(default)]
    pub master_mode_views: Option<CigGuid>,
}

impl Pooled for SMasterModeMFDViewList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smaster_mode_mfdview_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smaster_mode_mfdview_list }
}

impl<'a> Extract<'a> for SMasterModeMFDViewList {
    const TYPE_NAME: &'static str = "SMasterModeMFDViewList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            master_mode_views: inst.get("MasterModeViews").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMFDViewList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDViewList {
    /// DCB field: `views` (Class)
    #[serde(default)]
    pub views: Option<Handle<SMasterModeMFDViewList>>,
}

impl Pooled for SMFDViewList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdview_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdview_list }
}

impl<'a> Extract<'a> for SMFDViewList {
    const TYPE_NAME: &'static str = "SMFDViewList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            views: match inst.get("views") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMasterModeMFDViewList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMasterModeMFDViewList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMFDParamsDiagnostics`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDParamsDiagnostics {
    /// DCB field: `healthThresholdDamaged` (Single)
    #[serde(default)]
    pub health_threshold_damaged: f32,
    /// DCB field: `healthThresholdCritical` (Single)
    #[serde(default)]
    pub health_threshold_critical: f32,
    /// DCB field: `excludedItemTypes` (Class (array))
    #[serde(default)]
    pub excluded_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// DCB field: `includedItemTypes` (Class (array))
    #[serde(default)]
    pub included_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// DCB field: `typeIcons` (String)
    #[serde(default)]
    pub type_icons: String,
}

impl Pooled for SMFDParamsDiagnostics {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smfdparams_diagnostics }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smfdparams_diagnostics }
}

impl<'a> Extract<'a> for SMFDParamsDiagnostics {
    const TYPE_NAME: &'static str = "SMFDParamsDiagnostics";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            health_threshold_damaged: inst.get_f32("healthThresholdDamaged").unwrap_or_default(),
            health_threshold_critical: inst.get_f32("healthThresholdCritical").unwrap_or_default(),
            excluded_item_types: inst.get_array("excludedItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            included_item_types: inst.get_array("includedItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            type_icons: inst.get_str("typeIcons").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMobiGlasAppLink`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMobiGlasAppLink {
    /// DCB field: `targetApp` (Reference)
    #[serde(default)]
    pub target_app: Option<CigGuid>,
    /// DCB field: `targetDataEntry` (Reference)
    #[serde(default)]
    pub target_data_entry: Option<CigGuid>,
    /// DCB field: `displayIcon` (String)
    #[serde(default)]
    pub display_icon: String,
}

impl Pooled for SMobiGlasAppLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smobi_glas_app_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smobi_glas_app_link }
}

impl<'a> Extract<'a> for SMobiGlasAppLink {
    const TYPE_NAME: &'static str = "SMobiGlasAppLink";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            target_app: inst.get("targetApp").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            target_data_entry: inst.get("targetDataEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_icon: inst.get_str("displayIcon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMobiGlasAppParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMobiGlasAppParamsBase {
}

impl Pooled for SMobiGlasAppParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sm.smobi_glas_app_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sm.smobi_glas_app_params_base }
}

impl<'a> Extract<'a> for SMobiGlasAppParamsBase {
    const TYPE_NAME: &'static str = "SMobiGlasAppParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

