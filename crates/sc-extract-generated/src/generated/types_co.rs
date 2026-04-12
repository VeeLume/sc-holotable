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

/// DCB type: `CostModifierPerAgentType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostModifierPerAgentType {
    /// DCB field: `agentType` (EnumChoice)
    #[serde(default)]
    pub agent_type: String,
    /// DCB field: `traversalCostVariants` (Class (array))
    #[serde(default)]
    pub traversal_cost_variants: Vec<Handle<TraversalCostConditionTags>>,
    /// DCB field: `defaultCostMultiplier` (Single)
    #[serde(default)]
    pub default_cost_multiplier: f32,
    /// DCB field: `blocksTraversability` (Boolean)
    #[serde(default)]
    pub blocks_traversability: bool,
    /// DCB field: `blocksStopping` (Boolean)
    #[serde(default)]
    pub blocks_stopping: bool,
}

impl Pooled for CostModifierPerAgentType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.cost_modifier_per_agent_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.cost_modifier_per_agent_type }
}

impl<'a> Extract<'a> for CostModifierPerAgentType {
    const TYPE_NAME: &'static str = "CostModifierPerAgentType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            agent_type: inst.get_str("agentType").map(String::from).unwrap_or_default(),
            traversal_cost_variants: inst.get_array("traversalCostVariants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TraversalCostConditionTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TraversalCostConditionTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_cost_multiplier: inst.get_f32("defaultCostMultiplier").unwrap_or_default(),
            blocks_traversability: inst.get_bool("blocksTraversability").unwrap_or_default(),
            blocks_stopping: inst.get_bool("blocksStopping").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonTargetingSameTargetScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTargetingSameTargetScore {
    /// DCB field: `targetedByOneNPC` (Single)
    #[serde(default)]
    pub targeted_by_one_npc: f32,
    /// DCB field: `targetedByTwoNPC` (Single)
    #[serde(default)]
    pub targeted_by_two_npc: f32,
    /// DCB field: `targetedByThreeNPC` (Single)
    #[serde(default)]
    pub targeted_by_three_npc: f32,
    /// DCB field: `targetedByFourNPC` (Single)
    #[serde(default)]
    pub targeted_by_four_npc: f32,
    /// DCB field: `targetedByFiveNPC` (Single)
    #[serde(default)]
    pub targeted_by_five_npc: f32,
}

impl Pooled for CommonTargetingSameTargetScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.common_targeting_same_target_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.common_targeting_same_target_score }
}

impl<'a> Extract<'a> for CommonTargetingSameTargetScore {
    const TYPE_NAME: &'static str = "CommonTargetingSameTargetScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            targeted_by_one_npc: inst.get_f32("targetedByOneNPC").unwrap_or_default(),
            targeted_by_two_npc: inst.get_f32("targetedByTwoNPC").unwrap_or_default(),
            targeted_by_three_npc: inst.get_f32("targetedByThreeNPC").unwrap_or_default(),
            targeted_by_four_npc: inst.get_f32("targetedByFourNPC").unwrap_or_default(),
            targeted_by_five_npc: inst.get_f32("targetedByFiveNPC").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonTargetVisibilityScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTargetVisibilityScore {
    /// DCB field: `isVisible` (Single)
    #[serde(default)]
    pub is_visible: f32,
    /// DCB field: `isNotVisible` (Single)
    #[serde(default)]
    pub is_not_visible: f32,
}

impl Pooled for CommonTargetVisibilityScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.common_target_visibility_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.common_target_visibility_score }
}

impl<'a> Extract<'a> for CommonTargetVisibilityScore {
    const TYPE_NAME: &'static str = "CommonTargetVisibilityScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_visible: inst.get_f32("isVisible").unwrap_or_default(),
            is_not_visible: inst.get_f32("isNotVisible").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonCurrentTargetDistanceScore`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonCurrentTargetDistanceScore {
    /// DCB field: `lowDistToTarget` (Single)
    #[serde(default)]
    pub low_dist_to_target: f32,
    /// DCB field: `mediumDistToTarget` (Single)
    #[serde(default)]
    pub medium_dist_to_target: f32,
    /// DCB field: `highDistToTarget` (Single)
    #[serde(default)]
    pub high_dist_to_target: f32,
}

impl Pooled for CommonCurrentTargetDistanceScore {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.common_current_target_distance_score }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.common_current_target_distance_score }
}

impl<'a> Extract<'a> for CommonCurrentTargetDistanceScore {
    const TYPE_NAME: &'static str = "CommonCurrentTargetDistanceScore";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            low_dist_to_target: inst.get_f32("lowDistToTarget").unwrap_or_default(),
            medium_dist_to_target: inst.get_f32("mediumDistToTarget").unwrap_or_default(),
            high_dist_to_target: inst.get_f32("highDistToTarget").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommonTacticScores`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonTacticScores {
    /// DCB field: `tacticName` (String)
    #[serde(default)]
    pub tactic_name: String,
    /// DCB field: `amountOfEntitiesTargetingSameTargetScore` (Class)
    #[serde(default)]
    pub amount_of_entities_targeting_same_target_score: Option<Handle<CommonTargetingSameTargetScore>>,
    /// DCB field: `isCurrentTargetVisibleScore` (Class)
    #[serde(default)]
    pub is_current_target_visible_score: Option<Handle<CommonTargetVisibilityScore>>,
    /// DCB field: `currentDistanceToTargetScore` (Class)
    #[serde(default)]
    pub current_distance_to_target_score: Option<Handle<CommonCurrentTargetDistanceScore>>,
}

impl Pooled for CommonTacticScores {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.common_tactic_scores }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.common_tactic_scores }
}

impl<'a> Extract<'a> for CommonTacticScores {
    const TYPE_NAME: &'static str = "CommonTacticScores";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tactic_name: inst.get_str("tacticName").map(String::from).unwrap_or_default(),
            amount_of_entities_targeting_same_target_score: match inst.get("amountOfEntitiesTargetingSameTargetScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonTargetingSameTargetScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonTargetingSameTargetScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_current_target_visible_score: match inst.get("isCurrentTargetVisibleScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonTargetVisibilityScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonTargetVisibilityScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            current_distance_to_target_score: match inst.get("currentDistanceToTargetScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommonCurrentTargetDistanceScore>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommonCurrentTargetDistanceScore>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ConsumableParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableParams {
    /// DCB field: `liquidOneShotConsumptionRate` (Single)
    #[serde(default)]
    pub liquid_one_shot_consumption_rate: f32,
    /// DCB field: `liquidOneShotVisualConsumptionTime` (Single)
    #[serde(default)]
    pub liquid_one_shot_visual_consumption_time: f32,
    /// DCB field: `liquidContinuousConsumptionRate` (Single)
    #[serde(default)]
    pub liquid_continuous_consumption_rate: f32,
}

impl Pooled for ConsumableParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.consumable_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.consumable_params }
}

impl<'a> Extract<'a> for ConsumableParams {
    const TYPE_NAME: &'static str = "ConsumableParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            liquid_one_shot_consumption_rate: inst.get_f32("liquidOneShotConsumptionRate").unwrap_or_default(),
            liquid_one_shot_visual_consumption_time: inst.get_f32("liquidOneShotVisualConsumptionTime").unwrap_or_default(),
            liquid_continuous_consumption_rate: inst.get_f32("liquidContinuousConsumptionRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `CombatMarker`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatMarker {
    /// DCB field: `objectName` (String)
    #[serde(default)]
    pub object_name: String,
    /// DCB field: `minimumScale` (Single)
    #[serde(default)]
    pub minimum_scale: f32,
    /// DCB field: `inverseScaleMultiplier` (Single)
    #[serde(default)]
    pub inverse_scale_multiplier: f32,
    /// DCB field: `hitAnimTotalTime` (Single)
    #[serde(default)]
    pub hit_anim_total_time: f32,
    /// DCB field: `hitAnimationOffsetSize` (Single)
    #[serde(default)]
    pub hit_animation_offset_size: f32,
    /// DCB field: `easeType` (EnumChoice)
    #[serde(default)]
    pub ease_type: String,
    /// DCB field: `textOffset` (Single)
    #[serde(default)]
    pub text_offset: f32,
    /// DCB field: `introAnimTime` (Single)
    #[serde(default)]
    pub intro_anim_time: f32,
    /// DCB field: `introAnimPitchRotationFrequency` (Single)
    #[serde(default)]
    pub intro_anim_pitch_rotation_frequency: f32,
    /// DCB field: `introAnimYawRotationFrequency` (Single)
    #[serde(default)]
    pub intro_anim_yaw_rotation_frequency: f32,
    /// DCB field: `introAnimRollRotationFrequency` (Single)
    #[serde(default)]
    pub intro_anim_roll_rotation_frequency: f32,
    /// DCB field: `introAnimEaseType` (EnumChoice)
    #[serde(default)]
    pub intro_anim_ease_type: String,
    /// DCB field: `introStartingScale` (Single)
    #[serde(default)]
    pub intro_starting_scale: f32,
    /// DCB field: `introStartingOffsetScale` (Single)
    #[serde(default)]
    pub intro_starting_offset_scale: f32,
    /// DCB field: `introAnimOffset` (Single)
    #[serde(default)]
    pub intro_anim_offset: f32,
    /// DCB field: `transitionAnimLength` (Single)
    #[serde(default)]
    pub transition_anim_length: f32,
    /// DCB field: `transitionAnimEaseType` (EnumChoice)
    #[serde(default)]
    pub transition_anim_ease_type: String,
    /// DCB field: `rotationalAnimationClamp` (Single)
    #[serde(default)]
    pub rotational_animation_clamp: f32,
    /// DCB field: `rotationalAnimationIntegrationTime` (Single)
    #[serde(default)]
    pub rotational_animation_integration_time: f32,
    /// DCB field: `signalLostAnimationTime` (Single)
    #[serde(default)]
    pub signal_lost_animation_time: f32,
    /// DCB field: `signalLostAnimationPulseFrequency` (Single)
    #[serde(default)]
    pub signal_lost_animation_pulse_frequency: f32,
    /// DCB field: `unfocusedObjectName` (String)
    #[serde(default)]
    pub unfocused_object_name: String,
    /// DCB field: `unfocusedMarkerScale` (Single)
    #[serde(default)]
    pub unfocused_marker_scale: f32,
    /// DCB field: `hitAnimationColor` (Class)
    #[serde(default)]
    pub hit_animation_color: Option<Handle<RGB>>,
    /// DCB field: `hitAnimationFlickerTime` (Single)
    #[serde(default)]
    pub hit_animation_flicker_time: f32,
    /// DCB field: `hitAnimOffsetFactor` (Single)
    #[serde(default)]
    pub hit_anim_offset_factor: f32,
    /// DCB field: `transitionScaleCurve` (Class)
    #[serde(default)]
    pub transition_scale_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `gainedFocusAnimTotalTime` (Single)
    #[serde(default)]
    pub gained_focus_anim_total_time: f32,
    /// DCB field: `gainedFocusAnimFlickerTime` (Single)
    #[serde(default)]
    pub gained_focus_anim_flicker_time: f32,
    /// DCB field: `unFocusedRotationFactor` (Single)
    #[serde(default)]
    pub un_focused_rotation_factor: f32,
}

impl Pooled for CombatMarker {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.combat_marker }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.combat_marker }
}

