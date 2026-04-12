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

/// DCB type: `RagdollRecoveryConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagdollRecoveryConfig {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `physRootAdjust` (Class)
    #[serde(default)]
    pub phys_root_adjust: Option<Handle<AngYPR>>,
    /// DCB field: `defaultRecoveryAnims` (String (array))
    #[serde(default)]
    pub default_recovery_anims: Vec<String>,
}

impl Pooled for RagdollRecoveryConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.ragdoll_recovery_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.ragdoll_recovery_config }
}

impl<'a> Extract<'a> for RagdollRecoveryConfig {
    const TYPE_NAME: &'static str = "RagdollRecoveryConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            phys_root_adjust: match inst.get("physRootAdjust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AngYPR>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AngYPR>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_recovery_anims: inst.get_array("defaultRecoveryAnims")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Range`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// DCB field: `minimum` (Single)
    #[serde(default)]
    pub minimum: f32,
    /// DCB field: `maximum` (Single)
    #[serde(default)]
    pub maximum: f32,
}

impl Pooled for Range {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.range }
}

impl<'a> Extract<'a> for Range {
    const TYPE_NAME: &'static str = "Range";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum: inst.get_f32("minimum").unwrap_or_default(),
            maximum: inst.get_f32("maximum").unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibraryElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibraryElement {
    /// DCB field: `filePath` (String)
    #[serde(default)]
    pub file_path: String,
    /// DCB field: `displayName` (String)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `isMainModule` (Boolean)
    #[serde(default)]
    pub is_main_module: bool,
    /// DCB field: `isIndependantModule` (Boolean)
    #[serde(default)]
    pub is_independant_module: bool,
}

impl Pooled for RaSTaRLibraryElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.ra_sta_rlibrary_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.ra_sta_rlibrary_element }
}

impl<'a> Extract<'a> for RaSTaRLibraryElement {
    const TYPE_NAME: &'static str = "RaSTaRLibraryElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            file_path: inst.get_str("filePath").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            is_main_module: inst.get_bool("isMainModule").unwrap_or_default(),
            is_independant_module: inst.get_bool("isIndependantModule").unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibraryCategory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibraryCategory {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `elements` (Reference (array))
    #[serde(default)]
    pub elements: Vec<CigGuid>,
}

impl Pooled for RaSTaRLibraryCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.ra_sta_rlibrary_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.ra_sta_rlibrary_category }
}

impl<'a> Extract<'a> for RaSTaRLibraryCategory {
    const TYPE_NAME: &'static str = "RaSTaRLibraryCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            elements: inst.get_array("elements")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RaSTaRLibrary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaSTaRLibrary {
    /// DCB field: `categories` (Class (array))
    #[serde(default)]
    pub categories: Vec<Handle<RaSTaRLibraryCategory>>,
}

impl Pooled for RaSTaRLibrary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.ra_sta_rlibrary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.ra_sta_rlibrary }
}

