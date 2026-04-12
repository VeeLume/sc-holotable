// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missionbroker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Date`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date {
    /// `day` (Int32)
    #[serde(default)]
    pub day: i32,
    /// `month` (EnumChoice)
    #[serde(default)]
    pub month: String,
    /// `year` (Int32)
    #[serde(default)]
    pub year: i32,
}

impl Pooled for Date {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.date }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.date }
}

impl<'a> Extract<'a> for Date {
    const TYPE_NAME: &'static str = "Date";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            day: inst.get_i32("day").unwrap_or_default(),
            month: inst.get_str("month").map(String::from).unwrap_or_default(),
            year: inst.get_i32("year").unwrap_or_default(),
        }
    }
}

/// DCB type: `Time`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    /// `hour` (Int32)
    #[serde(default)]
    pub hour: i32,
    /// `minute` (Int32)
    #[serde(default)]
    pub minute: i32,
    /// `second` (Int32)
    #[serde(default)]
    pub second: i32,
}

impl Pooled for Time {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.time }
}

impl<'a> Extract<'a> for Time {
    const TYPE_NAME: &'static str = "Time";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hour: inst.get_i32("hour").unwrap_or_default(),
            minute: inst.get_i32("minute").unwrap_or_default(),
            second: inst.get_i32("second").unwrap_or_default(),
        }
    }
}

/// DCB type: `DateTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
    /// `date` (Class)
    #[serde(default)]
    pub date: Option<Handle<Date>>,
    /// `time` (Class)
    #[serde(default)]
    pub time: Option<Handle<Time>>,
}

impl Pooled for DateTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.date_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.date_time }
}

impl<'a> Extract<'a> for DateTime {
    const TYPE_NAME: &'static str = "DateTime";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            date: match inst.get("date") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Date>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Date>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time: match inst.get("time") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Time>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Time>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DateTimeSchedule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeSchedule {
    /// `start` (Class)
    #[serde(default)]
    pub start: Option<Handle<DateTime>>,
    /// `end` (Class)
    #[serde(default)]
    pub end: Option<Handle<DateTime>>,
    /// `repeatType` (EnumChoice)
    #[serde(default)]
    pub repeat_type: String,
    /// `repeatEnd` (Class)
    #[serde(default)]
    pub repeat_end: Option<Handle<Date>>,
}

impl Pooled for DateTimeSchedule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.date_time_schedule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.date_time_schedule }
}

impl<'a> Extract<'a> for DateTimeSchedule {
    const TYPE_NAME: &'static str = "DateTimeSchedule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start: match inst.get("start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DateTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DateTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end: match inst.get("end") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DateTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DateTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            repeat_type: inst.get_str("repeatType").map(String::from).unwrap_or_default(),
            repeat_end: match inst.get("repeatEnd") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Date>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Date>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ReputationPrerequisiteRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationPrerequisiteRange {
    /// `minValue` (Single)
    #[serde(default)]
    pub min_value: f32,
    /// `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
}

impl Pooled for ReputationPrerequisiteRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.reputation_prerequisite_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.reputation_prerequisite_range }
}

impl<'a> Extract<'a> for ReputationPrerequisiteRange {
    const TYPE_NAME: &'static str = "ReputationPrerequisiteRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `ReputationPrerequisites`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationPrerequisites {
    /// `wantedLevelJurisdictionOverride` (Reference)
    #[serde(default)]
    pub wanted_level_jurisdiction_override: Option<CigGuid>,
    /// `wantedLevel` (Class)
    #[serde(default)]
    pub wanted_level: Option<Handle<ReputationPrerequisiteRange>>,
}

impl Pooled for ReputationPrerequisites {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.reputation_prerequisites }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.reputation_prerequisites }
}

impl<'a> Extract<'a> for ReputationPrerequisites {
    const TYPE_NAME: &'static str = "ReputationPrerequisites";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wanted_level_jurisdiction_override: inst.get("wantedLevelJurisdictionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            wanted_level: match inst.get("wantedLevel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ReputationPrerequisiteRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReputationPrerequisiteRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionDeadline`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDeadline {
    /// `missionCompletionTime` (Single)
    #[serde(default)]
    pub mission_completion_time: f32,
    /// `missionAutoEnd` (Boolean)
    #[serde(default)]
    pub mission_auto_end: bool,
    /// `missionResultAfterTimerEnd` (EnumChoice)
    #[serde(default)]
    pub mission_result_after_timer_end: String,
    /// `remainingTimeToShowTimer` (Single)
    #[serde(default)]
    pub remaining_time_to_show_timer: f32,
    /// `missionEndReason` (Locale)
    #[serde(default)]
    pub mission_end_reason: String,
}

