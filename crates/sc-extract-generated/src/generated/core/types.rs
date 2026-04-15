// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `core`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ActivityBehaviorRequestCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBehaviorRequestCondition {
}

impl Pooled for ActivityBehaviorRequestCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.activity_behavior_request_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.activity_behavior_request_condition }
}

impl<'a> Extract<'a> for ActivityBehaviorRequestCondition {
    const TYPE_NAME: &'static str = "ActivityBehaviorRequestCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `IObservableExtender`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IObservableExtender {
}

impl Pooled for IObservableExtender {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.iobservable_extender }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.iobservable_extender }
}

impl<'a> Extract<'a> for IObservableExtender {
    const TYPE_NAME: &'static str = "IObservableExtender";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `IVisionComponentAdapter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IVisionComponentAdapter {
}

impl Pooled for IVisionComponentAdapter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ivision_component_adapter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ivision_component_adapter }
}

impl<'a> Extract<'a> for IVisionComponentAdapter {
    const TYPE_NAME: &'static str = "IVisionComponentAdapter";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MovementSystemAdditionalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSystemAdditionalParams {
}

impl Pooled for MovementSystemAdditionalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.movement_system_additional_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.movement_system_additional_params }
}

impl<'a> Extract<'a> for MovementSystemAdditionalParams {
    const TYPE_NAME: &'static str = "MovementSystemAdditionalParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `NavLinkLocation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavLinkLocation {
}

impl Pooled for NavLinkLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.nav_link_location }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.nav_link_location }
}

impl<'a> Extract<'a> for NavLinkLocation {
    const TYPE_NAME: &'static str = "NavLinkLocation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `NavigationLinkController`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationLinkController {
}

impl Pooled for NavigationLinkController {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.navigation_link_controller }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.navigation_link_controller }
}

impl<'a> Extract<'a> for NavigationLinkController {
    const TYPE_NAME: &'static str = "NavigationLinkController";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TraversalCostShapeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalCostShapeConfig {
}

impl Pooled for TraversalCostShapeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.traversal_cost_shape_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.traversal_cost_shape_config }
}

impl<'a> Extract<'a> for TraversalCostShapeConfig {
    const TYPE_NAME: &'static str = "TraversalCostShapeConfig";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `NavigationTriggerAdapter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationTriggerAdapter {
}

impl Pooled for NavigationTriggerAdapter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.navigation_trigger_adapter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.navigation_trigger_adapter }
}

impl<'a> Extract<'a> for NavigationTriggerAdapter {
    const TYPE_NAME: &'static str = "NavigationTriggerAdapter";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `INavigationCostVolumeExtender`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct INavigationCostVolumeExtender {
}

impl Pooled for INavigationCostVolumeExtender {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.inavigation_cost_volume_extender }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.inavigation_cost_volume_extender }
}

impl<'a> Extract<'a> for INavigationCostVolumeExtender {
    const TYPE_NAME: &'static str = "INavigationCostVolumeExtender";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ActionAreaExtensionType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAreaExtensionType {
}

impl Pooled for ActionAreaExtensionType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.action_area_extension_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.action_area_extension_type }
}

impl<'a> Extract<'a> for ActionAreaExtensionType {
    const TYPE_NAME: &'static str = "ActionAreaExtensionType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SActorForceReactionProceduralLeanPoseList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorForceReactionProceduralLeanPoseList {
}

impl Pooled for SActorForceReactionProceduralLeanPoseList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sactor_force_reaction_procedural_lean_pose_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sactor_force_reaction_procedural_lean_pose_list }
}

impl<'a> Extract<'a> for SActorForceReactionProceduralLeanPoseList {
    const TYPE_NAME: &'static str = "SActorForceReactionProceduralLeanPoseList";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `IMannequinActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMannequinActionDef {
}

impl Pooled for IMannequinActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.imannequin_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.imannequin_action_def }
}

impl<'a> Extract<'a> for IMannequinActionDef {
    const TYPE_NAME: &'static str = "IMannequinActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SActorStanceDimensionsExtraDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorStanceDimensionsExtraDef {
}

impl Pooled for SActorStanceDimensionsExtraDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sactor_stance_dimensions_extra_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sactor_stance_dimensions_extra_def }
}

impl<'a> Extract<'a> for SActorStanceDimensionsExtraDef {
    const TYPE_NAME: &'static str = "SActorStanceDimensionsExtraDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusEffectValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectValue {
}

impl Pooled for StatusEffectValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.status_effect_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.status_effect_value }
}

impl<'a> Extract<'a> for StatusEffectValue {
    const TYPE_NAME: &'static str = "StatusEffectValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusEffectSetupBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectSetupBase {
}

impl Pooled for StatusEffectSetupBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.status_effect_setup_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.status_effect_setup_base }
}

impl<'a> Extract<'a> for StatusEffectSetupBase {
    const TYPE_NAME: &'static str = "StatusEffectSetupBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusMaskedRetriggerSetupBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMaskedRetriggerSetupBase {
}

impl Pooled for StatusMaskedRetriggerSetupBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.status_masked_retrigger_setup_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.status_masked_retrigger_setup_base }
}

impl<'a> Extract<'a> for StatusMaskedRetriggerSetupBase {
    const TYPE_NAME: &'static str = "StatusMaskedRetriggerSetupBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LinkedStatPassValueBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatPassValueBase {
}

impl Pooled for LinkedStatPassValueBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.linked_stat_pass_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.linked_stat_pass_value_base }
}

impl<'a> Extract<'a> for LinkedStatPassValueBase {
    const TYPE_NAME: &'static str = "LinkedStatPassValueBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LinkedStatRuleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedStatRuleBase {
}

impl Pooled for LinkedStatRuleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.linked_stat_rule_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.linked_stat_rule_base }
}

impl<'a> Extract<'a> for LinkedStatRuleBase {
    const TYPE_NAME: &'static str = "LinkedStatRuleBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusBuffTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBuffTypeBase {
}

impl Pooled for StatusBuffTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.status_buff_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.status_buff_type_base }
}

impl<'a> Extract<'a> for StatusBuffTypeBase {
    const TYPE_NAME: &'static str = "StatusBuffTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuffDurationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffDurationBase {
}

impl Pooled for BuffDurationBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.buff_duration_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.buff_duration_base }
}

impl<'a> Extract<'a> for BuffDurationBase {
    const TYPE_NAME: &'static str = "BuffDurationBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CounterMeasureBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterMeasureBaseParams {
}

impl Pooled for CounterMeasureBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.counter_measure_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.counter_measure_base_params }
}

impl<'a> Extract<'a> for CounterMeasureBaseParams {
    const TYPE_NAME: &'static str = "CounterMeasureBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerAnimationTaskParamsBase`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerAnimationTaskParamsBase {
}

impl Pooled for SSequencerAnimationTaskParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_animation_task_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_animation_task_params_base }
}

impl<'a> Extract<'a> for SSequencerAnimationTaskParamsBase {
    const TYPE_NAME: &'static str = "SSequencerAnimationTaskParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AudioBreathStyleBaseNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleBaseNode {
}

impl Pooled for AudioBreathStyleBaseNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.audio_breath_style_base_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.audio_breath_style_base_node }
}

impl<'a> Extract<'a> for AudioBreathStyleBaseNode {
    const TYPE_NAME: &'static str = "AudioBreathStyleBaseNode";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_Node`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Node {
}

impl Pooled for BuildingBlocks_Node {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_node }
}

impl<'a> Extract<'a> for BuildingBlocks_Node {
    const TYPE_NAME: &'static str = "BuildingBlocks_Node";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsPathBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsPathBase {
}

impl Pooled for BuildingBlocks_BindingsPathBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_path_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_path_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsPathBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsPathBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsBooleanBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsBooleanBase {
}

impl Pooled for BuildingBlocks_BindingsBooleanBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_boolean_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_boolean_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsBooleanBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsBooleanBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsIntegerBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsIntegerBase {
}

impl Pooled for BuildingBlocks_BindingsIntegerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_integer_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_integer_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsIntegerBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsIntegerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsNumberBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsNumberBase {
}

impl Pooled for BuildingBlocks_BindingsNumberBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_number_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_number_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsNumberBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsNumberBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BindingsOperations_WaveformShapeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingsOperations_WaveformShapeBase {
}

impl Pooled for BindingsOperations_WaveformShapeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.bindings_operations_waveform_shape_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.bindings_operations_waveform_shape_base }
}

impl<'a> Extract<'a> for BindingsOperations_WaveformShapeBase {
    const TYPE_NAME: &'static str = "BindingsOperations_WaveformShapeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsStringBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsStringBase {
}

impl Pooled for BuildingBlocks_BindingsStringBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_string_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_string_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsStringBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsStringBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsLocalizedBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsLocalizedBase {
}

impl Pooled for BuildingBlocks_BindingsLocalizedBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_localized_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_localized_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsLocalizedBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsLocalizedBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsColorBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsColorBase {
}

impl Pooled for BuildingBlocks_BindingsColorBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_color_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_color_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsColorBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsColorBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsVectorBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsVectorBase {
}

impl Pooled for BuildingBlocks_BindingsVectorBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_vector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_vector_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsVectorBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsVectorBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsRotationBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsRotationBase {
}

impl Pooled for BuildingBlocks_BindingsRotationBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_rotation_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_rotation_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsRotationBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsRotationBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_BindingsTransformBase`
/// Inherits from: `BuildingBlocks_BindingsOperationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_BindingsTransformBase {
}

impl Pooled for BuildingBlocks_BindingsTransformBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_bindings_transform_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_bindings_transform_base }
}

impl<'a> Extract<'a> for BuildingBlocks_BindingsTransformBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_BindingsTransformBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_LayoutPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LayoutPolicyBase {
}

impl Pooled for BuildingBlocks_LayoutPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_layout_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_layout_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_LayoutPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_LayoutPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_LayoutPolicyItemBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LayoutPolicyItemBase {
}

impl Pooled for BuildingBlocks_LayoutPolicyItemBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_layout_policy_item_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_layout_policy_item_base }
}

impl<'a> Extract<'a> for BuildingBlocks_LayoutPolicyItemBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_LayoutPolicyItemBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_ScrollPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ScrollPolicyBase {
}

impl Pooled for BuildingBlocks_ScrollPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_scroll_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_scroll_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ScrollPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ScrollPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FieldModifierBase {
}

impl Pooled for BuildingBlocks_FieldModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_field_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_field_modifier_base }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierEnumeratedTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FieldModifierEnumeratedTypeBase {
}

impl Pooled for BuildingBlocks_FieldModifierEnumeratedTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_field_modifier_enumerated_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_field_modifier_enumerated_type_base }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierEnumeratedTypeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierEnumeratedTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_RendererPolicyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_RendererPolicyBase {
}

impl Pooled for BuildingBlocks_RendererPolicyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_renderer_policy_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_renderer_policy_base }
}

impl<'a> Extract<'a> for BuildingBlocks_RendererPolicyBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_RendererPolicyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_FieldModifierRecordRefTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FieldModifierRecordRefTypeBase {
}

impl Pooled for BuildingBlocks_FieldModifierRecordRefTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_field_modifier_record_ref_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_field_modifier_record_ref_type_base }
}

impl<'a> Extract<'a> for BuildingBlocks_FieldModifierRecordRefTypeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_FieldModifierRecordRefTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TimingFunctionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimingFunctionBase {
}

impl Pooled for BuildingBlocks_TimingFunctionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_timing_function_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_timing_function_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TimingFunctionBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimingFunctionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TimelineTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimelineTypeBase {
}

impl Pooled for BuildingBlocks_TimelineTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_timeline_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_timeline_type_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TimelineTypeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimelineTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TransformerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TransformerBase {
}

impl Pooled for BuildingBlocks_TransformerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_transformer_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_transformer_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TransformerBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TransformerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_SlicerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_SlicerBase {
}

impl Pooled for BuildingBlocks_SlicerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_slicer_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_slicer_base }
}

impl<'a> Extract<'a> for BuildingBlocks_SlicerBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_SlicerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TriggerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TriggerBase {
}

impl Pooled for BuildingBlocks_TriggerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_trigger_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_trigger_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TriggerBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TriggerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TextFormatModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TextFormatModifierBase {
}

