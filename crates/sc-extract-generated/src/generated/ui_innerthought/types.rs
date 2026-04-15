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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `InnerThought_CycleAnimBase`
pub struct InnerThought_CycleAnimBase {
    /// `length` (Single)
    pub length: f32,
    /// `amount` (Single)
    pub amount: f32,
    /// `stagger` (Single)
    pub stagger: f32,
}

impl Pooled for InnerThought_CycleAnimBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_cycle_anim_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_cycle_anim_base }
}

impl<'a> Extract<'a> for InnerThought_CycleAnimBase {
    const TYPE_NAME: &'static str = "InnerThought_CycleAnimBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            length: inst.get_f32("length").unwrap_or_default(),
            amount: inst.get_f32("amount").unwrap_or_default(),
            stagger: inst.get_f32("stagger").unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_CycleAnimRotateX`
/// Inherits from: `InnerThought_CycleAnimBase`
pub struct InnerThought_CycleAnimRotateX {
    /// `length` (Single)
    pub length: f32,
    /// `amount` (Single)
    pub amount: f32,
    /// `stagger` (Single)
    pub stagger: f32,
}

impl Pooled for InnerThought_CycleAnimRotateX {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_cycle_anim_rotate_x }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_cycle_anim_rotate_x }
}

impl<'a> Extract<'a> for InnerThought_CycleAnimRotateX {
    const TYPE_NAME: &'static str = "InnerThought_CycleAnimRotateX";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            length: inst.get_f32("length").unwrap_or_default(),
            amount: inst.get_f32("amount").unwrap_or_default(),
            stagger: inst.get_f32("stagger").unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_CycleAnimRotateY`
/// Inherits from: `InnerThought_CycleAnimBase`
pub struct InnerThought_CycleAnimRotateY {
    /// `length` (Single)
    pub length: f32,
    /// `amount` (Single)
    pub amount: f32,
    /// `stagger` (Single)
    pub stagger: f32,
}

impl Pooled for InnerThought_CycleAnimRotateY {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_cycle_anim_rotate_y }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_cycle_anim_rotate_y }
}

impl<'a> Extract<'a> for InnerThought_CycleAnimRotateY {
    const TYPE_NAME: &'static str = "InnerThought_CycleAnimRotateY";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            length: inst.get_f32("length").unwrap_or_default(),
            amount: inst.get_f32("amount").unwrap_or_default(),
            stagger: inst.get_f32("stagger").unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_CycleAnimRotateZ`
/// Inherits from: `InnerThought_CycleAnimBase`
pub struct InnerThought_CycleAnimRotateZ {
    /// `length` (Single)
    pub length: f32,
    /// `amount` (Single)
    pub amount: f32,
    /// `stagger` (Single)
    pub stagger: f32,
}

impl Pooled for InnerThought_CycleAnimRotateZ {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_cycle_anim_rotate_z }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_cycle_anim_rotate_z }
}

impl<'a> Extract<'a> for InnerThought_CycleAnimRotateZ {
    const TYPE_NAME: &'static str = "InnerThought_CycleAnimRotateZ";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            length: inst.get_f32("length").unwrap_or_default(),
            amount: inst.get_f32("amount").unwrap_or_default(),
            stagger: inst.get_f32("stagger").unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_LayoutGridSetThought`
pub struct InnerThought_LayoutGridSetThought {
    /// `justification` (EnumChoice)
    pub justification: InnerThoughtJustification,
    /// `offset` (Class)
    pub offset: Option<Handle<Vec3>>,
    /// `angle` (Class)
    pub angle: Option<Handle<Quat>>,
}

impl Pooled for InnerThought_LayoutGridSetThought {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_grid_set_thought }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_grid_set_thought }
}

impl<'a> Extract<'a> for InnerThought_LayoutGridSetThought {
    const TYPE_NAME: &'static str = "InnerThought_LayoutGridSetThought";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            justification: InnerThoughtJustification::from_dcb_str(inst.get_str("justification").unwrap_or("")),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            angle: match inst.get("angle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Quat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InnerThought_LayoutGridSet`
pub struct InnerThought_LayoutGridSet {
    /// `thoughts` (Class (array))
    pub thoughts: Vec<Handle<InnerThought_LayoutGridSetThought>>,
}

impl Pooled for InnerThought_LayoutGridSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_grid_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_grid_set }
}

