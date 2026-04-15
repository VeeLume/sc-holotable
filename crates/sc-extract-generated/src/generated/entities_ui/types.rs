// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-ui`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RecordRefUserVariableTypeFontStyle`
/// Inherits from: `RecordRefUserVariableTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRefUserVariableTypeFontStyle {
    /// `value` (Reference)
    #[serde(default)]
    pub value: Option<CigGuid>,
}

impl Pooled for RecordRefUserVariableTypeFontStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.record_ref_user_variable_type_font_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.record_ref_user_variable_type_font_style }
}

impl<'a> Extract<'a> for RecordRefUserVariableTypeFontStyle {
    const TYPE_NAME: &'static str = "RecordRefUserVariableTypeFontStyle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get("value").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SpawnNotifierEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnNotifierEntityComponentParams {
}

impl Pooled for SpawnNotifierEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.spawn_notifier_entity_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.spawn_notifier_entity_component_params }
}

impl<'a> Extract<'a> for SpawnNotifierEntityComponentParams {
    const TYPE_NAME: &'static str = "SpawnNotifierEntityComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMatchNetworkTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMatchNetworkTypeParams {
    /// `name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `description` (Locale)
    #[serde(default)]
    pub description: LocaleKey,
    /// `matchNetworkType` (EnumChoice)
    #[serde(default)]
    pub match_network_type: EMatchNetworkType,
}

impl Pooled for SMatchNetworkTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.smatch_network_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.smatch_network_type_params }
}

impl<'a> Extract<'a> for SMatchNetworkTypeParams {
    const TYPE_NAME: &'static str = "SMatchNetworkTypeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            match_network_type: EMatchNetworkType::from_dcb_str(inst.get_str("matchNetworkType").unwrap_or("")),
        }
    }
}

/// DCB type: `PopupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupParams {
    /// `locTitle` (Locale)
    #[serde(default)]
    pub loc_title: LocaleKey,
    /// `locBody` (Locale)
    #[serde(default)]
    pub loc_body: LocaleKey,
    /// `locConfirm` (Locale)
    #[serde(default)]
    pub loc_confirm: LocaleKey,
    /// `locDecline` (Locale)
    #[serde(default)]
    pub loc_decline: LocaleKey,
}

impl Pooled for PopupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.popup_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.popup_params }
}

impl<'a> Extract<'a> for PopupParams {
    const TYPE_NAME: &'static str = "PopupParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loc_title: inst.get_str("locTitle").map(LocaleKey::from).unwrap_or_default(),
            loc_body: inst.get_str("locBody").map(LocaleKey::from).unwrap_or_default(),
            loc_confirm: inst.get_str("locConfirm").map(LocaleKey::from).unwrap_or_default(),
            loc_decline: inst.get_str("locDecline").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EAPatchNoteParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAPatchNoteParams {
    /// `note` (Locale)
    #[serde(default)]
    pub note: LocaleKey,
    /// `bold` (Boolean)
    #[serde(default)]
    pub bold: bool,
    /// `underlined` (Boolean)
    #[serde(default)]
    pub underlined: bool,
    /// `italic` (Boolean)
    #[serde(default)]
    pub italic: bool,
}

impl Pooled for EAPatchNoteParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.eapatch_note_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.eapatch_note_params }
}

impl<'a> Extract<'a> for EAPatchNoteParams {
    const TYPE_NAME: &'static str = "EAPatchNoteParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            note: inst.get_str("note").map(LocaleKey::from).unwrap_or_default(),
            bold: inst.get_bool("bold").unwrap_or_default(),
            underlined: inst.get_bool("underlined").unwrap_or_default(),
            italic: inst.get_bool("italic").unwrap_or_default(),
        }
    }
}

/// DCB type: `EAPatchNoteCategoryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAPatchNoteCategoryParams {
    /// `name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `notes` (Class (array))
    #[serde(default)]
    pub notes: Vec<Handle<EAPatchNoteParams>>,
}

