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

/// DCB type: `GameModule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModule {
    /// DCB field: `moduleCode` (String)
    #[serde(default)]
    pub module_code: String,
    /// DCB field: `rankTexturePrefix` (String)
    #[serde(default)]
    pub rank_texture_prefix: String,
}

impl Pooled for GameModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_module }
}

impl<'a> Extract<'a> for GameModule {
    const TYPE_NAME: &'static str = "GameModule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            module_code: inst.get_str("moduleCode").map(String::from).unwrap_or_default(),
            rank_texture_prefix: inst.get_str("rankTexturePrefix").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GameModeValidMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeValidMap {
    /// DCB field: `levelReference` (Reference)
    #[serde(default)]
    pub level_reference: Option<CigGuid>,
    /// DCB field: `devOnly` (Boolean)
    #[serde(default)]
    pub dev_only: bool,
}

impl Pooled for GameModeValidMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_mode_valid_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_mode_valid_map }
}

impl<'a> Extract<'a> for GameModeValidMap {
    const TYPE_NAME: &'static str = "GameModeValidMap";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            level_reference: inst.get("levelReference").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dev_only: inst.get_bool("devOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `GameModeCustomSetting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeCustomSetting {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `controlledTypes` (EnumChoice (array))
    #[serde(default)]
    pub controlled_types: Vec<String>,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// DCB field: `step` (Single)
    #[serde(default)]
    pub step: f32,
    /// DCB field: `defaultValueOverride` (Single)
    #[serde(default)]
    pub default_value_override: f32,
}

impl Pooled for GameModeCustomSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_mode_custom_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_mode_custom_setting }
}

impl<'a> Extract<'a> for GameModeCustomSetting {
    const TYPE_NAME: &'static str = "GameModeCustomSetting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            controlled_types: inst.get_array("controlledTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
            step: inst.get_f32("step").unwrap_or_default(),
            default_value_override: inst.get_f32("defaultValueOverride").unwrap_or_default(),
        }
    }
}

/// DCB type: `GameModeFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeFilter {
    /// DCB field: `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// DCB field: `visibleToPlayers` (Boolean)
    #[serde(default)]
    pub visible_to_players: bool,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for GameModeFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_mode_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_mode_filter }
}

