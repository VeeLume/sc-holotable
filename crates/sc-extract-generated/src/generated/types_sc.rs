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

/// DCB type: `SCProneMotionGraphDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCProneMotionGraphDef {
    /// DCB field: `turnTriggerYawThreshold` (Single)
    #[serde(default)]
    pub turn_trigger_yaw_threshold: f32,
}

impl Pooled for SCProneMotionGraphDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scprone_motion_graph_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scprone_motion_graph_def }
}

impl<'a> Extract<'a> for SCProneMotionGraphDef {
    const TYPE_NAME: &'static str = "SCProneMotionGraphDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            turn_trigger_yaw_threshold: inst.get_f32("turnTriggerYawThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCZeroGLaunchParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCZeroGLaunchParams {
    /// DCB field: `maxLaunchSpeed` (Single)
    #[serde(default)]
    pub max_launch_speed: f32,
    /// DCB field: `launchRotationDuration` (Single)
    #[serde(default)]
    pub launch_rotation_duration: f32,
    /// DCB field: `launchEdgeCheckRadius` (Single)
    #[serde(default)]
    pub launch_edge_check_radius: f32,
    /// DCB field: `launchEdgeCheckDistance` (Single)
    #[serde(default)]
    pub launch_edge_check_distance: f32,
    /// DCB field: `launchEdgeSurfaceHoverCheckRadius` (Single)
    #[serde(default)]
    pub launch_edge_surface_hover_check_radius: f32,
    /// DCB field: `launchEdgeSurfaceHoverCheckDistance` (Single)
    #[serde(default)]
    pub launch_edge_surface_hover_check_distance: f32,
}

impl Pooled for SCZeroGLaunchParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.sczero_glaunch_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.sczero_glaunch_params }
}

impl<'a> Extract<'a> for SCZeroGLaunchParams {
    const TYPE_NAME: &'static str = "SCZeroGLaunchParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_launch_speed: inst.get_f32("maxLaunchSpeed").unwrap_or_default(),
            launch_rotation_duration: inst.get_f32("launchRotationDuration").unwrap_or_default(),
            launch_edge_check_radius: inst.get_f32("launchEdgeCheckRadius").unwrap_or_default(),
            launch_edge_check_distance: inst.get_f32("launchEdgeCheckDistance").unwrap_or_default(),
            launch_edge_surface_hover_check_radius: inst.get_f32("launchEdgeSurfaceHoverCheckRadius").unwrap_or_default(),
            launch_edge_surface_hover_check_distance: inst.get_f32("launchEdgeSurfaceHoverCheckDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCDefaultZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDefaultZeroGTraversalParams {
    /// DCB field: `zeroGLaunchParams` (Class)
    #[serde(default)]
    pub zero_glaunch_params: Option<Handle<SCZeroGLaunchParams>>,
}

impl Pooled for SCDefaultZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scdefault_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scdefault_zero_gtraversal_params }
}

impl<'a> Extract<'a> for SCDefaultZeroGTraversalParams {
    const TYPE_NAME: &'static str = "SCDefaultZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zero_glaunch_params: match inst.get("zeroGLaunchParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCZeroGLaunchParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCZeroGLaunchParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCOptionalZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCOptionalZeroGTraversalParams {
    /// DCB field: `activationTag` (Reference)
    #[serde(default)]
    pub activation_tag: Option<CigGuid>,
    /// DCB field: `zeroGLaunchParams` (StrongPointer)
    #[serde(default)]
    pub zero_glaunch_params: Option<Handle<SCZeroGLaunchParams>>,
}

impl Pooled for SCOptionalZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scoptional_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scoptional_zero_gtraversal_params }
}

impl<'a> Extract<'a> for SCOptionalZeroGTraversalParams {
    const TYPE_NAME: &'static str = "SCOptionalZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            activation_tag: inst.get("activationTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            zero_glaunch_params: match inst.get("zeroGLaunchParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCZeroGLaunchParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCZeroGLaunchParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCustomizableMaterialEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomizableMaterialEntry {
    /// DCB field: `guid` (Guid)
    #[serde(default)]
    pub guid: CigGuid,
    /// DCB field: `filePath` (String)
    #[serde(default)]
    pub file_path: String,
}

impl Pooled for SCustomizableMaterialEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scustomizable_material_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scustomizable_material_entry }
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
    /// DCB field: `materials` (Class (array))
    #[serde(default)]
    pub materials: Vec<Handle<SCustomizableMaterialEntry>>,
    /// DCB field: `textures` (Class (array))
    #[serde(default)]
    pub textures: Vec<Handle<SCustomizableMaterialEntry>>,
    /// DCB field: `decals` (Class (array))
    #[serde(default)]
    pub decals: Vec<Handle<SCustomizableMaterialEntry>>,
}

impl Pooled for SCustomizableMaterialLookupTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scustomizable_material_lookup_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scustomizable_material_lookup_table }
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
    /// DCB field: `useEthnicity` (Boolean)
    #[serde(default)]
    pub use_ethnicity: bool,
    /// DCB field: `ethnicityRanges` (Class)
    #[serde(default)]
    pub ethnicity_ranges: Option<Handle<Vec2>>,
    /// DCB field: `useAgeRange` (Boolean)
    #[serde(default)]
    pub use_age_range: bool,
    /// DCB field: `ageRange` (Class)
    #[serde(default)]
    pub age_range: Option<Handle<Vec2>>,
    /// DCB field: `useBuildRanges` (Boolean)
    #[serde(default)]
    pub use_build_ranges: bool,
    /// DCB field: `buildRanges` (Class)
    #[serde(default)]
    pub build_ranges: Option<Handle<Vec2>>,
}

impl Pooled for SCharacterArchetypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_archetype_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_archetype_params }
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
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `headID` (Int32)
    #[serde(default)]
    pub head_id: i32,
    /// DCB field: `getParamsFromHeadId` (Boolean)
    #[serde(default)]
    pub get_params_from_head_id: bool,
    /// DCB field: `archetypeParams` (Class)
    #[serde(default)]
    pub archetype_params: Option<Handle<SCharacterArchetypeParams>>,
    /// DCB field: `materialEntries` (WeakPointer (array))
    #[serde(default)]
    pub material_entries: Vec<Handle<SCustomizableMaterialEntry>>,
    /// DCB field: `baseLoadoutOverride` (StrongPointer)
    #[serde(default)]
    pub base_loadout_override: Option<Handle<SArchetypeAssetDefBase>>,
    /// DCB field: `assets` (Class (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetList>>,
}

impl Pooled for SCharacterArchetype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_archetype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_archetype }
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
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `dnaFilePath` (String)
    #[serde(default)]
    pub dna_file_path: String,
    /// DCB field: `baseLoadout` (StrongPointer)
    #[serde(default)]
    pub base_loadout: Option<Handle<SArchetypeAssetDefBase>>,
    /// DCB field: `modelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<Handle<SGeometryModelTagBase>>,
    /// DCB field: `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// DCB field: `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
    /// DCB field: `conditions` (Class (array))
    #[serde(default)]
    pub conditions: Vec<Handle<SAssetListCondition>>,
    /// DCB field: `archetypes` (Class (array))
    #[serde(default)]
    pub archetypes: Vec<Handle<SCharacterArchetype>>,
    /// DCB field: `bodyMaterial` (WeakPointer)
    #[serde(default)]
    pub body_material: Option<Handle<SCustomizableMaterialEntry>>,
    /// DCB field: `bodySubmtls` (Int32 (array))
    #[serde(default)]
    pub body_submtls: Vec<i32>,
    /// DCB field: `archetypeLists` (Class (array))
    #[serde(default)]
    pub archetype_lists: Vec<Handle<SCharacterArchetypeList>>,
}

impl Pooled for SCharacterArchetypeList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_archetype_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_archetype_list }
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
    /// DCB field: `materialLookupTable` (Class)
    #[serde(default)]
    pub material_lookup_table: Option<Handle<SCustomizableMaterialLookupTable>>,
    /// DCB field: `archetypeLists` (Class (array))
    #[serde(default)]
    pub archetype_lists: Vec<Handle<SCharacterArchetypeList>>,
    /// DCB field: `additiveAssetLists` (Class (array))
    #[serde(default)]
    pub additive_asset_lists: Vec<Handle<SCharacterArchetypeAdditiveAssetList>>,
    /// DCB field: `allowedClasses` (Reference (array))
    #[serde(default)]
    pub allowed_classes: Vec<CigGuid>,
    /// DCB field: `serializationPreset` (Reference)
    #[serde(default)]
    pub serialization_preset: Option<CigGuid>,
}