impl Pooled for BuildingBlocks_TextFormatModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_text_format_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_text_format_modifier_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TextFormatModifierBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TextFormatModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_StyleSelectorConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_StyleSelectorConditionBase {
}

impl Pooled for BuildingBlocks_StyleSelectorConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_style_selector_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_style_selector_condition_base }
}

impl<'a> Extract<'a> for BuildingBlocks_StyleSelectorConditionBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_StyleSelectorConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_ColorBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ColorBase {
}

impl Pooled for BuildingBlocks_ColorBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_color_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_color_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ColorBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ColorBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_ContainerModeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_ContainerModeBase {
}

impl Pooled for BuildingBlocks_ContainerModeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.building_blocks_container_mode_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.building_blocks_container_mode_base }
}

impl<'a> Extract<'a> for BuildingBlocks_ContainerModeBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_ContainerModeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseCargoFillCapacityValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCargoFillCapacityValue {
}

impl Pooled for BaseCargoFillCapacityValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_cargo_fill_capacity_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_cargo_fill_capacity_value }
}

impl<'a> Extract<'a> for BaseCargoFillCapacityValue {
    const TYPE_NAME: &'static str = "BaseCargoFillCapacityValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCustomzierColorDefBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCustomzierColorDefBase {
}

impl Pooled for SCustomzierColorDefBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scustomzier_color_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scustomzier_color_def_base }
}

impl<'a> Extract<'a> for SCustomzierColorDefBase {
    const TYPE_NAME: &'static str = "SCustomzierColorDefBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCharacterCustomizerRandomizationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterCustomizerRandomizationParams {
}

impl Pooled for SCharacterCustomizerRandomizationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scharacter_customizer_randomization_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scharacter_customizer_randomization_params }
}

impl<'a> Extract<'a> for SCharacterCustomizerRandomizationParams {
    const TYPE_NAME: &'static str = "SCharacterCustomizerRandomizationParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCharacterValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCharacterValidationParams {
}

impl Pooled for SCharacterValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scharacter_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scharacter_validation_params }
}

impl<'a> Extract<'a> for SCharacterValidationParams {
    const TYPE_NAME: &'static str = "SCharacterValidationParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TemperatureDamageControl`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureDamageControl {
}

impl Pooled for TemperatureDamageControl {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.temperature_damage_control }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.temperature_damage_control }
}

impl<'a> Extract<'a> for TemperatureDamageControl {
    const TYPE_NAME: &'static str = "TemperatureDamageControl";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingCost_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingCost_Base {
}

impl Pooled for LegacyCraftingCost_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.legacy_crafting_cost_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.legacy_crafting_cost_base }
}

impl<'a> Extract<'a> for LegacyCraftingCost_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingCost_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingOutput_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingOutput_Base {
}

impl Pooled for LegacyCraftingOutput_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.legacy_crafting_output_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.legacy_crafting_output_base }
}

impl<'a> Extract<'a> for LegacyCraftingOutput_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingOutput_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipe_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipe_Base {
}

impl Pooled for LegacyCraftingRecipe_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.legacy_crafting_recipe_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.legacy_crafting_recipe_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipe_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipe_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeDef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeDef_Base {
}

impl Pooled for LegacyCraftingRecipeDef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.legacy_crafting_recipe_def_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.legacy_crafting_recipe_def_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeDef_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeDef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LegacyCraftingRecipeList_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCraftingRecipeList_Base {
}

impl Pooled for LegacyCraftingRecipeList_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.legacy_crafting_recipe_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.legacy_crafting_recipe_list_base }
}

impl<'a> Extract<'a> for LegacyCraftingRecipeList_Base {
    const TYPE_NAME: &'static str = "LegacyCraftingRecipeList_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSensorMineTriggerType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSensorMineTriggerType {
}

impl Pooled for SSensorMineTriggerType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssensor_mine_trigger_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssensor_mine_trigger_type }
}

impl<'a> Extract<'a> for SSensorMineTriggerType {
    const TYPE_NAME: &'static str = "SSensorMineTriggerType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HandholdAttachPointChoiceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandholdAttachPointChoiceParams {
}

impl Pooled for HandholdAttachPointChoiceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.handhold_attach_point_choice_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.handhold_attach_point_choice_params }
}

impl<'a> Extract<'a> for HandholdAttachPointChoiceParams {
    const TYPE_NAME: &'static str = "HandholdAttachPointChoiceParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestableTagListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableTagListBase {
}

impl Pooled for HarvestableTagListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.harvestable_tag_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.harvestable_tag_list_base }
}

impl<'a> Extract<'a> for HarvestableTagListBase {
    const TYPE_NAME: &'static str = "HarvestableTagListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SubHarvestableConfigBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigBase {
}

impl Pooled for SubHarvestableConfigBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sub_harvestable_config_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sub_harvestable_config_base }
}

impl<'a> Extract<'a> for SubHarvestableConfigBase {
    const TYPE_NAME: &'static str = "SubHarvestableConfigBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SubHarvestableConfigSingleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubHarvestableConfigSingleBase {
}

impl Pooled for SubHarvestableConfigSingleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sub_harvestable_config_single_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sub_harvestable_config_single_base }
}

impl<'a> Extract<'a> for SubHarvestableConfigSingleBase {
    const TYPE_NAME: &'static str = "SubHarvestableConfigSingleBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestConditionBase {
}

impl Pooled for HarvestConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.harvest_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.harvest_condition_base }
}

impl<'a> Extract<'a> for HarvestConditionBase {
    const TYPE_NAME: &'static str = "HarvestConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HarvestableAreaTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestableAreaTypeBase {
}

impl Pooled for HarvestableAreaTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.harvestable_area_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.harvestable_area_type_base }
}

impl<'a> Extract<'a> for HarvestableAreaTypeBase {
    const TYPE_NAME: &'static str = "HarvestableAreaTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemThrottleParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemThrottleParamsBase {
}

impl Pooled for ItemThrottleParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_throttle_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_throttle_params_base }
}

impl<'a> Extract<'a> for ItemThrottleParamsBase {
    const TYPE_NAME: &'static str = "ItemThrottleParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemModifierLifetime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemModifierLifetime {
}

impl Pooled for ItemModifierLifetime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_modifier_lifetime }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_modifier_lifetime }
}

impl<'a> Extract<'a> for ItemModifierLifetime {
    const TYPE_NAME: &'static str = "ItemModifierLifetime";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseItemModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItemModifierParams {
}

impl Pooled for BaseItemModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_item_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_item_modifier_params }
}

impl<'a> Extract<'a> for BaseItemModifierParams {
    const TYPE_NAME: &'static str = "BaseItemModifierParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LightFlickerWaveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightFlickerWaveParams {
}

impl Pooled for LightFlickerWaveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.light_flicker_wave_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.light_flicker_wave_params }
}

impl<'a> Extract<'a> for LightFlickerWaveParams {
    const TYPE_NAME: &'static str = "LightFlickerWaveParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMisfireCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMisfireCondition {
}

impl Pooled for SMisfireCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.smisfire_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.smisfire_condition }
}

impl<'a> Extract<'a> for SMisfireCondition {
    const TYPE_NAME: &'static str = "SMisfireCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMisfireDamage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMisfireDamage {
}

impl Pooled for SMisfireDamage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.smisfire_damage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.smisfire_damage }
}

impl<'a> Extract<'a> for SMisfireDamage {
    const TYPE_NAME: &'static str = "SMisfireDamage";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SLegacyItemMisfireParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLegacyItemMisfireParams {
}

impl Pooled for SLegacyItemMisfireParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.slegacy_item_misfire_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.slegacy_item_misfire_params }
}

impl<'a> Extract<'a> for SLegacyItemMisfireParams {
    const TYPE_NAME: &'static str = "SLegacyItemMisfireParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ParticleEffectNoneTintingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEffectNoneTintingParams {
}

impl Pooled for ParticleEffectNoneTintingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.particle_effect_none_tinting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.particle_effect_none_tinting_params }
}

impl<'a> Extract<'a> for ParticleEffectNoneTintingParams {
    const TYPE_NAME: &'static str = "ParticleEffectNoneTintingParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SQedVisualGraphTransitionTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQedVisualGraphTransitionTypeParams {
}

impl Pooled for SQedVisualGraphTransitionTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sqed_visual_graph_transition_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sqed_visual_graph_transition_type_params }
}

impl<'a> Extract<'a> for SQedVisualGraphTransitionTypeParams {
    const TYPE_NAME: &'static str = "SQedVisualGraphTransitionTypeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `STargetingMethodBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetingMethodBase {
}

impl Pooled for STargetingMethodBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.stargeting_method_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.stargeting_method_base }
}

impl<'a> Extract<'a> for STargetingMethodBase {
    const TYPE_NAME: &'static str = "STargetingMethodBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CtxGraph_Node`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Node {
}

impl Pooled for CtxGraph_Node {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ctx_graph_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ctx_graph_node }
}

impl<'a> Extract<'a> for CtxGraph_Node {
    const TYPE_NAME: &'static str = "CtxGraph_Node";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CtxGraph_Component`
/// Inherits from: `CtxGraph_Node`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Component {
}

impl Pooled for CtxGraph_Component {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ctx_graph_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ctx_graph_component }
}

impl<'a> Extract<'a> for CtxGraph_Component {
    const TYPE_NAME: &'static str = "CtxGraph_Component";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemControllableParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemControllableParams {
}

impl Pooled for SCItemControllableParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_controllable_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_controllable_params }
}

impl<'a> Extract<'a> for SCItemControllableParams {
    const TYPE_NAME: &'static str = "SCItemControllableParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemControlCondition_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemControlCondition_Base {
}

impl Pooled for SCItemControlCondition_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_control_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_control_condition_base }
}

impl<'a> Extract<'a> for SCItemControlCondition_Base {
    const TYPE_NAME: &'static str = "SCItemControlCondition_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemControlBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemControlBaseParams {
}

impl Pooled for SCItemControlBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_control_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_control_base_params }
}

impl<'a> Extract<'a> for SCItemControlBaseParams {
    const TYPE_NAME: &'static str = "SCItemControlBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemControlPriorityValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemControlPriorityValue {
}

impl Pooled for SCItemControlPriorityValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_control_priority_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_control_priority_value }
}

impl<'a> Extract<'a> for SCItemControlPriorityValue {
    const TYPE_NAME: &'static str = "SCItemControlPriorityValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ConversationNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationNode_Base {
}

impl Pooled for ConversationNode_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.conversation_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.conversation_node_base }
}

impl<'a> Extract<'a> for ConversationNode_Base {
    const TYPE_NAME: &'static str = "ConversationNode_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingCostContext_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingCostContext_Base {
}

impl Pooled for CraftingCostContext_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_cost_context_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_cost_context_base }
}

impl<'a> Extract<'a> for CraftingCostContext_Base {
    const TYPE_NAME: &'static str = "CraftingCostContext_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingOptionalEffect_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingOptionalEffect_Base {
}

impl Pooled for CraftingOptionalEffect_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_optional_effect_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_optional_effect_base }
}

impl<'a> Extract<'a> for CraftingOptionalEffect_Base {
    const TYPE_NAME: &'static str = "CraftingOptionalEffect_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingGameplayPropertyModifierValueRange_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGameplayPropertyModifierValueRange_Base {
}

impl Pooled for CraftingGameplayPropertyModifierValueRange_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_gameplay_property_modifier_value_range_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_gameplay_property_modifier_value_range_base }
}

impl<'a> Extract<'a> for CraftingGameplayPropertyModifierValueRange_Base {
    const TYPE_NAME: &'static str = "CraftingGameplayPropertyModifierValueRange_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingGameplayPropertyModifier_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGameplayPropertyModifier_Base {
}

impl Pooled for CraftingGameplayPropertyModifier_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_gameplay_property_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_gameplay_property_modifier_base }
}

impl<'a> Extract<'a> for CraftingGameplayPropertyModifier_Base {
    const TYPE_NAME: &'static str = "CraftingGameplayPropertyModifier_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingGameplayPropertyModifiers_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingGameplayPropertyModifiers_Base {
}

