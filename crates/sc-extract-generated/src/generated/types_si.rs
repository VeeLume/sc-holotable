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

/// DCB type: `SignatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureParams {
    /// DCB field: `defaultBodyTemperature` (Single)
    #[serde(default)]
    pub default_body_temperature: f32,
    /// DCB field: `distressIdentitySignature` (Single)
    #[serde(default)]
    pub distress_identity_signature: f32,
}

impl Pooled for SignatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.signature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.signature_params }
}

impl<'a> Extract<'a> for SignatureParams {
    const TYPE_NAME: &'static str = "SignatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_body_temperature: inst.get_f32("defaultBodyTemperature").unwrap_or_default(),
            distress_identity_signature: inst.get_f32("distressIdentitySignature").unwrap_or_default(),
        }
    }
}

/// DCB type: `SInputDeflectionTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInputDeflectionTimeParams {
    /// DCB field: `minDeflectionTime` (Single)
    #[serde(default)]
    pub min_deflection_time: f32,
    /// DCB field: `maxDeflectionTime` (Single)
    #[serde(default)]
    pub max_deflection_time: f32,
    /// DCB field: `penaltyMapping` (Class)
    #[serde(default)]
    pub penalty_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SInputDeflectionTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinput_deflection_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinput_deflection_time_params }
}

impl<'a> Extract<'a> for SInputDeflectionTimeParams {
    const TYPE_NAME: &'static str = "SInputDeflectionTimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_deflection_time: inst.get_f32("minDeflectionTime").unwrap_or_default(),
            max_deflection_time: inst.get_f32("maxDeflectionTime").unwrap_or_default(),
            penalty_mapping: match inst.get("penaltyMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SIRoundsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIRoundsModule {
}

impl Pooled for SIRoundsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sirounds_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sirounds_module }
}

impl<'a> Extract<'a> for SIRoundsModule {
    const TYPE_NAME: &'static str = "SIRoundsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemPortLoadoutBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemPortLoadoutBaseParams {
    /// DCB field: `WearRange` (StrongPointer)
    #[serde(default)]
    pub wear_range: Option<Handle<Range>>,
    /// DCB field: `DirtRange` (StrongPointer)
    #[serde(default)]
    pub dirt_range: Option<Handle<Range>>,
    /// DCB field: `SkipInventoryItemsOnMissionEntities` (Boolean)
    #[serde(default)]
    pub skip_inventory_items_on_mission_entities: bool,
    /// DCB field: `InventoryItems` (Class (array))
    #[serde(default)]
    pub inventory_items: Vec<Handle<SLoadoutInventoryItem>>,
}

impl Pooled for SItemPortLoadoutBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sitem_port_loadout_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sitem_port_loadout_base_params }
}

impl<'a> Extract<'a> for SItemPortLoadoutBaseParams {
    const TYPE_NAME: &'static str = "SItemPortLoadoutBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wear_range: match inst.get("WearRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dirt_range: match inst.get("DirtRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skip_inventory_items_on_mission_entities: inst.get_bool("SkipInventoryItemsOnMissionEntities").unwrap_or_default(),
            inventory_items: inst.get_array("InventoryItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SLoadoutInventoryItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SLoadoutInventoryItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifierNumber`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierNumber {
    /// DCB field: `value` (Single)
    #[serde(default)]
    pub value: f32,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SIFCSModifierNumber {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsmodifier_number }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsmodifier_number }
}

impl<'a> Extract<'a> for SIFCSModifierNumber {
    const TYPE_NAME: &'static str = "SIFCSModifierNumber";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_f32("value").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifierVector`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierVector {
    /// DCB field: `value` (Class)
    #[serde(default)]
    pub value: Option<Handle<Vec3>>,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SIFCSModifierVector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsmodifier_vector }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsmodifier_vector }
}

impl<'a> Extract<'a> for SIFCSModifierVector {
    const TYPE_NAME: &'static str = "SIFCSModifierVector";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifiersLegacy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifiersLegacy {
    /// DCB field: `numbers` (Class)
    #[serde(default)]
    pub numbers: Option<Handle<SIFCSModifierNumber>>,
    /// DCB field: `vectors` (Class)
    #[serde(default)]
    pub vectors: Option<Handle<SIFCSModifierVector>>,
}

impl Pooled for SIFCSModifiersLegacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsmodifiers_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsmodifiers_legacy }
}

