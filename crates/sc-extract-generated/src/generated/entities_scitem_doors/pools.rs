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

/// Pool storage for the `entities-scitem-doors` feature.
#[derive(Default)]
pub struct EntitiesScitemDoorsPools {
    pub seffect_input_params_particle: Vec<Option<SEffectInputParamsParticle>>,
    pub seffect_param_particle: Vec<Option<SEffectParamParticle>>,
    pub seffect_params_node_particle: Vec<Option<SEffectParamsNodeParticle>>,
    pub geom_entity_group_params: Vec<Option<GeomEntityGroupParams>>,
    pub set_door_power_state_event: Vec<Option<SetDoorPowerStateEvent>>,
    pub sentity_traversing_node_type_zone_host_entity: Vec<Option<SEntityTraversingNodeTypeZoneHostEntity>>,
    pub shydraulic_pumpable_component_params: Vec<Option<SHydraulicPumpableComponentParams>>,
    pub sinstanced_interior_gateway_params: Vec<Option<SInstancedInteriorGatewayParams>>,
    pub interaction_condition_eligible_for_prison_release: Vec<Option<InteractionConditionEligibleForPrisonRelease>>,
    pub interaction_condition_access_reserved_room: Vec<Option<InteractionConditionAccessReservedRoom>>,
    pub interaction_condition_player_in_front: Vec<Option<InteractionConditionPlayerInFront>>,
    pub set_door_auto_close_gameplay_trigger: Vec<Option<SetDoorAutoCloseGameplayTrigger>>,
    pub loadout_check_group: Vec<Option<LoadoutCheckGroup>>,
    pub loadout_entity_check: Vec<Option<LoadoutEntityCheck>>,
    pub check_entities_on_actors_loadout_within_area_gameplay_trigger: Vec<Option<CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger>>,
    pub scitem_proximity_sensor_box_params: Vec<Option<SCItemProximitySensorBoxParams>>,
    pub scitem_door_code_procedural_params: Vec<Option<SCItemDoorCodeProceduralParams>>,
    pub sdoor_collision_reaction_toggle_params: Vec<Option<SDoorCollisionReactionToggleParams>>,
    pub sdoor_collision_reaction_open_params: Vec<Option<SDoorCollisionReactionOpenParams>>,
    pub scitem_door_hazard_lights_params: Vec<Option<SCItemDoorHazardLightsParams>>,
}
