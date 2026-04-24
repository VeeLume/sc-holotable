// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `vfx`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `AtmosphericFlightEffects`
pub struct AtmosphericFlightEffects {
    /// `maxAtmosphericPressure` (Single)
    pub max_atmospheric_pressure: f32,
    /// `trailFading` (Class)
    pub trail_fading: Option<Handle<TrailFadingSettings>>,
    /// `reverseTrailDisabling` (StrongPointer)
    pub reverse_trail_disabling: Option<Handle<ReverseTrailsSetting>>,
    /// `engineTrails` (Class)
    pub engine_trails: Option<Handle<GlobalEngineTrailsSetting>>,
    /// `aerodynamicTrails` (Class)
    pub aerodynamic_trails: Option<Handle<GlobalAerodynamicTrailSettings>>,
    /// `atmosphericHeating` (Class)
    pub atmospheric_heating: Option<Handle<GlobalAtmosphericHeatingSettings>>,
    /// `customEnvironmentEffects` (Class)
    pub custom_environment_effects: Option<Handle<GlobalEnvironmentEffectSettings>>,
}

impl Pooled for AtmosphericFlightEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.atmospheric_flight_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.atmospheric_flight_effects }
}

impl<'a> Extract<'a> for AtmosphericFlightEffects {
    const TYPE_NAME: &'static str = "AtmosphericFlightEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_atmospheric_pressure: inst.get_f32("maxAtmosphericPressure").unwrap_or_default(),
            trail_fading: match inst.get("trailFading") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TrailFadingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            reverse_trail_disabling: match inst.get("reverseTrailDisabling") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReverseTrailsSetting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            engine_trails: match inst.get("engineTrails") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalEngineTrailsSetting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            aerodynamic_trails: match inst.get("aerodynamicTrails") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalAerodynamicTrailSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            atmospheric_heating: match inst.get("atmosphericHeating") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalAtmosphericHeatingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            custom_environment_effects: match inst.get("customEnvironmentEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalEnvironmentEffectSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TrailFadingSettings`
pub struct TrailFadingSettings {
    /// `minimumVisibleSpeed` (Single)
    pub minimum_visible_speed: f32,
    /// `speedFadeRatio` (Single)
    pub speed_fade_ratio: f32,
    /// `lowIdleBound` (Single)
    pub low_idle_bound: f32,
    /// `idleThrustBound` (Single)
    pub idle_thrust_bound: f32,
    /// `thrustAfterburnBound` (Single)
    pub thrust_afterburn_bound: f32,
}

impl Pooled for TrailFadingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.trail_fading_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.trail_fading_settings }
}

impl<'a> Extract<'a> for TrailFadingSettings {
    const TYPE_NAME: &'static str = "TrailFadingSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_visible_speed: inst.get_f32("minimumVisibleSpeed").unwrap_or_default(),
            speed_fade_ratio: inst.get_f32("speedFadeRatio").unwrap_or_default(),
            low_idle_bound: inst.get_f32("lowIdleBound").unwrap_or_default(),
            idle_thrust_bound: inst.get_f32("idleThrustBound").unwrap_or_default(),
            thrust_afterburn_bound: inst.get_f32("thrustAfterburnBound").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReverseTrailsSetting`
pub struct ReverseTrailsSetting {
    /// `disabledAngle` (Single)
    pub disabled_angle: f32,
    /// `disabledFadeAngle` (Single)
    pub disabled_fade_angle: f32,
}

impl Pooled for ReverseTrailsSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.reverse_trails_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.reverse_trails_setting }
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

/// DCB type: `GlobalEngineTrailsSetting`
pub struct GlobalEngineTrailsSetting {
    /// `pressureThreshold` (Single)
    pub pressure_threshold: f32,
    /// `contrailPressureFadeRange` (Single)
    pub contrail_pressure_fade_range: f32,
    /// `contrailCloudDensityRange` (Class)
    pub contrail_cloud_density_range: Option<Handle<Range>>,
}

impl Pooled for GlobalEngineTrailsSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_engine_trails_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_engine_trails_setting }
}

impl<'a> Extract<'a> for GlobalEngineTrailsSetting {
    const TYPE_NAME: &'static str = "GlobalEngineTrailsSetting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pressure_threshold: inst.get_f32("pressureThreshold").unwrap_or_default(),
            contrail_pressure_fade_range: inst.get_f32("contrailPressureFadeRange").unwrap_or_default(),
            contrail_cloud_density_range: match inst.get("contrailCloudDensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAtmosphericHeatingSettings`
pub struct GlobalAtmosphericHeatingSettings {
    /// `flareStartTemperature` (Single)
    pub flare_start_temperature: f32,
    /// `gravityDirectionBias` (Single)
    pub gravity_direction_bias: f32,
    /// `relativeAltitudeRange` (Class)
    pub relative_altitude_range: Option<Handle<Range>>,
    /// `relativeAltitudePeakStrength` (Single)
    pub relative_altitude_peak_strength: f32,
    /// `minimumSpeed` (Single)
    pub minimum_speed: f32,
    /// `maximumNonVehicleSpeed` (Single)
    pub maximum_non_vehicle_speed: f32,
    /// `maximumNonVehicleAngularVelocity` (Class)
    pub maximum_non_vehicle_angular_velocity: Option<Handle<Vec3>>,
    /// `fadeAngleRange` (Class)
    pub fade_angle_range: Option<Handle<Range>>,
}

impl Pooled for GlobalAtmosphericHeatingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_atmospheric_heating_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_atmospheric_heating_settings }
}