impl Pooled for CraftingGameplayPropertyModifiers_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_gameplay_property_modifiers_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_gameplay_property_modifiers_base }
}

impl<'a> Extract<'a> for CraftingGameplayPropertyModifiers_Base {
    const TYPE_NAME: &'static str = "CraftingGameplayPropertyModifiers_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingResult_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResult_Base {
}

impl Pooled for CraftingResult_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_result_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_result_base }
}

impl<'a> Extract<'a> for CraftingResult_Base {
    const TYPE_NAME: &'static str = "CraftingResult_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingRecipeCosts_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipeCosts_Base {
}

impl Pooled for CraftingRecipeCosts_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_recipe_costs_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_recipe_costs_base }
}

impl<'a> Extract<'a> for CraftingRecipeCosts_Base {
    const TYPE_NAME: &'static str = "CraftingRecipeCosts_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingRecipeCosts_Base_NonRef`
/// Inherits from: `CraftingRecipeCosts_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipeCosts_Base_NonRef {
}

impl Pooled for CraftingRecipeCosts_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_recipe_costs_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_recipe_costs_base_non_ref }
}

impl<'a> Extract<'a> for CraftingRecipeCosts_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingRecipeCosts_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingRecipeResults_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipeResults_Base {
}

impl Pooled for CraftingRecipeResults_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_recipe_results_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_recipe_results_base }
}

impl<'a> Extract<'a> for CraftingRecipeResults_Base {
    const TYPE_NAME: &'static str = "CraftingRecipeResults_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingProcessSpecificRecipeData_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingProcessSpecificRecipeData_Base {
}

impl Pooled for CraftingProcessSpecificRecipeData_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_process_specific_recipe_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_process_specific_recipe_data_base }
}

impl<'a> Extract<'a> for CraftingProcessSpecificRecipeData_Base {
    const TYPE_NAME: &'static str = "CraftingProcessSpecificRecipeData_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingRecipe_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipe_Base {
}

impl Pooled for CraftingRecipe_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_recipe_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_recipe_base }
}

impl<'a> Extract<'a> for CraftingRecipe_Base {
    const TYPE_NAME: &'static str = "CraftingRecipe_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingRecipe_Base_NonRef`
/// Inherits from: `CraftingRecipe_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipe_Base_NonRef {
}

impl Pooled for CraftingRecipe_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_recipe_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_recipe_base_non_ref }
}

impl<'a> Extract<'a> for CraftingRecipe_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingRecipe_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingResearchUnlock_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResearchUnlock_Base {
}

impl Pooled for CraftingResearchUnlock_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_research_unlock_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_research_unlock_base }
}

impl<'a> Extract<'a> for CraftingResearchUnlock_Base {
    const TYPE_NAME: &'static str = "CraftingResearchUnlock_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingResearch_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResearch_Base {
}

impl Pooled for CraftingResearch_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_research_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_research_base }
}

impl<'a> Extract<'a> for CraftingResearch_Base {
    const TYPE_NAME: &'static str = "CraftingResearch_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingBlueprintTier_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingBlueprintTier_Base {
}

impl Pooled for CraftingBlueprintTier_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_blueprint_tier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_blueprint_tier_base }
}

impl<'a> Extract<'a> for CraftingBlueprintTier_Base {
    const TYPE_NAME: &'static str = "CraftingBlueprintTier_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BlueprintCategoryAvailability_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintCategoryAvailability_Base {
}

impl Pooled for BlueprintCategoryAvailability_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.blueprint_category_availability_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.blueprint_category_availability_base }
}

impl<'a> Extract<'a> for BlueprintCategoryAvailability_Base {
    const TYPE_NAME: &'static str = "BlueprintCategoryAvailability_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingProcess_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingProcess_Base {
}

impl Pooled for CraftingProcess_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_process_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_process_base }
}

impl<'a> Extract<'a> for CraftingProcess_Base {
    const TYPE_NAME: &'static str = "CraftingProcess_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GenericCraftingProcess_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericCraftingProcess_Base {
}

impl Pooled for GenericCraftingProcess_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.generic_crafting_process_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.generic_crafting_process_base }
}

impl<'a> Extract<'a> for GenericCraftingProcess_Base {
    const TYPE_NAME: &'static str = "GenericCraftingProcess_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingBlueprint_Base_NonRef`
/// Inherits from: `CraftingBlueprint_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingBlueprint_Base_NonRef {
}

impl Pooled for CraftingBlueprint_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_blueprint_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_blueprint_base_non_ref }
}

impl<'a> Extract<'a> for CraftingBlueprint_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingBlueprint_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultBlueprintSelection_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultBlueprintSelection_Base {
}

impl Pooled for DefaultBlueprintSelection_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.default_blueprint_selection_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.default_blueprint_selection_base }
}

impl<'a> Extract<'a> for DefaultBlueprintSelection_Base {
    const TYPE_NAME: &'static str = "DefaultBlueprintSelection_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityDistribution_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityDistribution_Base {
}

impl Pooled for CraftingQualityDistribution_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_quality_distribution_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_quality_distribution_base }
}

impl<'a> Extract<'a> for CraftingQualityDistribution_Base {
    const TYPE_NAME: &'static str = "CraftingQualityDistribution_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityDistribution_Base_NonRef`
/// Inherits from: `CraftingQualityDistribution_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityDistribution_Base_NonRef {
}

impl Pooled for CraftingQualityDistribution_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_quality_distribution_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_quality_distribution_base_non_ref }
}

impl<'a> Extract<'a> for CraftingQualityDistribution_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingQualityDistribution_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityLocationOverride_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityLocationOverride_Base {
}

impl Pooled for CraftingQualityLocationOverride_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_quality_location_override_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_quality_location_override_base }
}

impl<'a> Extract<'a> for CraftingQualityLocationOverride_Base {
    const TYPE_NAME: &'static str = "CraftingQualityLocationOverride_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CraftingQualityLocationOverride_Base_NonRef`
/// Inherits from: `CraftingQualityLocationOverride_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingQualityLocationOverride_Base_NonRef {
}

impl Pooled for CraftingQualityLocationOverride_Base_NonRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.crafting_quality_location_override_base_non_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.crafting_quality_location_override_base_non_ref }
}

impl<'a> Extract<'a> for CraftingQualityLocationOverride_Base_NonRef {
    const TYPE_NAME: &'static str = "CraftingQualityLocationOverride_Base_NonRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CustomFloat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFloat {
}

impl Pooled for CustomFloat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.custom_float }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.custom_float }
}

impl<'a> Extract<'a> for CustomFloat {
    const TYPE_NAME: &'static str = "CustomFloat";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DamageBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageBase {
}

impl Pooled for DamageBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.damage_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.damage_base }
}

impl<'a> Extract<'a> for DamageBase {
    const TYPE_NAME: &'static str = "DamageBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DamageResistanceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResistanceBase {
}

impl Pooled for DamageResistanceBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.damage_resistance_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.damage_resistance_base }
}

impl<'a> Extract<'a> for DamageResistanceBase {
    const TYPE_NAME: &'static str = "DamageResistanceBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ExplosionBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionBaseParams {
}

impl Pooled for ExplosionBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.explosion_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.explosion_base_params }
}

impl<'a> Extract<'a> for ExplosionBaseParams {
    const TYPE_NAME: &'static str = "ExplosionBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionDef {
}

impl Pooled for DefaultActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.default_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.default_action_def }
}

impl<'a> Extract<'a> for DefaultActionDef {
    const TYPE_NAME: &'static str = "DefaultActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityEntryCondition {
}

impl Pooled for DefaultActionsEntityEntryCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.default_actions_entity_entry_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.default_actions_entity_entry_condition }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsEntityState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityState {
}

impl Pooled for DefaultActionsEntityState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.default_actions_entity_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.default_actions_entity_state }
}

impl<'a> Extract<'a> for DefaultActionsEntityState {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DialogueContextEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContextEntry {
}

impl Pooled for DialogueContextEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.dialogue_context_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.dialogue_context_entry }
}

impl<'a> Extract<'a> for DialogueContextEntry {
    const TYPE_NAME: &'static str = "DialogueContextEntry";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIRoundsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIRoundsModule {
}

impl Pooled for SIRoundsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sirounds_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sirounds_module }
}

impl<'a> Extract<'a> for SIRoundsModule {
    const TYPE_NAME: &'static str = "SIRoundsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEffectParamsNodeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEffectParamsNodeBase {
}

impl Pooled for SEffectParamsNodeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.seffect_params_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.seffect_params_node_base }
}

impl<'a> Extract<'a> for SEffectParamsNodeBase {
    const TYPE_NAME: &'static str = "SEffectParamsNodeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataForgeComponentParams {
}

impl Pooled for DataForgeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.data_forge_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.data_forge_component_params }
}

impl<'a> Extract<'a> for DataForgeComponentParams {
    const TYPE_NAME: &'static str = "DataForgeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityClassStaticDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClassStaticDataParams {
}

impl Pooled for EntityClassStaticDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entity_class_static_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entity_class_static_data_params }
}

impl<'a> Extract<'a> for EntityClassStaticDataParams {
    const TYPE_NAME: &'static str = "EntityClassStaticDataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SHazardAreaShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHazardAreaShapeParams {
}

impl Pooled for SHazardAreaShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.shazard_area_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.shazard_area_shape_params }
}

impl<'a> Extract<'a> for SHazardAreaShapeParams {
    const TYPE_NAME: &'static str = "SHazardAreaShapeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `FloatUserVariableTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatUserVariableTask {
}

impl Pooled for FloatUserVariableTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.float_user_variable_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.float_user_variable_task }
}

impl<'a> Extract<'a> for FloatUserVariableTask {
    const TYPE_NAME: &'static str = "FloatUserVariableTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `IntUserVariableTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntUserVariableTask {
}

impl Pooled for IntUserVariableTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.int_user_variable_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.int_user_variable_task }
}

impl<'a> Extract<'a> for IntUserVariableTask {
    const TYPE_NAME: &'static str = "IntUserVariableTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BoolUserVariableTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolUserVariableTask {
}

impl Pooled for BoolUserVariableTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.bool_user_variable_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.bool_user_variable_task }
}

impl<'a> Extract<'a> for BoolUserVariableTask {
    const TYPE_NAME: &'static str = "BoolUserVariableTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StringUserVariableTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringUserVariableTask {
}

impl Pooled for StringUserVariableTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.string_user_variable_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.string_user_variable_task }
}

impl<'a> Extract<'a> for StringUserVariableTask {
    const TYPE_NAME: &'static str = "StringUserVariableTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `RecordRefUserVariableTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRefUserVariableTypeBase {
}

impl Pooled for RecordRefUserVariableTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.record_ref_user_variable_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.record_ref_user_variable_type_base }
}

impl<'a> Extract<'a> for RecordRefUserVariableTypeBase {
    const TYPE_NAME: &'static str = "RecordRefUserVariableTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CloneLocationMedicalTier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneLocationMedicalTier {
}

impl Pooled for CloneLocationMedicalTier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.clone_location_medical_tier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.clone_location_medical_tier }
}

impl<'a> Extract<'a> for CloneLocationMedicalTier {
    const TYPE_NAME: &'static str = "CloneLocationMedicalTier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerCarryableTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerCarryableTaskParams {
}

impl Pooled for SSequencerCarryableTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_carryable_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_carryable_task_params }
}

impl<'a> Extract<'a> for SSequencerCarryableTaskParams {
    const TYPE_NAME: &'static str = "SSequencerCarryableTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ChatProviderSettingsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatProviderSettingsBase {
}

impl Pooled for ChatProviderSettingsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.chat_provider_settings_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.chat_provider_settings_base }
}

impl<'a> Extract<'a> for ChatProviderSettingsBase {
    const TYPE_NAME: &'static str = "ChatProviderSettingsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SLoadoutRequirementBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadoutRequirementBase {
}

impl Pooled for SLoadoutRequirementBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sloadout_requirement_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sloadout_requirement_base }
}

impl<'a> Extract<'a> for SLoadoutRequirementBase {
    const TYPE_NAME: &'static str = "SLoadoutRequirementBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerEntityDragTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerEntityDragTaskParams {
}

impl Pooled for SSequencerEntityDragTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_entity_drag_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_entity_drag_task_params }
}