impl Pooled for EAPatchNoteCategoryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.eapatch_note_category_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.eapatch_note_category_params }
}

impl<'a> Extract<'a> for EAPatchNoteCategoryParams {
    const TYPE_NAME: &'static str = "EAPatchNoteCategoryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
            notes: inst.get_array("notes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EAPatchNoteParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EAPatchNoteParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SocialTab`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialTab {
    /// `name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `activeNetworkTypes` (EnumChoice (array))
    #[serde(default)]
    pub active_network_types: Vec<EMatchNetworkType>,
}

impl Pooled for SocialTab {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.social_tab }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.social_tab }
}

impl<'a> Extract<'a> for SocialTab {
    const TYPE_NAME: &'static str = "SocialTab";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
            active_network_types: inst.get_array("activeNetworkTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EMatchNetworkType::from_dcb_str)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EAExperimentalModeResetTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAExperimentalModeResetTimeParams {
    /// `weekday` (EnumChoice)
    #[serde(default)]
    pub weekday: EWeekday,
    /// `hour` (Int32)
    #[serde(default)]
    pub hour: i32,
    /// `minute` (Int32)
    #[serde(default)]
    pub minute: i32,
    /// `showSecondsInTimer` (Boolean)
    #[serde(default)]
    pub show_seconds_in_timer: bool,
    /// `locRestartRequired` (Locale)
    #[serde(default)]
    pub loc_restart_required: LocaleKey,
}

impl Pooled for EAExperimentalModeResetTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.eaexperimental_mode_reset_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.eaexperimental_mode_reset_time_params }
}

impl<'a> Extract<'a> for EAExperimentalModeResetTimeParams {
    const TYPE_NAME: &'static str = "EAExperimentalModeResetTimeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            weekday: EWeekday::from_dcb_str(inst.get_str("weekday").unwrap_or("")),
            hour: inst.get_i32("hour").unwrap_or_default(),
            minute: inst.get_i32("minute").unwrap_or_default(),
            show_seconds_in_timer: inst.get_bool("showSecondsInTimer").unwrap_or_default(),
            loc_restart_required: inst.get_str("locRestartRequired").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EASpecialEventInformationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EASpecialEventInformationParams {
    /// `badgeId` (UInt32)
    #[serde(default)]
    pub badge_id: u32,
    /// `eventTitle` (Locale)
    #[serde(default)]
    pub event_title: LocaleKey,
    /// `eventDescription` (Locale)
    #[serde(default)]
    pub event_description: LocaleKey,
    /// `bannerImage` (String)
    #[serde(default)]
    pub banner_image: String,
    /// `loadscreenOverride` (String)
    #[serde(default)]
    pub loadscreen_override: String,
    /// `backgroundVideoOverride` (String)
    #[serde(default)]
    pub background_video_override: String,
    /// `modesActiveWithEvent` (EnumChoice (array))
    #[serde(default)]
    pub modes_active_with_event: Vec<EGameModeId>,
    /// `style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
}

impl Pooled for EASpecialEventInformationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.easpecial_event_information_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.easpecial_event_information_params }
}