impl Pooled for SCharacterGenerationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_generation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_generation_params }
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

/// DCB type: `SCharacterSerializationTexture`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterSerializationTexture {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `slot` (Int32)
    #[serde(default)]
    pub slot: i32,
}

impl Pooled for SCharacterSerializationTexture {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_serialization_texture }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_serialization_texture }
}

impl<'a> Extract<'a> for SCharacterSerializationTexture {
    const TYPE_NAME: &'static str = "SCharacterSerializationTexture";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            slot: inst.get_i32("slot").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterSerializationMaterialsSettingsPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterSerializationMaterialsSettingsPreset {
    /// DCB field: `attachmentName` (String)
    #[serde(default)]
    pub attachment_name: String,
    /// DCB field: `shaderParamsToExport` (String (array))
    #[serde(default)]
    pub shader_params_to_export: Vec<String>,
    /// DCB field: `textureSlotsToExport` (Class (array))
    #[serde(default)]
    pub texture_slots_to_export: Vec<Handle<SCharacterSerializationTexture>>,
    /// DCB field: `includeMaterialGUID` (Boolean)
    #[serde(default)]
    pub include_material_guid: bool,
    /// DCB field: `subMaterials` (Int32 (array))
    #[serde(default)]
    pub sub_materials: Vec<i32>,
}

impl Pooled for SCharacterSerializationMaterialsSettingsPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_serialization_materials_settings_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_serialization_materials_settings_preset }
}

impl<'a> Extract<'a> for SCharacterSerializationMaterialsSettingsPreset {
    const TYPE_NAME: &'static str = "SCharacterSerializationMaterialsSettingsPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attachment_name: inst.get_str("attachmentName").map(String::from).unwrap_or_default(),
            shader_params_to_export: inst.get_array("shaderParamsToExport")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            texture_slots_to_export: inst.get_array("textureSlotsToExport")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterSerializationTexture>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterSerializationTexture>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            include_material_guid: inst.get_bool("includeMaterialGUID").unwrap_or_default(),
            sub_materials: inst.get_array("subMaterials")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterArchetypeAdditiveAssetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterArchetypeAdditiveAssetList {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagList>>,
    /// DCB field: `forbiddenTags` (Class)
    #[serde(default)]
    pub forbidden_tags: Option<Handle<TagList>>,
    /// DCB field: `conditions` (Class (array))
    #[serde(default)]
    pub conditions: Vec<Handle<SAssetListCondition>>,
    /// DCB field: `assets` (StrongPointer (array))
    #[serde(default)]
    pub assets: Vec<Handle<SArchetypeAssetDefBase>>,
    /// DCB field: `childLists` (Class (array))
    #[serde(default)]
    pub child_lists: Vec<Handle<SCharacterArchetypeAdditiveAssetList>>,
}

impl Pooled for SCharacterArchetypeAdditiveAssetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharacter_archetype_additive_asset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharacter_archetype_additive_asset_list }
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

/// DCB type: `SCDynamicRigIntensityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicRigIntensityParams {
    /// DCB field: `referenceLightMinIntensity` (Single)
    #[serde(default)]
    pub reference_light_min_intensity: f32,
    /// DCB field: `referenceLightMaxIntensity` (Single)
    #[serde(default)]
    pub reference_light_max_intensity: f32,
    /// DCB field: `rigLightMinIntensity` (Single)
    #[serde(default)]
    pub rig_light_min_intensity: f32,
    /// DCB field: `rigLightMaxIntensity` (Single)
    #[serde(default)]
    pub rig_light_max_intensity: f32,
    /// DCB field: `backupIntensity` (Single)
    #[serde(default)]
    pub backup_intensity: f32,
    /// DCB field: `maxSaturation` (Single)
    #[serde(default)]
    pub max_saturation: f32,
}

impl Pooled for SCDynamicRigIntensityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scdynamic_rig_intensity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scdynamic_rig_intensity_params }
}

impl<'a> Extract<'a> for SCDynamicRigIntensityParams {
    const TYPE_NAME: &'static str = "SCDynamicRigIntensityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reference_light_min_intensity: inst.get_f32("referenceLightMinIntensity").unwrap_or_default(),
            reference_light_max_intensity: inst.get_f32("referenceLightMaxIntensity").unwrap_or_default(),
            rig_light_min_intensity: inst.get_f32("rigLightMinIntensity").unwrap_or_default(),
            rig_light_max_intensity: inst.get_f32("rigLightMaxIntensity").unwrap_or_default(),
            backup_intensity: inst.get_f32("backupIntensity").unwrap_or_default(),
            max_saturation: inst.get_f32("maxSaturation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCDynamicRigLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicRigLightParams {
    /// DCB field: `lightRadius` (Single)
    #[serde(default)]
    pub light_radius: f32,
    /// DCB field: `bulbRadius` (Single)
    #[serde(default)]
    pub bulb_radius: f32,
    /// DCB field: `FOV` (Single)
    #[serde(default)]
    pub fov: f32,
    /// DCB field: `importance` (EnumChoice)
    #[serde(default)]
    pub importance: String,
    /// DCB field: `shadowCast` (Boolean)
    #[serde(default)]
    pub shadow_cast: bool,
    /// DCB field: `maxVisDistance` (Single)
    #[serde(default)]
    pub max_vis_distance: f32,
    /// DCB field: `distanceFadeRange` (Single)
    #[serde(default)]
    pub distance_fade_range: f32,
    /// DCB field: `maxShadowCastDistance` (Single)
    #[serde(default)]
    pub max_shadow_cast_distance: f32,
    /// DCB field: `focusOffset` (Class)
    #[serde(default)]
    pub focus_offset: Option<Handle<Vec3>>,
    /// DCB field: `intensity` (Class)
    #[serde(default)]
    pub intensity: Option<Handle<SCDynamicRigIntensityParams>>,
}

impl Pooled for SCDynamicRigLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scdynamic_rig_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scdynamic_rig_light_params }
}

impl<'a> Extract<'a> for SCDynamicRigLightParams {
    const TYPE_NAME: &'static str = "SCDynamicRigLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_radius: inst.get_f32("lightRadius").unwrap_or_default(),
            bulb_radius: inst.get_f32("bulbRadius").unwrap_or_default(),
            fov: inst.get_f32("FOV").unwrap_or_default(),
            importance: inst.get_str("importance").map(String::from).unwrap_or_default(),
            shadow_cast: inst.get_bool("shadowCast").unwrap_or_default(),
            max_vis_distance: inst.get_f32("maxVisDistance").unwrap_or_default(),
            distance_fade_range: inst.get_f32("distanceFadeRange").unwrap_or_default(),
            max_shadow_cast_distance: inst.get_f32("maxShadowCastDistance").unwrap_or_default(),
            focus_offset: match inst.get("focusOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            intensity: match inst.get("intensity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDynamicRigIntensityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDynamicRigIntensityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCDynamicLightingRigGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCDynamicLightingRigGlobalParams {
    /// DCB field: `projectorTexture` (String)
    #[serde(default)]
    pub projector_texture: String,
    /// DCB field: `offsetPerAxis` (Class)
    #[serde(default)]
    pub offset_per_axis: Option<Handle<Vec3>>,
    /// DCB field: `frameOfReferenceVerticalOffset` (Single)
    #[serde(default)]
    pub frame_of_reference_vertical_offset: f32,
    /// DCB field: `colorLerpTime` (Single)
    #[serde(default)]
    pub color_lerp_time: f32,
    /// DCB field: `positionLerpTime` (Single)
    #[serde(default)]
    pub position_lerp_time: f32,
    /// DCB field: `minIntensityScaleWhileLerping` (Single)
    #[serde(default)]
    pub min_intensity_scale_while_lerping: f32,
    /// DCB field: `minRepositionDistanceFromPlayer` (Single)
    #[serde(default)]
    pub min_reposition_distance_from_player: f32,
    /// DCB field: `minRepositionMovementDistance` (Single)
    #[serde(default)]
    pub min_reposition_movement_distance: f32,
    /// DCB field: `extendedProjectorFOVScale` (Single)
    #[serde(default)]
    pub extended_projector_fovscale: f32,
    /// DCB field: `allowRigWithHelmetOn` (Boolean)
    #[serde(default)]
    pub allow_rig_with_helmet_on: bool,
    /// DCB field: `lights` (Class)
    #[serde(default)]
    pub lights: Option<Handle<SCDynamicRigLightParams>>,
}

impl Pooled for SCDynamicLightingRigGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scdynamic_lighting_rig_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scdynamic_lighting_rig_global_params }
}

