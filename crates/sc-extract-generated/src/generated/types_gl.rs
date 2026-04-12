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

/// DCB type: `GlobalEngineTrailsSetting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEngineTrailsSetting {
    /// DCB field: `pressureThreshold` (Single)
    #[serde(default)]
    pub pressure_threshold: f32,
    /// DCB field: `contrailPressureFadeRange` (Single)
    #[serde(default)]
    pub contrail_pressure_fade_range: f32,
    /// DCB field: `contrailCloudDensityRange` (Class)
    #[serde(default)]
    pub contrail_cloud_density_range: Option<Handle<Range>>,
}

impl Pooled for GlobalEngineTrailsSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_engine_trails_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_engine_trails_setting }
}

impl<'a> Extract<'a> for GlobalEngineTrailsSetting {
    const TYPE_NAME: &'static str = "GlobalEngineTrailsSetting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pressure_threshold: inst.get_f32("pressureThreshold").unwrap_or_default(),
            contrail_pressure_fade_range: inst.get_f32("contrailPressureFadeRange").unwrap_or_default(),
            contrail_cloud_density_range: match inst.get("contrailCloudDensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAtmosphericHeatingSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAtmosphericHeatingSettings {
    /// DCB field: `flareStartTemperature` (Single)
    #[serde(default)]
    pub flare_start_temperature: f32,
    /// DCB field: `gravityDirectionBias` (Single)
    #[serde(default)]
    pub gravity_direction_bias: f32,
    /// DCB field: `relativeAltitudeRange` (Class)
    #[serde(default)]
    pub relative_altitude_range: Option<Handle<Range>>,
    /// DCB field: `relativeAltitudePeakStrength` (Single)
    #[serde(default)]
    pub relative_altitude_peak_strength: f32,
    /// DCB field: `minimumSpeed` (Single)
    #[serde(default)]
    pub minimum_speed: f32,
    /// DCB field: `maximumNonVehicleSpeed` (Single)
    #[serde(default)]
    pub maximum_non_vehicle_speed: f32,
    /// DCB field: `maximumNonVehicleAngularVelocity` (Class)
    #[serde(default)]
    pub maximum_non_vehicle_angular_velocity: Option<Handle<Vec3>>,
    /// DCB field: `fadeAngleRange` (Class)
    #[serde(default)]
    pub fade_angle_range: Option<Handle<Range>>,
}

impl Pooled for GlobalAtmosphericHeatingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_atmospheric_heating_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_atmospheric_heating_settings }
}

impl<'a> Extract<'a> for GlobalAtmosphericHeatingSettings {
    const TYPE_NAME: &'static str = "GlobalAtmosphericHeatingSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flare_start_temperature: inst.get_f32("flareStartTemperature").unwrap_or_default(),
            gravity_direction_bias: inst.get_f32("gravityDirectionBias").unwrap_or_default(),
            relative_altitude_range: match inst.get("relativeAltitudeRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            relative_altitude_peak_strength: inst.get_f32("relativeAltitudePeakStrength").unwrap_or_default(),
            minimum_speed: inst.get_f32("minimumSpeed").unwrap_or_default(),
            maximum_non_vehicle_speed: inst.get_f32("maximumNonVehicleSpeed").unwrap_or_default(),
            maximum_non_vehicle_angular_velocity: match inst.get("maximumNonVehicleAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_angle_range: match inst.get("fadeAngleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAerodynamicTrailSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAerodynamicTrailSettings {
    /// DCB field: `maximumAngleOfAttack` (Single)
    #[serde(default)]
    pub maximum_angle_of_attack: f32,
    /// DCB field: `maximumRollVelocity` (Single)
    #[serde(default)]
    pub maximum_roll_velocity: f32,
    /// DCB field: `maximumDewPointDeviation` (Single)
    #[serde(default)]
    pub maximum_dew_point_deviation: f32,
    /// DCB field: `speedInfluence` (Single)
    #[serde(default)]
    pub speed_influence: f32,
    /// DCB field: `engineTrailReduction` (Single)
    #[serde(default)]
    pub engine_trail_reduction: f32,
    /// DCB field: `heatingReduction` (Single)
    #[serde(default)]
    pub heating_reduction: f32,
    /// DCB field: `cloudDensityRange` (Class)
    #[serde(default)]
    pub cloud_density_range: Option<Handle<Range>>,
}

impl Pooled for GlobalAerodynamicTrailSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_aerodynamic_trail_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_aerodynamic_trail_settings }
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalEnvironmentEffectSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEnvironmentEffectSettings {
    /// DCB field: `cullDistance` (Single)
    #[serde(default)]
    pub cull_distance: f32,
    /// DCB field: `cullDistanceRange` (Single)
    #[serde(default)]
    pub cull_distance_range: f32,
}

impl Pooled for GlobalEnvironmentEffectSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_environment_effect_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_environment_effect_settings }
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

/// DCB type: `GlobalAudioSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAudioSettings {
    /// DCB field: `enablePropagationPathing` (Boolean)
    #[serde(default)]
    pub enable_propagation_pathing: bool,
    /// DCB field: `enablePropagationPathingActorOnly` (Boolean)
    #[serde(default)]
    pub enable_propagation_pathing_actor_only: bool,
    /// DCB field: `gamePausedTrigger` (Class)
    #[serde(default)]
    pub game_paused_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `gameUnpausedTrigger` (Class)
    #[serde(default)]
    pub game_unpaused_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for GlobalAudioSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_audio_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_audio_settings }
}

impl<'a> Extract<'a> for GlobalAudioSettings {
    const TYPE_NAME: &'static str = "GlobalAudioSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_propagation_pathing: inst.get_bool("enablePropagationPathing").unwrap_or_default(),
            enable_propagation_pathing_actor_only: inst.get_bool("enablePropagationPathingActorOnly").unwrap_or_default(),
            game_paused_trigger: match inst.get("gamePausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_unpaused_trigger: match inst.get("gameUnpausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalGasCloudVDB_GameplayParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalGasCloudVDB_GameplayParams {
    /// DCB field: `opticalDensityRange` (Class)
    #[serde(default)]
    pub optical_density_range: Option<Handle<Range>>,
    /// DCB field: `gameplayDensityCurve` (Class)
    #[serde(default)]
    pub gameplay_density_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for GlobalGasCloudVDB_GameplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_gas_cloud_vdb_gameplay_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_gas_cloud_vdb_gameplay_params }
}

impl<'a> Extract<'a> for GlobalGasCloudVDB_GameplayParams {
    const TYPE_NAME: &'static str = "GlobalGasCloudVDB_GameplayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            optical_density_range: match inst.get("opticalDensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gameplay_density_curve: match inst.get("gameplayDensityCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalGasCloudVDBParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalGasCloudVDBParams {
    /// DCB field: `gameplay` (Class)
    #[serde(default)]
    pub gameplay: Option<Handle<GlobalGasCloudVDB_GameplayParams>>,
}

impl Pooled for GlobalGasCloudVDBParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_gas_cloud_vdbparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_gas_cloud_vdbparams }
}