impl<'a> Extract<'a> for EASpecialEventInformationParams {
    const TYPE_NAME: &'static str = "EASpecialEventInformationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            badge_id: inst.get_u32("badgeId").unwrap_or_default(),
            event_title: inst.get_str("eventTitle").map(LocaleKey::from).unwrap_or_default(),
            event_description: inst.get_str("eventDescription").map(LocaleKey::from).unwrap_or_default(),
            banner_image: inst.get_str("bannerImage").map(String::from).unwrap_or_default(),
            loadscreen_override: inst.get_str("loadscreenOverride").map(String::from).unwrap_or_default(),
            background_video_override: inst.get_str("backgroundVideoOverride").map(String::from).unwrap_or_default(),
            modes_active_with_event: inst.get_array("modesActiveWithEvent")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EGameModeId::from_dcb_str)).collect())
                .unwrap_or_default(),
            style: inst.get("style").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityComponentFrontendEAUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityComponentFrontendEAUIProviderParams {
    /// `matchNetworkTypeNames` (Class (array))
    #[serde(default)]
    pub match_network_type_names: Vec<Handle<SMatchNetworkTypeParams>>,
    /// `anyMapOptionName` (Locale)
    #[serde(default)]
    pub any_map_option_name: LocaleKey,
    /// `anyMapOptionDescription` (Locale)
    #[serde(default)]
    pub any_map_option_description: LocaleKey,
    /// `anyMapOptionThumbnail` (String)
    #[serde(default)]
    pub any_map_option_thumbnail: String,
    /// `anyTeamOptionName` (Locale)
    #[serde(default)]
    pub any_team_option_name: LocaleKey,
    /// `anyTeamOptionDescription` (Locale)
    #[serde(default)]
    pub any_team_option_description: LocaleKey,
    /// `locReadyCount` (Locale)
    #[serde(default)]
    pub loc_ready_count: LocaleKey,
    /// `locErrorReadyCheck` (Locale)
    #[serde(default)]
    pub loc_error_ready_check: LocaleKey,
    /// `locErrorPlayerLimit` (Locale)
    #[serde(default)]
    pub loc_error_player_limit: LocaleKey,
    /// `locErrorMissingBadge` (Locale)
    #[serde(default)]
    pub loc_error_missing_badge: LocaleKey,
    /// `locErrorMinPlayers` (Locale)
    #[serde(default)]
    pub loc_error_min_players: LocaleKey,
    /// `featuredGameMode` (EnumChoice)
    #[serde(default)]
    pub featured_game_mode: EGameModeId,
    /// `popupParams` (Class)
    #[serde(default)]
    pub popup_params: Option<Handle<PopupParams>>,
    /// `patchNoteCategories` (Class (array))
    #[serde(default)]
    pub patch_note_categories: Vec<Handle<EAPatchNoteCategoryParams>>,
    /// `socialTabs` (Class (array))
    #[serde(default)]
    pub social_tabs: Vec<Handle<SocialTab>>,
    /// `experimentalModeResetTime` (Class)
    #[serde(default)]
    pub experimental_mode_reset_time: Option<Handle<EAExperimentalModeResetTimeParams>>,
    /// `specialEventInformation` (Class)
    #[serde(default)]
    pub special_event_information: Option<Handle<EASpecialEventInformationParams>>,
}

impl Pooled for SEntityComponentFrontendEAUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.sentity_component_frontend_eauiprovider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.sentity_component_frontend_eauiprovider_params }
}

