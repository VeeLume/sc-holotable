// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-scitem-actormovables` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemActormovablesPools {
    #[serde(default)]
    pub build_mode_kiosk_provider_component_params: Vec<Option<BuildModeKioskProviderComponentParams>>,
    #[serde(default)]
    pub interaction_condition_hover_power_stage_equal: Vec<Option<InteractionConditionHoverPowerStageEqual>>,
    #[serde(default)]
    pub state_modifier_hover_power_stage: Vec<Option<StateModifierHoverPowerStage>>,
    #[serde(default)]
    pub sstate_modifier_movable_mover: Vec<Option<SStateModifierMovableMover>>,
    #[serde(default)]
    pub sentity_hover_physics_partial_params: Vec<Option<SEntityHoverPhysicsPartialParams>>,
    #[serde(default)]
    pub sentity_hover_physics_controller_params: Vec<Option<SEntityHoverPhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_hover_physics_controller_component_params: Vec<Option<SEntityHoverPhysicsControllerComponentParams>>,
}
