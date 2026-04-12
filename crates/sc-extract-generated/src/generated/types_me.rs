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

/// DCB type: `MedBedTierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedBedTierParams {
    /// DCB field: `medBedTier` (EnumChoice)
    #[serde(default)]
    pub med_bed_tier: String,
    /// DCB field: `defaultRespawnRange` (Int64)
    #[serde(default)]
    pub default_respawn_range: i64,
    /// DCB field: `drugEfficacyForConsumableTypes` (Class (array))
    #[serde(default)]
    pub drug_efficacy_for_consumable_types: Vec<Handle<DrugEfficacyForConsumableType>>,
    /// DCB field: `statTypesToHeal` (EnumChoice (array))
    #[serde(default)]
    pub stat_types_to_heal: Vec<String>,
}

impl Pooled for MedBedTierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.med_bed_tier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.med_bed_tier_params }
}

impl<'a> Extract<'a> for MedBedTierParams {
    const TYPE_NAME: &'static str = "MedBedTierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            med_bed_tier: inst.get_str("medBedTier").map(String::from).unwrap_or_default(),
            default_respawn_range: inst.get_i64("defaultRespawnRange").unwrap_or_default(),
            drug_efficacy_for_consumable_types: inst.get_array("drugEfficacyForConsumableTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DrugEfficacyForConsumableType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DrugEfficacyForConsumableType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            stat_types_to_heal: inst.get_array("statTypesToHeal")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MedBedResourceConsumptionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedBedResourceConsumptionParams {
    /// DCB field: `bedResource` (Reference)
    #[serde(default)]
    pub bed_resource: Option<CigGuid>,
    /// DCB field: `resourceAmountForSurgeryT1` (StrongPointer)
    #[serde(default)]
    pub resource_amount_for_surgery_t1: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `resourceAmountForSurgeryT2` (StrongPointer)
    #[serde(default)]
    pub resource_amount_for_surgery_t2: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `resourceAmountForSurgeryT3` (StrongPointer)
    #[serde(default)]
    pub resource_amount_for_surgery_t3: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `resourceAmountForCloning` (StrongPointer)
    #[serde(default)]
    pub resource_amount_for_cloning: Option<Handle<SBaseCargoUnit>>,
}

impl Pooled for MedBedResourceConsumptionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.med_bed_resource_consumption_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.med_bed_resource_consumption_params }
}

impl<'a> Extract<'a> for MedBedResourceConsumptionParams {
    const TYPE_NAME: &'static str = "MedBedResourceConsumptionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bed_resource: inst.get("bedResource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            resource_amount_for_surgery_t1: match inst.get("resourceAmountForSurgeryT1") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            resource_amount_for_surgery_t2: match inst.get("resourceAmountForSurgeryT2") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            resource_amount_for_surgery_t3: match inst.get("resourceAmountForSurgeryT3") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            resource_amount_for_cloning: match inst.get("resourceAmountForCloning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MedicalItemTierConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalItemTierConfig {
    /// DCB field: `medBedConfigs` (Class)
    #[serde(default)]
    pub med_bed_configs: Option<Handle<MedBedTierParams>>,
    /// DCB field: `drugEfficacyConfig` (Class (array))
    #[serde(default)]
    pub drug_efficacy_config: Vec<Handle<DrugEfficacyForItemType>>,
    /// DCB field: `availableDrugTypes` (Class (array))
    #[serde(default)]
    pub available_drug_types: Vec<Handle<DrugTypeToApply>>,
    /// DCB field: `resourceConsumption` (Class)
    #[serde(default)]
    pub resource_consumption: Option<Handle<MedBedResourceConsumptionParams>>,
}

impl Pooled for MedicalItemTierConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.medical_item_tier_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.medical_item_tier_config }
}

