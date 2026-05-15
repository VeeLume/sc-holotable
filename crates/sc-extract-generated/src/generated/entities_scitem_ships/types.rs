// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-ships`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ItemModifierTimedLife`
/// Inherits from: `ItemModifierLifetime`
pub struct ItemModifierTimedLife {
    /// `lifetime` (Single)
    pub lifetime: f32,
}

impl Pooled for ItemModifierTimedLife {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.item_modifier_timed_life
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.item_modifier_timed_life
    }
}

impl<'a> Extract<'a> for ItemModifierTimedLife {
    const TYPE_NAME: &'static str = "ItemModifierTimedLife";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lifetime: inst.get_f32("lifetime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemMiningBoosterParams`
/// Inherits from: `LifetimeControlledItemModifierParams`
pub struct ItemMiningBoosterParams {
    /// `modifierLifetime` (StrongPointer)
    pub modifier_lifetime: Option<ItemModifierLifetimePtr>,
    /// `powerLevelChange` (Single)
    pub power_level_change: f32,
    /// `showInUI` (Boolean)
    pub show_in_ui: bool,
    /// `isGood` (Boolean)
    pub is_good: bool,
}

impl Pooled for ItemMiningBoosterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.item_mining_booster_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.item_mining_booster_params
    }
}

impl<'a> Extract<'a> for ItemMiningBoosterParams {
    const TYPE_NAME: &'static str = "ItemMiningBoosterParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modifier_lifetime: match inst.get("modifierLifetime") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ItemModifierLifetimePtr::from_ref(b, r))
                }
                _ => None,
            },
            power_level_change: inst.get_f32("powerLevelChange").unwrap_or_default(),
            show_in_ui: inst.get_bool("showInUI").unwrap_or_default(),
            is_good: inst.get_bool("isGood").unwrap_or_default(),
        }
    }
}

/// DCB type: `SMisfireFunctionalityCondition`
/// Inherits from: `SMisfireCondition`
pub struct SMisfireFunctionalityCondition {
    /// `functionalityMin` (Single)
    pub functionality_min: f32,
    /// `minTimeForTrigger` (Single)
    pub min_time_for_trigger: f32,
    /// `meanTimeForCondition` (Single)
    pub mean_time_for_condition: f32,
    /// `misfireArray` (WeakPointer (array))
    pub misfire_array: Vec<SMisfireEffectPtr>,
}

impl Pooled for SMisfireFunctionalityCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.smisfire_functionality_condition
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.smisfire_functionality_condition
    }
}

