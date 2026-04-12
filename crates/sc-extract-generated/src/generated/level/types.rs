// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `level`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Level`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    /// `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// `locDisplayName` (Locale)
    #[serde(default)]
    pub loc_display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `displayFeatures` (StrongPointer)
    #[serde(default)]
    pub display_features: Option<Handle<SCExtendedLocalizationLevelParams>>,
    /// `thumbnail` (String)
    #[serde(default)]
    pub thumbnail: String,
    /// `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// `universeLocationUniqueId` (UInt64)
    #[serde(default)]
    pub universe_location_unique_id: u64,
    /// `defaultGameRules` (Reference)
    #[serde(default)]
    pub default_game_rules: Option<CigGuid>,
    /// `validGameRules` (Reference (array))
    #[serde(default)]
    pub valid_game_rules: Vec<CigGuid>,
    /// `loadingScreenInfo` (Class)
    #[serde(default)]
    pub loading_screen_info: Option<Handle<SLoadingScreenInformationDef>>,
    /// `potentialSpawnLocations` (Reference (array))
    #[serde(default)]
    pub potential_spawn_locations: Vec<CigGuid>,
    /// `potentialDevOnlySpawnLocations` (Reference (array))
    #[serde(default)]
    pub potential_dev_only_spawn_locations: Vec<CigGuid>,
    /// `systemImagePath` (String)
    #[serde(default)]
    pub system_image_path: String,
}

impl Pooled for Level {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.level.level }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.level.level }
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