impl<'a> Extract<'a> for CombatMarker {
    const TYPE_NAME: &'static str = "CombatMarker";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            object_name: inst.get_str("objectName").map(String::from).unwrap_or_default(),
            minimum_scale: inst.get_f32("minimumScale").unwrap_or_default(),
            inverse_scale_multiplier: inst.get_f32("inverseScaleMultiplier").unwrap_or_default(),
            hit_anim_total_time: inst.get_f32("hitAnimTotalTime").unwrap_or_default(),
            hit_animation_offset_size: inst.get_f32("hitAnimationOffsetSize").unwrap_or_default(),
            ease_type: inst.get_str("easeType").map(String::from).unwrap_or_default(),
            text_offset: inst.get_f32("textOffset").unwrap_or_default(),
            intro_anim_time: inst.get_f32("introAnimTime").unwrap_or_default(),
            intro_anim_pitch_rotation_frequency: inst.get_f32("introAnimPitchRotationFrequency").unwrap_or_default(),
            intro_anim_yaw_rotation_frequency: inst.get_f32("introAnimYawRotationFrequency").unwrap_or_default(),
            intro_anim_roll_rotation_frequency: inst.get_f32("introAnimRollRotationFrequency").unwrap_or_default(),
            intro_anim_ease_type: inst.get_str("introAnimEaseType").map(String::from).unwrap_or_default(),
            intro_starting_scale: inst.get_f32("introStartingScale").unwrap_or_default(),
            intro_starting_offset_scale: inst.get_f32("introStartingOffsetScale").unwrap_or_default(),
            intro_anim_offset: inst.get_f32("introAnimOffset").unwrap_or_default(),
            transition_anim_length: inst.get_f32("transitionAnimLength").unwrap_or_default(),
            transition_anim_ease_type: inst.get_str("transitionAnimEaseType").map(String::from).unwrap_or_default(),
            rotational_animation_clamp: inst.get_f32("rotationalAnimationClamp").unwrap_or_default(),
            rotational_animation_integration_time: inst.get_f32("rotationalAnimationIntegrationTime").unwrap_or_default(),
            signal_lost_animation_time: inst.get_f32("signalLostAnimationTime").unwrap_or_default(),
            signal_lost_animation_pulse_frequency: inst.get_f32("signalLostAnimationPulseFrequency").unwrap_or_default(),
            unfocused_object_name: inst.get_str("unfocusedObjectName").map(String::from).unwrap_or_default(),
            unfocused_marker_scale: inst.get_f32("unfocusedMarkerScale").unwrap_or_default(),
            hit_animation_color: match inst.get("hitAnimationColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_animation_flicker_time: inst.get_f32("hitAnimationFlickerTime").unwrap_or_default(),
            hit_anim_offset_factor: inst.get_f32("hitAnimOffsetFactor").unwrap_or_default(),
            transition_scale_curve: match inst.get("transitionScaleCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gained_focus_anim_total_time: inst.get_f32("gainedFocusAnimTotalTime").unwrap_or_default(),
            gained_focus_anim_flicker_time: inst.get_f32("gainedFocusAnimFlickerTime").unwrap_or_default(),
            un_focused_rotation_factor: inst.get_f32("unFocusedRotationFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponseVariation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponseVariation {
    /// DCB field: `communicationName` (Reference)
    #[serde(default)]
    pub communication_name: Option<CigGuid>,
    /// DCB field: `initialDelay` (Single)
    #[serde(default)]
    pub initial_delay: f32,
    /// DCB field: `hasSfx` (Boolean)
    #[serde(default)]
    pub has_sfx: bool,
    /// DCB field: `refireDelaySfx` (Single)
    #[serde(default)]
    pub refire_delay_sfx: f32,
    /// DCB field: `hasDialogue` (Boolean)
    #[serde(default)]
    pub has_dialogue: bool,
    /// DCB field: `refireDelayDialogue` (Single)
    #[serde(default)]
    pub refire_delay_dialogue: f32,
    /// DCB field: `timeout` (Single)
    #[serde(default)]
    pub timeout: f32,
    /// DCB field: `queuingBehaviour` (Class)
    #[serde(default)]
    pub queuing_behaviour: Option<Handle<QueueingBehaviour>>,
    /// DCB field: `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<CockpitRuleBase>>,
}

impl Pooled for CockpitResponseVariation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.cockpit_response_variation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.cockpit_response_variation }
}

impl<'a> Extract<'a> for CockpitResponseVariation {
    const TYPE_NAME: &'static str = "CockpitResponseVariation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_name: inst.get("communicationName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            initial_delay: inst.get_f32("initialDelay").unwrap_or_default(),
            has_sfx: inst.get_bool("hasSfx").unwrap_or_default(),
            refire_delay_sfx: inst.get_f32("refireDelaySfx").unwrap_or_default(),
            has_dialogue: inst.get_bool("hasDialogue").unwrap_or_default(),
            refire_delay_dialogue: inst.get_f32("refireDelayDialogue").unwrap_or_default(),
            timeout: inst.get_f32("timeout").unwrap_or_default(),
            queuing_behaviour: match inst.get("queuingBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QueueingBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QueueingBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CockpitRuleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CockpitRuleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponse {
    /// DCB field: `concept` (String)
    #[serde(default)]
    pub concept: String,
    /// DCB field: `canPlayWhenLanded` (Boolean)
    #[serde(default)]
    pub can_play_when_landed: bool,
    /// DCB field: `canPlayWhenRacing` (Boolean)
    #[serde(default)]
    pub can_play_when_racing: bool,
    /// DCB field: `variations` (Class (array))
    #[serde(default)]
    pub variations: Vec<Handle<CockpitResponseVariation>>,
}

impl Pooled for CockpitResponse {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.cockpit_response }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.cockpit_response }
}

impl<'a> Extract<'a> for CockpitResponse {
    const TYPE_NAME: &'static str = "CockpitResponse";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            concept: inst.get_str("concept").map(String::from).unwrap_or_default(),
            can_play_when_landed: inst.get_bool("canPlayWhenLanded").unwrap_or_default(),
            can_play_when_racing: inst.get_bool("canPlayWhenRacing").unwrap_or_default(),
            variations: inst.get_array("variations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CockpitResponseVariation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CockpitResponseVariation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponses`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponses {
    /// DCB field: `responses` (Reference (array))
    #[serde(default)]
    pub responses: Vec<CigGuid>,
}

impl Pooled for CockpitResponses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.cockpit_responses }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.cockpit_responses }
}

impl<'a> Extract<'a> for CockpitResponses {
    const TYPE_NAME: &'static str = "CockpitResponses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitRuleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitRuleBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `priority` (Single)
    #[serde(default)]
    pub priority: f32,
}

impl Pooled for CockpitRuleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.cockpit_rule_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.cockpit_rule_base }
}

impl<'a> Extract<'a> for CockpitRuleBase {
    const TYPE_NAME: &'static str = "CockpitRuleBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommodityTemperatureTolerance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityTemperatureTolerance {
    /// DCB field: `optimalTempMin` (Single)
    #[serde(default)]
    pub optimal_temp_min: f32,
    /// DCB field: `optimalTempMax` (Single)
    #[serde(default)]
    pub optimal_temp_max: f32,
    /// DCB field: `OptimalTempFallOff` (Single)
    #[serde(default)]
    pub optimal_temp_fall_off: f32,
    /// DCB field: `damageCurveControl` (StrongPointer)
    #[serde(default)]
    pub damage_curve_control: Option<Handle<TemperatureDamageControl>>,
}

impl Pooled for CommodityTemperatureTolerance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.commodity_temperature_tolerance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.commodity_temperature_tolerance }
}

impl<'a> Extract<'a> for CommodityTemperatureTolerance {
    const TYPE_NAME: &'static str = "CommodityTemperatureTolerance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            optimal_temp_min: inst.get_f32("optimalTempMin").unwrap_or_default(),
            optimal_temp_max: inst.get_f32("optimalTempMax").unwrap_or_default(),
            optimal_temp_fall_off: inst.get_f32("OptimalTempFallOff").unwrap_or_default(),
            damage_curve_control: match inst.get("damageCurveControl") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TemperatureDamageControl>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TemperatureDamageControl>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommunicationConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `communications` (Class (array))
    #[serde(default)]
    pub communications: Vec<Handle<CommunicationEntry>>,
    /// DCB field: `subConfigs` (Reference (array))
    #[serde(default)]
    pub sub_configs: Vec<CigGuid>,
}

impl Pooled for CommunicationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_config }
}

impl<'a> Extract<'a> for CommunicationConfig {
    const TYPE_NAME: &'static str = "CommunicationConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            communications: inst.get_array("communications")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            sub_configs: inst.get_array("subConfigs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationEntry {
    /// DCB field: `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// DCB field: `channelName` (Reference)
    #[serde(default)]
    pub channel_name: Option<CigGuid>,
    /// DCB field: `method` (EnumChoice)
    #[serde(default)]
    pub method: String,
    /// DCB field: `forceAnimation` (Boolean)
    #[serde(default)]
    pub force_animation: bool,
    /// DCB field: `variations` (Class (array))
    #[serde(default)]
    pub variations: Vec<Handle<CommunicationVariation>>,
    /// DCB field: `entityRetriggerDelay` (Single)
    #[serde(default)]
    pub entity_retrigger_delay: f32,
    /// DCB field: `channelRetriggerDelay` (Single)
    #[serde(default)]
    pub channel_retrigger_delay: f32,
    /// DCB field: `gameTags` (Reference (array))
    #[serde(default)]
    pub game_tags: Vec<CigGuid>,
}

impl Pooled for CommunicationEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_entry }
}

impl<'a> Extract<'a> for CommunicationEntry {
    const TYPE_NAME: &'static str = "CommunicationEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            channel_name: inst.get("channelName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            method: inst.get_str("method").map(String::from).unwrap_or_default(),
            force_animation: inst.get_bool("forceAnimation").unwrap_or_default(),
            variations: inst.get_array("variations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationVariation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationVariation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            entity_retrigger_delay: inst.get_f32("entityRetriggerDelay").unwrap_or_default(),
            channel_retrigger_delay: inst.get_f32("channelRetriggerDelay").unwrap_or_default(),
            game_tags: inst.get_array("gameTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariation {
    /// DCB field: `animationFragmentId` (String)
    #[serde(default)]
    pub animation_fragment_id: String,
    /// DCB field: `animationFragmentTags` (String)
    #[serde(default)]
    pub animation_fragment_tags: String,
    /// DCB field: `soundName` (String)
    #[serde(default)]
    pub sound_name: String,
    /// DCB field: `overrideMinSilence` (Single)
    #[serde(default)]
    pub override_min_silence: f32,
    /// DCB field: `overrideMinSpeakerSilence` (Single)
    #[serde(default)]
    pub override_min_speaker_silence: f32,
    /// DCB field: `dialogueContext` (Reference)
    #[serde(default)]
    pub dialogue_context: Option<CigGuid>,
    /// DCB field: `playDialogueAsAnimation` (Boolean)
    #[serde(default)]
    pub play_dialogue_as_animation: bool,
    /// DCB field: `dialogueExternalSource` (Reference)
    #[serde(default)]
    pub dialogue_external_source: Option<CigGuid>,
    /// DCB field: `rules` (Class)
    #[serde(default)]
    pub rules: Option<Handle<CommunicationVariationRules>>,
    /// DCB field: `conditions` (Class)
    #[serde(default)]
    pub conditions: Option<Handle<CommunicationVariationCondition>>,
}

impl Pooled for CommunicationVariation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variation }
}

impl<'a> Extract<'a> for CommunicationVariation {
    const TYPE_NAME: &'static str = "CommunicationVariation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_fragment_id: inst.get_str("animationFragmentId").map(String::from).unwrap_or_default(),
            animation_fragment_tags: inst.get_str("animationFragmentTags").map(String::from).unwrap_or_default(),
            sound_name: inst.get_str("soundName").map(String::from).unwrap_or_default(),
            override_min_silence: inst.get_f32("overrideMinSilence").unwrap_or_default(),
            override_min_speaker_silence: inst.get_f32("overrideMinSpeakerSilence").unwrap_or_default(),
            dialogue_context: inst.get("dialogueContext").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            play_dialogue_as_animation: inst.get_bool("playDialogueAsAnimation").unwrap_or_default(),
            dialogue_external_source: inst.get("dialogueExternalSource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rules: match inst.get("rules") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationVariationRules>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationVariationRules>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conditions: match inst.get("conditions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationVariationCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationVariationCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommunicationVariationRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariationRules {
    /// DCB field: `timeout` (Single)
    #[serde(default)]
    pub timeout: f32,
    /// DCB field: `lookAtTarget` (Boolean)
    #[serde(default)]
    pub look_at_target: bool,
    /// DCB field: `finishAnimation` (Boolean)
    #[serde(default)]
    pub finish_animation: bool,
    /// DCB field: `finishSound` (Boolean)
    #[serde(default)]
    pub finish_sound: bool,
    /// DCB field: `finishVoice` (Boolean)
    #[serde(default)]
    pub finish_voice: bool,
    /// DCB field: `finishTimeout` (Boolean)
    #[serde(default)]
    pub finish_timeout: bool,
    /// DCB field: `blockMovement` (Boolean)
    #[serde(default)]
    pub block_movement: bool,
    /// DCB field: `blockFire` (Boolean)
    #[serde(default)]
    pub block_fire: bool,
}

impl Pooled for CommunicationVariationRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variation_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variation_rules }
}

