// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-worlddisplay`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `WorldDisplayRadar`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayRadar {
    /// `hoverSizeMultiplier` (Single)
    #[serde(default)]
    pub hover_size_multiplier: f32,
    /// `unknown` (Class)
    #[serde(default)]
    pub unknown: Option<Handle<WorldDisplayRadar_Icon>>,
    /// `defaultUp` (Class)
    #[serde(default)]
    pub default_up: Option<Handle<WorldDisplayRadar_Icon>>,
    /// `defaultDown` (Class)
    #[serde(default)]
    pub default_down: Option<Handle<WorldDisplayRadar_Icon>>,
    /// `missile` (Class)
    #[serde(default)]
    pub missile: Option<Handle<WorldDisplayRadar_Icon>>,
    /// `horizontalLine` (Class)
    #[serde(default)]
    pub horizontal_line: Option<Handle<WorldDisplayRadar_Line>>,
    /// `verticalLine` (Class)
    #[serde(default)]
    pub vertical_line: Option<Handle<WorldDisplayRadar_Line>>,
}

impl Pooled for WorldDisplayRadar {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_radar }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_radar }
}

impl<'a> Extract<'a> for WorldDisplayRadar {
    const TYPE_NAME: &'static str = "WorldDisplayRadar";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hover_size_multiplier: inst.get_f32("hoverSizeMultiplier").unwrap_or_default(),
            unknown: match inst.get("unknown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_up: match inst.get("defaultUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_down: match inst.get("defaultDown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            missile: match inst.get("missile") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Icon>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            horizontal_line: match inst.get("horizontalLine") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Line>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Line>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vertical_line: match inst.get("verticalLine") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayRadar_Line>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayRadar_Line>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WorldDisplayRadar_Icon`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayRadar_Icon {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `size` (Single)
    #[serde(default)]
    pub size: f32,
    /// `spriteSlot` (Class)
    #[serde(default)]
    pub sprite_slot: Option<Handle<SimpleSpriteSlot>>,
}

impl Pooled for WorldDisplayRadar_Icon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_radar_icon }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_radar_icon }
}

impl<'a> Extract<'a> for WorldDisplayRadar_Icon {
    const TYPE_NAME: &'static str = "WorldDisplayRadar_Icon";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            size: inst.get_f32("size").unwrap_or_default(),
            sprite_slot: match inst.get("spriteSlot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WorldDisplayRadar_Line`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayRadar_Line {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `width` (Single)
    #[serde(default)]
    pub width: f32,
    /// `spriteSlot` (Class)
    #[serde(default)]
    pub sprite_slot: Option<Handle<SimpleSpriteSlot>>,
}

impl Pooled for WorldDisplayRadar_Line {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_radar_line }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_radar_line }
}

impl<'a> Extract<'a> for WorldDisplayRadar_Line {
    const TYPE_NAME: &'static str = "WorldDisplayRadar_Line";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            width: inst.get_f32("width").unwrap_or_default(),
            sprite_slot: match inst.get("spriteSlot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WorldDisplayEnvironment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayEnvironment {
    /// `environments` (StrongPointer (array))
    #[serde(default)]
    pub environments: Vec<Handle<WorldDisplayEnvironmentBase>>,
}

impl Pooled for WorldDisplayEnvironment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_environment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_environment }
}

impl<'a> Extract<'a> for WorldDisplayEnvironment {
    const TYPE_NAME: &'static str = "WorldDisplayEnvironment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            environments: inst.get_array("environments")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WorldDisplayEnvironmentBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WorldDisplayEnvironmentBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WorldDisplayEnvironmentBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayEnvironmentBase {
    /// `alignment` (EnumChoice)
    #[serde(default)]
    pub alignment: String,
    /// `environmentColor` (StrongPointer)
    #[serde(default)]
    pub environment_color: Option<Handle<WorldDisplayEnvironmentColor>>,
}

impl Pooled for WorldDisplayEnvironmentBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_environment_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_environment_base }
}

impl<'a> Extract<'a> for WorldDisplayEnvironmentBase {
    const TYPE_NAME: &'static str = "WorldDisplayEnvironmentBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            alignment: inst.get_str("alignment").map(String::from).unwrap_or_default(),
            environment_color: match inst.get("environmentColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WorldDisplayEnvironmentColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WorldDisplayEnvironmentColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WorldDisplayEnvironmentColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayEnvironmentColor {
}

impl Pooled for WorldDisplayEnvironmentColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_worlddisplay.world_display_environment_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_worlddisplay.world_display_environment_color }
}

impl<'a> Extract<'a> for WorldDisplayEnvironmentColor {
    const TYPE_NAME: &'static str = "WorldDisplayEnvironmentColor";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

