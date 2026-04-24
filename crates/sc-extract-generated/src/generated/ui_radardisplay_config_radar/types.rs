// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-radardisplay_config_radar`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RadarPlate_Config`
pub struct RadarPlate_Config {
    /// `Color` (EnumChoice)
    pub color: HUDPalleteEntry,
    /// `Rotation` (Single)
    pub rotation: f32,
    /// `Segments` (Int32)
    pub segments: i32,
    /// `MainPlateSprite` (Class)
    pub main_plate_sprite: Option<Handle<SimpleSpriteSlot>>,
    /// `MainPlateInnerRings` (Int32)
    pub main_plate_inner_rings: i32,
    /// `InnerRingThickness` (Single)
    pub inner_ring_thickness: f32,
    /// `InnerRingUV_Start` (Class)
    pub inner_ring_uv_start: Option<Handle<Vec2>>,
    /// `InnerRingUV_Size` (Class)
    pub inner_ring_uv_size: Option<Handle<Vec2>>,
    /// `CenterIconScale` (Single)
    pub center_icon_scale: f32,
    /// `CenterIconSprite` (Class)
    pub center_icon_sprite: Option<Handle<SimpleSpriteSlot>>,
    /// `TopRingElevation` (Single)
    pub top_ring_elevation: f32,
    /// `TopRingThickness` (Single)
    pub top_ring_thickness: f32,
    /// `TopRingUV_Start` (Class)
    pub top_ring_uv_start: Option<Handle<Vec2>>,
    /// `TopRingUV_Size` (Class)
    pub top_ring_uv_size: Option<Handle<Vec2>>,
    /// `BottomARingElevation` (Single)
    pub bottom_aring_elevation: f32,
    /// `BottomARingSizeMultiplier` (Single)
    pub bottom_aring_size_multiplier: f32,
    /// `BottomARingGapAngle` (Single)
    pub bottom_aring_gap_angle: f32,
    /// `BottomARingThickness` (Single)
    pub bottom_aring_thickness: f32,
    /// `BottomARingUV_Start` (Class)
    pub bottom_aring_uv_start: Option<Handle<Vec2>>,
    /// `BottomARingUV_Size` (Class)
    pub bottom_aring_uv_size: Option<Handle<Vec2>>,
    /// `BottomBRingElevation` (Single)
    pub bottom_bring_elevation: f32,
    /// `BottomBRingSizeMultiplier` (Single)
    pub bottom_bring_size_multiplier: f32,
    /// `BottomBRingGapAngle` (Single)
    pub bottom_bring_gap_angle: f32,
    /// `BottomBRingThickness` (Single)
    pub bottom_bring_thickness: f32,
    /// `BottomBRingUV_Start` (Class)
    pub bottom_bring_uv_start: Option<Handle<Vec2>>,
    /// `BottomBRingUV_Size` (Class)
    pub bottom_bring_uv_size: Option<Handle<Vec2>>,
}

impl Pooled for RadarPlate_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radardisplay_config_radar.radar_plate_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radardisplay_config_radar.radar_plate_config }
}