impl<'a> Extract<'a> for SSequencerEntityDragTaskParams {
    const TYPE_NAME: &'static str = "SSequencerEntityDragTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EATransportBaseTransitionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EATransportBaseTransitionParams {
}

impl Pooled for EATransportBaseTransitionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.eatransport_base_transition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.eatransport_base_transition_params }
}

impl<'a> Extract<'a> for EATransportBaseTransitionParams {
    const TYPE_NAME: &'static str = "EATransportBaseTransitionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemMisfireParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemMisfireParams {
}

impl Pooled for SItemMisfireParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sitem_misfire_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sitem_misfire_params }
}

impl<'a> Extract<'a> for SItemMisfireParams {
    const TYPE_NAME: &'static str = "SItemMisfireParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SObjectMetadataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SObjectMetadataParams {
}

impl Pooled for SObjectMetadataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sobject_metadata_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sobject_metadata_params }
}

impl<'a> Extract<'a> for SObjectMetadataParams {
    const TYPE_NAME: &'static str = "SObjectMetadataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SAnimatedOutfitSwapData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnimatedOutfitSwapData {
}

impl Pooled for SAnimatedOutfitSwapData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sanimated_outfit_swap_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sanimated_outfit_swap_data }
}

impl<'a> Extract<'a> for SAnimatedOutfitSwapData {
    const TYPE_NAME: &'static str = "SAnimatedOutfitSwapData";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerDefTaskParams {
}

impl Pooled for SSequencerDefTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_def_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_def_task_params }
}

impl<'a> Extract<'a> for SSequencerDefTaskParams {
    const TYPE_NAME: &'static str = "SSequencerDefTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSpawnRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpawnRules {
}

impl Pooled for SSpawnRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sspawn_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sspawn_rules }
}

impl<'a> Extract<'a> for SSpawnRules {
    const TYPE_NAME: &'static str = "SSpawnRules";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BasePortRefillData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePortRefillData {
}

impl Pooled for BasePortRefillData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_port_refill_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_port_refill_data }
}

impl<'a> Extract<'a> for BasePortRefillData {
    const TYPE_NAME: &'static str = "BasePortRefillData";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseSpawnerPrerequisite`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseSpawnerPrerequisite {
}

impl Pooled for BaseSpawnerPrerequisite {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_spawner_prerequisite }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_spawner_prerequisite }
}

impl<'a> Extract<'a> for BaseSpawnerPrerequisite {
    const TYPE_NAME: &'static str = "BaseSpawnerPrerequisite";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerSpawnerTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerSpawnerTaskParams {
}

impl Pooled for SSequencerSpawnerTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_spawner_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_spawner_task_params }
}

impl<'a> Extract<'a> for SSequencerSpawnerTaskParams {
    const TYPE_NAME: &'static str = "SSequencerSpawnerTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerDespawnerTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerDespawnerTaskParams {
}

impl Pooled for SSequencerDespawnerTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_despawner_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_despawner_task_params }
}

impl<'a> Extract<'a> for SSequencerDespawnerTaskParams {
    const TYPE_NAME: &'static str = "SSequencerDespawnerTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityEffectSystem_PropertyModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityEffectSystem_PropertyModifier {
}

impl Pooled for SEntityEffectSystem_PropertyModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sentity_effect_system_property_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sentity_effect_system_property_modifier }
}

impl<'a> Extract<'a> for SEntityEffectSystem_PropertyModifier {
    const TYPE_NAME: &'static str = "SEntityEffectSystem_PropertyModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityEffectSystem_BaseSequencerTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityEffectSystem_BaseSequencerTask {
}

impl Pooled for EntityEffectSystem_BaseSequencerTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entity_effect_system_base_sequencer_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entity_effect_system_base_sequencer_task }
}

impl<'a> Extract<'a> for EntityEffectSystem_BaseSequencerTask {
    const TYPE_NAME: &'static str = "EntityEffectSystem_BaseSequencerTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EventDispatcher`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDispatcher {
}

impl Pooled for EventDispatcher {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.event_dispatcher }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.event_dispatcher }
}

impl<'a> Extract<'a> for EventDispatcher {
    const TYPE_NAME: &'static str = "EventDispatcher";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityDensityClassOverridesBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityDensityClassOverridesBase {
}

impl Pooled for SEntityDensityClassOverridesBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sentity_density_class_overrides_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sentity_density_class_overrides_base }
}

impl<'a> Extract<'a> for SEntityDensityClassOverridesBase {
    const TYPE_NAME: &'static str = "SEntityDensityClassOverridesBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityTraversingNodeTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityTraversingNodeTypeParams {
}

impl Pooled for SEntityTraversingNodeTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sentity_traversing_node_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sentity_traversing_node_type_params }
}

impl<'a> Extract<'a> for SEntityTraversingNodeTypeParams {
    const TYPE_NAME: &'static str = "SEntityTraversingNodeTypeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityTraversingExecuteNodeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityTraversingExecuteNodeBase {
}

impl Pooled for SEntityTraversingExecuteNodeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sentity_traversing_execute_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sentity_traversing_execute_node_base }
}

impl<'a> Extract<'a> for SEntityTraversingExecuteNodeBase {
    const TYPE_NAME: &'static str = "SEntityTraversingExecuteNodeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `FireVoxelSelectionShape`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireVoxelSelectionShape {
}

impl Pooled for FireVoxelSelectionShape {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.fire_voxel_selection_shape }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.fire_voxel_selection_shape }
}

impl<'a> Extract<'a> for FireVoxelSelectionShape {
    const TYPE_NAME: &'static str = "FireVoxelSelectionShape";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ExtinguishType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinguishType_Base {
}

impl Pooled for ExtinguishType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.extinguish_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.extinguish_type_base }
}

impl<'a> Extract<'a> for ExtinguishType_Base {
    const TYPE_NAME: &'static str = "ExtinguishType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `FireRepairerType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireRepairerType_Base {
}

impl Pooled for FireRepairerType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.fire_repairer_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.fire_repairer_type_base }
}

impl<'a> Extract<'a> for FireRepairerType_Base {
    const TYPE_NAME: &'static str = "FireRepairerType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SRtpcBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRtpcBehaviour {
}

impl Pooled for SRtpcBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.srtpc_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.srtpc_behaviour }
}

impl<'a> Extract<'a> for SRtpcBehaviour {
    const TYPE_NAME: &'static str = "SRtpcBehaviour";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIPickupModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPickupModule {
}

impl Pooled for SIPickupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sipickup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sipickup_module }
}

impl<'a> Extract<'a> for SIPickupModule {
    const TYPE_NAME: &'static str = "SIPickupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIDamageHandlingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDamageHandlingModule {
}

impl Pooled for SIDamageHandlingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sidamage_handling_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sidamage_handling_module }
}

impl<'a> Extract<'a> for SIDamageHandlingModule {
    const TYPE_NAME: &'static str = "SIDamageHandlingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISpectatorModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpectatorModule {
}

impl Pooled for SISpectatorModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sispectator_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sispectator_module }
}

impl<'a> Extract<'a> for SISpectatorModule {
    const TYPE_NAME: &'static str = "SISpectatorModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIPlayerSetupModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerSetupModule {
}

impl Pooled for SIPlayerSetupModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.siplayer_setup_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.siplayer_setup_module }
}

impl<'a> Extract<'a> for SIPlayerSetupModule {
    const TYPE_NAME: &'static str = "SIPlayerSetupModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStatsRecordingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStatsRecordingModule {
}

impl Pooled for SIStatsRecordingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sistats_recording_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sistats_recording_module }
}

impl<'a> Extract<'a> for SIStatsRecordingModule {
    const TYPE_NAME: &'static str = "SIStatsRecordingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SINotificationsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SINotificationsModule {
}

impl Pooled for SINotificationsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sinotifications_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sinotifications_module }
}

impl<'a> Extract<'a> for SINotificationsModule {
    const TYPE_NAME: &'static str = "SINotificationsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIObjectives`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIObjectives {
}

impl Pooled for SIObjectives {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.siobjectives }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.siobjectives }
}

impl<'a> Extract<'a> for SIObjectives {
    const TYPE_NAME: &'static str = "SIObjectives";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SICamerasModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SICamerasModule {
}

impl Pooled for SICamerasModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sicameras_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sicameras_module }
}

impl<'a> Extract<'a> for SICamerasModule {
    const TYPE_NAME: &'static str = "SICamerasModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIPlayerStats`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIPlayerStats {
}

impl Pooled for SIPlayerStats {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.siplayer_stats }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.siplayer_stats }
}

impl<'a> Extract<'a> for SIPlayerStats {
    const TYPE_NAME: &'static str = "SIPlayerStats";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISpawning`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISpawning {
}

impl Pooled for SISpawning {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sispawning }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sispawning }
}

impl<'a> Extract<'a> for SISpawning {
    const TYPE_NAME: &'static str = "SISpawning";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEASpawnRespawnSchedulerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEASpawnRespawnSchedulerParams {
}

impl Pooled for SEASpawnRespawnSchedulerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.seaspawn_respawn_scheduler_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.seaspawn_respawn_scheduler_params }
}

impl<'a> Extract<'a> for SEASpawnRespawnSchedulerParams {
    const TYPE_NAME: &'static str = "SEASpawnRespawnSchedulerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVictoryConditionsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVictoryConditionsModule {
}

impl Pooled for SIVictoryConditionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sivictory_conditions_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sivictory_conditions_module }
}

impl<'a> Extract<'a> for SIVictoryConditionsModule {
    const TYPE_NAME: &'static str = "SIVictoryConditionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIParamsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIParamsModule {
}

impl Pooled for SIParamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.siparams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.siparams_module }
}

impl<'a> Extract<'a> for SIParamsModule {
    const TYPE_NAME: &'static str = "SIParamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SISubsumptionMissionModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SISubsumptionMissionModule {
}

impl Pooled for SISubsumptionMissionModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sisubsumption_mission_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sisubsumption_mission_module }
}

impl<'a> Extract<'a> for SISubsumptionMissionModule {
    const TYPE_NAME: &'static str = "SISubsumptionMissionModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SChatChannelTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChatChannelTypeBase {
}

impl Pooled for SChatChannelTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.schat_channel_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.schat_channel_type_base }
}

impl<'a> Extract<'a> for SChatChannelTypeBase {
    const TYPE_NAME: &'static str = "SChatChannelTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ChatSystemOptionsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSystemOptionsModule {
}

impl Pooled for ChatSystemOptionsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.chat_system_options_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.chat_system_options_module }
}

impl<'a> Extract<'a> for ChatSystemOptionsModule {
    const TYPE_NAME: &'static str = "ChatSystemOptionsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIBettingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIBettingModule {
}

impl Pooled for SIBettingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sibetting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sibetting_module }
}

impl<'a> Extract<'a> for SIBettingModule {
    const TYPE_NAME: &'static str = "SIBettingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIDifficultyModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIDifficultyModule {
}

impl Pooled for SIDifficultyModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sidifficulty_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sidifficulty_module }
}

impl<'a> Extract<'a> for SIDifficultyModule {
    const TYPE_NAME: &'static str = "SIDifficultyModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIReputationModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIReputationModule {
}

impl Pooled for SIReputationModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sireputation_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sireputation_module }
}

impl<'a> Extract<'a> for SIReputationModule {
    const TYPE_NAME: &'static str = "SIReputationModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIStateModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIStateModule {
}

impl Pooled for SIStateModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sistate_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sistate_module }
}

impl<'a> Extract<'a> for SIStateModule {
    const TYPE_NAME: &'static str = "SIStateModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SITeamsModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SITeamsModule {
}

impl Pooled for SITeamsModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.siteams_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.siteams_module }
}

impl<'a> Extract<'a> for SITeamsModule {
    const TYPE_NAME: &'static str = "SITeamsModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SIVotingModule`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIVotingModule {
}

impl Pooled for SIVotingModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sivoting_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sivoting_module }
}

impl<'a> Extract<'a> for SIVotingModule {
    const TYPE_NAME: &'static str = "SIVotingModule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SGeometryModelTagBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryModelTagBase {
}

