// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-charactercustomizer`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EntityComponentCharacterCustomizerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentCharacterCustomizerParams {
    /// `cameraRigRoot` (Class)
    #[serde(default)]
    pub camera_rig_root: Option<Handle<EntityReferenceDef>>,
    /// `playerHighlightingParams` (Class)
    #[serde(default)]
    pub player_highlighting_params: Option<Handle<SFaceHighlightingParams>>,
    /// `currentEditFaceHighlightingParams` (Class)
    #[serde(default)]
    pub current_edit_face_highlighting_params: Option<Handle<SFaceHighlightingParams>>,
    /// `headLibraryTagPoint` (Class)
    #[serde(default)]
    pub head_library_tag_point: Option<Handle<EntityReferenceDef>>,
    /// `cameraParent` (Class)
    #[serde(default)]
    pub camera_parent: Option<Handle<EntityReferenceDef>>,
    /// `zoomNavSpline` (Class)
    #[serde(default)]
    pub zoom_nav_spline: Option<Handle<EntityReferenceDef>>,
    /// `libraryDeselectDelayTime` (Single)
    #[serde(default)]
    pub library_deselect_delay_time: f32,
    /// `libraryHeadCount` (Int32)
    #[serde(default)]
    pub library_head_count: i32,
    /// `customizerDNARegions` (Class)
    #[serde(default)]
    pub customizer_dnaregions: Option<Handle<SCharacterCustomizerDNARegionParams>>,
    /// `bodyTypes` (Class (array))
    #[serde(default)]
    pub body_types: Vec<Handle<SCharacterCustomizerBodyTypeParams>>,
    /// `blankSkinVariant` (Reference)
    #[serde(default)]
    pub blank_skin_variant: Option<CigGuid>,
    /// `controlParams` (Class)
    #[serde(default)]
    pub control_params: Option<Handle<SCharacterCustomizerControlParams>>,
    /// `materialEditingParams` (Class)
    #[serde(default)]
    pub material_editing_params: Option<Handle<SCharacterCustomizerMaterialEditParams>>,
    /// `headLibraryRollOnTriggers` (Boolean)
    #[serde(default)]
    pub head_library_roll_on_triggers: bool,
    /// `dnaBlendingPools` (Class (array))
    #[serde(default)]
    pub dna_blending_pools: Vec<Handle<SCharacterCustomizerDNAHeadPool>>,
    /// `useBlendingPoolsForFaceSculpting` (Boolean)
    #[serde(default)]
    pub use_blending_pools_for_face_sculpting: bool,
    /// `faceSculptingPools` (Class (array))
    #[serde(default)]
    pub face_sculpting_pools: Vec<Handle<SCharacterCustomizerDNAHeadPool>>,
    /// `featureLibraryParams` (Class)
    #[serde(default)]
    pub feature_library_params: Option<Handle<SCharacterCustomizerHeadLibraryParams>>,
    /// `dnaHeadLibraryParams` (Class)
    #[serde(default)]
    pub dna_head_library_params: Option<Handle<SCharacterCustomizerHeadLibraryParams>>,
    /// `dnaClippingVolumeParams` (Class)
    #[serde(default)]
    pub dna_clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `voiceOptions` (Class (array))
    #[serde(default)]
    pub voice_options: Vec<Handle<SCharacterCustomizerVoiceParams>>,
    /// `allowedClasses` (Reference)
    #[serde(default)]
    pub allowed_classes: Option<CigGuid>,
    /// `featureItemTags` (String (array))
    #[serde(default)]
    pub feature_item_tags: Vec<String>,
    /// `serializeToNetwork` (Boolean)
    #[serde(default)]
    pub serialize_to_network: bool,
    /// `serializationTimeout` (Single)
    #[serde(default)]
    pub serialization_timeout: f32,
    /// `loadoutReplacements` (Class (array))
    #[serde(default)]
    pub loadout_replacements: Vec<Handle<SCharacterCustomizerLoadoutItemReplacementMapping>>,
    /// `updatePlayerEntity` (Boolean)
    #[serde(default)]
    pub update_player_entity: bool,
    /// `clearAllPlayerLoadout` (Boolean)
    #[serde(default)]
    pub clear_all_player_loadout: bool,
    /// `cameraEntity` (Class)
    #[serde(default)]
    pub camera_entity: Option<Handle<EntityReferenceDef>>,
    /// `dialogueData` (Reference)
    #[serde(default)]
    pub dialogue_data: Option<CigGuid>,
    /// `flowSteps` (Class (array))
    #[serde(default)]
    pub flow_steps: Vec<Handle<SCharacterCustomizerStep>>,
    /// `customizableMaterials` (StrongPointer (array))
    #[serde(default)]
    pub customizable_materials: Vec<Handle<SCustomizableMaterialParams>>,
    /// `hidePlayer` (Boolean)
    #[serde(default)]
    pub hide_player: bool,
    /// `shutdownTime` (Single)
    #[serde(default)]
    pub shutdown_time: f32,
    /// `requireNameValues` (Boolean)
    #[serde(default)]
    pub require_name_values: bool,
}

impl Pooled for EntityComponentCharacterCustomizerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.entity_component_character_customizer_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.entity_component_character_customizer_params }
}

