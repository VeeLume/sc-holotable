// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-weapons`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SSensorMineProximityTrigger`
/// Inherits from: `SSensorMineTriggerType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSensorMineProximityTrigger {
    /// `MaxRaysPerMine` (Int32)
    #[serde(default)]
    pub max_rays_per_mine: i32,
    /// `Radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `WarningRadius` (Single)
    #[serde(default)]
    pub warning_radius: f32,
    /// `AngleVertical` (Single)
    #[serde(default)]
    pub angle_vertical: f32,
    /// `AngleHorizontal` (Single)
    #[serde(default)]
    pub angle_horizontal: f32,
    /// `GuideLaserLength` (Single)
    #[serde(default)]
    pub guide_laser_length: f32,
    /// `Front` (Class)
    #[serde(default)]
    pub front: Option<Handle<Vec3>>,
    /// `Up` (Class)
    #[serde(default)]
    pub up: Option<Handle<Vec3>>,
    /// `Center` (Class)
    #[serde(default)]
    pub center: Option<Handle<Vec3>>,
}

impl Pooled for SSensorMineProximityTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.ssensor_mine_proximity_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.ssensor_mine_proximity_trigger }
}

impl<'a> Extract<'a> for SSensorMineProximityTrigger {
    const TYPE_NAME: &'static str = "SSensorMineProximityTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_rays_per_mine: inst.get_i32("MaxRaysPerMine").unwrap_or_default(),
            radius: inst.get_f32("Radius").unwrap_or_default(),
            warning_radius: inst.get_f32("WarningRadius").unwrap_or_default(),
            angle_vertical: inst.get_f32("AngleVertical").unwrap_or_default(),
            angle_horizontal: inst.get_f32("AngleHorizontal").unwrap_or_default(),
            guide_laser_length: inst.get_f32("GuideLaserLength").unwrap_or_default(),
            front: match inst.get("Front") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            up: match inst.get("Up") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            center: match inst.get("Center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGalactapediaUnlockableComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGalactapediaUnlockableComponentParams {
    /// `unlockedByScan` (Boolean)
    #[serde(default)]
    pub unlocked_by_scan: bool,
    /// `unlockedByInteraction` (Boolean)
    #[serde(default)]
    pub unlocked_by_interaction: bool,
    /// `unlockedByPickingUp` (Boolean)
    #[serde(default)]
    pub unlocked_by_picking_up: bool,
    /// `entriesToUnlock` (Reference (array))
    #[serde(default)]
    pub entries_to_unlock: Vec<CigGuid>,
}

impl Pooled for SGalactapediaUnlockableComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sgalactapedia_unlockable_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sgalactapedia_unlockable_component_params }
}

impl<'a> Extract<'a> for SGalactapediaUnlockableComponentParams {
    const TYPE_NAME: &'static str = "SGalactapediaUnlockableComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            unlocked_by_scan: inst.get_bool("unlockedByScan").unwrap_or_default(),
            unlocked_by_interaction: inst.get_bool("unlockedByInteraction").unwrap_or_default(),
            unlocked_by_picking_up: inst.get_bool("unlockedByPickingUp").unwrap_or_default(),
            entries_to_unlock: inst.get_array("entriesToUnlock")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SChangeHoloEntityStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChangeHoloEntityStateModifier {
    /// `holoEntityState` (WeakPointer)
    #[serde(default)]
    pub holo_entity_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SChangeHoloEntityStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.schange_holo_entity_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.schange_holo_entity_state_modifier }
}

impl<'a> Extract<'a> for SChangeHoloEntityStateModifier {
    const TYPE_NAME: &'static str = "SChangeHoloEntityStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            holo_entity_state: match inst.get("holoEntityState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponAIBeamParams`
/// Inherits from: `SWeaponActionAIParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponAIBeamParams {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `aiShootingMode` (EnumChoice)
    #[serde(default)]
    pub ai_shooting_mode: EAIWeaponShootingMode,
    /// `duration` (Class)
    #[serde(default)]
    pub duration: Option<Handle<Range>>,
    /// `cooldown` (Class)
    #[serde(default)]
    pub cooldown: Option<Handle<Range>>,
    /// `minBullets` (Int32)
    #[serde(default)]
    pub min_bullets: i32,
}

impl Pooled for SWeaponAIBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sweapon_aibeam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sweapon_aibeam_params }
}

