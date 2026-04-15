// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-charactercustomizer` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesCharactercustomizerPools {
    #[serde(default)]
    pub entity_component_character_customizer_params: Vec<Option<EntityComponentCharacterCustomizerParams>>,
    #[serde(default)]
    pub scharacter_customizer_loadout_item_replacement_mapping: Vec<Option<SCharacterCustomizerLoadoutItemReplacementMapping>>,
    #[serde(default)]
    pub scharacter_customizer_head_library_params: Vec<Option<SCharacterCustomizerHeadLibraryParams>>,
    #[serde(default)]
    pub scharacter_customizer_blemish_map_params: Vec<Option<SCharacterCustomizerBlemishMapParams>>,
    #[serde(default)]
    pub scustomzier_color_srgba8: Vec<Option<SCustomzierColorSRGBA8>>,
    #[serde(default)]
    pub scharacter_customizer_default_shader_param: Vec<Option<SCharacterCustomizerDefaultShaderParam>>,
    #[serde(default)]
    pub scharacter_customizer_clamped_value_params: Vec<Option<SCharacterCustomizerClampedValueParams>>,
    #[serde(default)]
    pub scharacter_customizer_texture_params: Vec<Option<SCharacterCustomizerTextureParams>>,
    #[serde(default)]
    pub scharacter_customizer_texture_list: Vec<Option<SCharacterCustomizerTextureList>>,
    #[serde(default)]
    pub scharacter_customizer_makeup_params: Vec<Option<SCharacterCustomizerMakeupParams>>,
    #[serde(default)]
    pub scharacter_customizer_extra_texture_params: Vec<Option<SCharacterCustomizerExtraTextureParams>>,
    #[serde(default)]
    pub scharacter_customizer_multi_texture_params: Vec<Option<SCharacterCustomizerMultiTextureParams>>,
    #[serde(default)]
    pub scharacter_customizer_base_material_set: Vec<Option<SCharacterCustomizerBaseMaterialSet>>,
    #[serde(default)]
    pub scharacter_customizer_skin_base_material_list: Vec<Option<SCharacterCustomizerSkinBaseMaterialList>>,
    #[serde(default)]
    pub scharacter_customizer_material_edit_params: Vec<Option<SCharacterCustomizerMaterialEditParams>>,
    #[serde(default)]
    pub sface_highlighting_params: Vec<Option<SFaceHighlightingParams>>,
    #[serde(default)]
    pub scharacter_customizer_vertex_params: Vec<Option<SCharacterCustomizerVertexParams>>,
    #[serde(default)]
    pub scharacter_customizer_dnaregion_params: Vec<Option<SCharacterCustomizerDNARegionParams>>,
    #[serde(default)]
    pub scharacter_customizer_body_type_params: Vec<Option<SCharacterCustomizerBodyTypeParams>>,
    #[serde(default)]
    pub scharacter_customizer_randomization_override_params: Vec<Option<SCharacterCustomizerRandomizationOverrideParams>>,
    #[serde(default)]
    pub scharacter_customizer_control_params: Vec<Option<SCharacterCustomizerControlParams>>,
    #[serde(default)]
    pub scharacter_customizer_dnahead_pool: Vec<Option<SCharacterCustomizerDNAHeadPool>>,
    #[serde(default)]
    pub scustomizer_default_item: Vec<Option<SCustomizerDefaultItem>>,
    #[serde(default)]
    pub scharacter_customizer_dnahead_params: Vec<Option<SCharacterCustomizerDNAHeadParams>>,
    #[serde(default)]
    pub scharacter_customizer_clipping_volume_params: Vec<Option<SCharacterCustomizerClippingVolumeParams>>,
    #[serde(default)]
    pub scharacter_customizer_voice_params: Vec<Option<SCharacterCustomizerVoiceParams>>,
    #[serde(default)]
    pub scharacter_customizer_complexion_randomization_params: Vec<Option<SCharacterCustomizerComplexionRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_item_randomization_params: Vec<Option<SCharacterCustomizerItemRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_hair_randomization_params: Vec<Option<SCharacterCustomizerHairRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_makeup_slot_randomization_params: Vec<Option<SCharacterCustomizerMakeupSlotRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_tattoo_randomization_params: Vec<Option<SCharacterCustomizerTattooRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_random_color_def: Vec<Option<SCharacterCustomizerRandomColorDef>>,
    #[serde(default)]
    pub scharacter_customizer_eye_randomization_params: Vec<Option<SCharacterCustomizerEyeRandomizationParams>>,
    #[serde(default)]
    pub scharacter_customizer_hair_length_override: Vec<Option<SCharacterCustomizerHairLengthOverride>>,
    #[serde(default)]
    pub scharacter_customizer_hair_length_entry: Vec<Option<SCharacterCustomizerHairLengthEntry>>,
    #[serde(default)]
    pub scharacter_skin_validation_params: Vec<Option<SCharacterSkinValidationParams>>,
    #[serde(default)]
    pub scustomizable_material_params: Vec<Option<SCustomizableMaterialParams>>,
    #[serde(default)]
    pub scharacter_customizer_step: Vec<Option<SCharacterCustomizerStep>>,
    #[serde(default)]
    pub scharacter_customizer_feature_base: Vec<Option<SCharacterCustomizerFeatureBase>>,
    #[serde(default)]
    pub scharacter_customizer_feature_dna: Vec<Option<SCharacterCustomizerFeatureDNA>>,
    #[serde(default)]
    pub scharacter_customizer_feature_item_equip: Vec<Option<SCharacterCustomizerFeatureItemEquip>>,
    #[serde(default)]
    pub scharacter_customizer_feature_texture_select: Vec<Option<SCharacterCustomizerFeatureTextureSelect>>,
    #[serde(default)]
    pub scharacter_customizer_shader_param_base: Vec<Option<SCharacterCustomizerShaderParamBase>>,
    #[serde(default)]
    pub scharacter_customizer_shader_param_float: Vec<Option<SCharacterCustomizerShaderParamFloat>>,
    #[serde(default)]
    pub scharacter_customizer_shader_param_color: Vec<Option<SCharacterCustomizerShaderParamColor>>,
    #[serde(default)]
    pub scharacter_customizer_shader_param_copy: Vec<Option<SCharacterCustomizerShaderParamCopy>>,
    #[serde(default)]
    pub scharacter_customizer_feature_shader_edit: Vec<Option<SCharacterCustomizerFeatureShaderEdit>>,
    #[serde(default)]
    pub scharacter_customizer_feature_body_type_select: Vec<Option<SCharacterCustomizerFeatureBodyTypeSelect>>,
    #[serde(default)]
    pub scharacter_customizer_makeup_slot: Vec<Option<SCharacterCustomizerMakeupSlot>>,
    #[serde(default)]
    pub smodel_voice_tag_pair: Vec<Option<SModelVoiceTagPair>>,
}
