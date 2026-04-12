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

/// DCB type: `AtmosphericFlightEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericFlightEffects {
    /// DCB field: `maxAtmosphericPressure` (Single)
    #[serde(default)]
    pub max_atmospheric_pressure: f32,
    /// DCB field: `trailFading` (Class)
    #[serde(default)]
    pub trail_fading: Option<Handle<TrailFadingSettings>>,
    /// DCB field: `reverseTrailDisabling` (StrongPointer)
    #[serde(default)]
    pub reverse_trail_disabling: Option<Handle<ReverseTrailsSetting>>,
    /// DCB field: `engineTrails` (Class)
    #[serde(default)]
    pub engine_trails: Option<Handle<GlobalEngineTrailsSetting>>,
    /// DCB field: `aerodynamicTrails` (Class)
    #[serde(default)]
    pub aerodynamic_trails: Option<Handle<GlobalAerodynamicTrailSettings>>,
    /// DCB field: `atmosphericHeating` (Class)
    #[serde(default)]
    pub atmospheric_heating: Option<Handle<GlobalAtmosphericHeatingSettings>>,
    /// DCB field: `customEnvironmentEffects` (Class)
    #[serde(default)]
    pub custom_environment_effects: Option<Handle<GlobalEnvironmentEffectSettings>>,
}

impl Pooled for AtmosphericFlightEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmospheric_flight_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmospheric_flight_effects }
}

impl<'a> Extract<'a> for AtmosphericFlightEffects {
    const TYPE_NAME: &'static str = "AtmosphericFlightEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_atmospheric_pressure: inst.get_f32("maxAtmosphericPressure").unwrap_or_default(),
            trail_fading: match inst.get("trailFading") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TrailFadingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TrailFadingSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reverse_trail_disabling: match inst.get("reverseTrailDisabling") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ReverseTrailsSetting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReverseTrailsSetting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            engine_trails: match inst.get("engineTrails") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalEngineTrailsSetting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalEngineTrailsSetting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aerodynamic_trails: match inst.get("aerodynamicTrails") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalAerodynamicTrailSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalAerodynamicTrailSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmospheric_heating: match inst.get("atmosphericHeating") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalAtmosphericHeatingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalAtmosphericHeatingSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_environment_effects: match inst.get("customEnvironmentEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalEnvironmentEffectSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalEnvironmentEffectSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AttackCategoryParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackCategoryParamsBase {
    /// DCB field: `damageInfo` (Class)
    #[serde(default)]
    pub damage_info: Option<Handle<DamageInfo>>,
}

impl Pooled for AttackCategoryParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.attack_category_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.attack_category_params_base }
}

impl<'a> Extract<'a> for AttackCategoryParamsBase {
    const TYPE_NAME: &'static str = "AttackCategoryParamsBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_info: match inst.get("damageInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AttackCategoryParams`
///
/// Inherits from: `AttackCategoryParamsBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackCategoryParams {
    /// DCB field: `damageInfo` (Class)
    #[serde(default)]
    pub damage_info: Option<Handle<DamageInfo>>,
    /// DCB field: `actionCategory` (EnumChoice)
    #[serde(default)]
    pub action_category: String,
    /// DCB field: `cameraShakeParams` (Reference)
    #[serde(default)]
    pub camera_shake_params: Option<CigGuid>,
    /// DCB field: `stunRecoveryModifier` (Single)
    #[serde(default)]
    pub stun_recovery_modifier: f32,
    /// DCB field: `blockStunReductionModifier` (Single)
    #[serde(default)]
    pub block_stun_reduction_modifier: f32,
    /// DCB field: `blockStunStaminaModifier` (Single)
    #[serde(default)]
    pub block_stun_stamina_modifier: f32,
    /// DCB field: `attackImpulse` (Single)
    #[serde(default)]
    pub attack_impulse: f32,
    /// DCB field: `ignoreBodyPartImpulseScale` (Boolean)
    #[serde(default)]
    pub ignore_body_part_impulse_scale: bool,
    /// DCB field: `fullbodyAnimation` (Boolean)
    #[serde(default)]
    pub fullbody_animation: bool,
    /// DCB field: `forceKnockdown` (EnumChoice)
    #[serde(default)]
    pub force_knockdown: String,
}

impl Pooled for AttackCategoryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.attack_category_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.attack_category_params }
}