impl<'a> Extract<'a> for SCDynamicLightingRigGlobalParams {
    const TYPE_NAME: &'static str = "SCDynamicLightingRigGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            projector_texture: inst.get_str("projectorTexture").map(String::from).unwrap_or_default(),
            offset_per_axis: match inst.get("offsetPerAxis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frame_of_reference_vertical_offset: inst.get_f32("frameOfReferenceVerticalOffset").unwrap_or_default(),
            color_lerp_time: inst.get_f32("colorLerpTime").unwrap_or_default(),
            position_lerp_time: inst.get_f32("positionLerpTime").unwrap_or_default(),
            min_intensity_scale_while_lerping: inst.get_f32("minIntensityScaleWhileLerping").unwrap_or_default(),
            min_reposition_distance_from_player: inst.get_f32("minRepositionDistanceFromPlayer").unwrap_or_default(),
            min_reposition_movement_distance: inst.get_f32("minRepositionMovementDistance").unwrap_or_default(),
            extended_projector_fovscale: inst.get_f32("extendedProjectorFOVScale").unwrap_or_default(),
            allow_rig_with_helmet_on: inst.get_bool("allowRigWithHelmetOn").unwrap_or_default(),
            lights: match inst.get("lights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDynamicRigLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDynamicRigLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCombatTargeting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCombatTargeting {
    /// DCB field: `targetingMethodRecord` (Reference)
    #[serde(default)]
    pub targeting_method_record: Option<CigGuid>,
}

impl Pooled for SCombatTargeting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scombat_targeting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scombat_targeting }
}

impl<'a> Extract<'a> for SCombatTargeting {
    const TYPE_NAME: &'static str = "SCombatTargeting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method_record: inst.get("targetingMethodRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCEntranceItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCEntranceItem {
    /// DCB field: `entranceName` (Locale)
    #[serde(default)]
    pub entrance_name: String,
    /// DCB field: `jointName` (String)
    #[serde(default)]
    pub joint_name: String,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// DCB field: `shipState` (EnumChoice)
    #[serde(default)]
    pub ship_state: String,
}

impl Pooled for SCEntranceItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scentrance_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scentrance_item }
}

impl<'a> Extract<'a> for SCEntranceItem {
    const TYPE_NAME: &'static str = "SCEntranceItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entrance_name: inst.get_str("entranceName").map(String::from).unwrap_or_default(),
            joint_name: inst.get_str("jointName").map(String::from).unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_state: inst.get_str("shipState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCarryableIKInteraction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCarryableIKInteraction {
    /// DCB field: `helperName` (String)
    #[serde(default)]
    pub helper_name: String,
    /// DCB field: `targetOffset` (Class)
    #[serde(default)]
    pub target_offset: Option<Handle<QuatT>>,
    /// DCB field: `cdikTargetName` (String)
    #[serde(default)]
    pub cdik_target_name: String,
}

impl Pooled for SCarryableIKInteraction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scarryable_ikinteraction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scarryable_ikinteraction }
}

impl<'a> Extract<'a> for SCarryableIKInteraction {
    const TYPE_NAME: &'static str = "SCarryableIKInteraction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            helper_name: inst.get_str("helperName").map(String::from).unwrap_or_default(),
            target_offset: match inst.get("targetOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cdik_target_name: inst.get_str("cdikTargetName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCarryableIKInteractionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCarryableIKInteractionList {
    /// DCB field: `ikInteractions` (Class (array))
    #[serde(default)]
    pub ik_interactions: Vec<Handle<SCarryableIKInteraction>>,
}

impl Pooled for SCarryableIKInteractionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scarryable_ikinteraction_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scarryable_ikinteraction_list }
}

impl<'a> Extract<'a> for SCarryableIKInteractionList {
    const TYPE_NAME: &'static str = "SCarryableIKInteractionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ik_interactions: inst.get_array("ikInteractions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCarryableIKInteraction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCarryableIKInteraction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SChatChannelTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChatChannelTypeBase {
}

impl Pooled for SChatChannelTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.schat_channel_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.schat_channel_type_base }
}

impl<'a> Extract<'a> for SChatChannelTypeBase {
    const TYPE_NAME: &'static str = "SChatChannelTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SChatChannelFilterBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChatChannelFilterBase {
    /// DCB field: `chatChannelType` (StrongPointer (array))
    #[serde(default)]
    pub chat_channel_type: Vec<Handle<SChatChannelTypeBase>>,
}

impl Pooled for SChatChannelFilterBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.schat_channel_filter_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.schat_channel_filter_base }
}

impl<'a> Extract<'a> for SChatChannelFilterBase {
    const TYPE_NAME: &'static str = "SChatChannelFilterBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            chat_channel_type: inst.get_array("chatChannelType")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SChatChannelTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SChatChannelTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SChargeDrainHighlightOutlineValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChargeDrainHighlightOutlineValues {
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGB>>,
    /// DCB field: `occludedAlpha` (Single)
    #[serde(default)]
    pub occluded_alpha: f32,
    /// DCB field: `outlineWidth` (Single)
    #[serde(default)]
    pub outline_width: f32,
    /// DCB field: `outlineOnly` (Boolean)
    #[serde(default)]
    pub outline_only: bool,
}

impl Pooled for SChargeDrainHighlightOutlineValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharge_drain_highlight_outline_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharge_drain_highlight_outline_values }
}

impl<'a> Extract<'a> for SChargeDrainHighlightOutlineValues {
    const TYPE_NAME: &'static str = "SChargeDrainHighlightOutlineValues";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_alpha: inst.get_f32("occludedAlpha").unwrap_or_default(),
            outline_width: inst.get_f32("outlineWidth").unwrap_or_default(),
            outline_only: inst.get_bool("outlineOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SChargeDrainTargetStateOutlineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChargeDrainTargetStateOutlineParams {
    /// DCB field: `inoperableOutlineValues` (Class)
    #[serde(default)]
    pub inoperable_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// DCB field: `jumpstartRequiredOutlineValues` (Class)
    #[serde(default)]
    pub jumpstart_required_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// DCB field: `jumpstartPossibleOutlineValues` (Class)
    #[serde(default)]
    pub jumpstart_possible_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
    /// DCB field: `validTargetOutlineValues` (Class)
    #[serde(default)]
    pub valid_target_outline_values: Option<Handle<SChargeDrainHighlightOutlineValues>>,
}

impl Pooled for SChargeDrainTargetStateOutlineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharge_drain_target_state_outline_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharge_drain_target_state_outline_params }
}

impl<'a> Extract<'a> for SChargeDrainTargetStateOutlineParams {
    const TYPE_NAME: &'static str = "SChargeDrainTargetStateOutlineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            inoperable_outline_values: match inst.get("inoperableOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jumpstart_required_outline_values: match inst.get("jumpstartRequiredOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jumpstart_possible_outline_values: match inst.get("jumpstartPossibleOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            valid_target_outline_values: match inst.get("validTargetOutlineValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChargeDrainHighlightOutlineValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SChargeDrainCardParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChargeDrainCardParams {
    /// DCB field: `cardLerpSpeed` (Single)
    #[serde(default)]
    pub card_lerp_speed: f32,
    /// DCB field: `attachPointLerpSpeed` (Single)
    #[serde(default)]
    pub attach_point_lerp_speed: f32,
    /// DCB field: `closingDelay` (Single)
    #[serde(default)]
    pub closing_delay: f32,
    /// DCB field: `closingTransitionTime` (Single)
    #[serde(default)]
    pub closing_transition_time: f32,
    /// DCB field: `nearDistance` (Single)
    #[serde(default)]
    pub near_distance: f32,
    /// DCB field: `defaultScreenPos` (Class)
    #[serde(default)]
    pub default_screen_pos: Option<Handle<Vec2>>,
    /// DCB field: `maxDistScreenPosScale` (Single)
    #[serde(default)]
    pub max_dist_screen_pos_scale: f32,
}

impl Pooled for SChargeDrainCardParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scharge_drain_card_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scharge_drain_card_params }
}

