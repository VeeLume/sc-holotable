// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-uiconfig_starcitizen`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `InnerThought_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_Config {
    /// `tracking` (Single)
    #[serde(default)]
    pub tracking: f32,
    /// `forceCase` (EnumChoice)
    #[serde(default)]
    pub force_case: String,
    /// `geomFont` (Reference)
    #[serde(default)]
    pub geom_font: Option<CigGuid>,
}

impl Pooled for InnerThought_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.inner_thought_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.inner_thought_config }
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

/// DCB type: `UIConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// `DamageColours` (StrongPointer)
    #[serde(default)]
    pub damage_colours: Option<Handle<Flash_Palette>>,
    /// `ColorStates` (StrongPointer)
    #[serde(default)]
    pub color_states: Option<Handle<UIStateColor>>,
    /// `InnerThought` (StrongPointer)
    #[serde(default)]
    pub inner_thought: Option<Handle<InnerThought_Config>>,
    /// `FPSReticleConfig` (StrongPointer)
    #[serde(default)]
    pub fpsreticle_config: Option<Handle<FPSReticle_Config>>,
    /// `EVAReticleConfig` (StrongPointer)
    #[serde(default)]
    pub evareticle_config: Option<Handle<EVAReticle_Config>>,
    /// `playerChoiceIMConfig` (Reference)
    #[serde(default)]
    pub player_choice_imconfig: Option<CigGuid>,
    /// `visorHUDConfig` (Reference)
    #[serde(default)]
    pub visor_hudconfig: Option<CigGuid>,
    /// `playerChoicePITConfig` (Reference)
    #[serde(default)]
    pub player_choice_pitconfig: Option<CigGuid>,
    /// `flightHUDUIViewConfig` (Reference)
    #[serde(default)]
    pub flight_huduiview_config: Option<CigGuid>,
}

impl Pooled for UIConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.uiconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.uiconfig }
}

impl<'a> Extract<'a> for UIConfig {
    const TYPE_NAME: &'static str = "UIConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_colours: match inst.get("DamageColours") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Flash_Palette>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Flash_Palette>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color_states: match inst.get("ColorStates") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIStateColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIStateColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inner_thought: match inst.get("InnerThought") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InnerThought_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InnerThought_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fpsreticle_config: match inst.get("FPSReticleConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FPSReticle_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FPSReticle_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            evareticle_config: match inst.get("EVAReticleConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EVAReticle_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EVAReticle_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_choice_imconfig: inst.get("playerChoiceIMConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visor_hudconfig: inst.get("visorHUDConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            player_choice_pitconfig: inst.get("playerChoicePITConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            flight_huduiview_config: inst.get("flightHUDUIViewConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `FPSReticle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FPSReticle_Config {
    /// `AimAveragePoints` (Byte)
    #[serde(default)]
    pub aim_average_points: u32,
    /// `FlashWidth` (UInt16)
    #[serde(default)]
    pub flash_width: u32,
    /// `FlashHeight` (UInt16)
    #[serde(default)]
    pub flash_height: u32,
    /// `SpreadSize` (Single)
    #[serde(default)]
    pub spread_size: f32,
    /// `SpreadScaleMax` (Single)
    #[serde(default)]
    pub spread_scale_max: f32,
    /// `SpreadScaleInterpNeg` (Single)
    #[serde(default)]
    pub spread_scale_interp_neg: f32,
    /// `SpreadScaleInterpPos` (Single)
    #[serde(default)]
    pub spread_scale_interp_pos: f32,
    /// `SpreadAlphaInterpNeg` (Single)
    #[serde(default)]
    pub spread_alpha_interp_neg: f32,
    /// `SpreadAlphaInterpPos` (Single)
    #[serde(default)]
    pub spread_alpha_interp_pos: f32,
    /// `HiddenAlphaInterpNeg` (Single)
    #[serde(default)]
    pub hidden_alpha_interp_neg: f32,
    /// `MoveAlphaMinimum` (Single)
    #[serde(default)]
    pub move_alpha_minimum: f32,
    /// `MoveAlphaRange` (Single)
    #[serde(default)]
    pub move_alpha_range: f32,
}

impl Pooled for FPSReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.fpsreticle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.fpsreticle_config }
}

