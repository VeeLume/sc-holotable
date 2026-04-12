// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-playerchoice_library_playerchoicelibrary`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PlayerChoice_Option`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_Option {
    /// `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// `isPrimary` (Boolean)
    #[serde(default)]
    pub is_primary: bool,
}

impl Pooled for PlayerChoice_Option {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_library_playerchoicelibrary.player_choice_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_library_playerchoicelibrary.player_choice_option }
}

impl<'a> Extract<'a> for PlayerChoice_Option {
    const TYPE_NAME: &'static str = "PlayerChoice_Option";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            id: inst.get_i32("id").unwrap_or_default(),
            is_primary: inst.get_bool("isPrimary").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_OptionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_OptionList {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `options` (Class (array))
    #[serde(default)]
    pub options: Vec<Handle<PlayerChoice_Option>>,
}

impl Pooled for PlayerChoice_OptionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_library_playerchoicelibrary.player_choice_option_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_library_playerchoicelibrary.player_choice_option_list }
}

impl<'a> Extract<'a> for PlayerChoice_OptionList {
    const TYPE_NAME: &'static str = "PlayerChoice_OptionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerChoice_Option>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerChoice_Option>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_Library`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice_Library {
    /// `optionLists` (Class (array))
    #[serde(default)]
    pub option_lists: Vec<Handle<PlayerChoice_OptionList>>,
}

impl Pooled for PlayerChoice_Library {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_playerchoice_library_playerchoicelibrary.player_choice_library }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_playerchoice_library_playerchoicelibrary.player_choice_library }
}

impl<'a> Extract<'a> for PlayerChoice_Library {
    const TYPE_NAME: &'static str = "PlayerChoice_Library";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            option_lists: inst.get_array("optionLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerChoice_OptionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerChoice_OptionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