impl<'a> Extract<'a> for SChargeDrainCardParams {
    const TYPE_NAME: &'static str = "SChargeDrainCardParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            card_lerp_speed: inst.get_f32("cardLerpSpeed").unwrap_or_default(),
            attach_point_lerp_speed: inst.get_f32("attachPointLerpSpeed").unwrap_or_default(),
            closing_delay: inst.get_f32("closingDelay").unwrap_or_default(),
            closing_transition_time: inst.get_f32("closingTransitionTime").unwrap_or_default(),
            near_distance: inst.get_f32("nearDistance").unwrap_or_default(),
            default_screen_pos: match inst.get("defaultScreenPos") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_dist_screen_pos_scale: inst.get_f32("maxDistScreenPosScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCuttableShapeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCuttableShapeDefinition {
    /// DCB field: `shapeNamePrefix` (String)
    #[serde(default)]
    pub shape_name_prefix: String,
}

impl Pooled for SCuttableShapeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scuttable_shape_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scuttable_shape_definition }
}

impl<'a> Extract<'a> for SCuttableShapeDefinition {
    const TYPE_NAME: &'static str = "SCuttableShapeDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            shape_name_prefix: inst.get_str("shapeNamePrefix").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCExtendedLocalizationLevelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCExtendedLocalizationLevelParams {
    /// DCB field: `LogoSimplifiedWhite` (String)
    #[serde(default)]
    pub logo_simplified_white: String,
    /// DCB field: `FrontendBackground` (String)
    #[serde(default)]
    pub frontend_background: String,
    /// DCB field: `History` (Locale)
    #[serde(default)]
    pub history: String,
    /// DCB field: `Callout1` (Locale)
    #[serde(default)]
    pub callout1: String,
    /// DCB field: `Callout2` (Locale)
    #[serde(default)]
    pub callout2: String,
    /// DCB field: `Callout3` (Locale)
    #[serde(default)]
    pub callout3: String,
    /// DCB field: `UIPriority` (Int32)
    #[serde(default)]
    pub uipriority: i32,
    /// DCB field: `locationAudioPlayTrigger` (Class)
    #[serde(default)]
    pub location_audio_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `locationAudioStopTrigger` (Class)
    #[serde(default)]
    pub location_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for SCExtendedLocalizationLevelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scextended_localization_level_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scextended_localization_level_params }
}

impl<'a> Extract<'a> for SCExtendedLocalizationLevelParams {
    const TYPE_NAME: &'static str = "SCExtendedLocalizationLevelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            logo_simplified_white: inst.get_str("LogoSimplifiedWhite").map(String::from).unwrap_or_default(),
            frontend_background: inst.get_str("FrontendBackground").map(String::from).unwrap_or_default(),
            history: inst.get_str("History").map(String::from).unwrap_or_default(),
            callout1: inst.get_str("Callout1").map(String::from).unwrap_or_default(),
            callout2: inst.get_str("Callout2").map(String::from).unwrap_or_default(),
            callout3: inst.get_str("Callout3").map(String::from).unwrap_or_default(),
            uipriority: inst.get_i32("UIPriority").unwrap_or_default(),
            location_audio_play_trigger: match inst.get("locationAudioPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_audio_stop_trigger: match inst.get("locationAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanWaveDetectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanWaveDetectionParams {
    /// DCB field: `requireFullChargeDetection` (Boolean)
    #[serde(default)]
    pub require_full_charge_detection: bool,
    /// DCB field: `reflectScanWaveChargeLevel` (Boolean)
    #[serde(default)]
    pub reflect_scan_wave_charge_level: bool,
}

impl Pooled for ScanWaveDetectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_wave_detection_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_wave_detection_params }
}

impl<'a> Extract<'a> for ScanWaveDetectionParams {
    const TYPE_NAME: &'static str = "ScanWaveDetectionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            require_full_charge_detection: inst.get_bool("requireFullChargeDetection").unwrap_or_default(),
            reflect_scan_wave_charge_level: inst.get_bool("reflectScanWaveChargeLevel").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSharedParams {
    /// DCB field: `enableNewBindings` (Boolean)
    #[serde(default)]
    pub enable_new_bindings: bool,
    /// DCB field: `requireLockedTarget` (Boolean)
    #[serde(default)]
    pub require_locked_target: bool,
    /// DCB field: `enableManualFocusScan` (Boolean)
    #[serde(default)]
    pub enable_manual_focus_scan: bool,
    /// DCB field: `enableAutoFocusScan` (Boolean)
    #[serde(default)]
    pub enable_auto_focus_scan: bool,
    /// DCB field: `enablePassiveScan` (Boolean)
    #[serde(default)]
    pub enable_passive_scan: bool,
    /// DCB field: `enablePingWaveScan` (Boolean)
    #[serde(default)]
    pub enable_ping_wave_scan: bool,
    /// DCB field: `allowPassiveUnlockContactType` (Boolean)
    #[serde(default)]
    pub allow_passive_unlock_contact_type: bool,
    /// DCB field: `enableActiveUnlockName` (Boolean)
    #[serde(default)]
    pub enable_active_unlock_name: bool,
    /// DCB field: `unscannedText` (Locale)
    #[serde(default)]
    pub unscanned_text: String,
    /// DCB field: `sfxParams` (Class)
    #[serde(default)]
    pub sfx_params: Option<Handle<ScanSFXSharedParams>>,
}

impl Pooled for ScanSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_shared_params }
}

impl<'a> Extract<'a> for ScanSharedParams {
    const TYPE_NAME: &'static str = "ScanSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_new_bindings: inst.get_bool("enableNewBindings").unwrap_or_default(),
            require_locked_target: inst.get_bool("requireLockedTarget").unwrap_or_default(),
            enable_manual_focus_scan: inst.get_bool("enableManualFocusScan").unwrap_or_default(),
            enable_auto_focus_scan: inst.get_bool("enableAutoFocusScan").unwrap_or_default(),
            enable_passive_scan: inst.get_bool("enablePassiveScan").unwrap_or_default(),
            enable_ping_wave_scan: inst.get_bool("enablePingWaveScan").unwrap_or_default(),
            allow_passive_unlock_contact_type: inst.get_bool("allowPassiveUnlockContactType").unwrap_or_default(),
            enable_active_unlock_name: inst.get_bool("enableActiveUnlockName").unwrap_or_default(),
            unscanned_text: inst.get_str("unscannedText").map(String::from).unwrap_or_default(),
            sfx_params: match inst.get("sfxParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanSFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanSFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanSFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSFXSharedParams {
    /// DCB field: `startScan` (Class)
    #[serde(default)]
    pub start_scan: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopScan` (Class)
    #[serde(default)]
    pub stop_scan: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `infoPopulate` (Class)
    #[serde(default)]
    pub info_populate: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ScanSFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_sfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_sfxshared_params }
}

impl<'a> Extract<'a> for ScanSFXSharedParams {
    const TYPE_NAME: &'static str = "ScanSFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_scan: match inst.get("startScan") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_scan: match inst.get("stopScan") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            info_populate: match inst.get("infoPopulate") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanInformationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInformationDef {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `scanDisplayLayout` (Reference)
    #[serde(default)]
    pub scan_display_layout: Option<CigGuid>,
    /// DCB field: `scanProcedures` (StrongPointer (array))
    #[serde(default)]
    pub scan_procedures: Vec<Handle<ScanProcedureParams>>,
}

impl Pooled for ScanInformationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_information_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_information_def }
}

impl<'a> Extract<'a> for ScanInformationDef {
    const TYPE_NAME: &'static str = "ScanInformationDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            scan_display_layout: inst.get("scanDisplayLayout").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            scan_procedures: inst.get_array("scanProcedures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScanProcedureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScanProcedureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanInformationTable`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInformationTable {
    /// DCB field: `defs` (Reference (array))
    #[serde(default)]
    pub defs: Vec<CigGuid>,
}

impl Pooled for ScanInformationTable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_information_table }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_information_table }
}

impl<'a> Extract<'a> for ScanInformationTable {
    const TYPE_NAME: &'static str = "ScanInformationTable";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            defs: inst.get_array("defs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanBoxoutIconDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanBoxoutIconDef {
    /// DCB field: `showConditions` (EnumChoice (array))
    #[serde(default)]
    pub show_conditions: Vec<String>,
    /// DCB field: `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
}

impl Pooled for ScanBoxoutIconDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_boxout_icon_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_boxout_icon_def }
}