impl<'a> Extract<'a> for FPSReticle_Config {
    const TYPE_NAME: &'static str = "FPSReticle_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            aim_average_points: inst.get_u32("AimAveragePoints").unwrap_or_default(),
            flash_width: inst.get_u32("FlashWidth").unwrap_or_default(),
            flash_height: inst.get_u32("FlashHeight").unwrap_or_default(),
            spread_size: inst.get_f32("SpreadSize").unwrap_or_default(),
            spread_scale_max: inst.get_f32("SpreadScaleMax").unwrap_or_default(),
            spread_scale_interp_neg: inst.get_f32("SpreadScaleInterpNeg").unwrap_or_default(),
            spread_scale_interp_pos: inst.get_f32("SpreadScaleInterpPos").unwrap_or_default(),
            spread_alpha_interp_neg: inst.get_f32("SpreadAlphaInterpNeg").unwrap_or_default(),
            spread_alpha_interp_pos: inst.get_f32("SpreadAlphaInterpPos").unwrap_or_default(),
            hidden_alpha_interp_neg: inst.get_f32("HiddenAlphaInterpNeg").unwrap_or_default(),
            move_alpha_minimum: inst.get_f32("MoveAlphaMinimum").unwrap_or_default(),
            move_alpha_range: inst.get_f32("MoveAlphaRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `EVAReticle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAReticle_Config {
    /// `EnableFixedReticle` (Boolean)
    #[serde(default)]
    pub enable_fixed_reticle: bool,
    /// `EnableLookReticle` (Boolean)
    #[serde(default)]
    pub enable_look_reticle: bool,
    /// `EnableVelocityVector` (Boolean)
    #[serde(default)]
    pub enable_velocity_vector: bool,
    /// `EnableControlFrameReticle` (Boolean)
    #[serde(default)]
    pub enable_control_frame_reticle: bool,
}

impl Pooled for EVAReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.evareticle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.evareticle_config }
}

impl<'a> Extract<'a> for EVAReticle_Config {
    const TYPE_NAME: &'static str = "EVAReticle_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_fixed_reticle: inst.get_bool("EnableFixedReticle").unwrap_or_default(),
            enable_look_reticle: inst.get_bool("EnableLookReticle").unwrap_or_default(),
            enable_velocity_vector: inst.get_bool("EnableVelocityVector").unwrap_or_default(),
            enable_control_frame_reticle: inst.get_bool("EnableControlFrameReticle").unwrap_or_default(),
        }
    }
}

/// DCB type: `Flash_Palette`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flash_Palette {
    /// `Entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<Flash_PaletteEntry>>,
}

impl Pooled for Flash_Palette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.flash_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.flash_palette }
}

impl<'a> Extract<'a> for Flash_Palette {
    const TYPE_NAME: &'static str = "Flash_Palette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("Entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Flash_PaletteEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Flash_PaletteEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Flash_PaletteEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flash_PaletteEntry {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `FlashColor` (StrongPointer)
    #[serde(default)]
    pub flash_color: Option<Handle<SRGBA8>>,
}

impl Pooled for Flash_PaletteEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.flash_palette_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.flash_palette_entry }
}

impl<'a> Extract<'a> for Flash_PaletteEntry {
    const TYPE_NAME: &'static str = "Flash_PaletteEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            flash_color: match inst.get("FlashColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIStateColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateColor {
    /// `thresholds` (Class (array))
    #[serde(default)]
    pub thresholds: Vec<Handle<UIStateColor_Threshold>>,
}

impl Pooled for UIStateColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.uistate_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.uistate_color }
}

impl<'a> Extract<'a> for UIStateColor {
    const TYPE_NAME: &'static str = "UIStateColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            thresholds: inst.get_array("thresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIStateColor_Threshold>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIStateColor_Threshold>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UIStateColor_Threshold`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStateColor_Threshold {
    /// `minThresholdValue` (Single)
    #[serde(default)]
    pub min_threshold_value: f32,
    /// `stateColor` (EnumChoice)
    #[serde(default)]
    pub state_color: String,
}

impl Pooled for UIStateColor_Threshold {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uiconfig_starcitizen.uistate_color_threshold }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uiconfig_starcitizen.uistate_color_threshold }
}

impl<'a> Extract<'a> for UIStateColor_Threshold {
    const TYPE_NAME: &'static str = "UIStateColor_Threshold";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_threshold_value: inst.get_f32("minThresholdValue").unwrap_or_default(),
            state_color: inst.get_str("stateColor").map(String::from).unwrap_or_default(),
        }
    }
}

