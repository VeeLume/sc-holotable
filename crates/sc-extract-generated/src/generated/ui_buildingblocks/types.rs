// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-buildingblocks`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `BindingsOperations_WaveformShapeTriangle`
/// Inherits from: `BindingsOperations_WaveformShapeBase`
pub struct BindingsOperations_WaveformShapeTriangle {}

impl Pooled for BindingsOperations_WaveformShapeTriangle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_triangle
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_triangle
    }
}

impl<'a> Extract<'a> for BindingsOperations_WaveformShapeTriangle {
    const TYPE_NAME: &'static str = "BindingsOperations_WaveformShapeTriangle";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `BindingsOperations_WaveformShapeSquare`
/// Inherits from: `BindingsOperations_WaveformShapeBase`
pub struct BindingsOperations_WaveformShapeSquare {
    /// `interval` (Single)
    pub interval: f32,
}

impl Pooled for BindingsOperations_WaveformShapeSquare {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_square
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_square
    }
}

impl<'a> Extract<'a> for BindingsOperations_WaveformShapeSquare {
    const TYPE_NAME: &'static str = "BindingsOperations_WaveformShapeSquare";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interval: inst.get_f32("interval").unwrap_or_default(),
        }
    }
}

/// DCB type: `BindingsOperations_WaveformShapeSawtooth`
/// Inherits from: `BindingsOperations_WaveformShapeBase`
pub struct BindingsOperations_WaveformShapeSawtooth {
    /// `isReversed` (Boolean)
    pub is_reversed: bool,
}

impl Pooled for BindingsOperations_WaveformShapeSawtooth {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_sawtooth
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .bindings_operations_waveform_shape_sawtooth
    }
}

