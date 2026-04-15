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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ItemModifierTimedLife`
/// Inherits from: `ItemModifierLifetime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemModifierTimedLife {
    /// `lifetime` (Single)
    #[serde(default)]
    pub lifetime: f32,
}

impl Pooled for ItemModifierTimedLife {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.item_modifier_timed_life }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.item_modifier_timed_life }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMiningBoosterParams {
    /// `modifierLifetime` (StrongPointer)
    #[serde(default)]
    pub modifier_lifetime: Option<ItemModifierLifetimePtr>,
    /// `powerLevelChange` (Single)
    #[serde(default)]
    pub power_level_change: f32,
    /// `showInUI` (Boolean)
    #[serde(default)]
    pub show_in_ui: bool,
    /// `isGood` (Boolean)
    #[serde(default)]
    pub is_good: bool,
}

impl Pooled for ItemMiningBoosterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.item_mining_booster_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.item_mining_booster_params }
}

impl<'a> Extract<'a> for ItemMiningBoosterParams {
    const TYPE_NAME: &'static str = "ItemMiningBoosterParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modifier_lifetime: match inst.get("modifierLifetime") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ItemModifierLifetimePtr::from_ref(b, r)),
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMisfireFunctionalityCondition {
    /// `functionalityMin` (Single)
    #[serde(default)]
    pub functionality_min: f32,
    /// `minTimeForTrigger` (Single)
    #[serde(default)]
    pub min_time_for_trigger: f32,
    /// `meanTimeForCondition` (Single)
    #[serde(default)]
    pub mean_time_for_condition: f32,
    /// `misfireArray` (WeakPointer (array))
    #[serde(default)]
    pub misfire_array: Vec<SMisfireEffectPtr>,
}

impl Pooled for SMisfireFunctionalityCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smisfire_functionality_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smisfire_functionality_condition }
}

impl<'a> Extract<'a> for SMisfireFunctionalityCondition {
    const TYPE_NAME: &'static str = "SMisfireFunctionalityCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            functionality_min: inst.get_f32("functionalityMin").unwrap_or_default(),
            min_time_for_trigger: inst.get_f32("minTimeForTrigger").unwrap_or_default(),
            mean_time_for_condition: inst.get_f32("meanTimeForCondition").unwrap_or_default(),
            misfire_array: inst.get_array("misfireArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SMisfireEffectPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HoverPlane`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverPlane {
    /// `width` (Single)
    #[serde(default)]
    pub width: f32,
    /// `length` (Single)
    #[serde(default)]
    pub length: f32,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
}

impl Pooled for HoverPlane {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.hover_plane }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.hover_plane }
}

impl<'a> Extract<'a> for HoverPlane {
    const TYPE_NAME: &'static str = "HoverPlane";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            width: inst.get_f32("width").unwrap_or_default(),
            length: inst.get_f32("length").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SuspensionSprings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspensionSprings {
    /// `undampedFrequency` (Single)
    #[serde(default)]
    pub undamped_frequency: f32,
    /// `dampingRatio` (Single)
    #[serde(default)]
    pub damping_ratio: f32,
    /// `forceBlendOutDelay` (Single)
    #[serde(default)]
    pub force_blend_out_delay: f32,
    /// `forceBlendInRate` (Single)
    #[serde(default)]
    pub force_blend_in_rate: f32,
    /// `forceBlendOutRate` (Single)
    #[serde(default)]
    pub force_blend_out_rate: f32,
    /// `bumpStop` (Single)
    #[serde(default)]
    pub bump_stop: f32,
    /// `compressionForceCurve` (Class)
    #[serde(default)]
    pub compression_force_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SuspensionSprings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.suspension_springs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.suspension_springs }
}

