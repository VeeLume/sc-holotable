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

/// DCB type: `LinkedStatPassValueBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatPassValueBase {
}

impl Pooled for LinkedStatPassValueBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.linked_stat_pass_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.linked_stat_pass_value_base }
}

impl<'a> Extract<'a> for LinkedStatPassValueBase {
    const TYPE_NAME: &'static str = "LinkedStatPassValueBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LinkedStatRuleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatRuleBase {
}

impl Pooled for LinkedStatRuleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.linked_stat_rule_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.linked_stat_rule_base }
}

impl<'a> Extract<'a> for LinkedStatRuleBase {
    const TYPE_NAME: &'static str = "LinkedStatRuleBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LinkedStatSetup`
///
/// Inherits from: `LinkedStatSetupBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatSetup {
    /// DCB field: `ruleForPassingValue` (StrongPointer)
    #[serde(default)]
    pub rule_for_passing_value: Option<Handle<LinkedStatRuleBase>>,
    /// DCB field: `valueToPass` (StrongPointer)
    #[serde(default)]
    pub value_to_pass: Option<Handle<LinkedStatPassValueBase>>,
}

impl Pooled for LinkedStatSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.linked_stat_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.linked_stat_setup }
}

impl<'a> Extract<'a> for LinkedStatSetup {
    const TYPE_NAME: &'static str = "LinkedStatSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rule_for_passing_value: match inst.get("ruleForPassingValue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LinkedStatRuleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LinkedStatRuleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            value_to_pass: match inst.get("valueToPass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LinkedStatPassValueBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LinkedStatPassValueBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LinkedStatSetupPreset`
///
/// Inherits from: `LinkedStatSetupBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatSetupPreset {
    /// DCB field: `setup` (Class)
    #[serde(default)]
    pub setup: Option<Handle<LinkedStatSetup>>,
}

impl Pooled for LinkedStatSetupPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.linked_stat_setup_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.linked_stat_setup_preset }
}

impl<'a> Extract<'a> for LinkedStatSetupPreset {
    const TYPE_NAME: &'static str = "LinkedStatSetupPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            setup: match inst.get("setup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LinkedStatSetup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LinkedStatSetup>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LinkedStatBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatBase {
    /// DCB field: `linkedStat` (EnumChoice)
    #[serde(default)]
    pub linked_stat: String,
}

impl Pooled for LinkedStatBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.linked_stat_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.linked_stat_base }
}

impl<'a> Extract<'a> for LinkedStatBase {
    const TYPE_NAME: &'static str = "LinkedStatBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            linked_stat: inst.get_str("linkedStat").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LicensedItemModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensedItemModifier {
    /// DCB field: `LicensedItem` (Reference)
    #[serde(default)]
    pub licensed_item: Option<CigGuid>,
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `PercentageModifier` (Single)
    #[serde(default)]
    pub percentage_modifier: f32,
    /// DCB field: `DisableDuplicateBadgeCheck` (Boolean)
    #[serde(default)]
    pub disable_duplicate_badge_check: bool,
}

impl Pooled for LicensedItemModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.licensed_item_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.licensed_item_modifier }
}

impl<'a> Extract<'a> for LicensedItemModifier {
    const TYPE_NAME: &'static str = "LicensedItemModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            licensed_item: inst.get("LicensedItem").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            percentage_modifier: inst.get_f32("PercentageModifier").unwrap_or_default(),
            disable_duplicate_badge_check: inst.get_bool("DisableDuplicateBadgeCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningBehavior {
    /// DCB field: `effects` (Class (array))
    #[serde(default)]
    pub effects: Vec<Handle<LightningBehavior_Effect>>,
}

impl Pooled for LightningBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.lightning_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.lightning_behavior }
}

impl<'a> Extract<'a> for LightningBehavior {
    const TYPE_NAME: &'static str = "LightningBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effects: inst.get_array("effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LightningBehavior_Effect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LightningBehavior_Effect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningBehavior_Effect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningBehavior_Effect {
    /// DCB field: `lightningEffect` (String)
    #[serde(default)]
    pub lightning_effect: String,
    /// DCB field: `targetModes` (StrongPointer (array))
    #[serde(default)]
    pub target_modes: Vec<Handle<LightningTargetMode>>,
    /// DCB field: `audio` (Class)
    #[serde(default)]
    pub audio: Option<Handle<LightningStrikeAudio>>,
}

impl Pooled for LightningBehavior_Effect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.lightning_behavior_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.lightning_behavior_effect }
}

impl<'a> Extract<'a> for LightningBehavior_Effect {
    const TYPE_NAME: &'static str = "LightningBehavior_Effect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lightning_effect: inst.get_str("lightningEffect").map(String::from).unwrap_or_default(),
            target_modes: inst.get_array("targetModes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LightningTargetMode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LightningTargetMode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LightningStrikeAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LightningStrikeAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LightningTargetMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningTargetMode {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
}

impl Pooled for LightningTargetMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.lightning_target_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.lightning_target_mode }
}

impl<'a> Extract<'a> for LightningTargetMode {
    const TYPE_NAME: &'static str = "LightningTargetMode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningStrikeAudio`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningStrikeAudio {
    /// DCB field: `emitterTrigger` (Class)
    #[serde(default)]
    pub emitter_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `targetTrigger` (Class)
    #[serde(default)]
    pub target_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for LightningStrikeAudio {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_li.lightning_strike_audio }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_li.lightning_strike_audio }
}

impl<'a> Extract<'a> for LightningStrikeAudio {
    const TYPE_NAME: &'static str = "LightningStrikeAudio";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            emitter_trigger: match inst.get("emitterTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_trigger: match inst.get("targetTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