impl<'a> Extract<'a> for SWeaponAIBeamParams {
    const TYPE_NAME: &'static str = "SWeaponAIBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            ai_shooting_mode: EAIWeaponShootingMode::from_dcb_str(inst.get_str("aiShootingMode").unwrap_or("")),
            duration: match inst.get("duration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cooldown: match inst.get("cooldown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            min_bullets: inst.get_i32("minBullets").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponConditionTargetingFire`
/// Inherits from: `SWeaponConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponConditionTargetingFire {
    /// `comparer` (Boolean)
    #[serde(default)]
    pub comparer: bool,
}

impl Pooled for SWeaponConditionTargetingFire {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sweapon_condition_targeting_fire }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sweapon_condition_targeting_fire }
}

impl<'a> Extract<'a> for SWeaponConditionTargetingFire {
    const TYPE_NAME: &'static str = "SWeaponConditionTargetingFire";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            comparer: inst.get_bool("comparer").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponActionFireThrustParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponActionFireThrustParams {
    /// `positiveAccelerationLimit` (Single)
    #[serde(default)]
    pub positive_acceleration_limit: f32,
    /// `negativeAccelerationLimit` (Single)
    #[serde(default)]
    pub negative_acceleration_limit: f32,
    /// `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
}

impl Pooled for SWeaponActionFireThrustParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sweapon_action_fire_thrust_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sweapon_action_fire_thrust_params }
}

impl<'a> Extract<'a> for SWeaponActionFireThrustParams {
    const TYPE_NAME: &'static str = "SWeaponActionFireThrustParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            positive_acceleration_limit: inst.get_f32("positiveAccelerationLimit").unwrap_or_default(),
            negative_acceleration_limit: inst.get_f32("negativeAccelerationLimit").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SChargeDrainPrimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChargeDrainPrimeParams {
    /// `allowPriming` (Boolean)
    #[serde(default)]
    pub allow_priming: bool,
    /// `minPrimeAmount` (Int32)
    #[serde(default)]
    pub min_prime_amount: i32,
    /// `maxPrimeAmount` (Int32)
    #[serde(default)]
    pub max_prime_amount: i32,
    /// `minToMaxChargeTime` (Single)
    #[serde(default)]
    pub min_to_max_charge_time: f32,
    /// `resourceRatePrimeScalar` (Single)
    #[serde(default)]
    pub resource_rate_prime_scalar: f32,
    /// `unstablePrimingModifier` (Single)
    #[serde(default)]
    pub unstable_priming_modifier: f32,
    /// `baseStorageTime` (Single)
    #[serde(default)]
    pub base_storage_time: f32,
    /// `additionalStorageTimePerVolt` (Single)
    #[serde(default)]
    pub additional_storage_time_per_volt: f32,
    /// `minigameRadius` (Single)
    #[serde(default)]
    pub minigame_radius: f32,
    /// `secondsToFail` (Single)
    #[serde(default)]
    pub seconds_to_fail: f32,
    /// `recoilInterval` (Single)
    #[serde(default)]
    pub recoil_interval: f32,
    /// `boostingRecoil` (Reference)
    #[serde(default)]
    pub boosting_recoil: Option<CigGuid>,
    /// `voltBoostStateRecoil` (Reference)
    #[serde(default)]
    pub volt_boost_state_recoil: Option<CigGuid>,
    /// `voltBoostUnleashRecoil` (Reference)
    #[serde(default)]
    pub volt_boost_unleash_recoil: Option<CigGuid>,
    /// `voltBoostDumpRecoil` (Reference)
    #[serde(default)]
    pub volt_boost_dump_recoil: Option<CigGuid>,
    /// `damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<DamageBasePtr>,
    /// `hitRadius` (Single)
    #[serde(default)]
    pub hit_radius: f32,
    /// `hitType` (String)
    #[serde(default)]
    pub hit_type: String,
    /// `minChargeLossPercentage` (Single)
    #[serde(default)]
    pub min_charge_loss_percentage: f32,
    /// `maxChargeLossPercentage` (Single)
    #[serde(default)]
    pub max_charge_loss_percentage: f32,
    /// `lowerRandomLossLimitPercentage` (Single)
    #[serde(default)]
    pub lower_random_loss_limit_percentage: f32,
    /// `upperRandomLossLimitPercentage` (Single)
    #[serde(default)]
    pub upper_random_loss_limit_percentage: f32,
    /// `chargeFireActionTag` (Reference)
    #[serde(default)]
    pub charge_fire_action_tag: Option<CigGuid>,
}

impl Pooled for SChargeDrainPrimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.scharge_drain_prime_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.scharge_drain_prime_params }
}