impl<'a> Extract<'a> for SuspensionSprings {
    const TYPE_NAME: &'static str = "SuspensionSprings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            undamped_frequency: inst.get_f32("undampedFrequency").unwrap_or_default(),
            damping_ratio: inst.get_f32("dampingRatio").unwrap_or_default(),
            force_blend_out_delay: inst.get_f32("forceBlendOutDelay").unwrap_or_default(),
            force_blend_in_rate: inst.get_f32("forceBlendInRate").unwrap_or_default(),
            force_blend_out_rate: inst.get_f32("forceBlendOutRate").unwrap_or_default(),
            bump_stop: inst.get_f32("bumpStop").unwrap_or_default(),
            compression_force_curve: match inst.get("compressionForceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HoverHeight`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverHeight {
    /// `desiredHoverHeight` (Single)
    #[serde(default)]
    pub desired_hover_height: f32,
    /// `hoverHeightOffsetRange` (Class)
    #[serde(default)]
    pub hover_height_offset_range: Option<Handle<Range>>,
    /// `hoverHeightOffsetAcceleration` (Single)
    #[serde(default)]
    pub hover_height_offset_acceleration: f32,
    /// `hoverHeightOffsetMaxSpeed` (Single)
    #[serde(default)]
    pub hover_height_offset_max_speed: f32,
    /// `speedRangeForExtraHeight` (Class)
    #[serde(default)]
    pub speed_range_for_extra_height: Option<Handle<Range>>,
    /// `maxExtraHoverHeight` (Single)
    #[serde(default)]
    pub max_extra_hover_height: f32,
}

impl Pooled for HoverHeight {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.hover_height }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.hover_height }
}

impl<'a> Extract<'a> for HoverHeight {
    const TYPE_NAME: &'static str = "HoverHeight";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            desired_hover_height: inst.get_f32("desiredHoverHeight").unwrap_or_default(),
            hover_height_offset_range: match inst.get("hoverHeightOffsetRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hover_height_offset_acceleration: inst.get_f32("hoverHeightOffsetAcceleration").unwrap_or_default(),
            hover_height_offset_max_speed: inst.get_f32("hoverHeightOffsetMaxSpeed").unwrap_or_default(),
            speed_range_for_extra_height: match inst.get("speedRangeForExtraHeight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_extra_hover_height: inst.get_f32("maxExtraHoverHeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `HoverTilting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverTilting {
    /// `strafeBankFactor` (Single)
    #[serde(default)]
    pub strafe_bank_factor: f32,
    /// `forwardBackTiltFactor` (Single)
    #[serde(default)]
    pub forward_back_tilt_factor: f32,
    /// `turnBankFactor` (Single)
    #[serde(default)]
    pub turn_bank_factor: f32,
    /// `bankPerSpeedCurve` (Class)
    #[serde(default)]
    pub bank_per_speed_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for HoverTilting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.hover_tilting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.hover_tilting }
}

impl<'a> Extract<'a> for HoverTilting {
    const TYPE_NAME: &'static str = "HoverTilting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            strafe_bank_factor: inst.get_f32("strafeBankFactor").unwrap_or_default(),
            forward_back_tilt_factor: inst.get_f32("forwardBackTiltFactor").unwrap_or_default(),
            turn_bank_factor: inst.get_f32("turnBankFactor").unwrap_or_default(),
            bank_per_speed_curve: match inst.get("bankPerSpeedCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `HoverCollisions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverCollisions {
    /// `antiSpinThreshold` (Single)
    #[serde(default)]
    pub anti_spin_threshold: f32,
    /// `linearCollisionDamp` (Single)
    #[serde(default)]
    pub linear_collision_damp: f32,
    /// `angularCollisionDamp` (Single)
    #[serde(default)]
    pub angular_collision_damp: f32,
}

impl Pooled for HoverCollisions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.hover_collisions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.hover_collisions }
}

impl<'a> Extract<'a> for HoverCollisions {
    const TYPE_NAME: &'static str = "HoverCollisions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            anti_spin_threshold: inst.get_f32("antiSpinThreshold").unwrap_or_default(),
            linear_collision_damp: inst.get_f32("linearCollisionDamp").unwrap_or_default(),
            angular_collision_damp: inst.get_f32("angularCollisionDamp").unwrap_or_default(),
        }
    }
}