impl<'a> Extract<'a> for CommunicationVariationRules {
    const TYPE_NAME: &'static str = "CommunicationVariationRules";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            timeout: inst.get_f32("timeout").unwrap_or_default(),
            look_at_target: inst.get_bool("lookAtTarget").unwrap_or_default(),
            finish_animation: inst.get_bool("finishAnimation").unwrap_or_default(),
            finish_sound: inst.get_bool("finishSound").unwrap_or_default(),
            finish_voice: inst.get_bool("finishVoice").unwrap_or_default(),
            finish_timeout: inst.get_bool("finishTimeout").unwrap_or_default(),
            block_movement: inst.get_bool("blockMovement").unwrap_or_default(),
            block_fire: inst.get_bool("blockFire").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariationCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariationCondition {
    /// DCB field: `expression` (String)
    #[serde(default)]
    pub expression: String,
    /// DCB field: `conditionTags` (Class)
    #[serde(default)]
    pub condition_tags: Option<Handle<TagsDNF>>,
}

impl Pooled for CommunicationVariationCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variation_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variation_condition }
}

impl<'a> Extract<'a> for CommunicationVariationCondition {
    const TYPE_NAME: &'static str = "CommunicationVariationCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            expression: inst.get_str("expression").map(String::from).unwrap_or_default(),
            condition_tags: match inst.get("conditionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommunicationChannelConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannelConfig {
    /// DCB field: `ChannelConfigName` (String)
    #[serde(default)]
    pub channel_config_name: String,
    /// DCB field: `Channels` (Class (array))
    #[serde(default)]
    pub channels: Vec<Handle<CommunicationChannel>>,
}

impl Pooled for CommunicationChannelConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_channel_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_channel_config }
}

impl<'a> Extract<'a> for CommunicationChannelConfig {
    const TYPE_NAME: &'static str = "CommunicationChannelConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            channel_config_name: inst.get_str("ChannelConfigName").map(String::from).unwrap_or_default(),
            channels: inst.get_array("Channels")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationChannel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationChannel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationChannel`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannel {
    /// DCB field: `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// DCB field: `audioEventForExternalSources` (String)
    #[serde(default)]
    pub audio_event_for_external_sources: String,
    /// DCB field: `minSilence` (Single)
    #[serde(default)]
    pub min_silence: f32,
    /// DCB field: `flushSilence` (Single)
    #[serde(default)]
    pub flush_silence: f32,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `animationPriorityOverride` (Int32)
    #[serde(default)]
    pub animation_priority_override: i32,
    /// DCB field: `minSpeakerSilence` (Single)
    #[serde(default)]
    pub min_speaker_silence: f32,
    /// DCB field: `ignoreSpeakerSilence` (Boolean)
    #[serde(default)]
    pub ignore_speaker_silence: bool,
    /// DCB field: `subtitles` (Class)
    #[serde(default)]
    pub subtitles: Option<Handle<CommunicationSubtitleSettings>>,
    /// DCB field: `audioRTPC` (StrongPointer)
    #[serde(default)]
    pub audio_rtpc: Option<Handle<CommunicationAudioRTPC>>,
    /// DCB field: `subChannels` (Class (array))
    #[serde(default)]
    pub sub_channels: Vec<Handle<CommunicationChannel>>,
}

impl Pooled for CommunicationChannel {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_channel }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_channel }
}

impl<'a> Extract<'a> for CommunicationChannel {
    const TYPE_NAME: &'static str = "CommunicationChannel";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_event_for_external_sources: inst.get_str("audioEventForExternalSources").map(String::from).unwrap_or_default(),
            min_silence: inst.get_f32("minSilence").unwrap_or_default(),
            flush_silence: inst.get_f32("flushSilence").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            animation_priority_override: inst.get_i32("animationPriorityOverride").unwrap_or_default(),
            min_speaker_silence: inst.get_f32("minSpeakerSilence").unwrap_or_default(),
            ignore_speaker_silence: inst.get_bool("ignoreSpeakerSilence").unwrap_or_default(),
            subtitles: match inst.get("subtitles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationSubtitleSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationSubtitleSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_rtpc: match inst.get("audioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationAudioRTPC>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationAudioRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_channels: inst.get_array("subChannels")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationChannel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationChannel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationSubtitleSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSubtitleSettings {
    /// DCB field: `allow` (Boolean)
    #[serde(default)]
    pub allow: bool,
    /// DCB field: `variableName` (String)
    #[serde(default)]
    pub variable_name: String,
}

impl Pooled for CommunicationSubtitleSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_subtitle_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_subtitle_settings }
}

impl<'a> Extract<'a> for CommunicationSubtitleSettings {
    const TYPE_NAME: &'static str = "CommunicationSubtitleSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow: inst.get_bool("allow").unwrap_or_default(),
            variable_name: inst.get_str("variableName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationAudioRTPC`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAudioRTPC {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `value` (Single)
    #[serde(default)]
    pub value: f32,
}

impl Pooled for CommunicationAudioRTPC {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_audio_rtpc }
}

impl<'a> Extract<'a> for CommunicationAudioRTPC {
    const TYPE_NAME: &'static str = "CommunicationAudioRTPC";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariableConfig {
    /// DCB field: `variables` (Class (array))
    #[serde(default)]
    pub variables: Vec<Handle<CommunicationVariableBool>>,
}

impl Pooled for CommunicationVariableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variable_config }
}

impl<'a> Extract<'a> for CommunicationVariableConfig {
    const TYPE_NAME: &'static str = "CommunicationVariableConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationVariableBool>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationVariableBool>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariableBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariableBase {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `global` (Boolean)
    #[serde(default)]
    pub global: bool,
}

impl Pooled for CommunicationVariableBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variable_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variable_base }
}

impl<'a> Extract<'a> for CommunicationVariableBase {
    const TYPE_NAME: &'static str = "CommunicationVariableBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariableBool`
///
/// Inherits from: `CommunicationVariableBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariableBool {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `global` (Boolean)
    #[serde(default)]
    pub global: bool,
    /// DCB field: `value` (Boolean)
    #[serde(default)]
    pub value: bool,
}

impl Pooled for CommunicationVariableBool {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_variable_bool }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_variable_bool }
}

impl<'a> Extract<'a> for CommunicationVariableBool {
    const TYPE_NAME: &'static str = "CommunicationVariableBool";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationATLConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationATLConfig {
    /// DCB field: `playTriggerPrefix` (String)
    #[serde(default)]
    pub play_trigger_prefix: String,
    /// DCB field: `stopTriggerPrefix` (String)
    #[serde(default)]
    pub stop_trigger_prefix: String,
    /// DCB field: `speakerVoiceSwitch` (String)
    #[serde(default)]
    pub speaker_voice_switch: String,
    /// DCB field: `speakerTypeSwitch` (String)
    #[serde(default)]
    pub speaker_type_switch: String,
}

impl Pooled for CommunicationATLConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_atlconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_atlconfig }
}

impl<'a> Extract<'a> for CommunicationATLConfig {
    const TYPE_NAME: &'static str = "CommunicationATLConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            play_trigger_prefix: inst.get_str("playTriggerPrefix").map(String::from).unwrap_or_default(),
            stop_trigger_prefix: inst.get_str("stopTriggerPrefix").map(String::from).unwrap_or_default(),
            speaker_voice_switch: inst.get_str("speakerVoiceSwitch").map(String::from).unwrap_or_default(),
            speaker_type_switch: inst.get_str("speakerTypeSwitch").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationLocationAutoTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationLocationAutoTags {
    /// DCB field: `starMapObject` (Reference)
    #[serde(default)]
    pub star_map_object: Option<CigGuid>,
    /// DCB field: `actorInLocationMannequinTags` (String)
    #[serde(default)]
    pub actor_in_location_mannequin_tags: String,
    /// DCB field: `availableConversationTopics` (Reference (array))
    #[serde(default)]
    pub available_conversation_topics: Vec<CigGuid>,
    /// DCB field: `conversationTopicsToExclude` (Reference (array))
    #[serde(default)]
    pub conversation_topics_to_exclude: Vec<CigGuid>,
}

impl Pooled for CommunicationLocationAutoTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_location_auto_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_location_auto_tags }
}

impl<'a> Extract<'a> for CommunicationLocationAutoTags {
    const TYPE_NAME: &'static str = "CommunicationLocationAutoTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            star_map_object: inst.get("starMapObject").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_in_location_mannequin_tags: inst.get_str("actorInLocationMannequinTags").map(String::from).unwrap_or_default(),
            available_conversation_topics: inst.get_array("availableConversationTopics")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            conversation_topics_to_exclude: inst.get_array("conversationTopicsToExclude")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationAutoMannequinTagsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAutoMannequinTagsConfig {
    /// DCB field: `actorIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub actor_is_player_fragment_tags: String,
    /// DCB field: `targetIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub target_is_player_fragment_tags: String,
    /// DCB field: `targetIsAllyFragmentTags` (String)
    #[serde(default)]
    pub target_is_ally_fragment_tags: String,
    /// DCB field: `targetIsNeutralFragmentTags` (String)
    #[serde(default)]
    pub target_is_neutral_fragment_tags: String,
    /// DCB field: `targetIsEnemyFragmentTags` (String)
    #[serde(default)]
    pub target_is_enemy_fragment_tags: String,
    /// DCB field: `subjectIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub subject_is_player_fragment_tags: String,
    /// DCB field: `subjectIsAllyFragmentTags` (String)
    #[serde(default)]
    pub subject_is_ally_fragment_tags: String,
    /// DCB field: `subjectIsNeutralFragmentTags` (String)
    #[serde(default)]
    pub subject_is_neutral_fragment_tags: String,
    /// DCB field: `subjectIsEnemyFragmentTags` (String)
    #[serde(default)]
    pub subject_is_enemy_fragment_tags: String,
    /// DCB field: `subjectIsDisguisedFragmentTags` (String)
    #[serde(default)]
    pub subject_is_disguised_fragment_tags: String,
    /// DCB field: `locationsAutoTags` (Class (array))
    #[serde(default)]
    pub locations_auto_tags: Vec<Handle<CommunicationLocationAutoTags>>,
}

impl Pooled for CommunicationAutoMannequinTagsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_auto_mannequin_tags_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_auto_mannequin_tags_config }
}