impl<'a> Extract<'a> for GlobalAtmosphericHeatingSettings {
    const TYPE_NAME: &'static str = "GlobalAtmosphericHeatingSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flare_start_temperature: inst.get_f32("flareStartTemperature").unwrap_or_default(),
            gravity_direction_bias: inst.get_f32("gravityDirectionBias").unwrap_or_default(),
            relative_altitude_range: match inst.get("relativeAltitudeRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            relative_altitude_peak_strength: inst.get_f32("relativeAltitudePeakStrength").unwrap_or_default(),
            minimum_speed: inst.get_f32("minimumSpeed").unwrap_or_default(),
            maximum_non_vehicle_speed: inst.get_f32("maximumNonVehicleSpeed").unwrap_or_default(),
            maximum_non_vehicle_angular_velocity: match inst.get("maximumNonVehicleAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fade_angle_range: match inst.get("fadeAngleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAerodynamicTrailSettings`
pub struct GlobalAerodynamicTrailSettings {
    /// `maximumAngleOfAttack` (Single)
    pub maximum_angle_of_attack: f32,
    /// `maximumRollVelocity` (Single)
    pub maximum_roll_velocity: f32,
    /// `maximumDewPointDeviation` (Single)
    pub maximum_dew_point_deviation: f32,
    /// `speedInfluence` (Single)
    pub speed_influence: f32,
    /// `engineTrailReduction` (Single)
    pub engine_trail_reduction: f32,
    /// `heatingReduction` (Single)
    pub heating_reduction: f32,
    /// `cloudDensityRange` (Class)
    pub cloud_density_range: Option<Handle<Range>>,
}

impl Pooled for GlobalAerodynamicTrailSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_aerodynamic_trail_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_aerodynamic_trail_settings }
}

impl<'a> Extract<'a> for GlobalAerodynamicTrailSettings {
    const TYPE_NAME: &'static str = "GlobalAerodynamicTrailSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            maximum_angle_of_attack: inst.get_f32("maximumAngleOfAttack").unwrap_or_default(),
            maximum_roll_velocity: inst.get_f32("maximumRollVelocity").unwrap_or_default(),
            maximum_dew_point_deviation: inst.get_f32("maximumDewPointDeviation").unwrap_or_default(),
            speed_influence: inst.get_f32("speedInfluence").unwrap_or_default(),
            engine_trail_reduction: inst.get_f32("engineTrailReduction").unwrap_or_default(),
            heating_reduction: inst.get_f32("heatingReduction").unwrap_or_default(),
            cloud_density_range: match inst.get("cloudDensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalEnvironmentEffectSettings`
pub struct GlobalEnvironmentEffectSettings {
    /// `cullDistance` (Single)
    pub cull_distance: f32,
    /// `cullDistanceRange` (Single)
    pub cull_distance_range: f32,
}

impl Pooled for GlobalEnvironmentEffectSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_environment_effect_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_environment_effect_settings }
}

impl<'a> Extract<'a> for GlobalEnvironmentEffectSettings {
    const TYPE_NAME: &'static str = "GlobalEnvironmentEffectSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            cull_distance: inst.get_f32("cullDistance").unwrap_or_default(),
            cull_distance_range: inst.get_f32("cullDistanceRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `DamageMapDamageTypes`
pub struct DamageMapDamageTypes {
    /// `physical` (Class)
    pub physical: Option<Handle<DamageMapChannels>>,
    /// `energy` (Class)
    pub energy: Option<Handle<DamageMapChannels>>,
}

impl Pooled for DamageMapDamageTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.damage_map_damage_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.damage_map_damage_types }
}

impl<'a> Extract<'a> for DamageMapDamageTypes {
    const TYPE_NAME: &'static str = "DamageMapDamageTypes";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            physical: match inst.get("physical") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            energy: match inst.get("energy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageMapDamageForm`
pub struct DamageMapDamageForm {
    /// `damageStrengthMultipliers` (Class)
    pub damage_strength_multipliers: Option<Handle<DamageMapDamageTypes>>,
    /// `innerRadiusMultipliers` (Class)
    pub inner_radius_multipliers: Option<Handle<DamageMapChannels>>,
    /// `outerRadiusMultipliers` (Class)
    pub outer_radius_multipliers: Option<Handle<DamageMapChannels>>,
    /// `noiseStrength` (Class)
    pub noise_strength: Option<Handle<DamageMapChannels>>,
}

impl Pooled for DamageMapDamageForm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.damage_map_damage_form }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.damage_map_damage_form }
}