/// DCB type: `HoverHandling`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverHandling {
    /// `turnFriction` (Single)
    #[serde(default)]
    pub turn_friction: f32,
    /// `selfRightingAccelBoost` (Single)
    #[serde(default)]
    pub self_righting_accel_boost: f32,
    /// `hoverMaxSpeed` (Single)
    #[serde(default)]
    pub hover_max_speed: f32,
    /// `airControlMultiplier` (Single)
    #[serde(default)]
    pub air_control_multiplier: f32,
    /// `antiFallMultiplier` (Single)
    #[serde(default)]
    pub anti_fall_multiplier: f32,
    /// `lateralStrafeMultiplier` (Single)
    #[serde(default)]
    pub lateral_strafe_multiplier: f32,
    /// `maxSpeedMultiplierByHeight` (Class)
    #[serde(default)]
    pub max_speed_multiplier_by_height: Option<Handle<BezierCurve>>,
    /// `turnFrictionMultiplierByHeight` (Class)
    #[serde(default)]
    pub turn_friction_multiplier_by_height: Option<Handle<BezierCurve>>,
}

impl Pooled for HoverHandling {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.hover_handling }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.hover_handling }
}

impl<'a> Extract<'a> for HoverHandling {
    const TYPE_NAME: &'static str = "HoverHandling";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            turn_friction: inst.get_f32("turnFriction").unwrap_or_default(),
            self_righting_accel_boost: inst.get_f32("selfRightingAccelBoost").unwrap_or_default(),
            hover_max_speed: inst.get_f32("hoverMaxSpeed").unwrap_or_default(),
            air_control_multiplier: inst.get_f32("airControlMultiplier").unwrap_or_default(),
            anti_fall_multiplier: inst.get_f32("antiFallMultiplier").unwrap_or_default(),
            lateral_strafe_multiplier: inst.get_f32("lateralStrafeMultiplier").unwrap_or_default(),
            max_speed_multiplier_by_height: match inst.get("maxSpeedMultiplierByHeight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            turn_friction_multiplier_by_height: match inst.get("turnFrictionMultiplierByHeight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GravlevParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravlevParams {
    /// `hoverPlane` (Class)
    #[serde(default)]
    pub hover_plane: Option<Handle<HoverPlane>>,
    /// `springs` (Class)
    #[serde(default)]
    pub springs: Option<Handle<SuspensionSprings>>,
    /// `height` (Class)
    #[serde(default)]
    pub height: Option<Handle<HoverHeight>>,
    /// `tilting` (Class)
    #[serde(default)]
    pub tilting: Option<Handle<HoverTilting>>,
    /// `collisions` (Class)
    #[serde(default)]
    pub collisions: Option<Handle<HoverCollisions>>,
    /// `handling` (Class)
    #[serde(default)]
    pub handling: Option<Handle<HoverHandling>>,
    /// `hoverHeightRtpc` (Class)
    #[serde(default)]
    pub hover_height_rtpc: Option<Handle<AudioRtpc>>,
    /// `hoverHeightDifferentialRtpc` (Class)
    #[serde(default)]
    pub hover_height_differential_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for GravlevParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.gravlev_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.gravlev_params }
}

impl<'a> Extract<'a> for GravlevParams {
    const TYPE_NAME: &'static str = "GravlevParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hover_plane: match inst.get("hoverPlane") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HoverPlane>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            springs: match inst.get("springs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SuspensionSprings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            height: match inst.get("height") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HoverHeight>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tilting: match inst.get("tilting") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HoverTilting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            collisions: match inst.get("collisions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HoverCollisions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            handling: match inst.get("handling") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HoverHandling>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hover_height_rtpc: match inst.get("hoverHeightRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hover_height_differential_rtpc: match inst.get("hoverHeightDifferentialRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SIFCSModifierNumber`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierNumber {
    /// `value` (Single)
    #[serde(default)]
    pub value: f32,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: EIFCSModifierType,
}

impl Pooled for SIFCSModifierNumber {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifier_number }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifier_number }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierVector {
    /// `value` (Class)
    #[serde(default)]
    pub value: Option<Handle<Vec3>>,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: EIFCSModifierType,
}

impl Pooled for SIFCSModifierVector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifier_vector }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifier_vector }
}

impl<'a> Extract<'a> for SIFCSModifierVector {
    const TYPE_NAME: &'static str = "SIFCSModifierVector";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            r#type: EIFCSModifierType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

/// DCB type: `SIFCSModifiersLegacy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifiersLegacy {
    /// `numbers` (Class)
    #[serde(default)]
    pub numbers: Option<Handle<SIFCSModifierNumber>>,
    /// `vectors` (Class)
    #[serde(default)]
    pub vectors: Option<Handle<SIFCSModifierVector>>,
}

