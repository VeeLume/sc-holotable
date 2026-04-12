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

/// DCB type: `CharacterAccuracyModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAccuracyModifiers {
    /// DCB field: `timeSinceTargetSeen` (Class)
    #[serde(default)]
    pub time_since_target_seen: Option<Handle<AITimeSinceTargetSeen>>,
    /// DCB field: `targetStanceModifier` (Single)
    #[serde(default)]
    pub target_stance_modifier: f32,
}

impl Pooled for CharacterAccuracyModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character_accuracy_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character_accuracy_modifiers }
}

impl<'a> Extract<'a> for CharacterAccuracyModifiers {
    const TYPE_NAME: &'static str = "CharacterAccuracyModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            time_since_target_seen: match inst.get("timeSinceTargetSeen") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AITimeSinceTargetSeen>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AITimeSinceTargetSeen>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_stance_modifier: inst.get_f32("targetStanceModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `CharacterSkills`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkills {
    /// DCB field: `aiming` (Class)
    #[serde(default)]
    pub aiming: Option<Handle<Aiming>>,
    /// DCB field: `modifiers` (Class)
    #[serde(default)]
    pub modifiers: Option<Handle<CharacterAccuracyModifiers>>,
}

impl Pooled for CharacterSkills {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character_skills }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character_skills }
}

impl<'a> Extract<'a> for CharacterSkills {
    const TYPE_NAME: &'static str = "CharacterSkills";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aiming: match inst.get("aiming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Aiming>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Aiming>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: match inst.get("modifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CharacterAccuracyModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CharacterAccuracyModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CharacterPersonalData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPersonalData {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `firstName` (Locale)
    #[serde(default)]
    pub first_name: String,
    /// DCB field: `nickName` (Locale)
    #[serde(default)]
    pub nick_name: String,
    /// DCB field: `lastName` (Locale)
    #[serde(default)]
    pub last_name: String,
    /// DCB field: `randomNameParams` (Reference)
    #[serde(default)]
    pub random_name_params: Option<CigGuid>,
    /// DCB field: `affiliation` (Locale)
    #[serde(default)]
    pub affiliation: String,
    /// DCB field: `details` (Locale)
    #[serde(default)]
    pub details: String,
}

impl Pooled for CharacterPersonalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character_personal_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character_personal_data }
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
    /// DCB field: `characterPersonalData` (Class)
    #[serde(default)]
    pub character_personal_data: Option<Handle<CharacterPersonalData>>,
    /// DCB field: `audioSwitchStateName` (String)
    #[serde(default)]
    pub audio_switch_state_name: String,
    /// DCB field: `animationRemapAlias` (String)
    #[serde(default)]
    pub animation_remap_alias: String,
    /// DCB field: `characterMannequinTag` (String)
    #[serde(default)]
    pub character_mannequin_tag: String,
    /// DCB field: `styleMannequinTag` (String)
    #[serde(default)]
    pub style_mannequin_tag: String,
    /// DCB field: `combatStyle` (EnumChoice)
    #[serde(default)]
    pub combat_style: String,
    /// DCB field: `gender` (EnumChoice)
    #[serde(default)]
    pub gender: String,
    /// DCB field: `characterSpeedOverrides` (Class (array))
    #[serde(default)]
    pub character_speed_overrides: Vec<Handle<MovementSpeedOverride>>,
    /// DCB field: `nicknameChance` (Single)
    #[serde(default)]
    pub nickname_chance: f32,
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for Character {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character }
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

/// DCB type: `CharacterSerializationSettingsPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSerializationSettingsPreset {
    /// DCB field: `modelTag` (Boolean)
    #[serde(default)]
    pub model_tag: bool,
    /// DCB field: `voiceTag` (Boolean)
    #[serde(default)]
    pub voice_tag: bool,
    /// DCB field: `dna` (Boolean)
    #[serde(default)]
    pub dna: bool,
    /// DCB field: `loadout` (Boolean)
    #[serde(default)]
    pub loadout: bool,
    /// DCB field: `customizationLoadoutOnly` (Boolean)
    #[serde(default)]
    pub customization_loadout_only: bool,
    /// DCB field: `fullLoadout` (Boolean)
    #[serde(default)]
    pub full_loadout: bool,
    /// DCB field: `materials` (Boolean)
    #[serde(default)]
    pub materials: bool,
    /// DCB field: `fullMaterials` (Boolean)
    #[serde(default)]
    pub full_materials: bool,
    /// DCB field: `decals` (Boolean)
    #[serde(default)]
    pub decals: bool,
    /// DCB field: `materialSettings` (Class (array))
    #[serde(default)]
    pub material_settings: Vec<Handle<SCharacterSerializationMaterialsSettingsPreset>>,
}

impl Pooled for CharacterSerializationSettingsPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character_serialization_settings_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character_serialization_settings_preset }
}

impl<'a> Extract<'a> for CharacterSerializationSettingsPreset {
    const TYPE_NAME: &'static str = "CharacterSerializationSettingsPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            model_tag: inst.get_bool("modelTag").unwrap_or_default(),
            voice_tag: inst.get_bool("voiceTag").unwrap_or_default(),
            dna: inst.get_bool("dna").unwrap_or_default(),
            loadout: inst.get_bool("loadout").unwrap_or_default(),
            customization_loadout_only: inst.get_bool("customizationLoadoutOnly").unwrap_or_default(),
            full_loadout: inst.get_bool("fullLoadout").unwrap_or_default(),
            materials: inst.get_bool("materials").unwrap_or_default(),
            full_materials: inst.get_bool("fullMaterials").unwrap_or_default(),
            decals: inst.get_bool("decals").unwrap_or_default(),
            material_settings: inst.get_array("materialSettings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterSerializationMaterialsSettingsPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterSerializationMaterialsSettingsPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CharacterRandomNameParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRandomNameParams {
    /// DCB field: `firstNameString` (String)
    #[serde(default)]
    pub first_name_string: String,
    /// DCB field: `nickNameString` (String)
    #[serde(default)]
    pub nick_name_string: String,
    /// DCB field: `lastNameString` (String)
    #[serde(default)]
    pub last_name_string: String,
    /// DCB field: `numFirstNames` (Int32)
    #[serde(default)]
    pub num_first_names: i32,
    /// DCB field: `numNickNames` (Int32)
    #[serde(default)]
    pub num_nick_names: i32,
    /// DCB field: `numLastNames` (Int32)
    #[serde(default)]
    pub num_last_names: i32,
    /// DCB field: `nicknameChance` (Single)
    #[serde(default)]
    pub nickname_chance: f32,
}

impl Pooled for CharacterRandomNameParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.character_random_name_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.character_random_name_params }
}

impl<'a> Extract<'a> for CharacterRandomNameParams {
    const TYPE_NAME: &'static str = "CharacterRandomNameParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            first_name_string: inst.get_str("firstNameString").map(String::from).unwrap_or_default(),
            nick_name_string: inst.get_str("nickNameString").map(String::from).unwrap_or_default(),
            last_name_string: inst.get_str("lastNameString").map(String::from).unwrap_or_default(),
            num_first_names: inst.get_i32("numFirstNames").unwrap_or_default(),
            num_nick_names: inst.get_i32("numNickNames").unwrap_or_default(),
            num_last_names: inst.get_i32("numLastNames").unwrap_or_default(),
            nickname_chance: inst.get_f32("nicknameChance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatEmoteRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmoteRecord {
    /// DCB field: `packs` (Class (array))
    #[serde(default)]
    pub packs: Vec<Handle<ChatEmotePack>>,
}

impl Pooled for ChatEmoteRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_emote_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_emote_record }
}

impl<'a> Extract<'a> for ChatEmoteRecord {
    const TYPE_NAME: &'static str = "ChatEmoteRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            packs: inst.get_array("packs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatEmotePack>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatEmotePack>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatEmotePack`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmotePack {
    /// DCB field: `packType` (String)
    #[serde(default)]
    pub pack_type: String,
    /// DCB field: `emotes` (Class (array))
    #[serde(default)]
    pub emotes: Vec<Handle<ChatEmoteData>>,
}

impl Pooled for ChatEmotePack {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_emote_pack }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_emote_pack }
}

impl<'a> Extract<'a> for ChatEmotePack {
    const TYPE_NAME: &'static str = "ChatEmotePack";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pack_type: inst.get_str("packType").map(String::from).unwrap_or_default(),
            emotes: inst.get_array("emotes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatEmoteData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatEmoteData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatEmoteData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmoteData {
    /// DCB field: `emoteType` (Locale)
    #[serde(default)]
    pub emote_type: String,
    /// DCB field: `alternateEmoteTypes` (Locale (array))
    #[serde(default)]
    pub alternate_emote_types: Vec<String>,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `isInterruptable` (Boolean)
    #[serde(default)]
    pub is_interruptable: bool,
    /// DCB field: `animData` (Class)
    #[serde(default)]
    pub anim_data: Option<Handle<ChatEmoteAnimData>>,
}

impl Pooled for ChatEmoteData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_emote_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_emote_data }
}