impl<'a> Extract<'a> for RaSTaRLibrary {
    const TYPE_NAME: &'static str = "RaSTaRLibrary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RaSTaRLibraryCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RaSTaRLibraryCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarPlate_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarPlate_Config {
    /// DCB field: `Color` (EnumChoice)
    #[serde(default)]
    pub color: String,
    /// DCB field: `Rotation` (Single)
    #[serde(default)]
    pub rotation: f32,
    /// DCB field: `Segments` (Int32)
    #[serde(default)]
    pub segments: i32,
    /// DCB field: `MainPlateSprite` (Class)
    #[serde(default)]
    pub main_plate_sprite: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `MainPlateInnerRings` (Int32)
    #[serde(default)]
    pub main_plate_inner_rings: i32,
    /// DCB field: `InnerRingThickness` (Single)
    #[serde(default)]
    pub inner_ring_thickness: f32,
    /// DCB field: `InnerRingUV_Start` (Class)
    #[serde(default)]
    pub inner_ring_uv_start: Option<Handle<Vec2>>,
    /// DCB field: `InnerRingUV_Size` (Class)
    #[serde(default)]
    pub inner_ring_uv_size: Option<Handle<Vec2>>,
    /// DCB field: `CenterIconScale` (Single)
    #[serde(default)]
    pub center_icon_scale: f32,
    /// DCB field: `CenterIconSprite` (Class)
    #[serde(default)]
    pub center_icon_sprite: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `TopRingElevation` (Single)
    #[serde(default)]
    pub top_ring_elevation: f32,
    /// DCB field: `TopRingThickness` (Single)
    #[serde(default)]
    pub top_ring_thickness: f32,
    /// DCB field: `TopRingUV_Start` (Class)
    #[serde(default)]
    pub top_ring_uv_start: Option<Handle<Vec2>>,
    /// DCB field: `TopRingUV_Size` (Class)
    #[serde(default)]
    pub top_ring_uv_size: Option<Handle<Vec2>>,
    /// DCB field: `BottomARingElevation` (Single)
    #[serde(default)]
    pub bottom_aring_elevation: f32,
    /// DCB field: `BottomARingSizeMultiplier` (Single)
    #[serde(default)]
    pub bottom_aring_size_multiplier: f32,
    /// DCB field: `BottomARingGapAngle` (Single)
    #[serde(default)]
    pub bottom_aring_gap_angle: f32,
    /// DCB field: `BottomARingThickness` (Single)
    #[serde(default)]
    pub bottom_aring_thickness: f32,
    /// DCB field: `BottomARingUV_Start` (Class)
    #[serde(default)]
    pub bottom_aring_uv_start: Option<Handle<Vec2>>,
    /// DCB field: `BottomARingUV_Size` (Class)
    #[serde(default)]
    pub bottom_aring_uv_size: Option<Handle<Vec2>>,
    /// DCB field: `BottomBRingElevation` (Single)
    #[serde(default)]
    pub bottom_bring_elevation: f32,
    /// DCB field: `BottomBRingSizeMultiplier` (Single)
    #[serde(default)]
    pub bottom_bring_size_multiplier: f32,
    /// DCB field: `BottomBRingGapAngle` (Single)
    #[serde(default)]
    pub bottom_bring_gap_angle: f32,
    /// DCB field: `BottomBRingThickness` (Single)
    #[serde(default)]
    pub bottom_bring_thickness: f32,
    /// DCB field: `BottomBRingUV_Start` (Class)
    #[serde(default)]
    pub bottom_bring_uv_start: Option<Handle<Vec2>>,
    /// DCB field: `BottomBRingUV_Size` (Class)
    #[serde(default)]
    pub bottom_bring_uv_size: Option<Handle<Vec2>>,
}

impl Pooled for RadarPlate_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_plate_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_plate_config }
}

