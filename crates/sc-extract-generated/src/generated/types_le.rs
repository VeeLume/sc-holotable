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

/// DCB type: `LeanConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanConnection {
    /// DCB field: `waitUntilFinished` (Boolean)
    #[serde(default)]
    pub wait_until_finished: bool,
    /// DCB field: `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// DCB field: `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// DCB field: `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<LeanState>>,
}

impl Pooled for LeanConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.lean_connection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.lean_connection }
}

impl<'a> Extract<'a> for LeanConnection {
    const TYPE_NAME: &'static str = "LeanConnection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wait_until_finished: inst.get_bool("waitUntilFinished").unwrap_or_default(),
            delay_seconds: inst.get_f32("delaySeconds").unwrap_or_default(),
            wait_for_event: inst.get_str("waitForEvent").map(String::from).unwrap_or_default(),
            next_state: match inst.get("nextState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LeanState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LeanState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LeanState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanState {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// DCB field: `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// DCB field: `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<LeanConnection>>,
}

impl Pooled for LeanState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.lean_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.lean_state }
}

impl<'a> Extract<'a> for LeanState {
    const TYPE_NAME: &'static str = "LeanState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LeanConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LeanConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LeanGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanGraph {
    /// DCB field: `leanStates` (Class (array))
    #[serde(default)]
    pub lean_states: Vec<Handle<LeanState>>,
}

impl Pooled for LeanGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.lean_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.lean_graph }
}

impl<'a> Extract<'a> for LeanGraph {
    const TYPE_NAME: &'static str = "LeanGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lean_states: inst.get_array("leanStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LeanState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LeanState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LegacyCraftingRecipe_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipe_Base {
}

impl Pooled for LegacyCraftingRecipe_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.legacy_crafting_recipe_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.legacy_crafting_recipe_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipe_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipe_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeDef_Base {
}

impl Pooled for LegacyCraftingRecipeDef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.legacy_crafting_recipe_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.legacy_crafting_recipe_def_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDef_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDefRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeDefRecord {
    /// DCB field: `recipe` (StrongPointer)
    #[serde(default)]
    pub recipe: Option<Handle<LegacyCraftingRecipe_Base>>,
}

impl Pooled for LegacyCraftingRecipeDefRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.legacy_crafting_recipe_def_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.legacy_crafting_recipe_def_record }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDefRecord {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDefRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recipe: match inst.get("recipe") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LegacyCraftingRecipe_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LegacyCraftingRecipe_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LegacyCraftingRecipeListRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeListRecord {
    /// DCB field: `recipes` (StrongPointer (array))
    #[serde(default)]
    pub recipes: Vec<Handle<LegacyCraftingRecipeDef_Base>>,
}

impl Pooled for LegacyCraftingRecipeListRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.legacy_crafting_recipe_list_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.legacy_crafting_recipe_list_record }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeListRecord {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeListRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recipes: inst.get_array("recipes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LegacyCraftingRecipeDef_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LegacyCraftingRecipeDef_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Level`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    /// DCB field: `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// DCB field: `locDisplayName` (Locale)
    #[serde(default)]
    pub loc_display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `displayFeatures` (StrongPointer)
    #[serde(default)]
    pub display_features: Option<Handle<SCExtendedLocalizationLevelParams>>,
    /// DCB field: `thumbnail` (String)
    #[serde(default)]
    pub thumbnail: String,
    /// DCB field: `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// DCB field: `universeLocationUniqueId` (UInt64)
    #[serde(default)]
    pub universe_location_unique_id: u64,
    /// DCB field: `defaultGameRules` (Reference)
    #[serde(default)]
    pub default_game_rules: Option<CigGuid>,
    /// DCB field: `validGameRules` (Reference (array))
    #[serde(default)]
    pub valid_game_rules: Vec<CigGuid>,
    /// DCB field: `loadingScreenInfo` (Class)
    #[serde(default)]
    pub loading_screen_info: Option<Handle<SLoadingScreenInformationDef>>,
    /// DCB field: `potentialSpawnLocations` (Reference (array))
    #[serde(default)]
    pub potential_spawn_locations: Vec<CigGuid>,
    /// DCB field: `potentialDevOnlySpawnLocations` (Reference (array))
    #[serde(default)]
    pub potential_dev_only_spawn_locations: Vec<CigGuid>,
    /// DCB field: `systemImagePath` (String)
    #[serde(default)]
    pub system_image_path: String,
}

impl Pooled for Level {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.level }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.level }
}