impl<'a> Extract<'a> for ScanBoxoutIconDef {
    const TYPE_NAME: &'static str = "ScanBoxoutIconDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            show_conditions: inst.get_array("showConditions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanCustomData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCustomData {
    /// DCB field: `requiredTags` (Class)
    #[serde(default)]
    pub required_tags: Option<Handle<TagsDNFTerm>>,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displaySection` (EnumChoice)
    #[serde(default)]
    pub display_section: String,
    /// DCB field: `addToBoxoutDisplay` (Boolean)
    #[serde(default)]
    pub add_to_boxout_display: bool,
    /// DCB field: `displayInSameRowAsHeader` (Boolean)
    #[serde(default)]
    pub display_in_same_row_as_header: bool,
}

impl Pooled for ScanCustomData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_custom_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_custom_data }
}

impl<'a> Extract<'a> for ScanCustomData {
    const TYPE_NAME: &'static str = "ScanCustomData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNFTerm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_section: inst.get_str("displaySection").map(String::from).unwrap_or_default(),
            add_to_boxout_display: inst.get_bool("addToBoxoutDisplay").unwrap_or_default(),
            display_in_same_row_as_header: inst.get_bool("displayInSameRowAsHeader").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanCustomDataInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCustomDataInfo {
    /// DCB field: `scanProcedures` (Class (array))
    #[serde(default)]
    pub scan_procedures: Vec<Handle<CustomScanProcedureParams>>,
}

impl Pooled for ScanCustomDataInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_custom_data_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_custom_data_info }
}

impl<'a> Extract<'a> for ScanCustomDataInfo {
    const TYPE_NAME: &'static str = "ScanCustomDataInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scan_procedures: inst.get_array("scanProcedures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CustomScanProcedureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CustomScanProcedureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanCustomDataDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCustomDataDef {
    /// DCB field: `info` (Class)
    #[serde(default)]
    pub info: Option<Handle<ScanCustomDataInfo>>,
}

impl Pooled for ScanCustomDataDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_custom_data_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_custom_data_def }
}

impl<'a> Extract<'a> for ScanCustomDataDef {
    const TYPE_NAME: &'static str = "ScanCustomDataDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            info: match inst.get("info") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanCustomDataInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanCustomDataInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanDisplaySetupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplaySetupParams {
    /// DCB field: `scanTable` (EnumChoice)
    #[serde(default)]
    pub scan_table: String,
}

impl Pooled for ScanDisplaySetupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_display_setup_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_display_setup_params }
}

impl<'a> Extract<'a> for ScanDisplaySetupParams {
    const TYPE_NAME: &'static str = "ScanDisplaySetupParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scan_table: inst.get_str("scanTable").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanDisplaySectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplaySectionParams {
    /// DCB field: `displayInstances` (Reference (array))
    #[serde(default)]
    pub display_instances: Vec<CigGuid>,
}

impl Pooled for ScanDisplaySectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_display_section_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_display_section_params }
}

impl<'a> Extract<'a> for ScanDisplaySectionParams {
    const TYPE_NAME: &'static str = "ScanDisplaySectionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_instances: inst.get_array("displayInstances")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanDisplayInstanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplayInstanceParams {
    /// DCB field: `boxoutSetup` (StrongPointer)
    #[serde(default)]
    pub boxout_setup: Option<Handle<ScanDisplaySetupParams>>,
    /// DCB field: `displaySetup` (StrongPointer)
    #[serde(default)]
    pub display_setup: Option<Handle<ScanDisplaySetupParams>>,
}

impl Pooled for ScanDisplayInstanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_display_instance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_display_instance_params }
}

impl<'a> Extract<'a> for ScanDisplayInstanceParams {
    const TYPE_NAME: &'static str = "ScanDisplayInstanceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            boxout_setup: match inst.get("boxoutSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_setup: match inst.get("displaySetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScanDisplayLayoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplayLayoutParams {
    /// DCB field: `displaySections` (Class)
    #[serde(default)]
    pub display_sections: Option<Handle<ScanDisplaySectionParams>>,
    /// DCB field: `contactDisplay` (StrongPointer)
    #[serde(default)]
    pub contact_display: Option<Handle<ScanDisplaySetupParams>>,
    /// DCB field: `boxoutIconDefs` (Class (array))
    #[serde(default)]
    pub boxout_icon_defs: Vec<Handle<ScanBoxoutIconDef>>,
}

impl Pooled for ScanDisplayLayoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_display_layout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_display_layout_params }
}

impl<'a> Extract<'a> for ScanDisplayLayoutParams {
    const TYPE_NAME: &'static str = "ScanDisplayLayoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_sections: match inst.get("displaySections") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contact_display: match inst.get("contactDisplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanDisplaySetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanDisplaySetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            boxout_icon_defs: inst.get_array("boxoutIconDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScanBoxoutIconDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScanBoxoutIconDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScanProcedureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProcedureParams {
    /// DCB field: `requiredScanTag` (Reference)
    #[serde(default)]
    pub required_scan_tag: Option<CigGuid>,
    /// DCB field: `emissionBaseline` (Single)
    #[serde(default)]
    pub emission_baseline: f32,
    /// DCB field: `runtimeDuration` (Single)
    #[serde(default)]
    pub runtime_duration: f32,
    /// DCB field: `allowedInAIAutoScan` (Boolean)
    #[serde(default)]
    pub allowed_in_aiauto_scan: bool,
    /// DCB field: `allowedInFocalPointScan` (Boolean)
    #[serde(default)]
    pub allowed_in_focal_point_scan: bool,
    /// DCB field: `allowedInPingBroadScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_broad_scan: bool,
    /// DCB field: `allowedInPingFocusScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_focus_scan: bool,
    /// DCB field: `allowedInPassiveScan` (Boolean)
    #[serde(default)]
    pub allowed_in_passive_scan: bool,
}

impl Pooled for ScanProcedureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scan_procedure_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scan_procedure_params }
}

impl<'a> Extract<'a> for ScanProcedureParams {
    const TYPE_NAME: &'static str = "ScanProcedureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_scan_tag: inst.get("requiredScanTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            emission_baseline: inst.get_f32("emissionBaseline").unwrap_or_default(),
            runtime_duration: inst.get_f32("runtimeDuration").unwrap_or_default(),
            allowed_in_aiauto_scan: inst.get_bool("allowedInAIAutoScan").unwrap_or_default(),
            allowed_in_focal_point_scan: inst.get_bool("allowedInFocalPointScan").unwrap_or_default(),
            allowed_in_ping_broad_scan: inst.get_bool("allowedInPingBroadScan").unwrap_or_default(),
            allowed_in_ping_focus_scan: inst.get_bool("allowedInPingFocusScan").unwrap_or_default(),
            allowed_in_passive_scan: inst.get_bool("allowedInPassiveScan").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemLocalization`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemLocalization {
    /// DCB field: `Name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `ShortName` (Locale)
    #[serde(default)]
    pub short_name: String,
    /// DCB field: `Description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `displayFeatures` (Class)
    #[serde(default)]
    pub display_features: Option<Handle<SCExtendedLocalizationLevelParams>>,
}

impl Pooled for SCItemLocalization {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_localization }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_localization }
}

impl<'a> Extract<'a> for SCItemLocalization {
    const TYPE_NAME: &'static str = "SCItemLocalization";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            short_name: inst.get_str("ShortName").map(String::from).unwrap_or_default(),
            description: inst.get_str("Description").map(String::from).unwrap_or_default(),
            display_features: match inst.get("displayFeatures") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCExtendedLocalizationLevelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemLightAmplification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemLightAmplification {
    /// DCB field: `Tint` (Class)
    #[serde(default)]
    pub tint: Option<Handle<SRGB8>>,
    /// DCB field: `Saturation` (Single)
    #[serde(default)]
    pub saturation: f32,
    /// DCB field: `EVAmplification` (Single)
    #[serde(default)]
    pub evamplification: f32,
    /// DCB field: `AmplificationOnAudioTrigger` (Class)
    #[serde(default)]
    pub amplification_on_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `AmplificationOffAudioTrigger` (Class)
    #[serde(default)]
    pub amplification_off_audio_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for SCItemLightAmplification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_light_amplification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_light_amplification }
}