impl<'a> Extract<'a> for ChatEmoteData {
    const TYPE_NAME: &'static str = "ChatEmoteData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            emote_type: inst.get_str("emoteType").map(String::from).unwrap_or_default(),
            alternate_emote_types: inst.get_array("alternateEmoteTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            is_interruptable: inst.get_bool("isInterruptable").unwrap_or_default(),
            anim_data: match inst.get("animData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ChatEmoteAnimData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ChatEmoteAnimData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatEmoteAnimData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmoteAnimData {
    /// DCB field: `fragmentID` (String)
    #[serde(default)]
    pub fragment_id: String,
    /// DCB field: `tagID` (String)
    #[serde(default)]
    pub tag_id: String,
    /// DCB field: `textToDisplay` (Locale)
    #[serde(default)]
    pub text_to_display: String,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for ChatEmoteAnimData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_emote_anim_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_emote_anim_data }
}

impl<'a> Extract<'a> for ChatEmoteAnimData {
    const TYPE_NAME: &'static str = "ChatEmoteAnimData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_id: inst.get_str("fragmentID").map(String::from).unwrap_or_default(),
            tag_id: inst.get_str("tagID").map(String::from).unwrap_or_default(),
            text_to_display: inst.get_str("textToDisplay").map(String::from).unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatCommandFastAccess`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCommandFastAccess {
    /// DCB field: `commands` (Class (array))
    #[serde(default)]
    pub commands: Vec<Handle<ChatCommandName>>,
}

impl Pooled for ChatCommandFastAccess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_command_fast_access }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_command_fast_access }
}

impl<'a> Extract<'a> for ChatCommandFastAccess {
    const TYPE_NAME: &'static str = "ChatCommandFastAccess";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            commands: inst.get_array("commands")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatCommandName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatCommandName>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatCommandName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCommandName {
    /// DCB field: `commandName` (String)
    #[serde(default)]
    pub command_name: String,
}

impl Pooled for ChatCommandName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_command_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_command_name }
}

impl<'a> Extract<'a> for ChatCommandName {
    const TYPE_NAME: &'static str = "ChatCommandName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            command_name: inst.get_str("commandName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatFilterOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFilterOptions {
    /// DCB field: `options` (Class (array))
    #[serde(default)]
    pub options: Vec<Handle<ChatFilter>>,
}

impl Pooled for ChatFilterOptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_filter_options }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_filter_options }
}

impl<'a> Extract<'a> for ChatFilterOptions {
    const TYPE_NAME: &'static str = "ChatFilterOptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFilter {
    /// DCB field: `tagId` (Int32)
    #[serde(default)]
    pub tag_id: i32,
    /// DCB field: `localizedString` (String)
    #[serde(default)]
    pub localized_string: String,
}

impl Pooled for ChatFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_filter }
}

impl<'a> Extract<'a> for ChatFilter {
    const TYPE_NAME: &'static str = "ChatFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag_id: inst.get_i32("tagId").unwrap_or_default(),
            localized_string: inst.get_str("localizedString").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatManagerDefaultChannelColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerDefaultChannelColor {
    /// DCB field: `global` (EnumChoice)
    #[serde(default)]
    pub global: String,
    /// DCB field: `party` (EnumChoice)
    #[serde(default)]
    pub party: String,
    /// DCB field: `gameEntity` (EnumChoice)
    #[serde(default)]
    pub game_entity: String,
    /// DCB field: `whisper` (EnumChoice)
    #[serde(default)]
    pub whisper: String,
    /// DCB field: `team` (EnumChoice)
    #[serde(default)]
    pub team: String,
    /// DCB field: `squad` (EnumChoice)
    #[serde(default)]
    pub squad: String,
}

impl Pooled for ChatManagerDefaultChannelColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_manager_default_channel_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_manager_default_channel_color }
}

impl<'a> Extract<'a> for ChatManagerDefaultChannelColor {
    const TYPE_NAME: &'static str = "ChatManagerDefaultChannelColor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global: inst.get_str("global").map(String::from).unwrap_or_default(),
            party: inst.get_str("party").map(String::from).unwrap_or_default(),
            game_entity: inst.get_str("gameEntity").map(String::from).unwrap_or_default(),
            whisper: inst.get_str("whisper").map(String::from).unwrap_or_default(),
            team: inst.get_str("team").map(String::from).unwrap_or_default(),
            squad: inst.get_str("squad").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatManagerColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerColor {
    /// DCB field: `colorType` (EnumChoice)
    #[serde(default)]
    pub color_type: String,
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGB8>>,
}

impl Pooled for ChatManagerColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_manager_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_manager_color }
}

impl<'a> Extract<'a> for ChatManagerColor {
    const TYPE_NAME: &'static str = "ChatManagerColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color_type: inst.get_str("colorType").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatManagerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerGlobalParams {
    /// DCB field: `defaultChannelColor` (Class)
    #[serde(default)]
    pub default_channel_color: Option<Handle<ChatManagerDefaultChannelColor>>,
    /// DCB field: `colorOptions` (Class (array))
    #[serde(default)]
    pub color_options: Vec<Handle<ChatManagerColor>>,
}

impl Pooled for ChatManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_manager_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_manager_global_params }
}

impl<'a> Extract<'a> for ChatManagerGlobalParams {
    const TYPE_NAME: &'static str = "ChatManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_channel_color: match inst.get("defaultChannelColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ChatManagerDefaultChannelColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ChatManagerDefaultChannelColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color_options: inst.get_array("colorOptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatManagerColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatManagerColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatChannelFilterRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannelFilterRecord {
    /// DCB field: `chatChannelFilter` (StrongPointer)
    #[serde(default)]
    pub chat_channel_filter: Option<Handle<SChatChannelFilterBase>>,
}

impl Pooled for ChatChannelFilterRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_channel_filter_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_channel_filter_record }
}

impl<'a> Extract<'a> for ChatChannelFilterRecord {
    const TYPE_NAME: &'static str = "ChatChannelFilterRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            chat_channel_filter: match inst.get("chatChannelFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChatChannelFilterBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChatChannelFilterBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatSystemOptionsModule`
///
/// Inherits from: `DataForgeComponentParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSystemOptionsModule {
}

impl Pooled for ChatSystemOptionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.chat_system_options_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.chat_system_options_module }
}

