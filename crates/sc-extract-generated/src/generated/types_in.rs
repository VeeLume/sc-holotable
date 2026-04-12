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

/// DCB type: `IntoxicationIFCSModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationIFCSModifierParams {
    /// DCB field: `rotationDistortion` (Class)
    #[serde(default)]
    pub rotation_distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// DCB field: `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `pitch` (Class)
    #[serde(default)]
    pub pitch: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `roll` (Class)
    #[serde(default)]
    pub roll: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `rotationDelay` (Class)
    #[serde(default)]
    pub rotation_delay: Option<Handle<ToxiInputModifierDelay>>,
    /// DCB field: `linearDistortion` (Class)
    #[serde(default)]
    pub linear_distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// DCB field: `forward` (Class)
    #[serde(default)]
    pub forward: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `left` (Class)
    #[serde(default)]
    pub left: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `up` (Class)
    #[serde(default)]
    pub up: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `linearDelay` (Class)
    #[serde(default)]
    pub linear_delay: Option<Handle<ToxiInputModifierDelay>>,
}

impl Pooled for IntoxicationIFCSModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.intoxication_ifcsmodifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.intoxication_ifcsmodifier_params }
}

impl<'a> Extract<'a> for IntoxicationIFCSModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationIFCSModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation_distortion: match inst.get("rotationDistortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch: match inst.get("pitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll: match inst.get("roll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_delay: match inst.get("rotationDelay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_distortion: match inst.get("linearDistortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            forward: match inst.get("forward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left: match inst.get("left") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            up: match inst.get("up") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_delay: match inst.get("linearDelay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `IntoxicationTurretModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationTurretModifierParams {
    /// DCB field: `distortion` (Class)
    #[serde(default)]
    pub distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// DCB field: `delay` (Class)
    #[serde(default)]
    pub delay: Option<Handle<ToxiInputModifierDelay>>,
    /// DCB field: `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
    /// DCB field: `pitch` (Class)
    #[serde(default)]
    pub pitch: Option<Handle<ToxiInputModifierAxis>>,
}

impl Pooled for IntoxicationTurretModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.intoxication_turret_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.intoxication_turret_modifier_params }
}

impl<'a> Extract<'a> for IntoxicationTurretModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationTurretModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion: match inst.get("distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: match inst.get("delay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch: match inst.get("pitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `IntoxicationWheeledModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoxicationWheeledModifierParams {
    /// DCB field: `distortion` (Class)
    #[serde(default)]
    pub distortion: Option<Handle<ToxiInputModifierDistortion>>,
    /// DCB field: `delay` (Class)
    #[serde(default)]
    pub delay: Option<Handle<ToxiInputModifierDelay>>,
    /// DCB field: `yaw` (Class)
    #[serde(default)]
    pub yaw: Option<Handle<ToxiInputModifierAxis>>,
}

impl Pooled for IntoxicationWheeledModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.intoxication_wheeled_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.intoxication_wheeled_modifier_params }
}

impl<'a> Extract<'a> for IntoxicationWheeledModifierParams {
    const TYPE_NAME: &'static str = "IntoxicationWheeledModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion: match inst.get("distortion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDistortion>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: match inst.get("delay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierDelay>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierDelay>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw: match inst.get("yaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ToxiInputModifierAxis>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ToxiInputModifierAxis>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InputAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAction {
    /// DCB field: `actionName` (String)
    #[serde(default)]
    pub action_name: String,
}

impl Pooled for InputAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.input_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.input_action }
}

impl<'a> Extract<'a> for InputAction {
    const TYPE_NAME: &'static str = "InputAction";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action_name: inst.get_str("actionName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `InitialDamageOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialDamageOverride {
    /// DCB field: `initialDmgOverride` (Class)
    #[serde(default)]
    pub initial_dmg_override: Option<Handle<SInitialDamage>>,
}

impl Pooled for InitialDamageOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.initial_damage_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.initial_damage_override }
}

