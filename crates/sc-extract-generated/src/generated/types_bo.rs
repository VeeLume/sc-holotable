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

/// DCB type: `BoneCounterRotateConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneCounterRotateConfig {
    /// DCB field: `boneName` (String)
    #[serde(default)]
    pub bone_name: String,
    /// DCB field: `smoothingChainLength` (Int32)
    #[serde(default)]
    pub smoothing_chain_length: i32,
}

impl Pooled for BoneCounterRotateConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.bone_counter_rotate_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.bone_counter_rotate_config }
}

impl<'a> Extract<'a> for BoneCounterRotateConfig {
    const TYPE_NAME: &'static str = "BoneCounterRotateConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            bone_name: inst.get_str("boneName").map(String::from).unwrap_or_default(),
            smoothing_chain_length: inst.get_i32("smoothingChainLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `BodyPart`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPart {
    /// DCB field: `partHealthStat` (EnumChoice)
    #[serde(default)]
    pub part_health_stat: String,
    /// DCB field: `partWearStat` (EnumChoice)
    #[serde(default)]
    pub part_wear_stat: String,
    /// DCB field: `jointName` (String)
    #[serde(default)]
    pub joint_name: String,
}

impl Pooled for BodyPart {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.body_part }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.body_part }
}

impl<'a> Extract<'a> for BodyPart {
    const TYPE_NAME: &'static str = "BodyPart";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            part_health_stat: inst.get_str("partHealthStat").map(String::from).unwrap_or_default(),
            part_wear_stat: inst.get_str("partWearStat").map(String::from).unwrap_or_default(),
            joint_name: inst.get_str("jointName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BodyJoint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyJoint {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `hitReactionRegion` (EnumChoice)
    #[serde(default)]
    pub hit_reaction_region: String,
    /// DCB field: `hitReactionPart` (EnumChoice)
    #[serde(default)]
    pub hit_reaction_part: String,
    /// DCB field: `bodyPart` (Reference)
    #[serde(default)]
    pub body_part: Option<CigGuid>,
}

impl Pooled for BodyJoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.body_joint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.body_joint }
}

impl<'a> Extract<'a> for BodyJoint {
    const TYPE_NAME: &'static str = "BodyJoint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            hit_reaction_region: inst.get_str("hitReactionRegion").map(String::from).unwrap_or_default(),
            hit_reaction_part: inst.get_str("hitReactionPart").map(String::from).unwrap_or_default(),
            body_part: inst.get("bodyPart").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `BodyMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMapping {
    /// DCB field: `vehicleCollisionJointName` (String)
    #[serde(default)]
    pub vehicle_collision_joint_name: String,
    /// DCB field: `joints` (Class (array))
    #[serde(default)]
    pub joints: Vec<Handle<BodyJoint>>,
}

impl Pooled for BodyMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.body_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.body_mapping }
}

impl<'a> Extract<'a> for BodyMapping {
    const TYPE_NAME: &'static str = "BodyMapping";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_collision_joint_name: inst.get_str("vehicleCollisionJointName").map(String::from).unwrap_or_default(),
            joints: inst.get_array("joints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BodyJoint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BodyJoint>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BodyPartConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPartConfig {
    /// DCB field: `bodyParts` (Reference (array))
    #[serde(default)]
    pub body_parts: Vec<CigGuid>,
    /// DCB field: `damageMultiplier` (Single)
    #[serde(default)]
    pub damage_multiplier: f32,
    /// DCB field: `nakedDamageMultiplier` (Single)
    #[serde(default)]
    pub naked_damage_multiplier: f32,
    /// DCB field: `stunMultiplier` (Single)
    #[serde(default)]
    pub stun_multiplier: f32,
    /// DCB field: `impactForceMultiplier` (Single)
    #[serde(default)]
    pub impact_force_multiplier: f32,
    /// DCB field: `isHeadShot` (Boolean)
    #[serde(default)]
    pub is_head_shot: bool,
    /// DCB field: `isFallDamageTarget` (Boolean)
    #[serde(default)]
    pub is_fall_damage_target: bool,
    /// DCB field: `isFPViewOccluder` (Boolean)
    #[serde(default)]
    pub is_fpview_occluder: bool,
    /// DCB field: `damageCap` (Single)
    #[serde(default)]
    pub damage_cap: f32,
}

impl Pooled for BodyPartConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.body_part_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.body_part_config }
}

impl<'a> Extract<'a> for BodyPartConfig {
    const TYPE_NAME: &'static str = "BodyPartConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            body_parts: inst.get_array("bodyParts")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            damage_multiplier: inst.get_f32("damageMultiplier").unwrap_or_default(),
            naked_damage_multiplier: inst.get_f32("nakedDamageMultiplier").unwrap_or_default(),
            stun_multiplier: inst.get_f32("stunMultiplier").unwrap_or_default(),
            impact_force_multiplier: inst.get_f32("impactForceMultiplier").unwrap_or_default(),
            is_head_shot: inst.get_bool("isHeadShot").unwrap_or_default(),
            is_fall_damage_target: inst.get_bool("isFallDamageTarget").unwrap_or_default(),
            is_fpview_occluder: inst.get_bool("isFPViewOccluder").unwrap_or_default(),
            damage_cap: inst.get_f32("damageCap").unwrap_or_default(),
        }
    }
}

/// DCB type: `BodyHealthConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyHealthConfig {
    /// DCB field: `bleedingRtpc` (Class)
    #[serde(default)]
    pub bleeding_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `bleedRateRtpc` (Class)
    #[serde(default)]
    pub bleed_rate_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `isInvulnerable` (Boolean)
    #[serde(default)]
    pub is_invulnerable: bool,
    /// DCB field: `isVulnerableOnlyToPlayer` (Boolean)
    #[serde(default)]
    pub is_vulnerable_only_to_player: bool,
    /// DCB field: `setDamageEffectRatioOnChildren` (Boolean)
    #[serde(default)]
    pub set_damage_effect_ratio_on_children: bool,
    /// DCB field: `canBeRepaired` (Boolean)
    #[serde(default)]
    pub can_be_repaired: bool,
    /// DCB field: `ignoreZeroDamageHitEventsForTypes` (EnumChoice (array))
    #[serde(default)]
    pub ignore_zero_damage_hit_events_for_types: Vec<String>,
    /// DCB field: `bodyPartConfigs` (Class (array))
    #[serde(default)]
    pub body_part_configs: Vec<Handle<BodyPartConfig>>,
}

impl Pooled for BodyHealthConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.body_health_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.body_health_config }
}