impl<'a> Extract<'a> for RadarPlate_Config {
    const TYPE_NAME: &'static str = "RadarPlate_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: inst.get_str("Color").map(String::from).unwrap_or_default(),
            rotation: inst.get_f32("Rotation").unwrap_or_default(),
            segments: inst.get_i32("Segments").unwrap_or_default(),
            main_plate_sprite: match inst.get("MainPlateSprite") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            main_plate_inner_rings: inst.get_i32("MainPlateInnerRings").unwrap_or_default(),
            inner_ring_thickness: inst.get_f32("InnerRingThickness").unwrap_or_default(),
            inner_ring_uv_start: match inst.get("InnerRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inner_ring_uv_size: match inst.get("InnerRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            center_icon_scale: inst.get_f32("CenterIconScale").unwrap_or_default(),
            center_icon_sprite: match inst.get("CenterIconSprite") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_ring_elevation: inst.get_f32("TopRingElevation").unwrap_or_default(),
            top_ring_thickness: inst.get_f32("TopRingThickness").unwrap_or_default(),
            top_ring_uv_start: match inst.get("TopRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_ring_uv_size: match inst.get("TopRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_aring_elevation: inst.get_f32("BottomARingElevation").unwrap_or_default(),
            bottom_aring_size_multiplier: inst.get_f32("BottomARingSizeMultiplier").unwrap_or_default(),
            bottom_aring_gap_angle: inst.get_f32("BottomARingGapAngle").unwrap_or_default(),
            bottom_aring_thickness: inst.get_f32("BottomARingThickness").unwrap_or_default(),
            bottom_aring_uv_start: match inst.get("BottomARingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_aring_uv_size: match inst.get("BottomARingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_bring_elevation: inst.get_f32("BottomBRingElevation").unwrap_or_default(),
            bottom_bring_size_multiplier: inst.get_f32("BottomBRingSizeMultiplier").unwrap_or_default(),
            bottom_bring_gap_angle: inst.get_f32("BottomBRingGapAngle").unwrap_or_default(),
            bottom_bring_thickness: inst.get_f32("BottomBRingThickness").unwrap_or_default(),
            bottom_bring_uv_start: match inst.get("BottomBRingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom_bring_uv_size: match inst.get("BottomBRingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarDisplayEntryEffects_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDisplayEntryEffects_Config {
    /// DCB field: `activatePingDuration` (Single)
    #[serde(default)]
    pub activate_ping_duration: f32,
    /// DCB field: `activatePingMaxSize` (Single)
    #[serde(default)]
    pub activate_ping_max_size: f32,
    /// DCB field: `activatePingRelativeColorChange` (Single)
    #[serde(default)]
    pub activate_ping_relative_color_change: f32,
    /// DCB field: `fadeOutDuration` (Single)
    #[serde(default)]
    pub fade_out_duration: f32,
    /// DCB field: `fadeOutLineOutStart` (Single)
    #[serde(default)]
    pub fade_out_line_out_start: f32,
}

impl Pooled for RadarDisplayEntryEffects_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_display_entry_effects_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_display_entry_effects_config }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDisplay_Config {
    /// DCB field: `LineThickness` (Single)
    #[serde(default)]
    pub line_thickness: f32,
    /// DCB field: `QuadSize` (Single)
    #[serde(default)]
    pub quad_size: f32,
    /// DCB field: `LocalEntity` (Class)
    #[serde(default)]
    pub local_entity: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Line` (Class)
    #[serde(default)]
    pub line: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Number` (Class)
    #[serde(default)]
    pub number: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Missile` (Class)
    #[serde(default)]
    pub missile: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Core` (Class)
    #[serde(default)]
    pub core: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Beacon` (Class)
    #[serde(default)]
    pub beacon: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `Blob` (Class)
    #[serde(default)]
    pub blob: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `ArrowUp` (Class)
    #[serde(default)]
    pub arrow_up: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `ArrowDown` (Class)
    #[serde(default)]
    pub arrow_down: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `FocusedArrowUp` (Class)
    #[serde(default)]
    pub focused_arrow_up: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `FocusedArrowDown` (Class)
    #[serde(default)]
    pub focused_arrow_down: Option<Handle<SimpleSpriteSlot>>,
    /// DCB field: `ActivePingUV_Start` (Class)
    #[serde(default)]
    pub active_ping_uv_start: Option<Handle<Vec2>>,
    /// DCB field: `ActivePingUV_Size` (Class)
    #[serde(default)]
    pub active_ping_uv_size: Option<Handle<Vec2>>,
    /// DCB field: `ActivePingColor` (EnumChoice)
    #[serde(default)]
    pub active_ping_color: String,
    /// DCB field: `radarEntryEffects` (Class)
    #[serde(default)]
    pub radar_entry_effects: Option<Handle<RadarDisplayEntryEffects_Config>>,
    /// DCB field: `radarPlateSettings` (Class)
    #[serde(default)]
    pub radar_plate_settings: Option<Handle<RadarPlate_Config>>,
}

impl Pooled for RadarDisplay_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_display_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_display_config }
}

impl<'a> Extract<'a> for RadarDisplay_Config {
    const TYPE_NAME: &'static str = "RadarDisplay_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            line_thickness: inst.get_f32("LineThickness").unwrap_or_default(),
            quad_size: inst.get_f32("QuadSize").unwrap_or_default(),
            local_entity: match inst.get("LocalEntity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            line: match inst.get("Line") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            number: match inst.get("Number") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            missile: match inst.get("Missile") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            core: match inst.get("Core") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            beacon: match inst.get("Beacon") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blob: match inst.get("Blob") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arrow_up: match inst.get("ArrowUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arrow_down: match inst.get("ArrowDown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            focused_arrow_up: match inst.get("FocusedArrowUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            focused_arrow_down: match inst.get("FocusedArrowDown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SimpleSpriteSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SimpleSpriteSlot>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            active_ping_uv_start: match inst.get("ActivePingUV_Start") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            active_ping_uv_size: match inst.get("ActivePingUV_Size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            active_ping_color: inst.get_str("ActivePingColor").map(String::from).unwrap_or_default(),
            radar_entry_effects: match inst.get("radarEntryEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarDisplayEntryEffects_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarDisplayEntryEffects_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radar_plate_settings: match inst.get("radarPlateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarPlate_Config>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarPlate_Config>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarSystemGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSystemGlobalParams {
    /// DCB field: `paramsVersion` (UInt32)
    #[serde(default)]
    pub params_version: u32,
    /// DCB field: `signatureSystemParams` (Class)
    #[serde(default)]
    pub signature_system_params: Option<Handle<SignatureSystemGlobalParams>>,
    /// DCB field: `contactStateParams` (Class)
    #[serde(default)]
    pub contact_state_params: Option<Handle<ContactStateGlobalParams>>,
}

impl Pooled for RadarSystemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_system_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_system_global_params }
}

impl<'a> Extract<'a> for RadarSystemGlobalParams {
    const TYPE_NAME: &'static str = "RadarSystemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params_version: inst.get_u32("paramsVersion").unwrap_or_default(),
            signature_system_params: match inst.get("signatureSystemParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureSystemGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureSystemGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contact_state_params: match inst.get("contactStateParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactStateGlobalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactStateGlobalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarDeltaSignatureDetectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureDetectionParams {
    /// DCB field: `deltaSignatureDetection` (Reference)
    #[serde(default)]
    pub delta_signature_detection: Option<CigGuid>,
    /// DCB field: `alwaysDetect` (Boolean)
    #[serde(default)]
    pub always_detect: bool,
    /// DCB field: `markActiveDetection` (Boolean)
    #[serde(default)]
    pub mark_active_detection: bool,
    /// DCB field: `detectionChargeLevel` (EnumChoice)
    #[serde(default)]
    pub detection_charge_level: String,
    /// DCB field: `emissionModifier` (Single)
    #[serde(default)]
    pub emission_modifier: f32,
    /// DCB field: `scanWaveDetectionParams` (StrongPointer)
    #[serde(default)]
    pub scan_wave_detection_params: Option<Handle<ScanWaveDetectionParams>>,
}

impl Pooled for RadarDeltaSignatureDetectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_delta_signature_detection_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_delta_signature_detection_params }
}

impl<'a> Extract<'a> for RadarDeltaSignatureDetectionParams {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureDetectionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            delta_signature_detection: inst.get("deltaSignatureDetection").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            always_detect: inst.get_bool("alwaysDetect").unwrap_or_default(),
            mark_active_detection: inst.get_bool("markActiveDetection").unwrap_or_default(),
            detection_charge_level: inst.get_str("detectionChargeLevel").map(String::from).unwrap_or_default(),
            emission_modifier: inst.get_f32("emissionModifier").unwrap_or_default(),
            scan_wave_detection_params: match inst.get("scanWaveDetectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanWaveDetectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanWaveDetectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarSystemSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSystemSharedParams {
    /// DCB field: `paramsVersion` (UInt32)
    #[serde(default)]
    pub params_version: u32,
    /// DCB field: `focusAngles` (Single)
    #[serde(default)]
    pub focus_angles: f32,
    /// DCB field: `radarParams` (StrongPointer)
    #[serde(default)]
    pub radar_params: Option<Handle<RadarSharedParams>>,
    /// DCB field: `scanParams` (StrongPointer)
    #[serde(default)]
    pub scan_params: Option<Handle<ScanSharedParams>>,
    /// DCB field: `pingParams` (StrongPointer)
    #[serde(default)]
    pub ping_params: Option<Handle<PingSharedParams>>,
    /// DCB field: `highlightParams` (StrongPointer)
    #[serde(default)]
    pub highlight_params: Option<Handle<ContactHighlightSharedParams>>,
    /// DCB field: `taggingParams` (StrongPointer)
    #[serde(default)]
    pub tagging_params: Option<Handle<ContactTaggingSharedParams>>,
    /// DCB field: `occlusionParams` (StrongPointer)
    #[serde(default)]
    pub occlusion_params: Option<Handle<OcclusionCheckSharedParams>>,
    /// DCB field: `roomSharedParams` (StrongPointer)
    #[serde(default)]
    pub room_shared_params: Option<Handle<RoomSharedParams>>,
    /// DCB field: `displayRadarContactMarkers` (Boolean)
    #[serde(default)]
    pub display_radar_contact_markers: bool,
    /// DCB field: `applyNearbyAmbientSignatures` (Boolean)
    #[serde(default)]
    pub apply_nearby_ambient_signatures: bool,
    /// DCB field: `radarJammerParams` (StrongPointer)
    #[serde(default)]
    pub radar_jammer_params: Option<Handle<RadarJammerSharedParams>>,
    /// DCB field: `deltaSignatureDetectionParams` (Class (array))
    #[serde(default)]
    pub delta_signature_detection_params: Vec<Handle<RadarDeltaSignatureDetectionParams>>,
    /// DCB field: `deltaSignatureBaseSpike` (Class)
    #[serde(default)]
    pub delta_signature_base_spike: Option<Handle<DeltaSignatureSpikeParams>>,
    /// DCB field: `deltaSignatureSensitivityParams` (Class)
    #[serde(default)]
    pub delta_signature_sensitivity_params: Option<Handle<DeltaSignatureSensitivityParams>>,
}

impl Pooled for RadarSystemSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_system_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_system_shared_params }
}

impl<'a> Extract<'a> for RadarSystemSharedParams {
    const TYPE_NAME: &'static str = "RadarSystemSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params_version: inst.get_u32("paramsVersion").unwrap_or_default(),
            focus_angles: inst.get_f32("focusAngles").unwrap_or_default(),
            radar_params: match inst.get("radarParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_params: match inst.get("scanParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_params: match inst.get("pingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tagging_params: match inst.get("taggingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactTaggingSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactTaggingSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occlusion_params: match inst.get("occlusionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OcclusionCheckSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OcclusionCheckSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_shared_params: match inst.get("roomSharedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_radar_contact_markers: inst.get_bool("displayRadarContactMarkers").unwrap_or_default(),
            apply_nearby_ambient_signatures: inst.get_bool("applyNearbyAmbientSignatures").unwrap_or_default(),
            radar_jammer_params: match inst.get("radarJammerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarJammerSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarJammerSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delta_signature_detection_params: inst.get_array("deltaSignatureDetectionParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RadarDeltaSignatureDetectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RadarDeltaSignatureDetectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            delta_signature_base_spike: match inst.get("deltaSignatureBaseSpike") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DeltaSignatureSpikeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DeltaSignatureSpikeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delta_signature_sensitivity_params: match inst.get("deltaSignatureSensitivityParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DeltaSignatureSensitivityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DeltaSignatureSensitivityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSharedParams {
    /// DCB field: `maxPassiveDistance` (Single)
    #[serde(default)]
    pub max_passive_distance: f32,
    /// DCB field: `useRoomGraphForPassiveDetection` (Boolean)
    #[serde(default)]
    pub use_room_graph_for_passive_detection: bool,
}

impl Pooled for RadarSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_shared_params }
}

impl<'a> Extract<'a> for RadarSharedParams {
    const TYPE_NAME: &'static str = "RadarSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_passive_distance: inst.get_f32("maxPassiveDistance").unwrap_or_default(),
            use_room_graph_for_passive_detection: inst.get_bool("useRoomGraphForPassiveDetection").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarJammerSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarJammerSharedParams {
    /// DCB field: `generalHUDDistortionStrength` (Single)
    #[serde(default)]
    pub general_huddistortion_strength: f32,
    /// DCB field: `minimapDistortionStrength` (Single)
    #[serde(default)]
    pub minimap_distortion_strength: f32,
    /// DCB field: `jammedActiveDetectionRange` (Single)
    #[serde(default)]
    pub jammed_active_detection_range: f32,
    /// DCB field: `jammedPingNotificationText` (Locale)
    #[serde(default)]
    pub jammed_ping_notification_text: String,
    /// DCB field: `jammedPingNotificationAudioTag` (String)
    #[serde(default)]
    pub jammed_ping_notification_audio_tag: String,
}

impl Pooled for RadarJammerSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_jammer_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_jammer_shared_params }
}

impl<'a> Extract<'a> for RadarJammerSharedParams {
    const TYPE_NAME: &'static str = "RadarJammerSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            general_huddistortion_strength: inst.get_f32("generalHUDDistortionStrength").unwrap_or_default(),
            minimap_distortion_strength: inst.get_f32("minimapDistortionStrength").unwrap_or_default(),
            jammed_active_detection_range: inst.get_f32("jammedActiveDetectionRange").unwrap_or_default(),
            jammed_ping_notification_text: inst.get_str("jammedPingNotificationText").map(String::from).unwrap_or_default(),
            jammed_ping_notification_audio_tag: inst.get_str("jammedPingNotificationAudioTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGamePlayProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGamePlayProperties {
    /// DCB field: `perceivedByAISense` (Boolean)
    #[serde(default)]
    pub perceived_by_aisense: bool,
}

impl Pooled for RadarContactGamePlayProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_game_play_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_game_play_properties }
}

impl<'a> Extract<'a> for RadarContactGamePlayProperties {
    const TYPE_NAME: &'static str = "RadarContactGamePlayProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            perceived_by_aisense: inst.get_bool("perceivedByAISense").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarSignatureCategoryDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSignatureCategoryDefinition {
    /// DCB field: `default` (Reference)
    #[serde(default)]
    pub default: Option<CigGuid>,
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarSignatureCategoryDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_signature_category_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_signature_category_definition }
}

impl<'a> Extract<'a> for RadarSignatureCategoryDefinition {
    const TYPE_NAME: &'static str = "RadarSignatureCategoryDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default: inst.get("default").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarSignatureCategoryEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarSignatureCategoryEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for RadarSignatureCategoryEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_signature_category_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_signature_category_entry }
}

impl<'a> Extract<'a> for RadarSignatureCategoryEntry {
    const TYPE_NAME: &'static str = "RadarSignatureCategoryEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactTypeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactTypeDefinition {
    /// DCB field: `unknownType` (Reference)
    #[serde(default)]
    pub unknown_type: Option<CigGuid>,
    /// DCB field: `defaultAudioType` (Reference)
    #[serde(default)]
    pub default_audio_type: Option<CigGuid>,
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarContactTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_type_definition }
}

impl<'a> Extract<'a> for RadarContactTypeDefinition {
    const TYPE_NAME: &'static str = "RadarContactTypeDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            unknown_type: inst.get("unknownType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_audio_type: inst.get("defaultAudioType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactTypeEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactTypeEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `scanDef` (Reference)
    #[serde(default)]
    pub scan_def: Option<CigGuid>,
    /// DCB field: `gameplayProperties` (Class)
    #[serde(default)]
    pub gameplay_properties: Option<Handle<RadarContactGamePlayProperties>>,
    /// DCB field: `trackerType` (EnumChoice)
    #[serde(default)]
    pub tracker_type: String,
    /// DCB field: `markerConfig` (Reference)
    #[serde(default)]
    pub marker_config: Option<CigGuid>,
    /// DCB field: `contactHoloMinScreenSize` (Single)
    #[serde(default)]
    pub contact_holo_min_screen_size: f32,
    /// DCB field: `contactHighlightCategory` (EnumChoice)
    #[serde(default)]
    pub contact_highlight_category: String,
    /// DCB field: `contactTaggingCategory` (EnumChoice)
    #[serde(default)]
    pub contact_tagging_category: String,
    /// DCB field: `isObjectOfInterest` (Boolean)
    #[serde(default)]
    pub is_object_of_interest: bool,
    /// DCB field: `minimumInfluenceFactor` (Single)
    #[serde(default)]
    pub minimum_influence_factor: f32,
    /// DCB field: `minimumInfluenceOperation` (EnumChoice)
    #[serde(default)]
    pub minimum_influence_operation: String,
    /// DCB field: `onlyDetectableFromSameZone` (Boolean)
    #[serde(default)]
    pub only_detectable_from_same_zone: bool,
}

impl Pooled for RadarContactTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_type_entry }
}

impl<'a> Extract<'a> for RadarContactTypeEntry {
    const TYPE_NAME: &'static str = "RadarContactTypeEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            scan_def: inst.get("scanDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gameplay_properties: match inst.get("gameplayProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarContactGamePlayProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarContactGamePlayProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tracker_type: inst.get_str("trackerType").map(String::from).unwrap_or_default(),
            marker_config: inst.get("markerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contact_holo_min_screen_size: inst.get_f32("contactHoloMinScreenSize").unwrap_or_default(),
            contact_highlight_category: inst.get_str("contactHighlightCategory").map(String::from).unwrap_or_default(),
            contact_tagging_category: inst.get_str("contactTaggingCategory").map(String::from).unwrap_or_default(),
            is_object_of_interest: inst.get_bool("isObjectOfInterest").unwrap_or_default(),
            minimum_influence_factor: inst.get_f32("minimumInfluenceFactor").unwrap_or_default(),
            minimum_influence_operation: inst.get_str("minimumInfluenceOperation").map(String::from).unwrap_or_default(),
            only_detectable_from_same_zone: inst.get_bool("onlyDetectableFromSameZone").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGroupDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGroupDefinition {
    /// DCB field: `groups` (Reference (array))
    #[serde(default)]
    pub groups: Vec<CigGuid>,
}

impl Pooled for RadarContactGroupDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_group_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_group_definition }
}

impl<'a> Extract<'a> for RadarContactGroupDefinition {
    const TYPE_NAME: &'static str = "RadarContactGroupDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactGroupEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactGroupEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
    /// DCB field: `children` (Class (array))
    #[serde(default)]
    pub children: Vec<Handle<RadarContactSubGroupEntry>>,
}

impl Pooled for RadarContactGroupEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_group_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_group_entry }
}

impl<'a> Extract<'a> for RadarContactGroupEntry {
    const TYPE_NAME: &'static str = "RadarContactGroupEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            children: inst.get_array("children")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RadarContactSubGroupEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RadarContactSubGroupEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarContactSubGroupEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarContactSubGroupEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarContactSubGroupEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_contact_sub_group_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_contact_sub_group_entry }
}

impl<'a> Extract<'a> for RadarContactSubGroupEntry {
    const TYPE_NAME: &'static str = "RadarContactSubGroupEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureNotificationParams {
    /// DCB field: `requireLockedTarget` (Boolean)
    #[serde(default)]
    pub require_locked_target: bool,
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// DCB field: `globalCooldown` (Single)
    #[serde(default)]
    pub global_cooldown: f32,
    /// DCB field: `individualCooldown` (Single)
    #[serde(default)]
    pub individual_cooldown: f32,
}

impl Pooled for RadarDeltaSignatureNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_delta_signature_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_delta_signature_notification_params }
}

impl<'a> Extract<'a> for RadarDeltaSignatureNotificationParams {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            require_locked_target: inst.get_bool("requireLockedTarget").unwrap_or_default(),
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            global_cooldown: inst.get_f32("globalCooldown").unwrap_or_default(),
            individual_cooldown: inst.get_f32("individualCooldown").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureDefinition {
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for RadarDeltaSignatureDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_delta_signature_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_delta_signature_definition }
}

impl<'a> Extract<'a> for RadarDeltaSignatureDefinition {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDeltaSignatureEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDeltaSignatureEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `priority` (EnumChoice)
    #[serde(default)]
    pub priority: String,
    /// DCB field: `expireTime` (Single)
    #[serde(default)]
    pub expire_time: f32,
    /// DCB field: `notificationParams` (Class)
    #[serde(default)]
    pub notification_params: Option<Handle<RadarDeltaSignatureNotificationParams>>,
}

impl Pooled for RadarDeltaSignatureEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_delta_signature_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_delta_signature_entry }
}

impl<'a> Extract<'a> for RadarDeltaSignatureEntry {
    const TYPE_NAME: &'static str = "RadarDeltaSignatureEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            priority: inst.get_str("priority").map(String::from).unwrap_or_default(),
            expire_time: inst.get_f32("expireTime").unwrap_or_default(),
            notification_params: match inst.get("notificationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadarDeltaSignatureNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadarDeltaSignatureNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadiationStatePropertyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationStatePropertyParams {
    /// DCB field: `modifierType` (EnumChoice)
    #[serde(default)]
    pub modifier_type: String,
    /// DCB field: `signatureValue` (Single)
    #[serde(default)]
    pub signature_value: f32,
}

impl Pooled for RadiationStatePropertyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radiation_state_property_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radiation_state_property_params }
}

impl<'a> Extract<'a> for RadiationStatePropertyParams {
    const TYPE_NAME: &'static str = "RadiationStatePropertyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_type: inst.get_str("modifierType").map(String::from).unwrap_or_default(),
            signature_value: inst.get_f32("signatureValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadiationStateTemplateInternal`
///
/// Inherits from: `RadiationState` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationStateTemplateInternal {
    /// DCB field: `distortionMod` (EnumChoice)
    #[serde(default)]
    pub distortion_mod: String,
    /// DCB field: `distortion` (Single)
    #[serde(default)]
    pub distortion: f32,
    /// DCB field: `IR` (Class)
    #[serde(default)]
    pub ir: Option<Handle<RadiationStatePropertyParams>>,
    /// DCB field: `EM` (Class)
    #[serde(default)]
    pub em: Option<Handle<RadiationStatePropertyParams>>,
    /// DCB field: `CS` (Class)
    #[serde(default)]
    pub cs: Option<Handle<RadiationStatePropertyParams>>,
    /// DCB field: `hazardousRadiationMod` (EnumChoice)
    #[serde(default)]
    pub hazardous_radiation_mod: String,
    /// DCB field: `hazardousRadiationRate` (Single)
    #[serde(default)]
    pub hazardous_radiation_rate: f32,
}

impl Pooled for RadiationStateTemplateInternal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radiation_state_template_internal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radiation_state_template_internal }
}

impl<'a> Extract<'a> for RadiationStateTemplateInternal {
    const TYPE_NAME: &'static str = "RadiationStateTemplateInternal";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion_mod: inst.get_str("distortionMod").map(String::from).unwrap_or_default(),
            distortion: inst.get_f32("distortion").unwrap_or_default(),
            ir: match inst.get("IR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadiationStatePropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            em: match inst.get("EM") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadiationStatePropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cs: match inst.get("CS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadiationStatePropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hazardous_radiation_mod: inst.get_str("hazardousRadiationMod").map(String::from).unwrap_or_default(),
            hazardous_radiation_rate: inst.get_f32("hazardousRadiationRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadiationStateTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationStateTemplate {
    /// DCB field: `state` (Class)
    #[serde(default)]
    pub state: Option<Handle<RadiationStateTemplateInternal>>,
}

impl Pooled for RadiationStateTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radiation_state_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radiation_state_template }
}

impl<'a> Extract<'a> for RadiationStateTemplate {
    const TYPE_NAME: &'static str = "RadiationStateTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStateTemplateInternal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadiationStateTemplateInternal>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadiationBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationBehavior {
    /// DCB field: `surfaceRadiation` (StrongPointer)
    #[serde(default)]
    pub surface_radiation: Option<Handle<RadiationBehavior_SurfaceRadiationParams>>,
}

impl Pooled for RadiationBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radiation_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radiation_behavior }
}

