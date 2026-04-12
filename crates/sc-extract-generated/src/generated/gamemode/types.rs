// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `gamemode`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCustomizableMaterialEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomizableMaterialEntry {
    /// `guid` (Guid)
    #[serde(default)]
    pub guid: CigGuid,
    /// `filePath` (String)
    #[serde(default)]
    pub file_path: String,
}

impl Pooled for SCustomizableMaterialEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scustomizable_material_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scustomizable_material_entry }
}

impl<'a> Extract<'a> for SCustomizableMaterialEntry {
    const TYPE_NAME: &'static str = "SCustomizableMaterialEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            guid: inst.get_guid("guid").unwrap_or_default(),
            file_path: inst.get_str("filePath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCustomizableMaterialLookupTable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomizableMaterialLookupTable {
    /// `materials` (Class (array))
    #[serde(default)]
    pub materials: Vec<Handle<SCustomizableMaterialEntry>>,
    /// `textures` (Class (array))
    #[serde(default)]
    pub textures: Vec<Handle<SCustomizableMaterialEntry>>,
    /// `decals` (Class (array))
    #[serde(default)]
    pub decals: Vec<Handle<SCustomizableMaterialEntry>>,
}

impl Pooled for SCustomizableMaterialLookupTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scustomizable_material_lookup_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scustomizable_material_lookup_table }
}

impl<'a> Extract<'a> for SCustomizableMaterialLookupTable {
    const TYPE_NAME: &'static str = "SCustomizableMaterialLookupTable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            materials: inst.get_array("materials")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizableMaterialEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            textures: inst.get_array("textures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizableMaterialEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            decals: inst.get_array("decals")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizableMaterialEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterArchetypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterArchetypeParams {
    /// `useEthnicity` (Boolean)
    #[serde(default)]
    pub use_ethnicity: bool,
    /// `ethnicityRanges` (Class)
    #[serde(default)]
    pub ethnicity_ranges: Option<Handle<Vec2>>,
    /// `useAgeRange` (Boolean)
    #[serde(default)]
    pub use_age_range: bool,
    /// `ageRange` (Class)
    #[serde(default)]
    pub age_range: Option<Handle<Vec2>>,
    /// `useBuildRanges` (Boolean)
    #[serde(default)]
    pub use_build_ranges: bool,
    /// `buildRanges` (Class)
    #[serde(default)]
    pub build_ranges: Option<Handle<Vec2>>,
}

impl Pooled for SCharacterArchetypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scharacter_archetype_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scharacter_archetype_params }
}