impl<'a> Extract<'a> for BodyHealthConfig {
    const TYPE_NAME: &'static str = "BodyHealthConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bleeding_rtpc: match inst.get("bleedingRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bleed_rate_rtpc: match inst.get("bleedRateRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_invulnerable: inst.get_bool("isInvulnerable").unwrap_or_default(),
            is_vulnerable_only_to_player: inst.get_bool("isVulnerableOnlyToPlayer").unwrap_or_default(),
            set_damage_effect_ratio_on_children: inst.get_bool("setDamageEffectRatioOnChildren").unwrap_or_default(),
            can_be_repaired: inst.get_bool("canBeRepaired").unwrap_or_default(),
            ignore_zero_damage_hit_events_for_types: inst.get_array("ignoreZeroDamageHitEventsForTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            body_part_configs: inst.get_array("bodyPartConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BodyPartConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BodyPartConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BoxoutStat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxoutStat {
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `descriptor` (Locale)
    #[serde(default)]
    pub descriptor: String,
}

impl Pooled for BoxoutStat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.boxout_stat }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.boxout_stat }
}

impl<'a> Extract<'a> for BoxoutStat {
    const TYPE_NAME: &'static str = "BoxoutStat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            descriptor: inst.get_str("descriptor").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BoxoutItemStatus`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxoutItemStatus {
    /// DCB field: `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `toolTip` (Locale)
    #[serde(default)]
    pub tool_tip: String,
}

impl Pooled for BoxoutItemStatus {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.boxout_item_status }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.boxout_item_status }
}

impl<'a> Extract<'a> for BoxoutItemStatus {
    const TYPE_NAME: &'static str = "BoxoutItemStatus";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state: inst.get_str("state").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            tool_tip: inst.get_str("toolTip").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BoxoutAtmosphereStatus`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxoutAtmosphereStatus {
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `pressureRange` (Single)
    #[serde(default)]
    pub pressure_range: f32,
    /// DCB field: `temperatureRange` (Single)
    #[serde(default)]
    pub temperature_range: f32,
}

impl Pooled for BoxoutAtmosphereStatus {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_bo.boxout_atmosphere_status }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_bo.boxout_atmosphere_status }
}

impl<'a> Extract<'a> for BoxoutAtmosphereStatus {
    const TYPE_NAME: &'static str = "BoxoutAtmosphereStatus";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            pressure_range: inst.get_f32("pressureRange").unwrap_or_default(),
            temperature_range: inst.get_f32("temperatureRange").unwrap_or_default(),
        }
    }
}

