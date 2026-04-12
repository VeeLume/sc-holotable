// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-innerthought`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `InnerThought_AnimBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_AnimBase {
    /// `glyphStagger` (Single)
    #[serde(default)]
    pub glyph_stagger: f32,
    /// `length` (Single)
    #[serde(default)]
    pub length: f32,
    /// `randomStagger` (Boolean)
    #[serde(default)]
    pub random_stagger: bool,
    /// `interpolationMode` (EnumChoice)
    #[serde(default)]
    pub interpolation_mode: String,
}

impl Pooled for InnerThought_AnimBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_anim_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_anim_base }
}

impl<'a> Extract<'a> for InnerThought_AnimBase {
    const TYPE_NAME: &'static str = "InnerThought_AnimBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            glyph_stagger: inst.get_f32("glyphStagger").unwrap_or_default(),
            length: inst.get_f32("length").unwrap_or_default(),
            random_stagger: inst.get_bool("randomStagger").unwrap_or_default(),
            interpolation_mode: inst.get_str("interpolationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_Anim`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_Anim {
    /// `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<InnerThought_AnimBase>>,
}

impl Pooled for InnerThought_Anim {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_anim }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_anim }
}

impl<'a> Extract<'a> for InnerThought_Anim {
    const TYPE_NAME: &'static str = "InnerThought_Anim";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InnerThought_AnimBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InnerThought_AnimBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InnerThought_LayoutBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_LayoutBase {
}

impl Pooled for InnerThought_LayoutBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_base }
}

impl<'a> Extract<'a> for InnerThought_LayoutBase {
    const TYPE_NAME: &'static str = "InnerThought_LayoutBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `InnerThought_ColorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_ColorParams {
    /// `diffuseColor` (Class)
    #[serde(default)]
    pub diffuse_color: Option<Handle<RGB>>,
    /// `emissiveColor` (Class)
    #[serde(default)]
    pub emissive_color: Option<Handle<RGB>>,
    /// `rimColor` (Class)
    #[serde(default)]
    pub rim_color: Option<Handle<RGB>>,
    /// `silhouetteColor` (Class)
    #[serde(default)]
    pub silhouette_color: Option<Handle<RGB>>,
    /// `opacity` (Single)
    #[serde(default)]
    pub opacity: f32,
    /// `glow` (Single)
    #[serde(default)]
    pub glow: f32,
    /// `diffuseOpacity` (Single)
    #[serde(default)]
    pub diffuse_opacity: f32,
    /// `rimOpacity` (Single)
    #[serde(default)]
    pub rim_opacity: f32,
    /// `silhouetteOpacity` (Single)
    #[serde(default)]
    pub silhouette_opacity: f32,
    /// `silhouetteThickness` (Single)
    #[serde(default)]
    pub silhouette_thickness: f32,
}

impl Pooled for InnerThought_ColorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_color_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_color_params }
}

impl<'a> Extract<'a> for InnerThought_ColorParams {
    const TYPE_NAME: &'static str = "InnerThought_ColorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            diffuse_color: match inst.get("diffuseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emissive_color: match inst.get("emissiveColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rim_color: match inst.get("rimColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            silhouette_color: match inst.get("silhouetteColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            opacity: inst.get_f32("opacity").unwrap_or_default(),
            glow: inst.get_f32("glow").unwrap_or_default(),
            diffuse_opacity: inst.get_f32("diffuseOpacity").unwrap_or_default(),
            rim_opacity: inst.get_f32("rimOpacity").unwrap_or_default(),
            silhouette_opacity: inst.get_f32("silhouetteOpacity").unwrap_or_default(),
            silhouette_thickness: inst.get_f32("silhouetteThickness").unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_LayoutStates`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_LayoutStates {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `layout` (StrongPointer)
    #[serde(default)]
    pub layout: Option<Handle<InnerThought_LayoutBase>>,
}

impl Pooled for InnerThought_LayoutStates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_states }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_states }
}

impl<'a> Extract<'a> for InnerThought_LayoutStates {
    const TYPE_NAME: &'static str = "InnerThought_LayoutStates";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            layout: match inst.get("layout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InnerThought_LayoutBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InnerThought_LayoutBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InnerThought_Params`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_Params {
    /// `fontSize` (Single)
    #[serde(default)]
    pub font_size: f32,
    /// `radialSelection` (Boolean)
    #[serde(default)]
    pub radial_selection: bool,
    /// `loopedSelection` (Boolean)
    #[serde(default)]
    pub looped_selection: bool,
    /// `useDepthTest` (Boolean)
    #[serde(default)]
    pub use_depth_test: bool,
    /// `states` (Class (array))
    #[serde(default)]
    pub states: Vec<Handle<InnerThought_LayoutStates>>,
    /// `stateAnim` (Reference)
    #[serde(default)]
    pub state_anim: Option<CigGuid>,
    /// `selectionAnim` (Reference)
    #[serde(default)]
    pub selection_anim: Option<CigGuid>,
}

impl Pooled for InnerThought_Params {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_params }
}