impl<'a> Extract<'a> for SChargeDrainPrimeParams {
    const TYPE_NAME: &'static str = "SChargeDrainPrimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allow_priming: inst.get_bool("allowPriming").unwrap_or_default(),
            min_prime_amount: inst.get_i32("minPrimeAmount").unwrap_or_default(),
            max_prime_amount: inst.get_i32("maxPrimeAmount").unwrap_or_default(),
            min_to_max_charge_time: inst.get_f32("minToMaxChargeTime").unwrap_or_default(),
            resource_rate_prime_scalar: inst.get_f32("resourceRatePrimeScalar").unwrap_or_default(),
            unstable_priming_modifier: inst.get_f32("unstablePrimingModifier").unwrap_or_default(),
            base_storage_time: inst.get_f32("baseStorageTime").unwrap_or_default(),
            additional_storage_time_per_volt: inst.get_f32("additionalStorageTimePerVolt").unwrap_or_default(),
            minigame_radius: inst.get_f32("minigameRadius").unwrap_or_default(),
            seconds_to_fail: inst.get_f32("secondsToFail").unwrap_or_default(),
            recoil_interval: inst.get_f32("recoilInterval").unwrap_or_default(),
            boosting_recoil: inst.get("boostingRecoil").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            volt_boost_state_recoil: inst.get("voltBoostStateRecoil").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            volt_boost_unleash_recoil: inst.get("voltBoostUnleashRecoil").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            volt_boost_dump_recoil: inst.get("voltBoostDumpRecoil").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            damage: match inst.get("damage") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(DamageBasePtr::from_ref(b, r)),
                _ => None,
            },
            hit_radius: inst.get_f32("hitRadius").unwrap_or_default(),
            hit_type: inst.get_str("hitType").map(String::from).unwrap_or_default(),
            min_charge_loss_percentage: inst.get_f32("minChargeLossPercentage").unwrap_or_default(),
            max_charge_loss_percentage: inst.get_f32("maxChargeLossPercentage").unwrap_or_default(),
            lower_random_loss_limit_percentage: inst.get_f32("lowerRandomLossLimitPercentage").unwrap_or_default(),
            upper_random_loss_limit_percentage: inst.get_f32("upperRandomLossLimitPercentage").unwrap_or_default(),
            charge_fire_action_tag: inst.get("chargeFireActionTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SChargeDrainRangeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChargeDrainRangeParams {
    /// `maxBeamDistance` (Single)
    #[serde(default)]
    pub max_beam_distance: f32,
    /// `maxSensorDistance` (Single)
    #[serde(default)]
    pub max_sensor_distance: f32,
    /// `maxVoltBoostDistance` (Single)
    #[serde(default)]
    pub max_volt_boost_distance: f32,
}

impl Pooled for SChargeDrainRangeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.scharge_drain_range_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.scharge_drain_range_params }
}