impl<'a> Extract<'a> for GameModeFilter {
    const TYPE_NAME: &'static str = "GameModeFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_str("id").map(String::from).unwrap_or_default(),
            visible_to_players: inst.get_bool("visibleToPlayers").unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GameMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMode {
    /// DCB field: `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// DCB field: `requiredPass` (EnumChoice)
    #[serde(default)]
    pub required_pass: String,
    /// DCB field: `playedBadgeId` (EnumChoice)
    #[serde(default)]
    pub played_badge_id: String,
    /// DCB field: `locDisplayName` (Locale)
    #[serde(default)]
    pub loc_display_name: String,
    /// DCB field: `locSubtitle` (Locale)
    #[serde(default)]
    pub loc_subtitle: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `thumbnail` (String)
    #[serde(default)]
    pub thumbnail: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `backgroundVideos` (String (array))
    #[serde(default)]
    pub background_videos: Vec<String>,
    /// DCB field: `alias` (String)
    #[serde(default)]
    pub alias: String,
    /// DCB field: `leaderboardMatchCode` (String)
    #[serde(default)]
    pub leaderboard_match_code: String,
    /// DCB field: `loadingScreenInfo` (Class)
    #[serde(default)]
    pub loading_screen_info: Option<Handle<SLoadingScreenInformationDef>>,
    /// DCB field: `shardPersistenceEnabled` (Boolean)
    #[serde(default)]
    pub shard_persistence_enabled: bool,
    /// DCB field: `enableCustomization` (Boolean)
    #[serde(default)]
    pub enable_customization: bool,
    /// DCB field: `validNetworkTypes` (EnumChoice (array))
    #[serde(default)]
    pub valid_network_types: Vec<String>,
    /// DCB field: `filters` (Class (array))
    #[serde(default)]
    pub filters: Vec<Handle<GameModeFilter>>,
    /// DCB field: `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<SIParamsModule>>,
    /// DCB field: `bettingModule` (StrongPointer)
    #[serde(default)]
    pub betting_module: Option<Handle<SIBettingModule>>,
    /// DCB field: `camerasModule` (StrongPointer)
    #[serde(default)]
    pub cameras_module: Option<Handle<SICamerasModule>>,
    /// DCB field: `chatSystemOptions` (StrongPointer)
    #[serde(default)]
    pub chat_system_options: Option<Handle<ChatSystemOptionsModule>>,
    /// DCB field: `damageHandling` (StrongPointer)
    #[serde(default)]
    pub damage_handling: Option<Handle<SIDamageHandlingModule>>,
    /// DCB field: `difficultyModule` (StrongPointer)
    #[serde(default)]
    pub difficulty_module: Option<Handle<SIDifficultyModule>>,
    /// DCB field: `hostility` (StrongPointer)
    #[serde(default)]
    pub hostility: Option<Handle<SIHostilityModule>>,
    /// DCB field: `notifications` (StrongPointer)
    #[serde(default)]
    pub notifications: Option<Handle<SINotificationsModule>>,
    /// DCB field: `objectives` (StrongPointer)
    #[serde(default)]
    pub objectives: Option<Handle<SIObjectives>>,
    /// DCB field: `pickup` (StrongPointer)
    #[serde(default)]
    pub pickup: Option<Handle<SIPickupModule>>,
    /// DCB field: `playerSetup` (StrongPointer)
    #[serde(default)]
    pub player_setup: Option<Handle<SIPlayerSetupModule>>,
    /// DCB field: `playerStats` (StrongPointer)
    #[serde(default)]
    pub player_stats: Option<Handle<SIPlayerStats>>,
    /// DCB field: `reputationModule` (StrongPointer)
    #[serde(default)]
    pub reputation_module: Option<Handle<SIReputationModule>>,
    /// DCB field: `rounds` (StrongPointer)
    #[serde(default)]
    pub rounds: Option<Handle<SIRoundsModule>>,
    /// DCB field: `stateModule` (StrongPointer)
    #[serde(default)]
    pub state_module: Option<Handle<SIStateModule>>,
    /// DCB field: `scoring` (StrongPointer)
    #[serde(default)]
    pub scoring: Option<Handle<SIScoringModule>>,
    /// DCB field: `spawning` (StrongPointer)
    #[serde(default)]
    pub spawning: Option<Handle<SISpawning>>,
    /// DCB field: `spectator` (StrongPointer)
    #[serde(default)]
    pub spectator: Option<Handle<SISpectatorModule>>,
    /// DCB field: `statsRecording` (StrongPointer)
    #[serde(default)]
    pub stats_recording: Option<Handle<SIStatsRecordingModule>>,
    /// DCB field: `subsumptionMissionModule` (StrongPointer)
    #[serde(default)]
    pub subsumption_mission_module: Option<Handle<SISubsumptionMissionModule>>,
    /// DCB field: `teams` (StrongPointer)
    #[serde(default)]
    pub teams: Option<Handle<SITeamsModule>>,
    /// DCB field: `victoryConditions` (StrongPointer)
    #[serde(default)]
    pub victory_conditions: Option<Handle<SIVictoryConditionsModule>>,
    /// DCB field: `votingModule` (StrongPointer)
    #[serde(default)]
    pub voting_module: Option<Handle<SIVotingModule>>,
    /// DCB field: `mapSelection` (Boolean)
    #[serde(default)]
    pub map_selection: bool,
    /// DCB field: `validMaps` (Class (array))
    #[serde(default)]
    pub valid_maps: Vec<Handle<GameModeValidMap>>,
    /// DCB field: `customSettings` (Class (array))
    #[serde(default)]
    pub custom_settings: Vec<Handle<GameModeCustomSetting>>,
}

impl Pooled for GameMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_mode }
}