impl<'a> Extract<'a> for CommunicationAutoMannequinTagsConfig {
    const TYPE_NAME: &'static str = "CommunicationAutoMannequinTagsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_is_player_fragment_tags: inst.get_str("actorIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            target_is_player_fragment_tags: inst.get_str("targetIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            target_is_ally_fragment_tags: inst.get_str("targetIsAllyFragmentTags").map(String::from).unwrap_or_default(),
            target_is_neutral_fragment_tags: inst.get_str("targetIsNeutralFragmentTags").map(String::from).unwrap_or_default(),
            target_is_enemy_fragment_tags: inst.get_str("targetIsEnemyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_player_fragment_tags: inst.get_str("subjectIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_ally_fragment_tags: inst.get_str("subjectIsAllyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_neutral_fragment_tags: inst.get_str("subjectIsNeutralFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_enemy_fragment_tags: inst.get_str("subjectIsEnemyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_disguised_fragment_tags: inst.get_str("subjectIsDisguisedFragmentTags").map(String::from).unwrap_or_default(),
            locations_auto_tags: inst.get_array("locationsAutoTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationLocationAutoTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationLocationAutoTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContextualCommunicationConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualCommunicationConfig {
    /// DCB field: `responseEntries` (Class (array))
    #[serde(default)]
    pub response_entries: Vec<Handle<ContextualCommunicationResponse>>,
}

impl Pooled for ContextualCommunicationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contextual_communication_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contextual_communication_config }
}

impl<'a> Extract<'a> for ContextualCommunicationConfig {
    const TYPE_NAME: &'static str = "ContextualCommunicationConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            response_entries: inst.get_array("responseEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContextualCommunicationResponse>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContextualCommunicationResponse>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContextualCommunicationResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualCommunicationResponse {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `concept` (EnumChoice)
    #[serde(default)]
    pub concept: String,
    /// DCB field: `customConcept` (String)
    #[serde(default)]
    pub custom_concept: String,
    /// DCB field: `refireDelay` (Single)
    #[serde(default)]
    pub refire_delay: f32,
    /// DCB field: `rules` (Class (array))
    #[serde(default)]
    pub rules: Vec<Handle<ContextualCommunicationCondition>>,
    /// DCB field: `response` (Class)
    #[serde(default)]
    pub response: Option<Handle<CommunicationRequest>>,
    /// DCB field: `memoryVariables` (StrongPointer (array))
    #[serde(default)]
    pub memory_variables: Vec<Handle<CommunicationVariableBase>>,
}

impl Pooled for ContextualCommunicationResponse {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contextual_communication_response }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contextual_communication_response }
}

impl<'a> Extract<'a> for ContextualCommunicationResponse {
    const TYPE_NAME: &'static str = "ContextualCommunicationResponse";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            concept: inst.get_str("concept").map(String::from).unwrap_or_default(),
            custom_concept: inst.get_str("customConcept").map(String::from).unwrap_or_default(),
            refire_delay: inst.get_f32("refireDelay").unwrap_or_default(),
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContextualCommunicationCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContextualCommunicationCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            response: match inst.get("response") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationRequest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationRequest>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            memory_variables: inst.get_array("memoryVariables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationVariableBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationVariableBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationRequest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationRequest {
    /// DCB field: `communication` (Reference)
    #[serde(default)]
    pub communication: Option<CigGuid>,
    /// DCB field: `channelName` (Reference)
    #[serde(default)]
    pub channel_name: Option<CigGuid>,
}

impl Pooled for CommunicationRequest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_request }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_request }
}

impl<'a> Extract<'a> for CommunicationRequest {
    const TYPE_NAME: &'static str = "CommunicationRequest";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            communication: inst.get("communication").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            channel_name: inst.get("channelName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContextualCommunicationCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualCommunicationCondition {
    /// DCB field: `criteriaType` (EnumChoice)
    #[serde(default)]
    pub criteria_type: String,
    /// DCB field: `customCriteria` (String)
    #[serde(default)]
    pub custom_criteria: String,
    /// DCB field: `numberValue` (Single)
    #[serde(default)]
    pub number_value: f32,
    /// DCB field: `stringValue` (String)
    #[serde(default)]
    pub string_value: String,
    /// DCB field: `operation` (EnumChoice)
    #[serde(default)]
    pub operation: String,
}

impl Pooled for ContextualCommunicationCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contextual_communication_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contextual_communication_condition }
}

impl<'a> Extract<'a> for ContextualCommunicationCondition {
    const TYPE_NAME: &'static str = "ContextualCommunicationCondition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            criteria_type: inst.get_str("criteriaType").map(String::from).unwrap_or_default(),
            custom_criteria: inst.get_str("customCriteria").map(String::from).unwrap_or_default(),
            number_value: inst.get_f32("numberValue").unwrap_or_default(),
            string_value: inst.get_str("stringValue").map(String::from).unwrap_or_default(),
            operation: inst.get_str("operation").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationName {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for CommunicationName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_name }
}

impl<'a> Extract<'a> for CommunicationName {
    const TYPE_NAME: &'static str = "CommunicationName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationChannelName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannelName {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for CommunicationChannelName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.communication_channel_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.communication_channel_name }
}

impl<'a> Extract<'a> for CommunicationChannelName {
    const TYPE_NAME: &'static str = "CommunicationChannelName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ConversationStickyFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStickyFilter {
    /// DCB field: `movementParams` (Class)
    #[serde(default)]
    pub movement_params: Option<Handle<StickyFilterMovementParams>>,
    /// DCB field: `rotationParams` (Class)
    #[serde(default)]
    pub rotation_params: Option<Handle<StickyFilterRotationParams>>,
    /// DCB field: `autoCenterParams` (Class)
    #[serde(default)]
    pub auto_center_params: Option<Handle<StickyFilterAutocenterParams>>,
    /// DCB field: `dynamicCameraEffectsParams` (Reference)
    #[serde(default)]
    pub dynamic_camera_effects_params: Option<CigGuid>,
    /// DCB field: `highPriorityCameraEffects` (Boolean)
    #[serde(default)]
    pub high_priority_camera_effects: bool,
}

impl Pooled for ConversationStickyFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.conversation_sticky_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.conversation_sticky_filter }
}

impl<'a> Extract<'a> for ConversationStickyFilter {
    const TYPE_NAME: &'static str = "ConversationStickyFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_params: match inst.get("movementParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterMovementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StickyFilterMovementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_params: match inst.get("rotationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StickyFilterRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_center_params: match inst.get("autoCenterParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StickyFilterAutocenterParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StickyFilterAutocenterParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dynamic_camera_effects_params: inst.get("dynamicCameraEffectsParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            high_priority_camera_effects: inst.get_bool("highPriorityCameraEffects").unwrap_or_default(),
        }
    }
}

/// DCB type: `Conversation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// DCB field: `fragments` (StrongPointer (array))
    #[serde(default)]
    pub fragments: Vec<Handle<ConversationNode_Base>>,
}

impl Pooled for Conversation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.conversation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.conversation }
}

impl<'a> Extract<'a> for Conversation {
    const TYPE_NAME: &'static str = "Conversation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragments: inst.get_array("fragments")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ConversationNode_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ConversationNode_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConversationBank`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationBank {
    /// DCB field: `region` (String)
    #[serde(default)]
    pub region: String,
    /// DCB field: `conversations` (Reference (array))
    #[serde(default)]
    pub conversations: Vec<CigGuid>,
}

impl Pooled for ConversationBank {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.conversation_bank }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.conversation_bank }
}

impl<'a> Extract<'a> for ConversationBank {
    const TYPE_NAME: &'static str = "ConversationBank";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            region: inst.get_str("region").map(String::from).unwrap_or_default(),
            conversations: inst.get_array("conversations")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConversationNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationNode_Base {
}

impl Pooled for ConversationNode_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.conversation_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.conversation_node_base }
}

impl<'a> Extract<'a> for ConversationNode_Base {
    const TYPE_NAME: &'static str = "ConversationNode_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ConstantDOFPosWeights`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFPosWeights {
    /// DCB field: `gridDistance` (Int32)
    #[serde(default)]
    pub grid_distance: i32,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for ConstantDOFPosWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.constant_dofpos_weights }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.constant_dofpos_weights }
}

impl<'a> Extract<'a> for ConstantDOFPosWeights {
    const TYPE_NAME: &'static str = "ConstantDOFPosWeights";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grid_distance: inst.get_i32("gridDistance").unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFWeights`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFWeights {
    /// DCB field: `maxPositionWeight` (Single)
    #[serde(default)]
    pub max_position_weight: f32,
    /// DCB field: `positionWeights` (Class (array))
    #[serde(default)]
    pub position_weights: Vec<Handle<ConstantDOFPosWeights>>,
    /// DCB field: `NPC` (Single)
    #[serde(default)]
    pub npc: f32,
    /// DCB field: `localPlayer` (Single)
    #[serde(default)]
    pub local_player: f32,
    /// DCB field: `entity` (Single)
    #[serde(default)]
    pub entity: f32,
    /// DCB field: `actorLookingAtPlayer` (Single)
    #[serde(default)]
    pub actor_looking_at_player: f32,
    /// DCB field: `door` (Single)
    #[serde(default)]
    pub door: f32,
}

impl Pooled for ConstantDOFWeights {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.constant_dofweights }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.constant_dofweights }
}

impl<'a> Extract<'a> for ConstantDOFWeights {
    const TYPE_NAME: &'static str = "ConstantDOFWeights";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_position_weight: inst.get_f32("maxPositionWeight").unwrap_or_default(),
            position_weights: inst.get_array("positionWeights")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ConstantDOFPosWeights>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ConstantDOFPosWeights>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            npc: inst.get_f32("NPC").unwrap_or_default(),
            local_player: inst.get_f32("localPlayer").unwrap_or_default(),
            entity: inst.get_f32("entity").unwrap_or_default(),
            actor_looking_at_player: inst.get_f32("actorLookingAtPlayer").unwrap_or_default(),
            door: inst.get_f32("door").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGrid`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFGrid {
    /// DCB field: `verticalGridSize` (Int32)
    #[serde(default)]
    pub vertical_grid_size: i32,
    /// DCB field: `horizontalGridSize` (Int32)
    #[serde(default)]
    pub horizontal_grid_size: i32,
    /// DCB field: `verticalSpacing` (Single)
    #[serde(default)]
    pub vertical_spacing: f32,
    /// DCB field: `horizontalSpacing` (Single)
    #[serde(default)]
    pub horizontal_spacing: f32,
}

impl Pooled for ConstantDOFGrid {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.constant_dofgrid }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.constant_dofgrid }
}

impl<'a> Extract<'a> for ConstantDOFGrid {
    const TYPE_NAME: &'static str = "ConstantDOFGrid";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            vertical_grid_size: inst.get_i32("verticalGridSize").unwrap_or_default(),
            horizontal_grid_size: inst.get_i32("horizontalGridSize").unwrap_or_default(),
            vertical_spacing: inst.get_f32("verticalSpacing").unwrap_or_default(),
            horizontal_spacing: inst.get_f32("horizontalSpacing").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConstantDOFGlobalData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDOFGlobalData {
    /// DCB field: `movementThresholdToIgnorePlayer` (Single)
    #[serde(default)]
    pub movement_threshold_to_ignore_player: f32,
    /// DCB field: `distanceToIgnorePlayer` (Single)
    #[serde(default)]
    pub distance_to_ignore_player: f32,
    /// DCB field: `rotationThresholdToDisable` (Single)
    #[serde(default)]
    pub rotation_threshold_to_disable: f32,
    /// DCB field: `pierceability` (Int32)
    #[serde(default)]
    pub pierceability: i32,
    /// DCB field: `nonEntityDistanceScale` (Single)
    #[serde(default)]
    pub non_entity_distance_scale: f32,
    /// DCB field: `LODBiasOnTarget` (Int32)
    #[serde(default)]
    pub lodbias_on_target: i32,
    /// DCB field: `maxRangeScale` (Single)
    #[serde(default)]
    pub max_range_scale: f32,
    /// DCB field: `circleOfConfusion` (Single)
    #[serde(default)]
    pub circle_of_confusion: f32,
    /// DCB field: `gridParams` (Class)
    #[serde(default)]
    pub grid_params: Option<Handle<ConstantDOFGrid>>,
    /// DCB field: `weights` (Class)
    #[serde(default)]
    pub weights: Option<Handle<ConstantDOFWeights>>,
}

impl Pooled for ConstantDOFGlobalData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.constant_dofglobal_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.constant_dofglobal_data }
}

impl<'a> Extract<'a> for ConstantDOFGlobalData {
    const TYPE_NAME: &'static str = "ConstantDOFGlobalData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movement_threshold_to_ignore_player: inst.get_f32("movementThresholdToIgnorePlayer").unwrap_or_default(),
            distance_to_ignore_player: inst.get_f32("distanceToIgnorePlayer").unwrap_or_default(),
            rotation_threshold_to_disable: inst.get_f32("rotationThresholdToDisable").unwrap_or_default(),
            pierceability: inst.get_i32("pierceability").unwrap_or_default(),
            non_entity_distance_scale: inst.get_f32("nonEntityDistanceScale").unwrap_or_default(),
            lodbias_on_target: inst.get_i32("LODBiasOnTarget").unwrap_or_default(),
            max_range_scale: inst.get_f32("maxRangeScale").unwrap_or_default(),
            circle_of_confusion: inst.get_f32("circleOfConfusion").unwrap_or_default(),
            grid_params: match inst.get("gridParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConstantDOFGrid>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConstantDOFGrid>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weights: match inst.get("weights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConstantDOFWeights>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConstantDOFWeights>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommodityType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityType {
    /// DCB field: `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `defaultThumbnailPath` (String)
    #[serde(default)]
    pub default_thumbnail_path: String,
}