impl Pooled for MissionDeadline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.mission_deadline }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.mission_deadline }
}

impl<'a> Extract<'a> for MissionDeadline {
    const TYPE_NAME: &'static str = "MissionDeadline";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_completion_time: inst.get_f32("missionCompletionTime").unwrap_or_default(),
            mission_auto_end: inst.get_bool("missionAutoEnd").unwrap_or_default(),
            mission_result_after_timer_end: inst.get_str("missionResultAfterTimerEnd").map(String::from).unwrap_or_default(),
            remaining_time_to_show_timer: inst.get_f32("remainingTimeToShowTimer").unwrap_or_default(),
            mission_end_reason: inst.get_str("missionEndReason").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionReward {
    /// `reward` (Int32)
    #[serde(default)]
    pub reward: i32,
    /// `max` (Int32)
    #[serde(default)]
    pub max: i32,
    /// `plusBonuses` (Boolean)
    #[serde(default)]
    pub plus_bonuses: bool,
    /// `currencyType` (EnumChoice)
    #[serde(default)]
    pub currency_type: String,
    /// `reputationBonus` (Reference)
    #[serde(default)]
    pub reputation_bonus: Option<CigGuid>,
}

impl Pooled for MissionReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.mission_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.mission_reward }
}

impl<'a> Extract<'a> for MissionReward {
    const TYPE_NAME: &'static str = "MissionReward";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reward: inst.get_i32("reward").unwrap_or_default(),
            max: inst.get_i32("max").unwrap_or_default(),
            plus_bonuses: inst.get_bool("plusBonuses").unwrap_or_default(),
            currency_type: inst.get_str("currencyType").map(String::from).unwrap_or_default(),
            reputation_bonus: inst.get("reputationBonus").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MinRequiredMissions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinRequiredMissions {
    /// `minRequiredCompletedMissions` (Int32)
    #[serde(default)]
    pub min_required_completed_missions: i32,
    /// `completionTags` (Class)
    #[serde(default)]
    pub completion_tags: Option<Handle<TagList>>,
    /// `requiredMissions` (Reference (array))
    #[serde(default)]
    pub required_missions: Vec<CigGuid>,
    /// `journalEntryLabel` (Locale)
    #[serde(default)]
    pub journal_entry_label: String,
}

impl Pooled for MinRequiredMissions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.min_required_missions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.min_required_missions }
}