impl<'a> Extract<'a> for SCItemLightAmplification {
    const TYPE_NAME: &'static str = "SCItemLightAmplification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tint: match inst.get("Tint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            saturation: inst.get_f32("Saturation").unwrap_or_default(),
            evamplification: inst.get_f32("EVAmplification").unwrap_or_default(),
            amplification_on_audio_trigger: match inst.get("AmplificationOnAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            amplification_off_audio_trigger: match inst.get("AmplificationOffAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemManufacturer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemManufacturer {
    /// DCB field: `Localization` (Class)
    #[serde(default)]
    pub localization: Option<Handle<SCItemLocalization>>,
    /// DCB field: `Logo` (String)
    #[serde(default)]
    pub logo: String,
    /// DCB field: `LogoFullColor` (String)
    #[serde(default)]
    pub logo_full_color: String,
    /// DCB field: `LogoSimplifiedWhite` (String)
    #[serde(default)]
    pub logo_simplified_white: String,
    /// DCB field: `Code` (String)
    #[serde(default)]
    pub code: String,
    /// DCB field: `DashboardCanvasConfig` (Reference)
    #[serde(default)]
    pub dashboard_canvas_config: Option<CigGuid>,
    /// DCB field: `BuildingBlocksStyle` (Reference)
    #[serde(default)]
    pub building_blocks_style: Option<CigGuid>,
    /// DCB field: `AudioManufacturerTag` (Reference)
    #[serde(default)]
    pub audio_manufacturer_tag: Option<CigGuid>,
    /// DCB field: `LightAmplification` (Reference)
    #[serde(default)]
    pub light_amplification: Option<CigGuid>,
}

impl Pooled for SCItemManufacturer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_manufacturer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_manufacturer }
}

impl<'a> Extract<'a> for SCItemManufacturer {
    const TYPE_NAME: &'static str = "SCItemManufacturer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            localization: match inst.get("Localization") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemLocalization>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemLocalization>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            logo: inst.get_str("Logo").map(String::from).unwrap_or_default(),
            logo_full_color: inst.get_str("LogoFullColor").map(String::from).unwrap_or_default(),
            logo_simplified_white: inst.get_str("LogoSimplifiedWhite").map(String::from).unwrap_or_default(),
            code: inst.get_str("Code").map(String::from).unwrap_or_default(),
            dashboard_canvas_config: inst.get("DashboardCanvasConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            building_blocks_style: inst.get("BuildingBlocksStyle").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_manufacturer_tag: inst.get("AudioManufacturerTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            light_amplification: inst.get("LightAmplification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCItemVisorDashboardConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemVisorDashboardConfig {
    /// DCB field: `screens` (Class (array))
    #[serde(default)]
    pub screens: Vec<Handle<SCItemSeatDashboardScreen>>,
}

impl Pooled for SCItemVisorDashboardConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_visor_dashboard_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_visor_dashboard_config }
}

impl<'a> Extract<'a> for SCItemVisorDashboardConfig {
    const TYPE_NAME: &'static str = "SCItemVisorDashboardConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            screens: inst.get_array("screens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCItemSeatDashboardScreen>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCItemSeatDashboardScreen>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemSuitAtmosphereFuelConversionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSuitAtmosphereFuelConversionParams {
    /// DCB field: `Gas` (Reference)
    #[serde(default)]
    pub gas: Option<CigGuid>,
    /// DCB field: `massConversionRatio` (Single)
    #[serde(default)]
    pub mass_conversion_ratio: f32,
}

impl Pooled for SCItemSuitAtmosphereFuelConversionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_suit_atmosphere_fuel_conversion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_suit_atmosphere_fuel_conversion_params }
}

impl<'a> Extract<'a> for SCItemSuitAtmosphereFuelConversionParams {
    const TYPE_NAME: &'static str = "SCItemSuitAtmosphereFuelConversionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gas: inst.get("Gas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mass_conversion_ratio: inst.get_f32("massConversionRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemSuitFuelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSuitFuelParams {
    /// DCB field: `fuelResourcePrimary` (Reference)
    #[serde(default)]
    pub fuel_resource_primary: Option<CigGuid>,
    /// DCB field: `fuelResourceSecondary` (Reference)
    #[serde(default)]
    pub fuel_resource_secondary: Option<CigGuid>,
    /// DCB field: `primaryFuelBurnRateMicroSCU` (StrongPointer)
    #[serde(default)]
    pub primary_fuel_burn_rate_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `secondaryFuelBurnRateGrams` (Single)
    #[serde(default)]
    pub secondary_fuel_burn_rate_grams: f32,
    /// DCB field: `fuelRefillRateFromTankMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_tank_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `fuelRefillRateFromAtmosphereMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_atmosphere_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `fuelRefillRateFromInjectionMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_injection_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `gasConversionRatios` (Class (array))
    #[serde(default)]
    pub gas_conversion_ratios: Vec<Handle<SCItemSuitAtmosphereFuelConversionParams>>,
    /// DCB field: `usingSecondaryFuelMessage` (Locale)
    #[serde(default)]
    pub using_secondary_fuel_message: String,
}

impl Pooled for SCItemSuitFuelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_suit_fuel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_suit_fuel_params }
}

impl<'a> Extract<'a> for SCItemSuitFuelParams {
    const TYPE_NAME: &'static str = "SCItemSuitFuelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fuel_resource_primary: inst.get("fuelResourcePrimary").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fuel_resource_secondary: inst.get("fuelResourceSecondary").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            primary_fuel_burn_rate_micro_scu: match inst.get("primaryFuelBurnRateMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            secondary_fuel_burn_rate_grams: inst.get_f32("secondaryFuelBurnRateGrams").unwrap_or_default(),
            fuel_refill_rate_from_tank_micro_scu: match inst.get("fuelRefillRateFromTankMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fuel_refill_rate_from_atmosphere_micro_scu: match inst.get("fuelRefillRateFromAtmosphereMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fuel_refill_rate_from_injection_micro_scu: match inst.get("fuelRefillRateFromInjectionMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gas_conversion_ratios: inst.get_array("gasConversionRatios")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCItemSuitAtmosphereFuelConversionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCItemSuitAtmosphereFuelConversionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            using_secondary_fuel_message: inst.get_str("usingSecondaryFuelMessage").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemCommsComponentSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemCommsComponentSetup {
    /// DCB field: `showInContactsList` (Boolean)
    #[serde(default)]
    pub show_in_contacts_list: bool,
    /// DCB field: `broadcastRange` (Single)
    #[serde(default)]
    pub broadcast_range: f32,
    /// DCB field: `startIncomingCallSound` (Class)
    #[serde(default)]
    pub start_incoming_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopIncomingCallSound` (Class)
    #[serde(default)]
    pub stop_incoming_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `startOutgoingCallSound` (Class)
    #[serde(default)]
    pub start_outgoing_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopOutgoingCallSound` (Class)
    #[serde(default)]
    pub stop_outgoing_call_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `introTrigger` (Class)
    #[serde(default)]
    pub intro_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `outroTrigger` (Class)
    #[serde(default)]
    pub outro_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `joinTrigger` (Class)
    #[serde(default)]
    pub join_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `leaveTrigger` (Class)
    #[serde(default)]
    pub leave_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `renderParticles` (Boolean)
    #[serde(default)]
    pub render_particles: bool,
}

impl Pooled for SCItemCommsComponentSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_comms_component_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_comms_component_setup }
}

