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

/// Pool storage for the `contracts` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractsPools {
    #[serde(default)]
    pub mission_location_validation_entity_tags: Vec<Option<MissionLocationValidation_EntityTags>>,
    #[serde(default)]
    pub contract_generator_handler_linear_series: Vec<Option<ContractGeneratorHandler_LinearSeries>>,
    #[serde(default)]
    pub contract_generator_handler_tutorial_series_def: Vec<Option<ContractGeneratorHandler_TutorialSeriesDef>>,
    #[serde(default)]
    pub contract_generator_handler_list: Vec<Option<ContractGeneratorHandler_List>>,
    #[serde(default)]
    pub contract_generator_handler_legacy: Vec<Option<ContractGeneratorHandler_Legacy>>,
    #[serde(default)]
    pub contract_legacy: Vec<Option<ContractLegacy>>,
    #[serde(default)]
    pub career_contract: Vec<Option<CareerContract>>,
    #[serde(default)]
    pub contract_generator_handler_career: Vec<Option<ContractGeneratorHandler_Career>>,
    #[serde(default)]
    pub scontract_plugin_sscenario_progress: Vec<Option<SContractPlugin_SScenarioProgress>>,
    #[serde(default)]
    pub contract_prerequisite_locality: Vec<Option<ContractPrerequisite_Locality>>,
    #[serde(default)]
    pub contract_prerequisite_location: Vec<Option<ContractPrerequisite_Location>>,
    #[serde(default)]
    pub contract_prerequisite_location_property: Vec<Option<ContractPrerequisite_LocationProperty>>,
    #[serde(default)]
    pub contract_prerequisite_crime_stat: Vec<Option<ContractPrerequisite_CrimeStat>>,
    #[serde(default)]
    pub contract_prerequisite_reputation: Vec<Option<ContractPrerequisite_Reputation>>,
    #[serde(default)]
    pub contract_prerequisite_completed_contract_tags: Vec<Option<ContractPrerequisite_CompletedContractTags>>,
    #[serde(default)]
    pub contract_result_reward: Vec<Option<ContractResult_Reward>>,
    #[serde(default)]
    pub contract_result_legacy_reputation: Vec<Option<ContractResult_LegacyReputation>>,
    #[serde(default)]
    pub contract_result_calculated_reputation: Vec<Option<ContractResult_CalculatedReputation>>,
    #[serde(default)]
    pub contract_result_journal_entry: Vec<Option<ContractResult_JournalEntry>>,
    #[serde(default)]
    pub contract_result_refund_buy_in: Vec<Option<ContractResult_RefundBuyIn>>,
    #[serde(default)]
    pub contract_result_completion_tag: Vec<Option<ContractResult_CompletionTag>>,
    #[serde(default)]
    pub contract_result_completion_tags: Vec<Option<ContractResult_CompletionTags>>,
    #[serde(default)]
    pub min_completion_tags: Vec<Option<MinCompletionTags>>,
    #[serde(default)]
    pub contract_result_badge_award: Vec<Option<ContractResult_BadgeAward>>,
    #[serde(default)]
    pub contract_result_item: Vec<Option<ContractResult_Item>>,
    #[serde(default)]
    pub contract_result_completion_bounty: Vec<Option<ContractResult_CompletionBounty>>,
    #[serde(default)]
    pub item_award_entity_class: Vec<Option<ItemAwardEntityClass>>,
    #[serde(default)]
    pub item_award_weightings_params: Vec<Option<ItemAwardWeightingsParams>>,
    #[serde(default)]
    pub contract_result_items_weighting: Vec<Option<ContractResult_ItemsWeighting>>,
    #[serde(default)]
    pub contract_result_scenario_progress: Vec<Option<ContractResult_ScenarioProgress>>,
    #[serde(default)]
    pub reward_notification: Vec<Option<RewardNotification>>,
    #[serde(default)]
    pub blueprint_rewards: Vec<Option<BlueprintRewards>>,
    #[serde(default)]
    pub contract_auto_finish_settings: Vec<Option<ContractAutoFinishSettings>>,
    #[serde(default)]
    pub active_contract_settings: Vec<Option<ActiveContractSettings>>,
    #[serde(default)]
    pub contract_class_contract: Vec<Option<ContractClass_Contract>>,
    #[serde(default)]
    pub contract_class_pvpbounty: Vec<Option<ContractClass_PVPBounty>>,
    #[serde(default)]
    pub hauling_order_content_resource: Vec<Option<HaulingOrderContent_Resource>>,
    #[serde(default)]
    pub hauling_order_content_entity_classes: Vec<Option<HaulingOrderContent_EntityClasses>>,
    #[serde(default)]
    pub hauling_order_content_entity_class: Vec<Option<HaulingOrderContent_EntityClass>>,
    #[serde(default)]
    pub hauling_order_content_mission_item: Vec<Option<HaulingOrderContent_MissionItem>>,
    #[serde(default)]
    pub hauling_order_property: Vec<Option<HaulingOrder_Property>>,
    #[serde(default)]
    pub hauling_order_resource_unlimited_drop_off: Vec<Option<HaulingOrder_ResourceUnlimitedDropOff>>,
    #[serde(default)]
    pub hauling_order_entity_classes: Vec<Option<HaulingOrder_EntityClasses>>,
    #[serde(default)]
    pub hauling_order_entity_class: Vec<Option<HaulingOrder_EntityClass>>,
    #[serde(default)]
    pub hauling_order_mission_item: Vec<Option<HaulingOrder_MissionItem>>,
    #[serde(default)]
    pub hauling_order_or_option_and: Vec<Option<HaulingOrder_OrOption_And>>,
    #[serde(default)]
    pub hauling_order_content_or: Vec<Option<HaulingOrderContent_Or>>,
    #[serde(default)]
    pub mission_flow_condition_all_tokens_state: Vec<Option<MissionFlowCondition_AllTokensState>>,
    #[serde(default)]
    pub mission_flow_condition_any_tokens_state: Vec<Option<MissionFlowCondition_AnyTokensState>>,
    #[serde(default)]
    pub mission_flow_condition_on_mission_start: Vec<Option<MissionFlowCondition_OnMissionStart>>,
    #[serde(default)]
    pub mission_flow_condition_or: Vec<Option<MissionFlowCondition_OR>>,
    #[serde(default)]
    pub mission_random_phase_entry: Vec<Option<MissionRandomPhaseEntry>>,
    #[serde(default)]
    pub mission_flow_action_pick_random_mission_phase: Vec<Option<MissionFlowAction_PickRandomMissionPhase>>,
    #[serde(default)]
    pub mission_property_value_locations: Vec<Option<MissionPropertyValue_Locations>>,
    #[serde(default)]
    pub mission_property_value_hauling_orders: Vec<Option<MissionPropertyValue_HaulingOrders>>,
    #[serde(default)]
    pub mission_module_hierarchy_sub_mission: Vec<Option<MissionModuleHierarchySubMission>>,
    #[serde(default)]
    pub mission_module_hierarchy: Vec<Option<MissionModuleHierarchy>>,
    #[serde(default)]
    pub objective_handler_local: Vec<Option<ObjectiveHandler_Local>>,
    #[serde(default)]
    pub objective_handler_near_location: Vec<Option<ObjectiveHandler_NearLocation>>,
    #[serde(default)]
    pub contract_generator_handler_pvpbounty_def: Vec<Option<ContractGeneratorHandler_PVPBountyDef>>,
    #[serde(default)]
    pub spvpbounty_contract_generators: Vec<Option<SPVPBountyContractGenerators>>,
    #[serde(default)]
    pub global_mission_settings: Vec<Option<GlobalMissionSettings>>,
}