impl<'a> Extract<'a> for DamageMapDamageForm {
    const TYPE_NAME: &'static str = "DamageMapDamageForm";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_strength_multipliers: match inst.get("damageStrengthMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            inner_radius_multipliers: match inst.get("innerRadiusMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outer_radius_multipliers: match inst.get("outerRadiusMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            noise_strength: match inst.get("noiseStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DamageMapGlobalParams`
pub struct DamageMapGlobalParams {
    /// `impact` (Class)
    pub impact: Option<Handle<DamageMapDamageForm>>,
    /// `squib` (Class)
    pub squib: Option<Handle<DamageMapDamageForm>>,
    /// `explosions` (Class)
    pub explosions: Option<Handle<DamageMapDamageForm>>,
}

impl Pooled for DamageMapGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.damage_map_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.damage_map_global_params }
}

impl<'a> Extract<'a> for DamageMapGlobalParams {
    const TYPE_NAME: &'static str = "DamageMapGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            impact: match inst.get("impact") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            squib: match inst.get("squib") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            explosions: match inst.get("explosions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapDamageForm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DematerializeAnimation`
pub struct DematerializeAnimation {
    /// `dissolveStartTime` (Single)
    pub dissolve_start_time: f32,
    /// `dissolveDuration` (Single)
    pub dissolve_duration: f32,
    /// `headEffect` (Class)
    pub head_effect: Option<Handle<GlobalResourceParticle>>,
    /// `armEffect` (Class)
    pub arm_effect: Option<Handle<GlobalResourceParticle>>,
    /// `legEffect` (Class)
    pub leg_effect: Option<Handle<GlobalResourceParticle>>,
    /// `torsoEffect` (Class)
    pub torso_effect: Option<Handle<GlobalResourceParticle>>,
    /// `debugEffect` (Class)
    pub debug_effect: Option<Handle<GlobalResourceParticle>>,
}

impl Pooled for DematerializeAnimation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.dematerialize_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.dematerialize_animation }
}