impl<'a> Extract<'a> for SCItemCommsComponentSetup {
    const TYPE_NAME: &'static str = "SCItemCommsComponentSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            show_in_contacts_list: inst.get_bool("showInContactsList").unwrap_or_default(),
            broadcast_range: inst.get_f32("broadcastRange").unwrap_or_default(),
            start_incoming_call_sound: match inst.get("startIncomingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_incoming_call_sound: match inst.get("stopIncomingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_outgoing_call_sound: match inst.get("startOutgoingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_outgoing_call_sound: match inst.get("stopOutgoingCallSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            intro_trigger: match inst.get("introTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outro_trigger: match inst.get("outroTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            join_trigger: match inst.get("joinTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            leave_trigger: match inst.get("leaveTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_particles: inst.get_bool("renderParticles").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemDisplayScreenPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemDisplayScreenPreset {
    /// DCB field: `material` (String)
    #[serde(default)]
    pub material: String,
    /// DCB field: `geometryPath` (String)
    #[serde(default)]
    pub geometry_path: String,
}

impl Pooled for SCItemDisplayScreenPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_display_screen_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_display_screen_preset }
}

impl<'a> Extract<'a> for SCItemDisplayScreenPreset {
    const TYPE_NAME: &'static str = "SCItemDisplayScreenPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            material: inst.get_str("material").map(String::from).unwrap_or_default(),
            geometry_path: inst.get_str("geometryPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCSeatHeadPosAdjustSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCSeatHeadPosAdjustSetup {
    /// DCB field: `fpDesiredHeadPosition` (Class)
    #[serde(default)]
    pub fp_desired_head_position: Option<Handle<Vec3>>,
}

impl Pooled for SCSeatHeadPosAdjustSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scseat_head_pos_adjust_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scseat_head_pos_adjust_setup }
}

impl<'a> Extract<'a> for SCSeatHeadPosAdjustSetup {
    const TYPE_NAME: &'static str = "SCSeatHeadPosAdjustSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fp_desired_head_position: match inst.get("fpDesiredHeadPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemSeatHeadTrackingPositionLimitParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSeatHeadTrackingPositionLimitParams {
    /// DCB field: `leftRight` (Single)
    #[serde(default)]
    pub left_right: f32,
    /// DCB field: `up` (Single)
    #[serde(default)]
    pub up: f32,
    /// DCB field: `down` (Single)
    #[serde(default)]
    pub down: f32,
    /// DCB field: `forward` (Single)
    #[serde(default)]
    pub forward: f32,
    /// DCB field: `backward` (Single)
    #[serde(default)]
    pub backward: f32,
}

impl Pooled for SCItemSeatHeadTrackingPositionLimitParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_seat_head_tracking_position_limit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_seat_head_tracking_position_limit_params }
}

impl<'a> Extract<'a> for SCItemSeatHeadTrackingPositionLimitParams {
    const TYPE_NAME: &'static str = "SCItemSeatHeadTrackingPositionLimitParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_right: inst.get_f32("leftRight").unwrap_or_default(),
            up: inst.get_f32("up").unwrap_or_default(),
            down: inst.get_f32("down").unwrap_or_default(),
            forward: inst.get_f32("forward").unwrap_or_default(),
            backward: inst.get_f32("backward").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemSeatDashboardScreenPos`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSeatDashboardScreenPos {
    /// DCB field: `Helper` (String)
    #[serde(default)]
    pub helper: String,
    /// DCB field: `Scale` (Single)
    #[serde(default)]
    pub scale: f32,
    /// DCB field: `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
}

impl Pooled for SCItemSeatDashboardScreenPos {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_seat_dashboard_screen_pos }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_seat_dashboard_screen_pos }
}

impl<'a> Extract<'a> for SCItemSeatDashboardScreenPos {
    const TYPE_NAME: &'static str = "SCItemSeatDashboardScreenPos";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            helper: inst.get_str("Helper").map(String::from).unwrap_or_default(),
            scale: inst.get_f32("Scale").unwrap_or_default(),
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemSeatDashboardScreenStyle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSeatDashboardScreenStyle {
    /// DCB field: `Template` (String)
    #[serde(default)]
    pub template: String,
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SCItemSeatDashboardScreenStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_seat_dashboard_screen_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_seat_dashboard_screen_style }
}

impl<'a> Extract<'a> for SCItemSeatDashboardScreenStyle {
    const TYPE_NAME: &'static str = "SCItemSeatDashboardScreenStyle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            template: inst.get_str("Template").map(String::from).unwrap_or_default(),
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemSeatDashboardScreen`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSeatDashboardScreen {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `GeomName` (String)
    #[serde(default)]
    pub geom_name: String,
    /// DCB field: `MaterialOverride` (String)
    #[serde(default)]
    pub material_override: String,
    /// DCB field: `Style` (Class)
    #[serde(default)]
    pub style: Option<Handle<SCItemSeatDashboardScreenStyle>>,
    /// DCB field: `Position` (Class)
    #[serde(default)]
    pub position: Option<Handle<SCItemSeatDashboardScreenPos>>,
}

impl Pooled for SCItemSeatDashboardScreen {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_seat_dashboard_screen }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_seat_dashboard_screen }
}

impl<'a> Extract<'a> for SCItemSeatDashboardScreen {
    const TYPE_NAME: &'static str = "SCItemSeatDashboardScreen";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            geom_name: inst.get_str("GeomName").map(String::from).unwrap_or_default(),
            material_override: inst.get_str("MaterialOverride").map(String::from).unwrap_or_default(),
            style: match inst.get("Style") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemSeatDashboardScreenStyle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemSeatDashboardScreenStyle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position: match inst.get("Position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemSeatDashboardScreenPos>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemSeatDashboardScreenPos>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCSignatureSystemRoomParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCSignatureSystemRoomParams {
    /// DCB field: `ignoreInteriorVsExteriorCheck` (Boolean)
    #[serde(default)]
    pub ignore_interior_vs_exterior_check: bool,
}

impl Pooled for SCSignatureSystemRoomParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scsignature_system_room_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scsignature_system_room_params }
}

impl<'a> Extract<'a> for SCSignatureSystemRoomParams {
    const TYPE_NAME: &'static str = "SCSignatureSystemRoomParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ignore_interior_vs_exterior_check: inst.get_bool("ignoreInteriorVsExteriorCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCSignatureDeathParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCSignatureDeathParams {
    /// DCB field: `deathEmissionModifierParams` (StrongPointer)
    #[serde(default)]
    pub death_emission_modifier_params: Option<Handle<SSCSignatureEmissionBaseModifier>>,
}

impl Pooled for SCSignatureDeathParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scsignature_death_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scsignature_death_params }
}

impl<'a> Extract<'a> for SCSignatureDeathParams {
    const TYPE_NAME: &'static str = "SCSignatureDeathParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            death_emission_modifier_params: match inst.get("deathEmissionModifierParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureEmissionBaseModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureEmissionBaseModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScenarioProgress`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioProgress {
    /// DCB field: `multiplierRewards` (Class (array))
    #[serde(default)]
    pub multiplier_rewards: Vec<Handle<SResourceRewardMultiplier>>,
    /// DCB field: `factionRewardTiers` (Class (array))
    #[serde(default)]
    pub faction_reward_tiers: Vec<Handle<SScenarioProgressRewardsTiers>>,
}

impl Pooled for ScenarioProgress {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scenario_progress }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scenario_progress }
}

impl<'a> Extract<'a> for ScenarioProgress {
    const TYPE_NAME: &'static str = "ScenarioProgress";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            multiplier_rewards: inst.get_array("multiplierRewards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SResourceRewardMultiplier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SResourceRewardMultiplier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            faction_reward_tiers: inst.get_array("factionRewardTiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScenarioProgressRewardsTiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScenarioProgressRewardsTiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Library`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_Library {
    /// DCB field: `effectList` (Reference (array))
    #[serde(default)]
    pub effect_list: Vec<CigGuid>,
}

impl Pooled for ScreenEffects_Library {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_library }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_library }
}

impl<'a> Extract<'a> for ScreenEffects_Library {
    const TYPE_NAME: &'static str = "ScreenEffects_Library";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_list: inst.get_array("effectList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Effect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_Effect {
    /// DCB field: `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// DCB field: `disableInThirdPerson` (Boolean)
    #[serde(default)]
    pub disable_in_third_person: bool,
    /// DCB field: `sharedPattern` (StrongPointer)
    #[serde(default)]
    pub shared_pattern: Option<Handle<ScreenEffects_Pattern>>,
    /// DCB field: `parameters` (Class (array))
    #[serde(default)]
    pub parameters: Vec<Handle<ScreenEffects_Param>>,
}

impl Pooled for ScreenEffects_Effect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_effect }
}

impl<'a> Extract<'a> for ScreenEffects_Effect {
    const TYPE_NAME: &'static str = "ScreenEffects_Effect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            disable_in_third_person: inst.get_bool("disableInThirdPerson").unwrap_or_default(),
            shared_pattern: match inst.get("sharedPattern") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScreenEffects_Pattern>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScreenEffects_Pattern>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_Param>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScreenEffects_Param>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Pattern`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_Pattern {
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
}

impl Pooled for ScreenEffects_Pattern {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_pattern }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_pattern }
}

impl<'a> Extract<'a> for ScreenEffects_Pattern {
    const TYPE_NAME: &'static str = "ScreenEffects_Pattern";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_Param`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_Param {
    /// DCB field: `parameter` (EnumChoice)
    #[serde(default)]
    pub parameter: String,
    /// DCB field: `value` (StrongPointer)
    #[serde(default)]
    pub value: Option<Handle<ScreenEffects_ParamValue>>,
    /// DCB field: `strengthBehavior` (StrongPointer)
    #[serde(default)]
    pub strength_behavior: Option<Handle<ScreenEffects_ParamStrengthBehavior>>,
}

impl Pooled for ScreenEffects_Param {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_param }
}