impl<'a> Extract<'a> for MinRequiredMissions {
    const TYPE_NAME: &'static str = "MinRequiredMissions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_required_completed_missions: inst.get_i32("minRequiredCompletedMissions").unwrap_or_default(),
            completion_tags: match inst.get("completionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            required_missions: inst.get_array("requiredMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entry_label: inst.get_str("journalEntryLabel").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionBrokerEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionBrokerEntry {
    /// `notForRelease` (Boolean)
    #[serde(default)]
    pub not_for_release: bool,
    /// `owner` (Reference)
    #[serde(default)]
    pub owner: Option<CigGuid>,
    /// `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// `playerFacingDebugName` (String)
    #[serde(default)]
    pub player_facing_debug_name: String,
    /// `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// `titleHUD` (Locale)
    #[serde(default)]
    pub title_hud: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `missionGiver` (Locale)
    #[serde(default)]
    pub mission_giver: String,
    /// `commsChannelName` (Locale)
    #[serde(default)]
    pub comms_channel_name: String,
    /// `type` (Reference)
    #[serde(default)]
    pub r#type: Option<CigGuid>,
    /// `associatedMissions` (Reference (array))
    #[serde(default)]
    pub associated_missions: Vec<CigGuid>,
    /// `missionDifficulty` (Int32)
    #[serde(default)]
    pub mission_difficulty: i32,
    /// `localityAvailable` (Reference)
    #[serde(default)]
    pub locality_available: Option<CigGuid>,
    /// `locationMissionAvailable` (Reference)
    #[serde(default)]
    pub location_mission_available: Option<CigGuid>,
    /// `availableDateSchedule` (Class (array))
    #[serde(default)]
    pub available_date_schedule: Vec<Handle<DateTimeSchedule>>,
    /// `onlyAvailableIfAllMissionsNotAvailable` (Reference (array))
    #[serde(default)]
    pub only_available_if_all_missions_not_available: Vec<CigGuid>,
    /// `missionReward` (Class)
    #[serde(default)]
    pub mission_reward: Option<Handle<MissionReward>>,
    /// `partialRewardPayout` (StrongPointer)
    #[serde(default)]
    pub partial_reward_payout: Option<Handle<PartialContractRewards>>,
    /// `missionResultReputationRewards` (Class)
    #[serde(default)]
    pub mission_result_reputation_rewards: Option<Handle<SReputationAmountListParams>>,
    /// `journalEntriesToAddOnComplete` (Reference (array))
    #[serde(default)]
    pub journal_entries_to_add_on_complete: Vec<CigGuid>,
    /// `journalEntriesToRemoveOnComplete` (Reference (array))
    #[serde(default)]
    pub journal_entries_to_remove_on_complete: Vec<CigGuid>,
    /// `initiallyActive` (Boolean)
    #[serde(default)]
    pub initially_active: bool,
    /// `notifyOnAvailable` (Boolean)
    #[serde(default)]
    pub notify_on_available: bool,
    /// `preShowObjectives` (Boolean)
    #[serde(default)]
    pub pre_show_objectives: bool,
    /// `showAsOffer` (Boolean)
    #[serde(default)]
    pub show_as_offer: bool,
    /// `missionBuyInAmount` (Int32)
    #[serde(default)]
    pub mission_buy_in_amount: i32,
    /// `refundBuyInOnWithdraw` (Boolean)
    #[serde(default)]
    pub refund_buy_in_on_withdraw: bool,
    /// `hasCompleteButton` (Boolean)
    #[serde(default)]
    pub has_complete_button: bool,
    /// `onlyOwnerCanComplete` (Boolean)
    #[serde(default)]
    pub only_owner_can_complete: bool,
    /// `handlesAbandonRequest` (Boolean)
    #[serde(default)]
    pub handles_abandon_request: bool,
    /// `missionModulePerPlayer` (Boolean)
    #[serde(default)]
    pub mission_module_per_player: bool,
    /// `maxInstances` (Int32)
    #[serde(default)]
    pub max_instances: i32,
    /// `maxPlayersPerInstance` (Int32)
    #[serde(default)]
    pub max_players_per_instance: i32,
    /// `maxInstancesPerPlayer` (Int32)
    #[serde(default)]
    pub max_instances_per_player: i32,
    /// `canBeShared` (Boolean)
    #[serde(default)]
    pub can_be_shared: bool,
    /// `onceOnly` (Boolean)
    #[serde(default)]
    pub once_only: bool,
    /// `tutorial` (Boolean)
    #[serde(default)]
    pub tutorial: bool,
    /// `missionDeadline` (Class)
    #[serde(default)]
    pub mission_deadline: Option<Handle<MissionDeadline>>,
    /// `displayAlliedMarkers` (Boolean)
    #[serde(default)]
    pub display_allied_markers: bool,
    /// `availableInPrison` (Boolean)
    #[serde(default)]
    pub available_in_prison: bool,
    /// `failIfSentToPrison` (Boolean)
    #[serde(default)]
    pub fail_if_sent_to_prison: bool,
    /// `failIfBecameCriminal` (Boolean)
    #[serde(default)]
    pub fail_if_became_criminal: bool,
    /// `failIfLeavePrison` (Boolean)
    #[serde(default)]
    pub fail_if_leave_prison: bool,
    /// `completionTags` (Class)
    #[serde(default)]
    pub completion_tags: Option<Handle<TagList>>,
    /// `applyCompletionTagsOnFailed` (Boolean)
    #[serde(default)]
    pub apply_completion_tags_on_failed: bool,
    /// `applyCompletionTagsOnAbandoned` (Boolean)
    #[serde(default)]
    pub apply_completion_tags_on_abandoned: bool,
    /// `requestOnly` (Boolean)
    #[serde(default)]
    pub request_only: bool,
    /// `respawnTime` (Single)
    #[serde(default)]
    pub respawn_time: f32,
    /// `respawnTimeVariation` (Single)
    #[serde(default)]
    pub respawn_time_variation: f32,
    /// `instanceHasLifeTime` (Boolean)
    #[serde(default)]
    pub instance_has_life_time: bool,
    /// `showLifeTimeInMobiGlas` (Boolean)
    #[serde(default)]
    pub show_life_time_in_mobi_glas: bool,
    /// `instanceLifeTime` (Single)
    #[serde(default)]
    pub instance_life_time: f32,
    /// `instanceLifeTimeVariation` (Single)
    #[serde(default)]
    pub instance_life_time_variation: f32,
    /// `canReacceptAfterAbandoning` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_abandoning: bool,
    /// `abandonedCooldownTime` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time: f32,
    /// `abandonedCooldownTimeVariation` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time_variation: f32,
    /// `canReacceptAfterFailing` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_failing: bool,
    /// `hasPersonalCooldown` (Boolean)
    #[serde(default)]
    pub has_personal_cooldown: bool,
    /// `personalCooldownTime` (Single)
    #[serde(default)]
    pub personal_cooldown_time: f32,
    /// `personalCooldownTimeVariation` (Single)
    #[serde(default)]
    pub personal_cooldown_time_variation: f32,
    /// `moduleHandlesOwnShutdown` (Boolean)
    #[serde(default)]
    pub module_handles_own_shutdown: bool,
    /// `linkedMission` (Reference)
    #[serde(default)]
    pub linked_mission: Option<CigGuid>,
    /// `missionCompletePerk` (StrongPointer)
    #[serde(default)]
    pub mission_complete_perk: Option<Handle<MissionCompletePerkBaseDef>>,
    /// `modifiers` (StrongPointer (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BaseMissionModifier>>,
    /// `lawfulMission` (Boolean)
    #[serde(default)]
    pub lawful_mission: bool,
    /// `missionGiverRecord` (Reference)
    #[serde(default)]
    pub mission_giver_record: Option<CigGuid>,
    /// `invitationMission` (Reference)
    #[serde(default)]
    pub invitation_mission: Option<CigGuid>,
    /// `missionTags` (Reference (array))
    #[serde(default)]
    pub mission_tags: Vec<CigGuid>,
    /// `missionGiverFragmentTags` (String)
    #[serde(default)]
    pub mission_giver_fragment_tags: String,
    /// `reputationPrerequisites` (Class)
    #[serde(default)]
    pub reputation_prerequisites: Option<Handle<ReputationPrerequisites>>,
    /// `reputationRequirements` (StrongPointer)
    #[serde(default)]
    pub reputation_requirements: Option<Handle<SReputationMissionRequirementsParams>>,
    /// `minRequiredMissions` (Class (array))
    #[serde(default)]
    pub min_required_missions: Vec<Handle<MinRequiredMissions>>,
    /// `requiredMissions` (Reference (array))
    #[serde(default)]
    pub required_missions: Vec<CigGuid>,
    /// `requiredCompletedMissionTags` (Class (array))
    #[serde(default)]
    pub required_completed_mission_tags: Vec<Handle<TagSearchTerm>>,
    /// `requiredJournalEntries` (Reference (array))
    #[serde(default)]
    pub required_journal_entries: Vec<CigGuid>,
    /// `requiredAreaTags` (Class)
    #[serde(default)]
    pub required_area_tags: Option<Handle<TagList>>,
    /// `excludedAreaTags` (Class)
    #[serde(default)]
    pub excluded_area_tags: Option<Handle<TagList>>,
    /// `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
    /// `objectiveTokens` (Class (array))
    #[serde(default)]
    pub objective_tokens: Vec<Handle<ObjectiveToken>>,
    /// `missionFlow` (Class)
    #[serde(default)]
    pub mission_flow: Option<Handle<MissionFlow>>,
}

impl Pooled for MissionBrokerEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.mission_broker_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.mission_broker_entry }
}