impl<'a> Extract<'a> for SEntityComponentFrontendEAUIProviderParams {
    const TYPE_NAME: &'static str = "SEntityComponentFrontendEAUIProviderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_network_type_names: inst.get_array("matchNetworkTypeNames")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMatchNetworkTypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SMatchNetworkTypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            any_map_option_name: inst.get_str("anyMapOptionName").map(LocaleKey::from).unwrap_or_default(),
            any_map_option_description: inst.get_str("anyMapOptionDescription").map(LocaleKey::from).unwrap_or_default(),
            any_map_option_thumbnail: inst.get_str("anyMapOptionThumbnail").map(String::from).unwrap_or_default(),
            any_team_option_name: inst.get_str("anyTeamOptionName").map(LocaleKey::from).unwrap_or_default(),
            any_team_option_description: inst.get_str("anyTeamOptionDescription").map(LocaleKey::from).unwrap_or_default(),
            loc_ready_count: inst.get_str("locReadyCount").map(LocaleKey::from).unwrap_or_default(),
            loc_error_ready_check: inst.get_str("locErrorReadyCheck").map(LocaleKey::from).unwrap_or_default(),
            loc_error_player_limit: inst.get_str("locErrorPlayerLimit").map(LocaleKey::from).unwrap_or_default(),
            loc_error_missing_badge: inst.get_str("locErrorMissingBadge").map(LocaleKey::from).unwrap_or_default(),
            loc_error_min_players: inst.get_str("locErrorMinPlayers").map(LocaleKey::from).unwrap_or_default(),
            featured_game_mode: EGameModeId::from_dcb_str(inst.get_str("featuredGameMode").unwrap_or("")),
            popup_params: match inst.get("popupParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PopupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            patch_note_categories: inst.get_array("patchNoteCategories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EAPatchNoteCategoryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EAPatchNoteCategoryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            social_tabs: inst.get_array("socialTabs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SocialTab>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SocialTab>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            experimental_mode_reset_time: match inst.get("experimentalModeResetTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EAExperimentalModeResetTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            special_event_information: match inst.get("specialEventInformation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EASpecialEventInformationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityComponentFrontendPUUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentFrontendPUUIProviderParams {
    /// `introTitleLocId` (Locale)
    #[serde(default)]
    pub intro_title_loc_id: LocaleKey,
    /// `introDescriptionLocId` (Locale)
    #[serde(default)]
    pub intro_description_loc_id: LocaleKey,
    /// `hangarLocId` (Locale)
    #[serde(default)]
    pub hangar_loc_id: LocaleKey,
    /// `hangarDescriptionLocId` (Locale)
    #[serde(default)]
    pub hangar_description_loc_id: LocaleKey,
    /// `hangarSystemImagePath` (String)
    #[serde(default)]
    pub hangar_system_image_path: String,
    /// `regionLocalizationIdentifiers` (Locale (array))
    #[serde(default)]
    pub region_localization_identifiers: Vec<LocaleKey>,
    /// `soloplayWarningLocId` (Locale)
    #[serde(default)]
    pub soloplay_warning_loc_id: LocaleKey,
    /// `soloplayWarningConfirmLocId` (Locale)
    #[serde(default)]
    pub soloplay_warning_confirm_loc_id: LocaleKey,
    /// `soloplayWarningCancelLocId` (Locale)
    #[serde(default)]
    pub soloplay_warning_cancel_loc_id: LocaleKey,
    /// `firstSpawnWarningLocId` (Locale)
    #[serde(default)]
    pub first_spawn_warning_loc_id: LocaleKey,
    /// `firstSpawnWarningConfirmLocId` (Locale)
    #[serde(default)]
    pub first_spawn_warning_confirm_loc_id: LocaleKey,
    /// `firstSpawnWarningCancelLocId` (Locale)
    #[serde(default)]
    pub first_spawn_warning_cancel_loc_id: LocaleKey,
}

impl Pooled for EntityComponentFrontendPUUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.entity_component_frontend_puuiprovider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.entity_component_frontend_puuiprovider_params }
}

impl<'a> Extract<'a> for EntityComponentFrontendPUUIProviderParams {
    const TYPE_NAME: &'static str = "EntityComponentFrontendPUUIProviderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            intro_title_loc_id: inst.get_str("introTitleLocId").map(LocaleKey::from).unwrap_or_default(),
            intro_description_loc_id: inst.get_str("introDescriptionLocId").map(LocaleKey::from).unwrap_or_default(),
            hangar_loc_id: inst.get_str("hangarLocId").map(LocaleKey::from).unwrap_or_default(),
            hangar_description_loc_id: inst.get_str("hangarDescriptionLocId").map(LocaleKey::from).unwrap_or_default(),
            hangar_system_image_path: inst.get_str("hangarSystemImagePath").map(String::from).unwrap_or_default(),
            region_localization_identifiers: inst.get_array("regionLocalizationIdentifiers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(LocaleKey::from)).collect())
                .unwrap_or_default(),
            soloplay_warning_loc_id: inst.get_str("soloplayWarningLocId").map(LocaleKey::from).unwrap_or_default(),
            soloplay_warning_confirm_loc_id: inst.get_str("soloplayWarningConfirmLocId").map(LocaleKey::from).unwrap_or_default(),
            soloplay_warning_cancel_loc_id: inst.get_str("soloplayWarningCancelLocId").map(LocaleKey::from).unwrap_or_default(),
            first_spawn_warning_loc_id: inst.get_str("firstSpawnWarningLocId").map(LocaleKey::from).unwrap_or_default(),
            first_spawn_warning_confirm_loc_id: inst.get_str("firstSpawnWarningConfirmLocId").map(LocaleKey::from).unwrap_or_default(),
            first_spawn_warning_cancel_loc_id: inst.get_str("firstSpawnWarningCancelLocId").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SFrontendGameModeButton`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFrontendGameModeButton {
    /// `buttonEnum` (EnumChoice)
    #[serde(default)]
    pub button_enum: EFrontendGameModeButton,
    /// `buttonTitle` (Locale)
    #[serde(default)]
    pub button_title: LocaleKey,
    /// `buttonDescription` (Locale)
    #[serde(default)]
    pub button_description: LocaleKey,
    /// `imagePath` (String)
    #[serde(default)]
    pub image_path: String,
    /// `moviePath` (String)
    #[serde(default)]
    pub movie_path: String,
    /// `isActive` (Boolean)
    #[serde(default)]
    pub is_active: bool,
}

impl Pooled for SFrontendGameModeButton {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.sfrontend_game_mode_button }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.sfrontend_game_mode_button }
}

impl<'a> Extract<'a> for SFrontendGameModeButton {
    const TYPE_NAME: &'static str = "SFrontendGameModeButton";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            button_enum: EFrontendGameModeButton::from_dcb_str(inst.get_str("buttonEnum").unwrap_or("")),
            button_title: inst.get_str("buttonTitle").map(LocaleKey::from).unwrap_or_default(),
            button_description: inst.get_str("buttonDescription").map(LocaleKey::from).unwrap_or_default(),
            image_path: inst.get_str("imagePath").map(String::from).unwrap_or_default(),
            movie_path: inst.get_str("moviePath").map(String::from).unwrap_or_default(),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
        }
    }
}

/// DCB type: `FrontendOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendOverrideParams {
    /// `persistentUniverseActive` (Boolean)
    #[serde(default)]
    pub persistent_universe_active: bool,
    /// `arenaCommanderActive` (Boolean)
    #[serde(default)]
    pub arena_commander_active: bool,
    /// `tutorialDisabled` (Boolean)
    #[serde(default)]
    pub tutorial_disabled: bool,
    /// `disableResidenceSelectionWarning` (Boolean)
    #[serde(default)]
    pub disable_residence_selection_warning: bool,
    /// `backgroundVideoPath` (String)
    #[serde(default)]
    pub background_video_path: String,
    /// `disabledSystems` (Reference (array))
    #[serde(default)]
    pub disabled_systems: Vec<CigGuid>,
}

impl Pooled for FrontendOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.frontend_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.frontend_override_params }
}

impl<'a> Extract<'a> for FrontendOverrideParams {
    const TYPE_NAME: &'static str = "FrontendOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            persistent_universe_active: inst.get_bool("persistentUniverseActive").unwrap_or_default(),
            arena_commander_active: inst.get_bool("arenaCommanderActive").unwrap_or_default(),
            tutorial_disabled: inst.get_bool("tutorialDisabled").unwrap_or_default(),
            disable_residence_selection_warning: inst.get_bool("disableResidenceSelectionWarning").unwrap_or_default(),
            background_video_path: inst.get_str("backgroundVideoPath").map(String::from).unwrap_or_default(),
            disabled_systems: inst.get_array("disabledSystems")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentFrontendUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentFrontendUIProviderParams {
    /// `overrideFrontendParam` (Reference)
    #[serde(default)]
    pub override_frontend_param: Option<CigGuid>,
    /// `frontendGameModeButtons` (Class (array))
    #[serde(default)]
    pub frontend_game_mode_buttons: Vec<Handle<SFrontendGameModeButton>>,
}

impl Pooled for EntityComponentFrontendUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.entity_component_frontend_uiprovider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.entity_component_frontend_uiprovider_params }
}