impl<'a> Extract<'a> for Level {
    const TYPE_NAME: &'static str = "Level";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_str("id").map(String::from).unwrap_or_default(),
            loc_display_name: inst.get_str("locDisplayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            display_features: match inst.get("displayFeatures") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thumbnail: inst.get_str("thumbnail").map(String::from).unwrap_or_default(),
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            universe_location_unique_id: inst.get("universeLocationUniqueId").and_then(|v| v.as_u64()).unwrap_or_default(),
            default_game_rules: inst.get("defaultGameRules").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            valid_game_rules: inst.get_array("validGameRules")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            loading_screen_info: match inst.get("loadingScreenInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLoadingScreenInformationDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLoadingScreenInformationDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            potential_spawn_locations: inst.get_array("potentialSpawnLocations")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            potential_dev_only_spawn_locations: inst.get_array("potentialDevOnlySpawnLocations")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            system_image_path: inst.get_str("systemImagePath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `LedgeNearbyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeNearbyParams {
    /// DCB field: `maxDistanceIdle` (Single)
    #[serde(default)]
    pub max_distance_idle: f32,
    /// DCB field: `maxDistanceMovement` (Single)
    #[serde(default)]
    pub max_distance_movement: f32,
    /// DCB field: `maxDistanceInAirIdle` (Single)
    #[serde(default)]
    pub max_distance_in_air_idle: f32,
    /// DCB field: `maxDistanceInAirMovement` (Single)
    #[serde(default)]
    pub max_distance_in_air_movement: f32,
    /// DCB field: `minHeight` (Single)
    #[serde(default)]
    pub min_height: f32,
    /// DCB field: `maxHeightFromGround` (Single)
    #[serde(default)]
    pub max_height_from_ground: f32,
    /// DCB field: `maxHeightInAir` (Single)
    #[serde(default)]
    pub max_height_in_air: f32,
    /// DCB field: `maxSearchAngleDeg` (Single)
    #[serde(default)]
    pub max_search_angle_deg: f32,
    /// DCB field: `inAirInvalidUpperSearchEdgeHeight` (Single)
    #[serde(default)]
    pub in_air_invalid_upper_search_edge_height: f32,
    /// DCB field: `inAirInvalidUpperSearchEdgeDepth` (Single)
    #[serde(default)]
    pub in_air_invalid_upper_search_edge_depth: f32,
    /// DCB field: `searchDir` (Class)
    #[serde(default)]
    pub search_dir: Option<Handle<Vec3>>,
}

impl Pooled for LedgeNearbyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.ledge_nearby_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.ledge_nearby_params }
}

impl<'a> Extract<'a> for LedgeNearbyParams {
    const TYPE_NAME: &'static str = "LedgeNearbyParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_distance_idle: inst.get_f32("maxDistanceIdle").unwrap_or_default(),
            max_distance_movement: inst.get_f32("maxDistanceMovement").unwrap_or_default(),
            max_distance_in_air_idle: inst.get_f32("maxDistanceInAirIdle").unwrap_or_default(),
            max_distance_in_air_movement: inst.get_f32("maxDistanceInAirMovement").unwrap_or_default(),
            min_height: inst.get_f32("minHeight").unwrap_or_default(),
            max_height_from_ground: inst.get_f32("maxHeightFromGround").unwrap_or_default(),
            max_height_in_air: inst.get_f32("maxHeightInAir").unwrap_or_default(),
            max_search_angle_deg: inst.get_f32("maxSearchAngleDeg").unwrap_or_default(),
            in_air_invalid_upper_search_edge_height: inst.get_f32("inAirInvalidUpperSearchEdgeHeight").unwrap_or_default(),
            in_air_invalid_upper_search_edge_depth: inst.get_f32("inAirInvalidUpperSearchEdgeDepth").unwrap_or_default(),
            search_dir: match inst.get("searchDir") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LedgeTransitionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeTransitionParams {
    /// DCB field: `minHeight` (Single)
    #[serde(default)]
    pub min_height: f32,
    /// DCB field: `lowMaxHeight` (Single)
    #[serde(default)]
    pub low_max_height: f32,
    /// DCB field: `mediumMaxHeight` (Single)
    #[serde(default)]
    pub medium_max_height: f32,
    /// DCB field: `highMaxHeight` (Single)
    #[serde(default)]
    pub high_max_height: f32,
    /// DCB field: `ultraMaxHeight` (Single)
    #[serde(default)]
    pub ultra_max_height: f32,
    /// DCB field: `lowMinWarpDepth` (Single)
    #[serde(default)]
    pub low_min_warp_depth: f32,
    /// DCB field: `mediumMinWarpDepth` (Single)
    #[serde(default)]
    pub medium_min_warp_depth: f32,
    /// DCB field: `highMinWarpDepth` (Single)
    #[serde(default)]
    pub high_min_warp_depth: f32,
    /// DCB field: `ultraMinWarpDepth` (Single)
    #[serde(default)]
    pub ultra_min_warp_depth: f32,
    /// DCB field: `walkSlowTimeScale` (Single)
    #[serde(default)]
    pub walk_slow_time_scale: f32,
    /// DCB field: `walkFastTimeScale` (Single)
    #[serde(default)]
    pub walk_fast_time_scale: f32,
    /// DCB field: `runSlowTimeScale` (Single)
    #[serde(default)]
    pub run_slow_time_scale: f32,
    /// DCB field: `runFastTimeScale` (Single)
    #[serde(default)]
    pub run_fast_time_scale: f32,
    /// DCB field: `sprintTimeScale` (Single)
    #[serde(default)]
    pub sprint_time_scale: f32,
}

impl Pooled for LedgeTransitionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.ledge_transition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.ledge_transition_params }
}

impl<'a> Extract<'a> for LedgeTransitionParams {
    const TYPE_NAME: &'static str = "LedgeTransitionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_height: inst.get_f32("minHeight").unwrap_or_default(),
            low_max_height: inst.get_f32("lowMaxHeight").unwrap_or_default(),
            medium_max_height: inst.get_f32("mediumMaxHeight").unwrap_or_default(),
            high_max_height: inst.get_f32("highMaxHeight").unwrap_or_default(),
            ultra_max_height: inst.get_f32("ultraMaxHeight").unwrap_or_default(),
            low_min_warp_depth: inst.get_f32("lowMinWarpDepth").unwrap_or_default(),
            medium_min_warp_depth: inst.get_f32("mediumMinWarpDepth").unwrap_or_default(),
            high_min_warp_depth: inst.get_f32("highMinWarpDepth").unwrap_or_default(),
            ultra_min_warp_depth: inst.get_f32("ultraMinWarpDepth").unwrap_or_default(),
            walk_slow_time_scale: inst.get_f32("walkSlowTimeScale").unwrap_or_default(),
            walk_fast_time_scale: inst.get_f32("walkFastTimeScale").unwrap_or_default(),
            run_slow_time_scale: inst.get_f32("runSlowTimeScale").unwrap_or_default(),
            run_fast_time_scale: inst.get_f32("runFastTimeScale").unwrap_or_default(),
            sprint_time_scale: inst.get_f32("sprintTimeScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `LedgeGrabbingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeGrabbingParams {
    /// DCB field: `ledgeNearbyParams` (Class)
    #[serde(default)]
    pub ledge_nearby_params: Option<Handle<LedgeNearbyParams>>,
    /// DCB field: `vaultTransitionParams` (Class)
    #[serde(default)]
    pub vault_transition_params: Option<Handle<LedgeTransitionParams>>,
    /// DCB field: `mantleTransitionParams` (Class)
    #[serde(default)]
    pub mantle_transition_params: Option<Handle<LedgeTransitionParams>>,
}

impl Pooled for LedgeGrabbingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_le.ledge_grabbing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_le.ledge_grabbing_params }
}

impl<'a> Extract<'a> for LedgeGrabbingParams {
    const TYPE_NAME: &'static str = "LedgeGrabbingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ledge_nearby_params: match inst.get("ledgeNearbyParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeNearbyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeNearbyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vault_transition_params: match inst.get("vaultTransitionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeTransitionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeTransitionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mantle_transition_params: match inst.get("mantleTransitionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeTransitionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeTransitionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