impl<'a> Extract<'a> for SMisfireFunctionalityCondition {
    const TYPE_NAME: &'static str = "SMisfireFunctionalityCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            functionality_min: inst.get_f32("functionalityMin").unwrap_or_default(),
            min_time_for_trigger: inst.get_f32("minTimeForTrigger").unwrap_or_default(),
            mean_time_for_condition: inst.get_f32("meanTimeForCondition").unwrap_or_default(),
            misfire_array: inst
                .get_array("misfireArray")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(SMisfireEffectPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifierNumber`
pub struct SIFCSModifierNumber {
    /// `value` (Single)
    pub value: f32,
    /// `type` (EnumChoice)
    pub r#type: EIFCSModifierType,
}

impl Pooled for SIFCSModifierNumber {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.sifcsmodifier_number
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.sifcsmodifier_number
    }
}

impl<'a> Extract<'a> for SIFCSModifierNumber {
    const TYPE_NAME: &'static str = "SIFCSModifierNumber";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_f32("value").unwrap_or_default(),
            r#type: EIFCSModifierType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

/// DCB type: `SIFCSModifierVector`
pub struct SIFCSModifierVector {
    /// `value` (Class)
    pub value: Option<Handle<Vec3>>,
    /// `type` (EnumChoice)
    pub r#type: EIFCSModifierType,
}

impl Pooled for SIFCSModifierVector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.sifcsmodifier_vector
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.sifcsmodifier_vector
    }
}

impl<'a> Extract<'a> for SIFCSModifierVector {
    const TYPE_NAME: &'static str = "SIFCSModifierVector";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            r#type: EIFCSModifierType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

/// DCB type: `SIFCSModifiersLegacy`
pub struct SIFCSModifiersLegacy {
    /// `numbers` (Class (array))
    pub numbers: Vec<Handle<SIFCSModifierNumber>>,
    /// `vectors` (Class (array))
    pub vectors: Vec<Handle<SIFCSModifierVector>>,
}

impl Pooled for SIFCSModifiersLegacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.sifcsmodifiers_legacy
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.sifcsmodifiers_legacy
    }
}

impl<'a> Extract<'a> for SIFCSModifiersLegacy {
    const TYPE_NAME: &'static str = "SIFCSModifiersLegacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            numbers: inst
                .get_array("numbers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SIFCSModifierNumber>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SIFCSModifierNumber>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            vectors: inst
                .get_array("vectors")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SIFCSModifierVector>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SIFCSModifierVector>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SelfDestructStateModifier`
/// Inherits from: `AttachableStateModifierBase`
pub struct SelfDestructStateModifier {
    /// `context` (StrongPointer)
    pub context: Option<AttachableStateModifierContextBasePtr>,
    /// `offlineState` (WeakPointer)
    pub offline_state: Option<Handle<SInteractionState>>,
    /// `primedState` (WeakPointer)
    pub primed_state: Option<Handle<SInteractionState>>,
    /// `idleState` (WeakPointer)
    pub idle_state: Option<Handle<SInteractionState>>,
    /// `detonatedState` (WeakPointer)
    pub detonated_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SelfDestructStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.self_destruct_state_modifier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.self_destruct_state_modifier
    }
}

impl<'a> Extract<'a> for SelfDestructStateModifier {
    const TYPE_NAME: &'static str = "SelfDestructStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            context: match inst.get("context") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(AttachableStateModifierContextBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            offline_state: match inst.get("offlineState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            primed_state: match inst.get("primedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            idle_state: match inst.get("idleState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            detonated_state: match inst.get("detonatedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemEMPParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCItemEMPParams {
    /// `chargeTime` (Single)
    pub charge_time: f32,
    /// `distortionDamage` (Single)
    pub distortion_damage: f32,
    /// `empRadius` (Single)
    pub emp_radius: f32,
    /// `minEmpRadius` (Single)
    pub min_emp_radius: f32,
    /// `physRadius` (Single)
    pub phys_radius: f32,
    /// `minPhysRadius` (Single)
    pub min_phys_radius: f32,
    /// `pressure` (Single)
    pub pressure: f32,
    /// `unleashTime` (Single)
    pub unleash_time: f32,
    /// `cooldownTime` (Single)
    pub cooldown_time: f32,
    /// `ChargingParticle` (Class)
    pub charging_particle: Option<Handle<GlobalResourceParticle>>,
    /// `ChargedParticle` (Class)
    pub charged_particle: Option<Handle<GlobalResourceParticle>>,
    /// `ChargingTag` (Reference)
    pub charging_tag: Option<CigGuid>,
    /// `ChargedTag` (Reference)
    pub charged_tag: Option<CigGuid>,
    /// `StartChargingTrigger` (Reference)
    pub start_charging_trigger: Option<CigGuid>,
    /// `StopChargingTrigger` (Reference)
    pub stop_charging_trigger: Option<CigGuid>,
    /// `StartChargedTrigger` (Reference)
    pub start_charged_trigger: Option<CigGuid>,
    /// `StopChargedTrigger` (Reference)
    pub stop_charged_trigger: Option<CigGuid>,
    /// `StartUnleashTrigger` (Reference)
    pub start_unleash_trigger: Option<CigGuid>,
    /// `StopUnleashTrigger` (Reference)
    pub stop_unleash_trigger: Option<CigGuid>,
    /// `idleState` (WeakPointer)
    pub idle_state: Option<Handle<ItemResourceState>>,
    /// `chargingState` (WeakPointer)
    pub charging_state: Option<Handle<ItemResourceState>>,
    /// `chargedState` (WeakPointer)
    pub charged_state: Option<Handle<ItemResourceState>>,
    /// `releasingState` (WeakPointer)
    pub releasing_state: Option<Handle<ItemResourceState>>,
}

impl Pooled for SCItemEMPParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.scitem_empparams
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.scitem_empparams
    }
}

