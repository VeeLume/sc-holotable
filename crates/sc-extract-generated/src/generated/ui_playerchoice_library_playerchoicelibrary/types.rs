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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PlayerChoice_Option`
pub struct PlayerChoice_Option {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `id` (Int32)
    pub id: i32,
    /// `isPrimary` (Boolean)
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
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            id: inst.get_i32("id").unwrap_or_default(),
            is_primary: inst.get_bool("isPrimary").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_OptionList`
pub struct PlayerChoice_OptionList {
    /// `name` (String)
    pub name: String,
    /// `options` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerChoice_Option>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerChoice_Library`
pub struct PlayerChoice_Library {
    /// `optionLists` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerChoice_OptionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