impl<'a> Extract<'a> for GlobalGasCloudVDBParams {
    const TYPE_NAME: &'static str = "GlobalGasCloudVDBParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            gameplay: match inst.get("gameplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalGasCloudVDB_GameplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalGasCloudVDB_GameplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalShopCommodityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopCommodityParams {
    /// DCB field: `MaxKioskCargoGridDisplaySize` (Int32)
    #[serde(default)]
    pub max_kiosk_cargo_grid_display_size: i32,
    /// DCB field: `autoLoadingBasePrice` (Int32)
    #[serde(default)]
    pub auto_loading_base_price: i32,
    /// DCB field: `autoLoadingBoxSizePrices` (Class)
    #[serde(default)]
    pub auto_loading_box_size_prices: Option<Handle<SAutoLoadingBoxSizePrices>>,
    /// DCB field: `noSupplyLevel` (Single)
    #[serde(default)]
    pub no_supply_level: f32,
    /// DCB field: `VeryLowSupplyLevel` (Single)
    #[serde(default)]
    pub very_low_supply_level: f32,
    /// DCB field: `LowSupplyLevel` (Single)
    #[serde(default)]
    pub low_supply_level: f32,
    /// DCB field: `MediumSupplyLevel` (Single)
    #[serde(default)]
    pub medium_supply_level: f32,
    /// DCB field: `HighSupplyLevel` (Single)
    #[serde(default)]
    pub high_supply_level: f32,
    /// DCB field: `VeryHighSupplyLevel` (Single)
    #[serde(default)]
    pub very_high_supply_level: f32,
    /// DCB field: `noDemandLevel` (Single)
    #[serde(default)]
    pub no_demand_level: f32,
    /// DCB field: `VeryLowDemandLevel` (Single)
    #[serde(default)]
    pub very_low_demand_level: f32,
    /// DCB field: `LowDemandLevel` (Single)
    #[serde(default)]
    pub low_demand_level: f32,
    /// DCB field: `MediumDemandLevel` (Single)
    #[serde(default)]
    pub medium_demand_level: f32,
    /// DCB field: `HighDemandLevel` (Single)
    #[serde(default)]
    pub high_demand_level: f32,
    /// DCB field: `VeryHighDemandLevel` (Single)
    #[serde(default)]
    pub very_high_demand_level: f32,
    /// DCB field: `transactionSupportedResourceContainerTypes` (Class (array))
    #[serde(default)]
    pub transaction_supported_resource_container_types: Vec<Handle<SItemPortDefTypes>>,
    /// DCB field: `RMC_ResourceType` (Reference)
    #[serde(default)]
    pub rmc_resource_type: Option<CigGuid>,
    /// DCB field: `RMC_SalvageCannisterEntity` (Reference)
    #[serde(default)]
    pub rmc_salvage_cannister_entity: Option<CigGuid>,
    /// DCB field: `genericCrates` (Class)
    #[serde(default)]
    pub generic_crates: Option<Handle<SResourceTypeDefaultCargoContainers>>,
    /// DCB field: `Location_Select` (Locale)
    #[serde(default)]
    pub location_select: String,
    /// DCB field: `subLocation_All` (Locale)
    #[serde(default)]
    pub sub_location_all: String,
    /// DCB field: `subLocation_CargoGrid` (Locale)
    #[serde(default)]
    pub sub_location_cargo_grid: String,
    /// DCB field: `subLocation_GeneralStorage` (Locale)
    #[serde(default)]
    pub sub_location_general_storage: String,
    /// DCB field: `subLocation_ResourceContainers` (Locale)
    #[serde(default)]
    pub sub_location_resource_containers: String,
    /// DCB field: `subLocationItems_All` (Locale)
    #[serde(default)]
    pub sub_location_items_all: String,
}

impl Pooled for GlobalShopCommodityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_shop_commodity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_shop_commodity_params }
}

impl<'a> Extract<'a> for GlobalShopCommodityParams {
    const TYPE_NAME: &'static str = "GlobalShopCommodityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_kiosk_cargo_grid_display_size: inst.get_i32("MaxKioskCargoGridDisplaySize").unwrap_or_default(),
            auto_loading_base_price: inst.get_i32("autoLoadingBasePrice").unwrap_or_default(),
            auto_loading_box_size_prices: match inst.get("autoLoadingBoxSizePrices") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAutoLoadingBoxSizePrices>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAutoLoadingBoxSizePrices>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            no_supply_level: inst.get_f32("noSupplyLevel").unwrap_or_default(),
            very_low_supply_level: inst.get_f32("VeryLowSupplyLevel").unwrap_or_default(),
            low_supply_level: inst.get_f32("LowSupplyLevel").unwrap_or_default(),
            medium_supply_level: inst.get_f32("MediumSupplyLevel").unwrap_or_default(),
            high_supply_level: inst.get_f32("HighSupplyLevel").unwrap_or_default(),
            very_high_supply_level: inst.get_f32("VeryHighSupplyLevel").unwrap_or_default(),
            no_demand_level: inst.get_f32("noDemandLevel").unwrap_or_default(),
            very_low_demand_level: inst.get_f32("VeryLowDemandLevel").unwrap_or_default(),
            low_demand_level: inst.get_f32("LowDemandLevel").unwrap_or_default(),
            medium_demand_level: inst.get_f32("MediumDemandLevel").unwrap_or_default(),
            high_demand_level: inst.get_f32("HighDemandLevel").unwrap_or_default(),
            very_high_demand_level: inst.get_f32("VeryHighDemandLevel").unwrap_or_default(),
            transaction_supported_resource_container_types: inst.get_array("transactionSupportedResourceContainerTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rmc_resource_type: inst.get("RMC_ResourceType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rmc_salvage_cannister_entity: inst.get("RMC_SalvageCannisterEntity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            generic_crates: match inst.get("genericCrates") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_select: inst.get_str("Location_Select").map(String::from).unwrap_or_default(),
            sub_location_all: inst.get_str("subLocation_All").map(String::from).unwrap_or_default(),
            sub_location_cargo_grid: inst.get_str("subLocation_CargoGrid").map(String::from).unwrap_or_default(),
            sub_location_general_storage: inst.get_str("subLocation_GeneralStorage").map(String::from).unwrap_or_default(),
            sub_location_resource_containers: inst.get_str("subLocation_ResourceContainers").map(String::from).unwrap_or_default(),
            sub_location_items_all: inst.get_str("subLocationItems_All").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopTerminalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopTerminalParams {
    /// DCB field: `displayedItemsPerPage` (Int32)
    #[serde(default)]
    pub displayed_items_per_page: i32,
    /// DCB field: `confirmationAutoCloseTime` (Single)
    #[serde(default)]
    pub confirmation_auto_close_time: f32,
    /// DCB field: `maxBuyingMultiplier` (Int32)
    #[serde(default)]
    pub max_buying_multiplier: i32,
    /// DCB field: `inventoryQueryItemsType` (EnumChoice (array))
    #[serde(default)]
    pub inventory_query_items_type: Vec<String>,
    /// DCB field: `all_items_category` (Locale)
    #[serde(default)]
    pub all_items_category: String,
    /// DCB field: `shopErrors` (Class)
    #[serde(default)]
    pub shop_errors: Option<Handle<SGlobalShopErrors>>,
}

impl Pooled for GlobalShopTerminalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_shop_terminal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_shop_terminal_params }
}

impl<'a> Extract<'a> for GlobalShopTerminalParams {
    const TYPE_NAME: &'static str = "GlobalShopTerminalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            displayed_items_per_page: inst.get_i32("displayedItemsPerPage").unwrap_or_default(),
            confirmation_auto_close_time: inst.get_f32("confirmationAutoCloseTime").unwrap_or_default(),
            max_buying_multiplier: inst.get_i32("maxBuyingMultiplier").unwrap_or_default(),
            inventory_query_items_type: inst.get_array("inventoryQueryItemsType")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            all_items_category: inst.get_str("all_items_category").map(String::from).unwrap_or_default(),
            shop_errors: match inst.get("shopErrors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGlobalShopErrors>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGlobalShopErrors>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalShopSellingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopSellingParams {
    /// DCB field: `matchPercentage` (Single)
    #[serde(default)]
    pub match_percentage: f32,
    /// DCB field: `noMatchPercentage` (Single)
    #[serde(default)]
    pub no_match_percentage: f32,
    /// DCB field: `missionItemSellPriceReductionPercentage` (Single)
    #[serde(default)]
    pub mission_item_sell_price_reduction_percentage: f32,
    /// DCB field: `maxInventoryCurve` (Single (array))
    #[serde(default)]
    pub max_inventory_curve: Vec<f32>,
    /// DCB field: `wearCurve` (Single (array))
    #[serde(default)]
    pub wear_curve: Vec<f32>,
    /// DCB field: `itemTypeModifiers` (Class (array))
    #[serde(default)]
    pub item_type_modifiers: Vec<Handle<ItemTypeModifier>>,
}

impl Pooled for GlobalShopSellingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_shop_selling_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_shop_selling_params }
}