impl<'a> Extract<'a> for SCharacterArchetypeParams {
    const TYPE_NAME: &'static str = "SCharacterArchetypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_ethnicity: inst.get_bool("useEthnicity").unwrap_or_default(),
            ethnicity_ranges: match inst.get("ethnicityRanges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_age_range: inst.get_bool("useAgeRange").unwrap_or_default(),
            age_range: match inst.get("ageRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_build_ranges: inst.get_bool("useBuildRanges").unwrap_or_default(),
            build_ranges: match inst.get("buildRanges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterArchetype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterArchetype {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `headID` (Int32)
    #[serde(default)]
    pub head_id: i32,
    /// `getParamsFromHeadId` (Boolean)
    #[serde(default)]
    pub get_params_from_head_id: bool,
    /// `archetypeParams` (Class)
    #[serde(default)]
    pub archetype_params: Option<Handle<SCharacterArchetypeParams>>,
    /// `materialEntries` (WeakPointer (array))
    #[serde(default)]
    pub material_entries: Vec<Handle<SCustomizableMaterialEntry>>,
    /// `baseLoadoutOverride` (StrongPointer)
    #[serde(default)]
    pub base_loadout_override: Option<Handle<SArchetypeAssetDefBase>>,
    /// `assets` (Class (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetList>>,
}

impl Pooled for SCharacterArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scharacter_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scharacter_archetype }
}

impl<'a> Extract<'a> for SCharacterArchetype {
    const TYPE_NAME: &'static str = "SCharacterArchetype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            head_id: inst.get_i32("headID").unwrap_or_default(),
            get_params_from_head_id: inst.get_bool("getParamsFromHeadId").unwrap_or_default(),
            archetype_params: match inst.get("archetypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterArchetypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCharacterArchetypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material_entries: inst.get_array("materialEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizableMaterialEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            base_loadout_override: match inst.get("baseLoadoutOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assets: inst.get_array("assets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SArchetypeAssetList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SArchetypeAssetList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterArchetypeList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterArchetypeList {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `dnaFilePath` (String)
    #[serde(default)]
    pub dna_file_path: String,
    /// `baseLoadout` (StrongPointer)
    #[serde(default)]
    pub base_loadout: Option<Handle<SArchetypeAssetDefBase>>,
    /// `modelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<Handle<SGeometryModelTagBase>>,
    /// `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
    /// `conditions` (Class (array))
    #[serde(default)]
    pub conditions: Vec<Handle<SAssetListCondition>>,
    /// `archetypes` (Class (array))
    #[serde(default)]
    pub archetypes: Vec<Handle<SCharacterArchetype>>,
    /// `bodyMaterial` (WeakPointer)
    #[serde(default)]
    pub body_material: Option<Handle<SCustomizableMaterialEntry>>,
    /// `bodySubmtls` (Int32 (array))
    #[serde(default)]
    pub body_submtls: Vec<i32>,
    /// `archetypeLists` (Class (array))
    #[serde(default)]
    pub archetype_lists: Vec<Handle<SCharacterArchetypeList>>,
}

impl Pooled for SCharacterArchetypeList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scharacter_archetype_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scharacter_archetype_list }
}

impl<'a> Extract<'a> for SCharacterArchetypeList {
    const TYPE_NAME: &'static str = "SCharacterArchetypeList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            dna_file_path: inst.get_str("dnaFilePath").map(String::from).unwrap_or_default(),
            base_loadout: match inst.get("baseLoadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            model_tag: match inst.get("modelTag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryModelTagBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryModelTagBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forbidden_tags: match inst.get("forbiddenTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SAssetListCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SAssetListCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            archetypes: inst.get_array("archetypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterArchetype>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterArchetype>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            body_material: match inst.get("bodyMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            body_submtls: inst.get_array("bodySubmtls")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            archetype_lists: inst.get_array("archetypeLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterArchetypeList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterArchetypeList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterGenerationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterGenerationParams {
    /// `materialLookupTable` (Class)
    #[serde(default)]
    pub material_lookup_table: Option<Handle<SCustomizableMaterialLookupTable>>,
    /// `archetypeLists` (Class (array))
    #[serde(default)]
    pub archetype_lists: Vec<Handle<SCharacterArchetypeList>>,
    /// `additiveAssetLists` (Class (array))
    #[serde(default)]
    pub additive_asset_lists: Vec<Handle<SCharacterArchetypeAdditiveAssetList>>,
    /// `allowedClasses` (Reference (array))
    #[serde(default)]
    pub allowed_classes: Vec<CigGuid>,
    /// `serializationPreset` (Reference)
    #[serde(default)]
    pub serialization_preset: Option<CigGuid>,
}

impl Pooled for SCharacterGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scharacter_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scharacter_generation_params }
}

impl<'a> Extract<'a> for SCharacterGenerationParams {
    const TYPE_NAME: &'static str = "SCharacterGenerationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material_lookup_table: match inst.get("materialLookupTable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCustomizableMaterialLookupTable>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialLookupTable>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            archetype_lists: inst.get_array("archetypeLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterArchetypeList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterArchetypeList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            additive_asset_lists: inst.get_array("additiveAssetLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterArchetypeAdditiveAssetList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterArchetypeAdditiveAssetList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            allowed_classes: inst.get_array("allowedClasses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            serialization_preset: inst.get("serializationPreset").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SArchetypeAssetDefBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SArchetypeAssetDefBase {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
}

impl Pooled for SArchetypeAssetDefBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sarchetype_asset_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sarchetype_asset_def_base }
}

impl<'a> Extract<'a> for SArchetypeAssetDefBase {
    const TYPE_NAME: &'static str = "SArchetypeAssetDefBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SArchetypeAssetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SArchetypeAssetList {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `assets` (StrongPointer (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetDefBase>>,
}

impl Pooled for SArchetypeAssetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sarchetype_asset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sarchetype_asset_list }
}

impl<'a> Extract<'a> for SArchetypeAssetList {
    const TYPE_NAME: &'static str = "SArchetypeAssetList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            assets: inst.get_array("assets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SArchetypeAssetDefBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterArchetypeAdditiveAssetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterArchetypeAdditiveAssetList {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
    /// `conditions` (Class (array))
    #[serde(default)]
    pub conditions: Vec<Handle<SAssetListCondition>>,
    /// `assets` (StrongPointer (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetDefBase>>,
    /// `childLists` (Class (array))
    #[serde(default)]
    pub child_lists: Vec<Handle<SCharacterArchetypeAdditiveAssetList>>,
}

