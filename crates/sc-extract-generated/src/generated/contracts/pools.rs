// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `contracts` feature.
#[derive(Default)]
pub struct ContractsPools {
    pub mission_location_validation_entity_tags: Vec<Option<MissionLocationValidation_EntityTags>>,
    pub contract_generator_handler_linear_series:
        Vec<Option<ContractGeneratorHandler_LinearSeries>>,
    pub contract_generator_handler_tutorial_series_def:
        Vec<Option<ContractGeneratorHandler_TutorialSeriesDef>>,
    pub contract_generator_handler_list: Vec<Option<ContractGeneratorHandler_List>>,
    pub contract_generator_handler_legacy: Vec<Option<ContractGeneratorHandler_Legacy>>,
    pub contract_legacy: Vec<Option<ContractLegacy>>,
    pub career_contract: Vec<Option<CareerContract>>,
    pub contract_generator_handler_career: Vec<Option<ContractGeneratorHandler_Career>>,
    pub scontract_plugin_sscenario_progress: Vec<Option<SContractPlugin_SScenarioProgress>>,
    pub contract_prerequisite_locality: Vec<Option<ContractPrerequisite_Locality>>,
    pub contract_prerequisite_location: Vec<Option<ContractPrerequisite_Location>>,
    pub contract_prerequisite_location_property: Vec<Option<ContractPrerequisite_LocationProperty>>,
    pub contract_prerequisite_crime_stat: Vec<Option<ContractPrerequisite_CrimeStat>>,
    pub contract_prerequisite_reputation: Vec<Option<ContractPrerequisite_Reputation>>,
    pub contract_prerequisite_completed_contract_tags:
        Vec<Option<ContractPrerequisite_CompletedContractTags>>,
    pub contract_result_reward: Vec<Option<ContractResult_Reward>>,
    pub contract_result_legacy_reputation: Vec<Option<ContractResult_LegacyReputation>>,
    pub contract_result_calculated_reputation: Vec<Option<ContractResult_CalculatedReputation>>,
    pub contract_result_journal_entry: Vec<Option<ContractResult_JournalEntry>>,
    pub contract_result_refund_buy_in: Vec<Option<ContractResult_RefundBuyIn>>,
    pub contract_result_completion_tag: Vec<Option<ContractResult_CompletionTag>>,
    pub contract_result_completion_tags: Vec<Option<ContractResult_CompletionTags>>,
    pub min_completion_tags: Vec<Option<MinCompletionTags>>,
    pub contract_result_badge_award: Vec<Option<ContractResult_BadgeAward>>,
    pub contract_result_item: Vec<Option<ContractResult_Item>>,
    pub contract_result_completion_bounty: Vec<Option<ContractResult_CompletionBounty>>,
    pub item_award_entity_class: Vec<Option<ItemAwardEntityClass>>,
    pub item_award_weightings_params: Vec<Option<ItemAwardWeightingsParams>>,
    pub contract_result_items_weighting: Vec<Option<ContractResult_ItemsWeighting>>,
    pub contract_result_scenario_progress: Vec<Option<ContractResult_ScenarioProgress>>,
    pub reward_notification: Vec<Option<RewardNotification>>,
    pub blueprint_rewards: Vec<Option<BlueprintRewards>>,
    pub contract_auto_finish_settings: Vec<Option<ContractAutoFinishSettings>>,
    pub active_contract_settings: Vec<Option<ActiveContractSettings>>,
    pub contract_class_contract: Vec<Option<ContractClass_Contract>>,
    pub contract_class_pvpbounty: Vec<Option<ContractClass_PVPBounty>>,
    pub hauling_order_content_resource: Vec<Option<HaulingOrderContent_Resource>>,
    pub hauling_order_content_entity_classes: Vec<Option<HaulingOrderContent_EntityClasses>>,
    pub hauling_order_content_entity_class: Vec<Option<HaulingOrderContent_EntityClass>>,
    pub hauling_order_content_mission_item: Vec<Option<HaulingOrderContent_MissionItem>>,
    pub hauling_order_property: Vec<Option<HaulingOrder_Property>>,
    pub hauling_order_resource_unlimited_drop_off:
        Vec<Option<HaulingOrder_ResourceUnlimitedDropOff>>,
    pub hauling_order_entity_classes: Vec<Option<HaulingOrder_EntityClasses>>,
    pub hauling_order_entity_class: Vec<Option<HaulingOrder_EntityClass>>,
    pub hauling_order_mission_item: Vec<Option<HaulingOrder_MissionItem>>,
    pub hauling_order_or_option_and: Vec<Option<HaulingOrder_OrOption_And>>,
    pub hauling_order_content_or: Vec<Option<HaulingOrderContent_Or>>,
    pub mission_flow_condition_all_tokens_state: Vec<Option<MissionFlowCondition_AllTokensState>>,
    pub mission_flow_condition_any_tokens_state: Vec<Option<MissionFlowCondition_AnyTokensState>>,
    pub mission_flow_condition_on_mission_start: Vec<Option<MissionFlowCondition_OnMissionStart>>,
    pub mission_flow_condition_or: Vec<Option<MissionFlowCondition_OR>>,
    pub mission_random_phase_entry: Vec<Option<MissionRandomPhaseEntry>>,
    pub mission_flow_action_pick_random_mission_phase:
        Vec<Option<MissionFlowAction_PickRandomMissionPhase>>,
    pub mission_property_value_locations: Vec<Option<MissionPropertyValue_Locations>>,
    pub mission_property_value_hauling_orders: Vec<Option<MissionPropertyValue_HaulingOrders>>,
    pub mission_module_hierarchy_sub_mission: Vec<Option<MissionModuleHierarchySubMission>>,
    pub mission_module_hierarchy: Vec<Option<MissionModuleHierarchy>>,
    pub objective_handler_local: Vec<Option<ObjectiveHandler_Local>>,
    pub objective_handler_near_location: Vec<Option<ObjectiveHandler_NearLocation>>,
    pub contract_generator_handler_pvpbounty_def:
        Vec<Option<ContractGeneratorHandler_PVPBountyDef>>,
    pub spvpbounty_contract_generators: Vec<Option<SPVPBountyContractGenerators>>,
    pub global_mission_settings: Vec<Option<GlobalMissionSettings>>,
}