impl<'a> Extract<'a> for EntityComponentCharacterCustomizerParams {
    const TYPE_NAME: &'static str = "EntityComponentCharacterCustomizerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_rig_root: match inst.get("cameraRigRoot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_highlighting_params: match inst.get("playerHighlightingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFaceHighlightingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            current_edit_face_highlighting_params: match inst.get("currentEditFaceHighlightingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SFaceHighlightingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            head_library_tag_point: match inst.get("headLibraryTagPoint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            camera_parent: match inst.get("cameraParent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            zoom_nav_spline: match inst.get("zoomNavSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            library_deselect_delay_time: inst.get_f32("libraryDeselectDelayTime").unwrap_or_default(),
            library_head_count: inst.get_i32("libraryHeadCount").unwrap_or_default(),
            customizer_dnaregions: match inst.get("customizerDNARegions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerDNARegionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            body_types: inst.get_array("bodyTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerBodyTypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerBodyTypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            blank_skin_variant: inst.get("blankSkinVariant").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            control_params: match inst.get("controlParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerControlParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material_editing_params: match inst.get("materialEditingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerMaterialEditParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            head_library_roll_on_triggers: inst.get_bool("headLibraryRollOnTriggers").unwrap_or_default(),
            dna_blending_pools: inst.get_array("dnaBlendingPools")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadPool>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadPool>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            use_blending_pools_for_face_sculpting: inst.get_bool("useBlendingPoolsForFaceSculpting").unwrap_or_default(),
            face_sculpting_pools: inst.get_array("faceSculptingPools")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadPool>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadPool>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            feature_library_params: match inst.get("featureLibraryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerHeadLibraryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dna_head_library_params: match inst.get("dnaHeadLibraryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerHeadLibraryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dna_clipping_volume_params: match inst.get("dnaClippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            voice_options: inst.get_array("voiceOptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerVoiceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerVoiceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            allowed_classes: inst.get("allowedClasses").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            feature_item_tags: inst.get_array("featureItemTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            serialize_to_network: inst.get_bool("serializeToNetwork").unwrap_or_default(),
            serialization_timeout: inst.get_f32("serializationTimeout").unwrap_or_default(),
            loadout_replacements: inst.get_array("loadoutReplacements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerLoadoutItemReplacementMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerLoadoutItemReplacementMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            update_player_entity: inst.get_bool("updatePlayerEntity").unwrap_or_default(),
            clear_all_player_loadout: inst.get_bool("clearAllPlayerLoadout").unwrap_or_default(),
            camera_entity: match inst.get("cameraEntity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            dialogue_data: inst.get("dialogueData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            flow_steps: inst.get_array("flowSteps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerStep>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerStep>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            customizable_materials: inst.get_array("customizableMaterials")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hide_player: inst.get_bool("hidePlayer").unwrap_or_default(),
            shutdown_time: inst.get_f32("shutdownTime").unwrap_or_default(),
            require_name_values: inst.get_bool("requireNameValues").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerLoadoutItemReplacementMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerLoadoutItemReplacementMapping {
    /// `networkOnly` (Boolean)
    #[serde(default)]
    pub network_only: bool,
    /// `oldItemGUID` (String)
    #[serde(default)]
    pub old_item_guid: String,
    /// `newItem` (Reference)
    #[serde(default)]
    pub new_item: Option<CigGuid>,
}

impl Pooled for SCharacterCustomizerLoadoutItemReplacementMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_loadout_item_replacement_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_loadout_item_replacement_mapping }
}

impl<'a> Extract<'a> for SCharacterCustomizerLoadoutItemReplacementMapping {
    const TYPE_NAME: &'static str = "SCharacterCustomizerLoadoutItemReplacementMapping";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            network_only: inst.get_bool("networkOnly").unwrap_or_default(),
            old_item_guid: inst.get_str("oldItemGUID").map(String::from).unwrap_or_default(),
            new_item: inst.get("newItem").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCharacterCustomizerHeadLibraryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerHeadLibraryParams {
    /// `libraryRoot` (Class)
    #[serde(default)]
    pub library_root: Option<Handle<EntityReferenceDef>>,
    /// `columns` (Int32)
    #[serde(default)]
    pub columns: i32,
    /// `columnSpacing` (Single)
    #[serde(default)]
    pub column_spacing: f32,
    /// `rowSpacing` (Single)
    #[serde(default)]
    pub row_spacing: f32,
    /// `libraryScrollingIncrement` (Single)
    #[serde(default)]
    pub library_scrolling_increment: f32,
    /// `rowsOnScreen` (Int32)
    #[serde(default)]
    pub rows_on_screen: i32,
    /// `sourceHeadClass` (Reference)
    #[serde(default)]
    pub source_head_class: Option<CigGuid>,
    /// `additionalEntities` (Class (array))
    #[serde(default)]
    pub additional_entities: Vec<Handle<EntityReferenceDef>>,
    /// `defaultProtosHeadClass` (Reference)
    #[serde(default)]
    pub default_protos_head_class: Option<CigGuid>,
}

impl Pooled for SCharacterCustomizerHeadLibraryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_head_library_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_head_library_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerHeadLibraryParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerHeadLibraryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            library_root: match inst.get("libraryRoot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            columns: inst.get_i32("columns").unwrap_or_default(),
            column_spacing: inst.get_f32("columnSpacing").unwrap_or_default(),
            row_spacing: inst.get_f32("rowSpacing").unwrap_or_default(),
            library_scrolling_increment: inst.get_f32("libraryScrollingIncrement").unwrap_or_default(),
            rows_on_screen: inst.get_i32("rowsOnScreen").unwrap_or_default(),
            source_head_class: inst.get("sourceHeadClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            additional_entities: inst.get_array("additionalEntities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityReferenceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_protos_head_class: inst.get("defaultProtosHeadClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCharacterCustomizerBlemishMapParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerBlemishMapParams {
    /// `blemishMask` (String)
    #[serde(default)]
    pub blemish_mask: String,
    /// `blemishIdMask` (String)
    #[serde(default)]
    pub blemish_id_mask: String,
}

impl Pooled for SCharacterCustomizerBlemishMapParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_blemish_map_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_blemish_map_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerBlemishMapParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerBlemishMapParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blemish_mask: inst.get_str("blemishMask").map(String::from).unwrap_or_default(),
            blemish_id_mask: inst.get_str("blemishIdMask").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCustomzierColorSRGBA8`
/// Inherits from: `SCustomzierColorDefBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomzierColorSRGBA8 {
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGBA8>>,
}

impl Pooled for SCustomzierColorSRGBA8 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scustomzier_color_srgba8 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scustomzier_color_srgba8 }
}

impl<'a> Extract<'a> for SCustomzierColorSRGBA8 {
    const TYPE_NAME: &'static str = "SCustomzierColorSRGBA8";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerDefaultShaderParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerDefaultShaderParam {
    /// `param` (WeakPointer)
    #[serde(default)]
    pub param: Option<SCharacterCustomizerShaderParamBasePtr>,
    /// `colorValue` (StrongPointer)
    #[serde(default)]
    pub color_value: Option<SCustomzierColorDefBasePtr>,
    /// `floatValue` (Single)
    #[serde(default)]
    pub float_value: f32,
}

impl Pooled for SCharacterCustomizerDefaultShaderParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_default_shader_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_default_shader_param }
}

impl<'a> Extract<'a> for SCharacterCustomizerDefaultShaderParam {
    const TYPE_NAME: &'static str = "SCharacterCustomizerDefaultShaderParam";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            param: match inst.get("param") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerShaderParamBasePtr::from_ref(b, r)),
                _ => None,
            },
            color_value: match inst.get("colorValue") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCustomzierColorDefBasePtr::from_ref(b, r)),
                _ => None,
            },
            float_value: inst.get_f32("floatValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerClampedValueParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerClampedValueParams {
    /// `clampValue` (Boolean)
    #[serde(default)]
    pub clamp_value: bool,
    /// `minValue` (Single)
    #[serde(default)]
    pub min_value: f32,
    /// `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
}

impl Pooled for SCharacterCustomizerClampedValueParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_clamped_value_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_clamped_value_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerClampedValueParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerClampedValueParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            clamp_value: inst.get_bool("clampValue").unwrap_or_default(),
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerTextureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerTextureParams {
    /// `texturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// `numTilesU` (Int32)
    #[serde(default)]
    pub num_tiles_u: i32,
    /// `numTilesV` (Int32)
    #[serde(default)]
    pub num_tiles_v: i32,
    /// `offsetU` (Int32)
    #[serde(default)]
    pub offset_u: i32,
    /// `offsetV` (Int32)
    #[serde(default)]
    pub offset_v: i32,
    /// `supportsHueRotation` (Boolean)
    #[serde(default)]
    pub supports_hue_rotation: bool,
    /// `mirrorMode` (EnumChoice)
    #[serde(default)]
    pub mirror_mode: ETattooMirrorMode,
}

impl Pooled for SCharacterCustomizerTextureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_texture_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_texture_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerTextureParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerTextureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst.get_str("texturePath").map(String::from).unwrap_or_default(),
            num_tiles_u: inst.get_i32("numTilesU").unwrap_or_default(),
            num_tiles_v: inst.get_i32("numTilesV").unwrap_or_default(),
            offset_u: inst.get_i32("offsetU").unwrap_or_default(),
            offset_v: inst.get_i32("offsetV").unwrap_or_default(),
            supports_hue_rotation: inst.get_bool("supportsHueRotation").unwrap_or_default(),
            mirror_mode: ETattooMirrorMode::from_dcb_str(inst.get_str("mirrorMode").unwrap_or("")),
        }
    }
}

/// DCB type: `SCharacterCustomizerTextureList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerTextureList {
    /// `textures` (StrongPointer (array))
    #[serde(default)]
    pub textures: Vec<SCharacterCustomizerTextureParamsPtr>,
    /// `devOnlyTextures` (StrongPointer (array))
    #[serde(default)]
    pub dev_only_textures: Vec<SCharacterCustomizerTextureParamsPtr>,
}

impl Pooled for SCharacterCustomizerTextureList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_texture_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_texture_list }
}

impl<'a> Extract<'a> for SCharacterCustomizerTextureList {
    const TYPE_NAME: &'static str = "SCharacterCustomizerTextureList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            textures: inst.get_array("textures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerTextureParamsPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dev_only_textures: inst.get_array("devOnlyTextures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerTextureParamsPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerMakeupParams`
/// Inherits from: `SCharacterCustomizerTextureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerMakeupParams {
    /// `texturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// `numTilesU` (Int32)
    #[serde(default)]
    pub num_tiles_u: i32,
    /// `numTilesV` (Int32)
    #[serde(default)]
    pub num_tiles_v: i32,
    /// `offsetU` (Int32)
    #[serde(default)]
    pub offset_u: i32,
    /// `offsetV` (Int32)
    #[serde(default)]
    pub offset_v: i32,
    /// `supportsHueRotation` (Boolean)
    #[serde(default)]
    pub supports_hue_rotation: bool,
    /// `mirrorMode` (EnumChoice)
    #[serde(default)]
    pub mirror_mode: ETattooMirrorMode,
    /// `defaultShaderParams` (Class (array))
    #[serde(default)]
    pub default_shader_params: Vec<Handle<SCharacterCustomizerDefaultShaderParam>>,
}

impl Pooled for SCharacterCustomizerMakeupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_makeup_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_makeup_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerMakeupParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerMakeupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst.get_str("texturePath").map(String::from).unwrap_or_default(),
            num_tiles_u: inst.get_i32("numTilesU").unwrap_or_default(),
            num_tiles_v: inst.get_i32("numTilesV").unwrap_or_default(),
            offset_u: inst.get_i32("offsetU").unwrap_or_default(),
            offset_v: inst.get_i32("offsetV").unwrap_or_default(),
            supports_hue_rotation: inst.get_bool("supportsHueRotation").unwrap_or_default(),
            mirror_mode: ETattooMirrorMode::from_dcb_str(inst.get_str("mirrorMode").unwrap_or("")),
            default_shader_params: inst.get_array("defaultShaderParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerDefaultShaderParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerDefaultShaderParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerExtraTextureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerExtraTextureParams {
    /// `texturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// `textureSlot` (EnumChoice)
    #[serde(default)]
    pub texture_slot: ECharacterCustomizerTextureSlot,
}

impl Pooled for SCharacterCustomizerExtraTextureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_extra_texture_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_extra_texture_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerExtraTextureParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerExtraTextureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst.get_str("texturePath").map(String::from).unwrap_or_default(),
            texture_slot: ECharacterCustomizerTextureSlot::from_dcb_str(inst.get_str("textureSlot").unwrap_or("")),
        }
    }
}

/// DCB type: `SCharacterCustomizerMultiTextureParams`
/// Inherits from: `SCharacterCustomizerTextureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerMultiTextureParams {
    /// `texturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// `numTilesU` (Int32)
    #[serde(default)]
    pub num_tiles_u: i32,
    /// `numTilesV` (Int32)
    #[serde(default)]
    pub num_tiles_v: i32,
    /// `offsetU` (Int32)
    #[serde(default)]
    pub offset_u: i32,
    /// `offsetV` (Int32)
    #[serde(default)]
    pub offset_v: i32,
    /// `supportsHueRotation` (Boolean)
    #[serde(default)]
    pub supports_hue_rotation: bool,
    /// `mirrorMode` (EnumChoice)
    #[serde(default)]
    pub mirror_mode: ETattooMirrorMode,
    /// `additionalTextures` (Class (array))
    #[serde(default)]
    pub additional_textures: Vec<Handle<SCharacterCustomizerExtraTextureParams>>,
}

impl Pooled for SCharacterCustomizerMultiTextureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_multi_texture_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_multi_texture_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerMultiTextureParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerMultiTextureParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst.get_str("texturePath").map(String::from).unwrap_or_default(),
            num_tiles_u: inst.get_i32("numTilesU").unwrap_or_default(),
            num_tiles_v: inst.get_i32("numTilesV").unwrap_or_default(),
            offset_u: inst.get_i32("offsetU").unwrap_or_default(),
            offset_v: inst.get_i32("offsetV").unwrap_or_default(),
            supports_hue_rotation: inst.get_bool("supportsHueRotation").unwrap_or_default(),
            mirror_mode: ETattooMirrorMode::from_dcb_str(inst.get_str("mirrorMode").unwrap_or("")),
            additional_textures: inst.get_array("additionalTextures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerExtraTextureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerExtraTextureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerBaseMaterialSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerBaseMaterialSet {
    /// `headSkinBaseMaterial` (String)
    #[serde(default)]
    pub head_skin_base_material: String,
    /// `bodySkinBaseMaterial` (String)
    #[serde(default)]
    pub body_skin_base_material: String,
}

impl Pooled for SCharacterCustomizerBaseMaterialSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_base_material_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_base_material_set }
}

impl<'a> Extract<'a> for SCharacterCustomizerBaseMaterialSet {
    const TYPE_NAME: &'static str = "SCharacterCustomizerBaseMaterialSet";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            head_skin_base_material: inst.get_str("headSkinBaseMaterial").map(String::from).unwrap_or_default(),
            body_skin_base_material: inst.get_str("bodySkinBaseMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerSkinBaseMaterialList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerSkinBaseMaterialList {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `modelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<SGeometryModelTagBasePtr>,
    /// `skinBaseMaterials` (Class (array))
    #[serde(default)]
    pub skin_base_materials: Vec<Handle<SCharacterCustomizerBaseMaterialSet>>,
    /// `devOnly` (Boolean)
    #[serde(default)]
    pub dev_only: bool,
}

impl Pooled for SCharacterCustomizerSkinBaseMaterialList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_skin_base_material_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_skin_base_material_list }
}

impl<'a> Extract<'a> for SCharacterCustomizerSkinBaseMaterialList {
    const TYPE_NAME: &'static str = "SCharacterCustomizerSkinBaseMaterialList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            model_tag: match inst.get("modelTag") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SGeometryModelTagBasePtr::from_ref(b, r)),
                _ => None,
            },
            skin_base_materials: inst.get_array("skinBaseMaterials")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerBaseMaterialSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerBaseMaterialSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dev_only: inst.get_bool("devOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerMaterialEditParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerMaterialEditParams {
    /// `blemishMaps` (Class (array))
    #[serde(default)]
    pub blemish_maps: Vec<Handle<SCharacterCustomizerBlemishMapParams>>,
    /// `baseSkinMaterials` (Class (array))
    #[serde(default)]
    pub base_skin_materials: Vec<Handle<SCharacterCustomizerSkinBaseMaterialList>>,
    /// `hairDyeShiftMinFadeOut` (Single)
    #[serde(default)]
    pub hair_dye_shift_min_fade_out: f32,
    /// `hairLengths` (Class (array))
    #[serde(default)]
    pub hair_lengths: Vec<Handle<SCharacterCustomizerHairLengthEntry>>,
}

impl Pooled for SCharacterCustomizerMaterialEditParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_material_edit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_material_edit_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerMaterialEditParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerMaterialEditParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blemish_maps: inst.get_array("blemishMaps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerBlemishMapParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerBlemishMapParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            base_skin_materials: inst.get_array("baseSkinMaterials")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerSkinBaseMaterialList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerSkinBaseMaterialList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hair_dye_shift_min_fade_out: inst.get_f32("hairDyeShiftMinFadeOut").unwrap_or_default(),
            hair_lengths: inst.get_array("hairLengths")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerHairLengthEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerHairLengthEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SFaceHighlightingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFaceHighlightingParams {
    /// `wireFrameMaskPow` (Single)
    #[serde(default)]
    pub wire_frame_mask_pow: f32,
    /// `pulseSpeed` (Single)
    #[serde(default)]
    pub pulse_speed: f32,
    /// `highlightSymmetrical` (Boolean)
    #[serde(default)]
    pub highlight_symmetrical: bool,
    /// `wireFrameColour` (Class)
    #[serde(default)]
    pub wire_frame_colour: Option<Handle<SRGBA8>>,
}

impl Pooled for SFaceHighlightingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.sface_highlighting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.sface_highlighting_params }
}