impl Pooled for CommodityType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.commodity_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.commodity_type }
}

impl<'a> Extract<'a> for CommodityType {
    const TYPE_NAME: &'static str = "CommodityType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            default_thumbnail_path: inst.get_str("defaultThumbnailPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommoditySubtype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommoditySubtype {
    /// DCB field: `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `symbol` (String)
    #[serde(default)]
    pub symbol: String,
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGBA8>>,
    /// DCB field: `commodity` (Reference)
    #[serde(default)]
    pub commodity: Option<CigGuid>,
    /// DCB field: `volatility` (Single)
    #[serde(default)]
    pub volatility: f32,
    /// DCB field: `gForceTolerance` (Single)
    #[serde(default)]
    pub g_force_tolerance: f32,
    /// DCB field: `gForceDeltaToDamage` (Single)
    #[serde(default)]
    pub g_force_delta_to_damage: f32,
    /// DCB field: `HealthDecayOverTime` (Single)
    #[serde(default)]
    pub health_decay_over_time: f32,
    /// DCB field: `temperatureTolerance` (Class)
    #[serde(default)]
    pub temperature_tolerance: Option<Handle<CommodityTemperatureTolerance>>,
    /// DCB field: `damageResistance` (StrongPointer)
    #[serde(default)]
    pub damage_resistance: Option<Handle<DamageResistanceBase>>,
    /// DCB field: `refineOutput` (Reference)
    #[serde(default)]
    pub refine_output: Option<CigGuid>,
}

impl Pooled for CommoditySubtype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.commodity_subtype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.commodity_subtype }
}

impl<'a> Extract<'a> for CommoditySubtype {
    const TYPE_NAME: &'static str = "CommoditySubtype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            symbol: inst.get_str("symbol").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            commodity: inst.get("commodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            volatility: inst.get_f32("volatility").unwrap_or_default(),
            g_force_tolerance: inst.get_f32("gForceTolerance").unwrap_or_default(),
            g_force_delta_to_damage: inst.get_f32("gForceDeltaToDamage").unwrap_or_default(),
            health_decay_over_time: inst.get_f32("HealthDecayOverTime").unwrap_or_default(),
            temperature_tolerance: match inst.get("temperatureTolerance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommodityTemperatureTolerance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommodityTemperatureTolerance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage_resistance: match inst.get("damageResistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageResistanceBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageResistanceBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            refine_output: inst.get("refineOutput").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CommodityTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityTypeDatabase {
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
    /// DCB field: `subtypes` (Reference (array))
    #[serde(default)]
    pub subtypes: Vec<CigGuid>,
}

impl Pooled for CommodityTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.commodity_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.commodity_type_database }
}

impl<'a> Extract<'a> for CommodityTypeDatabase {
    const TYPE_NAME: &'static str = "CommodityTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            subtypes: inst.get_array("subtypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommodityDamageConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityDamageConfiguration {
    /// DCB field: `minimumSpeed` (Single)
    #[serde(default)]
    pub minimum_speed: f32,
    /// DCB field: `speedSquaredToDamage` (Single)
    #[serde(default)]
    pub speed_squared_to_damage: f32,
    /// DCB field: `defaultExplosionParams` (Class)
    #[serde(default)]
    pub default_explosion_params: Option<Handle<ExplosionParams>>,
    /// DCB field: `volatilePowerRtpc` (Class)
    #[serde(default)]
    pub volatile_power_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `volatilityRadiusFactor` (Single)
    #[serde(default)]
    pub volatility_radius_factor: f32,
    /// DCB field: `volatilityDamageFactor` (Single)
    #[serde(default)]
    pub volatility_damage_factor: f32,
    /// DCB field: `volatilityForceFactor` (Single)
    #[serde(default)]
    pub volatility_force_factor: f32,
    /// DCB field: `volatilityCommodityDamageFactor` (Single)
    #[serde(default)]
    pub volatility_commodity_damage_factor: f32,
    /// DCB field: `volatilityParticleStrengthFactor` (Single)
    #[serde(default)]
    pub volatility_particle_strength_factor: f32,
    /// DCB field: `gracePeriod` (Single)
    #[serde(default)]
    pub grace_period: f32,
}

impl Pooled for CommodityDamageConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.commodity_damage_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.commodity_damage_configuration }
}

impl<'a> Extract<'a> for CommodityDamageConfiguration {
    const TYPE_NAME: &'static str = "CommodityDamageConfiguration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_speed: inst.get_f32("minimumSpeed").unwrap_or_default(),
            speed_squared_to_damage: inst.get_f32("speedSquaredToDamage").unwrap_or_default(),
            default_explosion_params: match inst.get("defaultExplosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            volatile_power_rtpc: match inst.get("volatilePowerRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            volatility_radius_factor: inst.get_f32("volatilityRadiusFactor").unwrap_or_default(),
            volatility_damage_factor: inst.get_f32("volatilityDamageFactor").unwrap_or_default(),
            volatility_force_factor: inst.get_f32("volatilityForceFactor").unwrap_or_default(),
            volatility_commodity_damage_factor: inst.get_f32("volatilityCommodityDamageFactor").unwrap_or_default(),
            volatility_particle_strength_factor: inst.get_f32("volatilityParticleStrengthFactor").unwrap_or_default(),
            grace_period: inst.get_f32("gracePeriod").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConditionProhibitedItemsDisplayParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionProhibitedItemsDisplayParams {
    /// DCB field: `autoStoreText` (StrongPointer)
    #[serde(default)]
    pub auto_store_text: Option<Handle<BlockedTextParams>>,
    /// DCB field: `autoStoreColor` (StrongPointer)
    #[serde(default)]
    pub auto_store_color: Option<Handle<BlockedColorParams>>,
}

impl Pooled for ConditionProhibitedItemsDisplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.condition_prohibited_items_display_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.condition_prohibited_items_display_params }
}

impl<'a> Extract<'a> for ConditionProhibitedItemsDisplayParams {
    const TYPE_NAME: &'static str = "ConditionProhibitedItemsDisplayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            auto_store_text: match inst.get("autoStoreText") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedTextParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedTextParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_store_color: match inst.get("autoStoreColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedColorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedColorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ConditionDisplayParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDisplayParams {
    /// DCB field: `exclusiveDisplay` (Boolean)
    #[serde(default)]
    pub exclusive_display: bool,
    /// DCB field: `blockedText` (StrongPointer)
    #[serde(default)]
    pub blocked_text: Option<Handle<BlockedTextParams>>,
    /// DCB field: `blockedCursor` (StrongPointer)
    #[serde(default)]
    pub blocked_cursor: Option<Handle<BlockedCursorParams>>,
    /// DCB field: `blockedColor` (StrongPointer)
    #[serde(default)]
    pub blocked_color: Option<Handle<BlockedColorParams>>,
    /// DCB field: `blockedHint` (StrongPointer)
    #[serde(default)]
    pub blocked_hint: Option<Handle<BlockedHintParams>>,
    /// DCB field: `autoStoreOption` (StrongPointer)
    #[serde(default)]
    pub auto_store_option: Option<Handle<ConditionProhibitedItemsDisplayParams>>,
    /// DCB field: `blockedInteractionAttemptMonologue` (StrongPointer)
    #[serde(default)]
    pub blocked_interaction_attempt_monologue: Option<Handle<DialogueBundle>>,
}

impl Pooled for ConditionDisplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.condition_display_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.condition_display_params }
}

impl<'a> Extract<'a> for ConditionDisplayParams {
    const TYPE_NAME: &'static str = "ConditionDisplayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exclusive_display: inst.get_bool("exclusiveDisplay").unwrap_or_default(),
            blocked_text: match inst.get("blockedText") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedTextParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedTextParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_cursor: match inst.get("blockedCursor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedCursorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedCursorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_color: match inst.get("blockedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedColorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedColorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_hint: match inst.get("blockedHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedHintParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedHintParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_store_option: match inst.get("autoStoreOption") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConditionProhibitedItemsDisplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionProhibitedItemsDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_interaction_attempt_monologue: match inst.get("blockedInteractionAttemptMonologue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DialogueBundle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DialogueBundle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ControlledSubstanceClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledSubstanceClass {
    /// DCB field: `commodities` (Reference (array))
    #[serde(default)]
    pub commodities: Vec<CigGuid>,
    /// DCB field: `resources` (Reference (array))
    #[serde(default)]
    pub resources: Vec<CigGuid>,
    /// DCB field: `maxPossessionSCU` (Single)
    #[serde(default)]
    pub max_possession_scu: f32,
}

impl Pooled for ControlledSubstanceClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.controlled_substance_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.controlled_substance_class }
}

impl<'a> Extract<'a> for ControlledSubstanceClass {
    const TYPE_NAME: &'static str = "ControlledSubstanceClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            commodities: inst.get_array("commodities")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            resources: inst.get_array("resources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_possession_scu: inst.get_f32("maxPossessionSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHintGameModeRecords`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintGameModeRecords {
    /// DCB field: `onFootControlHints` (Reference)
    #[serde(default)]
    pub on_foot_control_hints: Option<CigGuid>,
    /// DCB field: `vehicleControlHints` (Reference)
    #[serde(default)]
    pub vehicle_control_hints: Option<CigGuid>,
    /// DCB field: `steeringSequenceControlHints` (Reference)
    #[serde(default)]
    pub steering_sequence_control_hints: Option<CigGuid>,
    /// DCB field: `groundVehicleControlHints` (Reference)
    #[serde(default)]
    pub ground_vehicle_control_hints: Option<CigGuid>,
    /// DCB field: `turretControlHints` (Reference)
    #[serde(default)]
    pub turret_control_hints: Option<CigGuid>,
    /// DCB field: `boatVehicleControlHints` (Reference)
    #[serde(default)]
    pub boat_vehicle_control_hints: Option<CigGuid>,
    /// DCB field: `mobiglasControlHints` (Reference)
    #[serde(default)]
    pub mobiglas_control_hints: Option<CigGuid>,
    /// DCB field: `transportedControlHints` (Reference)
    #[serde(default)]
    pub transported_control_hints: Option<CigGuid>,
}

impl Pooled for ControlHintGameModeRecords {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_game_mode_records }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_game_mode_records }
}

impl<'a> Extract<'a> for ControlHintGameModeRecords {
    const TYPE_NAME: &'static str = "ControlHintGameModeRecords";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            on_foot_control_hints: inst.get("onFootControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            vehicle_control_hints: inst.get("vehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            steering_sequence_control_hints: inst.get("steeringSequenceControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ground_vehicle_control_hints: inst.get("groundVehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            turret_control_hints: inst.get("turretControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            boat_vehicle_control_hints: inst.get("boatVehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mobiglas_control_hints: inst.get("mobiglasControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            transported_control_hints: inst.get("transportedControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ControlHints_Input`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_Input {
    /// DCB field: `activationMode` (EnumChoice)
    #[serde(default)]
    pub activation_mode: String,
}

impl Pooled for ControlHints_Input {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hints_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hints_input }
}