impl Pooled for SCharacterArchetypeAdditiveAssetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.scharacter_archetype_additive_asset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.scharacter_archetype_additive_asset_list }
}

impl<'a> Extract<'a> for SCharacterArchetypeAdditiveAssetList {
    const TYPE_NAME: &'static str = "SCharacterArchetypeAdditiveAssetList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forbidden_tags: match inst.get("forbiddenTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SAssetListCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SAssetListCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            assets: inst.get_array("assets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SArchetypeAssetDefBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SArchetypeAssetDefBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            child_lists: inst.get_array("childLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterArchetypeAdditiveAssetList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterArchetypeAdditiveAssetList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SAssetListCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAssetListCondition {
    /// `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
}

impl Pooled for SAssetListCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sasset_list_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sasset_list_condition }
}

impl<'a> Extract<'a> for SAssetListCondition {
    const TYPE_NAME: &'static str = "SAssetListCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forbidden_tags: match inst.get("forbiddenTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshotEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshotEntry {
    /// `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for SEAPlayerLoadoutSnapshotEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaplayer_loadout_snapshot_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaplayer_loadout_snapshot_entry }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshotEntry {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshotEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshots`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshots {
    /// `entries` (Class)
    #[serde(default)]
    pub entries: Option<Handle<SEAPlayerLoadoutSnapshotEntry>>,
}

impl Pooled for SEAPlayerLoadoutSnapshots {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaplayer_loadout_snapshots }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaplayer_loadout_snapshots }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshots {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshots";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: match inst.get("entries") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEALoadoutAttachment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutAttachment {
    /// `attachmentSlot` (EnumChoice)
    #[serde(default)]
    pub attachment_slot: String,
    /// `attachementClass` (Reference)
    #[serde(default)]
    pub attachement_class: Option<CigGuid>,
}

impl Pooled for SEALoadoutAttachment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sealoadout_attachment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sealoadout_attachment }
}

impl<'a> Extract<'a> for SEALoadoutAttachment {
    const TYPE_NAME: &'static str = "SEALoadoutAttachment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attachment_slot: inst.get_str("attachmentSlot").map(String::from).unwrap_or_default(),
            attachement_class: inst.get("attachementClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEALoadoutExplicit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutExplicit {
    /// `itemPort` (String)
    #[serde(default)]
    pub item_port: String,
    /// `itemClass` (Reference)
    #[serde(default)]
    pub item_class: Option<CigGuid>,
}

impl Pooled for SEALoadoutExplicit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sealoadout_explicit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sealoadout_explicit }
}

impl<'a> Extract<'a> for SEALoadoutExplicit {
    const TYPE_NAME: &'static str = "SEALoadoutExplicit";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_port: inst.get_str("itemPort").map(String::from).unwrap_or_default(),
            item_class: inst.get("itemClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEALoadoutItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutItem {
    /// `itemSlot` (EnumChoice)
    #[serde(default)]
    pub item_slot: String,
    /// `itemClass` (Reference)
    #[serde(default)]
    pub item_class: Option<CigGuid>,
    /// `itemAttachements` (Class (array))
    #[serde(default)]
    pub item_attachements: Vec<Handle<SEALoadoutAttachment>>,
    /// `itemExplicit` (Class (array))
    #[serde(default)]
    pub item_explicit: Vec<Handle<SEALoadoutExplicit>>,
}

impl Pooled for SEALoadoutItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sealoadout_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sealoadout_item }
}

impl<'a> Extract<'a> for SEALoadoutItem {
    const TYPE_NAME: &'static str = "SEALoadoutItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_slot: inst.get_str("itemSlot").map(String::from).unwrap_or_default(),
            item_class: inst.get("itemClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            item_attachements: inst.get_array("itemAttachements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutAttachment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutAttachment>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_explicit: inst.get_array("itemExplicit")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutExplicit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutExplicit>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEALoadoutSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutSet {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// `itemSlots` (Class (array))
    #[serde(default)]
    pub item_slots: Vec<Handle<SEALoadoutItem>>,
}

impl Pooled for SEALoadoutSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sealoadout_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sealoadout_set }
}