impl<'a> Extract<'a> for SIFCSModifiersLegacy {
    const TYPE_NAME: &'static str = "SIFCSModifiersLegacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            numbers: match inst.get("numbers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierNumber>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSModifierNumber>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vectors: match inst.get("vectors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierVector>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSModifierVector>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SIFCSEspParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSEspParams {
    /// DCB field: `triggerZoneRampInCurve` (Class)
    #[serde(default)]
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `maxTrackingStrength` (Single)
    #[serde(default)]
    pub max_tracking_strength: f32,
    /// DCB field: `distanceFalloffStart` (Single)
    #[serde(default)]
    pub distance_falloff_start: f32,
    /// DCB field: `distanceFalloffEnd` (Single)
    #[serde(default)]
    pub distance_falloff_end: f32,
    /// DCB field: `outerZoneDeg` (Single)
    #[serde(default)]
    pub outer_zone_deg: f32,
    /// DCB field: `innerZoneRatio` (Single)
    #[serde(default)]
    pub inner_zone_ratio: f32,
    /// DCB field: `adsZoneMinSizeDeg` (Single)
    #[serde(default)]
    pub ads_zone_min_size_deg: f32,
    /// DCB field: `targetChangedRampTime` (Single)
    #[serde(default)]
    pub target_changed_ramp_time: f32,
    /// DCB field: `dampeningRange` (Single)
    #[serde(default)]
    pub dampening_range: f32,
    /// DCB field: `inputScalerMin` (Single)
    #[serde(default)]
    pub input_scaler_min: f32,
    /// DCB field: `inputScalerMax` (Single)
    #[serde(default)]
    pub input_scaler_max: f32,
    /// DCB field: `inputScalerSmoothTime` (Single)
    #[serde(default)]
    pub input_scaler_smooth_time: f32,
    /// DCB field: `inputRelaxSpeed` (Single)
    #[serde(default)]
    pub input_relax_speed: f32,
    /// DCB field: `allowPulling` (Boolean)
    #[serde(default)]
    pub allow_pulling: bool,
    /// DCB field: `smoothingTime` (Single)
    #[serde(default)]
    pub smoothing_time: f32,
    /// DCB field: `smoothingTimeDecreaseMultiplier` (Single)
    #[serde(default)]
    pub smoothing_time_decrease_multiplier: f32,
}

impl Pooled for SIFCSEspParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsesp_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsesp_params }
}

impl<'a> Extract<'a> for SIFCSEspParams {
    const TYPE_NAME: &'static str = "SIFCSEspParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_tracking_strength: inst.get_f32("maxTrackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            target_changed_ramp_time: inst.get_f32("targetChangedRampTime").unwrap_or_default(),
            dampening_range: inst.get_f32("dampeningRange").unwrap_or_default(),
            input_scaler_min: inst.get_f32("inputScalerMin").unwrap_or_default(),
            input_scaler_max: inst.get_f32("inputScalerMax").unwrap_or_default(),
            input_scaler_smooth_time: inst.get_f32("inputScalerSmoothTime").unwrap_or_default(),
            input_relax_speed: inst.get_f32("inputRelaxSpeed").unwrap_or_default(),
            allow_pulling: inst.get_bool("allowPulling").unwrap_or_default(),
            smoothing_time: inst.get_f32("smoothingTime").unwrap_or_default(),
            smoothing_time_decrease_multiplier: inst.get_f32("smoothingTimeDecreaseMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSEsp`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSEsp {
    /// DCB field: `espPerType` (Class)
    #[serde(default)]
    pub esp_per_type: Option<Handle<SIFCSEspParams>>,
}

impl Pooled for SIFCSEsp {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsesp }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsesp }
}

impl<'a> Extract<'a> for SIFCSEsp {
    const TYPE_NAME: &'static str = "SIFCSEsp";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            esp_per_type: match inst.get("espPerType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSEspParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSEspParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SIFCSGameModePhysicsDamping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSGameModePhysicsDamping {
    /// DCB field: `dampingValue` (Single)
    #[serde(default)]
    pub damping_value: f32,
    /// DCB field: `transitionTime` (Single)
    #[serde(default)]
    pub transition_time: f32,
}

impl Pooled for SIFCSGameModePhysicsDamping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsgame_mode_physics_damping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsgame_mode_physics_damping }
}

impl<'a> Extract<'a> for SIFCSGameModePhysicsDamping {
    const TYPE_NAME: &'static str = "SIFCSGameModePhysicsDamping";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damping_value: inst.get_f32("dampingValue").unwrap_or_default(),
            transition_time: inst.get_f32("transitionTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSGameModeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSGameModeParams {
    /// DCB field: `enableNewModel` (Boolean)
    #[serde(default)]
    pub enable_new_model: bool,
    /// DCB field: `enforceLegacyEspForNewModel` (Boolean)
    #[serde(default)]
    pub enforce_legacy_esp_for_new_model: bool,
    /// DCB field: `enableDecoupledGlidingDefault` (Boolean)
    #[serde(default)]
    pub enable_decoupled_gliding_default: bool,
    /// DCB field: `enableDecoupledGlidingCoreOff` (Boolean)
    #[serde(default)]
    pub enable_decoupled_gliding_core_off: bool,
    /// DCB field: `allowDisablingIFCSCore` (Boolean)
    #[serde(default)]
    pub allow_disabling_ifcscore: bool,
    /// DCB field: `cruiseModeOnByDefault` (Boolean)
    #[serde(default)]
    pub cruise_mode_on_by_default: bool,
    /// DCB field: `physicsDamping` (Class)
    #[serde(default)]
    pub physics_damping: Option<Handle<SIFCSGameModePhysicsDamping>>,
    /// DCB field: `legacyIncludeWindInAerodynamics` (Boolean)
    #[serde(default)]
    pub legacy_include_wind_in_aerodynamics: bool,
    /// DCB field: `legacyMaxAcceptedWindSpeed` (Single)
    #[serde(default)]
    pub legacy_max_accepted_wind_speed: f32,
}

impl Pooled for SIFCSGameModeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sifcsgame_mode_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sifcsgame_mode_params }
}