impl<'a> Extract<'a> for DematerializeAnimation {
    const TYPE_NAME: &'static str = "DematerializeAnimation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dissolve_start_time: inst.get_f32("dissolveStartTime").unwrap_or_default(),
            dissolve_duration: inst.get_f32("dissolveDuration").unwrap_or_default(),
            head_effect: match inst.get("headEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            arm_effect: match inst.get("armEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            leg_effect: match inst.get("legEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            torso_effect: match inst.get("torsoEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            debug_effect: match inst.get("debugEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalGasCloudVDB_GameplayParams`
pub struct GlobalGasCloudVDB_GameplayParams {
    /// `opticalDensityRange` (Class)
    pub optical_density_range: Option<Handle<Range>>,
    /// `gameplayDensityCurve` (Class)
    pub gameplay_density_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for GlobalGasCloudVDB_GameplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_gas_cloud_vdb_gameplay_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_gas_cloud_vdb_gameplay_params }
}

impl<'a> Extract<'a> for GlobalGasCloudVDB_GameplayParams {
    const TYPE_NAME: &'static str = "GlobalGasCloudVDB_GameplayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            optical_density_range: match inst.get("opticalDensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            gameplay_density_curve: match inst.get("gameplayDensityCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalGasCloudVDBParams`
pub struct GlobalGasCloudVDBParams {
    /// `gameplay` (Class)
    pub gameplay: Option<Handle<GlobalGasCloudVDB_GameplayParams>>,
}

impl Pooled for GlobalGasCloudVDBParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_gas_cloud_vdbparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_gas_cloud_vdbparams }
}

impl<'a> Extract<'a> for GlobalGasCloudVDBParams {
    const TYPE_NAME: &'static str = "GlobalGasCloudVDBParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            gameplay: match inst.get("gameplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalGasCloudVDB_GameplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlanetEffectLODDistance`
pub struct PlanetEffectLODDistance {
    /// `minCameraDistance` (Single)
    pub min_camera_distance: f32,
    /// `maxCameraDistance` (Single)
    pub max_camera_distance: f32,
    /// `subPatchLength` (Single)
    pub sub_patch_length: f32,
}

impl Pooled for PlanetEffectLODDistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.planet_effect_loddistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.planet_effect_loddistance }
}

impl<'a> Extract<'a> for PlanetEffectLODDistance {
    const TYPE_NAME: &'static str = "PlanetEffectLODDistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_camera_distance: inst.get_f32("minCameraDistance").unwrap_or_default(),
            max_camera_distance: inst.get_f32("maxCameraDistance").unwrap_or_default(),
            sub_patch_length: inst.get_f32("subPatchLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalFogVolume`
pub struct GlobalFogVolume {
    /// `fogSize` (Class)
    pub fog_size: Option<Handle<Vec3>>,
    /// `noiseLifeTime` (Single)
    pub noise_life_time: f32,
    /// `softEdge` (Single)
    pub soft_edge: f32,
    /// `hideFarLodThreshold` (Single)
    pub hide_far_lod_threshold: f32,
    /// `fadeFarLodThreshold` (Single)
    pub fade_far_lod_threshold: f32,
    /// `maxDistance` (Single)
    pub max_distance: f32,
}

impl Pooled for GlobalFogVolume {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.global_fog_volume }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.global_fog_volume }
}

impl<'a> Extract<'a> for GlobalFogVolume {
    const TYPE_NAME: &'static str = "GlobalFogVolume";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fog_size: match inst.get("fogSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            noise_life_time: inst.get_f32("noiseLifeTime").unwrap_or_default(),
            soft_edge: inst.get_f32("softEdge").unwrap_or_default(),
            hide_far_lod_threshold: inst.get_f32("hideFarLodThreshold").unwrap_or_default(),
            fade_far_lod_threshold: inst.get_f32("fadeFarLodThreshold").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetEffectLOD`
pub struct PlanetEffectLOD {
    /// `LODList` (Class (array))
    pub lodlist: Vec<Handle<PlanetEffectLODDistance>>,
    /// `globalFogVolume` (Class)
    pub global_fog_volume: Option<Handle<GlobalFogVolume>>,
    /// `tintColorSampleCells` (UInt32)
    pub tint_color_sample_cells: u32,
    /// `sortByViewDistance` (Boolean)
    pub sort_by_view_distance: bool,
    /// `overrideHalfResInsertDepth` (Single)
    pub override_half_res_insert_depth: f32,
}

impl Pooled for PlanetEffectLOD {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.planet_effect_lod }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.planet_effect_lod }
}

impl<'a> Extract<'a> for PlanetEffectLOD {
    const TYPE_NAME: &'static str = "PlanetEffectLOD";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lodlist: inst.get_array("LODList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetEffectLODDistance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlanetEffectLODDistance>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_fog_volume: match inst.get("globalFogVolume") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalFogVolume>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tint_color_sample_cells: inst.get_u32("tintColorSampleCells").unwrap_or_default(),
            sort_by_view_distance: inst.get_bool("sortByViewDistance").unwrap_or_default(),
            override_half_res_insert_depth: inst.get_f32("overrideHalfResInsertDepth").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveEffectSettings`
pub struct QuantumDriveEffectSettings {
    /// `spoolingEffectFadeInDuration` (Single)
    pub spooling_effect_fade_in_duration: f32,
    /// `spoolingEffectFadeOutDuration` (Single)
    pub spooling_effect_fade_out_duration: f32,
    /// `spoolingEffectMultiplier` (Single)
    pub spooling_effect_multiplier: f32,
    /// `spoolingEffectSpeedInput` (Single)
    pub spooling_effect_speed_input: f32,
}

impl Pooled for QuantumDriveEffectSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.quantum_drive_effect_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.quantum_drive_effect_settings }
}