impl<'a> Extract<'a> for SEALoadoutSet {
    const TYPE_NAME: &'static str = "SEALoadoutSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            item_slots: inst.get_array("itemSlots")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEALoadoutCollection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEALoadoutCollection {
    /// `availableLoadouts` (Class (array))
    #[serde(default)]
    pub available_loadouts: Vec<Handle<SEALoadoutSet>>,
}

impl Pooled for SEALoadoutCollection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sealoadout_collection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sealoadout_collection }
}

impl<'a> Extract<'a> for SEALoadoutCollection {
    const TYPE_NAME: &'static str = "SEALoadoutCollection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            available_loadouts: inst.get_array("availableLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEAGlobalSpecialLoadout`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAGlobalSpecialLoadout {
    /// `sharedLoadouts` (Class (array))
    #[serde(default)]
    pub shared_loadouts: Vec<Handle<SEALoadoutSet>>,
    /// `teamLoadouts` (Class (array))
    #[serde(default)]
    pub team_loadouts: Vec<Handle<SEALoadoutCollection>>,
}

impl Pooled for SEAGlobalSpecialLoadout {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaglobal_special_loadout }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaglobal_special_loadout }
}

impl<'a> Extract<'a> for SEAGlobalSpecialLoadout {
    const TYPE_NAME: &'static str = "SEAGlobalSpecialLoadout";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shared_loadouts: inst.get_array("sharedLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            team_loadouts: inst.get_array("teamLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEALoadoutCollection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SEALoadoutCollection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEAGlobalEventLoadouts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAGlobalEventLoadouts {
    /// `loadouts` (Reference)
    #[serde(default)]
    pub loadouts: Option<CigGuid>,
}

impl Pooled for SEAGlobalEventLoadouts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaglobal_event_loadouts }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaglobal_event_loadouts }
}

impl<'a> Extract<'a> for SEAGlobalEventLoadouts {
    const TYPE_NAME: &'static str = "SEAGlobalEventLoadouts";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loadouts: inst.get("loadouts").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SIRoundsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIRoundsModule {
}

impl Pooled for SIRoundsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sirounds_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sirounds_module }
}

impl<'a> Extract<'a> for SIRoundsModule {
    const TYPE_NAME: &'static str = "SIRoundsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityDefaultLoadoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefaultLoadoutParams {
    /// `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<Handle<SItemPortLoadoutBaseParams>>,
}

impl Pooled for EntityDefaultLoadoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.entity_default_loadout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.entity_default_loadout_params }
}

impl<'a> Extract<'a> for EntityDefaultLoadoutParams {
    const TYPE_NAME: &'static str = "EntityDefaultLoadoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortLoadoutBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SIPickupModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPickupModule {
}

impl Pooled for SIPickupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sipickup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sipickup_module }
}

impl<'a> Extract<'a> for SIPickupModule {
    const TYPE_NAME: &'static str = "SIPickupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIDamageHandlingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDamageHandlingModule {
}

impl Pooled for SIDamageHandlingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sidamage_handling_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sidamage_handling_module }
}

impl<'a> Extract<'a> for SIDamageHandlingModule {
    const TYPE_NAME: &'static str = "SIDamageHandlingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIHostilityModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIHostilityModule {
    /// `allowHostilityCheckOnQuit` (Boolean)
    #[serde(default)]
    pub allow_hostility_check_on_quit: bool,
}

impl Pooled for SIHostilityModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sihostility_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sihostility_module }
}

impl<'a> Extract<'a> for SIHostilityModule {
    const TYPE_NAME: &'static str = "SIHostilityModule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_hostility_check_on_quit: inst.get_bool("allowHostilityCheckOnQuit").unwrap_or_default(),
        }
    }
}

/// DCB type: `SISpectatorModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpectatorModule {
}

impl Pooled for SISpectatorModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sispectator_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sispectator_module }
}

impl<'a> Extract<'a> for SISpectatorModule {
    const TYPE_NAME: &'static str = "SISpectatorModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SScoreEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScoreEvent {
    /// `points` (Int32)
    #[serde(default)]
    pub points: i32,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SScoreEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sscore_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sscore_event }
}

impl<'a> Extract<'a> for SScoreEvent {
    const TYPE_NAME: &'static str = "SScoreEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            points: inst.get_i32("points").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SPlayerScoring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPlayerScoring {
    /// `playerScoreEvents` (Class (array))
    #[serde(default)]
    pub player_score_events: Vec<Handle<SScoreEvent>>,
}

