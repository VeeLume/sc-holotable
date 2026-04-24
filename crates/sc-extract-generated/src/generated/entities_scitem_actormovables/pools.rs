// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `entities-scitem-actormovables` feature.
#[derive(Default)]
pub struct EntitiesScitemActormovablesPools {
    pub build_mode_kiosk_provider_component_params:
        Vec<Option<BuildModeKioskProviderComponentParams>>,
    pub interaction_condition_hover_power_stage_equal:
        Vec<Option<InteractionConditionHoverPowerStageEqual>>,
    pub state_modifier_hover_power_stage: Vec<Option<StateModifierHoverPowerStage>>,
    pub sstate_modifier_movable_mover: Vec<Option<SStateModifierMovableMover>>,
    pub sentity_hover_physics_partial_params: Vec<Option<SEntityHoverPhysicsPartialParams>>,
    pub sentity_hover_physics_controller_params: Vec<Option<SEntityHoverPhysicsControllerParams>>,
    pub sentity_hover_physics_controller_component_params:
        Vec<Option<SEntityHoverPhysicsControllerComponentParams>>,
}