impl<'a> Extract<'a> for QuantumDriveEffectSettings {
    const TYPE_NAME: &'static str = "QuantumDriveEffectSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            spooling_effect_fade_in_duration: inst.get_f32("spoolingEffectFadeInDuration").unwrap_or_default(),
            spooling_effect_fade_out_duration: inst.get_f32("spoolingEffectFadeOutDuration").unwrap_or_default(),
            spooling_effect_multiplier: inst.get_f32("spoolingEffectMultiplier").unwrap_or_default(),
            spooling_effect_speed_input: inst.get_f32("spoolingEffectSpeedInput").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Library`
pub struct ScreenEffects_Library {
    /// `effectList` (Reference (array))
    pub effect_list: Vec<CigGuid>,
}

impl Pooled for ScreenEffects_Library {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_library }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_library }
}

impl<'a> Extract<'a> for ScreenEffects_Library {
    const TYPE_NAME: &'static str = "ScreenEffects_Library";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_list: inst.get_array("effectList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Effect`
pub struct ScreenEffects_Effect {
    /// `name` (Reference)
    pub name: Option<CigGuid>,
    /// `disableInThirdPerson` (Boolean)
    pub disable_in_third_person: bool,
    /// `sharedPattern` (StrongPointer)
    pub shared_pattern: Option<ScreenEffects_PatternPtr>,
    /// `parameters` (Class (array))
    pub parameters: Vec<Handle<ScreenEffects_Param>>,
}

impl Pooled for ScreenEffects_Effect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_effect }
}

impl<'a> Extract<'a> for ScreenEffects_Effect {
    const TYPE_NAME: &'static str = "ScreenEffects_Effect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            disable_in_third_person: inst.get_bool("disableInThirdPerson").unwrap_or_default(),
            shared_pattern: match inst.get("sharedPattern") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ScreenEffects_PatternPtr::from_ref(b, r)),
                _ => None,
            },
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_Param>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ScreenEffects_Param>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Pattern`
pub struct ScreenEffects_Pattern {
    /// `duration` (Single)
    pub duration: f32,
}

impl Pooled for ScreenEffects_Pattern {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_pattern }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_pattern }
}

impl<'a> Extract<'a> for ScreenEffects_Pattern {
    const TYPE_NAME: &'static str = "ScreenEffects_Pattern";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Param`
pub struct ScreenEffects_Param {
    /// `parameter` (EnumChoice)
    pub parameter: PostEffectParams,
    /// `value` (StrongPointer)
    pub value: Option<ScreenEffects_ParamValuePtr>,
    /// `strengthBehavior` (StrongPointer)
    pub strength_behavior: Option<ScreenEffects_ParamStrengthBehaviorPtr>,
}

impl Pooled for ScreenEffects_Param {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_param }
}