impl<'a> Extract<'a> for AttackCategoryParams {
    const TYPE_NAME: &'static str = "AttackCategoryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_info: match inst.get("damageInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            action_category: inst.get_str("actionCategory").map(String::from).unwrap_or_default(),
            camera_shake_params: inst.get("cameraShakeParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stun_recovery_modifier: inst.get_f32("stunRecoveryModifier").unwrap_or_default(),
            block_stun_reduction_modifier: inst.get_f32("blockStunReductionModifier").unwrap_or_default(),
            block_stun_stamina_modifier: inst.get_f32("blockStunStaminaModifier").unwrap_or_default(),
            attack_impulse: inst.get_f32("attackImpulse").unwrap_or_default(),
            ignore_body_part_impulse_scale: inst.get_bool("ignoreBodyPartImpulseScale").unwrap_or_default(),
            fullbody_animation: inst.get_bool("fullbodyAnimation").unwrap_or_default(),
            force_knockdown: inst.get_str("forceKnockdown").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AttackDetectionConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackDetectionConfig {
    /// DCB field: `numHitsToClassAsAnAttack` (Int32)
    #[serde(default)]
    pub num_hits_to_class_as_an_attack: i32,
    /// DCB field: `attackDetectionTimeWindow` (Single)
    #[serde(default)]
    pub attack_detection_time_window: f32,
    /// DCB field: `attackDetectionTimeout` (Single)
    #[serde(default)]
    pub attack_detection_timeout: f32,
}

impl Pooled for AttackDetectionConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.attack_detection_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.attack_detection_config }
}

impl<'a> Extract<'a> for AttackDetectionConfig {
    const TYPE_NAME: &'static str = "AttackDetectionConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            num_hits_to_class_as_an_attack: inst.get_i32("numHitsToClassAsAnAttack").unwrap_or_default(),
            attack_detection_time_window: inst.get_f32("attackDetectionTimeWindow").unwrap_or_default(),
            attack_detection_timeout: inst.get_f32("attackDetectionTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphericCompositionTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericCompositionTemplate {
    /// DCB field: `composition` (Class)
    #[serde(default)]
    pub composition: Option<Handle<SAtmosphericCompositionParams>>,
}

impl Pooled for AtmosphericCompositionTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmospheric_composition_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmospheric_composition_template }
}

impl<'a> Extract<'a> for AtmosphericCompositionTemplate {
    const TYPE_NAME: &'static str = "AtmosphericCompositionTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            composition: match inst.get("composition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAtmosphericCompositionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAtmosphericCompositionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AtmosphereStateTemplateInternal`
///
/// Inherits from: `AtmosphereState` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStateTemplateInternal {
    /// DCB field: `pressureMod` (EnumChoice)
    #[serde(default)]
    pub pressure_mod: String,
    /// DCB field: `pressure` (Single)
    #[serde(default)]
    pub pressure: f32,
    /// DCB field: `temperatureMod` (EnumChoice)
    #[serde(default)]
    pub temperature_mod: String,
    /// DCB field: `temperature` (Single)
    #[serde(default)]
    pub temperature: f32,
    /// DCB field: `humidityMod` (EnumChoice)
    #[serde(default)]
    pub humidity_mod: String,
    /// DCB field: `humidity` (Single)
    #[serde(default)]
    pub humidity: f32,
}

impl Pooled for AtmosphereStateTemplateInternal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_state_template_internal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_state_template_internal }
}

impl<'a> Extract<'a> for AtmosphereStateTemplateInternal {
    const TYPE_NAME: &'static str = "AtmosphereStateTemplateInternal";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pressure_mod: inst.get_str("pressureMod").map(String::from).unwrap_or_default(),
            pressure: inst.get_f32("pressure").unwrap_or_default(),
            temperature_mod: inst.get_str("temperatureMod").map(String::from).unwrap_or_default(),
            temperature: inst.get_f32("temperature").unwrap_or_default(),
            humidity_mod: inst.get_str("humidityMod").map(String::from).unwrap_or_default(),
            humidity: inst.get_f32("humidity").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphereStateTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStateTemplate {
    /// DCB field: `state` (Class)
    #[serde(default)]
    pub state: Option<Handle<AtmosphereStateTemplateInternal>>,
}

impl Pooled for AtmosphereStateTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_state_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_state_template }
}

impl<'a> Extract<'a> for AtmosphereStateTemplate {
    const TYPE_NAME: &'static str = "AtmosphereStateTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AtmosphereStateTemplateInternal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AtmosphereStateTemplateInternal>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AtmosphereStatePressureTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStatePressureTemplate {
    /// DCB field: `pressure` (Single)
    #[serde(default)]
    pub pressure: f32,
}

impl Pooled for AtmosphereStatePressureTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_state_pressure_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_state_pressure_template }
}

impl<'a> Extract<'a> for AtmosphereStatePressureTemplate {
    const TYPE_NAME: &'static str = "AtmosphereStatePressureTemplate";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pressure: inst.get_f32("pressure").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphereStateTemperatureTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStateTemperatureTemplate {
    /// DCB field: `temperature` (Single)
    #[serde(default)]
    pub temperature: f32,
}

impl Pooled for AtmosphereStateTemperatureTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_state_temperature_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_state_temperature_template }
}

impl<'a> Extract<'a> for AtmosphereStateTemperatureTemplate {
    const TYPE_NAME: &'static str = "AtmosphereStateTemperatureTemplate";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            temperature: inst.get_f32("temperature").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphereStateHumidityTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStateHumidityTemplate {
    /// DCB field: `humidity` (Single)
    #[serde(default)]
    pub humidity: f32,
}

impl Pooled for AtmosphereStateHumidityTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_state_humidity_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_state_humidity_template }
}