impl<'a> Extract<'a> for EntityComponentFrontendUIProviderParams {
    const TYPE_NAME: &'static str = "EntityComponentFrontendUIProviderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            override_frontend_param: inst.get("overrideFrontendParam").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            frontend_game_mode_buttons: inst.get_array("frontendGameModeButtons")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SFrontendGameModeButton>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SFrontendGameModeButton>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FrontendControllerProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendControllerProviderParams {
}

impl Pooled for FrontendControllerProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.frontend_controller_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.frontend_controller_provider_params }
}

impl<'a> Extract<'a> for FrontendControllerProviderParams {
    const TYPE_NAME: &'static str = "FrontendControllerProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityComponentRttLiveCameraPreviewParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityComponentRttLiveCameraPreviewParams {
    /// `width` (Int32)
    #[serde(default)]
    pub width: i32,
    /// `height` (Int32)
    #[serde(default)]
    pub height: i32,
    /// `fov` (Int32)
    #[serde(default)]
    pub fov: i32,
    /// `near` (Single)
    #[serde(default)]
    pub near: f32,
    /// `far` (Single)
    #[serde(default)]
    pub far: f32,
}

impl Pooled for SEntityComponentRttLiveCameraPreviewParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.sentity_component_rtt_live_camera_preview_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.sentity_component_rtt_live_camera_preview_params }
}