impl<'a> Extract<'a> for InitialDamageOverride {
    const TYPE_NAME: &'static str = "InitialDamageOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_dmg_override: match inst.get("initialDmgOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInitialDamage>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInitialDamage>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InnerThought_AnimBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_AnimBase {
    /// DCB field: `glyphStagger` (Single)
    #[serde(default)]
    pub glyph_stagger: f32,
    /// DCB field: `length` (Single)
    #[serde(default)]
    pub length: f32,
    /// DCB field: `randomStagger` (Boolean)
    #[serde(default)]
    pub random_stagger: bool,
    /// DCB field: `interpolationMode` (EnumChoice)
    #[serde(default)]
    pub interpolation_mode: String,
}

impl Pooled for InnerThought_AnimBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_anim_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_anim_base }
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
    /// DCB field: `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<InnerThought_AnimBase>>,
}

impl Pooled for InnerThought_Anim {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_anim }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_anim }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_layout_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_layout_base }
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
    /// DCB field: `diffuseColor` (Class)
    #[serde(default)]
    pub diffuse_color: Option<Handle<RGB>>,
    /// DCB field: `emissiveColor` (Class)
    #[serde(default)]
    pub emissive_color: Option<Handle<RGB>>,
    /// DCB field: `rimColor` (Class)
    #[serde(default)]
    pub rim_color: Option<Handle<RGB>>,
    /// DCB field: `silhouetteColor` (Class)
    #[serde(default)]
    pub silhouette_color: Option<Handle<RGB>>,
    /// DCB field: `opacity` (Single)
    #[serde(default)]
    pub opacity: f32,
    /// DCB field: `glow` (Single)
    #[serde(default)]
    pub glow: f32,
    /// DCB field: `diffuseOpacity` (Single)
    #[serde(default)]
    pub diffuse_opacity: f32,
    /// DCB field: `rimOpacity` (Single)
    #[serde(default)]
    pub rim_opacity: f32,
    /// DCB field: `silhouetteOpacity` (Single)
    #[serde(default)]
    pub silhouette_opacity: f32,
    /// DCB field: `silhouetteThickness` (Single)
    #[serde(default)]
    pub silhouette_thickness: f32,
}

impl Pooled for InnerThought_ColorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_color_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_color_params }
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

/// DCB type: `InnerThought_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_Config {
    /// DCB field: `tracking` (Single)
    #[serde(default)]
    pub tracking: f32,
    /// DCB field: `forceCase` (EnumChoice)
    #[serde(default)]
    pub force_case: String,
    /// DCB field: `geomFont` (Reference)
    #[serde(default)]
    pub geom_font: Option<CigGuid>,
}

impl Pooled for InnerThought_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_config }
}

impl<'a> Extract<'a> for InnerThought_Config {
    const TYPE_NAME: &'static str = "InnerThought_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tracking: inst.get_f32("tracking").unwrap_or_default(),
            force_case: inst.get_str("forceCase").map(String::from).unwrap_or_default(),
            geom_font: inst.get("geomFont").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InnerThought_LayoutStates`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_LayoutStates {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `layout` (StrongPointer)
    #[serde(default)]
    pub layout: Option<Handle<InnerThought_LayoutBase>>,
}

impl Pooled for InnerThought_LayoutStates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_layout_states }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_layout_states }
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
    /// DCB field: `fontSize` (Single)
    #[serde(default)]
    pub font_size: f32,
    /// DCB field: `radialSelection` (Boolean)
    #[serde(default)]
    pub radial_selection: bool,
    /// DCB field: `loopedSelection` (Boolean)
    #[serde(default)]
    pub looped_selection: bool,
    /// DCB field: `useDepthTest` (Boolean)
    #[serde(default)]
    pub use_depth_test: bool,
    /// DCB field: `states` (Class (array))
    #[serde(default)]
    pub states: Vec<Handle<InnerThought_LayoutStates>>,
    /// DCB field: `stateAnim` (Reference)
    #[serde(default)]
    pub state_anim: Option<CigGuid>,
    /// DCB field: `selectionAnim` (Reference)
    #[serde(default)]
    pub selection_anim: Option<CigGuid>,
}

impl Pooled for InnerThought_Params {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_params }
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
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `maxHorizontalAngle` (Single)
    #[serde(default)]
    pub max_horizontal_angle: f32,
    /// DCB field: `maxVerticalAngle` (Single)
    #[serde(default)]
    pub max_vertical_angle: f32,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Deg3>>,
    /// DCB field: `bone` (EnumChoice)
    #[serde(default)]
    pub bone: String,
    /// DCB field: `boneOffset` (Class)
    #[serde(default)]
    pub bone_offset: Option<Handle<Vec3>>,
    /// DCB field: `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// DCB field: `translationRate` (Single)
    #[serde(default)]
    pub translation_rate: f32,
    /// DCB field: `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_ConversationSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_conversation_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_conversation_system_config }
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
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// DCB field: `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_InteractionSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_interaction_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_interaction_system_config }
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
    /// DCB field: `targetDistance` (Single)
    #[serde(default)]
    pub target_distance: f32,
    /// DCB field: `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// DCB field: `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// DCB field: `rotationRate` (Single)
    #[serde(default)]
    pub rotation_rate: f32,
    /// DCB field: `translationRate` (Single)
    #[serde(default)]
    pub translation_rate: f32,
    /// DCB field: `innerThought` (Reference)
    #[serde(default)]
    pub inner_thought: Option<CigGuid>,
}