impl Pooled for SPlayerScoring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.splayer_scoring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.splayer_scoring }
}

impl<'a> Extract<'a> for SPlayerScoring {
    const TYPE_NAME: &'static str = "SPlayerScoring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_score_events: inst.get_array("playerScoreEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScoreEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScoreEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STeamScoring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STeamScoring {
    /// `startTeamScore` (Int32)
    #[serde(default)]
    pub start_team_score: i32,
    /// `useScoreAsTime` (Boolean)
    #[serde(default)]
    pub use_score_as_time: bool,
    /// `teamScoreValue` (Int32)
    #[serde(default)]
    pub team_score_value: i32,
    /// `playerToTeamScoring` (EnumChoice (array))
    #[serde(default)]
    pub player_to_team_scoring: Vec<String>,
    /// `teamScoreEvents` (Class (array))
    #[serde(default)]
    pub team_score_events: Vec<Handle<SScoreEvent>>,
}

impl Pooled for STeamScoring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.steam_scoring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.steam_scoring }
}

impl<'a> Extract<'a> for STeamScoring {
    const TYPE_NAME: &'static str = "STeamScoring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_team_score: inst.get_i32("startTeamScore").unwrap_or_default(),
            use_score_as_time: inst.get_bool("useScoreAsTime").unwrap_or_default(),
            team_score_value: inst.get_i32("teamScoreValue").unwrap_or_default(),
            player_to_team_scoring: inst.get_array("playerToTeamScoring")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            team_score_events: inst.get_array("teamScoreEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScoreEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScoreEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EAGameCompletionAwardBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAGameCompletionAwardBaseParams {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for EAGameCompletionAwardBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.eagame_completion_award_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.eagame_completion_award_base_params }
}

impl<'a> Extract<'a> for EAGameCompletionAwardBaseParams {
    const TYPE_NAME: &'static str = "EAGameCompletionAwardBaseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIScoringModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIScoringModule {
    /// `countScoreDown` (Boolean)
    #[serde(default)]
    pub count_score_down: bool,
    /// `playerScoring` (StrongPointer)
    #[serde(default)]
    pub player_scoring: Option<Handle<SPlayerScoring>>,
    /// `teamScoring` (StrongPointer)
    #[serde(default)]
    pub team_scoring: Option<Handle<STeamScoring>>,
    /// `gameCompletionAward` (StrongPointer)
    #[serde(default)]
    pub game_completion_award: Option<Handle<EAGameCompletionAwardBaseParams>>,
    /// `RECMultiplier` (Single)
    #[serde(default)]
    pub recmultiplier: f32,
    /// `mvpType` (EnumChoice)
    #[serde(default)]
    pub mvp_type: String,
}

impl Pooled for SIScoringModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siscoring_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siscoring_module }
}

impl<'a> Extract<'a> for SIScoringModule {
    const TYPE_NAME: &'static str = "SIScoringModule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            count_score_down: inst.get_bool("countScoreDown").unwrap_or_default(),
            player_scoring: match inst.get("playerScoring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SPlayerScoring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SPlayerScoring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            team_scoring: match inst.get("teamScoring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STeamScoring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STeamScoring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_completion_award: match inst.get("gameCompletionAward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EAGameCompletionAwardBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EAGameCompletionAwardBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recmultiplier: inst.get_f32("RECMultiplier").unwrap_or_default(),
            mvp_type: inst.get_str("mvpType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIPlayerSetupModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerSetupModule {
}

impl Pooled for SIPlayerSetupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siplayer_setup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siplayer_setup_module }
}

impl<'a> Extract<'a> for SIPlayerSetupModule {
    const TYPE_NAME: &'static str = "SIPlayerSetupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStatsRecordingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStatsRecordingModule {
}

impl Pooled for SIStatsRecordingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sistats_recording_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sistats_recording_module }
}

impl<'a> Extract<'a> for SIStatsRecordingModule {
    const TYPE_NAME: &'static str = "SIStatsRecordingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SINotificationsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SINotificationsModule {
}

impl Pooled for SINotificationsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sinotifications_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sinotifications_module }
}

impl<'a> Extract<'a> for SINotificationsModule {
    const TYPE_NAME: &'static str = "SINotificationsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIObjectives`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIObjectives {
}

impl Pooled for SIObjectives {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siobjectives }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siobjectives }
}

impl<'a> Extract<'a> for SIObjectives {
    const TYPE_NAME: &'static str = "SIObjectives";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SICamerasModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SICamerasModule {
}