impl<'a> Extract<'a> for GameMode {
    const TYPE_NAME: &'static str = "GameMode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_str("id").map(String::from).unwrap_or_default(),
            required_pass: inst.get_str("requiredPass").map(String::from).unwrap_or_default(),
            played_badge_id: inst.get_str("playedBadgeId").map(String::from).unwrap_or_default(),
            loc_display_name: inst.get_str("locDisplayName").map(String::from).unwrap_or_default(),
            loc_subtitle: inst.get_str("locSubtitle").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            thumbnail: inst.get_str("thumbnail").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            background_videos: inst.get_array("backgroundVideos")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            alias: inst.get_str("alias").map(String::from).unwrap_or_default(),
            leaderboard_match_code: inst.get_str("leaderboardMatchCode").map(String::from).unwrap_or_default(),
            loading_screen_info: match inst.get("loadingScreenInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLoadingScreenInformationDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLoadingScreenInformationDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shard_persistence_enabled: inst.get_bool("shardPersistenceEnabled").unwrap_or_default(),
            enable_customization: inst.get_bool("enableCustomization").unwrap_or_default(),
            valid_network_types: inst.get_array("validNetworkTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            filters: inst.get_array("filters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GameModeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GameModeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIParamsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIParamsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            betting_module: match inst.get("bettingModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIBettingModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIBettingModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cameras_module: match inst.get("camerasModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SICamerasModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SICamerasModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chat_system_options: match inst.get("chatSystemOptions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ChatSystemOptionsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ChatSystemOptionsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage_handling: match inst.get("damageHandling") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIDamageHandlingModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIDamageHandlingModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            difficulty_module: match inst.get("difficultyModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIDifficultyModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIDifficultyModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hostility: match inst.get("hostility") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIHostilityModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIHostilityModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notifications: match inst.get("notifications") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SINotificationsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SINotificationsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            objectives: match inst.get("objectives") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIObjectives>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIObjectives>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pickup: match inst.get("pickup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIPickupModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIPickupModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_setup: match inst.get("playerSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIPlayerSetupModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIPlayerSetupModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_stats: match inst.get("playerStats") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIPlayerStats>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIPlayerStats>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reputation_module: match inst.get("reputationModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIReputationModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIReputationModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rounds: match inst.get("rounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIRoundsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIRoundsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state_module: match inst.get("stateModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIStateModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIStateModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scoring: match inst.get("scoring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIScoringModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIScoringModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawning: match inst.get("spawning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SISpawning>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SISpawning>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spectator: match inst.get("spectator") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SISpectatorModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SISpectatorModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stats_recording: match inst.get("statsRecording") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIStatsRecordingModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIStatsRecordingModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            subsumption_mission_module: match inst.get("subsumptionMissionModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SISubsumptionMissionModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SISubsumptionMissionModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            teams: match inst.get("teams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SITeamsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SITeamsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            victory_conditions: match inst.get("victoryConditions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIVictoryConditionsModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIVictoryConditionsModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            voting_module: match inst.get("votingModule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIVotingModule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIVotingModule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            map_selection: inst.get_bool("mapSelection").unwrap_or_default(),
            valid_maps: inst.get_array("validMaps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GameModeValidMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GameModeValidMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            custom_settings: inst.get_array("customSettings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GameModeCustomSetting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GameModeCustomSetting>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GameDifficultyModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDifficultyModifiers {
    /// DCB field: `difficulties` (Class)
    #[serde(default)]
    pub difficulties: Option<Handle<DifficultyLevelParams>>,
}

impl Pooled for GameDifficultyModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_difficulty_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_difficulty_modifiers }
}

impl<'a> Extract<'a> for GameDifficultyModifiers {
    const TYPE_NAME: &'static str = "GameDifficultyModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            difficulties: match inst.get("difficulties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DifficultyLevelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DifficultyLevelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GameNotificationDockItemParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameNotificationDockItemParams {
    /// DCB field: `notificationPositivePrompt` (Locale)
    #[serde(default)]
    pub notification_positive_prompt: String,
    /// DCB field: `notificationNegativePrompt` (Locale)
    #[serde(default)]
    pub notification_negative_prompt: String,
    /// DCB field: `notificationDismissPrompt` (Locale)
    #[serde(default)]
    pub notification_dismiss_prompt: String,
}

impl Pooled for GameNotificationDockItemParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.game_notification_dock_item_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.game_notification_dock_item_params }
}

impl<'a> Extract<'a> for GameNotificationDockItemParams {
    const TYPE_NAME: &'static str = "GameNotificationDockItemParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            notification_positive_prompt: inst.get_str("notificationPositivePrompt").map(String::from).unwrap_or_default(),
            notification_negative_prompt: inst.get_str("notificationNegativePrompt").map(String::from).unwrap_or_default(),
            notification_dismiss_prompt: inst.get_str("notificationDismissPrompt").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GasParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasParams {
    /// DCB field: `debugColor` (Class)
    #[serde(default)]
    pub debug_color: Option<Handle<RGB>>,
    /// DCB field: `chemicalSymbol` (Locale)
    #[serde(default)]
    pub chemical_symbol: String,
    /// DCB field: `fogDensity` (Single)
    #[serde(default)]
    pub fog_density: f32,
    /// DCB field: `fogColor` (Class)
    #[serde(default)]
    pub fog_color: Option<Handle<RGB>>,
}

impl Pooled for GasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ga.gas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ga.gas_params }
}

impl<'a> Extract<'a> for GasParams {
    const TYPE_NAME: &'static str = "GasParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_color: match inst.get("debugColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chemical_symbol: inst.get_str("chemicalSymbol").map(String::from).unwrap_or_default(),
            fog_density: inst.get_f32("fogDensity").unwrap_or_default(),
            fog_color: match inst.get("fogColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