impl Pooled for InnerThought_LegacyUseSystemConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inner_thought_legacy_use_system_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inner_thought_legacy_use_system_config }
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

/// DCB type: `InputPromptConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPromptConfig {
    /// DCB field: `ownerName` (String)
    #[serde(default)]
    pub owner_name: String,
    /// DCB field: `actionName` (Class)
    #[serde(default)]
    pub action_name: Option<Handle<InputAction>>,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `showLabel` (Boolean)
    #[serde(default)]
    pub show_label: bool,
    /// DCB field: `inputPromptMode` (EnumChoice)
    #[serde(default)]
    pub input_prompt_mode: String,
    /// DCB field: `inputPromptBoundTo` (EnumChoice)
    #[serde(default)]
    pub input_prompt_bound_to: String,
    /// DCB field: `inputPromptPriority` (EnumChoice)
    #[serde(default)]
    pub input_prompt_priority: String,
    /// DCB field: `helperName` (String)
    #[serde(default)]
    pub helper_name: String,
    /// DCB field: `objectSlot` (Int32)
    #[serde(default)]
    pub object_slot: i32,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// DCB field: `isAngleConstrained` (Boolean)
    #[serde(default)]
    pub is_angle_constrained: bool,
    /// DCB field: `shouldShowOnSuccessEffect` (Boolean)
    #[serde(default)]
    pub should_show_on_success_effect: bool,
    /// DCB field: `ignoreDifficultySettings` (Boolean)
    #[serde(default)]
    pub ignore_difficulty_settings: bool,
}

impl Pooled for InputPromptConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.input_prompt_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.input_prompt_config }
}

impl<'a> Extract<'a> for InputPromptConfig {
    const TYPE_NAME: &'static str = "InputPromptConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            owner_name: inst.get_str("ownerName").map(String::from).unwrap_or_default(),
            action_name: match inst.get("actionName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            show_label: inst.get_bool("showLabel").unwrap_or_default(),
            input_prompt_mode: inst.get_str("inputPromptMode").map(String::from).unwrap_or_default(),
            input_prompt_bound_to: inst.get_str("inputPromptBoundTo").map(String::from).unwrap_or_default(),
            input_prompt_priority: inst.get_str("inputPromptPriority").map(String::from).unwrap_or_default(),
            helper_name: inst.get_str("helperName").map(String::from).unwrap_or_default(),
            object_slot: inst.get_i32("objectSlot").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_angle_constrained: inst.get_bool("isAngleConstrained").unwrap_or_default(),
            should_show_on_success_effect: inst.get_bool("shouldShowOnSuccessEffect").unwrap_or_default(),
            ignore_difficulty_settings: inst.get_bool("ignoreDifficultySettings").unwrap_or_default(),
        }
    }
}

/// DCB type: `InstancedInteriorLocationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancedInteriorLocationParams {
    /// DCB field: `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// DCB field: `devOnly` (Boolean)
    #[serde(default)]
    pub dev_only: bool,
    /// DCB field: `defaultHangars` (Reference)
    #[serde(default)]
    pub default_hangars: Option<CigGuid>,
}