impl Pooled for SICamerasModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sicameras_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sicameras_module }
}

impl<'a> Extract<'a> for SICamerasModule {
    const TYPE_NAME: &'static str = "SICamerasModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIPlayerStats`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerStats {
}

impl Pooled for SIPlayerStats {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siplayer_stats }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siplayer_stats }
}

impl<'a> Extract<'a> for SIPlayerStats {
    const TYPE_NAME: &'static str = "SIPlayerStats";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISpawning`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpawning {
}

impl Pooled for SISpawning {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sispawning }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sispawning }
}

impl<'a> Extract<'a> for SISpawning {
    const TYPE_NAME: &'static str = "SISpawning";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVictoryConditionsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVictoryConditionsModule {
}

impl Pooled for SIVictoryConditionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sivictory_conditions_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sivictory_conditions_module }
}

impl<'a> Extract<'a> for SIVictoryConditionsModule {
    const TYPE_NAME: &'static str = "SIVictoryConditionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIParamsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIParamsModule {
}

impl Pooled for SIParamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siparams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siparams_module }
}

impl<'a> Extract<'a> for SIParamsModule {
    const TYPE_NAME: &'static str = "SIParamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISubsumptionMissionModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISubsumptionMissionModule {
}

impl Pooled for SISubsumptionMissionModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sisubsumption_mission_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sisubsumption_mission_module }
}

impl<'a> Extract<'a> for SISubsumptionMissionModule {
    const TYPE_NAME: &'static str = "SISubsumptionMissionModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameModeValidMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeValidMap {
    /// `levelReference` (Reference)
    #[serde(default)]
    pub level_reference: Option<CigGuid>,
    /// `devOnly` (Boolean)
    #[serde(default)]
    pub dev_only: bool,
}

impl Pooled for GameModeValidMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.game_mode_valid_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.game_mode_valid_map }
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
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `controlledTypes` (EnumChoice (array))
    #[serde(default)]
    pub controlled_types: Vec<String>,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// `step` (Single)
    #[serde(default)]
    pub step: f32,
    /// `defaultValueOverride` (Single)
    #[serde(default)]
    pub default_value_override: f32,
}

impl Pooled for GameModeCustomSetting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.game_mode_custom_setting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.game_mode_custom_setting }
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

/// DCB type: `ChatSystemOptionsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSystemOptionsModule {
}

impl Pooled for ChatSystemOptionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.chat_system_options_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.chat_system_options_module }
}