impl<'a> Extract<'a> for SIFCSGameModeParams {
    const TYPE_NAME: &'static str = "SIFCSGameModeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_new_model: inst.get_bool("enableNewModel").unwrap_or_default(),
            enforce_legacy_esp_for_new_model: inst.get_bool("enforceLegacyEspForNewModel").unwrap_or_default(),
            enable_decoupled_gliding_default: inst.get_bool("enableDecoupledGlidingDefault").unwrap_or_default(),
            enable_decoupled_gliding_core_off: inst.get_bool("enableDecoupledGlidingCoreOff").unwrap_or_default(),
            allow_disabling_ifcscore: inst.get_bool("allowDisablingIFCSCore").unwrap_or_default(),
            cruise_mode_on_by_default: inst.get_bool("cruiseModeOnByDefault").unwrap_or_default(),
            physics_damping: match inst.get("physicsDamping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSGameModePhysicsDamping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSGameModePhysicsDamping>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            legacy_include_wind_in_aerodynamics: inst.get_bool("legacyIncludeWindInAerodynamics").unwrap_or_default(),
            legacy_max_accepted_wind_speed: inst.get_f32("legacyMaxAcceptedWindSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SIPickupModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPickupModule {
}

impl Pooled for SIPickupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sipickup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sipickup_module }
}

impl<'a> Extract<'a> for SIPickupModule {
    const TYPE_NAME: &'static str = "SIPickupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIDamageHandlingModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDamageHandlingModule {
}

impl Pooled for SIDamageHandlingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sidamage_handling_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sidamage_handling_module }
}

impl<'a> Extract<'a> for SIDamageHandlingModule {
    const TYPE_NAME: &'static str = "SIDamageHandlingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIHostilityModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIHostilityModule {
    /// DCB field: `allowHostilityCheckOnQuit` (Boolean)
    #[serde(default)]
    pub allow_hostility_check_on_quit: bool,
}

impl Pooled for SIHostilityModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sihostility_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sihostility_module }
}

impl<'a> Extract<'a> for SIHostilityModule {
    const TYPE_NAME: &'static str = "SIHostilityModule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_hostility_check_on_quit: inst.get_bool("allowHostilityCheckOnQuit").unwrap_or_default(),
        }
    }
}

/// DCB type: `SISpectatorModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpectatorModule {
}

impl Pooled for SISpectatorModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sispectator_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sispectator_module }
}

impl<'a> Extract<'a> for SISpectatorModule {
    const TYPE_NAME: &'static str = "SISpectatorModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIScoringModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIScoringModule {
    /// DCB field: `countScoreDown` (Boolean)
    #[serde(default)]
    pub count_score_down: bool,
    /// DCB field: `playerScoring` (StrongPointer)
    #[serde(default)]
    pub player_scoring: Option<Handle<SPlayerScoring>>,
    /// DCB field: `teamScoring` (StrongPointer)
    #[serde(default)]
    pub team_scoring: Option<Handle<STeamScoring>>,
    /// DCB field: `gameCompletionAward` (StrongPointer)
    #[serde(default)]
    pub game_completion_award: Option<Handle<EAGameCompletionAwardBaseParams>>,
    /// DCB field: `RECMultiplier` (Single)
    #[serde(default)]
    pub recmultiplier: f32,
    /// DCB field: `mvpType` (EnumChoice)
    #[serde(default)]
    pub mvp_type: String,
}

impl Pooled for SIScoringModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siscoring_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siscoring_module }
}

impl<'a> Extract<'a> for SIScoringModule {
    const TYPE_NAME: &'static str = "SIScoringModule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            count_score_down: inst.get_bool("countScoreDown").unwrap_or_default(),
            player_scoring: match inst.get("playerScoring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SPlayerScoring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SPlayerScoring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            team_scoring: match inst.get("teamScoring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STeamScoring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STeamScoring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_completion_award: match inst.get("gameCompletionAward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EAGameCompletionAwardBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EAGameCompletionAwardBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recmultiplier: inst.get_f32("RECMultiplier").unwrap_or_default(),
            mvp_type: inst.get_str("mvpType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIPlayerSetupModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerSetupModule {
}

impl Pooled for SIPlayerSetupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siplayer_setup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siplayer_setup_module }
}

impl<'a> Extract<'a> for SIPlayerSetupModule {
    const TYPE_NAME: &'static str = "SIPlayerSetupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStatsRecordingModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStatsRecordingModule {
}

impl Pooled for SIStatsRecordingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sistats_recording_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sistats_recording_module }
}

impl<'a> Extract<'a> for SIStatsRecordingModule {
    const TYPE_NAME: &'static str = "SIStatsRecordingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SINotificationsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SINotificationsModule {
}

impl Pooled for SINotificationsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinotifications_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinotifications_module }
}

impl<'a> Extract<'a> for SINotificationsModule {
    const TYPE_NAME: &'static str = "SINotificationsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIObjectives`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIObjectives {
}

impl Pooled for SIObjectives {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siobjectives }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siobjectives }
}

impl<'a> Extract<'a> for SIObjectives {
    const TYPE_NAME: &'static str = "SIObjectives";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SICamerasModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SICamerasModule {
}

impl Pooled for SICamerasModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sicameras_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sicameras_module }
}

impl<'a> Extract<'a> for SICamerasModule {
    const TYPE_NAME: &'static str = "SICamerasModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIPlayerStats`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerStats {
}

impl Pooled for SIPlayerStats {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siplayer_stats }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siplayer_stats }
}

impl<'a> Extract<'a> for SIPlayerStats {
    const TYPE_NAME: &'static str = "SIPlayerStats";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISpawning`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpawning {
}