impl Pooled for SGeometryModelTagBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sgeometry_model_tag_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sgeometry_model_tag_base }
}

impl<'a> Extract<'a> for SGeometryModelTagBase {
    const TYPE_NAME: &'static str = "SGeometryModelTagBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SInitialDamageSpecifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInitialDamageSpecifierBase {
}

impl Pooled for SInitialDamageSpecifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sinitial_damage_specifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sinitial_damage_specifier_base }
}

impl<'a> Extract<'a> for SInitialDamageSpecifierBase {
    const TYPE_NAME: &'static str = "SInitialDamageSpecifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityComponentHealth_SBaseSequencerTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentHealth_SBaseSequencerTask {
}

impl Pooled for EntityComponentHealth_SBaseSequencerTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entity_component_health_sbase_sequencer_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entity_component_health_sbase_sequencer_task }
}

impl<'a> Extract<'a> for EntityComponentHealth_SBaseSequencerTask {
    const TYPE_NAME: &'static str = "EntityComponentHealth_SBaseSequencerTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HitBehaviorBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitBehaviorBase {
}

impl Pooled for HitBehaviorBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.hit_behavior_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.hit_behavior_base }
}

impl<'a> Extract<'a> for HitBehaviorBase {
    const TYPE_NAME: &'static str = "HitBehaviorBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `InnerThought_LayoutBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerThought_LayoutBase {
}

impl Pooled for InnerThought_LayoutBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.inner_thought_layout_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.inner_thought_layout_base }
}

impl<'a> Extract<'a> for InnerThought_LayoutBase {
    const TYPE_NAME: &'static str = "InnerThought_LayoutBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SHighlightBehaviorNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHighlightBehaviorNodeParams {
}

impl Pooled for SHighlightBehaviorNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.shighlight_behavior_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.shighlight_behavior_node_params }
}

impl<'a> Extract<'a> for SHighlightBehaviorNodeParams {
    const TYPE_NAME: &'static str = "SHighlightBehaviorNodeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SInteractionPointPrimitiveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInteractionPointPrimitiveParams {
}

impl Pooled for SInteractionPointPrimitiveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sinteraction_point_primitive_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sinteraction_point_primitive_params }
}

impl<'a> Extract<'a> for SInteractionPointPrimitiveParams {
    const TYPE_NAME: &'static str = "SInteractionPointPrimitiveParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SStateModifier {
}

impl Pooled for SStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sstate_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sstate_modifier }
}

impl<'a> Extract<'a> for SStateModifier {
    const TYPE_NAME: &'static str = "SStateModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityContextBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityContextBase {
}

impl Pooled for SEntityContextBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sentity_context_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sentity_context_base }
}

impl<'a> Extract<'a> for SEntityContextBase {
    const TYPE_NAME: &'static str = "SEntityContextBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AttachableStateModifierContextBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachableStateModifierContextBase {
}

impl Pooled for AttachableStateModifierContextBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.attachable_state_modifier_context_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.attachable_state_modifier_context_base }
}

impl<'a> Extract<'a> for AttachableStateModifierContextBase {
    const TYPE_NAME: &'static str = "AttachableStateModifierContextBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SBaseCargoUnit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBaseCargoUnit {
}

impl Pooled for SBaseCargoUnit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sbase_cargo_unit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sbase_cargo_unit }
}

impl<'a> Extract<'a> for SBaseCargoUnit {
    const TYPE_NAME: &'static str = "SBaseCargoUnit";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `InventoryContainerGridCellSizeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerGridCellSizeBase {
}

impl Pooled for InventoryContainerGridCellSizeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.inventory_container_grid_cell_size_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.inventory_container_grid_cell_size_base }
}

impl<'a> Extract<'a> for InventoryContainerGridCellSizeBase {
    const TYPE_NAME: &'static str = "InventoryContainerGridCellSizeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `InventoryContainerTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerTypeBase {
}

impl Pooled for InventoryContainerTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.inventory_container_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.inventory_container_type_base }
}

impl<'a> Extract<'a> for InventoryContainerTypeBase {
    const TYPE_NAME: &'static str = "InventoryContainerTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
}

impl Pooled for BaseItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_item }
}

impl<'a> Extract<'a> for BaseItem {
    const TYPE_NAME: &'static str = "BaseItem";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseExpirationTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseExpirationTypeParams {
}

impl Pooled for BaseExpirationTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_expiration_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_expiration_type_params }
}

impl<'a> Extract<'a> for BaseExpirationTypeParams {
    const TYPE_NAME: &'static str = "BaseExpirationTypeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemPortRuleDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemPortRuleDef {
}

impl Pooled for SItemPortRuleDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sitem_port_rule_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sitem_port_rule_def }
}

impl<'a> Extract<'a> for SItemPortRuleDef {
    const TYPE_NAME: &'static str = "SItemPortRuleDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemPortDefExtensionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemPortDefExtensionBase {
}

impl Pooled for SItemPortDefExtensionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sitem_port_def_extension_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sitem_port_def_extension_base }
}

impl<'a> Extract<'a> for SItemPortDefExtensionBase {
    const TYPE_NAME: &'static str = "SItemPortDefExtensionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SItemPortDefAttachmentImplementationBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemPortDefAttachmentImplementationBase {
}

impl Pooled for SItemPortDefAttachmentImplementationBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sitem_port_def_attachment_implementation_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sitem_port_def_attachment_implementation_base }
}

impl<'a> Extract<'a> for SItemPortDefAttachmentImplementationBase {
    const TYPE_NAME: &'static str = "SItemPortDefAttachmentImplementationBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlacementValidator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementValidator {
}

impl Pooled for PlacementValidator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.placement_validator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.placement_validator }
}

impl<'a> Extract<'a> for PlacementValidator {
    const TYPE_NAME: &'static str = "PlacementValidator";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceDynamicAmountBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceDynamicAmountBase {
}

impl Pooled for ItemResourceDynamicAmountBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_dynamic_amount_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_dynamic_amount_base }
}

impl<'a> Extract<'a> for ItemResourceDynamicAmountBase {
    const TYPE_NAME: &'static str = "ItemResourceDynamicAmountBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceConversionModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceConversionModifierBase {
}

impl Pooled for ItemResourceConversionModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_conversion_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_conversion_modifier_base }
}

impl<'a> Extract<'a> for ItemResourceConversionModifierBase {
    const TYPE_NAME: &'static str = "ItemResourceConversionModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceDynamicCompositionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceDynamicCompositionBase {
}

impl Pooled for ItemResourceDynamicCompositionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_dynamic_composition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_dynamic_composition_base }
}

impl<'a> Extract<'a> for ItemResourceDynamicCompositionBase {
    const TYPE_NAME: &'static str = "ItemResourceDynamicCompositionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceControlOutputBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceControlOutputBase {
}

impl Pooled for ItemResourceControlOutputBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_control_output_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_control_output_base }
}

impl<'a> Extract<'a> for ItemResourceControlOutputBase {
    const TYPE_NAME: &'static str = "ItemResourceControlOutputBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceControlParameterBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceControlParameterBase {
}

impl Pooled for ItemResourceControlParameterBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_control_parameter_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_control_parameter_base }
}

impl<'a> Extract<'a> for ItemResourceControlParameterBase {
    const TYPE_NAME: &'static str = "ItemResourceControlParameterBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceControlConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceControlConditionBase {
}

impl Pooled for ItemResourceControlConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_control_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_control_condition_base }
}

impl<'a> Extract<'a> for ItemResourceControlConditionBase {
    const TYPE_NAME: &'static str = "ItemResourceControlConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceDynamicResourceBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceDynamicResourceBase {
}

impl Pooled for ItemResourceDynamicResourceBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_dynamic_resource_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_dynamic_resource_base }
}

impl<'a> Extract<'a> for ItemResourceDynamicResourceBase {
    const TYPE_NAME: &'static str = "ItemResourceDynamicResourceBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SBaseResourceUnit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBaseResourceUnit {
}

impl Pooled for SBaseResourceUnit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sbase_resource_unit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sbase_resource_unit }
}

impl<'a> Extract<'a> for SBaseResourceUnit {
    const TYPE_NAME: &'static str = "SBaseResourceUnit";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemResourceDeltaBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceDeltaBase {
}

impl Pooled for ItemResourceDeltaBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_resource_delta_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_resource_delta_base }
}

impl<'a> Extract<'a> for ItemResourceDeltaBase {
    const TYPE_NAME: &'static str = "ItemResourceDeltaBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `FunctionalityModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalityModifierBase {
}

impl Pooled for FunctionalityModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.functionality_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.functionality_modifier_base }
}

impl<'a> Extract<'a> for FunctionalityModifierBase {
    const TYPE_NAME: &'static str = "FunctionalityModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseJournalEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseJournalEntry {
}

impl Pooled for BaseJournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_journal_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_journal_entry }
}

impl<'a> Extract<'a> for BaseJournalEntry {
    const TYPE_NAME: &'static str = "BaseJournalEntry";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ILightAIExtender`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ILightAIExtender {
}

impl Pooled for ILightAIExtender {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ilight_aiextender }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ilight_aiextender }
}

impl<'a> Extract<'a> for ILightAIExtender {
    const TYPE_NAME: &'static str = "ILightAIExtender";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHintCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintCondition {
}

impl Pooled for ControlHintCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.control_hint_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.control_hint_condition }
}

impl<'a> Extract<'a> for ControlHintCondition {
    const TYPE_NAME: &'static str = "ControlHintCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHintAlwaysDisplayCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintAlwaysDisplayCondition {
}

impl Pooled for ControlHintAlwaysDisplayCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.control_hint_always_display_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.control_hint_always_display_condition }
}

impl<'a> Extract<'a> for ControlHintAlwaysDisplayCondition {
    const TYPE_NAME: &'static str = "ControlHintAlwaysDisplayCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LongTermPersistenceSubTypeListOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceSubTypeListOption {
}

impl Pooled for LongTermPersistenceSubTypeListOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.long_term_persistence_sub_type_list_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.long_term_persistence_sub_type_list_option }
}

impl<'a> Extract<'a> for LongTermPersistenceSubTypeListOption {
    const TYPE_NAME: &'static str = "LongTermPersistenceSubTypeListOption";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntryOptionalData_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryOptionalData_Base {
}

impl Pooled for EntryOptionalData_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entry_optional_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entry_optional_data_base }
}

impl<'a> Extract<'a> for EntryOptionalData_Base {
    const TYPE_NAME: &'static str = "EntryOptionalData_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityClassList_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClassList_Base {
}

impl Pooled for EntityClassList_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entity_class_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entity_class_list_base }
}

impl<'a> Extract<'a> for EntityClassList_Base {
    const TYPE_NAME: &'static str = "EntityClassList_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PoolFilter_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilter_Base {
}

impl Pooled for PoolFilter_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.pool_filter_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.pool_filter_base }
}

impl<'a> Extract<'a> for PoolFilter_Base {
    const TYPE_NAME: &'static str = "PoolFilter_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PoolFilter_NoRef`
/// Inherits from: `PoolFilter_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolFilter_NoRef {
}

impl Pooled for PoolFilter_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.pool_filter_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.pool_filter_no_ref }
}

impl<'a> Extract<'a> for PoolFilter_NoRef {
    const TYPE_NAME: &'static str = "PoolFilter_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootTableV3_NoRef`
/// Inherits from: `LootTableV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableV3_NoRef {
}

impl Pooled for LootTableV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_table_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_table_v3_no_ref }
}

impl<'a> Extract<'a> for LootTableV3_NoRef {
    const TYPE_NAME: &'static str = "LootTableV3_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3_Base {
}

impl Pooled for LootArchetypeV3_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_archetype_v3_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_archetype_v3_base }
}

impl<'a> Extract<'a> for LootArchetypeV3_Base {
    const TYPE_NAME: &'static str = "LootArchetypeV3_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3_NoRef`
/// Inherits from: `LootArchetypeV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3_NoRef {
}

impl Pooled for LootArchetypeV3_NoRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_archetype_v3_no_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_archetype_v3_no_ref }
}

impl<'a> Extract<'a> for LootArchetypeV3_NoRef {
    const TYPE_NAME: &'static str = "LootArchetypeV3_NoRef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootArchetypeV3Selector_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootArchetypeV3Selector_Base {
}