impl<'a> Extract<'a> for GlobalShopSellingParams {
    const TYPE_NAME: &'static str = "GlobalShopSellingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_percentage: inst.get_f32("matchPercentage").unwrap_or_default(),
            no_match_percentage: inst.get_f32("noMatchPercentage").unwrap_or_default(),
            mission_item_sell_price_reduction_percentage: inst.get_f32("missionItemSellPriceReductionPercentage").unwrap_or_default(),
            max_inventory_curve: inst.get_array("maxInventoryCurve")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
            wear_curve: inst.get_array("wearCurve")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
            item_type_modifiers: inst.get_array("itemTypeModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemTypeModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopBuyingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopBuyingParams {
    /// DCB field: `tutorialItemBuyLimitPerHour` (UInt32)
    #[serde(default)]
    pub tutorial_item_buy_limit_per_hour: u32,
    /// DCB field: `licensedItemModifiers` (Class (array))
    #[serde(default)]
    pub licensed_item_modifiers: Vec<Handle<LicensedItemModifier>>,
}

impl Pooled for GlobalShopBuyingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_shop_buying_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_shop_buying_params }
}

impl<'a> Extract<'a> for GlobalShopBuyingParams {
    const TYPE_NAME: &'static str = "GlobalShopBuyingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tutorial_item_buy_limit_per_hour: inst.get_u32("tutorialItemBuyLimitPerHour").unwrap_or_default(),
            licensed_item_modifiers: inst.get_array("licensedItemModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LicensedItemModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LicensedItemModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointTuningParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointTuningParams {
    /// DCB field: `requiredTuningAmount` (Single)
    #[serde(default)]
    pub required_tuning_amount: f32,
}

impl Pooled for GlobalJumpPointTuningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_point_tuning_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_point_tuning_params }
}

impl<'a> Extract<'a> for GlobalJumpPointTuningParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointTuningParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_tuning_amount: inst.get_f32("requiredTuningAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointOpeningParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointOpeningParams {
    /// DCB field: `projectileTime` (Single)
    #[serde(default)]
    pub projectile_time: f32,
    /// DCB field: `tuningCollapseTime` (Single)
    #[serde(default)]
    pub tuning_collapse_time: f32,
    /// DCB field: `openEffectStartDelay` (Single)
    #[serde(default)]
    pub open_effect_start_delay: f32,
    /// DCB field: `revealTime` (Single)
    #[serde(default)]
    pub reveal_time: f32,
    /// DCB field: `revealAnimCurve` (Class)
    #[serde(default)]
    pub reveal_anim_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `revealFadeDelay` (Single)
    #[serde(default)]
    pub reveal_fade_delay: f32,
    /// DCB field: `openingTime` (Single)
    #[serde(default)]
    pub opening_time: f32,
    /// DCB field: `openingEndDelay` (Single)
    #[serde(default)]
    pub opening_end_delay: f32,
    /// DCB field: `apertureTimeScaleRange` (Class)
    #[serde(default)]
    pub aperture_time_scale_range: Option<Handle<Range>>,
}

impl Pooled for GlobalJumpPointOpeningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_point_opening_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_point_opening_params }
}

impl<'a> Extract<'a> for GlobalJumpPointOpeningParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointOpeningParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            projectile_time: inst.get_f32("projectileTime").unwrap_or_default(),
            tuning_collapse_time: inst.get_f32("tuningCollapseTime").unwrap_or_default(),
            open_effect_start_delay: inst.get_f32("openEffectStartDelay").unwrap_or_default(),
            reveal_time: inst.get_f32("revealTime").unwrap_or_default(),
            reveal_anim_curve: match inst.get("revealAnimCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reveal_fade_delay: inst.get_f32("revealFadeDelay").unwrap_or_default(),
            opening_time: inst.get_f32("openingTime").unwrap_or_default(),
            opening_end_delay: inst.get_f32("openingEndDelay").unwrap_or_default(),
            aperture_time_scale_range: match inst.get("apertureTimeScaleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpPointClosingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointClosingParams {
    /// DCB field: `closingTime` (Single)
    #[serde(default)]
    pub closing_time: f32,
    /// DCB field: `shrinkTime` (Single)
    #[serde(default)]
    pub shrink_time: f32,
    /// DCB field: `closingTriggerDelay` (Single)
    #[serde(default)]
    pub closing_trigger_delay: f32,
}

impl Pooled for GlobalJumpPointClosingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_point_closing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_point_closing_params }
}