impl Pooled for SISpawning {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sispawning }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sispawning }
}

impl<'a> Extract<'a> for SISpawning {
    const TYPE_NAME: &'static str = "SISpawning";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVictoryConditionsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVictoryConditionsModule {
}

impl Pooled for SIVictoryConditionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sivictory_conditions_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sivictory_conditions_module }
}

impl<'a> Extract<'a> for SIVictoryConditionsModule {
    const TYPE_NAME: &'static str = "SIVictoryConditionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIParamsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIParamsModule {
}

impl Pooled for SIParamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siparams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siparams_module }
}

impl<'a> Extract<'a> for SIParamsModule {
    const TYPE_NAME: &'static str = "SIParamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISubsumptionMissionModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISubsumptionMissionModule {
}

impl Pooled for SISubsumptionMissionModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sisubsumption_mission_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sisubsumption_mission_module }
}

impl<'a> Extract<'a> for SISubsumptionMissionModule {
    const TYPE_NAME: &'static str = "SISubsumptionMissionModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIBettingModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIBettingModule {
}

impl Pooled for SIBettingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sibetting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sibetting_module }
}

impl<'a> Extract<'a> for SIBettingModule {
    const TYPE_NAME: &'static str = "SIBettingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIDifficultyModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDifficultyModule {
}

impl Pooled for SIDifficultyModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sidifficulty_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sidifficulty_module }
}

impl<'a> Extract<'a> for SIDifficultyModule {
    const TYPE_NAME: &'static str = "SIDifficultyModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIReputationModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIReputationModule {
}

impl Pooled for SIReputationModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sireputation_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sireputation_module }
}

impl<'a> Extract<'a> for SIReputationModule {
    const TYPE_NAME: &'static str = "SIReputationModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStateModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStateModule {
}

impl Pooled for SIStateModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sistate_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sistate_module }
}

impl<'a> Extract<'a> for SIStateModule {
    const TYPE_NAME: &'static str = "SIStateModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SITeamsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SITeamsModule {
}

impl Pooled for SITeamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.siteams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.siteams_module }
}

impl<'a> Extract<'a> for SITeamsModule {
    const TYPE_NAME: &'static str = "SITeamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVotingModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVotingModule {
}

impl Pooled for SIVotingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sivoting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sivoting_module }
}

impl<'a> Extract<'a> for SIVotingModule {
    const TYPE_NAME: &'static str = "SIVotingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemTypeFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemTypeFilter {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `itemSubTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_sub_types: Vec<String>,
}

impl Pooled for SItemTypeFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sitem_type_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sitem_type_filter }
}

impl<'a> Extract<'a> for SItemTypeFilter {
    const TYPE_NAME: &'static str = "SItemTypeFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            item_sub_types: inst.get_array("itemSubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SInitialDamageSpecifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInitialDamageSpecifierBase {
}

impl Pooled for SInitialDamageSpecifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinitial_damage_specifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinitial_damage_specifier_base }
}

impl<'a> Extract<'a> for SInitialDamageSpecifierBase {
    const TYPE_NAME: &'static str = "SInitialDamageSpecifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SInitialDamage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInitialDamage {
    /// DCB field: `RandomSeed` (Int32)
    #[serde(default)]
    pub random_seed: i32,
    /// DCB field: `Damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<Handle<SInitialDamageSpecifierBase>>,
    /// DCB field: `BoundingBoxScale` (Class)
    #[serde(default)]
    pub bounding_box_scale: Option<Handle<Vec3>>,
    /// DCB field: `MaxDamageRatio` (Single)
    #[serde(default)]
    pub max_damage_ratio: f32,
    /// DCB field: `MinHitCount` (Int32)
    #[serde(default)]
    pub min_hit_count: i32,
    /// DCB field: `MaxHitCount` (Int32)
    #[serde(default)]
    pub max_hit_count: i32,
    /// DCB field: `MinHitRadiusFraction` (Single)
    #[serde(default)]
    pub min_hit_radius_fraction: f32,
    /// DCB field: `MaxHitRadiusFraction` (Single)
    #[serde(default)]
    pub max_hit_radius_fraction: f32,
    /// DCB field: `HitDamageVariationFactor` (Single)
    #[serde(default)]
    pub hit_damage_variation_factor: f32,
    /// DCB field: `DamageMapDamageScale` (Single)
    #[serde(default)]
    pub damage_map_damage_scale: f32,
    /// DCB field: `DamageMapNoiseStrength` (Class)
    #[serde(default)]
    pub damage_map_noise_strength: Option<Handle<DamageMapChannels>>,
}

impl Pooled for SInitialDamage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinitial_damage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinitial_damage }
}