impl<'a> Extract<'a> for ScreenEffects_Param {
    const TYPE_NAME: &'static str = "ScreenEffects_Param";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            parameter: inst.get_str("parameter").map(String::from).unwrap_or_default(),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScreenEffects_ParamValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScreenEffects_ParamValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            strength_behavior: match inst.get("strengthBehavior") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScreenEffects_ParamStrengthBehavior>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScreenEffects_ParamStrengthBehavior>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ScreenEffects_ParamValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_ParamValue {
}

impl Pooled for ScreenEffects_ParamValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_param_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_param_value }
}

impl<'a> Extract<'a> for ScreenEffects_ParamValue {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ScreenEffects_ParamStrengthBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_ParamStrengthBehavior {
    /// DCB field: `strengthTag` (Reference)
    #[serde(default)]
    pub strength_tag: Option<CigGuid>,
}

impl Pooled for ScreenEffects_ParamStrengthBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_param_strength_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_param_strength_behavior }
}

impl<'a> Extract<'a> for ScreenEffects_ParamStrengthBehavior {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamStrengthBehavior";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ScreenEffects_Debug`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_Debug {
    /// DCB field: `effectList` (Class (array))
    #[serde(default)]
    pub effect_list: Vec<Handle<ScreenEffects_DebugEffect>>,
}

impl Pooled for ScreenEffects_Debug {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_debug }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_debug }
}

impl<'a> Extract<'a> for ScreenEffects_Debug {
    const TYPE_NAME: &'static str = "ScreenEffects_Debug";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_list: inst.get_array("effectList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_DebugEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScreenEffects_DebugEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_DebugEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_DebugEffect {
    /// DCB field: `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `parameters` (Class (array))
    #[serde(default)]
    pub parameters: Vec<Handle<ScreenEffects_DebugParam>>,
}

impl Pooled for ScreenEffects_DebugEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_debug_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_debug_effect }
}

impl<'a> Extract<'a> for ScreenEffects_DebugEffect {
    const TYPE_NAME: &'static str = "ScreenEffects_DebugEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            enable: inst.get_bool("enable").unwrap_or_default(),
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScreenEffects_DebugParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScreenEffects_DebugParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScreenEffects_DebugParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_DebugParam {
    /// DCB field: `strengthTag` (Reference)
    #[serde(default)]
    pub strength_tag: Option<CigGuid>,
    /// DCB field: `strength` (Single)
    #[serde(default)]
    pub strength: f32,
}

impl Pooled for ScreenEffects_DebugParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.screen_effects_debug_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.screen_effects_debug_param }
}

impl<'a> Extract<'a> for ScreenEffects_DebugParam {
    const TYPE_NAME: &'static str = "ScreenEffects_DebugParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strength_tag: inst.get("strengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            strength: inst.get_f32("strength").unwrap_or_default(),
        }
    }
}

/// DCB type: `SConversationIconParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SConversationIconParams {
    /// DCB field: `useConversationIcon` (Boolean)
    #[serde(default)]
    pub use_conversation_icon: bool,
    /// DCB field: `entitySuperGUID` (String)
    #[serde(default)]
    pub entity_super_guid: String,
    /// DCB field: `iconVisibleGUID` (String)
    #[serde(default)]
    pub icon_visible_guid: String,
    /// DCB field: `distanceToSwitchToText` (Single)
    #[serde(default)]
    pub distance_to_switch_to_text: f32,
    /// DCB field: `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// DCB field: `rotationOffset` (Class)
    #[serde(default)]
    pub rotation_offset: Option<Handle<Vec3>>,
    /// DCB field: `iconScale` (Single)
    #[serde(default)]
    pub icon_scale: f32,
    /// DCB field: `textScale` (Single)
    #[serde(default)]
    pub text_scale: f32,
    /// DCB field: `alwaysFacePlayer` (Boolean)
    #[serde(default)]
    pub always_face_player: bool,
    /// DCB field: `maintainPosition` (Boolean)
    #[serde(default)]
    pub maintain_position: bool,
    /// DCB field: `usePlayerAsReferenceEntity` (Boolean)
    #[serde(default)]
    pub use_player_as_reference_entity: bool,
}

impl Pooled for SConversationIconParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.sconversation_icon_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.sconversation_icon_params }
}

impl<'a> Extract<'a> for SConversationIconParams {
    const TYPE_NAME: &'static str = "SConversationIconParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_conversation_icon: inst.get_bool("useConversationIcon").unwrap_or_default(),
            entity_super_guid: inst.get_str("entitySuperGUID").map(String::from).unwrap_or_default(),
            icon_visible_guid: inst.get_str("iconVisibleGUID").map(String::from).unwrap_or_default(),
            distance_to_switch_to_text: inst.get_f32("distanceToSwitchToText").unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_offset: match inst.get("rotationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            icon_scale: inst.get_f32("iconScale").unwrap_or_default(),
            text_scale: inst.get_f32("textScale").unwrap_or_default(),
            always_face_player: inst.get_bool("alwaysFacePlayer").unwrap_or_default(),
            maintain_position: inst.get_bool("maintainPosition").unwrap_or_default(),
            use_player_as_reference_entity: inst.get_bool("usePlayerAsReferenceEntity").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCObjectDataBankEntryMarkerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCObjectDataBankEntryMarkerConfig {
    /// DCB field: `managedLandingZoneMinimumDist` (Single)
    #[serde(default)]
    pub managed_landing_zone_minimum_dist: f32,
    /// DCB field: `managedLandingZoneMaximumDist` (Single)
    #[serde(default)]
    pub managed_landing_zone_maximum_dist: f32,
    /// DCB field: `unmanagedLandingZoneMinimumDist` (Single)
    #[serde(default)]
    pub unmanaged_landing_zone_minimum_dist: f32,
    /// DCB field: `unmanagedLandingZoneMaximumDist` (Single)
    #[serde(default)]
    pub unmanaged_landing_zone_maximum_dist: f32,
}

impl Pooled for SCObjectDataBankEntryMarkerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scobject_data_bank_entry_marker_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scobject_data_bank_entry_marker_config }
}

impl<'a> Extract<'a> for SCObjectDataBankEntryMarkerConfig {
    const TYPE_NAME: &'static str = "SCObjectDataBankEntryMarkerConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            managed_landing_zone_minimum_dist: inst.get_f32("managedLandingZoneMinimumDist").unwrap_or_default(),
            managed_landing_zone_maximum_dist: inst.get_f32("managedLandingZoneMaximumDist").unwrap_or_default(),
            unmanaged_landing_zone_minimum_dist: inst.get_f32("unmanagedLandingZoneMinimumDist").unwrap_or_default(),
            unmanaged_landing_zone_maximum_dist: inst.get_f32("unmanagedLandingZoneMaximumDist").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemUIView_DashboardCanvasViewDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemUIView_DashboardCanvasViewDef {
    /// DCB field: `screens` (Reference)
    #[serde(default)]
    pub screens: Option<CigGuid>,
}

impl Pooled for SCItemUIView_DashboardCanvasViewDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_uiview_dashboard_canvas_view_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_uiview_dashboard_canvas_view_def }
}

impl<'a> Extract<'a> for SCItemUIView_DashboardCanvasViewDef {
    const TYPE_NAME: &'static str = "SCItemUIView_DashboardCanvasViewDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            screens: inst.get("screens").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCItemUIView_DashboardCanvasDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemUIView_DashboardCanvasDef {
    /// DCB field: `views` (Class)
    #[serde(default)]
    pub views: Option<Handle<SCItemUIView_DashboardCanvasViewDef>>,
}

impl Pooled for SCItemUIView_DashboardCanvasDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scitem_uiview_dashboard_canvas_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scitem_uiview_dashboard_canvas_def }
}

impl<'a> Extract<'a> for SCItemUIView_DashboardCanvasDef {
    const TYPE_NAME: &'static str = "SCItemUIView_DashboardCanvasDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            views: match inst.get("views") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemUIView_DashboardCanvasViewDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemUIView_DashboardCanvasViewDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCurve {
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sc.scurve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sc.scurve }
}

impl<'a> Extract<'a> for SCurve {
    const TYPE_NAME: &'static str = "SCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