impl<'a> Extract<'a> for MissionBrokerEntry {
    const TYPE_NAME: &'static str = "MissionBrokerEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            owner: inst.get("owner").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            player_facing_debug_name: inst.get_str("playerFacingDebugName").map(String::from).unwrap_or_default(),
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            title_hud: inst.get_str("titleHUD").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            mission_giver: inst.get_str("missionGiver").map(String::from).unwrap_or_default(),
            comms_channel_name: inst.get_str("commsChannelName").map(String::from).unwrap_or_default(),
            r#type: inst.get("type").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            associated_missions: inst.get_array("associatedMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_difficulty: inst.get_i32("missionDifficulty").unwrap_or_default(),
            locality_available: inst.get("localityAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            location_mission_available: inst.get("locationMissionAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            available_date_schedule: inst.get_array("availableDateSchedule")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DateTimeSchedule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DateTimeSchedule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            only_available_if_all_missions_not_available: inst.get_array("onlyAvailableIfAllMissionsNotAvailable")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_reward: match inst.get("missionReward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionReward>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionReward>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            partial_reward_payout: match inst.get("partialRewardPayout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PartialContractRewards>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PartialContractRewards>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_result_reputation_rewards: match inst.get("missionResultReputationRewards") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationAmountListParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationAmountListParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            journal_entries_to_add_on_complete: inst.get_array("journalEntriesToAddOnComplete")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entries_to_remove_on_complete: inst.get_array("journalEntriesToRemoveOnComplete")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            initially_active: inst.get_bool("initiallyActive").unwrap_or_default(),
            notify_on_available: inst.get_bool("notifyOnAvailable").unwrap_or_default(),
            pre_show_objectives: inst.get_bool("preShowObjectives").unwrap_or_default(),
            show_as_offer: inst.get_bool("showAsOffer").unwrap_or_default(),
            mission_buy_in_amount: inst.get_i32("missionBuyInAmount").unwrap_or_default(),
            refund_buy_in_on_withdraw: inst.get_bool("refundBuyInOnWithdraw").unwrap_or_default(),
            has_complete_button: inst.get_bool("hasCompleteButton").unwrap_or_default(),
            only_owner_can_complete: inst.get_bool("onlyOwnerCanComplete").unwrap_or_default(),
            handles_abandon_request: inst.get_bool("handlesAbandonRequest").unwrap_or_default(),
            mission_module_per_player: inst.get_bool("missionModulePerPlayer").unwrap_or_default(),
            max_instances: inst.get_i32("maxInstances").unwrap_or_default(),
            max_players_per_instance: inst.get_i32("maxPlayersPerInstance").unwrap_or_default(),
            max_instances_per_player: inst.get_i32("maxInstancesPerPlayer").unwrap_or_default(),
            can_be_shared: inst.get_bool("canBeShared").unwrap_or_default(),
            once_only: inst.get_bool("onceOnly").unwrap_or_default(),
            tutorial: inst.get_bool("tutorial").unwrap_or_default(),
            mission_deadline: match inst.get("missionDeadline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionDeadline>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionDeadline>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_allied_markers: inst.get_bool("displayAlliedMarkers").unwrap_or_default(),
            available_in_prison: inst.get_bool("availableInPrison").unwrap_or_default(),
            fail_if_sent_to_prison: inst.get_bool("failIfSentToPrison").unwrap_or_default(),
            fail_if_became_criminal: inst.get_bool("failIfBecameCriminal").unwrap_or_default(),
            fail_if_leave_prison: inst.get_bool("failIfLeavePrison").unwrap_or_default(),
            completion_tags: match inst.get("completionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            apply_completion_tags_on_failed: inst.get_bool("applyCompletionTagsOnFailed").unwrap_or_default(),
            apply_completion_tags_on_abandoned: inst.get_bool("applyCompletionTagsOnAbandoned").unwrap_or_default(),
            request_only: inst.get_bool("requestOnly").unwrap_or_default(),
            respawn_time: inst.get_f32("respawnTime").unwrap_or_default(),
            respawn_time_variation: inst.get_f32("respawnTimeVariation").unwrap_or_default(),
            instance_has_life_time: inst.get_bool("instanceHasLifeTime").unwrap_or_default(),
            show_life_time_in_mobi_glas: inst.get_bool("showLifeTimeInMobiGlas").unwrap_or_default(),
            instance_life_time: inst.get_f32("instanceLifeTime").unwrap_or_default(),
            instance_life_time_variation: inst.get_f32("instanceLifeTimeVariation").unwrap_or_default(),
            can_reaccept_after_abandoning: inst.get_bool("canReacceptAfterAbandoning").unwrap_or_default(),
            abandoned_cooldown_time: inst.get_f32("abandonedCooldownTime").unwrap_or_default(),
            abandoned_cooldown_time_variation: inst.get_f32("abandonedCooldownTimeVariation").unwrap_or_default(),
            can_reaccept_after_failing: inst.get_bool("canReacceptAfterFailing").unwrap_or_default(),
            has_personal_cooldown: inst.get_bool("hasPersonalCooldown").unwrap_or_default(),
            personal_cooldown_time: inst.get_f32("personalCooldownTime").unwrap_or_default(),
            personal_cooldown_time_variation: inst.get_f32("personalCooldownTimeVariation").unwrap_or_default(),
            module_handles_own_shutdown: inst.get_bool("moduleHandlesOwnShutdown").unwrap_or_default(),
            linked_mission: inst.get("linkedMission").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_complete_perk: match inst.get("missionCompletePerk") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionCompletePerkBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionCompletePerkBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseMissionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseMissionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            lawful_mission: inst.get_bool("lawfulMission").unwrap_or_default(),
            mission_giver_record: inst.get("missionGiverRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            invitation_mission: inst.get("invitationMission").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_tags: inst.get_array("missionTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            mission_giver_fragment_tags: inst.get_str("missionGiverFragmentTags").map(String::from).unwrap_or_default(),
            reputation_prerequisites: match inst.get("reputationPrerequisites") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ReputationPrerequisites>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ReputationPrerequisites>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reputation_requirements: match inst.get("reputationRequirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationMissionRequirementsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationMissionRequirementsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_required_missions: inst.get_array("minRequiredMissions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MinRequiredMissions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MinRequiredMissions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            required_missions: inst.get_array("requiredMissions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            required_completed_mission_tags: inst.get_array("requiredCompletedMissionTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagSearchTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagSearchTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            required_journal_entries: inst.get_array("requiredJournalEntries")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            required_area_tags: match inst.get("requiredAreaTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            excluded_area_tags: match inst.get("excludedAreaTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            objective_tokens: inst.get_array("objectiveTokens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ObjectiveToken>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ObjectiveToken>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mission_flow: match inst.get("missionFlow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionFlow>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionFlow>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionCompletePerkBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCompletePerkBaseDef {
}

impl Pooled for MissionCompletePerkBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.mission_complete_perk_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.mission_complete_perk_base_def }
}

impl<'a> Extract<'a> for MissionCompletePerkBaseDef {
    const TYPE_NAME: &'static str = "MissionCompletePerkBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationAmountParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationAmountParams {
    /// `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// `reward` (Reference)
    #[serde(default)]
    pub reward: Option<CigGuid>,
}

impl Pooled for SReputationAmountParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.sreputation_amount_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.sreputation_amount_params }
}

impl<'a> Extract<'a> for SReputationAmountParams {
    const TYPE_NAME: &'static str = "SReputationAmountParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward: inst.get("reward").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationAmountListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationAmountListParams {
    /// `reputationAmounts` (Class (array))
    #[serde(default)]
    pub reputation_amounts: Vec<Handle<SReputationAmountParams>>,
}

impl Pooled for SReputationAmountListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.sreputation_amount_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.sreputation_amount_list_params }
}

impl<'a> Extract<'a> for SReputationAmountListParams {
    const TYPE_NAME: &'static str = "SReputationAmountListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reputation_amounts: inst.get_array("reputationAmounts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationAmountParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationAmountParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationMissionRequirementExpressionElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRequirementExpressionElement {
}

impl Pooled for SReputationMissionRequirementExpressionElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.sreputation_mission_requirement_expression_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.sreputation_mission_requirement_expression_element }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpressionElement {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpressionElement";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationMissionRequirementsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRequirementsParams {
    /// `expression` (StrongPointer (array))
    #[serde(default)]
    pub expression: Vec<Handle<SReputationMissionRequirementExpressionElement>>,
}

impl Pooled for SReputationMissionRequirementsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionbroker.sreputation_mission_requirements_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionbroker.sreputation_mission_requirements_params }
}

impl<'a> Extract<'a> for SReputationMissionRequirementsParams {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            expression: inst.get_array("expression")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationMissionRequirementExpressionElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationMissionRequirementExpressionElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