impl<'a> Extract<'a> for InnerThought_LayoutGridSet {
    const TYPE_NAME: &'static str = "InnerThought_LayoutGridSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            thoughts: inst.get_array("thoughts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InnerThought_LayoutGridSetThought>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<InnerThought_LayoutGridSetThought>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_LayoutGrid`
/// Inherits from: `InnerThought_LayoutChoiceBase`
pub struct InnerThought_LayoutGrid {
    /// `selectedColor` (Reference)
    pub selected_color: Option<CigGuid>,
    /// `unselectedColor` (Reference)
    pub unselected_color: Option<CigGuid>,
    /// `secondaryColor` (Reference)
    pub secondary_color: Option<CigGuid>,
    /// `selectedOffset` (Class)
    pub selected_offset: Option<Handle<Vec3>>,
    /// `unselectedOffset` (Class)
    pub unselected_offset: Option<Handle<Vec3>>,
    /// `secondaryOffset` (Class)
    pub secondary_offset: Option<Handle<Vec3>>,
    /// `primarySets` (Class (array))
    pub primary_sets: Vec<Handle<InnerThought_LayoutGridSet>>,
    /// `secondarySets` (Class (array))
    pub secondary_sets: Vec<Handle<InnerThought_LayoutGridSet>>,
}

impl Pooled for InnerThought_LayoutGrid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_grid }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_grid }
}

impl<'a> Extract<'a> for InnerThought_LayoutGrid {
    const TYPE_NAME: &'static str = "InnerThought_LayoutGrid";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            selected_color: inst.get("selectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unselected_color: inst.get("unselectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            secondary_color: inst.get("secondaryColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            selected_offset: match inst.get("selectedOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            unselected_offset: match inst.get("unselectedOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            secondary_offset: match inst.get("secondaryOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            primary_sets: inst.get_array("primarySets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InnerThought_LayoutGridSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<InnerThought_LayoutGridSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            secondary_sets: inst.get_array("secondarySets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InnerThought_LayoutGridSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<InnerThought_LayoutGridSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InnerThought_LayoutCurve`
/// Inherits from: `InnerThought_LayoutBase`
pub struct InnerThought_LayoutCurve {
    /// `shuffleSelectedToBottom` (Boolean)
    pub shuffle_selected_to_bottom: bool,
    /// `radius` (Single)
    pub radius: f32,
    /// `angle` (Single)
    pub angle: f32,
    /// `radiusOrientation` (Class)
    pub radius_orientation: Option<Handle<Quat>>,
    /// `cycles` (StrongPointer (array))
    pub cycles: Vec<InnerThought_CycleAnimBasePtr>,
    /// `selectedColor` (Reference)
    pub selected_color: Option<CigGuid>,
    /// `unselectedColorStart` (Reference)
    pub unselected_color_start: Option<CigGuid>,
    /// `unselectedColorEnd` (Reference)
    pub unselected_color_end: Option<CigGuid>,
    /// `selectedOffset` (Class)
    pub selected_offset: Option<Handle<Vec3>>,
    /// `unselectedOffset` (Class)
    pub unselected_offset: Option<Handle<Vec3>>,
    /// `selectedRotation` (Class)
    pub selected_rotation: Option<Handle<Deg3>>,
    /// `unselectedRotation` (Class)
    pub unselected_rotation: Option<Handle<Deg3>>,
}

impl Pooled for InnerThought_LayoutCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_curve }
}

impl<'a> Extract<'a> for InnerThought_LayoutCurve {
    const TYPE_NAME: &'static str = "InnerThought_LayoutCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shuffle_selected_to_bottom: inst.get_bool("shuffleSelectedToBottom").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            angle: inst.get_f32("angle").unwrap_or_default(),
            radius_orientation: match inst.get("radiusOrientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Quat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cycles: inst.get_array("cycles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(InnerThought_CycleAnimBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            selected_color: inst.get("selectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unselected_color_start: inst.get("unselectedColorStart").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unselected_color_end: inst.get("unselectedColorEnd").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            selected_offset: match inst.get("selectedOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            unselected_offset: match inst.get("unselectedOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            selected_rotation: match inst.get("selectedRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            unselected_rotation: match inst.get("unselectedRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InnerThought_LayoutPIT`
/// Inherits from: `InnerThought_LayoutBase`
pub struct InnerThought_LayoutPIT {
    /// `selectedColor` (Reference)
    pub selected_color: Option<CigGuid>,
    /// `unselectedColor` (Reference)
    pub unselected_color: Option<CigGuid>,
    /// `inactiveColor` (Reference)
    pub inactive_color: Option<CigGuid>,
}

impl Pooled for InnerThought_LayoutPIT {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_innerthought.inner_thought_layout_pit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_innerthought.inner_thought_layout_pit }
}

impl<'a> Extract<'a> for InnerThought_LayoutPIT {
    const TYPE_NAME: &'static str = "InnerThought_LayoutPIT";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            selected_color: inst.get("selectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            unselected_color: inst.get("unselectedColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            inactive_color: inst.get("inactiveColor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InnerThought_LegacyUseSystemConfig`
pub struct InnerThought_LegacyUseSystemConfig {
    /// `targetDistance` (Single)
    pub target_distance: f32,
    /// `minDistance` (Single)
    pub min_distance: f32,
    /// `maxDistance` (Single)
    pub max_distance: f32,
    /// `rotationRate` (Single)
    pub rotation_rate: f32,
    /// `translationRate` (Single)
    pub translation_rate: f32,
    /// `innerThought` (Reference)
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

