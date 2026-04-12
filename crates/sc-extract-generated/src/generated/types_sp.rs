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

/// DCB type: `SpineBone`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpineBone {
    /// DCB field: `boneName` (String)
    #[serde(default)]
    pub bone_name: String,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
    /// DCB field: `bonesToCounterRotate` (Class (array))
    #[serde(default)]
    pub bones_to_counter_rotate: Vec<Handle<BoneCounterRotateConfig>>,
}

impl Pooled for SpineBone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.spine_bone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.spine_bone }
}

impl<'a> Extract<'a> for SpineBone {
    const TYPE_NAME: &'static str = "SpineBone";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone_name: inst.get_str("boneName").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
            bones_to_counter_rotate: inst.get_array("bonesToCounterRotate")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BoneCounterRotateConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BoneCounterRotateConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SPlayerRoleShakeMultipliers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPlayerRoleShakeMultipliers {
    /// DCB field: `multipliers` (Single)
    #[serde(default)]
    pub multipliers: f32,
}

impl Pooled for SPlayerRoleShakeMultipliers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.splayer_role_shake_multipliers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.splayer_role_shake_multipliers }
}

impl<'a> Extract<'a> for SPlayerRoleShakeMultipliers {
    const TYPE_NAME: &'static str = "SPlayerRoleShakeMultipliers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            multipliers: inst.get_f32("multipliers").unwrap_or_default(),
        }
    }
}

/// DCB type: `SpeedThreshold`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedThreshold {
    /// DCB field: `inventoryWeight` (Single)
    #[serde(default)]
    pub inventory_weight: f32,
    /// DCB field: `speedFactor` (Single)
    #[serde(default)]
    pub speed_factor: f32,
}

impl Pooled for SpeedThreshold {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.speed_threshold }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.speed_threshold }
}

impl<'a> Extract<'a> for SpeedThreshold {
    const TYPE_NAME: &'static str = "SpeedThreshold";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            inventory_weight: inst.get_f32("inventoryWeight").unwrap_or_default(),
            speed_factor: inst.get_f32("speedFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `SpawnSettingsInventoryItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnSettingsInventoryItem {
    /// DCB field: `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
    /// DCB field: `quantity` (Int32)
    #[serde(default)]
    pub quantity: i32,
    /// DCB field: `amountInventoriesToFill` (Int32)
    #[serde(default)]
    pub amount_inventories_to_fill: i32,
    /// DCB field: `markAsMissionItem` (Boolean)
    #[serde(default)]
    pub mark_as_mission_item: bool,
}

impl Pooled for SpawnSettingsInventoryItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.spawn_settings_inventory_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.spawn_settings_inventory_item }
}

impl<'a> Extract<'a> for SpawnSettingsInventoryItem {
    const TYPE_NAME: &'static str = "SpawnSettingsInventoryItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            quantity: inst.get_i32("quantity").unwrap_or_default(),
            amount_inventories_to_fill: inst.get_i32("amountInventoriesToFill").unwrap_or_default(),
            mark_as_mission_item: inst.get_bool("markAsMissionItem").unwrap_or_default(),
        }
    }
}

/// DCB type: `SPrecisionTargetingItemName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPrecisionTargetingItemName {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for SPrecisionTargetingItemName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sprecision_targeting_item_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sprecision_targeting_item_name }
}

impl<'a> Extract<'a> for SPrecisionTargetingItemName {
    const TYPE_NAME: &'static str = "SPrecisionTargetingItemName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SPlayerScoring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPlayerScoring {
    /// DCB field: `playerScoreEvents` (Class (array))
    #[serde(default)]
    pub player_score_events: Vec<Handle<SScoreEvent>>,
}

impl Pooled for SPlayerScoring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.splayer_scoring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.splayer_scoring }
}