impl<'a> Extract<'a> for RadarPlate_Config {
    const TYPE_NAME: &'static str = "RadarPlate_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: HUDPalleteEntry::from_dcb_str(inst.get_str("Color").unwrap_or("")),
            rotation: inst.get_f32("Rotation").unwrap_or_default(),
            segments: inst.get_i32("Segments").unwrap_or_default(),
            main_plate_sprite: match inst.get("MainPlateSprite") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            main_plate_inner_rings: inst.get_i32("MainPlateInnerRings").unwrap_or_default(),
            inner_ring_thickness: inst.get_f32("InnerRingThickness").unwrap_or_default(),
            inner_ring_uv_start: match inst.get("InnerRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            inner_ring_uv_size: match inst.get("InnerRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            center_icon_scale: inst.get_f32("CenterIconScale").unwrap_or_default(),
            center_icon_sprite: match inst.get("CenterIconSprite") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            top_ring_elevation: inst.get_f32("TopRingElevation").unwrap_or_default(),
            top_ring_thickness: inst.get_f32("TopRingThickness").unwrap_or_default(),
            top_ring_uv_start: match inst.get("TopRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            top_ring_uv_size: match inst.get("TopRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bottom_aring_elevation: inst.get_f32("BottomARingElevation").unwrap_or_default(),
            bottom_aring_size_multiplier: inst.get_f32("BottomARingSizeMultiplier").unwrap_or_default(),
            bottom_aring_gap_angle: inst.get_f32("BottomARingGapAngle").unwrap_or_default(),
            bottom_aring_thickness: inst.get_f32("BottomARingThickness").unwrap_or_default(),
            bottom_aring_uv_start: match inst.get("BottomARingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bottom_aring_uv_size: match inst.get("BottomARingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bottom_bring_elevation: inst.get_f32("BottomBRingElevation").unwrap_or_default(),
            bottom_bring_size_multiplier: inst.get_f32("BottomBRingSizeMultiplier").unwrap_or_default(),
            bottom_bring_gap_angle: inst.get_f32("BottomBRingGapAngle").unwrap_or_default(),
            bottom_bring_thickness: inst.get_f32("BottomBRingThickness").unwrap_or_default(),
            bottom_bring_uv_start: match inst.get("BottomBRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bottom_bring_uv_size: match inst.get("BottomBRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarDisplayEntryEffects_Config`
pub struct RadarDisplayEntryEffects_Config {
    /// `activatePingDuration` (Single)
    pub activate_ping_duration: f32,
    /// `activatePingMaxSize` (Single)
    pub activate_ping_max_size: f32,
    /// `activatePingRelativeColorChange` (Single)
    pub activate_ping_relative_color_change: f32,
    /// `fadeOutDuration` (Single)
    pub fade_out_duration: f32,
    /// `fadeOutLineOutStart` (Single)
    pub fade_out_line_out_start: f32,
}

impl Pooled for RadarDisplayEntryEffects_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radardisplay_config_radar.radar_display_entry_effects_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radardisplay_config_radar.radar_display_entry_effects_config }
}

impl<'a> Extract<'a> for RadarDisplayEntryEffects_Config {
    const TYPE_NAME: &'static str = "RadarDisplayEntryEffects_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activate_ping_duration: inst.get_f32("activatePingDuration").unwrap_or_default(),
            activate_ping_max_size: inst.get_f32("activatePingMaxSize").unwrap_or_default(),
            activate_ping_relative_color_change: inst.get_f32("activatePingRelativeColorChange").unwrap_or_default(),
            fade_out_duration: inst.get_f32("fadeOutDuration").unwrap_or_default(),
            fade_out_line_out_start: inst.get_f32("fadeOutLineOutStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDisplay_Config`
pub struct RadarDisplay_Config {
    /// `LineThickness` (Single)
    pub line_thickness: f32,
    /// `QuadSize` (Single)
    pub quad_size: f32,
    /// `LocalEntity` (Class)
    pub local_entity: Option<Handle<SimpleSpriteSlot>>,
    /// `Line` (Class)
    pub line: Option<Handle<SimpleSpriteSlot>>,
    /// `Number` (Class)
    pub number: Option<Handle<SimpleSpriteSlot>>,
    /// `Missile` (Class)
    pub missile: Option<Handle<SimpleSpriteSlot>>,
    /// `Core` (Class)
    pub core: Option<Handle<SimpleSpriteSlot>>,
    /// `Beacon` (Class)
    pub beacon: Option<Handle<SimpleSpriteSlot>>,
    /// `Blob` (Class)
    pub blob: Option<Handle<SimpleSpriteSlot>>,
    /// `ArrowUp` (Class)
    pub arrow_up: Option<Handle<SimpleSpriteSlot>>,
    /// `ArrowDown` (Class)
    pub arrow_down: Option<Handle<SimpleSpriteSlot>>,
    /// `FocusedArrowUp` (Class)
    pub focused_arrow_up: Option<Handle<SimpleSpriteSlot>>,
    /// `FocusedArrowDown` (Class)
    pub focused_arrow_down: Option<Handle<SimpleSpriteSlot>>,
    /// `ActivePingUV_Start` (Class)
    pub active_ping_uv_start: Option<Handle<Vec2>>,
    /// `ActivePingUV_Size` (Class)
    pub active_ping_uv_size: Option<Handle<Vec2>>,
    /// `ActivePingColor` (EnumChoice)
    pub active_ping_color: HUDPalleteEntry,
    /// `radarEntryEffects` (Class)
    pub radar_entry_effects: Option<Handle<RadarDisplayEntryEffects_Config>>,
    /// `radarPlateSettings` (Class)
    pub radar_plate_settings: Option<Handle<RadarPlate_Config>>,
}

impl Pooled for RadarDisplay_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radardisplay_config_radar.radar_display_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radardisplay_config_radar.radar_display_config }
}

impl<'a> Extract<'a> for RadarDisplay_Config {
    const TYPE_NAME: &'static str = "RadarDisplay_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            line_thickness: inst.get_f32("LineThickness").unwrap_or_default(),
            quad_size: inst.get_f32("QuadSize").unwrap_or_default(),
            local_entity: match inst.get("LocalEntity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            line: match inst.get("Line") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            number: match inst.get("Number") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            missile: match inst.get("Missile") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            core: match inst.get("Core") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            beacon: match inst.get("Beacon") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blob: match inst.get("Blob") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            arrow_up: match inst.get("ArrowUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            arrow_down: match inst.get("ArrowDown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            focused_arrow_up: match inst.get("FocusedArrowUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            focused_arrow_down: match inst.get("FocusedArrowDown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            active_ping_uv_start: match inst.get("ActivePingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            active_ping_uv_size: match inst.get("ActivePingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            active_ping_color: HUDPalleteEntry::from_dcb_str(inst.get_str("ActivePingColor").unwrap_or("")),
            radar_entry_effects: match inst.get("radarEntryEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarDisplayEntryEffects_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radar_plate_settings: match inst.get("radarPlateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarPlate_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

