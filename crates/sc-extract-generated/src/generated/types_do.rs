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

/// DCB type: `DownedConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownedConfig {
    /// DCB field: `excessDamageToInstaKill` (Single)
    #[serde(default)]
    pub excess_damage_to_insta_kill: f32,
    /// DCB field: `damageToInstaKillFromDowned` (Single)
    #[serde(default)]
    pub damage_to_insta_kill_from_downed: f32,
    /// DCB field: `downedReviveTempFOVFadeInSpeed` (Single)
    #[serde(default)]
    pub downed_revive_temp_fovfade_in_speed: f32,
    /// DCB field: `downedReviveTempFOVFadeOutSpeed` (Single)
    #[serde(default)]
    pub downed_revive_temp_fovfade_out_speed: f32,
    /// DCB field: `downedReviveTempFOVDuration` (Single)
    #[serde(default)]
    pub downed_revive_temp_fovduration: f32,
    /// DCB field: `downedReviveTempFOVScaleAdjustment` (Single)
    #[serde(default)]
    pub downed_revive_temp_fovscale_adjustment: f32,
    /// DCB field: `landingZoneRescueTime` (Single)
    #[serde(default)]
    pub landing_zone_rescue_time: f32,
    /// DCB field: `isPlayerBuddy` (Boolean)
    #[serde(default)]
    pub is_player_buddy: bool,
}

impl Pooled for DownedConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_do.downed_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_do.downed_config }
}

impl<'a> Extract<'a> for DownedConfig {
    const TYPE_NAME: &'static str = "DownedConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            excess_damage_to_insta_kill: inst.get_f32("excessDamageToInstaKill").unwrap_or_default(),
            damage_to_insta_kill_from_downed: inst.get_f32("damageToInstaKillFromDowned").unwrap_or_default(),
            downed_revive_temp_fovfade_in_speed: inst.get_f32("downedReviveTempFOVFadeInSpeed").unwrap_or_default(),
            downed_revive_temp_fovfade_out_speed: inst.get_f32("downedReviveTempFOVFadeOutSpeed").unwrap_or_default(),
            downed_revive_temp_fovduration: inst.get_f32("downedReviveTempFOVDuration").unwrap_or_default(),
            downed_revive_temp_fovscale_adjustment: inst.get_f32("downedReviveTempFOVScaleAdjustment").unwrap_or_default(),
            landing_zone_rescue_time: inst.get_f32("landingZoneRescueTime").unwrap_or_default(),
            is_player_buddy: inst.get_bool("isPlayerBuddy").unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSlotVisibilityTagSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingSlotVisibilityTagSet {
    /// DCB field: `tags` (String (array))
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Pooled for DockingSlotVisibilityTagSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_do.docking_slot_visibility_tag_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_do.docking_slot_visibility_tag_set }
}

impl<'a> Extract<'a> for DockingSlotVisibilityTagSet {
    const TYPE_NAME: &'static str = "DockingSlotVisibilityTagSet";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSlotVisibilityRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingSlotVisibilityRule {
    /// DCB field: `modes` (EnumChoice (array))
    #[serde(default)]
    pub modes: Vec<String>,
    /// DCB field: `tagSets` (Class (array))
    #[serde(default)]
    pub tag_sets: Vec<Handle<DockingSlotVisibilityTagSet>>,
}

impl Pooled for DockingSlotVisibilityRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_do.docking_slot_visibility_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_do.docking_slot_visibility_rule }
}

impl<'a> Extract<'a> for DockingSlotVisibilityRule {
    const TYPE_NAME: &'static str = "DockingSlotVisibilityRule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modes: inst.get_array("modes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            tag_sets: inst.get_array("tagSets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DockingSlotVisibilityTagSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DockingSlotVisibilityTagSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSlotVisibility`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingSlotVisibility {
    /// DCB field: `rules` (Class (array))
    #[serde(default)]
    pub rules: Vec<Handle<DockingSlotVisibilityRule>>,
}

impl Pooled for DockingSlotVisibility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_do.docking_slot_visibility }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_do.docking_slot_visibility }
}

impl<'a> Extract<'a> for DockingSlotVisibility {
    const TYPE_NAME: &'static str = "DockingSlotVisibility";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DockingSlotVisibilityRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DockingSlotVisibilityRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSensitivity`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingSensitivity {
    /// DCB field: `DockingUIRotationalSensitivity` (Single)
    #[serde(default)]
    pub docking_uirotational_sensitivity: f32,
    /// DCB field: `DockingUILinearSensitivity` (Single)
    #[serde(default)]
    pub docking_uilinear_sensitivity: f32,
}

impl Pooled for DockingSensitivity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_do.docking_sensitivity }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_do.docking_sensitivity }
}

impl<'a> Extract<'a> for DockingSensitivity {
    const TYPE_NAME: &'static str = "DockingSensitivity";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            docking_uirotational_sensitivity: inst.get_f32("DockingUIRotationalSensitivity").unwrap_or_default(),
            docking_uilinear_sensitivity: inst.get_f32("DockingUILinearSensitivity").unwrap_or_default(),
        }
    }
}