impl<'a> Extract<'a> for MedicalItemTierConfig {
    const TYPE_NAME: &'static str = "MedicalItemTierConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            med_bed_configs: match inst.get("medBedConfigs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MedBedTierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MedBedTierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drug_efficacy_config: inst.get_array("drugEfficacyConfig")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DrugEfficacyForItemType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DrugEfficacyForItemType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            available_drug_types: inst.get_array("availableDrugTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DrugTypeToApply>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DrugTypeToApply>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            resource_consumption: match inst.get("resourceConsumption") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MedBedResourceConsumptionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MedBedResourceConsumptionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MegaMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegaMap {
    /// DCB field: `gameMode` (Reference)
    #[serde(default)]
    pub game_mode: Option<CigGuid>,
    /// DCB field: `SolarSystems` (Class (array))
    #[serde(default)]
    pub solar_systems: Vec<Handle<SMegaMapSolarSystem>>,
    /// DCB field: `singlePlayerOrMultiplayer` (EnumChoice)
    #[serde(default)]
    pub single_player_or_multiplayer: String,
    /// DCB field: `subsumptionMission` (String)
    #[serde(default)]
    pub subsumption_mission: String,
    /// DCB field: `subsumptionMissionInitParams` (StrongPointer (array))
    #[serde(default)]
    pub subsumption_mission_init_params: Vec<Handle<AbstractMissionInitParam>>,
    /// DCB field: `arenaCommanderScenarioParams` (StrongPointer)
    #[serde(default)]
    pub arena_commander_scenario_params: Option<Handle<ArenaCommanderScenarioParams>>,
    /// DCB field: `level` (Reference)
    #[serde(default)]
    pub level: Option<CigGuid>,
    /// DCB field: `trackViewIntro` (String)
    #[serde(default)]
    pub track_view_intro: String,
    /// DCB field: `rootLocation` (Reference)
    #[serde(default)]
    pub root_location: Option<CigGuid>,
    /// DCB field: `streamingMode` (EnumChoice)
    #[serde(default)]
    pub streaming_mode: String,
    /// DCB field: `bindCullingEnabled` (Boolean)
    #[serde(default)]
    pub bind_culling_enabled: bool,
    /// DCB field: `defaultWinningTeamOverride` (Int32)
    #[serde(default)]
    pub default_winning_team_override: i32,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `appearsInS42LevelSelect` (Boolean)
    #[serde(default)]
    pub appears_in_s42_level_select: bool,
    /// DCB field: `chapter` (Reference)
    #[serde(default)]
    pub chapter: Option<CigGuid>,
    /// DCB field: `chapterAlias` (String)
    #[serde(default)]
    pub chapter_alias: String,
    /// DCB field: `skipLoadScreen` (Boolean)
    #[serde(default)]
    pub skip_load_screen: bool,
    /// DCB field: `devDebugSkip` (Boolean)
    #[serde(default)]
    pub dev_debug_skip: bool,
}

impl Pooled for MegaMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.mega_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.mega_map }
}

impl<'a> Extract<'a> for MegaMap {
    const TYPE_NAME: &'static str = "MegaMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            game_mode: inst.get("gameMode").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            solar_systems: inst.get_array("SolarSystems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMegaMapSolarSystem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMegaMapSolarSystem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            single_player_or_multiplayer: inst.get_str("singlePlayerOrMultiplayer").map(String::from).unwrap_or_default(),
            subsumption_mission: inst.get_str("subsumptionMission").map(String::from).unwrap_or_default(),
            subsumption_mission_init_params: inst.get_array("subsumptionMissionInitParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AbstractMissionInitParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AbstractMissionInitParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            arena_commander_scenario_params: match inst.get("arenaCommanderScenarioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ArenaCommanderScenarioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArenaCommanderScenarioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            level: inst.get("level").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            track_view_intro: inst.get_str("trackViewIntro").map(String::from).unwrap_or_default(),
            root_location: inst.get("rootLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            streaming_mode: inst.get_str("streamingMode").map(String::from).unwrap_or_default(),
            bind_culling_enabled: inst.get_bool("bindCullingEnabled").unwrap_or_default(),
            default_winning_team_override: inst.get_i32("defaultWinningTeamOverride").unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            appears_in_s42_level_select: inst.get_bool("appearsInS42LevelSelect").unwrap_or_default(),
            chapter: inst.get("chapter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chapter_alias: inst.get_str("chapterAlias").map(String::from).unwrap_or_default(),
            skip_load_screen: inst.get_bool("skipLoadScreen").unwrap_or_default(),
            dev_debug_skip: inst.get_bool("devDebugSkip").unwrap_or_default(),
        }
    }
}

/// DCB type: `MeleeFragInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeFragInfo {
    /// DCB field: `attackType` (EnumChoice)
    #[serde(default)]
    pub attack_type: String,
    /// DCB field: `meleeAttackCategoryInfo` (WeakPointer)
    #[serde(default)]
    pub melee_attack_category_info: Option<Handle<MeleeAttackCategoryInfo>>,
}

impl Pooled for MeleeFragInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_frag_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_frag_info }
}