impl Pooled for SIFCSModifiersLegacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifiers_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifiers_legacy }
}

impl<'a> Extract<'a> for SIFCSModifiersLegacy {
    const TYPE_NAME: &'static str = "SIFCSModifiersLegacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            numbers: match inst.get("numbers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierNumber>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            vectors: match inst.get("vectors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierVector>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SendDockingEnableEvent`
/// Inherits from: `EventDispatcher`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDockingEnableEvent {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for SendDockingEnableEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.send_docking_enable_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.send_docking_enable_event }
}

impl<'a> Extract<'a> for SendDockingEnableEvent {
    const TYPE_NAME: &'static str = "SendDockingEnableEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `SelfDestructStateModifier`
/// Inherits from: `AttachableStateModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDestructStateModifier {
    /// `context` (StrongPointer)
    #[serde(default)]
    pub context: Option<AttachableStateModifierContextBasePtr>,
    /// `offlineState` (WeakPointer)
    #[serde(default)]
    pub offline_state: Option<Handle<SInteractionState>>,
    /// `primedState` (WeakPointer)
    #[serde(default)]
    pub primed_state: Option<Handle<SInteractionState>>,
    /// `idleState` (WeakPointer)
    #[serde(default)]
    pub idle_state: Option<Handle<SInteractionState>>,
    /// `detonatedState` (WeakPointer)
    #[serde(default)]
    pub detonated_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SelfDestructStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.self_destruct_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.self_destruct_state_modifier }
}

impl<'a> Extract<'a> for SelfDestructStateModifier {
    const TYPE_NAME: &'static str = "SelfDestructStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            context: match inst.get("context") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(AttachableStateModifierContextBasePtr::from_ref(b, r)),
                _ => None,
            },
            offline_state: match inst.get("offlineState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            primed_state: match inst.get("primedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            idle_state: match inst.get("idleState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            detonated_state: match inst.get("detonatedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemResourceDynamicAmountFuelNozzleFuel`
/// Inherits from: `ItemResourceDynamicAmountBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceDynamicAmountFuelNozzleFuel {
}

impl Pooled for ItemResourceDynamicAmountFuelNozzleFuel {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.item_resource_dynamic_amount_fuel_nozzle_fuel }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.item_resource_dynamic_amount_fuel_nozzle_fuel }
}

impl<'a> Extract<'a> for ItemResourceDynamicAmountFuelNozzleFuel {
    const TYPE_NAME: &'static str = "ItemResourceDynamicAmountFuelNozzleFuel";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemEMPParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemEMPParams {
    /// `chargeTime` (Single)
    #[serde(default)]
    pub charge_time: f32,
    /// `distortionDamage` (Single)
    #[serde(default)]
    pub distortion_damage: f32,
    /// `empRadius` (Single)
    #[serde(default)]
    pub emp_radius: f32,
    /// `minEmpRadius` (Single)
    #[serde(default)]
    pub min_emp_radius: f32,
    /// `physRadius` (Single)
    #[serde(default)]
    pub phys_radius: f32,
    /// `minPhysRadius` (Single)
    #[serde(default)]
    pub min_phys_radius: f32,
    /// `pressure` (Single)
    #[serde(default)]
    pub pressure: f32,
    /// `unleashTime` (Single)
    #[serde(default)]
    pub unleash_time: f32,
    /// `cooldownTime` (Single)
    #[serde(default)]
    pub cooldown_time: f32,
    /// `ChargingParticle` (Class)
    #[serde(default)]
    pub charging_particle: Option<Handle<GlobalResourceParticle>>,
    /// `ChargedParticle` (Class)
    #[serde(default)]
    pub charged_particle: Option<Handle<GlobalResourceParticle>>,
    /// `ChargingTag` (Reference)
    #[serde(default)]
    pub charging_tag: Option<CigGuid>,
    /// `ChargedTag` (Reference)
    #[serde(default)]
    pub charged_tag: Option<CigGuid>,
    /// `StartChargingTrigger` (Reference)
    #[serde(default)]
    pub start_charging_trigger: Option<CigGuid>,
    /// `StopChargingTrigger` (Reference)
    #[serde(default)]
    pub stop_charging_trigger: Option<CigGuid>,
    /// `StartChargedTrigger` (Reference)
    #[serde(default)]
    pub start_charged_trigger: Option<CigGuid>,
    /// `StopChargedTrigger` (Reference)
    #[serde(default)]
    pub stop_charged_trigger: Option<CigGuid>,
    /// `StartUnleashTrigger` (Reference)
    #[serde(default)]
    pub start_unleash_trigger: Option<CigGuid>,
    /// `StopUnleashTrigger` (Reference)
    #[serde(default)]
    pub stop_unleash_trigger: Option<CigGuid>,
    /// `idleState` (WeakPointer)
    #[serde(default)]
    pub idle_state: Option<Handle<ItemResourceState>>,
    /// `chargingState` (WeakPointer)
    #[serde(default)]
    pub charging_state: Option<Handle<ItemResourceState>>,
    /// `chargedState` (WeakPointer)
    #[serde(default)]
    pub charged_state: Option<Handle<ItemResourceState>>,
    /// `releasingState` (WeakPointer)
    #[serde(default)]
    pub releasing_state: Option<Handle<ItemResourceState>>,
}

