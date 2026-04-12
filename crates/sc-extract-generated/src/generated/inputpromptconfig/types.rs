// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `inputpromptconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `InputPromptConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPromptConfig {
    /// `ownerName` (String)
    #[serde(default)]
    pub owner_name: String,
    /// `actionName` (Class)
    #[serde(default)]
    pub action_name: Option<Handle<InputAction>>,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `showLabel` (Boolean)
    #[serde(default)]
    pub show_label: bool,
    /// `inputPromptMode` (EnumChoice)
    #[serde(default)]
    pub input_prompt_mode: String,
    /// `inputPromptBoundTo` (EnumChoice)
    #[serde(default)]
    pub input_prompt_bound_to: String,
    /// `inputPromptPriority` (EnumChoice)
    #[serde(default)]
    pub input_prompt_priority: String,
    /// `helperName` (String)
    #[serde(default)]
    pub helper_name: String,
    /// `objectSlot` (Int32)
    #[serde(default)]
    pub object_slot: i32,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// `isAngleConstrained` (Boolean)
    #[serde(default)]
    pub is_angle_constrained: bool,
    /// `shouldShowOnSuccessEffect` (Boolean)
    #[serde(default)]
    pub should_show_on_success_effect: bool,
    /// `ignoreDifficultySettings` (Boolean)
    #[serde(default)]
    pub ignore_difficulty_settings: bool,
}

impl Pooled for InputPromptConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inputpromptconfig.input_prompt_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inputpromptconfig.input_prompt_config }
}

impl<'a> Extract<'a> for InputPromptConfig {
    const TYPE_NAME: &'static str = "InputPromptConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            owner_name: inst.get_str("ownerName").map(String::from).unwrap_or_default(),
            action_name: match inst.get("actionName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            show_label: inst.get_bool("showLabel").unwrap_or_default(),
            input_prompt_mode: inst.get_str("inputPromptMode").map(String::from).unwrap_or_default(),
            input_prompt_bound_to: inst.get_str("inputPromptBoundTo").map(String::from).unwrap_or_default(),
            input_prompt_priority: inst.get_str("inputPromptPriority").map(String::from).unwrap_or_default(),
            helper_name: inst.get_str("helperName").map(String::from).unwrap_or_default(),
            object_slot: inst.get_i32("objectSlot").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_angle_constrained: inst.get_bool("isAngleConstrained").unwrap_or_default(),
            should_show_on_success_effect: inst.get_bool("shouldShowOnSuccessEffect").unwrap_or_default(),
            ignore_difficulty_settings: inst.get_bool("ignoreDifficultySettings").unwrap_or_default(),
        }
    }
}