impl<'a> Extract<'a> for AtmosphereStateHumidityTemplate {
    const TYPE_NAME: &'static str = "AtmosphereStateHumidityTemplate";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            humidity: inst.get_f32("humidity").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphereBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereBehavior {
    /// DCB field: `vehicleEffects` (Class)
    #[serde(default)]
    pub vehicle_effects: Option<Handle<Behavior_AtmosphereVehicleEffectParams>>,
    /// DCB field: `turbulence` (StrongPointer)
    #[serde(default)]
    pub turbulence: Option<Handle<AtmosphereBehavior_TurbulenceParams>>,
    /// DCB field: `weather` (StrongPointer)
    #[serde(default)]
    pub weather: Option<Handle<AtmosphereBehavior_WeatherParams>>,
    /// DCB field: `enableAtmosphericHeating` (Boolean)
    #[serde(default)]
    pub enable_atmospheric_heating: bool,
    /// DCB field: `enableActorVectorFields` (Boolean)
    #[serde(default)]
    pub enable_actor_vector_fields: bool,
    /// DCB field: `atmosphereEnvironmentTag` (Reference)
    #[serde(default)]
    pub atmosphere_environment_tag: Option<CigGuid>,
    /// DCB field: `atmospherePressureRtpc` (Class)
    #[serde(default)]
    pub atmosphere_pressure_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `atmosphereTemperatureRtpc` (Class)
    #[serde(default)]
    pub atmosphere_temperature_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `atmosphereHumidityRtpc` (Class)
    #[serde(default)]
    pub atmosphere_humidity_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for AtmosphereBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_behavior }
}

impl<'a> Extract<'a> for AtmosphereBehavior {
    const TYPE_NAME: &'static str = "AtmosphereBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_effects: match inst.get("vehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_AtmosphereVehicleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_AtmosphereVehicleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence: match inst.get("turbulence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AtmosphereBehavior_TurbulenceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AtmosphereBehavior_TurbulenceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weather: match inst.get("weather") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AtmosphereBehavior_WeatherParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AtmosphereBehavior_WeatherParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_atmospheric_heating: inst.get_bool("enableAtmosphericHeating").unwrap_or_default(),
            enable_actor_vector_fields: inst.get_bool("enableActorVectorFields").unwrap_or_default(),
            atmosphere_environment_tag: inst.get("atmosphereEnvironmentTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            atmosphere_pressure_rtpc: match inst.get("atmospherePressureRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_temperature_rtpc: match inst.get("atmosphereTemperatureRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_humidity_rtpc: match inst.get("atmosphereHumidityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AtmosphereBehavior_TurbulenceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereBehavior_TurbulenceParams {
    /// DCB field: `frequencyMultiplier` (Single)
    #[serde(default)]
    pub frequency_multiplier: f32,
    /// DCB field: `amplitudeMultiplier` (Single)
    #[serde(default)]
    pub amplitude_multiplier: f32,
    /// DCB field: `vibrationMultiplier` (Single)
    #[serde(default)]
    pub vibration_multiplier: f32,
}

impl Pooled for AtmosphereBehavior_TurbulenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_behavior_turbulence_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_behavior_turbulence_params }
}

impl<'a> Extract<'a> for AtmosphereBehavior_TurbulenceParams {
    const TYPE_NAME: &'static str = "AtmosphereBehavior_TurbulenceParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frequency_multiplier: inst.get_f32("frequencyMultiplier").unwrap_or_default(),
            amplitude_multiplier: inst.get_f32("amplitudeMultiplier").unwrap_or_default(),
            vibration_multiplier: inst.get_f32("vibrationMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `AtmosphereBehavior_WeatherParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereBehavior_WeatherParams {
    /// DCB field: `planetCloudCondensationAmount` (Single)
    #[serde(default)]
    pub planet_cloud_condensation_amount: f32,
    /// DCB field: `planetCloudDensityCondensationRange` (Class)
    #[serde(default)]
    pub planet_cloud_density_condensation_range: Option<Handle<Range>>,
    /// DCB field: `enableSurfaceWaterEffects` (Boolean)
    #[serde(default)]
    pub enable_surface_water_effects: bool,
    /// DCB field: `effects` (StrongPointer (array))
    #[serde(default)]
    pub effects: Vec<Handle<WeatherEffects_Atmosphere>>,
}

impl Pooled for AtmosphereBehavior_WeatherParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_at.atmosphere_behavior_weather_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_at.atmosphere_behavior_weather_params }
}

impl<'a> Extract<'a> for AtmosphereBehavior_WeatherParams {
    const TYPE_NAME: &'static str = "AtmosphereBehavior_WeatherParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            planet_cloud_condensation_amount: inst.get_f32("planetCloudCondensationAmount").unwrap_or_default(),
            planet_cloud_density_condensation_range: match inst.get("planetCloudDensityCondensationRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_surface_water_effects: inst.get_bool("enableSurfaceWaterEffects").unwrap_or_default(),
            effects: inst.get_array("effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WeatherEffects_Atmosphere>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WeatherEffects_Atmosphere>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