impl<'a> Extract<'a> for GlobalJumpPointClosingParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointClosingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            closing_time: inst.get_f32("closingTime").unwrap_or_default(),
            shrink_time: inst.get_f32("shrinkTime").unwrap_or_default(),
            closing_trigger_delay: inst.get_f32("closingTriggerDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpPointEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointEffectParams {
    /// DCB field: `tuningParams` (Class)
    #[serde(default)]
    pub tuning_params: Option<Handle<GlobalJumpPointTuningParams>>,
    /// DCB field: `openingParams` (Class)
    #[serde(default)]
    pub opening_params: Option<Handle<GlobalJumpPointOpeningParams>>,
    /// DCB field: `closingParams` (Class)
    #[serde(default)]
    pub closing_params: Option<Handle<GlobalJumpPointClosingParams>>,
}

impl Pooled for GlobalJumpPointEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_point_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_point_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpPointEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tuning_params: match inst.get("tuningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointTuningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpPointTuningParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            opening_params: match inst.get("openingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointOpeningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpPointOpeningParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            closing_params: match inst.get("closingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointClosingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpPointClosingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpPointParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpPointParams {
    /// DCB field: `alignmentAngle` (Single)
    #[serde(default)]
    pub alignment_angle: f32,
    /// DCB field: `alignmentTolerance` (Single)
    #[serde(default)]
    pub alignment_tolerance: f32,
    /// DCB field: `alignmentRange` (Single)
    #[serde(default)]
    pub alignment_range: f32,
    /// DCB field: `startClosingTime` (Single)
    #[serde(default)]
    pub start_closing_time: f32,
    /// DCB field: `shipPullInDelayTime` (Single)
    #[serde(default)]
    pub ship_pull_in_delay_time: f32,
    /// DCB field: `shipPullInBufferTime` (Single)
    #[serde(default)]
    pub ship_pull_in_buffer_time: f32,
    /// DCB field: `debrisPushOutAcceleration` (Single)
    #[serde(default)]
    pub debris_push_out_acceleration: f32,
    /// DCB field: `debrisPushOutMaximumSpeed` (Single)
    #[serde(default)]
    pub debris_push_out_maximum_speed: f32,
    /// DCB field: `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpPointEffectParams>>,
    /// DCB field: `uiConeParams` (Class)
    #[serde(default)]
    pub ui_cone_params: Option<Handle<JumpDriveUIConeParams>>,
}

impl Pooled for GlobalJumpPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_point_params }
}