impl<'a> Extract<'a> for ScreenEffects_Param {
    const TYPE_NAME: &'static str = "ScreenEffects_Param";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            parameter: PostEffectParams::from_dcb_str(inst.get_str("parameter").unwrap_or("")),
            value: match inst.get("value") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ScreenEffects_ParamValuePtr::from_ref(b, r)),
                _ => None,
            },
            strength_behavior: match inst.get("strengthBehavior") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ScreenEffects_ParamStrengthBehaviorPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScreenEffects_ParamStrengthBehavior`
pub struct ScreenEffects_ParamStrengthBehavior {
    /// `strengthTag` (Reference)
    pub strength_tag: Option<CigGuid>,
}

impl Pooled for ScreenEffects_ParamStrengthBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_param_strength_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_param_strength_behavior }
}

impl<'a> Extract<'a> for ScreenEffects_ParamStrengthBehavior {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamStrengthBehavior";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ScreenEffects_Pattern_Linear`
/// Inherits from: `ScreenEffects_Pattern`
pub struct ScreenEffects_Pattern_Linear {
    /// `duration` (Single)
    pub duration: f32,
    /// `mirrored` (Boolean)
    pub mirrored: bool,
}

impl Pooled for ScreenEffects_Pattern_Linear {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_pattern_linear }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_pattern_linear }
}

impl<'a> Extract<'a> for ScreenEffects_Pattern_Linear {
    const TYPE_NAME: &'static str = "ScreenEffects_Pattern_Linear";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
            mirrored: inst.get_bool("mirrored").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_ParamValue_Float`
/// Inherits from: `ScreenEffects_ParamValue`
pub struct ScreenEffects_ParamValue_Float {
    /// `value` (Single)
    pub value: f32,
    /// `limitStacking` (Boolean)
    pub limit_stacking: bool,
}

impl Pooled for ScreenEffects_ParamValue_Float {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_param_value_float }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_param_value_float }
}

impl<'a> Extract<'a> for ScreenEffects_ParamValue_Float {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamValue_Float";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_f32("value").unwrap_or_default(),
            limit_stacking: inst.get_bool("limitStacking").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_ParamStrengthBehavior_RangeEnable`
/// Inherits from: `ScreenEffects_ParamStrengthBehavior`
pub struct ScreenEffects_ParamStrengthBehavior_RangeEnable {
    /// `strengthTag` (Reference)
    pub strength_tag: Option<CigGuid>,
    /// `rangeStart` (Single)
    pub range_start: f32,
    /// `rangeEnd` (Single)
    pub range_end: f32,
}

impl Pooled for ScreenEffects_ParamStrengthBehavior_RangeEnable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_param_strength_behavior_range_enable }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_param_strength_behavior_range_enable }
}

impl<'a> Extract<'a> for ScreenEffects_ParamStrengthBehavior_RangeEnable {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamStrengthBehavior_RangeEnable";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            range_start: inst.get_f32("rangeStart").unwrap_or_default(),
            range_end: inst.get_f32("rangeEnd").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_ParamStrengthBehavior_RangeFade`
/// Inherits from: `ScreenEffects_ParamStrengthBehavior`
pub struct ScreenEffects_ParamStrengthBehavior_RangeFade {
    /// `strengthTag` (Reference)
    pub strength_tag: Option<CigGuid>,
    /// `rangeStart` (Single)
    pub range_start: f32,
    /// `rangeEnd` (Single)
    pub range_end: f32,
    /// `useSharedPattern` (Boolean)
    pub use_shared_pattern: bool,
}

impl Pooled for ScreenEffects_ParamStrengthBehavior_RangeFade {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_param_strength_behavior_range_fade }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_param_strength_behavior_range_fade }
}

impl<'a> Extract<'a> for ScreenEffects_ParamStrengthBehavior_RangeFade {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamStrengthBehavior_RangeFade";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            range_start: inst.get_f32("rangeStart").unwrap_or_default(),
            range_end: inst.get_f32("rangeEnd").unwrap_or_default(),
            use_shared_pattern: inst.get_bool("useSharedPattern").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Debug`
pub struct ScreenEffects_Debug {
    /// `effectList` (Class (array))
    pub effect_list: Vec<Handle<ScreenEffects_DebugEffect>>,
}

impl Pooled for ScreenEffects_Debug {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_debug }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_debug }
}

impl<'a> Extract<'a> for ScreenEffects_Debug {
    const TYPE_NAME: &'static str = "ScreenEffects_Debug";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_list: inst.get_array("effectList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_DebugEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ScreenEffects_DebugEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_DebugEffect`
pub struct ScreenEffects_DebugEffect {
    /// `name` (Reference)
    pub name: Option<CigGuid>,
    /// `enable` (Boolean)
    pub enable: bool,
    /// `parameters` (Class (array))
    pub parameters: Vec<Handle<ScreenEffects_DebugParam>>,
}

impl Pooled for ScreenEffects_DebugEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_debug_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_debug_effect }
}