impl Pooled for InstancedInteriorLocationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.instanced_interior_location_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.instanced_interior_location_params }
}

impl<'a> Extract<'a> for InstancedInteriorLocationParams {
    const TYPE_NAME: &'static str = "InstancedInteriorLocationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dev_only: inst.get_bool("devOnly").unwrap_or_default(),
            default_hangars: inst.get("defaultHangars").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InstancedInteriorLocationMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancedInteriorLocationMap {
    /// DCB field: `exitTimeBuffer` (Single)
    #[serde(default)]
    pub exit_time_buffer: f32,
    /// DCB field: `locationInteriors` (Reference (array))
    #[serde(default)]
    pub location_interiors: Vec<CigGuid>,
}

impl Pooled for InstancedInteriorLocationMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.instanced_interior_location_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.instanced_interior_location_map }
}

impl<'a> Extract<'a> for InstancedInteriorLocationMap {
    const TYPE_NAME: &'static str = "InstancedInteriorLocationMap";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            exit_time_buffer: inst.get_f32("exitTimeBuffer").unwrap_or_default(),
            location_interiors: inst.get_array("locationInteriors")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractionConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionParams {
    /// DCB field: `conditionDisplay` (StrongPointer)
    #[serde(default)]
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.interaction_condition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.interaction_condition_params }
}

impl<'a> Extract<'a> for InteractionConditionParams {
    const TYPE_NAME: &'static str = "InteractionConditionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConditionDisplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractionConditionPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionPreset {
    /// DCB field: `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<InteractionConditionParams>>,
    /// DCB field: `conditionToHideParams` (StrongPointer (array))
    #[serde(default)]
    pub condition_to_hide_params: Vec<Handle<InteractionConditionParams>>,
}

impl Pooled for InteractionConditionPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.interaction_condition_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.interaction_condition_preset }
}

impl<'a> Extract<'a> for InteractionConditionPreset {
    const TYPE_NAME: &'static str = "InteractionConditionPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            condition_to_hide_params: inst.get_array("conditionToHideParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractionConditionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionList {
    /// DCB field: `presetList` (Reference (array))
    #[serde(default)]
    pub preset_list: Vec<CigGuid>,
    /// DCB field: `conditionParams` (StrongPointer (array))
    #[serde(default)]
    pub condition_params: Vec<Handle<InteractionConditionParams>>,
    /// DCB field: `conditionToHideParams` (StrongPointer (array))
    #[serde(default)]
    pub condition_to_hide_params: Vec<Handle<InteractionConditionParams>>,
}

impl Pooled for InteractionConditionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.interaction_condition_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.interaction_condition_list }
}

impl<'a> Extract<'a> for InteractionConditionList {
    const TYPE_NAME: &'static str = "InteractionConditionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            preset_list: inst.get_array("presetList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            condition_params: inst.get_array("conditionParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            condition_to_hide_params: inst.get_array("conditionToHideParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InteractionConditionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InteractionConditionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractionPointTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPointTemplate {
    /// DCB field: `interactionPoint` (Class)
    #[serde(default)]
    pub interaction_point: Option<Handle<SInteractionPointParams>>,
}

impl Pooled for InteractionPointTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.interaction_point_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.interaction_point_template }
}

impl<'a> Extract<'a> for InteractionPointTemplate {
    const TYPE_NAME: &'static str = "InteractionPointTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction_point: match inst.get("interactionPoint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInteractionPointParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionPointParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InventoryContainerTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerTypeBase {
}

impl Pooled for InventoryContainerTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_container_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_container_type_base }
}

impl<'a> Extract<'a> for InventoryContainerTypeBase {
    const TYPE_NAME: &'static str = "InventoryContainerTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `InventoryContainerItemTypeFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerItemTypeFilter {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `itemSubTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_sub_types: Vec<String>,
}

impl Pooled for InventoryContainerItemTypeFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_container_item_type_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_container_item_type_filter }
}