impl<'a> Extract<'a> for SFaceHighlightingParams {
    const TYPE_NAME: &'static str = "SFaceHighlightingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wire_frame_mask_pow: inst.get_f32("wireFrameMaskPow").unwrap_or_default(),
            pulse_speed: inst.get_f32("pulseSpeed").unwrap_or_default(),
            highlight_symmetrical: inst.get_bool("highlightSymmetrical").unwrap_or_default(),
            wire_frame_colour: match inst.get("wireFrameColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerVertexParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerVertexParams {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `vertexId` (Int32)
    #[serde(default)]
    pub vertex_id: i32,
    /// `ignoreList` (WeakPointer (array))
    #[serde(default)]
    pub ignore_list: Vec<Handle<SCharacterCustomizerVertexParams>>,
    /// `shapingTolerance` (Single)
    #[serde(default)]
    pub shaping_tolerance: f32,
}

impl Pooled for SCharacterCustomizerVertexParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_vertex_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_vertex_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerVertexParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerVertexParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            vertex_id: inst.get_i32("vertexId").unwrap_or_default(),
            ignore_list: inst.get_array("ignoreList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCharacterCustomizerVertexParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            shaping_tolerance: inst.get_f32("shapingTolerance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerDNARegionParams`
/// Inherits from: `SCharacterCustomizerRegionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerDNARegionParams {
    /// `displayName` (String)
    #[serde(default)]
    pub display_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `visible` (Boolean)
    #[serde(default)]
    pub visible: bool,
    /// `symmetryRegion` (EnumChoice)
    #[serde(default)]
    pub symmetry_region: ECharacterCustomizerDNARegion,
    /// `shapingVertices` (Class (array))
    #[serde(default)]
    pub shaping_vertices: Vec<Handle<SCharacterCustomizerVertexParams>>,
    /// `shapingUIWeight` (Single)
    #[serde(default)]
    pub shaping_uiweight: f32,
}

impl Pooled for SCharacterCustomizerDNARegionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_dnaregion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_dnaregion_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerDNARegionParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerDNARegionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            visible: inst.get_bool("visible").unwrap_or_default(),
            symmetry_region: ECharacterCustomizerDNARegion::from_dcb_str(inst.get_str("symmetryRegion").unwrap_or("")),
            shaping_vertices: inst.get_array("shapingVertices")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerVertexParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerVertexParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            shaping_uiweight: inst.get_f32("shapingUIWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerBodyTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerBodyTypeParams {
    /// `displayName` (String)
    #[serde(default)]
    pub display_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `bodyTypeDummyClass` (Reference)
    #[serde(default)]
    pub body_type_dummy_class: Option<CigGuid>,
    /// `bodyTypeTagPoint` (Class)
    #[serde(default)]
    pub body_type_tag_point: Option<Handle<EntityReferenceDef>>,
    /// `overrideModelTag` (StrongPointer)
    #[serde(default)]
    pub override_model_tag: Option<SGeometryModelTagBasePtr>,
    /// `bodyTypeOffset` (Class)
    #[serde(default)]
    pub body_type_offset: Option<Handle<Vec3>>,
    /// `randomizationOverrideParams` (Class (array))
    #[serde(default)]
    pub randomization_override_params: Vec<Handle<SCharacterCustomizerRandomizationOverrideParams>>,
    /// `cameraRigOffset` (Class)
    #[serde(default)]
    pub camera_rig_offset: Option<Handle<Vec3>>,
}

impl Pooled for SCharacterCustomizerBodyTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_body_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_body_type_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerBodyTypeParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerBodyTypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            body_type_dummy_class: inst.get("bodyTypeDummyClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            body_type_tag_point: match inst.get("bodyTypeTagPoint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            override_model_tag: match inst.get("overrideModelTag") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SGeometryModelTagBasePtr::from_ref(b, r)),
                _ => None,
            },
            body_type_offset: match inst.get("bodyTypeOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            randomization_override_params: inst.get_array("randomizationOverrideParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerRandomizationOverrideParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerRandomizationOverrideParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            camera_rig_offset: match inst.get("cameraRigOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerRandomizationOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerRandomizationOverrideParams {
    /// `feature` (WeakPointer)
    #[serde(default)]
    pub feature: Option<SCharacterCustomizerFeatureBasePtr>,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
}

impl Pooled for SCharacterCustomizerRandomizationOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_randomization_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_randomization_override_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerRandomizationOverrideParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerRandomizationOverrideParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            feature: match inst.get("feature") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                _ => None,
            },
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerControlParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerControlParams {
    /// `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// `maxRotationSpeed` (Single)
    #[serde(default)]
    pub max_rotation_speed: f32,
    /// `tiltClamp` (Class)
    #[serde(default)]
    pub tilt_clamp: Option<Handle<Vec2>>,
    /// `enableTilt` (Boolean)
    #[serde(default)]
    pub enable_tilt: bool,
    /// `dnaDragRate` (Single)
    #[serde(default)]
    pub dna_drag_rate: f32,
    /// `dnaKeyboardEditRate` (Single)
    #[serde(default)]
    pub dna_keyboard_edit_rate: f32,
    /// `rotationLerpTime` (Single)
    #[serde(default)]
    pub rotation_lerp_time: f32,
    /// `libraryHeadBBoxScale` (Single)
    #[serde(default)]
    pub library_head_bbox_scale: f32,
    /// `wholeHeadRegionBBoxScale` (Single)
    #[serde(default)]
    pub whole_head_region_bbox_scale: f32,
    /// `libraryScrollSpeed` (Single)
    #[serde(default)]
    pub library_scroll_speed: f32,
    /// `libraryScrollTimeout` (Single)
    #[serde(default)]
    pub library_scroll_timeout: f32,
    /// `dnaKeyControlSpeed` (Single)
    #[serde(default)]
    pub dna_key_control_speed: f32,
    /// `bodyTypeSelectionLerpTime` (Single)
    #[serde(default)]
    pub body_type_selection_lerp_time: f32,
    /// `faceSculptingControlPointUIScaleFactor` (Single)
    #[serde(default)]
    pub face_sculpting_control_point_uiscale_factor: f32,
    /// `enforceSymmetry` (Boolean)
    #[serde(default)]
    pub enforce_symmetry: bool,
    /// `useAllDNAVertices` (Boolean)
    #[serde(default)]
    pub use_all_dnavertices: bool,
    /// `facesculptingUIRadius` (Single)
    #[serde(default)]
    pub facesculpting_uiradius: f32,
    /// `stepChangeDelayTime` (Single)
    #[serde(default)]
    pub step_change_delay_time: f32,
    /// `faceSculptingOcclusionOffset` (Single)
    #[serde(default)]
    pub face_sculpting_occlusion_offset: f32,
    /// `rotateCamera` (Boolean)
    #[serde(default)]
    pub rotate_camera: bool,
    /// `stepTransitionTime` (Single)
    #[serde(default)]
    pub step_transition_time: f32,
}

impl Pooled for SCharacterCustomizerControlParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_control_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_control_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerControlParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerControlParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            rotation_rate: inst.get_f32("rotationRate").unwrap_or_default(),
            max_rotation_speed: inst.get_f32("maxRotationSpeed").unwrap_or_default(),
            tilt_clamp: match inst.get("tiltClamp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enable_tilt: inst.get_bool("enableTilt").unwrap_or_default(),
            dna_drag_rate: inst.get_f32("dnaDragRate").unwrap_or_default(),
            dna_keyboard_edit_rate: inst.get_f32("dnaKeyboardEditRate").unwrap_or_default(),
            rotation_lerp_time: inst.get_f32("rotationLerpTime").unwrap_or_default(),
            library_head_bbox_scale: inst.get_f32("libraryHeadBBoxScale").unwrap_or_default(),
            whole_head_region_bbox_scale: inst.get_f32("wholeHeadRegionBBoxScale").unwrap_or_default(),
            library_scroll_speed: inst.get_f32("libraryScrollSpeed").unwrap_or_default(),
            library_scroll_timeout: inst.get_f32("libraryScrollTimeout").unwrap_or_default(),
            dna_key_control_speed: inst.get_f32("dnaKeyControlSpeed").unwrap_or_default(),
            body_type_selection_lerp_time: inst.get_f32("bodyTypeSelectionLerpTime").unwrap_or_default(),
            face_sculpting_control_point_uiscale_factor: inst.get_f32("faceSculptingControlPointUIScaleFactor").unwrap_or_default(),
            enforce_symmetry: inst.get_bool("enforceSymmetry").unwrap_or_default(),
            use_all_dnavertices: inst.get_bool("useAllDNAVertices").unwrap_or_default(),
            facesculpting_uiradius: inst.get_f32("facesculptingUIRadius").unwrap_or_default(),
            step_change_delay_time: inst.get_f32("stepChangeDelayTime").unwrap_or_default(),
            face_sculpting_occlusion_offset: inst.get_f32("faceSculptingOcclusionOffset").unwrap_or_default(),
            rotate_camera: inst.get_bool("rotateCamera").unwrap_or_default(),
            step_transition_time: inst.get_f32("stepTransitionTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerDNAHeadPool`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerDNAHeadPool {
    /// `tag` (StrongPointer)
    #[serde(default)]
    pub tag: Option<SGeometryModelTagBasePtr>,
    /// `defaultMaterialPaths` (String (array))
    #[serde(default)]
    pub default_material_paths: Vec<String>,
    /// `exclusionPool` (Boolean)
    #[serde(default)]
    pub exclusion_pool: bool,
    /// `heads` (Class (array))
    #[serde(default)]
    pub heads: Vec<Handle<SCharacterCustomizerDNAHeadParams>>,
}

impl Pooled for SCharacterCustomizerDNAHeadPool {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_dnahead_pool }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_dnahead_pool }
}