impl Pooled for SCItemEMPParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.scitem_empparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.scitem_empparams }
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
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            charged_particle: match inst.get("ChargedParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            charging_tag: inst.get("ChargingTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            charged_tag: inst.get("ChargedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_charging_trigger: inst.get("StartChargingTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stop_charging_trigger: inst.get("StopChargingTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_charged_trigger: inst.get("StartChargedTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stop_charged_trigger: inst.get("StopChargedTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_unleash_trigger: inst.get("StartUnleashTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stop_unleash_trigger: inst.get("StopUnleashTrigger").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            idle_state: match inst.get("idleState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charging_state: match inst.get("chargingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charged_state: match inst.get("chargedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            releasing_state: match inst.get("releasingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemFuelNozzleParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemFuelNozzleParams {
    /// `fuelMaxFlowMultiplier` (Single)
    #[serde(default)]
    pub fuel_max_flow_multiplier: f32,
    /// `fuelTankCapacityMultiplier` (Single)
    #[serde(default)]
    pub fuel_tank_capacity_multiplier: f32,
    /// `fuelSafeSpeed` (Single)
    #[serde(default)]
    pub fuel_safe_speed: f32,
    /// `fuelSafeSpeedRN` (Single)
    #[serde(default)]
    pub fuel_safe_speed_rn: f32,
    /// `fuelDamageMultiplier` (Single)
    #[serde(default)]
    pub fuel_damage_multiplier: f32,
    /// `waveModifier` (Single)
    #[serde(default)]
    pub wave_modifier: f32,
    /// `waveModifierRN` (Single)
    #[serde(default)]
    pub wave_modifier_rn: f32,
    /// `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// `flowSpeedModifier` (Single)
    #[serde(default)]
    pub flow_speed_modifier: f32,
    /// `fuelPodMultiplier` (Single)
    #[serde(default)]
    pub fuel_pod_multiplier: f32,
    /// `fuelFlowLoopStartAudioTrigger` (Class)
    #[serde(default)]
    pub fuel_flow_loop_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `fuelFlowLoopStopAudioTrigger` (Class)
    #[serde(default)]
    pub fuel_flow_loop_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `fuelFlowRateAudioRtpcs` (Class (array))
    #[serde(default)]
    pub fuel_flow_rate_audio_rtpcs: Vec<Handle<AudioRtpc>>,
    /// `fuelSpillAudioRtpc` (Class)
    #[serde(default)]
    pub fuel_spill_audio_rtpc: Option<Handle<AudioRtpc>>,
    /// `stateToPowerOff` (WeakPointer)
    #[serde(default)]
    pub state_to_power_off: Option<Handle<SInteractionState>>,
}

impl Pooled for SCItemFuelNozzleParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.scitem_fuel_nozzle_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.scitem_fuel_nozzle_params }
}

impl<'a> Extract<'a> for SCItemFuelNozzleParams {
    const TYPE_NAME: &'static str = "SCItemFuelNozzleParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fuel_max_flow_multiplier: inst.get_f32("fuelMaxFlowMultiplier").unwrap_or_default(),
            fuel_tank_capacity_multiplier: inst.get_f32("fuelTankCapacityMultiplier").unwrap_or_default(),
            fuel_safe_speed: inst.get_f32("fuelSafeSpeed").unwrap_or_default(),
            fuel_safe_speed_rn: inst.get_f32("fuelSafeSpeedRN").unwrap_or_default(),
            fuel_damage_multiplier: inst.get_f32("fuelDamageMultiplier").unwrap_or_default(),
            wave_modifier: inst.get_f32("waveModifier").unwrap_or_default(),
            wave_modifier_rn: inst.get_f32("waveModifierRN").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            flow_speed_modifier: inst.get_f32("flowSpeedModifier").unwrap_or_default(),
            fuel_pod_multiplier: inst.get_f32("fuelPodMultiplier").unwrap_or_default(),
            fuel_flow_loop_start_audio_trigger: match inst.get("fuelFlowLoopStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fuel_flow_loop_stop_audio_trigger: match inst.get("fuelFlowLoopStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fuel_flow_rate_audio_rtpcs: inst.get_array("fuelFlowRateAudioRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            fuel_spill_audio_rtpc: match inst.get("fuelSpillAudioRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            state_to_power_off: match inst.get("stateToPowerOff") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemSpaceMineParams`
/// Inherits from: `SExplosiveOrdnanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSpaceMineParams {
    /// `audioParams` (Class)
    #[serde(default)]
    pub audio_params: Option<Handle<SOrdnanceAudioParams>>,
    /// `requiresLauncher` (Boolean)
    #[serde(default)]
    pub requires_launcher: bool,
    /// `enableLifetime` (Boolean)
    #[serde(default)]
    pub enable_lifetime: bool,
    /// `maxLifetime` (Single)
    #[serde(default)]
    pub max_lifetime: f32,
    /// `armTime` (Single)
    #[serde(default)]
    pub arm_time: f32,
    /// `maxArmableOverride` (Int32)
    #[serde(default)]
    pub max_armable_override: i32,
    /// `igniteTime` (Single)
    #[serde(default)]
    pub ignite_time: f32,
    /// `collisionDelayTime` (Single)
    #[serde(default)]
    pub collision_delay_time: f32,
    /// `explosionSafetyDistance` (Single)
    #[serde(default)]
    pub explosion_safety_distance: f32,
    /// `projectileProximity` (Single)
    #[serde(default)]
    pub projectile_proximity: f32,
    /// `explosionParams` (Class)
    #[serde(default)]
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// `clusterParams` (StrongPointer)
    #[serde(default)]
    pub cluster_params: Option<Handle<SOrdnanceClusterParams>>,
    /// `emissionsParams` (Class)
    #[serde(default)]
    pub emissions_params: Option<Handle<SOrdnanceEmissionsParams>>,
}

impl Pooled for SCItemSpaceMineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.scitem_space_mine_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.scitem_space_mine_params }
}

impl<'a> Extract<'a> for SCItemSpaceMineParams {
    const TYPE_NAME: &'static str = "SCItemSpaceMineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SOrdnanceAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
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
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cluster_params: match inst.get("clusterParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SOrdnanceClusterParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emissions_params: match inst.get("emissionsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SOrdnanceEmissionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMFDParamsDiagnostics`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDParamsDiagnostics {
    /// `healthThresholdDamaged` (Single)
    #[serde(default)]
    pub health_threshold_damaged: f32,
    /// `healthThresholdCritical` (Single)
    #[serde(default)]
    pub health_threshold_critical: f32,
    /// `excludedItemTypes` (Class (array))
    #[serde(default)]
    pub excluded_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `includedItemTypes` (Class (array))
    #[serde(default)]
    pub included_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `typeIcons` (String)
    #[serde(default)]
    pub type_icons: String,
}

impl Pooled for SMFDParamsDiagnostics {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdparams_diagnostics }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdparams_diagnostics }
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            included_item_types: inst.get_array("includedItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            type_icons: inst.get_str("typeIcons").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SDummyLauncher`
/// Inherits from: `SLauncherBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDummyLauncher {
}

impl Pooled for SDummyLauncher {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sdummy_launcher }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sdummy_launcher }
}

impl<'a> Extract<'a> for SDummyLauncher {
    const TYPE_NAME: &'static str = "SDummyLauncher";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

