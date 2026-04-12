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

/// Pool storage for the `gamemode` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GamemodePools {
    #[serde(default)]
    pub scustomizable_material_entry: Vec<Option<SCustomizableMaterialEntry>>,
    #[serde(default)]
    pub scustomizable_material_lookup_table: Vec<Option<SCustomizableMaterialLookupTable>>,
    #[serde(default)]
    pub scharacter_archetype_params: Vec<Option<SCharacterArchetypeParams>>,
    #[serde(default)]
    pub scharacter_archetype: Vec<Option<SCharacterArchetype>>,
    #[serde(default)]
    pub scharacter_archetype_list: Vec<Option<SCharacterArchetypeList>>,
    #[serde(default)]
    pub scharacter_generation_params: Vec<Option<SCharacterGenerationParams>>,
    #[serde(default)]
    pub sarchetype_asset_def_base: Vec<Option<SArchetypeAssetDefBase>>,
    #[serde(default)]
    pub sarchetype_asset_list: Vec<Option<SArchetypeAssetList>>,
    #[serde(default)]
    pub scharacter_archetype_additive_asset_list: Vec<Option<SCharacterArchetypeAdditiveAssetList>>,
    #[serde(default)]
    pub sasset_list_condition: Vec<Option<SAssetListCondition>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshot_entry: Vec<Option<SEAPlayerLoadoutSnapshotEntry>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshots: Vec<Option<SEAPlayerLoadoutSnapshots>>,
    #[serde(default)]
    pub sealoadout_attachment: Vec<Option<SEALoadoutAttachment>>,
    #[serde(default)]
    pub sealoadout_explicit: Vec<Option<SEALoadoutExplicit>>,
    #[serde(default)]
    pub sealoadout_item: Vec<Option<SEALoadoutItem>>,
    #[serde(default)]
    pub sealoadout_set: Vec<Option<SEALoadoutSet>>,
    #[serde(default)]
    pub sealoadout_collection: Vec<Option<SEALoadoutCollection>>,
    #[serde(default)]
    pub seaglobal_special_loadout: Vec<Option<SEAGlobalSpecialLoadout>>,
    #[serde(default)]
    pub seaglobal_event_loadouts: Vec<Option<SEAGlobalEventLoadouts>>,
    #[serde(default)]
    pub sirounds_module: Vec<Option<SIRoundsModule>>,
    #[serde(default)]
    pub entity_default_loadout_params: Vec<Option<EntityDefaultLoadoutParams>>,
    #[serde(default)]
    pub sipickup_module: Vec<Option<SIPickupModule>>,
    #[serde(default)]
    pub sidamage_handling_module: Vec<Option<SIDamageHandlingModule>>,
    #[serde(default)]
    pub sihostility_module: Vec<Option<SIHostilityModule>>,
    #[serde(default)]
    pub sispectator_module: Vec<Option<SISpectatorModule>>,
    #[serde(default)]
    pub sscore_event: Vec<Option<SScoreEvent>>,
    #[serde(default)]
    pub splayer_scoring: Vec<Option<SPlayerScoring>>,
    #[serde(default)]
    pub steam_scoring: Vec<Option<STeamScoring>>,
    #[serde(default)]
    pub eagame_completion_award_base_params: Vec<Option<EAGameCompletionAwardBaseParams>>,
    #[serde(default)]
    pub siscoring_module: Vec<Option<SIScoringModule>>,
    #[serde(default)]
    pub siplayer_setup_module: Vec<Option<SIPlayerSetupModule>>,
    #[serde(default)]
    pub sistats_recording_module: Vec<Option<SIStatsRecordingModule>>,
    #[serde(default)]
    pub sinotifications_module: Vec<Option<SINotificationsModule>>,
    #[serde(default)]
    pub siobjectives: Vec<Option<SIObjectives>>,
    #[serde(default)]
    pub sicameras_module: Vec<Option<SICamerasModule>>,
    #[serde(default)]
    pub siplayer_stats: Vec<Option<SIPlayerStats>>,
    #[serde(default)]
    pub sispawning: Vec<Option<SISpawning>>,
    #[serde(default)]
    pub sivictory_conditions_module: Vec<Option<SIVictoryConditionsModule>>,
    #[serde(default)]
    pub siparams_module: Vec<Option<SIParamsModule>>,
    #[serde(default)]
    pub sisubsumption_mission_module: Vec<Option<SISubsumptionMissionModule>>,
    #[serde(default)]
    pub game_mode_valid_map: Vec<Option<GameModeValidMap>>,
    #[serde(default)]
    pub game_mode_custom_setting: Vec<Option<GameModeCustomSetting>>,
    #[serde(default)]
    pub chat_system_options_module: Vec<Option<ChatSystemOptionsModule>>,
    #[serde(default)]
    pub game_mode_filter: Vec<Option<GameModeFilter>>,
    #[serde(default)]
    pub game_mode: Vec<Option<GameMode>>,
    #[serde(default)]
    pub sibetting_module: Vec<Option<SIBettingModule>>,
    #[serde(default)]
    pub difficulty_modifier_range: Vec<Option<DifficultyModifierRange>>,
    #[serde(default)]
    pub vehicle_difficulty_params: Vec<Option<VehicleDifficultyParams>>,
    #[serde(default)]
    pub difficulty_level_params: Vec<Option<DifficultyLevelParams>>,
    #[serde(default)]
    pub game_difficulty_modifiers: Vec<Option<GameDifficultyModifiers>>,
    #[serde(default)]
    pub sidifficulty_module: Vec<Option<SIDifficultyModule>>,
    #[serde(default)]
    pub sireputation_module: Vec<Option<SIReputationModule>>,
    #[serde(default)]
    pub sistate_module: Vec<Option<SIStateModule>>,
    #[serde(default)]
    pub siteams_module: Vec<Option<SITeamsModule>>,
    #[serde(default)]
    pub sivoting_module: Vec<Option<SIVotingModule>>,
    #[serde(default)]
    pub player_ship_respawn_ship_info: Vec<Option<PlayerShipRespawnShipInfo>>,
    #[serde(default)]
    pub player_ship_respawn: Vec<Option<PlayerShipRespawn>>,
}