impl<'a> Extract<'a> for ControlHints_Input {
    const TYPE_NAME: &'static str = "ControlHints_Input";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_mode: inst.get_str("activationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHints_HintDisplayInfoAction`
///
/// Inherits from: `ControlHints_HintDisplayInfo` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_HintDisplayInfoAction {
    /// DCB field: `overrideName` (Boolean)
    #[serde(default)]
    pub override_name: bool,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `includeSeparator` (Boolean)
    #[serde(default)]
    pub include_separator: bool,
    /// DCB field: `separatorInputName` (Locale)
    #[serde(default)]
    pub separator_input_name: String,
    /// DCB field: `separatorIconPath` (String)
    #[serde(default)]
    pub separator_icon_path: String,
    /// DCB field: `actions` (StrongPointer (array))
    #[serde(default)]
    pub actions: Vec<Handle<ControlHints_Input>>,
}

impl Pooled for ControlHints_HintDisplayInfoAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hints_hint_display_info_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hints_hint_display_info_action }
}

impl<'a> Extract<'a> for ControlHints_HintDisplayInfoAction {
    const TYPE_NAME: &'static str = "ControlHints_HintDisplayInfoAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            override_name: inst.get_bool("overrideName").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            include_separator: inst.get_bool("includeSeparator").unwrap_or_default(),
            separator_input_name: inst.get_str("separatorInputName").map(String::from).unwrap_or_default(),
            separator_icon_path: inst.get_str("separatorIconPath").map(String::from).unwrap_or_default(),
            actions: inst.get_array("actions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHints_Input>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHints_Input>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHint_DisplayInfoSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHint_DisplayInfoSet {
    /// DCB field: `overrideHint` (Class)
    #[serde(default)]
    pub override_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// DCB field: `gamepadHint` (StrongPointer)
    #[serde(default)]
    pub gamepad_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// DCB field: `joystickHint` (StrongPointer)
    #[serde(default)]
    pub joystick_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// DCB field: `alwaysShowIfBound` (Boolean)
    #[serde(default)]
    pub always_show_if_bound: bool,
}

impl Pooled for ControlHint_DisplayInfoSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_display_info_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_display_info_set }
}

impl<'a> Extract<'a> for ControlHint_DisplayInfoSet {
    const TYPE_NAME: &'static str = "ControlHint_DisplayInfoSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            override_hint: match inst.get("overrideHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gamepad_hint: match inst.get("gamepadHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            joystick_hint: match inst.get("joystickHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            always_show_if_bound: inst.get_bool("alwaysShowIfBound").unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHintCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintCondition {
}

impl Pooled for ControlHintCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_condition }
}

impl<'a> Extract<'a> for ControlHintCondition {
    const TYPE_NAME: &'static str = "ControlHintCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHintDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintDef {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `condition` (StrongPointer)
    #[serde(default)]
    pub condition: Option<Handle<ControlHintCondition>>,
    /// DCB field: `alwaysDisplayCondition` (StrongPointer)
    #[serde(default)]
    pub always_display_condition: Option<Handle<ControlHintAlwaysDisplayCondition>>,
    /// DCB field: `hintDisplayInfoSet` (Class (array))
    #[serde(default)]
    pub hint_display_info_set: Vec<Handle<ControlHint_DisplayInfoSet>>,
}

impl Pooled for ControlHintDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_def }
}

impl<'a> Extract<'a> for ControlHintDef {
    const TYPE_NAME: &'static str = "ControlHintDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            condition: match inst.get("condition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            always_display_condition: match inst.get("alwaysDisplayCondition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintAlwaysDisplayCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintAlwaysDisplayCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hint_display_info_set: inst.get_array("hintDisplayInfoSet")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHint_DisplayInfoSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHint_DisplayInfoSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHintAlwaysDisplayCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintAlwaysDisplayCondition {
}

impl Pooled for ControlHintAlwaysDisplayCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_always_display_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_always_display_condition }
}

impl<'a> Extract<'a> for ControlHintAlwaysDisplayCondition {
    const TYPE_NAME: &'static str = "ControlHintAlwaysDisplayCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHint_Entry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHint_Entry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `controlHintList` (Class (array))
    #[serde(default)]
    pub control_hint_list: Vec<Handle<ControlHintDef>>,
}

impl Pooled for ControlHint_Entry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hint_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hint_entry }
}

impl<'a> Extract<'a> for ControlHint_Entry {
    const TYPE_NAME: &'static str = "ControlHint_Entry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            control_hint_list: inst.get_array("controlHintList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHintDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHintDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHints_Preset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_Preset {
    /// DCB field: `hintSlots` (Class (array))
    #[serde(default)]
    pub hint_slots: Vec<Handle<ControlHint_Entry>>,
}

impl Pooled for ControlHints_Preset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.control_hints_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.control_hints_preset }
}

impl<'a> Extract<'a> for ControlHints_Preset {
    const TYPE_NAME: &'static str = "ControlHints_Preset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hint_slots: inst.get_array("hintSlots")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHint_Entry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHint_Entry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractStringParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStringParam {
    /// DCB field: `param` (EnumChoice)
    #[serde(default)]
    pub param: String,
    /// DCB field: `value` (Locale)
    #[serde(default)]
    pub value: String,
}

impl Pooled for ContractStringParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_string_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_string_param }
}

impl<'a> Extract<'a> for ContractStringParam {
    const TYPE_NAME: &'static str = "ContractStringParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            param: inst.get_str("param").map(String::from).unwrap_or_default(),
            value: inst.get_str("value").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractBoolParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractBoolParam {
    /// DCB field: `param` (EnumChoice)
    #[serde(default)]
    pub param: String,
    /// DCB field: `value` (Boolean)
    #[serde(default)]
    pub value: bool,
}

impl Pooled for ContractBoolParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_bool_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_bool_param }
}

impl<'a> Extract<'a> for ContractBoolParam {
    const TYPE_NAME: &'static str = "ContractBoolParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            param: inst.get_str("param").map(String::from).unwrap_or_default(),
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractIntParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractIntParam {
    /// DCB field: `param` (EnumChoice)
    #[serde(default)]
    pub param: String,
    /// DCB field: `value` (Int32)
    #[serde(default)]
    pub value: i32,
}

impl Pooled for ContractIntParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_int_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_int_param }
}

impl<'a> Extract<'a> for ContractIntParam {
    const TYPE_NAME: &'static str = "ContractIntParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            param: inst.get_str("param").map(String::from).unwrap_or_default(),
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractAvailability`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAvailability {
    /// DCB field: `prerequisites` (StrongPointer (array))
    #[serde(default)]
    pub prerequisites: Vec<Handle<ContractPrerequisiteBase>>,
    /// DCB field: `notifyOnAvailable` (Boolean)
    #[serde(default)]
    pub notify_on_available: bool,
    /// DCB field: `maxPlayersPerInstance` (Int32)
    #[serde(default)]
    pub max_players_per_instance: i32,
    /// DCB field: `onceOnly` (Boolean)
    #[serde(default)]
    pub once_only: bool,
    /// DCB field: `availableInPrison` (Boolean)
    #[serde(default)]
    pub available_in_prison: bool,
    /// DCB field: `canReacceptAfterAbandoning` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_abandoning: bool,
    /// DCB field: `abandonedCooldownTime` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time: f32,
    /// DCB field: `abandonedCooldownTimeVariation` (Single)
    #[serde(default)]
    pub abandoned_cooldown_time_variation: f32,
    /// DCB field: `canReacceptAfterFailing` (Boolean)
    #[serde(default)]
    pub can_reaccept_after_failing: bool,
    /// DCB field: `hasPersonalCooldown` (Boolean)
    #[serde(default)]
    pub has_personal_cooldown: bool,
    /// DCB field: `personalCooldownTime` (Single)
    #[serde(default)]
    pub personal_cooldown_time: f32,
    /// DCB field: `personalCooldownTimeVariation` (Single)
    #[serde(default)]
    pub personal_cooldown_time_variation: f32,
    /// DCB field: `hideInMobiGlas` (Boolean)
    #[serde(default)]
    pub hide_in_mobi_glas: bool,
}

impl Pooled for ContractAvailability {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_availability }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_availability }
}

impl<'a> Extract<'a> for ContractAvailability {
    const TYPE_NAME: &'static str = "ContractAvailability";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            prerequisites: inst.get_array("prerequisites")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractPrerequisiteBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractPrerequisiteBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            notify_on_available: inst.get_bool("notifyOnAvailable").unwrap_or_default(),
            max_players_per_instance: inst.get_i32("maxPlayersPerInstance").unwrap_or_default(),
            once_only: inst.get_bool("onceOnly").unwrap_or_default(),
            available_in_prison: inst.get_bool("availableInPrison").unwrap_or_default(),
            can_reaccept_after_abandoning: inst.get_bool("canReacceptAfterAbandoning").unwrap_or_default(),
            abandoned_cooldown_time: inst.get_f32("abandonedCooldownTime").unwrap_or_default(),
            abandoned_cooldown_time_variation: inst.get_f32("abandonedCooldownTimeVariation").unwrap_or_default(),
            can_reaccept_after_failing: inst.get_bool("canReacceptAfterFailing").unwrap_or_default(),
            has_personal_cooldown: inst.get_bool("hasPersonalCooldown").unwrap_or_default(),
            personal_cooldown_time: inst.get_f32("personalCooldownTime").unwrap_or_default(),
            personal_cooldown_time_variation: inst.get_f32("personalCooldownTimeVariation").unwrap_or_default(),
            hide_in_mobi_glas: inst.get_bool("hideInMobiGlas").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractGeneratorHandlerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGeneratorHandlerBase {
    /// DCB field: `notForRelease` (Boolean)
    #[serde(default)]
    pub not_for_release: bool,
    /// DCB field: `workInProgress` (Boolean)
    #[serde(default)]
    pub work_in_progress: bool,
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `required_active_scenarios` (Reference (array))
    #[serde(default)]
    pub required_active_scenarios: Vec<CigGuid>,
    /// DCB field: `defaultAvailability` (Class)
    #[serde(default)]
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// DCB field: `contractParams` (Class)
    #[serde(default)]
    pub contract_params: Option<Handle<ContractParamOverrides>>,
}

impl Pooled for ContractGeneratorHandlerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_generator_handler_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_generator_handler_base }
}

impl<'a> Extract<'a> for ContractGeneratorHandlerBase {
    const TYPE_NAME: &'static str = "ContractGeneratorHandlerBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            required_active_scenarios: inst.get_array("required_active_scenarios")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractAvailability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContractAvailability>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractParamOverrides>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContractParamOverrides>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractPropertyTagReplacement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractPropertyTagReplacement {
    /// DCB field: `templateTag` (Reference)
    #[serde(default)]
    pub template_tag: Option<CigGuid>,
    /// DCB field: `replacementTag` (Reference)
    #[serde(default)]
    pub replacement_tag: Option<CigGuid>,
}

impl Pooled for ContractPropertyTagReplacement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_property_tag_replacement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_property_tag_replacement }
}

impl<'a> Extract<'a> for ContractPropertyTagReplacement {
    const TYPE_NAME: &'static str = "ContractPropertyTagReplacement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            template_tag: inst.get("templateTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            replacement_tag: inst.get("replacementTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractParamOverrides`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParamOverrides {
    /// DCB field: `stringParamOverrides` (Class (array))
    #[serde(default)]
    pub string_param_overrides: Vec<Handle<ContractStringParam>>,
    /// DCB field: `boolParamOverrides` (Class (array))
    #[serde(default)]
    pub bool_param_overrides: Vec<Handle<ContractBoolParam>>,
    /// DCB field: `intParamOverrides` (Class (array))
    #[serde(default)]
    pub int_param_overrides: Vec<Handle<ContractIntParam>>,
    /// DCB field: `propertyTagReplacement` (Class (array))
    #[serde(default)]
    pub property_tag_replacement: Vec<Handle<ContractPropertyTagReplacement>>,
    /// DCB field: `propertyOverrides` (Class (array))
    #[serde(default)]
    pub property_overrides: Vec<Handle<MissionProperty>>,
    /// DCB field: `modifierOverrides` (StrongPointer (array))
    #[serde(default)]
    pub modifier_overrides: Vec<Handle<BaseMissionModifier>>,
    /// DCB field: `missionTypeOverride` (Reference)
    #[serde(default)]
    pub mission_type_override: Option<CigGuid>,
}

impl Pooled for ContractParamOverrides {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_param_overrides }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_param_overrides }
}