impl<'a> Extract<'a> for SCharacterCustomizerDNAHeadPool {
    const TYPE_NAME: &'static str = "SCharacterCustomizerDNAHeadPool";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: match inst.get("tag") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SGeometryModelTagBasePtr::from_ref(b, r)),
                _ => None,
            },
            default_material_paths: inst.get_array("defaultMaterialPaths")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            exclusion_pool: inst.get_bool("exclusionPool").unwrap_or_default(),
            heads: inst.get_array("heads")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerDNAHeadParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCustomizerDefaultItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomizerDefaultItem {
    /// `itemportName` (String)
    #[serde(default)]
    pub itemport_name: String,
    /// `class` (Reference)
    #[serde(default)]
    pub class: Option<CigGuid>,
    /// `children` (Class (array))
    #[serde(default)]
    pub children: Vec<Handle<SCustomizerDefaultItem>>,
}

impl Pooled for SCustomizerDefaultItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scustomizer_default_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scustomizer_default_item }
}

impl<'a> Extract<'a> for SCustomizerDefaultItem {
    const TYPE_NAME: &'static str = "SCustomizerDefaultItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            itemport_name: inst.get_str("itemportName").map(String::from).unwrap_or_default(),
            class: inst.get("class").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizerDefaultItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCustomizerDefaultItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerDNAHeadParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerDNAHeadParams {
    /// `headId` (Int32)
    #[serde(default)]
    pub head_id: i32,
    /// `customMaterial` (String)
    #[serde(default)]
    pub custom_material: String,
    /// `defaultLoadoutItems` (Class (array))
    #[serde(default)]
    pub default_loadout_items: Vec<Handle<SCustomizerDefaultItem>>,
    /// `irisColor` (Class)
    #[serde(default)]
    pub iris_color: Option<Handle<RGB>>,
}

impl Pooled for SCharacterCustomizerDNAHeadParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_dnahead_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_dnahead_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerDNAHeadParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerDNAHeadParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            head_id: inst.get_i32("headId").unwrap_or_default(),
            custom_material: inst.get_str("customMaterial").map(String::from).unwrap_or_default(),
            default_loadout_items: inst.get_array("defaultLoadoutItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCustomizerDefaultItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCustomizerDefaultItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            iris_color: match inst.get("irisColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerClippingVolumeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerClippingVolumeParams {
    /// `area` (Class)
    #[serde(default)]
    pub area: Option<Handle<EntityReferenceDef>>,
    /// `transitionTime` (Single)
    #[serde(default)]
    pub transition_time: f32,
    /// `fadeDistance` (Single)
    #[serde(default)]
    pub fade_distance: f32,
}

impl Pooled for SCharacterCustomizerClippingVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_clipping_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_clipping_volume_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerClippingVolumeParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerClippingVolumeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            area: match inst.get("area") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            transition_time: inst.get_f32("transitionTime").unwrap_or_default(),
            fade_distance: inst.get_f32("fadeDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerVoiceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerVoiceParams {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `modelVoiceTagPairs` (Class (array))
    #[serde(default)]
    pub model_voice_tag_pairs: Vec<Handle<SModelVoiceTagPair>>,
}

impl Pooled for SCharacterCustomizerVoiceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_voice_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_voice_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerVoiceParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerVoiceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            model_voice_tag_pairs: inst.get_array("modelVoiceTagPairs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SModelVoiceTagPair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SModelVoiceTagPair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerComplexionRandomizationParams`
/// Inherits from: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerComplexionRandomizationParams {
    /// `frecklesOpacityLimits` (Class)
    #[serde(default)]
    pub freckles_opacity_limits: Option<Handle<Vec2>>,
    /// `frecklesAmountLimits` (Class)
    #[serde(default)]
    pub freckles_amount_limits: Option<Handle<Vec2>>,
    /// `sunspotsOpacityLimits` (Class)
    #[serde(default)]
    pub sunspots_opacity_limits: Option<Handle<Vec2>>,
    /// `sunspotsAmountLimits` (Class)
    #[serde(default)]
    pub sunspots_amount_limits: Option<Handle<Vec2>>,
}

impl Pooled for SCharacterCustomizerComplexionRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_complexion_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_complexion_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerComplexionRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerComplexionRandomizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            freckles_opacity_limits: match inst.get("frecklesOpacityLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            freckles_amount_limits: match inst.get("frecklesAmountLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sunspots_opacity_limits: match inst.get("sunspotsOpacityLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sunspots_amount_limits: match inst.get("sunspotsAmountLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerItemRandomizationParams`
/// Inherits from: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerItemRandomizationParams {
    /// `noItemProbability` (Single)
    #[serde(default)]
    pub no_item_probability: f32,
}

impl Pooled for SCharacterCustomizerItemRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_item_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_item_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerItemRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerItemRandomizationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            no_item_probability: inst.get_f32("noItemProbability").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerHairRandomizationParams`
/// Inherits from: `SCharacterCustomizerItemRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerHairRandomizationParams {
    /// `noItemProbability` (Single)
    #[serde(default)]
    pub no_item_probability: f32,
    /// `melaninLimits` (Class)
    #[serde(default)]
    pub melanin_limits: Option<Handle<Vec2>>,
    /// `rednessLimits` (Class)
    #[serde(default)]
    pub redness_limits: Option<Handle<Vec2>>,
}

impl Pooled for SCharacterCustomizerHairRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_hair_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_hair_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerHairRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerHairRandomizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            no_item_probability: inst.get_f32("noItemProbability").unwrap_or_default(),
            melanin_limits: match inst.get("melaninLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            redness_limits: match inst.get("rednessLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerMakeupSlotRandomizationParams`
/// Inherits from: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerMakeupSlotRandomizationParams {
    /// `noMakeupProbability` (Single)
    #[serde(default)]
    pub no_makeup_probability: f32,
    /// `makeupSlot` (WeakPointer)
    #[serde(default)]
    pub makeup_slot: Option<Handle<SCharacterCustomizerMakeupSlot>>,
}

impl Pooled for SCharacterCustomizerMakeupSlotRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_makeup_slot_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_makeup_slot_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerMakeupSlotRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerMakeupSlotRandomizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            no_makeup_probability: inst.get_f32("noMakeupProbability").unwrap_or_default(),
            makeup_slot: match inst.get("makeupSlot") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCharacterCustomizerMakeupSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerTattooRandomizationParams`
/// Inherits from: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerTattooRandomizationParams {
    /// `noTattooProbability` (Single)
    #[serde(default)]
    pub no_tattoo_probability: f32,
    /// `tattooAgeLimits` (Class)
    #[serde(default)]
    pub tattoo_age_limits: Option<Handle<Vec2>>,
    /// `tattooFeatureParams` (WeakPointer)
    #[serde(default)]
    pub tattoo_feature_params: Option<SCharacterCustomizerFeatureTextureSelectPtr>,
}

impl Pooled for SCharacterCustomizerTattooRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_tattoo_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_tattoo_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerTattooRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerTattooRandomizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            no_tattoo_probability: inst.get_f32("noTattooProbability").unwrap_or_default(),
            tattoo_age_limits: match inst.get("tattooAgeLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            tattoo_feature_params: match inst.get("tattooFeatureParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerFeatureTextureSelectPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerRandomColorDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerRandomColorDef {
    /// `probability` (Single)
    #[serde(default)]
    pub probability: f32,
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGB>>,
}

impl Pooled for SCharacterCustomizerRandomColorDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_random_color_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_random_color_def }
}

impl<'a> Extract<'a> for SCharacterCustomizerRandomColorDef {
    const TYPE_NAME: &'static str = "SCharacterCustomizerRandomColorDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability: inst.get_f32("probability").unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerEyeRandomizationParams`
/// Inherits from: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerEyeRandomizationParams {
    /// `irisColors` (Class (array))
    #[serde(default)]
    pub iris_colors: Vec<Handle<SCharacterCustomizerRandomColorDef>>,
}

impl Pooled for SCharacterCustomizerEyeRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_eye_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_eye_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerEyeRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerEyeRandomizationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            iris_colors: inst.get_array("irisColors")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerRandomColorDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerRandomColorDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerHairLengthOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerHairLengthOverride {
    /// `modelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<SGeometryModelTagBasePtr>,
    /// `hairLength` (Single)
    #[serde(default)]
    pub hair_length: f32,
}

impl Pooled for SCharacterCustomizerHairLengthOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_hair_length_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_hair_length_override }
}