impl<'a> Extract<'a> for ChatSystemOptionsModule {
    const TYPE_NAME: &'static str = "ChatSystemOptionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ChildMissionPhase`
///
/// Inherits from: `ObjectiveToken` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildMissionPhase {
    /// DCB field: `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `startsActive` (Boolean)
    #[serde(default)]
    pub starts_active: bool,
    /// DCB field: `properties` (StrongPointer (array))
    #[serde(default)]
    pub properties: Vec<Handle<ObjectivePropertyBase>>,
    /// DCB field: `objectiveHandler` (StrongPointer)
    #[serde(default)]
    pub objective_handler: Option<Handle<ObjectiveHandlerBase>>,
    /// DCB field: `rewardContribution` (StrongPointer)
    #[serde(default)]
    pub reward_contribution: Option<Handle<ObjectiveRewardContributionBase>>,
    /// DCB field: `displayInfo` (Class)
    #[serde(default)]
    pub display_info: Option<Handle<ObjectiveDisplayInfo>>,
    /// DCB field: `commsNotifications` (Class (array))
    #[serde(default)]
    pub comms_notifications: Vec<Handle<CommsNotificationSelector>>,
    /// DCB field: `childMissionPhases` (Class (array))
    #[serde(default)]
    pub child_mission_phases: Vec<Handle<ChildMissionPhase>>,
    /// DCB field: `variableName` (String)
    #[serde(default)]
    pub variable_name: String,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for ChildMissionPhase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ch.child_mission_phase }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ch.child_mission_phase }
}

impl<'a> Extract<'a> for ChildMissionPhase {
    const TYPE_NAME: &'static str = "ChildMissionPhase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            starts_active: inst.get_bool("startsActive").unwrap_or_default(),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ObjectivePropertyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ObjectivePropertyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            objective_handler: match inst.get("objectiveHandler") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveHandlerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveHandlerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reward_contribution: match inst.get("rewardContribution") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveRewardContributionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveRewardContributionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_info: match inst.get("displayInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectiveDisplayInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectiveDisplayInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            comms_notifications: inst.get_array("commsNotifications")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommsNotificationSelector>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommsNotificationSelector>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            child_mission_phases: inst.get_array("childMissionPhases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChildMissionPhase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChildMissionPhase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            variable_name: inst.get_str("variableName").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