impl<'a> Extract<'a> for SChargeDrainRangeParams {
    const TYPE_NAME: &'static str = "SChargeDrainRangeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_beam_distance: inst.get_f32("maxBeamDistance").unwrap_or_default(),
            max_sensor_distance: inst.get_f32("maxSensorDistance").unwrap_or_default(),
            max_volt_boost_distance: inst.get_f32("maxVoltBoostDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponActionFireChargeDrainParams`
/// Inherits from: `SWeaponActionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponActionFireChargeDrainParams {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `localisedName` (Locale)
    #[serde(default)]
    pub localised_name: LocaleKey,
    /// `mannequinTag` (Class)
    #[serde(default)]
    pub mannequin_tag: Option<Handle<SMannequinTagParams>>,
    /// `entityTag` (Reference)
    #[serde(default)]
    pub entity_tag: Option<CigGuid>,
    /// `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagList>>,
    /// `uiBindingsTag` (Reference)
    #[serde(default)]
    pub ui_bindings_tag: Option<CigGuid>,
    /// `aiShootingMode` (EnumChoice)
    #[serde(default)]
    pub ai_shooting_mode: EAIWeaponShootingMode,
    /// `switchFireModeAudioTrigger` (Class)
    #[serde(default)]
    pub switch_fire_mode_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `selectableCondition` (StrongPointer)
    #[serde(default)]
    pub selectable_condition: Option<SWeaponConditionBasePtr>,
    /// `hasReloadModesOnUI` (Boolean)
    #[serde(default)]
    pub has_reload_modes_on_ui: bool,
    /// `localisedFunctionalityName` (Locale)
    #[serde(default)]
    pub localised_functionality_name: LocaleKey,
    /// `functionalityTag` (Reference)
    #[serde(default)]
    pub functionality_tag: Option<CigGuid>,
    /// `fireHelper` (String)
    #[serde(default)]
    pub fire_helper: String,
    /// `toggle` (Boolean)
    #[serde(default)]
    pub toggle: bool,
    /// `rangeParams` (Class)
    #[serde(default)]
    pub range_params: Option<Handle<SChargeDrainRangeParams>>,
    /// `maxAimRadius` (Single)
    #[serde(default)]
    pub max_aim_radius: f32,
    /// `resourceRate` (Single)
    #[serde(default)]
    pub resource_rate: f32,
    /// `chargeDrainMode` (EnumChoice)
    #[serde(default)]
    pub charge_drain_mode: EChargeDrainMode,
    /// `ammoType` (EnumChoice)
    #[serde(default)]
    pub ammo_type: EAmmoContainerType,
    /// `wearPerSecond` (Single)
    #[serde(default)]
    pub wear_per_second: f32,
    /// `recoilInterval` (Single)
    #[serde(default)]
    pub recoil_interval: f32,
    /// `recoil` (Reference)
    #[serde(default)]
    pub recoil: Option<CigGuid>,
    /// `primeParams` (Class)
    #[serde(default)]
    pub prime_params: Option<Handle<SChargeDrainPrimeParams>>,
    /// `overloadDuration` (Single)
    #[serde(default)]
    pub overload_duration: f32,
    /// `targetResource` (Reference)
    #[serde(default)]
    pub target_resource: Option<CigGuid>,
    /// `shouldDryFireInGreenZones` (Boolean)
    #[serde(default)]
    pub should_dry_fire_in_green_zones: bool,
    /// `fireFragment` (Class)
    #[serde(default)]
    pub fire_fragment: Option<Handle<SFragmentParams>>,
    /// `stopFireFragment` (Class)
    #[serde(default)]
    pub stop_fire_fragment: Option<Handle<SFragmentParams>>,
    /// `primingFragment` (Class)
    #[serde(default)]
    pub priming_fragment: Option<Handle<SFragmentParams>>,
    /// `primedFragment` (Class)
    #[serde(default)]
    pub primed_fragment: Option<Handle<SFragmentParams>>,
    /// `overloadFragment` (Class)
    #[serde(default)]
    pub overload_fragment: Option<Handle<SFragmentParams>>,
    /// `startFireOneShotAudioTrigger` (Class)
    #[serde(default)]
    pub start_fire_one_shot_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `startFireLoopAudioTrigger` (Class)
    #[serde(default)]
    pub start_fire_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopFireAudioTrigger` (Class)
    #[serde(default)]
    pub stop_fire_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `dryFireAudioTrigger` (Class)
    #[serde(default)]
    pub dry_fire_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `startOverloadAudioTrigger` (Class)
    #[serde(default)]
    pub start_overload_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopOverloadAudioTrigger` (Class)
    #[serde(default)]
    pub stop_overload_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `overloadFireAttemptAudioTrigger` (Class)
    #[serde(default)]
    pub overload_fire_attempt_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `startPrimingLoopAudioTrigger` (Class)
    #[serde(default)]
    pub start_priming_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopPrimingLoopAudioTrigger` (Class)
    #[serde(default)]
    pub stop_priming_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `startPrimingUnstableLoopAudioTrigger` (Class)
    #[serde(default)]
    pub start_priming_unstable_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopPrimingUnstableLoopAudioTrigger` (Class)
    #[serde(default)]
    pub stop_priming_unstable_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `primingUnstableAudioTrigger` (Class)
    #[serde(default)]
    pub priming_unstable_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `primeUnstableAudioRetriggerTime` (Single)
    #[serde(default)]
    pub prime_unstable_audio_retrigger_time: f32,
    /// `startPrimedLoopAudioTrigger` (Class)
    #[serde(default)]
    pub start_primed_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopPrimedLoopAudioTrigger` (Class)
    #[serde(default)]
    pub stop_primed_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `onJumpstartAudioTrigger` (Class)
    #[serde(default)]
    pub on_jumpstart_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `postJumpstartAudioTrigger` (Class)
    #[serde(default)]
    pub post_jumpstart_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `dumpPrimedChargeAudioTrigger` (Class)
    #[serde(default)]
    pub dump_primed_charge_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `weaponBatteryLevelRTPC` (Class)
    #[serde(default)]
    pub weapon_battery_level_rtpc: Option<Handle<AudioRtpc>>,
    /// `weaponPrimedPercentageRTPC` (Class)
    #[serde(default)]
    pub weapon_primed_percentage_rtpc: Option<Handle<AudioRtpc>>,
    /// `fireEffects` (Class (array))
    #[serde(default)]
    pub fire_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `overloadEffects` (Class (array))
    #[serde(default)]
    pub overload_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `overloadFireAttemptEffects` (Class (array))
    #[serde(default)]
    pub overload_fire_attempt_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `primedEffects` (Class (array))
    #[serde(default)]
    pub primed_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `postJumpstartEffects` (Class (array))
    #[serde(default)]
    pub post_jumpstart_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `dumpPrimedChargeEffects` (Class (array))
    #[serde(default)]
    pub dump_primed_charge_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `beamGroup` (StrongPointer)
    #[serde(default)]
    pub beam_group: Option<Handle<SBeamGroupParams>>,
    /// `jumpstartBeamGroup` (StrongPointer)
    #[serde(default)]
    pub jumpstart_beam_group: Option<Handle<SBeamGroupParams>>,
    /// `updateCondition` (StrongPointer)
    #[serde(default)]
    pub update_condition: Option<SWeaponConditionBasePtr>,
    /// `signatureEmitterParams` (StrongPointer)
    #[serde(default)]
    pub signature_emitter_params: Option<Handle<SSCSignatureEmitterParams>>,
}

impl Pooled for SWeaponActionFireChargeDrainParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sweapon_action_fire_charge_drain_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sweapon_action_fire_charge_drain_params }
}

impl<'a> Extract<'a> for SWeaponActionFireChargeDrainParams {
    const TYPE_NAME: &'static str = "SWeaponActionFireChargeDrainParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            localised_name: inst.get_str("localisedName").map(LocaleKey::from).unwrap_or_default(),
            mannequin_tag: match inst.get("mannequinTag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMannequinTagParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entity_tag: inst.get("entityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ui_bindings_tag: inst.get("uiBindingsTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ai_shooting_mode: EAIWeaponShootingMode::from_dcb_str(inst.get_str("aiShootingMode").unwrap_or("")),
            switch_fire_mode_audio_trigger: match inst.get("switchFireModeAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            selectable_condition: match inst.get("selectableCondition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SWeaponConditionBasePtr::from_ref(b, r)),
                _ => None,
            },
            has_reload_modes_on_ui: inst.get_bool("hasReloadModesOnUI").unwrap_or_default(),
            localised_functionality_name: inst.get_str("localisedFunctionalityName").map(LocaleKey::from).unwrap_or_default(),
            functionality_tag: inst.get("functionalityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fire_helper: inst.get_str("fireHelper").map(String::from).unwrap_or_default(),
            toggle: inst.get_bool("toggle").unwrap_or_default(),
            range_params: match inst.get("rangeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainRangeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_aim_radius: inst.get_f32("maxAimRadius").unwrap_or_default(),
            resource_rate: inst.get_f32("resourceRate").unwrap_or_default(),
            charge_drain_mode: EChargeDrainMode::from_dcb_str(inst.get_str("chargeDrainMode").unwrap_or("")),
            ammo_type: EAmmoContainerType::from_dcb_str(inst.get_str("ammoType").unwrap_or("")),
            wear_per_second: inst.get_f32("wearPerSecond").unwrap_or_default(),
            recoil_interval: inst.get_f32("recoilInterval").unwrap_or_default(),
            recoil: inst.get("recoil").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            prime_params: match inst.get("primeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainPrimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            overload_duration: inst.get_f32("overloadDuration").unwrap_or_default(),
            target_resource: inst.get("targetResource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            should_dry_fire_in_green_zones: inst.get_bool("shouldDryFireInGreenZones").unwrap_or_default(),
            fire_fragment: match inst.get("fireFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_fire_fragment: match inst.get("stopFireFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            priming_fragment: match inst.get("primingFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            primed_fragment: match inst.get("primedFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            overload_fragment: match inst.get("overloadFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_fire_one_shot_audio_trigger: match inst.get("startFireOneShotAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_fire_loop_audio_trigger: match inst.get("startFireLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_fire_audio_trigger: match inst.get("stopFireAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dry_fire_audio_trigger: match inst.get("dryFireAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_overload_audio_trigger: match inst.get("startOverloadAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_overload_audio_trigger: match inst.get("stopOverloadAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            overload_fire_attempt_audio_trigger: match inst.get("overloadFireAttemptAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_priming_loop_audio_trigger: match inst.get("startPrimingLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_priming_loop_audio_trigger: match inst.get("stopPrimingLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_priming_unstable_loop_audio_trigger: match inst.get("startPrimingUnstableLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_priming_unstable_loop_audio_trigger: match inst.get("stopPrimingUnstableLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            priming_unstable_audio_trigger: match inst.get("primingUnstableAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            prime_unstable_audio_retrigger_time: inst.get_f32("primeUnstableAudioRetriggerTime").unwrap_or_default(),
            start_primed_loop_audio_trigger: match inst.get("startPrimedLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_primed_loop_audio_trigger: match inst.get("stopPrimedLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            on_jumpstart_audio_trigger: match inst.get("onJumpstartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            post_jumpstart_audio_trigger: match inst.get("postJumpstartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dump_primed_charge_audio_trigger: match inst.get("dumpPrimedChargeAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            weapon_battery_level_rtpc: match inst.get("weaponBatteryLevelRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            weapon_primed_percentage_rtpc: match inst.get("weaponPrimedPercentageRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fire_effects: inst.get_array("fireEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            overload_effects: inst.get_array("overloadEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            overload_fire_attempt_effects: inst.get_array("overloadFireAttemptEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            primed_effects: inst.get_array("primedEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            post_jumpstart_effects: inst.get_array("postJumpstartEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dump_primed_charge_effects: inst.get_array("dumpPrimedChargeEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            beam_group: match inst.get("beamGroup") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBeamGroupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jumpstart_beam_group: match inst.get("jumpstartBeamGroup") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBeamGroupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            update_condition: match inst.get("updateCondition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SWeaponConditionBasePtr::from_ref(b, r)),
                _ => None,
            },
            signature_emitter_params: match inst.get("signatureEmitterParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureEmitterParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SExtinguisherImpactParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SExtinguisherImpactParams {
    /// `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `distanceFromImpactPoint` (Single)
    #[serde(default)]
    pub distance_from_impact_point: f32,
}

impl Pooled for SExtinguisherImpactParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sextinguisher_impact_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sextinguisher_impact_params }
}