impl<'a> Extract<'a> for SCharacterCustomizerHairLengthOverride {
    const TYPE_NAME: &'static str = "SCharacterCustomizerHairLengthOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            model_tag: match inst.get("modelTag") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SGeometryModelTagBasePtr::from_ref(b, r)),
                _ => None,
            },
            hair_length: inst.get_f32("hairLength").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerHairLengthEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerHairLengthEntry {
    /// `hairClass` (Reference)
    #[serde(default)]
    pub hair_class: Option<CigGuid>,
    /// `hairLength` (Single)
    #[serde(default)]
    pub hair_length: f32,
    /// `hairLengthOverrides` (Class (array))
    #[serde(default)]
    pub hair_length_overrides: Vec<Handle<SCharacterCustomizerHairLengthOverride>>,
}

impl Pooled for SCharacterCustomizerHairLengthEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_hair_length_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_hair_length_entry }
}

impl<'a> Extract<'a> for SCharacterCustomizerHairLengthEntry {
    const TYPE_NAME: &'static str = "SCharacterCustomizerHairLengthEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hair_class: inst.get("hairClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            hair_length: inst.get_f32("hairLength").unwrap_or_default(),
            hair_length_overrides: inst.get_array("hairLengthOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerHairLengthOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerHairLengthOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterSkinValidationParams`
/// Inherits from: `SCharacterValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterSkinValidationParams {
    /// `minHue` (Single)
    #[serde(default)]
    pub min_hue: f32,
    /// `maxHue` (Single)
    #[serde(default)]
    pub max_hue: f32,
}

impl Pooled for SCharacterSkinValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_skin_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_skin_validation_params }
}

impl<'a> Extract<'a> for SCharacterSkinValidationParams {
    const TYPE_NAME: &'static str = "SCharacterSkinValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_hue: inst.get_f32("minHue").unwrap_or_default(),
            max_hue: inst.get_f32("maxHue").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCustomizableMaterialParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomizableMaterialParams {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `attachmentName` (String)
    #[serde(default)]
    pub attachment_name: String,
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: EItemType,
    /// `submaterialsToEdit` (Int32 (array))
    #[serde(default)]
    pub submaterials_to_edit: Vec<i32>,
    /// `linkedMaterials` (WeakPointer (array))
    #[serde(default)]
    pub linked_materials: Vec<Handle<SCustomizableMaterialParams>>,
    /// `additionalFlags` (String)
    #[serde(default)]
    pub additional_flags: String,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
}

impl Pooled for SCustomizableMaterialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scustomizable_material_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scustomizable_material_params }
}

impl<'a> Extract<'a> for SCustomizableMaterialParams {
    const TYPE_NAME: &'static str = "SCustomizableMaterialParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            attachment_name: inst.get_str("attachmentName").map(String::from).unwrap_or_default(),
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            submaterials_to_edit: inst.get_array("submaterialsToEdit")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            linked_materials: inst.get_array("linkedMaterials")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            additional_flags: inst.get_str("additionalFlags").map(String::from).unwrap_or_default(),
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerStep`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerStep {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `features` (StrongPointer (array))
    #[serde(default)]
    pub features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `isVisible` (Boolean)
    #[serde(default)]
    pub is_visible: bool,
    /// `uiVisible` (Boolean)
    #[serde(default)]
    pub ui_visible: bool,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `playerDummyVisible` (Boolean)
    #[serde(default)]
    pub player_dummy_visible: bool,
    /// `allowFlowgraphTransition` (Boolean)
    #[serde(default)]
    pub allow_flowgraph_transition: bool,
}

impl Pooled for SCharacterCustomizerStep {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_step }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_step }
}

impl<'a> Extract<'a> for SCharacterCustomizerStep {
    const TYPE_NAME: &'static str = "SCharacterCustomizerStep";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            features: inst.get_array("features")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_visible: inst.get_bool("isVisible").unwrap_or_default(),
            ui_visible: inst.get_bool("uiVisible").unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            player_dummy_visible: inst.get_bool("playerDummyVisible").unwrap_or_default(),
            allow_flowgraph_transition: inst.get_bool("allowFlowgraphTransition").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureBase {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
}

impl Pooled for SCharacterCustomizerFeatureBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_base }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureBase {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureDNA`
/// Inherits from: `SCharacterCustomizerFeatureBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureDNA {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
    /// `editType` (EnumChoice)
    #[serde(default)]
    pub edit_type: EDNAEditType,
}

impl Pooled for SCharacterCustomizerFeatureDNA {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_dna }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_dna }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureDNA {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureDNA";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            edit_type: EDNAEditType::from_dcb_str(inst.get_str("editType").unwrap_or("")),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureItemEquip`
/// Inherits from: `SCharacterCustomizerItemSelect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureItemEquip {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
    /// `clippingVolumeParams` (Class)
    #[serde(default)]
    pub clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `advancedModeClippingVolumeParams` (Class)
    #[serde(default)]
    pub advanced_mode_clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `addEmptyEntry` (Boolean)
    #[serde(default)]
    pub add_empty_entry: bool,
    /// `featureLibOffset` (Class)
    #[serde(default)]
    pub feature_lib_offset: Option<Handle<Vec3>>,
    /// `advancedModeLibOffset` (Class)
    #[serde(default)]
    pub advanced_mode_lib_offset: Option<Handle<Vec3>>,
    /// `featureLibRowsOnScreen` (Int32)
    #[serde(default)]
    pub feature_lib_rows_on_screen: i32,
    /// `advancedModeRowsOnScreen` (Int32)
    #[serde(default)]
    pub advanced_mode_rows_on_screen: i32,
    /// `itemSelectMode` (EnumChoice)
    #[serde(default)]
    pub item_select_mode: ECharacterCustomizerItemSelectMode,
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: EItemType,
    /// `requiredPortTag` (String)
    #[serde(default)]
    pub required_port_tag: String,
    /// `requiredTags` (String)
    #[serde(default)]
    pub required_tags: String,
    /// `showOnParentFeatureHeadLibrary` (Boolean)
    #[serde(default)]
    pub show_on_parent_feature_head_library: bool,
    /// `linkedItemPortNames` (String (array))
    #[serde(default)]
    pub linked_item_port_names: Vec<String>,
}