impl<'a> Extract<'a> for InnerThought_Params {
    const TYPE_NAME: &'static str = "InnerThought_Params";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            font_size: inst.get_f32("fontSize").unwrap_or_default(),
            radial_selection: inst.get_bool("radialSelection").unwrap_or_default(),
            looped_selection: inst.get_bool("loopedSelection").unwrap_or_default(),
            use_depth_test: inst.get_bool("useDepthTest").unwrap_or_default(),
            states: inst.get_array("states")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InnerThought_LayoutStates>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InnerThought_LayoutStates>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            state_anim: inst.get("stateAnim").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            selection_anim: inst.get("selectionAnim").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InnerThought_ConversationSystemConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_ConversationSystemConfig {
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `maxHorizontalAngle` (Single)
    #[serde(default)]
    pub max_horizontal_angle: f32,
    /// `maxVerticalAngle` (Single)
    #[serde(default)]
    pub max_vertical_angle: f32,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Deg3>>,
    /// `bone` (EnumChoice)
    #[serde(default)]
    pub bone: String,
    /// `boneOffset` (Class)
    #[serde(default)]
    pub bone_offset: Option<Handle<Vec3>>,
    /// `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// `translationRate` (Single)
    #[serde(default)]
    pub translation_rate: f32,
    /// `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_ConversationSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_conversation_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_conversation_system_config }
}

impl<'a> Extract<'a> for InnerThought_ConversationSystemConfig {
    const TYPE_NAME: &'static str = "InnerThought_ConversationSystemConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            max_horizontal_angle: inst.get_f32("maxHorizontalAngle").unwrap_or_default(),
            max_vertical_angle: inst.get_f32("maxVerticalAngle").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bone: inst.get_str("bone").map(String::from).unwrap_or_default(),
            bone_offset: match inst.get("boneOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_rate: inst.get_f32("rotationRate").unwrap_or_default(),
            translation_rate: inst.get_f32("translationRate").unwrap_or_default(),
            inner_thought: inst.get("innerThought").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InnerThought_InteractionSystemConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_InteractionSystemConfig {
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_InteractionSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_interaction_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_interaction_system_config }
}

impl<'a> Extract<'a> for InnerThought_InteractionSystemConfig {
    const TYPE_NAME: &'static str = "InnerThought_InteractionSystemConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            rotation_rate: inst.get_f32("rotationRate").unwrap_or_default(),
            inner_thought: inst.get("innerThought").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InnerThought_LegacyUseSystemConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_LegacyUseSystemConfig {
    /// `targetDistance` (Single)
    #[serde(default)]
    pub target_distance: f32,
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// `translationRate` (Single)
    #[serde(default)]
    pub translation_rate: f32,
    /// `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_LegacyUseSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_legacy_use_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_legacy_use_system_config }
}

impl<'a> Extract<'a> for InnerThought_LegacyUseSystemConfig {
    const TYPE_NAME: &'static str = "InnerThought_LegacyUseSystemConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            target_distance: inst.get_f32("targetDistance").unwrap_or_default(),
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            rotation_rate: inst.get_f32("rotationRate").unwrap_or_default(),
            translation_rate: inst.get_f32("translationRate").unwrap_or_default(),
            inner_thought: inst.get("innerThought").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