impl<'a> Extract<'a> for SInitialDamage {
    const TYPE_NAME: &'static str = "SInitialDamage";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            random_seed: inst.get_i32("RandomSeed").unwrap_or_default(),
            damage: match inst.get("Damage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInitialDamageSpecifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInitialDamageSpecifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bounding_box_scale: match inst.get("BoundingBoxScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_damage_ratio: inst.get_f32("MaxDamageRatio").unwrap_or_default(),
            min_hit_count: inst.get_i32("MinHitCount").unwrap_or_default(),
            max_hit_count: inst.get_i32("MaxHitCount").unwrap_or_default(),
            min_hit_radius_fraction: inst.get_f32("MinHitRadiusFraction").unwrap_or_default(),
            max_hit_radius_fraction: inst.get_f32("MaxHitRadiusFraction").unwrap_or_default(),
            hit_damage_variation_factor: inst.get_f32("HitDamageVariationFactor").unwrap_or_default(),
            damage_map_damage_scale: inst.get_f32("DamageMapDamageScale").unwrap_or_default(),
            damage_map_noise_strength: match inst.get("DamageMapNoiseStrength") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageMapChannels>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageMapChannels>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SInteractionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInteractionParams {
    /// DCB field: `Interaction` (WeakPointer)
    #[serde(default)]
    pub interaction: Option<Handle<SSharedInteractionParams>>,
    /// DCB field: `DisplayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `IsPrimary` (Boolean)
    #[serde(default)]
    pub is_primary: bool,
}

impl Pooled for SInteractionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinteraction_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinteraction_params }
}

impl<'a> Extract<'a> for SInteractionParams {
    const TYPE_NAME: &'static str = "SInteractionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction: match inst.get("Interaction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSharedInteractionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_name: inst.get_str("DisplayName").map(String::from).unwrap_or_default(),
            is_primary: inst.get_bool("IsPrimary").unwrap_or_default(),
        }
    }
}

/// DCB type: `SInteractionPointPrimitiveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInteractionPointPrimitiveParams {
}

impl Pooled for SInteractionPointPrimitiveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinteraction_point_primitive_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinteraction_point_primitive_params }
}

impl<'a> Extract<'a> for SInteractionPointPrimitiveParams {
    const TYPE_NAME: &'static str = "SInteractionPointPrimitiveParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SInteractionPointParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInteractionPointParams {
    /// DCB field: `Interactions` (StrongPointer (array))
    #[serde(default)]
    pub interactions: Vec<Handle<SInteractionParams>>,
    /// DCB field: `HelperName` (String)
    #[serde(default)]
    pub helper_name: String,
    /// DCB field: `Tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `Primitive` (StrongPointer)
    #[serde(default)]
    pub primitive: Option<Handle<SInteractionPointPrimitiveParams>>,
    /// DCB field: `PlayerAnimatedInteractionParams` (StrongPointer)
    #[serde(default)]
    pub player_animated_interaction_params: Option<Handle<PlayerAnimatedInteractionBase>>,
    /// DCB field: `HighlightBehaviors` (Class (array))
    #[serde(default)]
    pub highlight_behaviors: Vec<Handle<SHighlightBehaviorNode>>,
    /// DCB field: `AdditionalCollisionBones` (String (array))
    #[serde(default)]
    pub additional_collision_bones: Vec<String>,
    /// DCB field: `InteractionPointModifiers` (StrongPointer (array))
    #[serde(default)]
    pub interaction_point_modifiers: Vec<Handle<SInteractionPointModifier>>,
    /// DCB field: `InteractionPointInternalSettingTemplateRef` (Reference)
    #[serde(default)]
    pub interaction_point_internal_setting_template_ref: Option<CigGuid>,
    /// DCB field: `FirstInteractionPrimary` (Boolean)
    #[serde(default)]
    pub first_interaction_primary: bool,
    /// DCB field: `UseHelperLocation` (Boolean)
    #[serde(default)]
    pub use_helper_location: bool,
    /// DCB field: `ApplyToEntireSkeleton` (Boolean)
    #[serde(default)]
    pub apply_to_entire_skeleton: bool,
    /// DCB field: `longRangeIP` (Boolean)
    #[serde(default)]
    pub long_range_ip: bool,
    /// DCB field: `Radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `ShowHighlight` (Boolean)
    #[serde(default)]
    pub show_highlight: bool,
    /// DCB field: `HighlightOnRayHitEntity` (Boolean)
    #[serde(default)]
    pub highlight_on_ray_hit_entity: bool,
    /// DCB field: `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// DCB field: `TextJustification` (EnumChoice)
    #[serde(default)]
    pub text_justification: String,
    /// DCB field: `TextOrientation` (EnumChoice)
    #[serde(default)]
    pub text_orientation: String,
    /// DCB field: `TextScale` (Single)
    #[serde(default)]
    pub text_scale: f32,
    /// DCB field: `ShowInteractionTextAtCursorPos` (Boolean)
    #[serde(default)]
    pub show_interaction_text_at_cursor_pos: bool,
    /// DCB field: `ScalePromptOffsetWithEntity` (Boolean)
    #[serde(default)]
    pub scale_prompt_offset_with_entity: bool,
    /// DCB field: `InteractionPromptOffset` (Class)
    #[serde(default)]
    pub interaction_prompt_offset: Option<Handle<QuatT>>,
    /// DCB field: `PromptStyle` (EnumChoice)
    #[serde(default)]
    pub prompt_style: String,
    /// DCB field: `OverrideControlHintDescription` (Boolean)
    #[serde(default)]
    pub override_control_hint_description: bool,
    /// DCB field: `InteractionPromptScale` (Single)
    #[serde(default)]
    pub interaction_prompt_scale: f32,
    /// DCB field: `InteractionPromptLabelVerticalOffset` (Single)
    #[serde(default)]
    pub interaction_prompt_label_vertical_offset: f32,
    /// DCB field: `InteractionPromptBound` (EnumChoice)
    #[serde(default)]
    pub interaction_prompt_bound: String,
    /// DCB field: `UseHorizontalDistance` (Boolean)
    #[serde(default)]
    pub use_horizontal_distance: bool,
    /// DCB field: `MaxCursorScreenDistance` (Single)
    #[serde(default)]
    pub max_cursor_screen_distance: f32,
    /// DCB field: `ShowInnerThoughtsOffScreen` (Boolean)
    #[serde(default)]
    pub show_inner_thoughts_off_screen: bool,
    /// DCB field: `OnlySelectableInLookDir` (Boolean)
    #[serde(default)]
    pub only_selectable_in_look_dir: bool,
    /// DCB field: `AngleConstraint` (Class)
    #[serde(default)]
    pub angle_constraint: Option<Handle<SAngleConstraint>>,
}

