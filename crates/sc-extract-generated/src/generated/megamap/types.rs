// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `megamap`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ArenaCommanderLocationObjectContainersParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderLocationObjectContainersParams {
    /// `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// `overrideRootOC` (Boolean)
    #[serde(default)]
    pub override_root_oc: bool,
    /// `objectContainers` (String (array))
    #[serde(default)]
    pub object_containers: Vec<String>,
}

impl Pooled for ArenaCommanderLocationObjectContainersParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.megamap.arena_commander_location_object_containers_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.megamap.arena_commander_location_object_containers_params }
}

impl<'a> Extract<'a> for ArenaCommanderLocationObjectContainersParams {
    const TYPE_NAME: &'static str = "ArenaCommanderLocationObjectContainersParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            override_root_oc: inst.get_bool("overrideRootOC").unwrap_or_default(),
            object_containers: inst.get_array("objectContainers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ArenaCommanderPlanetOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderPlanetOverrideParams {
    /// `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// `fixedRotation` (Single)
    #[serde(default)]
    pub fixed_rotation: f32,
}

impl Pooled for ArenaCommanderPlanetOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.megamap.arena_commander_planet_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.megamap.arena_commander_planet_override_params }
}

impl<'a> Extract<'a> for ArenaCommanderPlanetOverrideParams {
    const TYPE_NAME: &'static str = "ArenaCommanderPlanetOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fixed_rotation: inst.get_f32("fixedRotation").unwrap_or_default(),
        }
    }
}

/// DCB type: `ArenaCommanderScenarioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCommanderScenarioParams {
    /// `locationObjectContainersParams` (StrongPointer)
    #[serde(default)]
    pub location_object_containers_params: Option<Handle<ArenaCommanderLocationObjectContainersParams>>,
    /// `planetOverrideParams` (StrongPointer)
    #[serde(default)]
    pub planet_override_params: Option<Handle<ArenaCommanderPlanetOverrideParams>>,
}

impl Pooled for ArenaCommanderScenarioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.megamap.arena_commander_scenario_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.megamap.arena_commander_scenario_params }
}

impl<'a> Extract<'a> for ArenaCommanderScenarioParams {
    const TYPE_NAME: &'static str = "ArenaCommanderScenarioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_object_containers_params: match inst.get("locationObjectContainersParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderLocationObjectContainersParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            planet_override_params: match inst.get("planetOverrideParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderPlanetOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMegaMapSolarSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMegaMapSolarSystem {
    /// `Record` (Reference)
    #[serde(default)]
    pub record: Option<CigGuid>,
    /// `ObjectContainers` (String (array))
    #[serde(default)]
    pub object_containers: Vec<String>,
}

impl Pooled for SMegaMapSolarSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.megamap.smega_map_solar_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.megamap.smega_map_solar_system }
}

impl<'a> Extract<'a> for SMegaMapSolarSystem {
    const TYPE_NAME: &'static str = "SMegaMapSolarSystem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            record: inst.get("Record").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            object_containers: inst.get_array("ObjectContainers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MegaMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegaMap {
    /// `gameMode` (Reference)
    #[serde(default)]
    pub game_mode: Option<CigGuid>,
    /// `SolarSystems` (Class (array))
    #[serde(default)]
    pub solar_systems: Vec<Handle<SMegaMapSolarSystem>>,
    /// `singlePlayerOrMultiplayer` (EnumChoice)
    #[serde(default)]
    pub single_player_or_multiplayer: SinglePlayerOrMultiplayer,
    /// `subsumptionMission` (String)
    #[serde(default)]
    pub subsumption_mission: String,
    /// `subsumptionMissionInitParams` (StrongPointer (array))
    #[serde(default)]
    pub subsumption_mission_init_params: Vec<AbstractMissionInitParamPtr>,
    /// `arenaCommanderScenarioParams` (StrongPointer)
    #[serde(default)]
    pub arena_commander_scenario_params: Option<Handle<ArenaCommanderScenarioParams>>,
    /// `level` (Reference)
    #[serde(default)]
    pub level: Option<CigGuid>,
    /// `trackViewIntro` (String)
    #[serde(default)]
    pub track_view_intro: String,
    /// `rootLocation` (Reference)
    #[serde(default)]
    pub root_location: Option<CigGuid>,
    /// `streamingMode` (EnumChoice)
    #[serde(default)]
    pub streaming_mode: LevelStreamingMode,
    /// `bindCullingEnabled` (Boolean)
    #[serde(default)]
    pub bind_culling_enabled: bool,
    /// `defaultWinningTeamOverride` (Int32)
    #[serde(default)]
    pub default_winning_team_override: i32,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: LocaleKey,
    /// `appearsInS42LevelSelect` (Boolean)
    #[serde(default)]
    pub appears_in_s42_level_select: bool,
    /// `chapter` (Reference)
    #[serde(default)]
    pub chapter: Option<CigGuid>,
    /// `chapterAlias` (String)
    #[serde(default)]
    pub chapter_alias: String,
    /// `skipLoadScreen` (Boolean)
    #[serde(default)]
    pub skip_load_screen: bool,
    /// `devDebugSkip` (Boolean)
    #[serde(default)]
    pub dev_debug_skip: bool,
}

impl Pooled for MegaMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.megamap.mega_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.megamap.mega_map }
}

impl<'a> Extract<'a> for MegaMap {
    const TYPE_NAME: &'static str = "MegaMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            game_mode: inst.get("gameMode").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            solar_systems: inst.get_array("SolarSystems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMegaMapSolarSystem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SMegaMapSolarSystem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            single_player_or_multiplayer: SinglePlayerOrMultiplayer::from_dcb_str(inst.get_str("singlePlayerOrMultiplayer").unwrap_or("")),
            subsumption_mission: inst.get_str("subsumptionMission").map(String::from).unwrap_or_default(),
            subsumption_mission_init_params: inst.get_array("subsumptionMissionInitParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(AbstractMissionInitParamPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            arena_commander_scenario_params: match inst.get("arenaCommanderScenarioParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderScenarioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            level: inst.get("level").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            track_view_intro: inst.get_str("trackViewIntro").map(String::from).unwrap_or_default(),
            root_location: inst.get("rootLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            streaming_mode: LevelStreamingMode::from_dcb_str(inst.get_str("streamingMode").unwrap_or("")),
            bind_culling_enabled: inst.get_bool("bindCullingEnabled").unwrap_or_default(),
            default_winning_team_override: inst.get_i32("defaultWinningTeamOverride").unwrap_or_default(),
            display_name: inst.get_str("displayName").map(LocaleKey::from).unwrap_or_default(),
            appears_in_s42_level_select: inst.get_bool("appearsInS42LevelSelect").unwrap_or_default(),
            chapter: inst.get("chapter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chapter_alias: inst.get_str("chapterAlias").map(String::from).unwrap_or_default(),
            skip_load_screen: inst.get_bool("skipLoadScreen").unwrap_or_default(),
            dev_debug_skip: inst.get_bool("devDebugSkip").unwrap_or_default(),
        }
    }
}