impl<'a> Extract<'a> for SPlayerScoring {
    const TYPE_NAME: &'static str = "SPlayerScoring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_score_events: inst.get_array("playerScoreEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScoreEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScoreEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpeedThrottleConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedThrottleConfiguration {
    /// DCB field: `activeMode` (EnumChoice)
    #[serde(default)]
    pub active_mode: String,
    /// DCB field: `defaultSpeed` (EnumChoice)
    #[serde(default)]
    pub default_speed: String,
    /// DCB field: `defaultSpeedWithWeapon` (EnumChoice)
    #[serde(default)]
    pub default_speed_with_weapon: String,
    /// DCB field: `mouseWheelSpeedStep` (Single)
    #[serde(default)]
    pub mouse_wheel_speed_step: f32,
    /// DCB field: `holdTime` (Single)
    #[serde(default)]
    pub hold_time: f32,
    /// DCB field: `durationAccelerateToFastRun` (Single)
    #[serde(default)]
    pub duration_accelerate_to_fast_run: f32,
    /// DCB field: `durationBoostToSprint` (Single)
    #[serde(default)]
    pub duration_boost_to_sprint: f32,
    /// DCB field: `durationUnboostFromSprint` (Single)
    #[serde(default)]
    pub duration_unboost_from_sprint: f32,
    /// DCB field: `durationDecelerateToDefault` (Single)
    #[serde(default)]
    pub duration_decelerate_to_default: f32,
    /// DCB field: `withWeaponDurationModifier` (Single)
    #[serde(default)]
    pub with_weapon_duration_modifier: f32,
    /// DCB field: `defaultTutorialSpeed` (EnumChoice)
    #[serde(default)]
    pub default_tutorial_speed: String,
}

impl Pooled for SpeedThrottleConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.speed_throttle_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.speed_throttle_configuration }
}

impl<'a> Extract<'a> for SpeedThrottleConfiguration {
    const TYPE_NAME: &'static str = "SpeedThrottleConfiguration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            active_mode: inst.get_str("activeMode").map(String::from).unwrap_or_default(),
            default_speed: inst.get_str("defaultSpeed").map(String::from).unwrap_or_default(),
            default_speed_with_weapon: inst.get_str("defaultSpeedWithWeapon").map(String::from).unwrap_or_default(),
            mouse_wheel_speed_step: inst.get_f32("mouseWheelSpeedStep").unwrap_or_default(),
            hold_time: inst.get_f32("holdTime").unwrap_or_default(),
            duration_accelerate_to_fast_run: inst.get_f32("durationAccelerateToFastRun").unwrap_or_default(),
            duration_boost_to_sprint: inst.get_f32("durationBoostToSprint").unwrap_or_default(),
            duration_unboost_from_sprint: inst.get_f32("durationUnboostFromSprint").unwrap_or_default(),
            duration_decelerate_to_default: inst.get_f32("durationDecelerateToDefault").unwrap_or_default(),
            with_weapon_duration_modifier: inst.get_f32("withWeaponDurationModifier").unwrap_or_default(),
            default_tutorial_speed: inst.get_str("defaultTutorialSpeed").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SPVPBountyContractGenerators`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPVPBountyContractGenerators {
    /// DCB field: `locationAvailable` (Reference)
    #[serde(default)]
    pub location_available: Option<CigGuid>,
    /// DCB field: `contractGenerator` (Reference)
    #[serde(default)]
    pub contract_generator: Option<CigGuid>,
}

impl Pooled for SPVPBountyContractGenerators {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.spvpbounty_contract_generators }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.spvpbounty_contract_generators }
}

impl<'a> Extract<'a> for SPVPBountyContractGenerators {
    const TYPE_NAME: &'static str = "SPVPBountyContractGenerators";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_available: inst.get("locationAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contract_generator: inst.get("contractGenerator").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SPerkParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkParamsBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `displayDescription` (Locale)
    #[serde(default)]
    pub display_description: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for SPerkParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sperk_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sperk_params_base }
}

impl<'a> Extract<'a> for SPerkParamsBase {
    const TYPE_NAME: &'static str = "SPerkParamsBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_description: inst.get_str("displayDescription").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SPerkReputationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkReputationParams {
    /// DCB field: `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// DCB field: `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// DCB field: `perk` (StrongPointer)
    #[serde(default)]
    pub perk: Option<Handle<SPerkParamsBase>>,
}

impl Pooled for SPerkReputationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sperk_reputation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sperk_reputation_params }
}

