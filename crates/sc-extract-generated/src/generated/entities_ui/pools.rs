// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-ui` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesUiPools {
    #[serde(default)]
    pub record_ref_user_variable_type_font_style: Vec<Option<RecordRefUserVariableTypeFontStyle>>,
    #[serde(default)]
    pub spawn_notifier_entity_component_params: Vec<Option<SpawnNotifierEntityComponentParams>>,
    #[serde(default)]
    pub smatch_network_type_params: Vec<Option<SMatchNetworkTypeParams>>,
    #[serde(default)]
    pub popup_params: Vec<Option<PopupParams>>,
    #[serde(default)]
    pub eapatch_note_params: Vec<Option<EAPatchNoteParams>>,
    #[serde(default)]
    pub eapatch_note_category_params: Vec<Option<EAPatchNoteCategoryParams>>,
    #[serde(default)]
    pub social_tab: Vec<Option<SocialTab>>,
    #[serde(default)]
    pub eaexperimental_mode_reset_time_params: Vec<Option<EAExperimentalModeResetTimeParams>>,
    #[serde(default)]
    pub easpecial_event_information_params: Vec<Option<EASpecialEventInformationParams>>,
    #[serde(default)]
    pub sentity_component_frontend_eauiprovider_params: Vec<Option<SEntityComponentFrontendEAUIProviderParams>>,
    #[serde(default)]
    pub entity_component_frontend_puuiprovider_params: Vec<Option<EntityComponentFrontendPUUIProviderParams>>,
    #[serde(default)]
    pub sfrontend_game_mode_button: Vec<Option<SFrontendGameModeButton>>,
    #[serde(default)]
    pub frontend_override_params: Vec<Option<FrontendOverrideParams>>,
    #[serde(default)]
    pub entity_component_frontend_uiprovider_params: Vec<Option<EntityComponentFrontendUIProviderParams>>,
    #[serde(default)]
    pub frontend_controller_provider_params: Vec<Option<FrontendControllerProviderParams>>,
    #[serde(default)]
    pub sentity_component_rtt_live_camera_preview_params: Vec<Option<SEntityComponentRttLiveCameraPreviewParams>>,
    #[serde(default)]
    pub uiinterior_map_section_params: Vec<Option<UIInteriorMapSectionParams>>,
    #[serde(default)]
    pub uiinterior_map_label_params: Vec<Option<UIInteriorMapLabelParams>>,
    #[serde(default)]
    pub television_screen_params: Vec<Option<TelevisionScreenParams>>,
}