impl<'a> Extract<'a> for MeleeFragInfo {
    const TYPE_NAME: &'static str = "MeleeFragInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attack_type: inst.get_str("attackType").map(String::from).unwrap_or_default(),
            melee_attack_category_info: match inst.get("meleeAttackCategoryInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MeleeAttackCategoryInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeAttackCategoryInfo {
    /// DCB field: `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// DCB field: `attackCategoryParams` (WeakPointer)
    #[serde(default)]
    pub attack_category_params: Option<Handle<AttackCategoryParams>>,
}

impl Pooled for MeleeAttackCategoryInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_attack_category_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_attack_category_info }
}

impl<'a> Extract<'a> for MeleeAttackCategoryInfo {
    const TYPE_NAME: &'static str = "MeleeAttackCategoryInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            attack_category_params: match inst.get("attackCategoryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AttackCategoryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AttackCategoryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MeleeAttackInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeAttackInfo {
    /// DCB field: `meleeAttacks` (Class (array))
    #[serde(default)]
    pub melee_attacks: Vec<Handle<MeleeFragInfo>>,
}

impl Pooled for MeleeAttackInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_attack_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_attack_info }
}

impl<'a> Extract<'a> for MeleeAttackInfo {
    const TYPE_NAME: &'static str = "MeleeAttackInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            melee_attacks: inst.get_array("meleeAttacks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeFragInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeFragInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MeleeCombatConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeCombatConfig {
    /// DCB field: `attackNodes` (Class (array))
    #[serde(default)]
    pub attack_nodes: Vec<Handle<MeleeAttackInfo>>,
    /// DCB field: `attackNodesZeroG` (Class (array))
    #[serde(default)]
    pub attack_nodes_zero_g: Vec<Handle<MeleeAttackInfo>>,
    /// DCB field: `meleeAttackCategoryInfo` (Class (array))
    #[serde(default)]
    pub melee_attack_category_info: Vec<Handle<MeleeAttackCategoryInfo>>,
    /// DCB field: `attackCategoryParams` (Class (array))
    #[serde(default)]
    pub attack_category_params: Vec<Handle<AttackCategoryParams>>,
    /// DCB field: `zLockTriggerTime` (Single)
    #[serde(default)]
    pub z_lock_trigger_time: f32,
    /// DCB field: `zLockStickyFilter` (Reference)
    #[serde(default)]
    pub z_lock_sticky_filter: Option<CigGuid>,
}

impl Pooled for MeleeCombatConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_combat_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_combat_config }
}

impl<'a> Extract<'a> for MeleeCombatConfig {
    const TYPE_NAME: &'static str = "MeleeCombatConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attack_nodes: inst.get_array("attackNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            attack_nodes_zero_g: inst.get_array("attackNodesZeroG")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            melee_attack_category_info: inst.get_array("meleeAttackCategoryInfo")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            attack_category_params: inst.get_array("attackCategoryParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AttackCategoryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AttackCategoryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            z_lock_trigger_time: inst.get_f32("zLockTriggerTime").unwrap_or_default(),
            z_lock_sticky_filter: inst.get("zLockStickyFilter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MeleeComboChainLink`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeComboChainLink {
    /// DCB field: `attackType` (EnumChoice)
    #[serde(default)]
    pub attack_type: String,
    /// DCB field: `classType` (EnumChoice)
    #[serde(default)]
    pub class_type: String,
}

impl Pooled for MeleeComboChainLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_combo_chain_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_combo_chain_link }
}

impl<'a> Extract<'a> for MeleeComboChainLink {
    const TYPE_NAME: &'static str = "MeleeComboChainLink";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_type: inst.get_str("attackType").map(String::from).unwrap_or_default(),
            class_type: inst.get_str("classType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MeleeAttackCombo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeAttackCombo {
    /// DCB field: `comboChain` (Class (array))
    #[serde(default)]
    pub combo_chain: Vec<Handle<MeleeComboChainLink>>,
    /// DCB field: `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
}

impl Pooled for MeleeAttackCombo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_me.melee_attack_combo }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_me.melee_attack_combo }
}

impl<'a> Extract<'a> for MeleeAttackCombo {
    const TYPE_NAME: &'static str = "MeleeAttackCombo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            combo_chain: inst.get_array("comboChain")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeComboChainLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeComboChainLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cooldown: inst.get_f32("cooldown").unwrap_or_default(),
        }
    }
}