impl Pooled for SInteractionPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinteraction_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinteraction_point_params }
}

impl<'a> Extract<'a> for SInteractionPointParams {
    const TYPE_NAME: &'static str = "SInteractionPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interactions: inst.get_array("Interactions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SInteractionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            helper_name: inst.get_str("HelperName").map(String::from).unwrap_or_default(),
            tags: inst.get_array("Tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            primitive: match inst.get("Primitive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInteractionPointPrimitiveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionPointPrimitiveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_animated_interaction_params: match inst.get("PlayerAnimatedInteractionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteractionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteractionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_behaviors: inst.get_array("HighlightBehaviors")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHighlightBehaviorNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHighlightBehaviorNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            additional_collision_bones: inst.get_array("AdditionalCollisionBones")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            interaction_point_modifiers: inst.get_array("InteractionPointModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SInteractionPointModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SInteractionPointModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            interaction_point_internal_setting_template_ref: inst.get("InteractionPointInternalSettingTemplateRef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            first_interaction_primary: inst.get_bool("FirstInteractionPrimary").unwrap_or_default(),
            use_helper_location: inst.get_bool("UseHelperLocation").unwrap_or_default(),
            apply_to_entire_skeleton: inst.get_bool("ApplyToEntireSkeleton").unwrap_or_default(),
            long_range_ip: inst.get_bool("longRangeIP").unwrap_or_default(),
            radius: inst.get_f32("Radius").unwrap_or_default(),
            show_highlight: inst.get_bool("ShowHighlight").unwrap_or_default(),
            highlight_on_ray_hit_entity: inst.get_bool("HighlightOnRayHitEntity").unwrap_or_default(),
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            text_justification: inst.get_str("TextJustification").map(String::from).unwrap_or_default(),
            text_orientation: inst.get_str("TextOrientation").map(String::from).unwrap_or_default(),
            text_scale: inst.get_f32("TextScale").unwrap_or_default(),
            show_interaction_text_at_cursor_pos: inst.get_bool("ShowInteractionTextAtCursorPos").unwrap_or_default(),
            scale_prompt_offset_with_entity: inst.get_bool("ScalePromptOffsetWithEntity").unwrap_or_default(),
            interaction_prompt_offset: match inst.get("InteractionPromptOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            prompt_style: inst.get_str("PromptStyle").map(String::from).unwrap_or_default(),
            override_control_hint_description: inst.get_bool("OverrideControlHintDescription").unwrap_or_default(),
            interaction_prompt_scale: inst.get_f32("InteractionPromptScale").unwrap_or_default(),
            interaction_prompt_label_vertical_offset: inst.get_f32("InteractionPromptLabelVerticalOffset").unwrap_or_default(),
            interaction_prompt_bound: inst.get_str("InteractionPromptBound").map(String::from).unwrap_or_default(),
            use_horizontal_distance: inst.get_bool("UseHorizontalDistance").unwrap_or_default(),
            max_cursor_screen_distance: inst.get_f32("MaxCursorScreenDistance").unwrap_or_default(),
            show_inner_thoughts_off_screen: inst.get_bool("ShowInnerThoughtsOffScreen").unwrap_or_default(),
            only_selectable_in_look_dir: inst.get_bool("OnlySelectableInLookDir").unwrap_or_default(),
            angle_constraint: match inst.get("AngleConstraint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAngleConstraint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAngleConstraint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SInteractionPointModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInteractionPointModifier {
    /// DCB field: `conditionParams` (StrongPointer (array))
    #[serde(default)]
    pub condition_params: Vec<Handle<InteractionConditionParams>>,
}

impl Pooled for SInteractionPointModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sinteraction_point_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sinteraction_point_modifier }
}

impl<'a> Extract<'a> for SInteractionPointModifier {
    const TYPE_NAME: &'static str = "SInteractionPointModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_params: inst.get_array("conditionParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SItemPortDefTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemPortDefTypes {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `SubTypes` (EnumChoice (array))
    #[serde(default)]
    pub sub_types: Vec<String>,
}

impl Pooled for SItemPortDefTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sitem_port_def_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sitem_port_def_types }
}

impl<'a> Extract<'a> for SItemPortDefTypes {
    const TYPE_NAME: &'static str = "SItemPortDefTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            sub_types: inst.get_array("SubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SignatureSystemGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureSystemGlobalParams {
    /// DCB field: `globalDBFactor` (Single)
    #[serde(default)]
    pub global_dbfactor: f32,
    /// DCB field: `globalAmbientIRFactor` (Single)
    #[serde(default)]
    pub global_ambient_irfactor: f32,
    /// DCB field: `dBSignaturePeakHoldTime` (Single)
    #[serde(default)]
    pub d_bsignature_peak_hold_time: f32,
    /// DCB field: `signatureUIParams` (Class)
    #[serde(default)]
    pub signature_uiparams: Option<Handle<SignatureUIGlobalParams>>,
    /// DCB field: `actorMultiplierParams` (Class)
    #[serde(default)]
    pub actor_multiplier_params: Option<Handle<ActorSignatureMultiplierGlobalParams>>,
    /// DCB field: `crossSectionParams` (Class)
    #[serde(default)]
    pub cross_section_params: Option<Handle<CrossSectionGlobalParams>>,
    /// DCB field: `signatureTypeParams` (Class)
    #[serde(default)]
    pub signature_type_params: Option<Handle<SignatureTypeGlobalParams>>,
    /// DCB field: `masterModeDeltaSignatureType` (Class)
    #[serde(default)]
    pub master_mode_delta_signature_type: Option<Handle<MasterModeSwitchDeltaSignatureTypes>>,
    /// DCB field: `scanWaveTriggeredDeltaSignatureType` (Reference)
    #[serde(default)]
    pub scan_wave_triggered_delta_signature_type: Option<CigGuid>,
}

impl Pooled for SignatureSystemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.signature_system_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.signature_system_global_params }
}