impl<'a> Extract<'a> for SExtinguisherImpactParams {
    const TYPE_NAME: &'static str = "SExtinguisherImpactParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            distance_from_impact_point: inst.get_f32("distanceFromImpactPoint").unwrap_or_default(),
        }
    }
}

/// DCB type: `STemperatureReadOutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STemperatureReadOutParams {
    /// `updateIntervalTime` (Single)
    #[serde(default)]
    pub update_interval_time: f32,
    /// `smoothingFactor` (Single)
    #[serde(default)]
    pub smoothing_factor: f32,
    /// `smoothingSamples` (Int32)
    #[serde(default)]
    pub smoothing_samples: i32,
}

impl Pooled for STemperatureReadOutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.stemperature_read_out_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.stemperature_read_out_params }
}

impl<'a> Extract<'a> for STemperatureReadOutParams {
    const TYPE_NAME: &'static str = "STemperatureReadOutParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            update_interval_time: inst.get_f32("updateIntervalTime").unwrap_or_default(),
            smoothing_factor: inst.get_f32("smoothingFactor").unwrap_or_default(),
            smoothing_samples: inst.get_i32("smoothingSamples").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVectorFieldParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVectorFieldParams {
    /// `path` (String)
    #[serde(default)]
    pub path: String,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `width` (Single)
    #[serde(default)]
    pub width: f32,
    /// `falloff` (Single)
    #[serde(default)]
    pub falloff: f32,
    /// `maxStrength` (Single)
    #[serde(default)]
    pub max_strength: f32,
}

impl Pooled for SVectorFieldParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.svector_field_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.svector_field_params }
}