impl<'a> Extract<'a> for ChatSystemOptionsModule {
    const TYPE_NAME: &'static str = "ChatSystemOptionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameModeFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeFilter {
    /// `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// `visibleToPlayers` (Boolean)
    #[serde(default)]
    pub visible_to_players: bool,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for GameModeFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.game_mode_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.game_mode_filter }
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
    /// `id` (EnumChoice)
    #[serde(default)]
    pub id: String,
    /// `requiredPass` (EnumChoice)
    #[serde(default)]
    pub required_pass: String,
    /// `playedBadgeId` (EnumChoice)
    #[serde(default)]
    pub played_badge_id: String,
    /// `locDisplayName` (Locale)
    #[serde(default)]
    pub loc_display_name: String,
    /// `locSubtitle` (Locale)
    #[serde(default)]
    pub loc_subtitle: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `thumbnail` (String)
    #[serde(default)]
    pub thumbnail: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// `backgroundVideos` (String (array))
    #[serde(default)]
    pub background_videos: Vec<String>,
    /// `alias` (String)
    #[serde(default)]
    pub alias: String,
    /// `leaderboardMatchCode` (String)
    #[serde(default)]
    pub leaderboard_match_code: String,
    /// `loadingScreenInfo` (Class)
    #[serde(default)]
    pub loading_screen_info: Option<Handle<SLoadingScreenInformationDef>>,
    /// `shardPersistenceEnabled` (Boolean)
    #[serde(default)]
    pub shard_persistence_enabled: bool,
    /// `enableCustomization` (Boolean)
    #[serde(default)]
    pub enable_customization: bool,
    /// `validNetworkTypes` (EnumChoice (array))
    #[serde(default)]
    pub valid_network_types: Vec<String>,
    /// `filters` (Class (array))
    #[serde(default)]
    pub filters: Vec<Handle<GameModeFilter>>,
    /// `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<SIParamsModule>>,
    /// `bettingModule` (StrongPointer)
    #[serde(default)]
    pub betting_module: Option<Handle<SIBettingModule>>,
    /// `camerasModule` (StrongPointer)
    #[serde(default)]
    pub cameras_module: Option<Handle<SICamerasModule>>,
    /// `chatSystemOptions` (StrongPointer)
    #[serde(default)]
    pub chat_system_options: Option<Handle<ChatSystemOptionsModule>>,
    /// `damageHandling` (StrongPointer)
    #[serde(default)]
    pub damage_handling: Option<Handle<SIDamageHandlingModule>>,
    /// `difficultyModule` (StrongPointer)
    #[serde(default)]
    pub difficulty_module: Option<Handle<SIDifficultyModule>>,
    /// `hostility` (StrongPointer)
    #[serde(default)]
    pub hostility: Option<Handle<SIHostilityModule>>,
    /// `notifications` (StrongPointer)
    #[serde(default)]
    pub notifications: Option<Handle<SINotificationsModule>>,
    /// `objectives` (StrongPointer)
    #[serde(default)]
    pub objectives: Option<Handle<SIObjectives>>,
    /// `pickup` (StrongPointer)
    #[serde(default)]
    pub pickup: Option<Handle<SIPickupModule>>,
    /// `playerSetup` (StrongPointer)
    #[serde(default)]
    pub player_setup: Option<Handle<SIPlayerSetupModule>>,
    /// `playerStats` (StrongPointer)
    #[serde(default)]
    pub player_stats: Option<Handle<SIPlayerStats>>,
    /// `reputationModule` (StrongPointer)
    #[serde(default)]
    pub reputation_module: Option<Handle<SIReputationModule>>,
    /// `rounds` (StrongPointer)
    #[serde(default)]
    pub rounds: Option<Handle<SIRoundsModule>>,
    /// `stateModule` (StrongPointer)
    #[serde(default)]
    pub state_module: Option<Handle<SIStateModule>>,
    /// `scoring` (StrongPointer)
    #[serde(default)]
    pub scoring: Option<Handle<SIScoringModule>>,
    /// `spawning` (StrongPointer)
    #[serde(default)]
    pub spawning: Option<Handle<SISpawning>>,
    /// `spectator` (StrongPointer)
    #[serde(default)]
    pub spectator: Option<Handle<SISpectatorModule>>,
    /// `statsRecording` (StrongPointer)
    #[serde(default)]
    pub stats_recording: Option<Handle<SIStatsRecordingModule>>,
    /// `subsumptionMissionModule` (StrongPointer)
    #[serde(default)]
    pub subsumption_mission_module: Option<Handle<SISubsumptionMissionModule>>,
    /// `teams` (StrongPointer)
    #[serde(default)]
    pub teams: Option<Handle<SITeamsModule>>,
    /// `victoryConditions` (StrongPointer)
    #[serde(default)]
    pub victory_conditions: Option<Handle<SIVictoryConditionsModule>>,
    /// `votingModule` (StrongPointer)
    #[serde(default)]
    pub voting_module: Option<Handle<SIVotingModule>>,
    /// `mapSelection` (Boolean)
    #[serde(default)]
    pub map_selection: bool,
    /// `validMaps` (Class (array))
    #[serde(default)]
    pub valid_maps: Vec<Handle<GameModeValidMap>>,
    /// `customSettings` (Class (array))
    #[serde(default)]
    pub custom_settings: Vec<Handle<GameModeCustomSetting>>,
}

impl Pooled for GameMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.game_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.game_mode }
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

/// DCB type: `SIBettingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIBettingModule {
}

impl Pooled for SIBettingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sibetting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sibetting_module }
}

impl<'a> Extract<'a> for SIBettingModule {
    const TYPE_NAME: &'static str = "SIBettingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DifficultyModifierRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyModifierRange {
    /// `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// `exposedToPlayer` (Boolean)
    #[serde(default)]
    pub exposed_to_player: bool,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for DifficultyModifierRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.difficulty_modifier_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.difficulty_modifier_range }
}

impl<'a> Extract<'a> for DifficultyModifierRange {
    const TYPE_NAME: &'static str = "DifficultyModifierRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
            exposed_to_player: inst.get_bool("exposedToPlayer").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleDifficultyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleDifficultyParams {
    /// `damageModifierToAiVehicles` (Single)
    #[serde(default)]
    pub damage_modifier_to_ai_vehicles: f32,
    /// `damageModifierFromAiVehicles` (Single)
    #[serde(default)]
    pub damage_modifier_from_ai_vehicles: f32,
}

impl Pooled for VehicleDifficultyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.vehicle_difficulty_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.vehicle_difficulty_params }
}