impl<'a> Extract<'a> for InventoryContainerItemTypeFilter {
    const TYPE_NAME: &'static str = "InventoryContainerItemTypeFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            item_sub_types: inst.get_array("itemSubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InventoryContainerManager`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerManager {
    /// DCB field: `lootBoxes` (Reference (array))
    #[serde(default)]
    pub loot_boxes: Vec<CigGuid>,
    /// DCB field: `spawnLootGrid` (Class)
    #[serde(default)]
    pub spawn_loot_grid: Option<Handle<Vec3>>,
    /// DCB field: `closedInventoryNonStorableItemTypes` (Class (array))
    #[serde(default)]
    pub closed_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// DCB field: `closedInventoryNonStorableOutfitItemTypes` (EnumChoice (array))
    #[serde(default)]
    pub closed_inventory_non_storable_outfit_item_types: Vec<String>,
    /// DCB field: `openInventoryNonStorableItemTypes` (Class (array))
    #[serde(default)]
    pub open_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// DCB field: `itemTypeOpenInventoryOccupantProperties` (Class (array))
    #[serde(default)]
    pub item_type_open_inventory_occupant_properties: Vec<Handle<OpenInventoryOccupantItemTypeProperties>>,
    /// DCB field: `defaultOpenInventoryOccupantProperties` (Class)
    #[serde(default)]
    pub default_open_inventory_occupant_properties: Option<Handle<CargoGridOccupantProperties>>,
    /// DCB field: `smallItemDropDist` (Single)
    #[serde(default)]
    pub small_item_drop_dist: f32,
    /// DCB field: `BigItemDropDist` (Single)
    #[serde(default)]
    pub big_item_drop_dist: f32,
    /// DCB field: `bigItemVolumeThresholdSCU` (Single)
    #[serde(default)]
    pub big_item_volume_threshold_scu: f32,
    /// DCB field: `dropItemMaxHeight` (Single)
    #[serde(default)]
    pub drop_item_max_height: f32,
    /// DCB field: `dropItemSurfaceOffset` (Single)
    #[serde(default)]
    pub drop_item_surface_offset: f32,
    /// DCB field: `dropItemRetryOffset` (Single)
    #[serde(default)]
    pub drop_item_retry_offset: f32,
    /// DCB field: `numberOfRetries` (Int32)
    #[serde(default)]
    pub number_of_retries: i32,
}

impl Pooled for InventoryContainerManager {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_container_manager }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_container_manager }
}

impl<'a> Extract<'a> for InventoryContainerManager {
    const TYPE_NAME: &'static str = "InventoryContainerManager";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_boxes: inst.get_array("lootBoxes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            spawn_loot_grid: match inst.get("spawnLootGrid") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            closed_inventory_non_storable_item_types: inst.get_array("closedInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            closed_inventory_non_storable_outfit_item_types: inst.get_array("closedInventoryNonStorableOutfitItemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            open_inventory_non_storable_item_types: inst.get_array("openInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_type_open_inventory_occupant_properties: inst.get_array("itemTypeOpenInventoryOccupantProperties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_open_inventory_occupant_properties: match inst.get("defaultOpenInventoryOccupantProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            small_item_drop_dist: inst.get_f32("smallItemDropDist").unwrap_or_default(),
            big_item_drop_dist: inst.get_f32("BigItemDropDist").unwrap_or_default(),
            big_item_volume_threshold_scu: inst.get_f32("bigItemVolumeThresholdSCU").unwrap_or_default(),
            drop_item_max_height: inst.get_f32("dropItemMaxHeight").unwrap_or_default(),
            drop_item_surface_offset: inst.get_f32("dropItemSurfaceOffset").unwrap_or_default(),
            drop_item_retry_offset: inst.get_f32("dropItemRetryOffset").unwrap_or_default(),
            number_of_retries: inst.get_i32("numberOfRetries").unwrap_or_default(),
        }
    }
}

/// DCB type: `InventoryContainer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainer {
    /// DCB field: `excludedItemSubTypes` (Class (array))
    #[serde(default)]
    pub excluded_item_sub_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// DCB field: `interiorDimensions` (Class)
    #[serde(default)]
    pub interior_dimensions: Option<Handle<Vec3>>,
    /// DCB field: `inventoryType` (StrongPointer)
    #[serde(default)]
    pub inventory_type: Option<Handle<InventoryContainerTypeBase>>,
}

impl Pooled for InventoryContainer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_container }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_container }
}