impl<'a> Extract<'a> for GlobalJumpPointParams {
    const TYPE_NAME: &'static str = "GlobalJumpPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            alignment_angle: inst.get_f32("alignmentAngle").unwrap_or_default(),
            alignment_tolerance: inst.get_f32("alignmentTolerance").unwrap_or_default(),
            alignment_range: inst.get_f32("alignmentRange").unwrap_or_default(),
            start_closing_time: inst.get_f32("startClosingTime").unwrap_or_default(),
            ship_pull_in_delay_time: inst.get_f32("shipPullInDelayTime").unwrap_or_default(),
            ship_pull_in_buffer_time: inst.get_f32("shipPullInBufferTime").unwrap_or_default(),
            debris_push_out_acceleration: inst.get_f32("debrisPushOutAcceleration").unwrap_or_default(),
            debris_push_out_maximum_speed: inst.get_f32("debrisPushOutMaximumSpeed").unwrap_or_default(),
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpPointEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpPointEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_cone_params: match inst.get("uiConeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveUIConeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveUIConeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpTunnelLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelLightParams {
    /// DCB field: `distanceAhead` (Single)
    #[serde(default)]
    pub distance_ahead: f32,
    /// DCB field: `intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `bulbRadius` (Single)
    #[serde(default)]
    pub bulb_radius: f32,
    /// DCB field: `animSpeed` (Single)
    #[serde(default)]
    pub anim_speed: f32,
    /// DCB field: `lightStyle` (Byte)
    #[serde(default)]
    pub light_style: u32,
}

impl Pooled for GlobalJumpTunnelLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_light_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelLightParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelLightParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            distance_ahead: inst.get_f32("distanceAhead").unwrap_or_default(),
            intensity: inst.get_f32("intensity").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            bulb_radius: inst.get_f32("bulbRadius").unwrap_or_default(),
            anim_speed: inst.get_f32("animSpeed").unwrap_or_default(),
            light_style: inst.get_u32("lightStyle").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelFogParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelFogParams {
    /// DCB field: `fogStartOffset` (Single)
    #[serde(default)]
    pub fog_start_offset: f32,
    /// DCB field: `fogEndOffset` (Single)
    #[serde(default)]
    pub fog_end_offset: f32,
    /// DCB field: `fogEndIntensityDistanceRange` (Class)
    #[serde(default)]
    pub fog_end_intensity_distance_range: Option<Handle<Range>>,
    /// DCB field: `fogAnimatedIntensityScaleRange` (Class)
    #[serde(default)]
    pub fog_animated_intensity_scale_range: Option<Handle<Range>>,
}

impl Pooled for GlobalJumpTunnelFogParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_fog_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_fog_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelFogParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelFogParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fog_start_offset: inst.get_f32("fogStartOffset").unwrap_or_default(),
            fog_end_offset: inst.get_f32("fogEndOffset").unwrap_or_default(),
            fog_end_intensity_distance_range: match inst.get("fogEndIntensityDistanceRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fog_animated_intensity_scale_range: match inst.get("fogAnimatedIntensityScaleRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpTunnelPassByLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelPassByLightParams {
    /// DCB field: `intensityRange` (Class)
    #[serde(default)]
    pub intensity_range: Option<Handle<Range>>,
    /// DCB field: `radiusRange` (Class)
    #[serde(default)]
    pub radius_range: Option<Handle<Range>>,
    /// DCB field: `bulbRange` (Class)
    #[serde(default)]
    pub bulb_range: Option<Handle<Range>>,
    /// DCB field: `spacingRange` (Class)
    #[serde(default)]
    pub spacing_range: Option<Handle<Range>>,
    /// DCB field: `distanceFromSpline` (Class)
    #[serde(default)]
    pub distance_from_spline: Option<Handle<Range>>,
    /// DCB field: `speedRange` (Class)
    #[serde(default)]
    pub speed_range: Option<Handle<Range>>,
    /// DCB field: `colorRandomOffsetRange` (Class)
    #[serde(default)]
    pub color_random_offset_range: Option<Handle<Range>>,
    /// DCB field: `entranceOffset` (Single)
    #[serde(default)]
    pub entrance_offset: f32,
    /// DCB field: `spawnChance` (Single)
    #[serde(default)]
    pub spawn_chance: f32,
    /// DCB field: `maxRange` (Single)
    #[serde(default)]
    pub max_range: f32,
    /// DCB field: `fadeOutDistStart` (Single)
    #[serde(default)]
    pub fade_out_dist_start: f32,
}

impl Pooled for GlobalJumpTunnelPassByLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_pass_by_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_pass_by_light_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelPassByLightParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelPassByLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            intensity_range: match inst.get("intensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radius_range: match inst.get("radiusRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bulb_range: match inst.get("bulbRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spacing_range: match inst.get("spacingRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_from_spline: match inst.get("distanceFromSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed_range: match inst.get("speedRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color_random_offset_range: match inst.get("colorRandomOffsetRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entrance_offset: inst.get_f32("entranceOffset").unwrap_or_default(),
            spawn_chance: inst.get_f32("spawnChance").unwrap_or_default(),
            max_range: inst.get_f32("maxRange").unwrap_or_default(),
            fade_out_dist_start: inst.get_f32("fadeOutDistStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelProbeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelProbeParams {
    /// DCB field: `probeRadius` (Single)
    #[serde(default)]
    pub probe_radius: f32,
}

impl Pooled for GlobalJumpTunnelProbeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_probe_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_probe_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelProbeParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelProbeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            probe_radius: inst.get_f32("probeRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelEffectParams {
    /// DCB field: `failEffect` (Class)
    #[serde(default)]
    pub fail_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `interiorExitEffect` (Class)
    #[serde(default)]
    pub interior_exit_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `spaceloopEffect` (Class)
    #[serde(default)]
    pub spaceloop_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `centralSplineEffect` (Class)
    #[serde(default)]
    pub central_spline_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `spaceFillingSplineEffect` (Class)
    #[serde(default)]
    pub space_filling_spline_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `sunFlareEffect` (Class)
    #[serde(default)]
    pub sun_flare_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `sunLightParams` (Class)
    #[serde(default)]
    pub sun_light_params: Option<Handle<GlobalJumpTunnelLightParams>>,
    /// DCB field: `probeParams` (Class)
    #[serde(default)]
    pub probe_params: Option<Handle<GlobalJumpTunnelProbeParams>>,
    /// DCB field: `fogParams` (Class)
    #[serde(default)]
    pub fog_params: Option<Handle<GlobalJumpTunnelFogParams>>,
    /// DCB field: `passByLightParams` (Class)
    #[serde(default)]
    pub pass_by_light_params: Option<Handle<GlobalJumpTunnelPassByLightParams>>,
    /// DCB field: `splineLength` (Single)
    #[serde(default)]
    pub spline_length: f32,
}

impl Pooled for GlobalJumpTunnelEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fail_effect: match inst.get("failEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            interior_exit_effect: match inst.get("interiorExitEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spaceloop_effect: match inst.get("spaceloopEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            central_spline_effect: match inst.get("centralSplineEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            space_filling_spline_effect: match inst.get("spaceFillingSplineEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sun_flare_effect: match inst.get("sunFlareEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sun_light_params: match inst.get("sunLightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            probe_params: match inst.get("probeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelProbeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelProbeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fog_params: match inst.get("fogParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelFogParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelFogParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pass_by_light_params: match inst.get("passByLightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelPassByLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelPassByLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spline_length: inst.get_f32("splineLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpTunnelCameraEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelCameraEffectParams {
    /// DCB field: `alignmentToSpline` (Class)
    #[serde(default)]
    pub alignment_to_spline: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `distortionRatio` (Class)
    #[serde(default)]
    pub distortion_ratio: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `openingProximity` (Class)
    #[serde(default)]
    pub opening_proximity: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `wallProximity` (Class)
    #[serde(default)]
    pub wall_proximity: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `failureState` (Class)
    #[serde(default)]
    pub failure_state: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `velocityStrength` (Class)
    #[serde(default)]
    pub velocity_strength: Option<Handle<JumpTunnelCameraEffects>>,
    /// DCB field: `velocityStrengthParams` (Class)
    #[serde(default)]
    pub velocity_strength_params: Option<Handle<JumpDriveVelocityStrengthParams>>,
}

impl Pooled for GlobalJumpTunnelCameraEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_camera_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_camera_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelCameraEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelCameraEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            alignment_to_spline: match inst.get("alignmentToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distortion_ratio: match inst.get("distortionRatio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            opening_proximity: match inst.get("openingProximity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wall_proximity: match inst.get("wallProximity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failure_state: match inst.get("failureState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            velocity_strength: match inst.get("velocityStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            velocity_strength_params: match inst.get("velocityStrengthParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveVelocityStrengthParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveVelocityStrengthParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalJumpTunnelHostParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpTunnelHostParams {
    /// DCB field: `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `tunnelGenerationParams` (Class)
    #[serde(default)]
    pub tunnel_generation_params: Option<Handle<SJumpTunnelGenerationParams>>,
    /// DCB field: `tunnelDistortionParams` (Class)
    #[serde(default)]
    pub tunnel_distortion_params: Option<Handle<SJumpTunnelDistortionParams>>,
    /// DCB field: `tunnelFailureParams` (Class)
    #[serde(default)]
    pub tunnel_failure_params: Option<Handle<SJumpTunnelFailureParams>>,
    /// DCB field: `tunnelExitParams` (Class)
    #[serde(default)]
    pub tunnel_exit_params: Option<Handle<SJumpTunnelExitParams>>,
    /// DCB field: `visualParams` (Class)
    #[serde(default)]
    pub visual_params: Option<Handle<SJumpTunnelVisualParams>>,
    /// DCB field: `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpTunnelEffectParams>>,
    /// DCB field: `entityPullInAcceleration` (Single)
    #[serde(default)]
    pub entity_pull_in_acceleration: f32,
}

impl Pooled for GlobalJumpTunnelHostParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_tunnel_host_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_tunnel_host_params }
}

impl<'a> Extract<'a> for GlobalJumpTunnelHostParams {
    const TYPE_NAME: &'static str = "GlobalJumpTunnelHostParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_generation_params: match inst.get("tunnelGenerationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelGenerationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelGenerationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_distortion_params: match inst.get("tunnelDistortionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelDistortionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelDistortionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_failure_params: match inst.get("tunnelFailureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelFailureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelFailureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_exit_params: match inst.get("tunnelExitParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelExitParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelExitParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_params: match inst.get("visualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpTunnelVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpTunnelVisualParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_pull_in_acceleration: inst.get_f32("entityPullInAcceleration").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveTuningEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveTuningEffectParams {
    /// DCB field: `interferenceStrength` (Single)
    #[serde(default)]
    pub interference_strength: f32,
    /// DCB field: `midPointForInterferencePercentage` (Single)
    #[serde(default)]
    pub mid_point_for_interference_percentage: f32,
}

impl Pooled for GlobalJumpDriveTuningEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_drive_tuning_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_drive_tuning_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveTuningEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveTuningEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interference_strength: inst.get_f32("interferenceStrength").unwrap_or_default(),
            mid_point_for_interference_percentage: inst.get_f32("midPointForInterferencePercentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveEntryEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveEntryEffectParams {
    /// DCB field: `trailStrengthMaxDistance` (Single)
    #[serde(default)]
    pub trail_strength_max_distance: f32,
}

impl Pooled for GlobalJumpDriveEntryEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_drive_entry_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_drive_entry_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveEntryEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveEntryEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trail_strength_max_distance: inst.get_f32("trailStrengthMaxDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveExitEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveExitEffectParams {
    /// DCB field: `trailStrengthDelay` (Single)
    #[serde(default)]
    pub trail_strength_delay: f32,
}

impl Pooled for GlobalJumpDriveExitEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_drive_exit_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_drive_exit_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveExitEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveExitEffectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trail_strength_delay: inst.get_f32("trailStrengthDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveEffectParams {
    /// DCB field: `tuningParams` (Class)
    #[serde(default)]
    pub tuning_params: Option<Handle<GlobalJumpDriveTuningEffectParams>>,
    /// DCB field: `entryParams` (Class)
    #[serde(default)]
    pub entry_params: Option<Handle<GlobalJumpDriveEntryEffectParams>>,
    /// DCB field: `exitParams` (Class)
    #[serde(default)]
    pub exit_params: Option<Handle<GlobalJumpDriveExitEffectParams>>,
    /// DCB field: `failureBuildUpTime` (Single)
    #[serde(default)]
    pub failure_build_up_time: f32,
    /// DCB field: `failureDissipationTime` (Single)
    #[serde(default)]
    pub failure_dissipation_time: f32,
}

impl Pooled for GlobalJumpDriveEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_drive_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_drive_effect_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveEffectParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tuning_params: match inst.get("tuningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveTuningEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpDriveTuningEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entry_params: match inst.get("entryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveEntryEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpDriveEntryEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exit_params: match inst.get("exitParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveExitEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpDriveExitEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failure_build_up_time: inst.get_f32("failureBuildUpTime").unwrap_or_default(),
            failure_dissipation_time: inst.get_f32("failureDissipationTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalJumpDriveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJumpDriveParams {
    /// DCB field: `audioParams` (Class)
    #[serde(default)]
    pub audio_params: Option<Handle<JumpDriveAudioParams>>,
    /// DCB field: `musicParams` (Class)
    #[serde(default)]
    pub music_params: Option<Handle<JumpDriveMusicParams>>,
    /// DCB field: `effectParams` (Class)
    #[serde(default)]
    pub effect_params: Option<Handle<GlobalJumpDriveEffectParams>>,
    /// DCB field: `approachRingParams` (Class)
    #[serde(default)]
    pub approach_ring_params: Option<Handle<JumpDriveApproachRingsParams>>,
    /// DCB field: `malfunction` (StrongPointer)
    #[serde(default)]
    pub malfunction: Option<Handle<SMisfireEffect>>,
    /// DCB field: `checksPassedDelay` (Single)
    #[serde(default)]
    pub checks_passed_delay: f32,
    /// DCB field: `obstructionPaddingSize` (Single)
    #[serde(default)]
    pub obstruction_padding_size: f32,
    /// DCB field: `wallRepelBounceVelocity` (Single)
    #[serde(default)]
    pub wall_repel_bounce_velocity: f32,
}

impl Pooled for GlobalJumpDriveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_jump_drive_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_jump_drive_params }
}

impl<'a> Extract<'a> for GlobalJumpDriveParams {
    const TYPE_NAME: &'static str = "GlobalJumpDriveParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            music_params: match inst.get("musicParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveMusicParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effect_params: match inst.get("effectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpDriveEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpDriveEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            approach_ring_params: match inst.get("approachRingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveApproachRingsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveApproachRingsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            malfunction: match inst.get("malfunction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMisfireEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMisfireEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            checks_passed_delay: inst.get_f32("checksPassedDelay").unwrap_or_default(),
            obstruction_padding_size: inst.get_f32("obstructionPaddingSize").unwrap_or_default(),
            wall_repel_bounce_velocity: inst.get_f32("wallRepelBounceVelocity").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalCargoLoadingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCargoLoadingParams {
    /// DCB field: `vehicleVelocityEpsilon` (Single)
    #[serde(default)]
    pub vehicle_velocity_epsilon: f32,
    /// DCB field: `vehicleMovementTimeLimit` (Single)
    #[serde(default)]
    pub vehicle_movement_time_limit: f32,
    /// DCB field: `uiDisplayTimeLimit` (Single)
    #[serde(default)]
    pub ui_display_time_limit: f32,
    /// DCB field: `revokeTimeDelay` (Single)
    #[serde(default)]
    pub revoke_time_delay: f32,
    /// DCB field: `uiTimeRemainingForTimeOutWarning` (Single)
    #[serde(default)]
    pub ui_time_remaining_for_time_out_warning: f32,
    /// DCB field: `forfeitTimeBuffer` (Single)
    #[serde(default)]
    pub forfeit_time_buffer: f32,
    /// DCB field: `initialMovementHintTimeBuffer` (Single)
    #[serde(default)]
    pub initial_movement_hint_time_buffer: f32,
    /// DCB field: `cargoDeckLoadingTimePerSCU` (Single)
    #[serde(default)]
    pub cargo_deck_loading_time_per_scu: f32,
    /// DCB field: `autoLoadingBaseLoadingTime` (Single)
    #[serde(default)]
    pub auto_loading_base_loading_time: f32,
    /// DCB field: `autoLoadingBaseUnloadingTime` (Single)
    #[serde(default)]
    pub auto_loading_base_unloading_time: f32,
    /// DCB field: `autoLoadingBoxSizeLoadingTime` (Class)
    #[serde(default)]
    pub auto_loading_box_size_loading_time: Option<Handle<SAutoLoadingBoxSizeLoadingTime>>,
    /// DCB field: `autoLoadingBoxSizeUnloadingTime` (Class)
    #[serde(default)]
    pub auto_loading_box_size_unloading_time: Option<Handle<SAutoLoadingBoxSizeLoadingTime>>,
    /// DCB field: `warningCargoRemovalNotification` (Class)
    #[serde(default)]
    pub warning_cargo_removal_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyCargoRemovalNotification` (Class)
    #[serde(default)]
    pub notify_cargo_removal_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `reminderRetrieveCargoFromLoadingAreaNotification` (Class)
    #[serde(default)]
    pub reminder_retrieve_cargo_from_loading_area_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyCargoTransferredNotification` (Class)
    #[serde(default)]
    pub notify_cargo_transferred_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyCargoTransferInterruptedObstructionNotification` (Class)
    #[serde(default)]
    pub notify_cargo_transfer_interrupted_obstruction_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyCargoTransferInterruptedShipMovingNotification` (Class)
    #[serde(default)]
    pub notify_cargo_transfer_interrupted_ship_moving_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyCargoTransferInterruptedGenericNotification` (Class)
    #[serde(default)]
    pub notify_cargo_transfer_interrupted_generic_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `initialMovementHintTimeBufferNotification` (Class)
    #[serde(default)]
    pub initial_movement_hint_time_buffer_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyLoadingAreaRevokedTimeoutNotification` (Class)
    #[serde(default)]
    pub notify_loading_area_revoked_timeout_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// DCB field: `notifyLoadingAreaRevokedGenericNotification` (Class)
    #[serde(default)]
    pub notify_loading_area_revoked_generic_notification: Option<Handle<CargoLoadingNotificationParams>>,
}

impl Pooled for GlobalCargoLoadingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_cargo_loading_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_cargo_loading_params }
}

impl<'a> Extract<'a> for GlobalCargoLoadingParams {
    const TYPE_NAME: &'static str = "GlobalCargoLoadingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_velocity_epsilon: inst.get_f32("vehicleVelocityEpsilon").unwrap_or_default(),
            vehicle_movement_time_limit: inst.get_f32("vehicleMovementTimeLimit").unwrap_or_default(),
            ui_display_time_limit: inst.get_f32("uiDisplayTimeLimit").unwrap_or_default(),
            revoke_time_delay: inst.get_f32("revokeTimeDelay").unwrap_or_default(),
            ui_time_remaining_for_time_out_warning: inst.get_f32("uiTimeRemainingForTimeOutWarning").unwrap_or_default(),
            forfeit_time_buffer: inst.get_f32("forfeitTimeBuffer").unwrap_or_default(),
            initial_movement_hint_time_buffer: inst.get_f32("initialMovementHintTimeBuffer").unwrap_or_default(),
            cargo_deck_loading_time_per_scu: inst.get_f32("cargoDeckLoadingTimePerSCU").unwrap_or_default(),
            auto_loading_base_loading_time: inst.get_f32("autoLoadingBaseLoadingTime").unwrap_or_default(),
            auto_loading_base_unloading_time: inst.get_f32("autoLoadingBaseUnloadingTime").unwrap_or_default(),
            auto_loading_box_size_loading_time: match inst.get("autoLoadingBoxSizeLoadingTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_loading_box_size_unloading_time: match inst.get("autoLoadingBoxSizeUnloadingTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            warning_cargo_removal_notification: match inst.get("warningCargoRemovalNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_cargo_removal_notification: match inst.get("notifyCargoRemovalNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reminder_retrieve_cargo_from_loading_area_notification: match inst.get("reminderRetrieveCargoFromLoadingAreaNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_cargo_transferred_notification: match inst.get("notifyCargoTransferredNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_cargo_transfer_interrupted_obstruction_notification: match inst.get("notifyCargoTransferInterruptedObstructionNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_cargo_transfer_interrupted_ship_moving_notification: match inst.get("notifyCargoTransferInterruptedShipMovingNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_cargo_transfer_interrupted_generic_notification: match inst.get("notifyCargoTransferInterruptedGenericNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            initial_movement_hint_time_buffer_notification: match inst.get("initialMovementHintTimeBufferNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_loading_area_revoked_timeout_notification: match inst.get("notifyLoadingAreaRevokedTimeoutNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notify_loading_area_revoked_generic_notification: match inst.get("notifyLoadingAreaRevokedGenericNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoLoadingNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalMarkerConfigs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMarkerConfigs {
    /// DCB field: `missionPointMarkerConfig` (Reference)
    #[serde(default)]
    pub mission_point_marker_config: Option<CigGuid>,
    /// DCB field: `partyMemberMarkerConfig` (Reference)
    #[serde(default)]
    pub party_member_marker_config: Option<CigGuid>,
    /// DCB field: `landingAreaMarkerConfig` (Reference)
    #[serde(default)]
    pub landing_area_marker_config: Option<CigGuid>,
    /// DCB field: `unattendedVehicleMarkerConfig` (Reference)
    #[serde(default)]
    pub unattended_vehicle_marker_config: Option<CigGuid>,
}

impl Pooled for GlobalMarkerConfigs {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_marker_configs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_marker_configs }
}

impl<'a> Extract<'a> for GlobalMarkerConfigs {
    const TYPE_NAME: &'static str = "GlobalMarkerConfigs";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_point_marker_config: inst.get("missionPointMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            party_member_marker_config: inst.get("partyMemberMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            landing_area_marker_config: inst.get("landingAreaMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unattended_vehicle_marker_config: inst.get("unattendedVehicleMarkerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `GlobalMissionSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMissionSettings {
    /// DCB field: `locationValidation` (StrongPointer (array))
    #[serde(default)]
    pub location_validation: Vec<Handle<MissionLocationValidation>>,
    /// DCB field: `defaultJurisdictionForPlayerCrimeStats` (Reference)
    #[serde(default)]
    pub default_jurisdiction_for_player_crime_stats: Option<CigGuid>,
    /// DCB field: `PVPBountyContractGenerators` (Class (array))
    #[serde(default)]
    pub pvpbounty_contract_generators: Vec<Handle<SPVPBountyContractGenerators>>,
}

impl Pooled for GlobalMissionSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_mission_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_mission_settings }
}

impl<'a> Extract<'a> for GlobalMissionSettings {
    const TYPE_NAME: &'static str = "GlobalMissionSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_validation: inst.get_array("locationValidation")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionLocationValidation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionLocationValidation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_jurisdiction_for_player_crime_stats: inst.get("defaultJurisdictionForPlayerCrimeStats").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pvpbounty_contract_generators: inst.get_array("PVPBountyContractGenerators")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPVPBountyContractGenerators>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPVPBountyContractGenerators>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalFogVolume`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalFogVolume {
    /// DCB field: `fogSize` (Class)
    #[serde(default)]
    pub fog_size: Option<Handle<Vec3>>,
    /// DCB field: `noiseLifeTime` (Single)
    #[serde(default)]
    pub noise_life_time: f32,
    /// DCB field: `softEdge` (Single)
    #[serde(default)]
    pub soft_edge: f32,
    /// DCB field: `hideFarLodThreshold` (Single)
    #[serde(default)]
    pub hide_far_lod_threshold: f32,
    /// DCB field: `fadeFarLodThreshold` (Single)
    #[serde(default)]
    pub fade_far_lod_threshold: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
}

impl Pooled for GlobalFogVolume {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_fog_volume }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_fog_volume }
}

impl<'a> Extract<'a> for GlobalFogVolume {
    const TYPE_NAME: &'static str = "GlobalFogVolume";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fog_size: match inst.get("fogSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
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

/// DCB type: `GlobalResourceCGF`
///
/// Inherits from: `GlobalResourceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceCGF {
    /// DCB field: `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceCGF {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_resource_cgf }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_resource_cgf }
}

impl<'a> Extract<'a> for GlobalResourceCGF {
    const TYPE_NAME: &'static str = "GlobalResourceCGF";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalResourceGeometry`
///
/// Inherits from: `GlobalResourceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceGeometry {
    /// DCB field: `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceGeometry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_resource_geometry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_resource_geometry }
}

impl<'a> Extract<'a> for GlobalResourceGeometry {
    const TYPE_NAME: &'static str = "GlobalResourceGeometry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalResourceParticle`
///
/// Inherits from: `GlobalResourceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceParticle {
    /// DCB field: `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceParticle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_resource_particle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_resource_particle }
}

impl<'a> Extract<'a> for GlobalResourceParticle {
    const TYPE_NAME: &'static str = "GlobalResourceParticle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalResourceMaterial`
///
/// Inherits from: `GlobalResourceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceMaterial {
    /// DCB field: `path` (String)
    #[serde(default)]
    pub path: String,
}

impl Pooled for GlobalResourceMaterial {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_resource_material }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_resource_material }
}

impl<'a> Extract<'a> for GlobalResourceMaterial {
    const TYPE_NAME: &'static str = "GlobalResourceMaterial";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalResourceAudio`
///
/// Inherits from: `GlobalResourceBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalResourceAudio {
    /// DCB field: `audioTrigger` (String)
    #[serde(default)]
    pub audio_trigger: String,
}

impl Pooled for GlobalResourceAudio {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_resource_audio }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_resource_audio }
}

impl<'a> Extract<'a> for GlobalResourceAudio {
    const TYPE_NAME: &'static str = "GlobalResourceAudio";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_trigger: inst.get_str("audioTrigger").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalGasParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalGasParams {
    /// DCB field: `transferRatePerAtmosphereSquareMetre` (Double)
    #[serde(default)]
    pub transfer_rate_per_atmosphere_square_metre: f64,
    /// DCB field: `minimumTransferRate` (Double)
    #[serde(default)]
    pub minimum_transfer_rate: f64,
    /// DCB field: `mixRatePerSquareMetre` (Double)
    #[serde(default)]
    pub mix_rate_per_square_metre: f64,
    /// DCB field: `mixAbsPressureDifferenceThreshold` (Double)
    #[serde(default)]
    pub mix_abs_pressure_difference_threshold: f64,
    /// DCB field: `massAdditionRatePerCubicMetre_MatchingGases` (Double)
    #[serde(default)]
    pub mass_addition_rate_per_cubic_metre_matching_gases: f64,
    /// DCB field: `massSubtractionRatePerCubicMetre_MatchingGases` (Double)
    #[serde(default)]
    pub mass_subtraction_rate_per_cubic_metre_matching_gases: f64,
    /// DCB field: `massSubtractionRatePerCubicMetre_ForeignGases` (Double)
    #[serde(default)]
    pub mass_subtraction_rate_per_cubic_metre_foreign_gases: f64,
    /// DCB field: `resourceCostPerKilogramCorrected` (Double)
    #[serde(default)]
    pub resource_cost_per_kilogram_corrected: f64,
    /// DCB field: `thermalEnergyCorrectionRatePerCubicMetre` (Double)
    #[serde(default)]
    pub thermal_energy_correction_rate_per_cubic_metre: f64,
    /// DCB field: `resourceCostPerJoule` (Double)
    #[serde(default)]
    pub resource_cost_per_joule: f64,
    /// DCB field: `humidityCorrectionRate` (Double)
    #[serde(default)]
    pub humidity_correction_rate: f64,
    /// DCB field: `resourceCostPerHumidity` (Double)
    #[serde(default)]
    pub resource_cost_per_humidity: f64,
    /// DCB field: `apparentTemperatureParams` (Class)
    #[serde(default)]
    pub apparent_temperature_params: Option<Handle<ApparentTemperatureParams>>,
}

impl Pooled for GlobalGasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_gas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_gas_params }
}

impl<'a> Extract<'a> for GlobalGasParams {
    const TYPE_NAME: &'static str = "GlobalGasParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            transfer_rate_per_atmosphere_square_metre: inst.get_f64("transferRatePerAtmosphereSquareMetre").unwrap_or_default(),
            minimum_transfer_rate: inst.get_f64("minimumTransferRate").unwrap_or_default(),
            mix_rate_per_square_metre: inst.get_f64("mixRatePerSquareMetre").unwrap_or_default(),
            mix_abs_pressure_difference_threshold: inst.get_f64("mixAbsPressureDifferenceThreshold").unwrap_or_default(),
            mass_addition_rate_per_cubic_metre_matching_gases: inst.get_f64("massAdditionRatePerCubicMetre_MatchingGases").unwrap_or_default(),
            mass_subtraction_rate_per_cubic_metre_matching_gases: inst.get_f64("massSubtractionRatePerCubicMetre_MatchingGases").unwrap_or_default(),
            mass_subtraction_rate_per_cubic_metre_foreign_gases: inst.get_f64("massSubtractionRatePerCubicMetre_ForeignGases").unwrap_or_default(),
            resource_cost_per_kilogram_corrected: inst.get_f64("resourceCostPerKilogramCorrected").unwrap_or_default(),
            thermal_energy_correction_rate_per_cubic_metre: inst.get_f64("thermalEnergyCorrectionRatePerCubicMetre").unwrap_or_default(),
            resource_cost_per_joule: inst.get_f64("resourceCostPerJoule").unwrap_or_default(),
            humidity_correction_rate: inst.get_f64("humidityCorrectionRate").unwrap_or_default(),
            resource_cost_per_humidity: inst.get_f64("resourceCostPerHumidity").unwrap_or_default(),
            apparent_temperature_params: match inst.get("apparentTemperatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ApparentTemperatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ApparentTemperatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalRoomStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalRoomStateParams {
    /// DCB field: `typeRanges` (Class)
    #[serde(default)]
    pub type_ranges: Option<Handle<Range>>,
    /// DCB field: `typeDebugColors` (Class)
    #[serde(default)]
    pub type_debug_colors: Option<Handle<RGB>>,
    /// DCB field: `debugParticles` (Class)
    #[serde(default)]
    pub debug_particles: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `defaultSpaceDust` (Class)
    #[serde(default)]
    pub default_space_dust: Option<Handle<WeatherEffects_SpaceLoopEffect>>,
}

impl Pooled for GlobalRoomStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_room_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_room_state_params }
}

impl<'a> Extract<'a> for GlobalRoomStateParams {
    const TYPE_NAME: &'static str = "GlobalRoomStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_ranges: match inst.get("typeRanges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            type_debug_colors: match inst.get("typeDebugColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            debug_particles: match inst.get("debugParticles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_space_dust: match inst.get("defaultSpaceDust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeatherEffects_SpaceLoopEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WeatherEffects_SpaceLoopEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalTutorialParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTutorialParams {
    /// DCB field: `validStartingAreas` (Locale (array))
    #[serde(default)]
    pub valid_starting_areas: Vec<String>,
}

impl Pooled for GlobalTutorialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gl.global_tutorial_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gl.global_tutorial_params }
}

impl<'a> Extract<'a> for GlobalTutorialParams {
    const TYPE_NAME: &'static str = "GlobalTutorialParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            valid_starting_areas: inst.get_array("validStartingAreas")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