impl Pooled for LootArchetypeV3Selector_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_archetype_v3_selector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_archetype_v3_selector_base }
}

impl<'a> Extract<'a> for LootArchetypeV3Selector_Base {
    const TYPE_NAME: &'static str = "LootArchetypeV3Selector_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ArchetypeOptionalDataV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeOptionalDataV3_Base {
}

impl Pooled for ArchetypeOptionalDataV3_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.archetype_optional_data_v3_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.archetype_optional_data_v3_base }
}

impl<'a> Extract<'a> for ArchetypeOptionalDataV3_Base {
    const TYPE_NAME: &'static str = "ArchetypeOptionalDataV3_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SpawnWithV3Selector_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnWithV3Selector_Base {
}

impl Pooled for SpawnWithV3Selector_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.spawn_with_v3_selector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.spawn_with_v3_selector_base }
}

impl<'a> Extract<'a> for SpawnWithV3Selector_Base {
    const TYPE_NAME: &'static str = "SpawnWithV3Selector_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootTableOptionalDataV3_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableOptionalDataV3_Base {
}

impl Pooled for LootTableOptionalDataV3_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_table_optional_data_v3_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_table_optional_data_v3_base }
}

impl<'a> Extract<'a> for LootTableOptionalDataV3_Base {
    const TYPE_NAME: &'static str = "LootTableOptionalDataV3_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootV3SecondaryChoicesRecordRef_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoicesRecordRef_Base {
}

impl Pooled for LootV3SecondaryChoicesRecordRef_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_v3_secondary_choices_record_ref_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_v3_secondary_choices_record_ref_base }
}

impl<'a> Extract<'a> for LootV3SecondaryChoicesRecordRef_Base {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoicesRecordRef_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LootV3SecondaryChoiceEntrySelector_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootV3SecondaryChoiceEntrySelector_Base {
}

impl Pooled for LootV3SecondaryChoiceEntrySelector_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loot_v3_secondary_choice_entry_selector_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loot_v3_secondary_choice_entry_selector_base }
}

impl<'a> Extract<'a> for LootV3SecondaryChoiceEntrySelector_Base {
    const TYPE_NAME: &'static str = "LootV3SecondaryChoiceEntrySelector_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `QuantityRange_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityRange_Base {
}

impl Pooled for QuantityRange_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.quantity_range_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.quantity_range_base }
}

impl<'a> Extract<'a> for QuantityRange_Base {
    const TYPE_NAME: &'static str = "QuantityRange_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MapDisplayStartModeBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDisplayStartModeBaseParams {
}

impl Pooled for MapDisplayStartModeBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.map_display_start_mode_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.map_display_start_mode_base_params }
}

impl<'a> Extract<'a> for MapDisplayStartModeBaseParams {
    const TYPE_NAME: &'static str = "MapDisplayStartModeBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Marker_ShowRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_ShowRule {
}

impl Pooled for Marker_ShowRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.marker_show_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.marker_show_rule }
}

impl<'a> Extract<'a> for Marker_ShowRule {
    const TYPE_NAME: &'static str = "Marker_ShowRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Marker_AbilityBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_AbilityBase {
}

impl Pooled for Marker_AbilityBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.marker_ability_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.marker_ability_base }
}

impl<'a> Extract<'a> for Marker_AbilityBase {
    const TYPE_NAME: &'static str = "Marker_AbilityBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `VisibilityConditionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityConditionDef {
}

impl Pooled for VisibilityConditionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.visibility_condition_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.visibility_condition_def }
}

impl<'a> Extract<'a> for VisibilityConditionDef {
    const TYPE_NAME: &'static str = "VisibilityConditionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionLocationValidation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationValidation {
}

impl Pooled for MissionLocationValidation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.mission_location_validation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.mission_location_validation }
}

impl<'a> Extract<'a> for MissionLocationValidation {
    const TYPE_NAME: &'static str = "MissionLocationValidation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractGenerationParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGenerationParamsBase {
}

impl Pooled for ContractGenerationParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.contract_generation_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.contract_generation_params_base }
}

impl<'a> Extract<'a> for ContractGenerationParamsBase {
    const TYPE_NAME: &'static str = "ContractGenerationParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SContractPlugin_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SContractPlugin_Base {
}

impl Pooled for SContractPlugin_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scontract_plugin_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scontract_plugin_base }
}

impl<'a> Extract<'a> for SContractPlugin_Base {
    const TYPE_NAME: &'static str = "SContractPlugin_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractPrerequisiteBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractPrerequisiteBase {
}

impl Pooled for ContractPrerequisiteBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.contract_prerequisite_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.contract_prerequisite_base }
}

impl<'a> Extract<'a> for ContractPrerequisiteBase {
    const TYPE_NAME: &'static str = "ContractPrerequisiteBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemAwardWeightingsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAwardWeightingsBase {
}

impl Pooled for ItemAwardWeightingsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_award_weightings_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_award_weightings_base }
}

impl<'a> Extract<'a> for ItemAwardWeightingsBase {
    const TYPE_NAME: &'static str = "ItemAwardWeightingsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractClassBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClassBase {
}

impl Pooled for ContractClassBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.contract_class_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.contract_class_base }
}

impl<'a> Extract<'a> for ContractClassBase {
    const TYPE_NAME: &'static str = "ContractClassBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HaulingOrderContentBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaulingOrderContentBase {
}

impl Pooled for HaulingOrderContentBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.hauling_order_content_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.hauling_order_content_base }
}

impl<'a> Extract<'a> for HaulingOrderContentBase {
    const TYPE_NAME: &'static str = "HaulingOrderContentBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Hauling_EntityClassListBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hauling_EntityClassListBase {
}

impl Pooled for Hauling_EntityClassListBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.hauling_entity_class_list_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.hauling_entity_class_list_base }
}

impl<'a> Extract<'a> for Hauling_EntityClassListBase {
    const TYPE_NAME: &'static str = "Hauling_EntityClassListBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `HaulingOrder_OrOption_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaulingOrder_OrOption_Base {
}

impl Pooled for HaulingOrder_OrOption_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.hauling_order_or_option_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.hauling_order_or_option_base }
}

impl<'a> Extract<'a> for HaulingOrder_OrOption_Base {
    const TYPE_NAME: &'static str = "HaulingOrder_OrOption_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionFlowConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlowConditionBase {
}

impl Pooled for MissionFlowConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.mission_flow_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.mission_flow_condition_base }
}

impl<'a> Extract<'a> for MissionFlowConditionBase {
    const TYPE_NAME: &'static str = "MissionFlowConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionFlowActionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionFlowActionBase {
}

impl Pooled for MissionFlowActionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.mission_flow_action_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.mission_flow_action_base }
}

impl<'a> Extract<'a> for MissionFlowActionBase {
    const TYPE_NAME: &'static str = "MissionFlowActionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseMissionPropertyValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMissionPropertyValue {
}

impl Pooled for BaseMissionPropertyValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_mission_property_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_mission_property_value }
}

impl<'a> Extract<'a> for BaseMissionPropertyValue {
    const TYPE_NAME: &'static str = "BaseMissionPropertyValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseDataSetMatchCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseDataSetMatchCondition {
}

impl Pooled for BaseDataSetMatchCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_data_set_match_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_data_set_match_condition }
}

impl<'a> Extract<'a> for BaseDataSetMatchCondition {
    const TYPE_NAME: &'static str = "BaseDataSetMatchCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LocationEntityType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationEntityType_Base {
}

impl Pooled for LocationEntityType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.location_entity_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.location_entity_type_base }
}

impl<'a> Extract<'a> for LocationEntityType_Base {
    const TYPE_NAME: &'static str = "LocationEntityType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ModuleDeclarationType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclarationType_Base {
}

impl Pooled for ModuleDeclarationType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.module_declaration_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.module_declaration_type_base }
}

impl<'a> Extract<'a> for ModuleDeclarationType_Base {
    const TYPE_NAME: &'static str = "ModuleDeclarationType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectivePropertyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectivePropertyBase {
}

impl Pooled for ObjectivePropertyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.objective_property_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.objective_property_base }
}

impl<'a> Extract<'a> for ObjectivePropertyBase {
    const TYPE_NAME: &'static str = "ObjectivePropertyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectiveHandlerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveHandlerBase {
}

impl Pooled for ObjectiveHandlerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.objective_handler_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.objective_handler_base }
}

impl<'a> Extract<'a> for ObjectiveHandlerBase {
    const TYPE_NAME: &'static str = "ObjectiveHandlerBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ObjectiveRewardContributionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveRewardContributionBase {
}

impl Pooled for ObjectiveRewardContributionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.objective_reward_contribution_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.objective_reward_contribution_base }
}

impl<'a> Extract<'a> for ObjectiveRewardContributionBase {
    const TYPE_NAME: &'static str = "ObjectiveRewardContributionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MusicLogicNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLogicNode {
}

impl Pooled for MusicLogicNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.music_logic_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.music_logic_node }
}

impl<'a> Extract<'a> for MusicLogicNode {
    const TYPE_NAME: &'static str = "MusicLogicNode";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SObjectDataBankEntryTrackerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SObjectDataBankEntryTrackerParams {
}

impl Pooled for SObjectDataBankEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sobject_data_bank_entry_tracker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sobject_data_bank_entry_tracker_params }
}

impl<'a> Extract<'a> for SObjectDataBankEntryTrackerParams {
    const TYPE_NAME: &'static str = "SObjectDataBankEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SandboxInfractionBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInfractionBaseDef {
}

impl Pooled for SandboxInfractionBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sandbox_infraction_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sandbox_infraction_base_def }
}

impl<'a> Extract<'a> for SandboxInfractionBaseDef {
    const TYPE_NAME: &'static str = "SandboxInfractionBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SandboxTriggerBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerBaseDef {
}

impl Pooled for SandboxTriggerBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sandbox_trigger_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sandbox_trigger_base_def }
}

impl<'a> Extract<'a> for SandboxTriggerBaseDef {
    const TYPE_NAME: &'static str = "SandboxTriggerBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MissionCompletePerkBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCompletePerkBaseDef {
}

impl Pooled for MissionCompletePerkBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.mission_complete_perk_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.mission_complete_perk_base_def }
}

impl<'a> Extract<'a> for MissionCompletePerkBaseDef {
    const TYPE_NAME: &'static str = "MissionCompletePerkBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CoolingEqualizationParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolingEqualizationParamsBase {
}

impl Pooled for CoolingEqualizationParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.cooling_equalization_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.cooling_equalization_params_base }
}

impl<'a> Extract<'a> for CoolingEqualizationParamsBase {
    const TYPE_NAME: &'static str = "CoolingEqualizationParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionBase {
}

impl Pooled for PlayerAnimatedInteractionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.player_animated_interaction_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.player_animated_interaction_base }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionBase {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlayerChoiceMenuOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoiceMenuOption {
}

impl Pooled for PlayerChoiceMenuOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.player_choice_menu_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.player_choice_menu_option }
}

impl<'a> Extract<'a> for PlayerChoiceMenuOption {
    const TYPE_NAME: &'static str = "PlayerChoiceMenuOption";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PlanetDayNightTemperatureBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetDayNightTemperatureBaseParams {
}

impl Pooled for PlanetDayNightTemperatureBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.planet_day_night_temperature_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.planet_day_night_temperature_base_params }
}

impl<'a> Extract<'a> for PlanetDayNightTemperatureBaseParams {
    const TYPE_NAME: &'static str = "PlanetDayNightTemperatureBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ProceduralLayoutNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutNode_Base {
}

impl Pooled for ProceduralLayoutNode_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.procedural_layout_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.procedural_layout_node_base }
}

impl<'a> Extract<'a> for ProceduralLayoutNode_Base {
    const TYPE_NAME: &'static str = "ProceduralLayoutNode_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SQuantumDriveEffectBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumDriveEffectBaseParams {
}

impl Pooled for SQuantumDriveEffectBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.squantum_drive_effect_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.squantum_drive_effect_base_params }
}

impl<'a> Extract<'a> for SQuantumDriveEffectBaseParams {
    const TYPE_NAME: &'static str = "SQuantumDriveEffectBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemRadarSensitivityModifierType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemRadarSensitivityModifierType {
}