impl Pooled for SCharacterCustomizerFeatureItemEquip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_item_equip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_item_equip }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureItemEquip {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureItemEquip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            clipping_volume_params: match inst.get("clippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_clipping_volume_params: match inst.get("advancedModeClippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            add_empty_entry: inst.get_bool("addEmptyEntry").unwrap_or_default(),
            feature_lib_offset: match inst.get("featureLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_lib_offset: match inst.get("advancedModeLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            feature_lib_rows_on_screen: inst.get_i32("featureLibRowsOnScreen").unwrap_or_default(),
            advanced_mode_rows_on_screen: inst.get_i32("advancedModeRowsOnScreen").unwrap_or_default(),
            item_select_mode: ECharacterCustomizerItemSelectMode::from_dcb_str(inst.get_str("itemSelectMode").unwrap_or("")),
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            required_port_tag: inst.get_str("requiredPortTag").map(String::from).unwrap_or_default(),
            required_tags: inst.get_str("requiredTags").map(String::from).unwrap_or_default(),
            show_on_parent_feature_head_library: inst.get_bool("showOnParentFeatureHeadLibrary").unwrap_or_default(),
            linked_item_port_names: inst.get_array("linkedItemPortNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureTextureSelect`
/// Inherits from: `SCharacterCustomizerItemSelect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureTextureSelect {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
    /// `clippingVolumeParams` (Class)
    #[serde(default)]
    pub clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `advancedModeClippingVolumeParams` (Class)
    #[serde(default)]
    pub advanced_mode_clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `addEmptyEntry` (Boolean)
    #[serde(default)]
    pub add_empty_entry: bool,
    /// `featureLibOffset` (Class)
    #[serde(default)]
    pub feature_lib_offset: Option<Handle<Vec3>>,
    /// `advancedModeLibOffset` (Class)
    #[serde(default)]
    pub advanced_mode_lib_offset: Option<Handle<Vec3>>,
    /// `featureLibRowsOnScreen` (Int32)
    #[serde(default)]
    pub feature_lib_rows_on_screen: i32,
    /// `advancedModeRowsOnScreen` (Int32)
    #[serde(default)]
    pub advanced_mode_rows_on_screen: i32,
    /// `itemSelectMode` (EnumChoice)
    #[serde(default)]
    pub item_select_mode: ECharacterCustomizerItemSelectMode,
    /// `slot` (EnumChoice)
    #[serde(default)]
    pub slot: ECharacterCustomizerTextureSelectSlot,
    /// `textureSlot` (EnumChoice)
    #[serde(default)]
    pub texture_slot: ECharacterCustomizerTextureSlot,
    /// `subMaterialsToEdit` (Int32 (array))
    #[serde(default)]
    pub sub_materials_to_edit: Vec<i32>,
    /// `textures` (Class)
    #[serde(default)]
    pub textures: Option<Handle<SCharacterCustomizerTextureList>>,
}

impl Pooled for SCharacterCustomizerFeatureTextureSelect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_texture_select }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_texture_select }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureTextureSelect {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureTextureSelect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            clipping_volume_params: match inst.get("clippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_clipping_volume_params: match inst.get("advancedModeClippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            add_empty_entry: inst.get_bool("addEmptyEntry").unwrap_or_default(),
            feature_lib_offset: match inst.get("featureLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_lib_offset: match inst.get("advancedModeLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            feature_lib_rows_on_screen: inst.get_i32("featureLibRowsOnScreen").unwrap_or_default(),
            advanced_mode_rows_on_screen: inst.get_i32("advancedModeRowsOnScreen").unwrap_or_default(),
            item_select_mode: ECharacterCustomizerItemSelectMode::from_dcb_str(inst.get_str("itemSelectMode").unwrap_or("")),
            slot: ECharacterCustomizerTextureSelectSlot::from_dcb_str(inst.get_str("slot").unwrap_or("")),
            texture_slot: ECharacterCustomizerTextureSlot::from_dcb_str(inst.get_str("textureSlot").unwrap_or("")),
            sub_materials_to_edit: inst.get_array("subMaterialsToEdit")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            textures: match inst.get("textures") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerTextureList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCharacterCustomizerShaderParamBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerShaderParamBase {
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `applyToCurrentFeatureMaterial` (Boolean)
    #[serde(default)]
    pub apply_to_current_feature_material: bool,
    /// `shaderParamName` (String)
    #[serde(default)]
    pub shader_param_name: String,
    /// `bindingsURL` (String)
    #[serde(default)]
    pub bindings_url: String,
    /// `setFromCurrentBindings` (Boolean)
    #[serde(default)]
    pub set_from_current_bindings: bool,
    /// `requiresAdvancedMode` (Boolean)
    #[serde(default)]
    pub requires_advanced_mode: bool,
    /// `canBeEdited` (Boolean)
    #[serde(default)]
    pub can_be_edited: bool,
}

impl Pooled for SCharacterCustomizerShaderParamBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_shader_param_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_shader_param_base }
}

impl<'a> Extract<'a> for SCharacterCustomizerShaderParamBase {
    const TYPE_NAME: &'static str = "SCharacterCustomizerShaderParamBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            apply_to_current_feature_material: inst.get_bool("applyToCurrentFeatureMaterial").unwrap_or_default(),
            shader_param_name: inst.get_str("shaderParamName").map(String::from).unwrap_or_default(),
            bindings_url: inst.get_str("bindingsURL").map(String::from).unwrap_or_default(),
            set_from_current_bindings: inst.get_bool("setFromCurrentBindings").unwrap_or_default(),
            requires_advanced_mode: inst.get_bool("requiresAdvancedMode").unwrap_or_default(),
            can_be_edited: inst.get_bool("canBeEdited").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerShaderParamFloat`
/// Inherits from: `SCharacterCustomizerShaderParamBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerShaderParamFloat {
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `applyToCurrentFeatureMaterial` (Boolean)
    #[serde(default)]
    pub apply_to_current_feature_material: bool,
    /// `shaderParamName` (String)
    #[serde(default)]
    pub shader_param_name: String,
    /// `bindingsURL` (String)
    #[serde(default)]
    pub bindings_url: String,
    /// `setFromCurrentBindings` (Boolean)
    #[serde(default)]
    pub set_from_current_bindings: bool,
    /// `requiresAdvancedMode` (Boolean)
    #[serde(default)]
    pub requires_advanced_mode: bool,
    /// `canBeEdited` (Boolean)
    #[serde(default)]
    pub can_be_edited: bool,
    /// `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// `max` (Single)
    #[serde(default)]
    pub max: f32,
}

impl Pooled for SCharacterCustomizerShaderParamFloat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_shader_param_float }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_shader_param_float }
}

impl<'a> Extract<'a> for SCharacterCustomizerShaderParamFloat {
    const TYPE_NAME: &'static str = "SCharacterCustomizerShaderParamFloat";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            apply_to_current_feature_material: inst.get_bool("applyToCurrentFeatureMaterial").unwrap_or_default(),
            shader_param_name: inst.get_str("shaderParamName").map(String::from).unwrap_or_default(),
            bindings_url: inst.get_str("bindingsURL").map(String::from).unwrap_or_default(),
            set_from_current_bindings: inst.get_bool("setFromCurrentBindings").unwrap_or_default(),
            requires_advanced_mode: inst.get_bool("requiresAdvancedMode").unwrap_or_default(),
            can_be_edited: inst.get_bool("canBeEdited").unwrap_or_default(),
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerShaderParamColor`
/// Inherits from: `SCharacterCustomizerShaderParamBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerShaderParamColor {
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `applyToCurrentFeatureMaterial` (Boolean)
    #[serde(default)]
    pub apply_to_current_feature_material: bool,
    /// `shaderParamName` (String)
    #[serde(default)]
    pub shader_param_name: String,
    /// `bindingsURL` (String)
    #[serde(default)]
    pub bindings_url: String,
    /// `setFromCurrentBindings` (Boolean)
    #[serde(default)]
    pub set_from_current_bindings: bool,
    /// `requiresAdvancedMode` (Boolean)
    #[serde(default)]
    pub requires_advanced_mode: bool,
    /// `canBeEdited` (Boolean)
    #[serde(default)]
    pub can_be_edited: bool,
    /// `useConstantSaturationColorPicker` (Boolean)
    #[serde(default)]
    pub use_constant_saturation_color_picker: bool,
    /// `shaderToUImultiplier` (Single)
    #[serde(default)]
    pub shader_to_uimultiplier: f32,
}

impl Pooled for SCharacterCustomizerShaderParamColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_shader_param_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_shader_param_color }
}

impl<'a> Extract<'a> for SCharacterCustomizerShaderParamColor {
    const TYPE_NAME: &'static str = "SCharacterCustomizerShaderParamColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            apply_to_current_feature_material: inst.get_bool("applyToCurrentFeatureMaterial").unwrap_or_default(),
            shader_param_name: inst.get_str("shaderParamName").map(String::from).unwrap_or_default(),
            bindings_url: inst.get_str("bindingsURL").map(String::from).unwrap_or_default(),
            set_from_current_bindings: inst.get_bool("setFromCurrentBindings").unwrap_or_default(),
            requires_advanced_mode: inst.get_bool("requiresAdvancedMode").unwrap_or_default(),
            can_be_edited: inst.get_bool("canBeEdited").unwrap_or_default(),
            use_constant_saturation_color_picker: inst.get_bool("useConstantSaturationColorPicker").unwrap_or_default(),
            shader_to_uimultiplier: inst.get_f32("shaderToUImultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerShaderParamCopy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerShaderParamCopy {
    /// `sourceParam` (WeakPointer)
    #[serde(default)]
    pub source_param: Option<SCharacterCustomizerShaderParamBasePtr>,
    /// `targetParams` (WeakPointer (array))
    #[serde(default)]
    pub target_params: Vec<SCharacterCustomizerShaderParamBasePtr>,
}

impl Pooled for SCharacterCustomizerShaderParamCopy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_shader_param_copy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_shader_param_copy }
}

impl<'a> Extract<'a> for SCharacterCustomizerShaderParamCopy {
    const TYPE_NAME: &'static str = "SCharacterCustomizerShaderParamCopy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            source_param: match inst.get("sourceParam") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerShaderParamBasePtr::from_ref(b, r)),
                _ => None,
            },
            target_params: inst.get_array("targetParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerShaderParamBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureShaderEdit`
/// Inherits from: `SCharacterCustomizerFeatureBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureShaderEdit {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
    /// `subMaterialsToEdit` (Int32 (array))
    #[serde(default)]
    pub sub_materials_to_edit: Vec<i32>,
    /// `shaderParams` (StrongPointer (array))
    #[serde(default)]
    pub shader_params: Vec<SCharacterCustomizerShaderParamBasePtr>,
    /// `shaderParamCopyBindings` (Class (array))
    #[serde(default)]
    pub shader_param_copy_bindings: Vec<Handle<SCharacterCustomizerShaderParamCopy>>,
}