impl<'a> Extract<'a> for InventoryContainer {
    const TYPE_NAME: &'static str = "InventoryContainer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            excluded_item_sub_types: inst.get_array("excludedItemSubTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            interior_dimensions: match inst.get("interiorDimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_type: match inst.get("inventoryType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InventoryContainerTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InventoryContainerTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InventoryLocation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLocation {
    /// DCB field: `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// DCB field: `visibleToPlayer` (Boolean)
    #[serde(default)]
    pub visible_to_player: bool,
    /// DCB field: `lawful` (Boolean)
    #[serde(default)]
    pub lawful: bool,
    /// DCB field: `impoundLocation` (Boolean)
    #[serde(default)]
    pub impound_location: bool,
}

impl Pooled for InventoryLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_location }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_location }
}

impl<'a> Extract<'a> for InventoryLocation {
    const TYPE_NAME: &'static str = "InventoryLocation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visible_to_player: inst.get_bool("visibleToPlayer").unwrap_or_default(),
            lawful: inst.get_bool("lawful").unwrap_or_default(),
            impound_location: inst.get_bool("impoundLocation").unwrap_or_default(),
        }
    }
}

/// DCB type: `InfractionParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfractionParameters {
    /// DCB field: `isFelony` (Int32)
    #[serde(default)]
    pub is_felony: i32,
    /// DCB field: `graceAllowance` (Int32)
    #[serde(default)]
    pub grace_allowance: i32,
    /// DCB field: `graceAllowanceCooldown` (Single)
    #[serde(default)]
    pub grace_allowance_cooldown: f32,
    /// DCB field: `gracePeriod` (Single)
    #[serde(default)]
    pub grace_period: f32,
    /// DCB field: `graceCooloffScale` (Single)
    #[serde(default)]
    pub grace_cooloff_scale: f32,
    /// DCB field: `graceWarnings` (Locale (array))
    #[serde(default)]
    pub grace_warnings: Vec<String>,
    /// DCB field: `displayGraceTime` (Int32)
    #[serde(default)]
    pub display_grace_time: i32,
    /// DCB field: `escalatedPaymentFineMultiplier` (Single)
    #[serde(default)]
    pub escalated_payment_fine_multiplier: f32,
    /// DCB field: `earlyPaymentPeriod` (Single)
    #[serde(default)]
    pub early_payment_period: f32,
    /// DCB field: `lifetime` (Single)
    #[serde(default)]
    pub lifetime: f32,
    /// DCB field: `coolOffTime` (Single)
    #[serde(default)]
    pub cool_off_time: f32,
    /// DCB field: `pressChargesNotificationTime` (Single)
    #[serde(default)]
    pub press_charges_notification_time: f32,
    /// DCB field: `removeTimeSeconds` (Single)
    #[serde(default)]
    pub remove_time_seconds: f32,
    /// DCB field: `felonyMerits` (Int32)
    #[serde(default)]
    pub felony_merits: i32,
    /// DCB field: `ignoreIfAgainstPartyMember` (Int32)
    #[serde(default)]
    pub ignore_if_against_party_member: i32,
    /// DCB field: `hideCrimeNotification` (Int32)
    #[serde(default)]
    pub hide_crime_notification: i32,
    /// DCB field: `hideCrimeInJournal` (Int32)
    #[serde(default)]
    pub hide_crime_in_journal: i32,
}

impl Pooled for InfractionParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.infraction_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.infraction_parameters }
}

