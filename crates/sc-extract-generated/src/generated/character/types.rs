// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `character`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CharacterPersonalData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPersonalData {
    /// `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// `firstName` (Locale)
    #[serde(default)]
    pub first_name: String,
    /// `nickName` (Locale)
    #[serde(default)]
    pub nick_name: String,
    /// `lastName` (Locale)
    #[serde(default)]
    pub last_name: String,
    /// `randomNameParams` (Reference)
    #[serde(default)]
    pub random_name_params: Option<CigGuid>,
    /// `affiliation` (Locale)
    #[serde(default)]
    pub affiliation: String,
    /// `details` (Locale)
    #[serde(default)]
    pub details: String,
}

impl Pooled for CharacterPersonalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.character_personal_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.character_personal_data }
}

impl<'a> Extract<'a> for CharacterPersonalData {
    const TYPE_NAME: &'static str = "CharacterPersonalData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            first_name: inst.get_str("firstName").map(String::from).unwrap_or_default(),
            nick_name: inst.get_str("nickName").map(String::from).unwrap_or_default(),
            last_name: inst.get_str("lastName").map(String::from).unwrap_or_default(),
            random_name_params: inst.get("randomNameParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            affiliation: inst.get_str("affiliation").map(String::from).unwrap_or_default(),
            details: inst.get_str("details").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Character`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// `characterPersonalData` (Class)
    #[serde(default)]
    pub character_personal_data: Option<Handle<CharacterPersonalData>>,
    /// `audioSwitchStateName` (String)
    #[serde(default)]
    pub audio_switch_state_name: String,
    /// `animationRemapAlias` (String)
    #[serde(default)]
    pub animation_remap_alias: String,
    /// `characterMannequinTag` (String)
    #[serde(default)]
    pub character_mannequin_tag: String,
    /// `styleMannequinTag` (String)
    #[serde(default)]
    pub style_mannequin_tag: String,
    /// `combatStyle` (EnumChoice)
    #[serde(default)]
    pub combat_style: String,
    /// `gender` (EnumChoice)
    #[serde(default)]
    pub gender: String,
    /// `characterSpeedOverrides` (Class (array))
    #[serde(default)]
    pub character_speed_overrides: Vec<Handle<MovementSpeedOverride>>,
    /// `nicknameChance` (Single)
    #[serde(default)]
    pub nickname_chance: f32,
    /// `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for Character {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.character }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.character }
}

impl<'a> Extract<'a> for Character {
    const TYPE_NAME: &'static str = "Character";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            character_personal_data: match inst.get("characterPersonalData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CharacterPersonalData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CharacterPersonalData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_switch_state_name: inst.get_str("audioSwitchStateName").map(String::from).unwrap_or_default(),
            animation_remap_alias: inst.get_str("animationRemapAlias").map(String::from).unwrap_or_default(),
            character_mannequin_tag: inst.get_str("characterMannequinTag").map(String::from).unwrap_or_default(),
            style_mannequin_tag: inst.get_str("styleMannequinTag").map(String::from).unwrap_or_default(),
            combat_style: inst.get_str("combatStyle").map(String::from).unwrap_or_default(),
            gender: inst.get_str("gender").map(String::from).unwrap_or_default(),
            character_speed_overrides: inst.get_array("characterSpeedOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MovementSpeedOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MovementSpeedOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            nickname_chance: inst.get_f32("nicknameChance").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MovementSpeedOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSpeedOverride {
    /// `speedCategory` (EnumChoice)
    #[serde(default)]
    pub speed_category: String,
    /// `speedOverride` (Single)
    #[serde(default)]
    pub speed_override: f32,
}

impl Pooled for MovementSpeedOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.movement_speed_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.movement_speed_override }
}

impl<'a> Extract<'a> for MovementSpeedOverride {
    const TYPE_NAME: &'static str = "MovementSpeedOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            speed_category: inst.get_str("speedCategory").map(String::from).unwrap_or_default(),
            speed_override: inst.get_f32("speedOverride").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultPlayerLoadoutEntitlementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementParams {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `LoadoutId` (UInt32)
    #[serde(default)]
    pub loadout_id: u32,
    /// `Entitlement` (EnumChoice)
    #[serde(default)]
    pub entitlement: String,
    /// `Loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.default_player_loadout_entitlement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.default_player_loadout_entitlement_params }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementParams {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            loadout_id: inst.get_u32("LoadoutId").unwrap_or_default(),
            entitlement: inst.get_str("Entitlement").map(String::from).unwrap_or_default(),
            loadout: match inst.get("Loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultPlayerLoadoutEntitlementRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementRecord {
    /// `Loadouts` (Class (array))
    #[serde(default)]
    pub loadouts: Vec<Handle<DefaultPlayerLoadoutEntitlementParams>>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.default_player_loadout_entitlement_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.default_player_loadout_entitlement_record }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementRecord {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadouts: inst.get_array("Loadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