impl<'a> Extract<'a> for SPerkReputationParams {
    const TYPE_NAME: &'static str = "SPerkReputationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            perk: match inst.get("perk") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SPerkParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SPerkParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SPerkReputationListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkReputationListParams {
    /// DCB field: `perks` (Class (array))
    #[serde(default)]
    pub perks: Vec<Handle<SPerkReputationParams>>,
}

impl Pooled for SPerkReputationListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sperk_reputation_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sperk_reputation_list_params }
}

impl<'a> Extract<'a> for SPerkReputationListParams {
    const TYPE_NAME: &'static str = "SPerkReputationListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            perks: inst.get_array("perks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPerkReputationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPerkReputationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedPitchLadderParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SProjectedPitchLadderParams {
    /// DCB field: `visibleSizeAngle` (Single)
    #[serde(default)]
    pub visible_size_angle: f32,
    /// DCB field: `visibleFadeRatio` (Single)
    #[serde(default)]
    pub visible_fade_ratio: f32,
    /// DCB field: `incrementAngle` (Single)
    #[serde(default)]
    pub increment_angle: f32,
    /// DCB field: `centersEnabled` (Boolean)
    #[serde(default)]
    pub centers_enabled: bool,
    /// DCB field: `centersAlignmentType` (EnumChoice)
    #[serde(default)]
    pub centers_alignment_type: String,
    /// DCB field: `sidesEnabled` (Boolean)
    #[serde(default)]
    pub sides_enabled: bool,
    /// DCB field: `sidesHorizontalOffsetAngle` (Single)
    #[serde(default)]
    pub sides_horizontal_offset_angle: f32,
    /// DCB field: `sidesPositionType` (EnumChoice)
    #[serde(default)]
    pub sides_position_type: String,
    /// DCB field: `sidesAlignmentType` (EnumChoice)
    #[serde(default)]
    pub sides_alignment_type: String,
    /// DCB field: `labelsEnabled` (Boolean)
    #[serde(default)]
    pub labels_enabled: bool,
    /// DCB field: `labelsHorizontalOffsetAngle` (Single)
    #[serde(default)]
    pub labels_horizontal_offset_angle: f32,
    /// DCB field: `labelsPositionType` (EnumChoice)
    #[serde(default)]
    pub labels_position_type: String,
    /// DCB field: `labelsAlignmentType` (EnumChoice)
    #[serde(default)]
    pub labels_alignment_type: String,
    /// DCB field: `enableZeroPitchElements` (Boolean)
    #[serde(default)]
    pub enable_zero_pitch_elements: bool,
}

impl Pooled for SProjectedPitchLadderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sprojected_pitch_ladder_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sprojected_pitch_ladder_params }
}