impl<'a> Extract<'a> for VehicleDifficultyParams {
    const TYPE_NAME: &'static str = "VehicleDifficultyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_modifier_to_ai_vehicles: inst.get_f32("damageModifierToAiVehicles").unwrap_or_default(),
            damage_modifier_from_ai_vehicles: inst.get_f32("damageModifierFromAiVehicles").unwrap_or_default(),
        }
    }
}

/// DCB type: `DifficultyLevelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyLevelParams {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `displayHeader` (Locale)
    #[serde(default)]
    pub display_header: String,
    /// `displayBody` (Locale)
    #[serde(default)]
    pub display_body: String,
    /// `vehicleParams` (Class)
    #[serde(default)]
    pub vehicle_params: Option<Handle<VehicleDifficultyParams>>,
    /// `modifiers` (Class)
    #[serde(default)]
    pub modifiers: Option<Handle<DifficultyModifierRange>>,
}

impl Pooled for DifficultyLevelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.difficulty_level_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.difficulty_level_params }
}

impl<'a> Extract<'a> for DifficultyLevelParams {
    const TYPE_NAME: &'static str = "DifficultyLevelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_header: inst.get_str("displayHeader").map(String::from).unwrap_or_default(),
            display_body: inst.get_str("displayBody").map(String::from).unwrap_or_default(),
            vehicle_params: match inst.get("vehicleParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VehicleDifficultyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VehicleDifficultyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: match inst.get("modifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DifficultyModifierRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DifficultyModifierRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GameDifficultyModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDifficultyModifiers {
    /// `difficulties` (Class)
    #[serde(default)]
    pub difficulties: Option<Handle<DifficultyLevelParams>>,
}

impl Pooled for GameDifficultyModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.game_difficulty_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.game_difficulty_modifiers }
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

/// DCB type: `SIDifficultyModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDifficultyModule {
}

impl Pooled for SIDifficultyModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sidifficulty_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sidifficulty_module }
}

impl<'a> Extract<'a> for SIDifficultyModule {
    const TYPE_NAME: &'static str = "SIDifficultyModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIReputationModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIReputationModule {
}

impl Pooled for SIReputationModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sireputation_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sireputation_module }
}

impl<'a> Extract<'a> for SIReputationModule {
    const TYPE_NAME: &'static str = "SIReputationModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStateModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStateModule {
}

impl Pooled for SIStateModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sistate_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sistate_module }
}

impl<'a> Extract<'a> for SIStateModule {
    const TYPE_NAME: &'static str = "SIStateModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SITeamsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SITeamsModule {
}

impl Pooled for SITeamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.siteams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.siteams_module }
}

impl<'a> Extract<'a> for SITeamsModule {
    const TYPE_NAME: &'static str = "SITeamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVotingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVotingModule {
}

impl Pooled for SIVotingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.sivoting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.sivoting_module }
}

impl<'a> Extract<'a> for SIVotingModule {
    const TYPE_NAME: &'static str = "SIVotingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlayerShipRespawnShipInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawnShipInfo {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `RespawnWaitTime` (Int32)
    #[serde(default)]
    pub respawn_wait_time: i32,
    /// `InstantRespawnCost` (Int32)
    #[serde(default)]
    pub instant_respawn_cost: i32,
}

impl Pooled for PlayerShipRespawnShipInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.player_ship_respawn_ship_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.player_ship_respawn_ship_info }
}

impl<'a> Extract<'a> for PlayerShipRespawnShipInfo {
    const TYPE_NAME: &'static str = "PlayerShipRespawnShipInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            respawn_wait_time: inst.get_i32("RespawnWaitTime").unwrap_or_default(),
            instant_respawn_cost: inst.get_i32("InstantRespawnCost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerShipRespawn`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawn {
    /// `Ships` (Class (array))
    #[serde(default)]
    pub ships: Vec<Handle<PlayerShipRespawnShipInfo>>,
}

impl Pooled for PlayerShipRespawn {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.player_ship_respawn }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.player_ship_respawn }
}

impl<'a> Extract<'a> for PlayerShipRespawn {
    const TYPE_NAME: &'static str = "PlayerShipRespawn";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ships: inst.get_array("Ships")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