impl<'a> Extract<'a> for InfractionParameters {
    const TYPE_NAME: &'static str = "InfractionParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_felony: inst.get_i32("isFelony").unwrap_or_default(),
            grace_allowance: inst.get_i32("graceAllowance").unwrap_or_default(),
            grace_allowance_cooldown: inst.get_f32("graceAllowanceCooldown").unwrap_or_default(),
            grace_period: inst.get_f32("gracePeriod").unwrap_or_default(),
            grace_cooloff_scale: inst.get_f32("graceCooloffScale").unwrap_or_default(),
            grace_warnings: inst.get_array("graceWarnings")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            display_grace_time: inst.get_i32("displayGraceTime").unwrap_or_default(),
            escalated_payment_fine_multiplier: inst.get_f32("escalatedPaymentFineMultiplier").unwrap_or_default(),
            early_payment_period: inst.get_f32("earlyPaymentPeriod").unwrap_or_default(),
            lifetime: inst.get_f32("lifetime").unwrap_or_default(),
            cool_off_time: inst.get_f32("coolOffTime").unwrap_or_default(),
            press_charges_notification_time: inst.get_f32("pressChargesNotificationTime").unwrap_or_default(),
            remove_time_seconds: inst.get_f32("removeTimeSeconds").unwrap_or_default(),
            felony_merits: inst.get_i32("felonyMerits").unwrap_or_default(),
            ignore_if_against_party_member: inst.get_i32("ignoreIfAgainstPartyMember").unwrap_or_default(),
            hide_crime_notification: inst.get_i32("hideCrimeNotification").unwrap_or_default(),
            hide_crime_in_journal: inst.get_i32("hideCrimeInJournal").unwrap_or_default(),
        }
    }
}

/// DCB type: `InfractionDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfractionDefinition {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `triggers` (EnumChoice (array))
    #[serde(default)]
    pub triggers: Vec<String>,
    /// DCB field: `defaultParameters` (Class)
    #[serde(default)]
    pub default_parameters: Option<Handle<InfractionParameters>>,
}

impl Pooled for InfractionDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.infraction_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.infraction_definition }
}

impl<'a> Extract<'a> for InfractionDefinition {
    const TYPE_NAME: &'static str = "InfractionDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            triggers: inst.get_array("triggers")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            default_parameters: match inst.get("defaultParameters") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InfractionParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InfractionParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Infraction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infraction {
    /// DCB field: `definition` (Reference)
    #[serde(default)]
    pub definition: Option<CigGuid>,
    /// DCB field: `parameterOverrides` (Class)
    #[serde(default)]
    pub parameter_overrides: Option<Handle<InfractionParameters>>,
}

impl Pooled for Infraction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.infraction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.infraction }
}

impl<'a> Extract<'a> for Infraction {
    const TYPE_NAME: &'static str = "Infraction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            definition: inst.get("definition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            parameter_overrides: match inst.get("parameterOverrides") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InfractionParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InfractionParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InteriorMapSectionDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteriorMapSectionDefinition {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `restrictViewBounds` (Boolean)
    #[serde(default)]
    pub restrict_view_bounds: bool,
}

impl Pooled for InteriorMapSectionDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.interior_map_section_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.interior_map_section_definition }
}

impl<'a> Extract<'a> for InteriorMapSectionDefinition {
    const TYPE_NAME: &'static str = "InteriorMapSectionDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            restrict_view_bounds: inst.get_bool("restrictViewBounds").unwrap_or_default(),
        }
    }
}

/// DCB type: `InventorySortMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySortMode {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for InventorySortMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_sort_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_sort_mode }
}

impl<'a> Extract<'a> for InventorySortMode {
    const TYPE_NAME: &'static str = "InventorySortMode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `InventoryContainerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerParams {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `tags` (String)
    #[serde(default)]
    pub tags: String,
    /// DCB field: `itemCategory` (Class)
    #[serde(default)]
    pub item_category: Option<Handle<ItemCategory>>,
}

impl Pooled for InventoryContainerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_container_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_container_params }
}

impl<'a> Extract<'a> for InventoryContainerParams {
    const TYPE_NAME: &'static str = "InventoryContainerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            tags: inst.get_str("tags").map(String::from).unwrap_or_default(),
            item_category: match inst.get("itemCategory") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InventoryDropDetachRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryDropDetachRules {
    /// DCB field: `category` (Class)
    #[serde(default)]
    pub category: Option<Handle<ItemCategory>>,
    /// DCB field: `dropDetachTypes` (Class (array))
    #[serde(default)]
    pub drop_detach_types: Vec<Handle<ItemCategory>>,
}

impl Pooled for InventoryDropDetachRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_in.inventory_drop_detach_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_in.inventory_drop_detach_rules }
}

impl<'a> Extract<'a> for InventoryDropDetachRules {
    const TYPE_NAME: &'static str = "InventoryDropDetachRules";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            category: match inst.get("category") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_detach_types: inst.get_array("dropDetachTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