impl<'a> Extract<'a> for SProjectedPitchLadderParams {
    const TYPE_NAME: &'static str = "SProjectedPitchLadderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            visible_size_angle: inst.get_f32("visibleSizeAngle").unwrap_or_default(),
            visible_fade_ratio: inst.get_f32("visibleFadeRatio").unwrap_or_default(),
            increment_angle: inst.get_f32("incrementAngle").unwrap_or_default(),
            centers_enabled: inst.get_bool("centersEnabled").unwrap_or_default(),
            centers_alignment_type: inst.get_str("centersAlignmentType").map(String::from).unwrap_or_default(),
            sides_enabled: inst.get_bool("sidesEnabled").unwrap_or_default(),
            sides_horizontal_offset_angle: inst.get_f32("sidesHorizontalOffsetAngle").unwrap_or_default(),
            sides_position_type: inst.get_str("sidesPositionType").map(String::from).unwrap_or_default(),
            sides_alignment_type: inst.get_str("sidesAlignmentType").map(String::from).unwrap_or_default(),
            labels_enabled: inst.get_bool("labelsEnabled").unwrap_or_default(),
            labels_horizontal_offset_angle: inst.get_f32("labelsHorizontalOffsetAngle").unwrap_or_default(),
            labels_position_type: inst.get_str("labelsPositionType").map(String::from).unwrap_or_default(),
            labels_alignment_type: inst.get_str("labelsAlignmentType").map(String::from).unwrap_or_default(),
            enable_zero_pitch_elements: inst.get_bool("enableZeroPitchElements").unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedYawLineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SProjectedYawLineParams {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `startAngle` (Single)
    #[serde(default)]
    pub start_angle: f32,
    /// DCB field: `endAngle` (Single)
    #[serde(default)]
    pub end_angle: f32,
    /// DCB field: `ticksEnabled` (Boolean)
    #[serde(default)]
    pub ticks_enabled: bool,
    /// DCB field: `tickForwardFadeStartAngle` (Single)
    #[serde(default)]
    pub tick_forward_fade_start_angle: f32,
    /// DCB field: `tickForwardFadeEndAngle` (Single)
    #[serde(default)]
    pub tick_forward_fade_end_angle: f32,
    /// DCB field: `tickBorderFadeAngle` (Single)
    #[serde(default)]
    pub tick_border_fade_angle: f32,
    /// DCB field: `tickIncrementAngle` (Single)
    #[serde(default)]
    pub tick_increment_angle: f32,
    /// DCB field: `tickIncrementVisualAngleRatio` (Single)
    #[serde(default)]
    pub tick_increment_visual_angle_ratio: f32,
    /// DCB field: `tickAlignmentType` (EnumChoice)
    #[serde(default)]
    pub tick_alignment_type: String,
    /// DCB field: `ticksAddCorners` (Boolean)
    #[serde(default)]
    pub ticks_add_corners: bool,
    /// DCB field: `ticksAsFullCircle` (Boolean)
    #[serde(default)]
    pub ticks_as_full_circle: bool,
    /// DCB field: `fixYawLineToAngle` (Boolean)
    #[serde(default)]
    pub fix_yaw_line_to_angle: bool,
    /// DCB field: `fixedAngle` (Single)
    #[serde(default)]
    pub fixed_angle: f32,
    /// DCB field: `anchorType` (EnumChoice)
    #[serde(default)]
    pub anchor_type: String,
}

impl Pooled for SProjectedYawLineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sprojected_yaw_line_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sprojected_yaw_line_params }
}

impl<'a> Extract<'a> for SProjectedYawLineParams {
    const TYPE_NAME: &'static str = "SProjectedYawLineParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            start_angle: inst.get_f32("startAngle").unwrap_or_default(),
            end_angle: inst.get_f32("endAngle").unwrap_or_default(),
            ticks_enabled: inst.get_bool("ticksEnabled").unwrap_or_default(),
            tick_forward_fade_start_angle: inst.get_f32("tickForwardFadeStartAngle").unwrap_or_default(),
            tick_forward_fade_end_angle: inst.get_f32("tickForwardFadeEndAngle").unwrap_or_default(),
            tick_border_fade_angle: inst.get_f32("tickBorderFadeAngle").unwrap_or_default(),
            tick_increment_angle: inst.get_f32("tickIncrementAngle").unwrap_or_default(),
            tick_increment_visual_angle_ratio: inst.get_f32("tickIncrementVisualAngleRatio").unwrap_or_default(),
            tick_alignment_type: inst.get_str("tickAlignmentType").map(String::from).unwrap_or_default(),
            ticks_add_corners: inst.get_bool("ticksAddCorners").unwrap_or_default(),
            ticks_as_full_circle: inst.get_bool("ticksAsFullCircle").unwrap_or_default(),
            fix_yaw_line_to_angle: inst.get_bool("fixYawLineToAngle").unwrap_or_default(),
            fixed_angle: inst.get_f32("fixedAngle").unwrap_or_default(),
            anchor_type: inst.get_str("anchorType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedDisplayParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SProjectedDisplayParams {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `pitchOffset` (Single)
    #[serde(default)]
    pub pitch_offset: f32,
    /// DCB field: `yawOffset` (Single)
    #[serde(default)]
    pub yaw_offset: f32,
    /// DCB field: `alignmentType` (EnumChoice)
    #[serde(default)]
    pub alignment_type: String,
}

impl Pooled for SProjectedDisplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sprojected_display_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sprojected_display_params }
}