impl<'a> Extract<'a> for ContractParamOverrides {
    const TYPE_NAME: &'static str = "ContractParamOverrides";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            string_param_overrides: inst.get_array("stringParamOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractStringParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractStringParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            bool_param_overrides: inst.get_array("boolParamOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractBoolParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractBoolParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            int_param_overrides: inst.get_array("intParamOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractIntParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractIntParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            property_tag_replacement: inst.get_array("propertyTagReplacement")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractPropertyTagReplacement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractPropertyTagReplacement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            property_overrides: inst.get_array("propertyOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            modifier_overrides: inst.get_array("modifierOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseMissionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseMissionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mission_type_override: inst.get("missionTypeOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractGenerator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGenerator {
    /// DCB field: `generators` (StrongPointer (array))
    #[serde(default)]
    pub generators: Vec<Handle<ContractGeneratorHandlerBase>>,
    /// DCB field: `required_active_scenarios` (Reference (array))
    #[serde(default)]
    pub required_active_scenarios: Vec<CigGuid>,
}

impl Pooled for ContractGenerator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_generator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_generator }
}

impl<'a> Extract<'a> for ContractGenerator {
    const TYPE_NAME: &'static str = "ContractGenerator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            generators: inst.get_array("generators")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractGeneratorHandlerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractGeneratorHandlerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            required_active_scenarios: inst.get_array("required_active_scenarios")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractPrerequisiteBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractPrerequisiteBase {
}

impl Pooled for ContractPrerequisiteBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_prerequisite_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_prerequisite_base }
}

impl<'a> Extract<'a> for ContractPrerequisiteBase {
    const TYPE_NAME: &'static str = "ContractPrerequisiteBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractDifficultyProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDifficultyProfile {
    /// DCB field: `mechanicalSkillWeight` (Single)
    #[serde(default)]
    pub mechanical_skill_weight: f32,
    /// DCB field: `mentalLoadWeight` (Single)
    #[serde(default)]
    pub mental_load_weight: f32,
    /// DCB field: `riskOfLossWeight` (Single)
    #[serde(default)]
    pub risk_of_loss_weight: f32,
    /// DCB field: `gameKnowledgeWeight` (Single)
    #[serde(default)]
    pub game_knowledge_weight: f32,
}

impl Pooled for ContractDifficultyProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_difficulty_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_difficulty_profile }
}

impl<'a> Extract<'a> for ContractDifficultyProfile {
    const TYPE_NAME: &'static str = "ContractDifficultyProfile";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mechanical_skill_weight: inst.get_f32("mechanicalSkillWeight").unwrap_or_default(),
            mental_load_weight: inst.get_f32("mentalLoadWeight").unwrap_or_default(),
            risk_of_loss_weight: inst.get_f32("riskOfLossWeight").unwrap_or_default(),
            game_knowledge_weight: inst.get_f32("gameKnowledgeWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractDisplayInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDisplayInfo {
    /// DCB field: `displayString` (Locale)
    #[serde(default)]
    pub display_string: String,
    /// DCB field: `type` (Reference)
    #[serde(default)]
    pub r#type: Option<CigGuid>,
    /// DCB field: `illegal` (Boolean)
    #[serde(default)]
    pub illegal: bool,
    /// DCB field: `showLifeTimeInMobiGlas` (Boolean)
    #[serde(default)]
    pub show_life_time_in_mobi_glas: bool,
    /// DCB field: `preShowObjectives` (Boolean)
    #[serde(default)]
    pub pre_show_objectives: bool,
}

impl Pooled for ContractDisplayInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_display_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_display_info }
}

impl<'a> Extract<'a> for ContractDisplayInfo {
    const TYPE_NAME: &'static str = "ContractDisplayInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_string: inst.get_str("displayString").map(String::from).unwrap_or_default(),
            r#type: inst.get("type").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            illegal: inst.get_bool("illegal").unwrap_or_default(),
            show_life_time_in_mobi_glas: inst.get_bool("showLifeTimeInMobiGlas").unwrap_or_default(),
            pre_show_objectives: inst.get_bool("preShowObjectives").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractClassBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClassBase {
}

impl Pooled for ContractClassBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_class_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_class_base }
}

impl<'a> Extract<'a> for ContractClassBase {
    const TYPE_NAME: &'static str = "ContractClassBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractCommsNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCommsNotification {
    /// DCB field: `communicationTags` (WeakPointer)
    #[serde(default)]
    pub communication_tags: Option<Handle<MissionProperty>>,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
}

impl Pooled for ContractCommsNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_comms_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_comms_notification }
}

impl<'a> Extract<'a> for ContractCommsNotification {
    const TYPE_NAME: &'static str = "ContractCommsNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_tags: match inst.get("communicationTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: inst.get_f32("delay").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractEndCommsNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEndCommsNotification {
    /// DCB field: `reason` (EnumChoice)
    #[serde(default)]
    pub reason: String,
    /// DCB field: `communicationTags` (WeakPointer)
    #[serde(default)]
    pub communication_tags: Option<Handle<MissionProperty>>,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
}

impl Pooled for ContractEndCommsNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_end_comms_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_end_comms_notification }
}

impl<'a> Extract<'a> for ContractEndCommsNotification {
    const TYPE_NAME: &'static str = "ContractEndCommsNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reason: inst.get_str("reason").map(String::from).unwrap_or_default(),
            communication_tags: match inst.get("communicationTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: inst.get_f32("delay").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTemplate {
    /// DCB field: `notForRelease` (Boolean)
    #[serde(default)]
    pub not_for_release: bool,
    /// DCB field: `workInProgress` (Boolean)
    #[serde(default)]
    pub work_in_progress: bool,
    /// DCB field: `owner` (Reference)
    #[serde(default)]
    pub owner: Option<CigGuid>,
    /// DCB field: `contractClass` (StrongPointer)
    #[serde(default)]
    pub contract_class: Option<Handle<ContractClassBase>>,
    /// DCB field: `contractDisplayInfo` (StrongPointer)
    #[serde(default)]
    pub contract_display_info: Option<Handle<ContractDisplayInfo>>,
    /// DCB field: `modifiers` (StrongPointer (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BaseMissionModifier>>,
    /// DCB field: `contractProperties` (Class (array))
    #[serde(default)]
    pub contract_properties: Vec<Handle<MissionProperty>>,
    /// DCB field: `objectiveTokens` (Class (array))
    #[serde(default)]
    pub objective_tokens: Vec<Handle<ObjectiveToken>>,
    /// DCB field: `partialRewardPayout` (StrongPointer)
    #[serde(default)]
    pub partial_reward_payout: Option<Handle<PartialContractRewards>>,
    /// DCB field: `missionFlow` (Class)
    #[serde(default)]
    pub mission_flow: Option<Handle<MissionFlow>>,
    /// DCB field: `contractStartCommsNotification` (StrongPointer)
    #[serde(default)]
    pub contract_start_comms_notification: Option<Handle<ContractCommsNotification>>,
    /// DCB field: `contractEndCommsNotifications` (StrongPointer (array))
    #[serde(default)]
    pub contract_end_comms_notifications: Vec<Handle<ContractEndCommsNotification>>,
}

impl Pooled for ContractTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contract_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contract_template }
}

impl<'a> Extract<'a> for ContractTemplate {
    const TYPE_NAME: &'static str = "ContractTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            owner: inst.get("owner").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contract_class: match inst.get("contractClass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractClassBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContractClassBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_display_info: match inst.get("contractDisplayInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractDisplayInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContractDisplayInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseMissionModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseMissionModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            contract_properties: inst.get_array("contractProperties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            objective_tokens: inst.get_array("objectiveTokens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ObjectiveToken>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ObjectiveToken>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            partial_reward_payout: match inst.get("partialRewardPayout") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PartialContractRewards>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PartialContractRewards>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mission_flow: match inst.get("missionFlow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionFlow>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionFlow>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_start_comms_notification: match inst.get("contractStartCommsNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractCommsNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContractCommsNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_end_comms_notifications: inst.get_array("contractEndCommsNotifications")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ContractEndCommsNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ContractEndCommsNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsNotificationSelector`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationSelector {
    /// DCB field: `trigger` (EnumChoice)
    #[serde(default)]
    pub trigger: String,
    /// DCB field: `communicationTags` (WeakPointer)
    #[serde(default)]
    pub communication_tags: Option<Handle<ObjectivePropertyBase>>,
    /// DCB field: `commsTiming` (EnumChoice)
    #[serde(default)]
    pub comms_timing: String,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
}

impl Pooled for CommsNotificationSelector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification_selector }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification_selector }
}

impl<'a> Extract<'a> for CommsNotificationSelector {
    const TYPE_NAME: &'static str = "CommsNotificationSelector";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger: inst.get_str("trigger").map(String::from).unwrap_or_default(),
            communication_tags: match inst.get("communicationTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ObjectivePropertyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ObjectivePropertyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            comms_timing: inst.get_str("commsTiming").map(String::from).unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsNotificationStageCamera`
///
/// Inherits from: `CommsNotificationStageBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationStageCamera {
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Vec3>>,
    /// DCB field: `FOV` (Single)
    #[serde(default)]
    pub fov: f32,
}

impl Pooled for CommsNotificationStageCamera {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification_stage_camera }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification_stage_camera }
}

impl<'a> Extract<'a> for CommsNotificationStageCamera {
    const TYPE_NAME: &'static str = "CommsNotificationStageCamera";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fov: inst.get_f32("FOV").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsNotificationStageActorMark`
///
/// Inherits from: `CommsNotificationStageBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationStageActorMark {
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Vec3>>,
}

impl Pooled for CommsNotificationStageActorMark {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification_stage_actor_mark }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification_stage_actor_mark }
}

impl<'a> Extract<'a> for CommsNotificationStageActorMark {
    const TYPE_NAME: &'static str = "CommsNotificationStageActorMark";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommsNotificationStageListItem`
///
/// Inherits from: `CommsNotificationStageBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationStageListItem {
    /// DCB field: `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Vec3>>,
}

impl Pooled for CommsNotificationStageListItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification_stage_list_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification_stage_list_item }
}

impl<'a> Extract<'a> for CommsNotificationStageListItem {
    const TYPE_NAME: &'static str = "CommsNotificationStageListItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommsNotificationStage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationStage {
    /// DCB field: `camera` (Class)
    #[serde(default)]
    pub camera: Option<Handle<CommsNotificationStageCamera>>,
    /// DCB field: `actorMark` (Class)
    #[serde(default)]
    pub actor_mark: Option<Handle<CommsNotificationStageActorMark>>,
    /// DCB field: `objects` (StrongPointer (array))
    #[serde(default)]
    pub objects: Vec<Handle<CommsNotificationStageListItem>>,
}

impl Pooled for CommsNotificationStage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification_stage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification_stage }
}

impl<'a> Extract<'a> for CommsNotificationStage {
    const TYPE_NAME: &'static str = "CommsNotificationStage";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera: match inst.get("camera") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommsNotificationStageCamera>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommsNotificationStageCamera>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_mark: match inst.get("actorMark") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommsNotificationStageActorMark>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommsNotificationStageActorMark>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            objects: inst.get_array("objects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommsNotificationStageListItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommsNotificationStageListItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotification {
    /// DCB field: `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagList>>,
    /// DCB field: `communicationName` (Reference)
    #[serde(default)]
    pub communication_name: Option<CigGuid>,
    /// DCB field: `characterEntityClass` (Reference)
    #[serde(default)]
    pub character_entity_class: Option<CigGuid>,
    /// DCB field: `fakeCommsAudioEntityClass` (Reference)
    #[serde(default)]
    pub fake_comms_audio_entity_class: Option<CigGuid>,
    /// DCB field: `stage` (Reference)
    #[serde(default)]
    pub stage: Option<CigGuid>,
}