impl<'a> Extract<'a> for ScreenEffects_DebugEffect {
    const TYPE_NAME: &'static str = "ScreenEffects_DebugEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            enable: inst.get_bool("enable").unwrap_or_default(),
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_DebugParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ScreenEffects_DebugParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_DebugParam`
pub struct ScreenEffects_DebugParam {
    /// `strengthTag` (Reference)
    pub strength_tag: Option<CigGuid>,
    /// `strength` (Single)
    pub strength: f32,
}

impl Pooled for ScreenEffects_DebugParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.screen_effects_debug_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.screen_effects_debug_param }
}

impl<'a> Extract<'a> for ScreenEffects_DebugParam {
    const TYPE_NAME: &'static str = "ScreenEffects_DebugParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            strength: inst.get_f32("strength").unwrap_or_default(),
        }
    }
}

/// DCB type: `VideoCommsShaderParams`
pub struct VideoCommsShaderParams {
    /// `fadeInSplineHighTech` (Class)
    pub fade_in_spline_high_tech: Option<Handle<BezierCurve>>,
    /// `fadeOutSplineHighTech` (Class)
    pub fade_out_spline_high_tech: Option<Handle<BezierCurve>>,
    /// `switchCommsSplineHighTech` (Class)
    pub switch_comms_spline_high_tech: Option<Handle<BezierCurve>>,
    /// `fadeInSplineLowTech` (Class)
    pub fade_in_spline_low_tech: Option<Handle<BezierCurve>>,
    /// `fadeOutSplineLowTech` (Class)
    pub fade_out_spline_low_tech: Option<Handle<BezierCurve>>,
    /// `switchCommsSplineLowTech` (Class)
    pub switch_comms_spline_low_tech: Option<Handle<BezierCurve>>,
    /// `lowTechMaterial` (String)
    pub low_tech_material: String,
    /// `highTechMaterial` (String)
    pub high_tech_material: String,
}

impl Pooled for VideoCommsShaderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.video_comms_shader_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.video_comms_shader_params }
}

impl<'a> Extract<'a> for VideoCommsShaderParams {
    const TYPE_NAME: &'static str = "VideoCommsShaderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fade_in_spline_high_tech: match inst.get("fadeInSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fade_out_spline_high_tech: match inst.get("fadeOutSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            switch_comms_spline_high_tech: match inst.get("switchCommsSplineHighTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fade_in_spline_low_tech: match inst.get("fadeInSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fade_out_spline_low_tech: match inst.get("fadeOutSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            switch_comms_spline_low_tech: match inst.get("switchCommsSplineLowTech") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            low_tech_material: inst.get_str("lowTechMaterial").map(String::from).unwrap_or_default(),
            high_tech_material: inst.get_str("highTechMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `WaterInteractionEffectParams`
pub struct WaterInteractionEffectParams {
    /// `effect` (Class)
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// `maxDuration` (Single)
    pub max_duration: f32,
    /// `velocityRange` (Class)
    pub velocity_range: Option<Handle<Range>>,
}

impl Pooled for WaterInteractionEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.water_interaction_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.water_interaction_effect_params }
}

impl<'a> Extract<'a> for WaterInteractionEffectParams {
    const TYPE_NAME: &'static str = "WaterInteractionEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_duration: inst.get_f32("maxDuration").unwrap_or_default(),
            velocity_range: match inst.get("velocityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WaterEffectsGlobalParams`
pub struct WaterEffectsGlobalParams {
    /// `exitWaterEffect` (Class)
    pub exit_water_effect: Option<Handle<WaterInteractionEffectParams>>,
    /// `enterWaterEffect` (Class)
    pub enter_water_effect: Option<Handle<WaterInteractionEffectParams>>,
}

impl Pooled for WaterEffectsGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vfx.water_effects_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vfx.water_effects_global_params }
}

impl<'a> Extract<'a> for WaterEffectsGlobalParams {
    const TYPE_NAME: &'static str = "WaterEffectsGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exit_water_effect: match inst.get("exitWaterEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WaterInteractionEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enter_water_effect: match inst.get("enterWaterEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WaterInteractionEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