impl<'a> Extract<'a> for SProjectedDisplayParams {
    const TYPE_NAME: &'static str = "SProjectedDisplayParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            pitch_offset: inst.get_f32("pitchOffset").unwrap_or_default(),
            yaw_offset: inst.get_f32("yawOffset").unwrap_or_default(),
            alignment_type: inst.get_str("alignmentType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SProjectedHudParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SProjectedHudParams {
    /// DCB field: `pitchLadder` (Class)
    #[serde(default)]
    pub pitch_ladder: Option<Handle<SProjectedPitchLadderParams>>,
    /// DCB field: `yawLine` (Class)
    #[serde(default)]
    pub yaw_line: Option<Handle<SProjectedYawLineParams>>,
    /// DCB field: `display` (Class)
    #[serde(default)]
    pub display: Option<Handle<SProjectedDisplayParams>>,
    /// DCB field: `coilArrowShow` (Boolean)
    #[serde(default)]
    pub coil_arrow_show: bool,
    /// DCB field: `coilArrowOffsetAngle` (Single)
    #[serde(default)]
    pub coil_arrow_offset_angle: f32,
    /// DCB field: `coilArrowRotatesToTarget` (Boolean)
    #[serde(default)]
    pub coil_arrow_rotates_to_target: bool,
}

impl Pooled for SProjectedHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.sprojected_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.sprojected_hud_params }
}

impl<'a> Extract<'a> for SProjectedHudParams {
    const TYPE_NAME: &'static str = "SProjectedHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pitch_ladder: match inst.get("pitchLadder") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedPitchLadderParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SProjectedPitchLadderParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw_line: match inst.get("yawLine") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedYawLineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SProjectedYawLineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display: match inst.get("display") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SProjectedDisplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SProjectedDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            coil_arrow_show: inst.get_bool("coilArrowShow").unwrap_or_default(),
            coil_arrow_offset_angle: inst.get_f32("coilArrowOffsetAngle").unwrap_or_default(),
            coil_arrow_rotates_to_target: inst.get_bool("coilArrowRotatesToTarget").unwrap_or_default(),
        }
    }
}

/// DCB type: `SpawnDescriptionEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnDescriptionEntry {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `spawnGroup` (StrongPointer)
    #[serde(default)]
    pub spawn_group: Option<Handle<BaseMissionPropertyValue>>,
}

impl Pooled for SpawnDescriptionEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.spawn_description_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.spawn_description_entry }
}

impl<'a> Extract<'a> for SpawnDescriptionEntry {
    const TYPE_NAME: &'static str = "SpawnDescriptionEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            spawn_group: match inst.get("spawnGroup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseMissionPropertyValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseMissionPropertyValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SpawnDescriptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnDescriptions {
    /// DCB field: `spawnDescriptions` (Class (array))
    #[serde(default)]
    pub spawn_descriptions: Vec<Handle<SpawnDescriptionEntry>>,
}

impl Pooled for SpawnDescriptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.spawn_descriptions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.spawn_descriptions }
}

impl<'a> Extract<'a> for SpawnDescriptions {
    const TYPE_NAME: &'static str = "SpawnDescriptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spawn_descriptions: inst.get_array("spawnDescriptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpawnDescriptionEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpawnDescriptionEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpecialEventManufacturer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventManufacturer {
}

impl Pooled for SpecialEventManufacturer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.special_event_manufacturer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.special_event_manufacturer }
}

impl<'a> Extract<'a> for SpecialEventManufacturer {
    const TYPE_NAME: &'static str = "SpecialEventManufacturer";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SpecialEventDay`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventDay {
    /// DCB field: `manufacturers` (Reference (array))
    #[serde(default)]
    pub manufacturers: Vec<CigGuid>,
    /// DCB field: `displayNames` (Locale (array))
    #[serde(default)]
    pub display_names: Vec<String>,
}

impl Pooled for SpecialEventDay {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.special_event_day }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.special_event_day }
}

impl<'a> Extract<'a> for SpecialEventDay {
    const TYPE_NAME: &'static str = "SpecialEventDay";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            manufacturers: inst.get_array("manufacturers")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            display_names: inst.get_array("displayNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpecialEventDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEventDatabase {
    /// DCB field: `days` (Reference (array))
    #[serde(default)]
    pub days: Vec<CigGuid>,
}

impl Pooled for SpecialEventDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sp.special_event_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sp.special_event_database }
}

impl<'a> Extract<'a> for SpecialEventDatabase {
    const TYPE_NAME: &'static str = "SpecialEventDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            days: inst.get_array("days")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