impl Pooled for SCItemRadarSensitivityModifierType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_radar_sensitivity_modifier_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_radar_sensitivity_modifier_type }
}

impl<'a> Extract<'a> for SCItemRadarSensitivityModifierType {
    const TYPE_NAME: &'static str = "SCItemRadarSensitivityModifierType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ScanCustomValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCustomValue {
}

impl Pooled for ScanCustomValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scan_custom_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scan_custom_value }
}

impl<'a> Extract<'a> for ScanCustomValue {
    const TYPE_NAME: &'static str = "ScanCustomValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ScanDisplayConditionBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDisplayConditionBaseParams {
}

impl Pooled for ScanDisplayConditionBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scan_display_condition_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scan_display_condition_base_params }
}

impl<'a> Extract<'a> for ScanDisplayConditionBaseParams {
    const TYPE_NAME: &'static str = "ScanDisplayConditionBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SBBDynamicPropertyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBBDynamicPropertyBase {
}

impl Pooled for SBBDynamicPropertyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sbbdynamic_property_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sbbdynamic_property_base }
}

impl<'a> Extract<'a> for SBBDynamicPropertyBase {
    const TYPE_NAME: &'static str = "SBBDynamicPropertyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationStateModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateModifierBase {
}

impl Pooled for SReputationStateModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sreputation_state_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sreputation_state_modifier_base }
}

impl<'a> Extract<'a> for SReputationStateModifierBase {
    const TYPE_NAME: &'static str = "SReputationStateModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationMissionRequirementExpressionElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRequirementExpressionElement {
}

impl Pooled for SReputationMissionRequirementExpressionElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sreputation_mission_requirement_expression_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sreputation_mission_requirement_expression_element }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpressionElement {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpressionElement";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseDensityUnit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseDensityUnit {
}

impl Pooled for BaseDensityUnit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_density_unit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_density_unit }
}

impl<'a> Extract<'a> for BaseDensityUnit {
    const TYPE_NAME: &'static str = "BaseDensityUnit";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ResourceTypeDensityType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeDensityType {
}

impl Pooled for ResourceTypeDensityType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.resource_type_density_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.resource_type_density_type }
}

impl<'a> Extract<'a> for ResourceTypeDensityType {
    const TYPE_NAME: &'static str = "ResourceTypeDensityType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `RoomExtension`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomExtension {
}

impl Pooled for RoomExtension {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.room_extension }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.room_extension }
}

impl<'a> Extract<'a> for RoomExtension {
    const TYPE_NAME: &'static str = "RoomExtension";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `VolumeShape`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeShape {
}

impl Pooled for VolumeShape {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.volume_shape }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.volume_shape }
}

impl<'a> Extract<'a> for VolumeShape {
    const TYPE_NAME: &'static str = "VolumeShape";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SAtmosphericCompositionBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAtmosphericCompositionBaseParams {
}

impl Pooled for SAtmosphericCompositionBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.satmospheric_composition_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.satmospheric_composition_base_params }
}

impl<'a> Extract<'a> for SAtmosphericCompositionBaseParams {
    const TYPE_NAME: &'static str = "SAtmosphericCompositionBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AsteroidStateBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidStateBase {
}

impl Pooled for AsteroidStateBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.asteroid_state_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.asteroid_state_base }
}

impl<'a> Extract<'a> for AsteroidStateBase {
    const TYPE_NAME: &'static str = "AsteroidStateBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AtmosphereStateBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereStateBase {
}

impl Pooled for AtmosphereStateBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.atmosphere_state_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.atmosphere_state_base }
}

impl<'a> Extract<'a> for AtmosphereStateBase {
    const TYPE_NAME: &'static str = "AtmosphereStateBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AerodynamicTrailCalculation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AerodynamicTrailCalculation {
}

impl Pooled for AerodynamicTrailCalculation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.aerodynamic_trail_calculation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.aerodynamic_trail_calculation }
}

impl<'a> Extract<'a> for AerodynamicTrailCalculation {
    const TYPE_NAME: &'static str = "AerodynamicTrailCalculation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ElectricalStateBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalStateBase {
}

impl Pooled for ElectricalStateBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.electrical_state_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.electrical_state_base }
}

impl<'a> Extract<'a> for ElectricalStateBase {
    const TYPE_NAME: &'static str = "ElectricalStateBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `RadiationStateBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationStateBase {
}

impl Pooled for RadiationStateBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.radiation_state_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.radiation_state_base }
}

impl<'a> Extract<'a> for RadiationStateBase {
    const TYPE_NAME: &'static str = "RadiationStateBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBaseInteractionGameplayTrigger {
}

impl Pooled for SBaseInteractionGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sbase_interaction_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sbase_interaction_gameplay_trigger }
}

impl<'a> Extract<'a> for SBaseInteractionGameplayTrigger {
    const TYPE_NAME: &'static str = "SBaseInteractionGameplayTrigger";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SDisruptionGameplayTriggerType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDisruptionGameplayTriggerType {
}

impl Pooled for SDisruptionGameplayTriggerType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sdisruption_gameplay_trigger_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sdisruption_gameplay_trigger_type }
}

impl<'a> Extract<'a> for SDisruptionGameplayTriggerType {
    const TYPE_NAME: &'static str = "SDisruptionGameplayTriggerType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DegradationAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationAction {
}

impl Pooled for DegradationAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.degradation_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.degradation_action }
}

impl<'a> Extract<'a> for DegradationAction {
    const TYPE_NAME: &'static str = "DegradationAction";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LoadoutCheckType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutCheckType {
}

impl Pooled for LoadoutCheckType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loadout_check_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loadout_check_type }
}

impl<'a> Extract<'a> for LoadoutCheckType {
    const TYPE_NAME: &'static str = "LoadoutCheckType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCheckType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCheckType {
}

impl Pooled for SCheckType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scheck_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scheck_type }
}

impl<'a> Extract<'a> for SCheckType {
    const TYPE_NAME: &'static str = "SCheckType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_TargetType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_TargetType_Base {
}

impl Pooled for GameplayTrigger_TargetType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_target_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_target_type_base }
}

impl<'a> Extract<'a> for GameplayTrigger_TargetType_Base {
    const TYPE_NAME: &'static str = "GameplayTrigger_TargetType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_FilterType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_FilterType_Base {
}

impl Pooled for GameplayTrigger_FilterType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_filter_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_filter_type_base }
}

impl<'a> Extract<'a> for GameplayTrigger_FilterType_Base {
    const TYPE_NAME: &'static str = "GameplayTrigger_FilterType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_Executor_ActivateInteraction_Base`
/// Inherits from: `GameplayTrigger_Executor_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_Executor_ActivateInteraction_Base {
}

impl Pooled for GameplayTrigger_Executor_ActivateInteraction_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_executor_activate_interaction_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_executor_activate_interaction_base }
}

impl<'a> Extract<'a> for GameplayTrigger_Executor_ActivateInteraction_Base {
    const TYPE_NAME: &'static str = "GameplayTrigger_Executor_ActivateInteraction_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_Executor_SetInteractionState_Base`
/// Inherits from: `GameplayTrigger_Executor_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_Executor_SetInteractionState_Base {
}

impl Pooled for GameplayTrigger_Executor_SetInteractionState_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_executor_set_interaction_state_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_executor_set_interaction_state_base }
}

impl<'a> Extract<'a> for GameplayTrigger_Executor_SetInteractionState_Base {
    const TYPE_NAME: &'static str = "GameplayTrigger_Executor_SetInteractionState_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GT_CommunicationMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GT_CommunicationMessage {
}

impl Pooled for GT_CommunicationMessage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gt_communication_message }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gt_communication_message }
}

impl<'a> Extract<'a> for GT_CommunicationMessage {
    const TYPE_NAME: &'static str = "GT_CommunicationMessage";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GT_CommunicationTarget`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GT_CommunicationTarget {
}

impl Pooled for GT_CommunicationTarget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gt_communication_target }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gt_communication_target }
}

impl<'a> Extract<'a> for GT_CommunicationTarget {
    const TYPE_NAME: &'static str = "GT_CommunicationTarget";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `EntityFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityFilter {
}

impl Pooled for EntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.entity_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.entity_filter }
}

impl<'a> Extract<'a> for EntityFilter {
    const TYPE_NAME: &'static str = "EntityFilter";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AreaCommunicationMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaCommunicationMessage {
}

impl Pooled for AreaCommunicationMessage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.area_communication_message }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.area_communication_message }
}

impl<'a> Extract<'a> for AreaCommunicationMessage {
    const TYPE_NAME: &'static str = "AreaCommunicationMessage";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SelfCommunicationMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCommunicationMessage {
}

impl Pooled for SelfCommunicationMessage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.self_communication_message }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.self_communication_message }
}

impl<'a> Extract<'a> for SelfCommunicationMessage {
    const TYPE_NAME: &'static str = "SelfCommunicationMessage";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTriggerCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTriggerCondition {
}

impl Pooled for GameplayTriggerCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_condition }
}

impl<'a> Extract<'a> for GameplayTriggerCondition {
    const TYPE_NAME: &'static str = "GameplayTriggerCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ConditionalResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalResult {
}

impl Pooled for ConditionalResult {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.conditional_result }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.conditional_result }
}

impl<'a> Extract<'a> for ConditionalResult {
    const TYPE_NAME: &'static str = "ConditionalResult";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TriggeredHealth`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggeredHealth {
}

impl Pooled for TriggeredHealth {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.triggered_health }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.triggered_health }
}

impl<'a> Extract<'a> for TriggeredHealth {
    const TYPE_NAME: &'static str = "TriggeredHealth";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SModHealth`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SModHealth {
}

impl Pooled for SModHealth {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.smod_health }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.smod_health }
}

impl<'a> Extract<'a> for SModHealth {
    const TYPE_NAME: &'static str = "SModHealth";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `VulnerabilityState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityState {
}

impl Pooled for VulnerabilityState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.vulnerability_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.vulnerability_state }
}

impl<'a> Extract<'a> for VulnerabilityState {
    const TYPE_NAME: &'static str = "VulnerabilityState";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_Physics_SetParameter_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_Physics_SetParameter_Base {
}

impl Pooled for GameplayTrigger_Physics_SetParameter_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_physics_set_parameter_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_physics_set_parameter_base }
}

impl<'a> Extract<'a> for GameplayTrigger_Physics_SetParameter_Base {
    const TYPE_NAME: &'static str = "GameplayTrigger_Physics_SetParameter_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SUserVariableOperationType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SUserVariableOperationType {
}

impl Pooled for SUserVariableOperationType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.suser_variable_operation_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.suser_variable_operation_type }
}

impl<'a> Extract<'a> for SUserVariableOperationType {
    const TYPE_NAME: &'static str = "SUserVariableOperationType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `GameplayTrigger_InterpolationType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_InterpolationType {
}

impl Pooled for GameplayTrigger_InterpolationType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.gameplay_trigger_interpolation_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.gameplay_trigger_interpolation_type }
}

impl<'a> Extract<'a> for GameplayTrigger_InterpolationType {
    const TYPE_NAME: &'static str = "GameplayTrigger_InterpolationType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ClothingType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClothingType {
}

impl Pooled for ClothingType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.clothing_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.clothing_type }
}

impl<'a> Extract<'a> for ClothingType {
    const TYPE_NAME: &'static str = "ClothingType";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `PurchasableDisplayBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchasableDisplayBase {
}

impl Pooled for PurchasableDisplayBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.purchasable_display_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.purchasable_display_base }
}

impl<'a> Extract<'a> for PurchasableDisplayBase {
    const TYPE_NAME: &'static str = "PurchasableDisplayBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemProximitySensorShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemProximitySensorShapeParams {
}

impl Pooled for SCItemProximitySensorShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_proximity_sensor_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_proximity_sensor_shape_params }
}

impl<'a> Extract<'a> for SCItemProximitySensorShapeParams {
    const TYPE_NAME: &'static str = "SCItemProximitySensorShapeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SDoorCollisionReactionBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDoorCollisionReactionBaseParams {
}

impl Pooled for SDoorCollisionReactionBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sdoor_collision_reaction_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sdoor_collision_reaction_base_params }
}

