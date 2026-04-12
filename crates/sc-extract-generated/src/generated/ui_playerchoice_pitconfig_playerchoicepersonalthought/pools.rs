// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-playerchoice_pitconfig_playerchoicepersonalthought` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiPlayerchoice_pitconfig_playerchoicepersonalthoughtPools {
    #[serde(default)]
    pub personal_thought_camera_effects_params: Vec<Option<PersonalThoughtCameraEffectsParams>>,
    #[serde(default)]
    pub action_rule_list: Vec<Option<ActionRuleList>>,
    #[serde(default)]
    pub personal_thought_actions_rules_params: Vec<Option<PersonalThoughtActionsRulesParams>>,
    #[serde(default)]
    pub type_subtype_params: Vec<Option<TypeSubtypeParams>>,
    #[serde(default)]
    pub personal_thought_action_description: Vec<Option<PersonalThoughtActionDescription>>,
    #[serde(default)]
    pub personal_thought_action_descriptions_list: Vec<Option<PersonalThoughtActionDescriptionsList>>,
    #[serde(default)]
    pub personal_thought_hologram_actions_list: Vec<Option<PersonalThoughtHologramActionsList>>,
    #[serde(default)]
    pub personal_thought_option: Vec<Option<PersonalThoughtOption>>,
    #[serde(default)]
    pub personal_thought_category_action: Vec<Option<PersonalThoughtCategoryAction>>,
    #[serde(default)]
    pub personal_thought_inventory_filter: Vec<Option<PersonalThoughtInventoryFilter>>,
    #[serde(default)]
    pub looting_tab_params: Vec<Option<LootingTabParams>>,
    #[serde(default)]
    pub looting_inventory_params: Vec<Option<LootingInventoryParams>>,
    #[serde(default)]
    pub personal_thought_inventory_params: Vec<Option<PersonalThoughtInventoryParams>>,
    #[serde(default)]
    pub looting_item_port_size_class: Vec<Option<LootingItemPortSizeClass>>,
    #[serde(default)]
    pub proximity_inventory_detection_params: Vec<Option<ProximityInventoryDetectionParams>>,
    #[serde(default)]
    pub inventory_sort_mode: Vec<Option<InventorySortMode>>,
    #[serde(default)]
    pub personal_thought_inventory_sort_menu: Vec<Option<PersonalThoughtInventorySortMenu>>,
    #[serde(default)]
    pub personal_thought_looting_screen_params: Vec<Option<PersonalThoughtLootingScreenParams>>,
    #[serde(default)]
    pub personal_thought_pop_window_params: Vec<Option<PersonalThoughtPopWindowParams>>,
    #[serde(default)]
    pub personal_thought_inventory_actions_params: Vec<Option<PersonalThoughtInventoryActionsParams>>,
    #[serde(default)]
    pub personal_thought_inventory_grid_params: Vec<Option<PersonalThoughtInventoryGridParams>>,
    #[serde(default)]
    pub personal_thought_inventory_item_orientation_offset: Vec<Option<PersonalThoughtInventoryItemOrientationOffset>>,
    #[serde(default)]
    pub personal_thought_inventory_item_light_group: Vec<Option<PersonalThoughtInventoryItemLightGroup>>,
    #[serde(default)]
    pub personal_thought_inventory_item_uiicon: Vec<Option<PersonalThoughtInventoryItemUIIcon>>,
    #[serde(default)]
    pub inventory_container_params: Vec<Option<InventoryContainerParams>>,
    #[serde(default)]
    pub personal_thought_contextual_actions_menu: Vec<Option<PersonalThoughtContextualActionsMenu>>,
    #[serde(default)]
    pub personal_thought_game_mode_def: Vec<Option<PersonalThoughtGameModeDef>>,
    #[serde(default)]
    pub personal_thought_contextual_actions_menus_params: Vec<Option<PersonalThoughtContextualActionsMenusParams>>,
    #[serde(default)]
    pub item_category: Vec<Option<ItemCategory>>,
    #[serde(default)]
    pub quick_access_wheel_element: Vec<Option<QuickAccessWheelElement>>,
    #[serde(default)]
    pub personal_thought_quick_access_wheel: Vec<Option<PersonalThoughtQuickAccessWheel>>,
    #[serde(default)]
    pub personal_thought_quick_access_wheels: Vec<Option<PersonalThoughtQuickAccessWheels>>,
    #[serde(default)]
    pub personal_thought_hologram_animation_params: Vec<Option<PersonalThoughtHologramAnimationParams>>,
    #[serde(default)]
    pub personal_thought_hologram_params: Vec<Option<PersonalThoughtHologramParams>>,
    #[serde(default)]
    pub personal_thought_force_close_action_list: Vec<Option<PersonalThoughtForceCloseActionList>>,
    #[serde(default)]
    pub personal_thought_force_close_actions_params: Vec<Option<PersonalThoughtForceCloseActionsParams>>,
    #[serde(default)]
    pub inventory_drop_detach_rules: Vec<Option<InventoryDropDetachRules>>,
    #[serde(default)]
    pub player_choice_pitconfig: Vec<Option<PlayerChoice_PITConfig>>,
}