impl<'a> Extract<'a> for SignatureSystemGlobalParams {
    const TYPE_NAME: &'static str = "SignatureSystemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_dbfactor: inst.get_f32("globalDBFactor").unwrap_or_default(),
            global_ambient_irfactor: inst.get_f32("globalAmbientIRFactor").unwrap_or_default(),
            d_bsignature_peak_hold_time: inst.get_f32("dBSignaturePeakHoldTime").unwrap_or_default(),
            signature_uiparams: match inst.get("signatureUIParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureUIGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureUIGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_multiplier_params: match inst.get("actorMultiplierParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSignatureMultiplierGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorSignatureMultiplierGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cross_section_params: match inst.get("crossSectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CrossSectionGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CrossSectionGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            signature_type_params: match inst.get("signatureTypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureTypeGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureTypeGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            master_mode_delta_signature_type: match inst.get("masterModeDeltaSignatureType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MasterModeSwitchDeltaSignatureTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MasterModeSwitchDeltaSignatureTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_wave_triggered_delta_signature_type: inst.get("scanWaveTriggeredDeltaSignatureType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SignatureUIGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureUIGlobalParams {
    /// DCB field: `warningUnderStateTime` (Single)
    #[serde(default)]
    pub warning_under_state_time: f32,
    /// DCB field: `signatureDisplayTime` (Single)
    #[serde(default)]
    pub signature_display_time: f32,
    /// DCB field: `signatureFadeTime` (Single)
    #[serde(default)]
    pub signature_fade_time: f32,
    /// DCB field: `emissionDisplayIncrease` (Single)
    #[serde(default)]
    pub emission_display_increase: f32,
    /// DCB field: `emissionMemoryTime` (Single)
    #[serde(default)]
    pub emission_memory_time: f32,
}

impl Pooled for SignatureUIGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.signature_uiglobal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.signature_uiglobal_params }
}

impl<'a> Extract<'a> for SignatureUIGlobalParams {
    const TYPE_NAME: &'static str = "SignatureUIGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            warning_under_state_time: inst.get_f32("warningUnderStateTime").unwrap_or_default(),
            signature_display_time: inst.get_f32("signatureDisplayTime").unwrap_or_default(),
            signature_fade_time: inst.get_f32("signatureFadeTime").unwrap_or_default(),
            emission_display_increase: inst.get_f32("emissionDisplayIncrease").unwrap_or_default(),
            emission_memory_time: inst.get_f32("emissionMemoryTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SignatureTypeGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureTypeGlobalParams {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `allowDampening` (Boolean)
    #[serde(default)]
    pub allow_dampening: bool,
    /// DCB field: `allowGenerateUnknownContacts` (Boolean)
    #[serde(default)]
    pub allow_generate_unknown_contacts: bool,
    /// DCB field: `allowVisibleContacts` (Boolean)
    #[serde(default)]
    pub allow_visible_contacts: bool,
    /// DCB field: `allowGenerateBlobs` (Boolean)
    #[serde(default)]
    pub allow_generate_blobs: bool,
    /// DCB field: `nearbyAmbientMultiplier` (Single)
    #[serde(default)]
    pub nearby_ambient_multiplier: f32,
}

impl Pooled for SignatureTypeGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.signature_type_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.signature_type_global_params }
}

impl<'a> Extract<'a> for SignatureTypeGlobalParams {
    const TYPE_NAME: &'static str = "SignatureTypeGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            allow_dampening: inst.get_bool("allowDampening").unwrap_or_default(),
            allow_generate_unknown_contacts: inst.get_bool("allowGenerateUnknownContacts").unwrap_or_default(),
            allow_visible_contacts: inst.get_bool("allowVisibleContacts").unwrap_or_default(),
            allow_generate_blobs: inst.get_bool("allowGenerateBlobs").unwrap_or_default(),
            nearby_ambient_multiplier: inst.get_f32("nearbyAmbientMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SItemShopReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemShopReference {
    /// DCB field: `reference` (EnumChoice)
    #[serde(default)]
    pub reference: String,
    /// DCB field: `adjustmentMode` (EnumChoice)
    #[serde(default)]
    pub adjustment_mode: String,
    /// DCB field: `adjustmentValue` (Class)
    #[serde(default)]
    pub adjustment_value: Option<Handle<Vec3>>,
}

impl Pooled for SItemShopReference {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sitem_shop_reference }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sitem_shop_reference }
}