impl<'a> Extract<'a> for SDoorCollisionReactionBaseParams {
    const TYPE_NAME: &'static str = "SDoorCollisionReactionBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemDoorPortalModeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemDoorPortalModeParams {
}

impl Pooled for SCItemDoorPortalModeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_door_portal_mode_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_door_portal_mode_params }
}

impl<'a> Extract<'a> for SCItemDoorPortalModeParams {
    const TYPE_NAME: &'static str = "SCItemDoorPortalModeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerItemDoorTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerItemDoorTaskParams {
}

impl Pooled for SSequencerItemDoorTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_item_door_task_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_item_door_task_params }
}

impl<'a> Extract<'a> for SSequencerItemDoorTaskParams {
    const TYPE_NAME: &'static str = "SSequencerItemDoorTaskParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AccessibilityBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityBaseParams {
}

impl Pooled for AccessibilityBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.accessibility_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.accessibility_base_params }
}

impl<'a> Extract<'a> for AccessibilityBaseParams {
    const TYPE_NAME: &'static str = "AccessibilityBaseParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SElevatorBaseCollisionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SElevatorBaseCollisionParams {
}

impl Pooled for SElevatorBaseCollisionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.selevator_base_collision_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.selevator_base_collision_params }
}

impl<'a> Extract<'a> for SElevatorBaseCollisionParams {
    const TYPE_NAME: &'static str = "SElevatorBaseCollisionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BaseHoloDisplayProvider`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseHoloDisplayProvider {
}

impl Pooled for BaseHoloDisplayProvider {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.base_holo_display_provider }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.base_holo_display_provider }
}

impl<'a> Extract<'a> for BaseHoloDisplayProvider {
    const TYPE_NAME: &'static str = "BaseHoloDisplayProvider";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WorldDisplayEnvironmentColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldDisplayEnvironmentColor {
}

impl Pooled for WorldDisplayEnvironmentColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.world_display_environment_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.world_display_environment_color }
}

impl<'a> Extract<'a> for WorldDisplayEnvironmentColor {
    const TYPE_NAME: &'static str = "WorldDisplayEnvironmentColor";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SToolArmDeployCondition_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SToolArmDeployCondition_Base {
}

impl Pooled for SToolArmDeployCondition_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.stool_arm_deploy_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.stool_arm_deploy_condition_base }
}

impl<'a> Extract<'a> for SToolArmDeployCondition_Base {
    const TYPE_NAME: &'static str = "SToolArmDeployCondition_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SCItemTurretAngleLimitParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemTurretAngleLimitParams {
}

impl Pooled for SCItemTurretAngleLimitParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.scitem_turret_angle_limit_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.scitem_turret_angle_limit_params }
}

impl<'a> Extract<'a> for SCItemTurretAngleLimitParams {
    const TYPE_NAME: &'static str = "SCItemTurretAngleLimitParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WeaponAIAimingMethod`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAIAimingMethod {
}

impl Pooled for WeaponAIAimingMethod {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.weapon_aiaiming_method }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.weapon_aiaiming_method }
}

impl<'a> Extract<'a> for WeaponAIAimingMethod {
    const TYPE_NAME: &'static str = "WeaponAIAimingMethod";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SLauncherBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLauncherBase {
}

impl Pooled for SLauncherBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.slauncher_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.slauncher_base }
}

impl<'a> Extract<'a> for SLauncherBase {
    const TYPE_NAME: &'static str = "SLauncherBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SWeaponConditionBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponConditionBase {
}

impl Pooled for SWeaponConditionBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sweapon_condition_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sweapon_condition_base }
}

impl<'a> Extract<'a> for SWeaponConditionBase {
    const TYPE_NAME: &'static str = "SWeaponConditionBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SAuxiliaryWeaponActionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAuxiliaryWeaponActionParams {
}

impl Pooled for SAuxiliaryWeaponActionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sauxiliary_weapon_action_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sauxiliary_weapon_action_params }
}

impl<'a> Extract<'a> for SAuxiliaryWeaponActionParams {
    const TYPE_NAME: &'static str = "SAuxiliaryWeaponActionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemRecoverySetConditionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecoverySetConditionDef {
}

impl Pooled for ItemRecoverySetConditionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_recovery_set_condition_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_recovery_set_condition_def }
}

impl<'a> Extract<'a> for ItemRecoverySetConditionDef {
    const TYPE_NAME: &'static str = "ItemRecoverySetConditionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureEmissionBaseModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureEmissionBaseModifier {
}

impl Pooled for SSCSignatureEmissionBaseModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sscsignature_emission_base_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sscsignature_emission_base_modifier }
}

impl<'a> Extract<'a> for SSCSignatureEmissionBaseModifier {
    const TYPE_NAME: &'static str = "SSCSignatureEmissionBaseModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureParamsBase {
}

impl Pooled for SSCSignatureParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sscsignature_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sscsignature_params_base }
}

impl<'a> Extract<'a> for SSCSignatureParamsBase {
    const TYPE_NAME: &'static str = "SSCSignatureParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemCrossSectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemCrossSectionParams {
}

impl Pooled for SSCSignatureSystemCrossSectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sscsignature_system_cross_section_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sscsignature_system_cross_section_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemCrossSectionParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemCrossSectionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioModifier {
}

impl Pooled for SSCSignatureSystemAudioModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sscsignature_system_audio_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sscsignature_system_audio_modifier }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioModifier {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioSubRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioSubRule {
}

impl Pooled for SSCSignatureSystemAudioSubRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sscsignature_system_audio_sub_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sscsignature_system_audio_sub_rule }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioSubRule {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioSubRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CompletionTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTypeBase {
}

impl Pooled for CompletionTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.completion_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.completion_type_base }
}

impl<'a> Extract<'a> for CompletionTypeBase {
    const TYPE_NAME: &'static str = "CompletionTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ScreenEffects_ParamValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenEffects_ParamValue {
}

impl Pooled for ScreenEffects_ParamValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.screen_effects_param_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.screen_effects_param_value }
}

impl<'a> Extract<'a> for ScreenEffects_ParamValue {
    const TYPE_NAME: &'static str = "ScreenEffects_ParamValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityClearance_OutfitRequirementDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityClearance_OutfitRequirementDef {
}

impl Pooled for SecurityClearance_OutfitRequirementDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.security_clearance_outfit_requirement_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.security_clearance_outfit_requirement_def }
}

impl<'a> Extract<'a> for SecurityClearance_OutfitRequirementDef {
    const TYPE_NAME: &'static str = "SecurityClearance_OutfitRequirementDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityManualInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityManualInput {
}

impl Pooled for SecurityManualInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.security_manual_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.security_manual_input }
}

impl<'a> Extract<'a> for SecurityManualInput {
    const TYPE_NAME: &'static str = "SecurityManualInput";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNotifications`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNotifications {
}

impl Pooled for SecurityNotifications {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.security_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.security_notifications }
}

impl<'a> Extract<'a> for SecurityNotifications {
    const TYPE_NAME: &'static str = "SecurityNotifications";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNetworkVariableValue_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariableValue_Base {
}

impl Pooled for SecurityNetworkVariableValue_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.security_network_variable_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.security_network_variable_value_base }
}

impl<'a> Extract<'a> for SecurityNetworkVariableValue_Base {
    const TYPE_NAME: &'static str = "SecurityNetworkVariableValue_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SecurityNetworkVariableEffect_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityNetworkVariableEffect_Base {
}

impl Pooled for SecurityNetworkVariableEffect_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.security_network_variable_effect_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.security_network_variable_effect_base }
}

impl<'a> Extract<'a> for SecurityNetworkVariableEffect_Base {
    const TYPE_NAME: &'static str = "SecurityNetworkVariableEffect_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServiceBeaconCreatorParamsBase {
}

impl Pooled for SServiceBeaconCreatorParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sservice_beacon_creator_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sservice_beacon_creator_params_base }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParamsBase {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TimeValue_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeValue_Base {
}

impl Pooled for TimeValue_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.time_value_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.time_value_base }
}

impl<'a> Extract<'a> for TimeValue_Base {
    const TYPE_NAME: &'static str = "TimeValue_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TransportPermissionsInterface`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportPermissionsInterface {
}

impl Pooled for TransportPermissionsInterface {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.transport_permissions_interface }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.transport_permissions_interface }
}

impl<'a> Extract<'a> for TransportPermissionsInterface {
    const TYPE_NAME: &'static str = "TransportPermissionsInterface";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemPortViewInformation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPortViewInformation {
}

impl Pooled for ItemPortViewInformation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.item_port_view_information }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.item_port_view_information }
}

impl<'a> Extract<'a> for ItemPortViewInformation {
    const TYPE_NAME: &'static str = "ItemPortViewInformation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LoadoutEditorAdditionalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutEditorAdditionalParams {
}

impl Pooled for LoadoutEditorAdditionalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.loadout_editor_additional_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.loadout_editor_additional_params }
}

impl<'a> Extract<'a> for LoadoutEditorAdditionalParams {
    const TYPE_NAME: &'static str = "LoadoutEditorAdditionalParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AreaAlignmentSlotTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaAlignmentSlotTypeParams {
}

impl Pooled for AreaAlignmentSlotTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.area_alignment_slot_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.area_alignment_slot_type_params }
}

impl<'a> Extract<'a> for AreaAlignmentSlotTypeParams {
    const TYPE_NAME: &'static str = "AreaAlignmentSlotTypeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSpecializedDataEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpecializedDataEntry {
}

impl Pooled for SSpecializedDataEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.sspecialized_data_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.sspecialized_data_entry }
}

impl<'a> Extract<'a> for SSpecializedDataEntry {
    const TYPE_NAME: &'static str = "SSpecializedDataEntry";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `UsableSlottingReferenceElementBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsableSlottingReferenceElementBase {
}

impl Pooled for UsableSlottingReferenceElementBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.usable_slotting_reference_element_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.usable_slotting_reference_element_base }
}

impl<'a> Extract<'a> for UsableSlottingReferenceElementBase {
    const TYPE_NAME: &'static str = "UsableSlottingReferenceElementBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSequencerUsableTask`
/// Inherits from: `SSequencerDefTaskParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSequencerUsableTask {
}

impl Pooled for SSequencerUsableTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.ssequencer_usable_task }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.ssequencer_usable_task }
}

impl<'a> Extract<'a> for SSequencerUsableTask {
    const TYPE_NAME: &'static str = "SSequencerUsableTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WeatherEffects_Asteroid`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Asteroid {
}

impl Pooled for WeatherEffects_Asteroid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.weather_effects_asteroid }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.weather_effects_asteroid }
}

impl<'a> Extract<'a> for WeatherEffects_Asteroid {
    const TYPE_NAME: &'static str = "WeatherEffects_Asteroid";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `WeatherEffects_Atmosphere`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Atmosphere {
}

impl Pooled for WeatherEffects_Atmosphere {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.weather_effects_atmosphere }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.weather_effects_atmosphere }
}

impl<'a> Extract<'a> for WeatherEffects_Atmosphere {
    const TYPE_NAME: &'static str = "WeatherEffects_Atmosphere";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MobiGlasAppDataBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasAppDataBase {
}

impl Pooled for MobiGlasAppDataBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.mobi_glas_app_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.mobi_glas_app_data_base }
}

impl<'a> Extract<'a> for MobiGlasAppDataBase {
    const TYPE_NAME: &'static str = "MobiGlasAppDataBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMobiGlasAppDataPacketBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMobiGlasAppDataPacketBase {
}

impl Pooled for SMobiGlasAppDataPacketBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.smobi_glas_app_data_packet_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.smobi_glas_app_data_packet_base }
}

impl<'a> Extract<'a> for SMobiGlasAppDataPacketBase {
    const TYPE_NAME: &'static str = "SMobiGlasAppDataPacketBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMobiGlasAppParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMobiGlasAppParamsBase {
}

impl Pooled for SMobiGlasAppParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.core.smobi_glas_app_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.core.smobi_glas_app_params_base }
}

impl<'a> Extract<'a> for SMobiGlasAppParamsBase {
    const TYPE_NAME: &'static str = "SMobiGlasAppParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