impl Pooled for CommsNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notification }
}

impl<'a> Extract<'a> for CommsNotification {
    const TYPE_NAME: &'static str = "CommsNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            communication_name: inst.get("communicationName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            character_entity_class: inst.get("characterEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fake_comms_audio_entity_class: inst.get("fakeCommsAudioEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stage: inst.get("stage").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `CommsNotificationsGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationsGlobalParams {
    /// DCB field: `channelName` (Reference)
    #[serde(default)]
    pub channel_name: Option<CigGuid>,
    /// DCB field: `expiry` (Single)
    #[serde(default)]
    pub expiry: f32,
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `fakeCommsAudioEntityClass3D` (Reference)
    #[serde(default)]
    pub fake_comms_audio_entity_class3_d: Option<CigGuid>,
}

impl Pooled for CommsNotificationsGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_notifications_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_notifications_global_params }
}

impl<'a> Extract<'a> for CommsNotificationsGlobalParams {
    const TYPE_NAME: &'static str = "CommsNotificationsGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            channel_name: inst.get("channelName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            expiry: inst.get_f32("expiry").unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            fake_comms_audio_entity_class3_d: inst.get("fakeCommsAudioEntityClass3D").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContactStateGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactStateGlobalParams {
    /// DCB field: `contactStateIcons` (String)
    #[serde(default)]
    pub contact_state_icons: String,
}

impl Pooled for ContactStateGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_state_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_state_global_params }
}

impl<'a> Extract<'a> for ContactStateGlobalParams {
    const TYPE_NAME: &'static str = "ContactStateGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            contact_state_icons: inst.get_str("contactStateIcons").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ContactHighlightLayersParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightLayersParams {
    /// DCB field: `highlightParams` (Class)
    #[serde(default)]
    pub highlight_params: Option<Handle<ContactHighlightStateParams>>,
}

impl Pooled for ContactHighlightLayersParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_highlight_layers_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_highlight_layers_params }
}

impl<'a> Extract<'a> for ContactHighlightLayersParams {
    const TYPE_NAME: &'static str = "ContactHighlightLayersParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactHighlightSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightSharedParams {
    /// DCB field: `enableMaterialHighlight` (Boolean)
    #[serde(default)]
    pub enable_material_highlight: bool,
    /// DCB field: `lifeTimeTotal` (Single)
    #[serde(default)]
    pub life_time_total: f32,
    /// DCB field: `lifeTimeInterference` (Single)
    #[serde(default)]
    pub life_time_interference: f32,
    /// DCB field: `lifeTimeFlare` (Single)
    #[serde(default)]
    pub life_time_flare: f32,
    /// DCB field: `maxHighlightDistance` (Single)
    #[serde(default)]
    pub max_highlight_distance: f32,
    /// DCB field: `minHighlightDistance` (Single)
    #[serde(default)]
    pub min_highlight_distance: f32,
    /// DCB field: `maxHighlightOpacity` (Single)
    #[serde(default)]
    pub max_highlight_opacity: f32,
    /// DCB field: `minHighlightOpacity` (Single)
    #[serde(default)]
    pub min_highlight_opacity: f32,
    /// DCB field: `opacityFadeInDelay` (Single)
    #[serde(default)]
    pub opacity_fade_in_delay: f32,
    /// DCB field: `opacityFadeInDuration` (Single)
    #[serde(default)]
    pub opacity_fade_in_duration: f32,
    /// DCB field: `opacityFadeOutDuration` (Single)
    #[serde(default)]
    pub opacity_fade_out_duration: f32,
    /// DCB field: `highlightLayers` (Class)
    #[serde(default)]
    pub highlight_layers: Option<Handle<ContactHighlightLayersParams>>,
    /// DCB field: `audioTriggerTags` (Reference)
    #[serde(default)]
    pub audio_trigger_tags: Option<CigGuid>,
}

impl Pooled for ContactHighlightSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_highlight_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_highlight_shared_params }
}

impl<'a> Extract<'a> for ContactHighlightSharedParams {
    const TYPE_NAME: &'static str = "ContactHighlightSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_material_highlight: inst.get_bool("enableMaterialHighlight").unwrap_or_default(),
            life_time_total: inst.get_f32("lifeTimeTotal").unwrap_or_default(),
            life_time_interference: inst.get_f32("lifeTimeInterference").unwrap_or_default(),
            life_time_flare: inst.get_f32("lifeTimeFlare").unwrap_or_default(),
            max_highlight_distance: inst.get_f32("maxHighlightDistance").unwrap_or_default(),
            min_highlight_distance: inst.get_f32("minHighlightDistance").unwrap_or_default(),
            max_highlight_opacity: inst.get_f32("maxHighlightOpacity").unwrap_or_default(),
            min_highlight_opacity: inst.get_f32("minHighlightOpacity").unwrap_or_default(),
            opacity_fade_in_delay: inst.get_f32("opacityFadeInDelay").unwrap_or_default(),
            opacity_fade_in_duration: inst.get_f32("opacityFadeInDuration").unwrap_or_default(),
            opacity_fade_out_duration: inst.get_f32("opacityFadeOutDuration").unwrap_or_default(),
            highlight_layers: match inst.get("highlightLayers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightLayersParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightLayersParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_trigger_tags: inst.get("audioTriggerTags").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ContactHighlightVisualBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightVisualBaseParams {
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGBA>>,
}

impl Pooled for ContactHighlightVisualBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_highlight_visual_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_highlight_visual_base_params }
}

impl<'a> Extract<'a> for ContactHighlightVisualBaseParams {
    const TYPE_NAME: &'static str = "ContactHighlightVisualBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactHighlightStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactHighlightStateParams {
    /// DCB field: `defaultParams` (StrongPointer)
    #[serde(default)]
    pub default_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// DCB field: `occludedParams` (StrongPointer)
    #[serde(default)]
    pub occluded_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// DCB field: `flareParams` (StrongPointer)
    #[serde(default)]
    pub flare_params: Option<Handle<ContactHighlightVisualBaseParams>>,
    /// DCB field: `occludedFlareParams` (StrongPointer)
    #[serde(default)]
    pub occluded_flare_params: Option<Handle<ContactHighlightVisualBaseParams>>,
}

impl Pooled for ContactHighlightStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_highlight_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_highlight_state_params }
}

impl<'a> Extract<'a> for ContactHighlightStateParams {
    const TYPE_NAME: &'static str = "ContactHighlightStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_params: match inst.get("defaultParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_params: match inst.get("occludedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flare_params: match inst.get("flareParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_flare_params: match inst.get("occludedFlareParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ContactHighlightVisualBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContactTaggingSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTaggingSharedParams {
    /// DCB field: `viewAngle` (Single)
    #[serde(default)]
    pub view_angle: f32,
    /// DCB field: `maxTaggingDistance` (Single)
    #[serde(default)]
    pub max_tagging_distance: f32,
}

impl Pooled for ContactTaggingSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.contact_tagging_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.contact_tagging_shared_params }
}

impl<'a> Extract<'a> for ContactTaggingSharedParams {
    const TYPE_NAME: &'static str = "ContactTaggingSharedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_angle: inst.get_f32("viewAngle").unwrap_or_default(),
            max_tagging_distance: inst.get_f32("maxTaggingDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ConsumableType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableType {
    /// DCB field: `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// DCB field: `subtypes` (Reference (array))
    #[serde(default)]
    pub subtypes: Vec<CigGuid>,
}

impl Pooled for ConsumableType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.consumable_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.consumable_type }
}

impl<'a> Extract<'a> for ConsumableType {
    const TYPE_NAME: &'static str = "ConsumableType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            subtypes: inst.get_array("subtypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConsumableSubtype`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableSubtype {
    /// DCB field: `typeName` (String)
    #[serde(default)]
    pub type_name: String,
    /// DCB field: `consumableName` (Locale)
    #[serde(default)]
    pub consumable_name: String,
    /// DCB field: `effectsPerMicroSCU` (StrongPointer (array))
    #[serde(default)]
    pub effects_per_micro_scu: Vec<Handle<ConsumableEffect>>,
    /// DCB field: `tintOverride` (StrongPointer)
    #[serde(default)]
    pub tint_override: Option<Handle<RGBA>>,
}

impl Pooled for ConsumableSubtype {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.consumable_subtype }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.consumable_subtype }
}

impl<'a> Extract<'a> for ConsumableSubtype {
    const TYPE_NAME: &'static str = "ConsumableSubtype";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            consumable_name: inst.get_str("consumableName").map(String::from).unwrap_or_default(),
            effects_per_micro_scu: inst.get_array("effectsPerMicroSCU")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ConsumableEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ConsumableEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            tint_override: match inst.get("tintOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ConsumableTypeDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTypeDatabase {
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for ConsumableTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.consumable_type_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.consumable_type_database }
}

impl<'a> Extract<'a> for ConsumableTypeDatabase {
    const TYPE_NAME: &'static str = "ConsumableTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ConsumableEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableEffect {
    /// DCB field: `effectDescription` (Locale)
    #[serde(default)]
    pub effect_description: String,
}

impl Pooled for ConsumableEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.consumable_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.consumable_effect }
}

impl<'a> Extract<'a> for ConsumableEffect {
    const TYPE_NAME: &'static str = "ConsumableEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            effect_description: inst.get_str("effectDescription").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsAudioEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsAudioEffect {
    /// DCB field: `busName` (String)
    #[serde(default)]
    pub bus_name: String,
    /// DCB field: `locationId` (EnumChoice)
    #[serde(default)]
    pub location_id: String,
}

impl Pooled for CommsAudioEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_audio_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_audio_effect }
}

impl<'a> Extract<'a> for CommsAudioEffect {
    const TYPE_NAME: &'static str = "CommsAudioEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            bus_name: inst.get_str("busName").map(String::from).unwrap_or_default(),
            location_id: inst.get_str("locationId").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CorpseInteractionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpseInteractionParams {
    /// DCB field: `itemRecoveryBlacklist` (Reference (array))
    #[serde(default)]
    pub item_recovery_blacklist: Vec<CigGuid>,
    /// DCB field: `corpseClasses` (Reference (array))
    #[serde(default)]
    pub corpse_classes: Vec<CigGuid>,
    /// DCB field: `allowedTypes` (EnumChoice (array))
    #[serde(default)]
    pub allowed_types: Vec<String>,
    /// DCB field: `allowedSubTypes` (EnumChoice (array))
    #[serde(default)]
    pub allowed_sub_types: Vec<String>,
}

impl Pooled for CorpseInteractionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.corpse_interaction_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.corpse_interaction_params }
}

impl<'a> Extract<'a> for CorpseInteractionParams {
    const TYPE_NAME: &'static str = "CorpseInteractionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_recovery_blacklist: inst.get_array("itemRecoveryBlacklist")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            corpse_classes: inst.get_array("corpseClasses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            allowed_types: inst.get_array("allowedTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            allowed_sub_types: inst.get_array("allowedSubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CompletionTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTypeBase {
}

impl Pooled for CompletionTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.completion_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.completion_type_base }
}

impl<'a> Extract<'a> for CompletionTypeBase {
    const TYPE_NAME: &'static str = "CompletionTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CommsChannelDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsChannelDef {
    /// DCB field: `channelId` (String)
    #[serde(default)]
    pub channel_id: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `canHaveMultipleInstances` (Boolean)
    #[serde(default)]
    pub can_have_multiple_instances: bool,
    /// DCB field: `useAreaOfEffect` (Boolean)
    #[serde(default)]
    pub use_area_of_effect: bool,
}

impl Pooled for CommsChannelDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_co.comms_channel_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_co.comms_channel_def }
}

impl<'a> Extract<'a> for CommsChannelDef {
    const TYPE_NAME: &'static str = "CommsChannelDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            channel_id: inst.get_str("channelId").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            can_have_multiple_instances: inst.get_bool("canHaveMultipleInstances").unwrap_or_default(),
            use_area_of_effect: inst.get_bool("useAreaOfEffect").unwrap_or_default(),
        }
    }
}