impl<'a> Extract<'a> for SItemShopReference {
    const TYPE_NAME: &'static str = "SItemShopReference";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reference: inst.get_str("reference").map(String::from).unwrap_or_default(),
            adjustment_mode: inst.get_str("adjustmentMode").map(String::from).unwrap_or_default(),
            adjustment_value: match inst.get("adjustmentValue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SItemShopARParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemShopARParams {
    /// DCB field: `showOnCenter` (Boolean)
    #[serde(default)]
    pub show_on_center: bool,
    /// DCB field: `distancePositionUpdate` (Single)
    #[serde(default)]
    pub distance_position_update: f32,
    /// DCB field: `updateOrientationEveryFrame` (Boolean)
    #[serde(default)]
    pub update_orientation_every_frame: bool,
    /// DCB field: `silhouetteWithRack` (Boolean)
    #[serde(default)]
    pub silhouette_with_rack: bool,
    /// DCB field: `bounds` (Class)
    #[serde(default)]
    pub bounds: Option<Handle<SItemShopReference>>,
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<SItemShopReference>>,
    /// DCB field: `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<SItemShopReference>>,
    /// DCB field: `faceMinX` (Boolean)
    #[serde(default)]
    pub face_min_x: bool,
    /// DCB field: `faceMaxX` (Boolean)
    #[serde(default)]
    pub face_max_x: bool,
    /// DCB field: `faceMinY` (Boolean)
    #[serde(default)]
    pub face_min_y: bool,
    /// DCB field: `faceMaxY` (Boolean)
    #[serde(default)]
    pub face_max_y: bool,
    /// DCB field: `faceMinZ` (Boolean)
    #[serde(default)]
    pub face_min_z: bool,
    /// DCB field: `faceMaxZ` (Boolean)
    #[serde(default)]
    pub face_max_z: bool,
}

impl Pooled for SItemShopARParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.sitem_shop_arparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.sitem_shop_arparams }
}

impl<'a> Extract<'a> for SItemShopARParams {
    const TYPE_NAME: &'static str = "SItemShopARParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            show_on_center: inst.get_bool("showOnCenter").unwrap_or_default(),
            distance_position_update: inst.get_f32("distancePositionUpdate").unwrap_or_default(),
            update_orientation_every_frame: inst.get_bool("updateOrientationEveryFrame").unwrap_or_default(),
            silhouette_with_rack: inst.get_bool("silhouetteWithRack").unwrap_or_default(),
            bounds: match inst.get("bounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            face_min_x: inst.get_bool("faceMinX").unwrap_or_default(),
            face_max_x: inst.get_bool("faceMaxX").unwrap_or_default(),
            face_min_y: inst.get_bool("faceMinY").unwrap_or_default(),
            face_max_y: inst.get_bool("faceMaxY").unwrap_or_default(),
            face_min_z: inst.get_bool("faceMinZ").unwrap_or_default(),
            face_max_z: inst.get_bool("faceMaxZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `SimpleSpriteSheet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleSpriteSheet {
    /// DCB field: `textureName` (String)
    #[serde(default)]
    pub texture_name: String,
    /// DCB field: `columnCount` (Byte)
    #[serde(default)]
    pub column_count: u32,
    /// DCB field: `rowCount` (Byte)
    #[serde(default)]
    pub row_count: u32,
}

impl Pooled for SimpleSpriteSheet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.simple_sprite_sheet }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.simple_sprite_sheet }
}

impl<'a> Extract<'a> for SimpleSpriteSheet {
    const TYPE_NAME: &'static str = "SimpleSpriteSheet";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            texture_name: inst.get_str("textureName").map(String::from).unwrap_or_default(),
            column_count: inst.get_u32("columnCount").unwrap_or_default(),
            row_count: inst.get_u32("rowCount").unwrap_or_default(),
        }
    }
}

/// DCB type: `SimpleSpriteSlot`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleSpriteSlot {
    /// DCB field: `sheet` (Reference)
    #[serde(default)]
    pub sheet: Option<CigGuid>,
    /// DCB field: `column` (Byte)
    #[serde(default)]
    pub column: u32,
    /// DCB field: `row` (Byte)
    #[serde(default)]
    pub row: u32,
    /// DCB field: `flipU` (Boolean)
    #[serde(default)]
    pub flip_u: bool,
    /// DCB field: `flipV` (Boolean)
    #[serde(default)]
    pub flip_v: bool,
}

impl Pooled for SimpleSpriteSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_si.simple_sprite_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_si.simple_sprite_slot }
}

impl<'a> Extract<'a> for SimpleSpriteSlot {
    const TYPE_NAME: &'static str = "SimpleSpriteSlot";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sheet: inst.get("sheet").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            column: inst.get_u32("column").unwrap_or_default(),
            row: inst.get_u32("row").unwrap_or_default(),
            flip_u: inst.get_bool("flipU").unwrap_or_default(),
            flip_v: inst.get_bool("flipV").unwrap_or_default(),
        }
    }
}