impl<'a> Extract<'a> for RadiationBehavior {
    const TYPE_NAME: &'static str = "RadiationBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_radiation: match inst.get("surfaceRadiation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationBehavior_SurfaceRadiationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RadiationBehavior_SurfaceRadiationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadiationBehavior_SurfaceRadiationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationBehavior_SurfaceRadiationParams {
    /// DCB field: `scaleOnLargestAsteroid` (Single)
    #[serde(default)]
    pub scale_on_largest_asteroid: f32,
}

impl Pooled for RadiationBehavior_SurfaceRadiationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radiation_behavior_surface_radiation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radiation_behavior_surface_radiation_params }
}

impl<'a> Extract<'a> for RadiationBehavior_SurfaceRadiationParams {
    const TYPE_NAME: &'static str = "RadiationBehavior_SurfaceRadiationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scale_on_largest_asteroid: inst.get_f32("scaleOnLargestAsteroid").unwrap_or_default(),
        }
    }
}

/// DCB type: `RadarDisplay3DPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDisplay3DPreset {
    /// DCB field: `displayParams` (Class)
    #[serde(default)]
    pub display_params: Option<Handle<UIDataBankDisplay3DParams>>,
    /// DCB field: `quantumPathDisplaySettings` (Class)
    #[serde(default)]
    pub quantum_path_display_settings: Option<Handle<UIWorldDisplayPathParams>>,
}

impl Pooled for RadarDisplay3DPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ra.radar_display3_dpreset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ra.radar_display3_dpreset }
}

impl<'a> Extract<'a> for RadarDisplay3DPreset {
    const TYPE_NAME: &'static str = "RadarDisplay3DPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_params: match inst.get("displayParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quantum_path_display_settings: match inst.get("quantumPathDisplaySettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