impl<'a> Extract<'a> for SEntityComponentRttLiveCameraPreviewParams {
    const TYPE_NAME: &'static str = "SEntityComponentRttLiveCameraPreviewParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            width: inst.get_i32("width").unwrap_or_default(),
            height: inst.get_i32("height").unwrap_or_default(),
            fov: inst.get_i32("fov").unwrap_or_default(),
            near: inst.get_f32("near").unwrap_or_default(),
            far: inst.get_f32("far").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIInteriorMapSectionParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIInteriorMapSectionParams {
    /// `name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// `restrictViewBounds` (Boolean)
    #[serde(default)]
    pub restrict_view_bounds: bool,
}

impl Pooled for UIInteriorMapSectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.uiinterior_map_section_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.uiinterior_map_section_params }
}

impl<'a> Extract<'a> for UIInteriorMapSectionParams {
    const TYPE_NAME: &'static str = "UIInteriorMapSectionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            restrict_view_bounds: inst.get_bool("restrictViewBounds").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIInteriorMapLabelParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIInteriorMapLabelParams {
    /// `name` (Locale)
    #[serde(default)]
    pub name: LocaleKey,
    /// `textScale` (Single)
    #[serde(default)]
    pub text_scale: f32,
    /// `maxWidth` (Single)
    #[serde(default)]
    pub max_width: f32,
}

impl Pooled for UIInteriorMapLabelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.uiinterior_map_label_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.uiinterior_map_label_params }
}

impl<'a> Extract<'a> for UIInteriorMapLabelParams {
    const TYPE_NAME: &'static str = "UIInteriorMapLabelParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
            text_scale: inst.get_f32("textScale").unwrap_or_default(),
            max_width: inst.get_f32("maxWidth").unwrap_or_default(),
        }
    }
}

/// DCB type: `TelevisionScreenParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelevisionScreenParams {
    /// `nextInteraction` (WeakPointer)
    #[serde(default)]
    pub next_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `previousInteraction` (WeakPointer)
    #[serde(default)]
    pub previous_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `movieList` (Reference)
    #[serde(default)]
    pub movie_list: Option<CigGuid>,
}

impl Pooled for TelevisionScreenParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.television_screen_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.television_screen_params }
}

impl<'a> Extract<'a> for TelevisionScreenParams {
    const TYPE_NAME: &'static str = "TelevisionScreenParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            next_interaction: match inst.get("nextInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            previous_interaction: match inst.get("previousInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            movie_list: inst.get("movieList").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