impl<'a> Extract<'a> for BindingsOperations_WaveformShapeSawtooth {
    const TYPE_NAME: &'static str = "BindingsOperations_WaveformShapeSawtooth";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_reversed: inst.get_bool("isReversed").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsColorFromNumberRGBA`
/// Inherits from: `BuildingBlocks_BindingsColorBase`
pub struct BuildingBlocks_BindingsColorFromNumberRGBA {
    /// `inputRed` (WeakPointer)
    pub input_red: Option<BuildingBlocks_BindingsNumberBasePtr>,
    /// `inputGreen` (WeakPointer)
    pub input_green: Option<BuildingBlocks_BindingsNumberBasePtr>,
    /// `inputBlue` (WeakPointer)
    pub input_blue: Option<BuildingBlocks_BindingsNumberBasePtr>,
    /// `inputAlpha` (WeakPointer)
    pub input_alpha: Option<BuildingBlocks_BindingsNumberBasePtr>,
    /// `defaultRedValue` (Single)
    pub default_red_value: f32,
    /// `defaultGreenValue` (Single)
    pub default_green_value: f32,
    /// `defaultBlueValue` (Single)
    pub default_blue_value: f32,
    /// `defaultAlphaValue` (Single)
    pub default_alpha_value: f32,
}

impl Pooled for BuildingBlocks_BindingsColorFromNumberRGBA {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_bindings_color_from_number_rgba
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_bindings_color_from_number_rgba
    }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsColorFromNumberRGBA {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsColorFromNumberRGBA";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_red: match inst.get("inputRed") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_BindingsNumberBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            input_green: match inst.get("inputGreen") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_BindingsNumberBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            input_blue: match inst.get("inputBlue") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_BindingsNumberBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            input_alpha: match inst.get("inputAlpha") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_BindingsNumberBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            default_red_value: inst.get_f32("defaultRedValue").unwrap_or_default(),
            default_green_value: inst.get_f32("defaultGreenValue").unwrap_or_default(),
            default_blue_value: inst.get_f32("defaultBlueValue").unwrap_or_default(),
            default_alpha_value: inst.get_f32("defaultAlphaValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior`
/// Inherits from: `BuildingBlocks_FieldModifierEnumeratedTypeBase`
pub struct BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior {
    /// `value` (EnumChoice)
    pub value: BB_SizeBehavior,
}

impl Pooled for BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_field_modifier_enumerated_type_max_height_behavior
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_field_modifier_enumerated_type_max_height_behavior
    }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: BB_SizeBehavior::from_dcb_str(inst.get_str("value").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment`
/// Inherits from: `BuildingBlocks_FieldModifierEnumeratedTypeBase`
pub struct BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment {
    /// `value` (EnumChoice)
    pub value: BB_StrokeAlignment,
}

impl Pooled for BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_field_modifier_enumerated_type_stroke_alignment
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_field_modifier_enumerated_type_stroke_alignment
    }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: BB_StrokeAlignment::from_dcb_str(inst.get_str("value").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_TimingFunctionCustomCurve`
/// Inherits from: `BuildingBlocks_TimingFunctionBase`
pub struct BuildingBlocks_TimingFunctionCustomCurve {
    /// `curve` (Class)
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for BuildingBlocks_TimingFunctionCustomCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_timing_function_custom_curve
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_timing_function_custom_curve
    }
}

impl<'a> Extract<'a> for BuildingBlocks_TimingFunctionCustomCurve {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimingFunctionCustomCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_IntegerWidgetPairDef`
pub struct BuildingBlocks_IntegerWidgetPairDef {
    /// `first` (Int64)
    pub first: i64,
    /// `second` (WeakPointer)
    pub second: Option<BuildingBlocks_WidgetBasePtr>,
}

impl Pooled for BuildingBlocks_IntegerWidgetPairDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_integer_widget_pair_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_integer_widget_pair_def
    }
}

impl<'a> Extract<'a> for BuildingBlocks_IntegerWidgetPairDef {
    const TYPE_NAME: &'static str = "BuildingBlocks_IntegerWidgetPairDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            first: inst.get_i64("first").unwrap_or_default(),
            second: match inst.get("second") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetPolymorphic`
/// Inherits from: `BuildingBlocks_DisplayWidget`
pub struct BuildingBlocks_WidgetPolymorphic {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `variableName` (String)
    pub variable_name: String,
    /// `targetList` (Class (array))
    pub target_list: Vec<Handle<BuildingBlocks_IntegerWidgetPairDef>>,
}

impl Pooled for BuildingBlocks_WidgetPolymorphic {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_widget_polymorphic
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_widget_polymorphic
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetPolymorphic {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetPolymorphic";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            target_list: inst
                .get_array("targetList")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_IntegerWidgetPairDef>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<BuildingBlocks_IntegerWidgetPairDef>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetParticleEffect`
/// Inherits from: `BuildingBlocks_WidgetBase`
pub struct BuildingBlocks_WidgetParticleEffect {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `effect` (Class)
    pub effect: Option<Handle<GlobalResourceParticle>>,
}

impl Pooled for BuildingBlocks_WidgetParticleEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_widget_particle_effect
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_widget_particle_effect
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetParticleEffect {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetParticleEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_LookAtTransformer`
/// Inherits from: `BuildingBlocks_TransformerBase`
pub struct BuildingBlocks_LookAtTransformer {
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
}

impl Pooled for BuildingBlocks_LookAtTransformer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_look_at_transformer
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_look_at_transformer
    }
}

impl<'a> Extract<'a> for BuildingBlocks_LookAtTransformer {
    const TYPE_NAME: &'static str = "BuildingBlocks_LookAtTransformer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetEnvironmentProbe`
/// Inherits from: `BuildingBlocks_WidgetBase`
pub struct BuildingBlocks_WidgetEnvironmentProbe {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `probeRadius` (Single)
    pub probe_radius: f32,
    /// `probeIntensity` (Single)
    pub probe_intensity: f32,
    /// `probeColor` (StrongPointer)
    pub probe_color: Option<BuildingBlocks_ColorBasePtr>,
    /// `specularImage` (String)
    pub specular_image: String,
}

impl Pooled for BuildingBlocks_WidgetEnvironmentProbe {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_widget_environment_probe
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_widget_environment_probe
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetEnvironmentProbe {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetEnvironmentProbe";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            probe_radius: inst.get_f32("probeRadius").unwrap_or_default(),
            probe_intensity: inst.get_f32("probeIntensity").unwrap_or_default(),
            probe_color: match inst.get("probeColor") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ColorBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            specular_image: inst
                .get_str("specularImage")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GrabCameraControlParams`
pub struct GrabCameraControlParams {
    /// `responsiveness` (Single)
    pub responsiveness: f32,
    /// `rotationSpeed` (Single)
    pub rotation_speed: f32,
    /// `rotationSlowdown` (Single)
    pub rotation_slowdown: f32,
    /// `zoomSpeed` (Single)
    pub zoom_speed: f32,
    /// `zoomSlowdown` (Single)
    pub zoom_slowdown: f32,
    /// `minimumZoomDistance` (Single)
    pub minimum_zoom_distance: f32,
    /// `maximumZoomDistance` (Single)
    pub maximum_zoom_distance: f32,
    /// `maximumZoomSpeed` (Single)
    pub maximum_zoom_speed: f32,
    /// `isGrabbableOutOfBounds` (Boolean)
    pub is_grabbable_out_of_bounds: bool,
    /// `grabRotationMode` (EnumChoice)
    pub grab_rotation_mode: BB_GrabRotationMode,
    /// `panResponsiveness` (Single)
    pub pan_responsiveness: f32,
    /// `panSpeed` (Single)
    pub pan_speed: f32,
    /// `panSlowdown` (Single)
    pub pan_slowdown: f32,
}

impl Pooled for GrabCameraControlParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.grab_camera_control_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.grab_camera_control_params
    }
}

impl<'a> Extract<'a> for GrabCameraControlParams {
    const TYPE_NAME: &'static str = "GrabCameraControlParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responsiveness: inst.get_f32("responsiveness").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            rotation_slowdown: inst.get_f32("rotationSlowdown").unwrap_or_default(),
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            zoom_slowdown: inst.get_f32("zoomSlowdown").unwrap_or_default(),
            minimum_zoom_distance: inst.get_f32("minimumZoomDistance").unwrap_or_default(),
            maximum_zoom_distance: inst.get_f32("maximumZoomDistance").unwrap_or_default(),
            maximum_zoom_speed: inst.get_f32("maximumZoomSpeed").unwrap_or_default(),
            is_grabbable_out_of_bounds: inst.get_bool("isGrabbableOutOfBounds").unwrap_or_default(),
            grab_rotation_mode: BB_GrabRotationMode::from_dcb_str(
                inst.get_str("grabRotationMode").unwrap_or(""),
            ),
            pan_responsiveness: inst.get_f32("panResponsiveness").unwrap_or_default(),
            pan_speed: inst.get_f32("panSpeed").unwrap_or_default(),
            pan_slowdown: inst.get_f32("panSlowdown").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TargetSlicer`
/// Inherits from: `BuildingBlocks_SlicerBase`
pub struct BuildingBlocks_TargetSlicer {
    /// `target` (WeakPointer)
    pub target: Option<BuildingBlocks_WidgetBasePtr>,
    /// `coordinateMethod` (EnumChoice)
    pub coordinate_method: BB_CanvasCoordinateMethod,
}

impl Pooled for BuildingBlocks_TargetSlicer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_target_slicer
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_target_slicer
    }
}

impl<'a> Extract<'a> for BuildingBlocks_TargetSlicer {
    const TYPE_NAME: &'static str = "BuildingBlocks_TargetSlicer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target: match inst.get("target") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            coordinate_method: BB_CanvasCoordinateMethod::from_dcb_str(
                inst.get_str("coordinateMethod").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetPagination`
/// Inherits from: `BuildingBlocks_WidgetCanvas`
pub struct BuildingBlocks_WidgetPagination {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `showFirstPageButton` (Boolean)
    pub show_first_page_button: bool,
    /// `showPreviousPageButton` (Boolean)
    pub show_previous_page_button: bool,
    /// `showNextPageButton` (Boolean)
    pub show_next_page_button: bool,
    /// `showLastPageButton` (Boolean)
    pub show_last_page_button: bool,
    /// `pageNumberURL` (String)
    pub page_number_url: String,
    /// `numPagesURL` (String)
    pub num_pages_url: String,
}

impl Pooled for BuildingBlocks_WidgetPagination {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_widget_pagination
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_widget_pagination
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetPagination {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetPagination";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            show_first_page_button: inst.get_bool("showFirstPageButton").unwrap_or_default(),
            show_previous_page_button: inst.get_bool("showPreviousPageButton").unwrap_or_default(),
            show_next_page_button: inst.get_bool("showNextPageButton").unwrap_or_default(),
            show_last_page_button: inst.get_bool("showLastPageButton").unwrap_or_default(),
            page_number_url: inst
                .get_str("pageNumberURL")
                .map(String::from)
                .unwrap_or_default(),
            num_pages_url: inst
                .get_str("numPagesURL")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetRadioControl`
/// Inherits from: `BuildingBlocks_TogglerBase`
pub struct BuildingBlocks_WidgetRadioControl {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `variableName` (String)
    pub variable_name: String,
    /// `toggleActivationEvent` (EnumChoice)
    pub toggle_activation_event: BB_ActivationButtonAction,
}

impl Pooled for BuildingBlocks_WidgetRadioControl {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_widget_radio_control
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_widget_radio_control
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetRadioControl {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetRadioControl";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            toggle_activation_event: BB_ActivationButtonAction::from_dcb_str(
                inst.get_str("toggleActivationEvent").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `BuildingBlocks_WidgetBadge`
/// Inherits from: `BuildingBlocks_WidgetCanvas`
pub struct BuildingBlocks_WidgetBadge {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `labelProperties` (Class)
    pub label_properties: Option<Handle<BuildingBlocks_ComponentLabelProperties>>,
    /// `fillStyle` (EnumChoice)
    pub fill_style: BB_FillStyle,
}

impl Pooled for BuildingBlocks_WidgetBadge {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_widget_badge
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_widget_badge
    }
}

impl<'a> Extract<'a> for BuildingBlocks_WidgetBadge {
    const TYPE_NAME: &'static str = "BuildingBlocks_WidgetBadge";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            label_properties: match inst.get("labelProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentLabelProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            fill_style: BB_FillStyle::from_dcb_str(inst.get_str("fillStyle").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_ComponentRadioButton`
/// Inherits from: `BuildingBlocks_TogglerBase`
pub struct BuildingBlocks_ComponentRadioButton {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `variableName` (String)
    pub variable_name: String,
    /// `toggleActivationEvent` (EnumChoice)
    pub toggle_activation_event: BB_ActivationButtonAction,
    /// `labelProperties` (Class)
    pub label_properties: Option<Handle<BuildingBlocks_ComponentLabelProperties>>,
    /// `captionProperties` (Class)
    pub caption_properties: Option<Handle<BuildingBlocks_ComponentCaptionProperties>>,
    /// `alignment` (EnumChoice)
    pub alignment: BB_TextAlignment,
}

impl Pooled for BuildingBlocks_ComponentRadioButton {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_component_radio_button
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_component_radio_button
    }
}

impl<'a> Extract<'a> for BuildingBlocks_ComponentRadioButton {
    const TYPE_NAME: &'static str = "BuildingBlocks_ComponentRadioButton";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            toggle_activation_event: BB_ActivationButtonAction::from_dcb_str(
                inst.get_str("toggleActivationEvent").unwrap_or(""),
            ),
            label_properties: match inst.get("labelProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentLabelProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            caption_properties: match inst.get("captionProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentCaptionProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            alignment: BB_TextAlignment::from_dcb_str(inst.get_str("alignment").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_ComponentToggleListItem`
/// Inherits from: `BuildingBlocks_TogglerBase`
pub struct BuildingBlocks_ComponentToggleListItem {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `variableName` (String)
    pub variable_name: String,
    /// `toggleActivationEvent` (EnumChoice)
    pub toggle_activation_event: BB_ActivationButtonAction,
    /// `labelProperties` (Class)
    pub label_properties: Option<Handle<BuildingBlocks_ComponentLabelProperties>>,
    /// `captionProperties` (Class)
    pub caption_properties: Option<Handle<BuildingBlocks_ComponentCaptionProperties>>,
    /// `iconProperties` (Class)
    pub icon_properties: Option<Handle<BuildingBlocks_ComponentIconProperties>>,
    /// `alignment` (EnumChoice)
    pub alignment: BB_TextAlignment,
}

impl Pooled for BuildingBlocks_ComponentToggleListItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_component_toggle_list_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_component_toggle_list_item
    }
}

impl<'a> Extract<'a> for BuildingBlocks_ComponentToggleListItem {
    const TYPE_NAME: &'static str = "BuildingBlocks_ComponentToggleListItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            toggle_activation_event: BB_ActivationButtonAction::from_dcb_str(
                inst.get_str("toggleActivationEvent").unwrap_or(""),
            ),
            label_properties: match inst.get("labelProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentLabelProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            caption_properties: match inst.get("captionProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentCaptionProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            icon_properties: match inst.get("iconProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentIconProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            alignment: BB_TextAlignment::from_dcb_str(inst.get_str("alignment").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_ComponentRadialRangeSlider`
/// Inherits from: `BuildingBlocks_WidgetCanvas`
pub struct BuildingBlocks_ComponentRadialRangeSlider {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `targetVariableName` (String)
    pub target_variable_name: String,
    /// `currentVariableName` (String)
    pub current_variable_name: String,
    /// `labelProperties` (Class)
    pub label_properties: Option<Handle<BuildingBlocks_ComponentLabelProperties>>,
    /// `captionProperties` (Class)
    pub caption_properties: Option<Handle<BuildingBlocks_ComponentCaptionProperties>>,
    /// `limiterAngle` (Single)
    pub limiter_angle: f32,
    /// `startAngleOffset` (Single)
    pub start_angle_offset: f32,
    /// `meterState` (EnumChoice)
    pub meter_state: BB_ProgressMeterState,
    /// `incrementProportion` (UInt16)
    pub increment_proportion: u32,
}

impl Pooled for BuildingBlocks_ComponentRadialRangeSlider {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_component_radial_range_slider
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_component_radial_range_slider
    }
}

impl<'a> Extract<'a> for BuildingBlocks_ComponentRadialRangeSlider {
    const TYPE_NAME: &'static str = "BuildingBlocks_ComponentRadialRangeSlider";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            target_variable_name: inst
                .get_str("targetVariableName")
                .map(String::from)
                .unwrap_or_default(),
            current_variable_name: inst
                .get_str("currentVariableName")
                .map(String::from)
                .unwrap_or_default(),
            label_properties: match inst.get("labelProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentLabelProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            caption_properties: match inst.get("captionProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentCaptionProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            limiter_angle: inst.get_f32("limiterAngle").unwrap_or_default(),
            start_angle_offset: inst.get_f32("startAngleOffset").unwrap_or_default(),
            meter_state: BB_ProgressMeterState::from_dcb_str(
                inst.get_str("meterState").unwrap_or(""),
            ),
            increment_proportion: inst.get_u32("incrementProportion").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ComponentNotification`
/// Inherits from: `BuildingBlocks_TogglerBase`
pub struct BuildingBlocks_ComponentNotification {
    /// `name` (String)
    pub name: String,
    /// `styleTags` (Reference (array))
    pub style_tags: Vec<CigGuid>,
    /// `rendererType` (EnumChoice)
    pub renderer_type: BB_RendererType,
    /// `rendererPolicy` (StrongPointer)
    pub renderer_policy: Option<BuildingBlocks_RendererPolicyBasePtr>,
    /// `primitiveSettings` (Class)
    pub primitive_settings: Option<Handle<BuildingBlocks_PrimitiveSettings>>,
    /// `parent` (WeakPointer)
    pub parent: Option<BuildingBlocks_WidgetBasePtr>,
    /// `previewScene` (WeakPointer)
    pub preview_scene: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `previewSceneFlattened` (WeakPointer)
    pub preview_scene_flattened: Option<BuildingBlocks_PreviewScreenBasePtr>,
    /// `cullingLevel` (EnumChoice)
    pub culling_level: BB_CullingLevel,
    /// `isActive` (Boolean)
    pub is_active: bool,
    /// `affectsLayout` (Boolean)
    pub affects_layout: bool,
    /// `affectsAutosize` (Boolean)
    pub affects_autosize: bool,
    /// `exportNode` (Boolean)
    pub export_node: bool,
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `positionOffset` (Class)
    pub position_offset: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `orientationOffset` (Class)
    pub orientation_offset: Option<Handle<Deg3>>,
    /// `scale` (Class)
    pub scale: Option<Handle<Vec3>>,
    /// `sizing` (Class)
    pub sizing: Option<Handle<BuildingBlocks_Size>>,
    /// `autoScalingMethod` (EnumChoice)
    pub auto_scaling_method: BB_AutoScalingMethod,
    /// `padding` (Class)
    pub padding: Option<Handle<BuildingBlocks_TRBL>>,
    /// `margin` (Class)
    pub margin: Option<Handle<BuildingBlocks_TRBL>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `background` (Class)
    pub background: Option<Handle<BuildingBlocks_Background>>,
    /// `segmentedFill` (Class)
    pub segmented_fill: Option<Handle<BuildingBlocks_SegmentedFill>>,
    /// `svgFill` (Class)
    pub svg_fill: Option<Handle<BuildingBlocks_SvgFill>>,
    /// `border` (Class)
    pub border: Option<Handle<BuildingBlocks_Border>>,
    /// `layoutPolicy` (StrongPointer)
    pub layout_policy: Option<BuildingBlocks_LayoutPolicyBasePtr>,
    /// `layoutPolicyItem` (StrongPointer)
    pub layout_policy_item: Option<BuildingBlocks_LayoutPolicyItemBasePtr>,
    /// `layoutItemCommon` (StrongPointer)
    pub layout_item_common: Option<Handle<BuildingBlocks_LayoutItemCommon>>,
    /// `dropTargetPolicy` (StrongPointer)
    pub drop_target_policy: Option<BuildingBlocks_DropTargetPolicyBasePtr>,
    /// `draggablePolicy` (StrongPointer)
    pub draggable_policy: Option<BuildingBlocks_DraggablePolicyBasePtr>,
    /// `tooltipPolicy` (StrongPointer)
    pub tooltip_policy: Option<Handle<BuildingBlocks_TooltipPolicy>>,
    /// `contextMenuPolicy` (StrongPointer)
    pub context_menu_policy: Option<Handle<BuildingBlocks_ContextMenuPolicy>>,
    /// `grabControlsPolicy` (StrongPointer)
    pub grab_controls_policy: Option<Handle<BuildingBlocks_GrabControlsPolicy>>,
    /// `calloutSettings` (StrongPointer)
    pub callout_settings: Option<Handle<BuildingBlocks_CalloutSettings>>,
    /// `virtualCursorPolicy` (StrongPointer)
    pub virtual_cursor_policy: Option<Handle<BuildingBlocks_VirtualCursorPolicy>>,
    /// `overflow` (Class)
    pub overflow: Option<Handle<BuildingBlocks_Overflow>>,
    /// `scrollPolicy` (StrongPointer)
    pub scroll_policy: Option<BuildingBlocks_ScrollPolicyBasePtr>,
    /// `radialTransform` (Class)
    pub radial_transform: Option<Handle<BuildingBlocks_RadialTransform>>,
    /// `radialTransformChild` (Class)
    pub radial_transform_child: Option<Handle<BuildingBlocks_RadialTransformChild>>,
    /// `animation` (Class)
    pub animation: Option<Handle<BuildingBlocks_Animation>>,
    /// `interactions` (Class)
    pub interactions: Option<Handle<BuildingBlocks_Interactions>>,
    /// `inheritsScale` (Boolean)
    pub inherits_scale: bool,
    /// `inheritsRotation` (Boolean)
    pub inherits_rotation: bool,
    /// `inheritsTranslation` (Boolean)
    pub inherits_translation: bool,
    /// `inheritsAlpha` (Boolean)
    pub inherits_alpha: bool,
    /// `inheritsOverflow` (Boolean)
    pub inherits_overflow: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `layer` (Byte)
    pub layer: u32,
    /// `aspectRatioLibrary` (Reference)
    pub aspect_ratio_library: Option<CigGuid>,
    /// `focusIndex` (Int16)
    pub focus_index: i32,
    /// `inlineStyles` (Class (array))
    pub inline_styles: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `hoverCursor` (EnumChoice)
    pub hover_cursor: Cursor,
    /// `enableHeldCursor` (Boolean)
    pub enable_held_cursor: bool,
    /// `heldCursor` (EnumChoice)
    pub held_cursor: Cursor,
    /// `instantiated` (Boolean)
    pub instantiated: bool,
    /// `urlOptional` (String)
    pub url_optional: String,
    /// `urlPostfix` (String)
    pub url_postfix: String,
    /// `stylesheetOverride` (Reference)
    pub stylesheet_override: Option<CigGuid>,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `sizingMethod` (EnumChoice)
    pub sizing_method: BB_CanvasWidgetSizingMethod,
    /// `paramInputValues` (StrongPointer (array))
    pub param_input_values: Vec<BuildingBlocks_ComponentParameterInputBasePtr>,
    /// `variableName` (String)
    pub variable_name: String,
    /// `toggleActivationEvent` (EnumChoice)
    pub toggle_activation_event: BB_ActivationButtonAction,
    /// `labelProperties` (Class)
    pub label_properties: Option<Handle<BuildingBlocks_ComponentLabelProperties>>,
    /// `captionProperties` (Class)
    pub caption_properties: Option<Handle<BuildingBlocks_ComponentCaptionProperties>>,
    /// `iconProperties` (Class)
    pub icon_properties: Option<Handle<BuildingBlocks_ComponentIconProperties>>,
    /// `alignment` (EnumChoice)
    pub alignment: BB_TextAlignment,
    /// `closeButton` (Boolean)
    pub close_button: bool,
}

impl Pooled for BuildingBlocks_ComponentNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_component_notification
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_component_notification
    }
}

impl<'a> Extract<'a> for BuildingBlocks_ComponentNotification {
    const TYPE_NAME: &'static str = "BuildingBlocks_ComponentNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            style_tags: inst
                .get_array("styleTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            renderer_type: BB_RendererType::from_dcb_str(
                inst.get_str("rendererType").unwrap_or(""),
            ),
            renderer_policy: match inst.get("rendererPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_RendererPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            primitive_settings: match inst.get("primitiveSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_PrimitiveSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            parent: match inst.get("parent") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_WidgetBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene: match inst.get("previewScene") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            preview_scene_flattened: match inst.get("previewSceneFlattened") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_PreviewScreenBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            culling_level: BB_CullingLevel::from_dcb_str(
                inst.get_str("cullingLevel").unwrap_or(""),
            ),
            is_active: inst.get_bool("isActive").unwrap_or_default(),
            affects_layout: inst.get_bool("affectsLayout").unwrap_or_default(),
            affects_autosize: inst.get_bool("affectsAutosize").unwrap_or_default(),
            export_node: inst.get_bool("exportNode").unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale: match inst.get("scale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            sizing: match inst.get("sizing") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Size>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_scaling_method: BB_AutoScalingMethod::from_dcb_str(
                inst.get_str("autoScalingMethod").unwrap_or(""),
            ),
            padding: match inst.get("padding") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            margin: match inst.get("margin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_TRBL>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            background: match inst.get("background") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Background>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            segmented_fill: match inst.get("segmentedFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SegmentedFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            svg_fill: match inst.get("svgFill") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_SvgFill>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            border: match inst.get("border") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Border>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            layout_policy: match inst.get("layoutPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_policy_item: match inst.get("layoutPolicyItem") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_LayoutPolicyItemBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            layout_item_common: match inst.get("layoutItemCommon") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_LayoutItemCommon>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_target_policy: match inst.get("dropTargetPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DropTargetPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            draggable_policy: match inst.get("draggablePolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_DraggablePolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            tooltip_policy: match inst.get("tooltipPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_TooltipPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            context_menu_policy: match inst.get("contextMenuPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_ContextMenuPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            grab_controls_policy: match inst.get("grabControlsPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_GrabControlsPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            callout_settings: match inst.get("calloutSettings") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_CalloutSettings>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            virtual_cursor_policy: match inst.get("virtualCursorPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<BuildingBlocks_VirtualCursorPolicy>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            overflow: match inst.get("overflow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Overflow>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            scroll_policy: match inst.get("scrollPolicy") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BuildingBlocks_ScrollPolicyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            radial_transform: match inst.get("radialTransform") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransform>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            radial_transform_child: match inst.get("radialTransformChild") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_RadialTransformChild>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            animation: match inst.get("animation") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Animation>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interactions: match inst.get("interactions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_Interactions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            inherits_scale: inst.get_bool("inheritsScale").unwrap_or_default(),
            inherits_rotation: inst.get_bool("inheritsRotation").unwrap_or_default(),
            inherits_translation: inst.get_bool("inheritsTranslation").unwrap_or_default(),
            inherits_alpha: inst.get_bool("inheritsAlpha").unwrap_or_default(),
            inherits_overflow: inst.get_bool("inheritsOverflow").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            layer: inst.get_u32("layer").unwrap_or_default(),
            aspect_ratio_library: inst
                .get("aspectRatioLibrary")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            focus_index: inst.get_i32("focusIndex").unwrap_or_default(),
            inline_styles: inst
                .get_array("inlineStyles")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hover_cursor: Cursor::from_dcb_str(inst.get_str("hoverCursor").unwrap_or("")),
            enable_held_cursor: inst.get_bool("enableHeldCursor").unwrap_or_default(),
            held_cursor: Cursor::from_dcb_str(inst.get_str("heldCursor").unwrap_or("")),
            instantiated: inst.get_bool("instantiated").unwrap_or_default(),
            url_optional: inst
                .get_str("urlOptional")
                .map(String::from)
                .unwrap_or_default(),
            url_postfix: inst
                .get_str("urlPostfix")
                .map(String::from)
                .unwrap_or_default(),
            stylesheet_override: inst
                .get("stylesheetOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            sizing_method: BB_CanvasWidgetSizingMethod::from_dcb_str(
                inst.get_str("sizingMethod").unwrap_or(""),
            ),
            param_input_values: inst
                .get_array("paramInputValues")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(
                            BuildingBlocks_ComponentParameterInputBasePtr::from_ref(b, r),
                        ),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            toggle_activation_event: BB_ActivationButtonAction::from_dcb_str(
                inst.get_str("toggleActivationEvent").unwrap_or(""),
            ),
            label_properties: match inst.get("labelProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentLabelProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            caption_properties: match inst.get("captionProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentCaptionProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            icon_properties: match inst.get("iconProperties") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<BuildingBlocks_ComponentIconProperties>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            alignment: BB_TextAlignment::from_dcb_str(inst.get_str("alignment").unwrap_or("")),
            close_button: inst.get_bool("closeButton").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FontReplacementPair`
pub struct BuildingBlocks_FontReplacementPair {
    /// `englishFont` (Reference)
    pub english_font: Option<CigGuid>,
    /// `replacementFontName` (String)
    pub replacement_font_name: String,
    /// `replacementFontPaintFile` (String)
    pub replacement_font_paint_file: String,
}

impl Pooled for BuildingBlocks_FontReplacementPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_font_replacement_pair
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_font_replacement_pair
    }
}

impl<'a> Extract<'a> for BuildingBlocks_FontReplacementPair {
    const TYPE_NAME: &'static str = "BuildingBlocks_FontReplacementPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            english_font: inst
                .get("englishFont")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            replacement_font_name: inst
                .get_str("replacementFontName")
                .map(String::from)
                .unwrap_or_default(),
            replacement_font_paint_file: inst
                .get_str("replacementFontPaintFile")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_LanguageSpecificFontReplacement`
pub struct BuildingBlocks_LanguageSpecificFontReplacement {
    /// `fontReplacementList` (Class (array))
    pub font_replacement_list: Vec<Handle<BuildingBlocks_FontReplacementPair>>,
}

impl Pooled for BuildingBlocks_LanguageSpecificFontReplacement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_language_specific_font_replacement
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_language_specific_font_replacement
    }
}

impl<'a> Extract<'a> for BuildingBlocks_LanguageSpecificFontReplacement {
    const TYPE_NAME: &'static str = "BuildingBlocks_LanguageSpecificFontReplacement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            font_replacement_list: inst
                .get_array("fontReplacementList")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TriggerNavigation`
/// Inherits from: `BuildingBlocks_TriggerBase`
pub struct BuildingBlocks_TriggerNavigation {
    /// `layerName` (String)
    pub layer_name: String,
    /// `type` (EnumChoice)
    pub r#type: BB_NavigationType,
}

impl Pooled for BuildingBlocks_TriggerNavigation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_trigger_navigation
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_trigger_navigation
    }
}

impl<'a> Extract<'a> for BuildingBlocks_TriggerNavigation {
    const TYPE_NAME: &'static str = "BuildingBlocks_TriggerNavigation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            layer_name: inst
                .get_str("layerName")
                .map(String::from)
                .unwrap_or_default(),
            r#type: BB_NavigationType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

/// DCB type: `BuildingBlocks_TriggerSubsumptionBroadcast`
/// Inherits from: `BuildingBlocks_TriggerBase`
pub struct BuildingBlocks_TriggerSubsumptionBroadcast {
    /// `name` (String)
    pub name: String,
    /// `eventRange` (Single)
    pub event_range: f32,
}

impl Pooled for BuildingBlocks_TriggerSubsumptionBroadcast {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_buildingblocks
            .building_blocks_trigger_subsumption_broadcast
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_buildingblocks
            .building_blocks_trigger_subsumption_broadcast
    }
}

impl<'a> Extract<'a> for BuildingBlocks_TriggerSubsumptionBroadcast {
    const TYPE_NAME: &'static str = "BuildingBlocks_TriggerSubsumptionBroadcast";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            event_range: inst.get_f32("eventRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_ShapeCircle`
/// Inherits from: `BuildingBlocks_ShapeBase`
pub struct BuildingBlocks_ShapeCircle {
    /// `center` (Class)
    pub center: Option<Handle<Vec2>>,
    /// `showBoundsDebug` (Boolean)
    pub show_bounds_debug: bool,
    /// `outerRadius` (Single)
    pub outer_radius: f32,
    /// `innerRadius` (Single)
    pub inner_radius: f32,
}

impl Pooled for BuildingBlocks_ShapeCircle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.building_blocks_shape_circle
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.building_blocks_shape_circle
    }
}

impl<'a> Extract<'a> for BuildingBlocks_ShapeCircle {
    const TYPE_NAME: &'static str = "BuildingBlocks_ShapeCircle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            show_bounds_debug: inst.get_bool("showBoundsDebug").unwrap_or_default(),
            outer_radius: inst.get_f32("outerRadius").unwrap_or_default(),
            inner_radius: inst.get_f32("innerRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSensitivity`
pub struct DockingSensitivity {
    /// `DockingUIRotationalSensitivity` (Single)
    pub docking_uirotational_sensitivity: f32,
    /// `DockingUILinearSensitivity` (Single)
    pub docking_uilinear_sensitivity: f32,
}

impl Pooled for DockingSensitivity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.docking_sensitivity
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.docking_sensitivity
    }
}

impl<'a> Extract<'a> for DockingSensitivity {
    const TYPE_NAME: &'static str = "DockingSensitivity";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            docking_uirotational_sensitivity: inst
                .get_f32("DockingUIRotationalSensitivity")
                .unwrap_or_default(),
            docking_uilinear_sensitivity: inst
                .get_f32("DockingUILinearSensitivity")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DisplayState`
pub struct DisplayState {
    /// `minimumValue` (Single)
    pub minimum_value: f32,
    /// `maximumValue` (Single)
    pub maximum_value: f32,
    /// `displayDuration` (Single)
    pub display_duration: f32,
    /// `activeRange` (EnumChoice)
    pub active_range: ActiveRange,
    /// `enabled` (Boolean)
    pub enabled: bool,
}

impl Pooled for DisplayState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.display_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.display_state
    }
}

impl<'a> Extract<'a> for DisplayState {
    const TYPE_NAME: &'static str = "DisplayState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_value: inst.get_f32("minimumValue").unwrap_or_default(),
            maximum_value: inst.get_f32("maximumValue").unwrap_or_default(),
            display_duration: inst.get_f32("displayDuration").unwrap_or_default(),
            active_range: ActiveRange::from_dcb_str(inst.get_str("activeRange").unwrap_or("")),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusWidgetDisplayPreset`
pub struct StatusWidgetDisplayPreset {
    /// `ranges` (Class)
    pub ranges: Option<Handle<DisplayState>>,
    /// `incrementDisplayDuration` (Single)
    pub increment_display_duration: f32,
    /// `incrementStep` (Single)
    pub increment_step: f32,
    /// `maximumChangePerSecond` (Single)
    pub maximum_change_per_second: f32,
    /// `historySeconds` (Int32)
    pub history_seconds: i32,
    /// `historySamplesPerSecond` (Int32)
    pub history_samples_per_second: i32,
    /// `shownOnLens` (Boolean)
    pub shown_on_lens: bool,
    /// `shownOnVisor` (Boolean)
    pub shown_on_visor: bool,
}

impl Pooled for StatusWidgetDisplayPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.status_widget_display_preset
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.status_widget_display_preset
    }
}

impl<'a> Extract<'a> for StatusWidgetDisplayPreset {
    const TYPE_NAME: &'static str = "StatusWidgetDisplayPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ranges: match inst.get("ranges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DisplayState>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            increment_display_duration: inst
                .get_f32("incrementDisplayDuration")
                .unwrap_or_default(),
            increment_step: inst.get_f32("incrementStep").unwrap_or_default(),
            maximum_change_per_second: inst.get_f32("maximumChangePerSecond").unwrap_or_default(),
            history_seconds: inst.get_i32("historySeconds").unwrap_or_default(),
            history_samples_per_second: inst.get_i32("historySamplesPerSecond").unwrap_or_default(),
            shown_on_lens: inst.get_bool("shownOnLens").unwrap_or_default(),
            shown_on_visor: inst.get_bool("shownOnVisor").unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Layout`
pub struct VisorLens_Layout {
    /// `regions` (Reference (array))
    pub regions: Vec<CigGuid>,
}

impl Pooled for VisorLens_Layout {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.visor_lens_layout
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.visor_lens_layout
    }
}

impl<'a> Extract<'a> for VisorLens_Layout {
    const TYPE_NAME: &'static str = "VisorLens_Layout";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            regions: inst
                .get_array("regions")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Region`
pub struct VisorLens_Region {
    /// `name` (String)
    pub name: String,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Deg3>>,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    pub anchor: Option<Handle<Vec3>>,
    /// `pivot` (Class)
    pub pivot: Option<Handle<Vec3>>,
    /// `flexDirection` (EnumChoice)
    pub flex_direction: BB_FlexDirection,
    /// `flexAxisJustification` (EnumChoice)
    pub flex_axis_justification: BB_FlexAxisJustification,
    /// `flexCrossAxisJustification` (EnumChoice)
    pub flex_cross_axis_justification: BB_FlexAxisJustification,
    /// `flexItemAlignment` (EnumChoice)
    pub flex_item_alignment: BB_FlexItemAlignment,
    /// `widgets` (Class (array))
    pub widgets: Vec<Handle<VisorLens_Widget>>,
}

impl Pooled for VisorLens_Region {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.visor_lens_region
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.visor_lens_region
    }
}

impl<'a> Extract<'a> for VisorLens_Region {
    const TYPE_NAME: &'static str = "VisorLens_Region";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            flex_direction: BB_FlexDirection::from_dcb_str(
                inst.get_str("flexDirection").unwrap_or(""),
            ),
            flex_axis_justification: BB_FlexAxisJustification::from_dcb_str(
                inst.get_str("flexAxisJustification").unwrap_or(""),
            ),
            flex_cross_axis_justification: BB_FlexAxisJustification::from_dcb_str(
                inst.get_str("flexCrossAxisJustification").unwrap_or(""),
            ),
            flex_item_alignment: BB_FlexItemAlignment::from_dcb_str(
                inst.get_str("flexItemAlignment").unwrap_or(""),
            ),
            widgets: inst
                .get_array("widgets")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<VisorLens_Widget>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<VisorLens_Widget>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Widget`
pub struct VisorLens_Widget {
    /// `name` (String)
    pub name: String,
    /// `namespace` (String)
    pub namespace: String,
    /// `canvas` (Reference)
    pub canvas: Option<CigGuid>,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    pub orientation: Option<Handle<Ang3>>,
    /// `slot` (Int32)
    pub slot: i32,
    /// `showTags` (Reference (array))
    pub show_tags: Vec<CigGuid>,
    /// `hideTags` (Reference (array))
    pub hide_tags: Vec<CigGuid>,
}

impl Pooled for VisorLens_Widget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_buildingblocks.visor_lens_widget
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_buildingblocks.visor_lens_widget
    }
}

impl<'a> Extract<'a> for VisorLens_Widget {
    const TYPE_NAME: &'static str = "VisorLens_Widget";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            namespace: inst
                .get_str("namespace")
                .map(String::from)
                .unwrap_or_default(),
            canvas: inst
                .get("canvas")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            slot: inst.get_i32("slot").unwrap_or_default(),
            show_tags: inst
                .get_array("showTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hide_tags: inst
                .get_array("hideTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