impl<'a> Extract<'a> for SCItemEMPParams {
    const TYPE_NAME: &'static str = "SCItemEMPParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            charge_time: inst.get_f32("chargeTime").unwrap_or_default(),
            distortion_damage: inst.get_f32("distortionDamage").unwrap_or_default(),
            emp_radius: inst.get_f32("empRadius").unwrap_or_default(),
            min_emp_radius: inst.get_f32("minEmpRadius").unwrap_or_default(),
            phys_radius: inst.get_f32("physRadius").unwrap_or_default(),
            min_phys_radius: inst.get_f32("minPhysRadius").unwrap_or_default(),
            pressure: inst.get_f32("pressure").unwrap_or_default(),
            unleash_time: inst.get_f32("unleashTime").unwrap_or_default(),
            cooldown_time: inst.get_f32("cooldownTime").unwrap_or_default(),
            charging_particle: match inst.get("ChargingParticle") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            charged_particle: match inst.get("ChargedParticle") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            charging_tag: inst
                .get("ChargingTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            charged_tag: inst
                .get("ChargedTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            start_charging_trigger: inst
                .get("StartChargingTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            stop_charging_trigger: inst
                .get("StopChargingTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            start_charged_trigger: inst
                .get("StartChargedTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            stop_charged_trigger: inst
                .get("StopChargedTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            start_unleash_trigger: inst
                .get("StartUnleashTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            stop_unleash_trigger: inst
                .get("StopUnleashTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            idle_state: match inst.get("idleState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ItemResourceState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            charging_state: match inst.get("chargingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ItemResourceState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            charged_state: match inst.get("chargedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ItemResourceState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            releasing_state: match inst.get("releasingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ItemResourceState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemSpaceMineParams`
/// Inherits from: `SExplosiveOrdnanceParams`
pub struct SCItemSpaceMineParams {
    /// `audioParams` (Class)
    pub audio_params: Option<Handle<SOrdnanceAudioParams>>,
    /// `requiresLauncher` (Boolean)
    pub requires_launcher: bool,
    /// `enableLifetime` (Boolean)
    pub enable_lifetime: bool,
    /// `maxLifetime` (Single)
    pub max_lifetime: f32,
    /// `armTime` (Single)
    pub arm_time: f32,
    /// `maxArmableOverride` (Int32)
    pub max_armable_override: i32,
    /// `igniteTime` (Single)
    pub ignite_time: f32,
    /// `collisionDelayTime` (Single)
    pub collision_delay_time: f32,
    /// `explosionSafetyDistance` (Single)
    pub explosion_safety_distance: f32,
    /// `projectileProximity` (Single)
    pub projectile_proximity: f32,
    /// `explosionParams` (Class)
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// `clusterParams` (StrongPointer)
    pub cluster_params: Option<Handle<SOrdnanceClusterParams>>,
    /// `emissionsParams` (Class (array))
    pub emissions_params: Vec<Handle<SOrdnanceEmissionsParams>>,
}

impl Pooled for SCItemSpaceMineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.scitem_space_mine_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.scitem_space_mine_params
    }
}

impl<'a> Extract<'a> for SCItemSpaceMineParams {
    const TYPE_NAME: &'static str = "SCItemSpaceMineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SOrdnanceAudioParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            requires_launcher: inst.get_bool("requiresLauncher").unwrap_or_default(),
            enable_lifetime: inst.get_bool("enableLifetime").unwrap_or_default(),
            max_lifetime: inst.get_f32("maxLifetime").unwrap_or_default(),
            arm_time: inst.get_f32("armTime").unwrap_or_default(),
            max_armable_override: inst.get_i32("maxArmableOverride").unwrap_or_default(),
            ignite_time: inst.get_f32("igniteTime").unwrap_or_default(),
            collision_delay_time: inst.get_f32("collisionDelayTime").unwrap_or_default(),
            explosion_safety_distance: inst.get_f32("explosionSafetyDistance").unwrap_or_default(),
            projectile_proximity: inst.get_f32("projectileProximity").unwrap_or_default(),
            explosion_params: match inst.get("explosionParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ExplosionParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            cluster_params: match inst.get("clusterParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SOrdnanceClusterParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            emissions_params: inst
                .get_array("emissionsParams")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SOrdnanceEmissionsParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SOrdnanceEmissionsParams>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDParamsDiagnostics`
pub struct SMFDParamsDiagnostics {
    /// `healthThresholdDamaged` (Single)
    pub health_threshold_damaged: f32,
    /// `healthThresholdCritical` (Single)
    pub health_threshold_critical: f32,
    /// `excludedItemTypes` (Class (array))
    pub excluded_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `includedItemTypes` (Class (array))
    pub included_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `typeIcons` (String (array))
    pub type_icons: Vec<String>,
}

impl Pooled for SMFDParamsDiagnostics {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.smfdparams_diagnostics
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.smfdparams_diagnostics
    }
}

impl<'a> Extract<'a> for SMFDParamsDiagnostics {
    const TYPE_NAME: &'static str = "SMFDParamsDiagnostics";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            health_threshold_damaged: inst.get_f32("healthThresholdDamaged").unwrap_or_default(),
            health_threshold_critical: inst.get_f32("healthThresholdCritical").unwrap_or_default(),
            excluded_item_types: inst
                .get_array("excludedItemTypes")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SItemPortDefTypes>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemPortDefTypes>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            included_item_types: inst
                .get_array("includedItemTypes")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SItemPortDefTypes>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemPortDefTypes>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            type_icons: inst
                .get_array("typeIcons")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SDummyLauncher`
/// Inherits from: `SLauncherBase`
pub struct SDummyLauncher {}

impl Pooled for SDummyLauncher {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_ships.sdummy_launcher
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_ships.sdummy_launcher
    }
}

impl<'a> Extract<'a> for SDummyLauncher {
    const TYPE_NAME: &'static str = "SDummyLauncher";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