impl<'a> Extract<'a> for SVectorFieldParams {
    const TYPE_NAME: &'static str = "SVectorFieldParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            path: inst.get_str("path").map(String::from).unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            width: inst.get_f32("width").unwrap_or_default(),
            falloff: inst.get_f32("falloff").unwrap_or_default(),
            max_strength: inst.get_f32("maxStrength").unwrap_or_default(),
        }
    }
}

/// DCB type: `SExtinguisherVectorFieldParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SExtinguisherVectorFieldParams {
    /// `main` (Class)
    #[serde(default)]
    pub main: Option<Handle<SVectorFieldParams>>,
    /// `spray` (Class)
    #[serde(default)]
    pub spray: Option<Handle<SVectorFieldParams>>,
}

impl Pooled for SExtinguisherVectorFieldParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sextinguisher_vector_field_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sextinguisher_vector_field_params }
}

impl<'a> Extract<'a> for SExtinguisherVectorFieldParams {
    const TYPE_NAME: &'static str = "SExtinguisherVectorFieldParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            main: match inst.get("main") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVectorFieldParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spray: match inst.get("spray") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVectorFieldParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponActionFireExtinguisherParams`
/// Inherits from: `SWeaponActionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponActionFireExtinguisherParams {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `localisedName` (Locale)
    #[serde(default)]
    pub localised_name: LocaleKey,
    /// `mannequinTag` (Class)
    #[serde(default)]
    pub mannequin_tag: Option<Handle<SMannequinTagParams>>,
    /// `entityTag` (Reference)
    #[serde(default)]
    pub entity_tag: Option<CigGuid>,
    /// `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagList>>,
    /// `uiBindingsTag` (Reference)
    #[serde(default)]
    pub ui_bindings_tag: Option<CigGuid>,
    /// `aiShootingMode` (EnumChoice)
    #[serde(default)]
    pub ai_shooting_mode: EAIWeaponShootingMode,
    /// `switchFireModeAudioTrigger` (Class)
    #[serde(default)]
    pub switch_fire_mode_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `selectableCondition` (StrongPointer)
    #[serde(default)]
    pub selectable_condition: Option<SWeaponConditionBasePtr>,
    /// `hasReloadModesOnUI` (Boolean)
    #[serde(default)]
    pub has_reload_modes_on_ui: bool,
    /// `toggle` (Boolean)
    #[serde(default)]
    pub toggle: bool,
    /// `fireHelper` (String)
    #[serde(default)]
    pub fire_helper: String,
    /// `extinguishingRange` (Single)
    #[serde(default)]
    pub extinguishing_range: f32,
    /// `sensorRange` (Single)
    #[serde(default)]
    pub sensor_range: f32,
    /// `fireDetectionRange` (Single)
    #[serde(default)]
    pub fire_detection_range: f32,
    /// `fireDetectionIntervalTime` (Single)
    #[serde(default)]
    pub fire_detection_interval_time: f32,
    /// `temperatureReadOutParams` (Class)
    #[serde(default)]
    pub temperature_read_out_params: Option<Handle<STemperatureReadOutParams>>,
    /// `extinguishStrengthFalloffDistance` (Single)
    #[serde(default)]
    pub extinguish_strength_falloff_distance: f32,
    /// `coneAngle` (Single)
    #[serde(default)]
    pub cone_angle: f32,
    /// `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// `minRadius` (Single)
    #[serde(default)]
    pub min_radius: f32,
    /// `extinguishStrength` (Single)
    #[serde(default)]
    pub extinguish_strength: f32,
    /// `maxRangeExtinguishStrength` (Single)
    #[serde(default)]
    pub max_range_extinguish_strength: f32,
    /// `ammoPerSecond` (Single)
    #[serde(default)]
    pub ammo_per_second: f32,
    /// `wearPerSecond` (Single)
    #[serde(default)]
    pub wear_per_second: f32,
    /// `roomHelperClass` (Reference)
    #[serde(default)]
    pub room_helper_class: Option<CigGuid>,
    /// `vectorFieldParams` (Class)
    #[serde(default)]
    pub vector_field_params: Option<Handle<SExtinguisherVectorFieldParams>>,
    /// `fireEffects` (Class (array))
    #[serde(default)]
    pub fire_effects: Vec<Handle<SWeaponParticleEffectParams>>,
    /// `impactEffect` (Class)
    #[serde(default)]
    pub impact_effect: Option<Handle<SExtinguisherImpactParams>>,
    /// `fireFragment` (Class)
    #[serde(default)]
    pub fire_fragment: Option<Handle<SFragmentParams>>,
    /// `stopFireFragment` (Class)
    #[serde(default)]
    pub stop_fire_fragment: Option<Handle<SFragmentParams>>,
    /// `startFireOneShotAudioTrigger` (Class)
    #[serde(default)]
    pub start_fire_one_shot_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `startFireLoopAudioTrigger` (Class)
    #[serde(default)]
    pub start_fire_loop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopFireAudioTrigger` (Class)
    #[serde(default)]
    pub stop_fire_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `dryFireAudioTrigger` (Class)
    #[serde(default)]
    pub dry_fire_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `updateCondition` (StrongPointer)
    #[serde(default)]
    pub update_condition: Option<SWeaponConditionBasePtr>,
    /// `aimFireDetectionMode` (EnumChoice)
    #[serde(default)]
    pub aim_fire_detection_mode: EAimFireDetectionMode,
    /// `aimFireDetectionTemperature` (Single)
    #[serde(default)]
    pub aim_fire_detection_temperature: f32,
    /// `aimFireDetectionRadius` (Single)
    #[serde(default)]
    pub aim_fire_detection_radius: f32,
    /// `thrustParams` (Class)
    #[serde(default)]
    pub thrust_params: Option<Handle<SWeaponActionFireThrustParams>>,
}

impl Pooled for SWeaponActionFireExtinguisherParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_weapons.sweapon_action_fire_extinguisher_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_weapons.sweapon_action_fire_extinguisher_params }
}

impl<'a> Extract<'a> for SWeaponActionFireExtinguisherParams {
    const TYPE_NAME: &'static str = "SWeaponActionFireExtinguisherParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            localised_name: inst.get_str("localisedName").map(LocaleKey::from).unwrap_or_default(),
            mannequin_tag: match inst.get("mannequinTag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMannequinTagParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entity_tag: inst.get("entityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ui_bindings_tag: inst.get("uiBindingsTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ai_shooting_mode: EAIWeaponShootingMode::from_dcb_str(inst.get_str("aiShootingMode").unwrap_or("")),
            switch_fire_mode_audio_trigger: match inst.get("switchFireModeAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            selectable_condition: match inst.get("selectableCondition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SWeaponConditionBasePtr::from_ref(b, r)),
                _ => None,
            },
            has_reload_modes_on_ui: inst.get_bool("hasReloadModesOnUI").unwrap_or_default(),
            toggle: inst.get_bool("toggle").unwrap_or_default(),
            fire_helper: inst.get_str("fireHelper").map(String::from).unwrap_or_default(),
            extinguishing_range: inst.get_f32("extinguishingRange").unwrap_or_default(),
            sensor_range: inst.get_f32("sensorRange").unwrap_or_default(),
            fire_detection_range: inst.get_f32("fireDetectionRange").unwrap_or_default(),
            fire_detection_interval_time: inst.get_f32("fireDetectionIntervalTime").unwrap_or_default(),
            temperature_read_out_params: match inst.get("temperatureReadOutParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STemperatureReadOutParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            extinguish_strength_falloff_distance: inst.get_f32("extinguishStrengthFalloffDistance").unwrap_or_default(),
            cone_angle: inst.get_f32("coneAngle").unwrap_or_default(),
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
            min_radius: inst.get_f32("minRadius").unwrap_or_default(),
            extinguish_strength: inst.get_f32("extinguishStrength").unwrap_or_default(),
            max_range_extinguish_strength: inst.get_f32("maxRangeExtinguishStrength").unwrap_or_default(),
            ammo_per_second: inst.get_f32("ammoPerSecond").unwrap_or_default(),
            wear_per_second: inst.get_f32("wearPerSecond").unwrap_or_default(),
            room_helper_class: inst.get("roomHelperClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            vector_field_params: match inst.get("vectorFieldParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SExtinguisherVectorFieldParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fire_effects: inst.get_array("fireEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeaponParticleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeaponParticleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            impact_effect: match inst.get("impactEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SExtinguisherImpactParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fire_fragment: match inst.get("fireFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_fire_fragment: match inst.get("stopFireFragment") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_fire_one_shot_audio_trigger: match inst.get("startFireOneShotAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            start_fire_loop_audio_trigger: match inst.get("startFireLoopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_fire_audio_trigger: match inst.get("stopFireAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dry_fire_audio_trigger: match inst.get("dryFireAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            update_condition: match inst.get("updateCondition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SWeaponConditionBasePtr::from_ref(b, r)),
                _ => None,
            },
            aim_fire_detection_mode: EAimFireDetectionMode::from_dcb_str(inst.get_str("aimFireDetectionMode").unwrap_or("")),
            aim_fire_detection_temperature: inst.get_f32("aimFireDetectionTemperature").unwrap_or_default(),
            aim_fire_detection_radius: inst.get_f32("aimFireDetectionRadius").unwrap_or_default(),
            thrust_params: match inst.get("thrustParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponActionFireThrustParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