impl Pooled for SCharacterCustomizerFeatureShaderEdit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_shader_edit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_shader_edit }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureShaderEdit {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureShaderEdit";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            sub_materials_to_edit: inst.get_array("subMaterialsToEdit")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            shader_params: inst.get_array("shaderParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerShaderParamBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            shader_param_copy_bindings: inst.get_array("shaderParamCopyBindings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCharacterCustomizerShaderParamCopy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SCharacterCustomizerShaderParamCopy>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerFeatureBodyTypeSelect`
/// Inherits from: `SCharacterCustomizerFeatureBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerFeatureBodyTypeSelect {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
}

impl Pooled for SCharacterCustomizerFeatureBodyTypeSelect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_feature_body_type_select }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_feature_body_type_select }
}

impl<'a> Extract<'a> for SCharacterCustomizerFeatureBodyTypeSelect {
    const TYPE_NAME: &'static str = "SCharacterCustomizerFeatureBodyTypeSelect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SCharacterCustomizerMakeupSlot`
/// Inherits from: `SCharacterCustomizerFeatureTextureSelect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerMakeupSlot {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `displayNameLocId` (Locale)
    #[serde(default)]
    pub display_name_loc_id: LocaleKey,
    /// `idString` (String)
    #[serde(default)]
    pub id_string: String,
    /// `subFeatures` (StrongPointer (array))
    #[serde(default)]
    pub sub_features: Vec<SCharacterCustomizerFeatureBasePtr>,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `randomizationParams` (StrongPointer)
    #[serde(default)]
    pub randomization_params: Option<SCharacterCustomizerRandomizationParamsPtr>,
    /// `validationParams` (StrongPointer)
    #[serde(default)]
    pub validation_params: Option<SCharacterValidationParamsPtr>,
    /// `featureType` (EnumChoice)
    #[serde(default)]
    pub feature_type: ECharacterCustomizerFeature,
    /// `materialParams` (WeakPointer)
    #[serde(default)]
    pub material_params: Option<Handle<SCustomizableMaterialParams>>,
    /// `precacheSkinMaterials` (Boolean)
    #[serde(default)]
    pub precache_skin_materials: bool,
    /// `supportsSubFeatureSelection` (Boolean)
    #[serde(default)]
    pub supports_sub_feature_selection: bool,
    /// `devModeOnly` (Boolean)
    #[serde(default)]
    pub dev_mode_only: bool,
    /// `itemportsToHide` (String (array))
    #[serde(default)]
    pub itemports_to_hide: Vec<String>,
    /// `libraryItemportsToHide` (String (array))
    #[serde(default)]
    pub library_itemports_to_hide: Vec<String>,
    /// `clippingVolumeParams` (Class)
    #[serde(default)]
    pub clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `advancedModeClippingVolumeParams` (Class)
    #[serde(default)]
    pub advanced_mode_clipping_volume_params: Option<Handle<SCharacterCustomizerClippingVolumeParams>>,
    /// `addEmptyEntry` (Boolean)
    #[serde(default)]
    pub add_empty_entry: bool,
    /// `featureLibOffset` (Class)
    #[serde(default)]
    pub feature_lib_offset: Option<Handle<Vec3>>,
    /// `advancedModeLibOffset` (Class)
    #[serde(default)]
    pub advanced_mode_lib_offset: Option<Handle<Vec3>>,
    /// `featureLibRowsOnScreen` (Int32)
    #[serde(default)]
    pub feature_lib_rows_on_screen: i32,
    /// `advancedModeRowsOnScreen` (Int32)
    #[serde(default)]
    pub advanced_mode_rows_on_screen: i32,
    /// `itemSelectMode` (EnumChoice)
    #[serde(default)]
    pub item_select_mode: ECharacterCustomizerItemSelectMode,
    /// `slot` (EnumChoice)
    #[serde(default)]
    pub slot: ECharacterCustomizerTextureSelectSlot,
    /// `textureSlot` (EnumChoice)
    #[serde(default)]
    pub texture_slot: ECharacterCustomizerTextureSlot,
    /// `subMaterialsToEdit` (Int32 (array))
    #[serde(default)]
    pub sub_materials_to_edit: Vec<i32>,
    /// `textures` (Class)
    #[serde(default)]
    pub textures: Option<Handle<SCharacterCustomizerTextureList>>,
    /// `smoothLimits` (Class)
    #[serde(default)]
    pub smooth_limits: Option<Handle<SCharacterCustomizerClampedValueParams>>,
    /// `metalLimits` (Class)
    #[serde(default)]
    pub metal_limits: Option<Handle<SCharacterCustomizerClampedValueParams>>,
    /// `opacityLimits` (Class)
    #[serde(default)]
    pub opacity_limits: Option<Handle<SCharacterCustomizerClampedValueParams>>,
    /// `basicModeChannel` (Int32)
    #[serde(default)]
    pub basic_mode_channel: i32,
}

impl Pooled for SCharacterCustomizerMakeupSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.scharacter_customizer_makeup_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.scharacter_customizer_makeup_slot }
}

impl<'a> Extract<'a> for SCharacterCustomizerMakeupSlot {
    const TYPE_NAME: &'static str = "SCharacterCustomizerMakeupSlot";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            display_name_loc_id: inst.get_str("displayNameLocId").map(LocaleKey::from).unwrap_or_default(),
            id_string: inst.get_str("idString").map(String::from).unwrap_or_default(),
            sub_features: inst.get_array("subFeatures")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SCharacterCustomizerFeatureBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            randomization_params: match inst.get("randomizationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterCustomizerRandomizationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            validation_params: match inst.get("validationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SCharacterValidationParamsPtr::from_ref(b, r)),
                _ => None,
            },
            feature_type: ECharacterCustomizerFeature::from_dcb_str(inst.get_str("featureType").unwrap_or("")),
            material_params: match inst.get("materialParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCustomizableMaterialParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precache_skin_materials: inst.get_bool("precacheSkinMaterials").unwrap_or_default(),
            supports_sub_feature_selection: inst.get_bool("supportsSubFeatureSelection").unwrap_or_default(),
            dev_mode_only: inst.get_bool("devModeOnly").unwrap_or_default(),
            itemports_to_hide: inst.get_array("itemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            library_itemports_to_hide: inst.get_array("libraryItemportsToHide")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            clipping_volume_params: match inst.get("clippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_clipping_volume_params: match inst.get("advancedModeClippingVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClippingVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            add_empty_entry: inst.get_bool("addEmptyEntry").unwrap_or_default(),
            feature_lib_offset: match inst.get("featureLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            advanced_mode_lib_offset: match inst.get("advancedModeLibOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            feature_lib_rows_on_screen: inst.get_i32("featureLibRowsOnScreen").unwrap_or_default(),
            advanced_mode_rows_on_screen: inst.get_i32("advancedModeRowsOnScreen").unwrap_or_default(),
            item_select_mode: ECharacterCustomizerItemSelectMode::from_dcb_str(inst.get_str("itemSelectMode").unwrap_or("")),
            slot: ECharacterCustomizerTextureSelectSlot::from_dcb_str(inst.get_str("slot").unwrap_or("")),
            texture_slot: ECharacterCustomizerTextureSlot::from_dcb_str(inst.get_str("textureSlot").unwrap_or("")),
            sub_materials_to_edit: inst.get_array("subMaterialsToEdit")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            textures: match inst.get("textures") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerTextureList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            smooth_limits: match inst.get("smoothLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClampedValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            metal_limits: match inst.get("metalLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClampedValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            opacity_limits: match inst.get("opacityLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCharacterCustomizerClampedValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            basic_mode_channel: inst.get_i32("basicModeChannel").unwrap_or_default(),
        }
    }
}

/// DCB type: `SModelVoiceTagPair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SModelVoiceTagPair {
    /// `modelTag` (Reference)
    #[serde(default)]
    pub model_tag: Option<CigGuid>,
    /// `voiceTag` (Reference)
    #[serde(default)]
    pub voice_tag: Option<CigGuid>,
}

impl Pooled for SModelVoiceTagPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_charactercustomizer.smodel_voice_tag_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_charactercustomizer.smodel_voice_tag_pair }
}

impl<'a> Extract<'a> for SModelVoiceTagPair {
    const TYPE_NAME: &'static str = "SModelVoiceTagPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            model_tag: inst.get("modelTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            voice_tag: inst.get("voiceTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

