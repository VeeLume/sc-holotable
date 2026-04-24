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

/// Pool storage for the `entities-roomsystem` feature.
#[derive(Default)]
pub struct EntitiesRoomsystemPools {
    pub fire_area_hazards: Vec<Option<FireAreaHazards>>,
    pub cigaudio_params: Vec<Option<CIGAudioParams>>,
    pub entity_component_fire_area: Vec<Option<EntityComponentFireArea>>,
    pub fire_voxel_selection_shape_box: Vec<Option<FireVoxelSelectionShape_Box>>,
    pub entity_component_fire_filter: Vec<Option<EntityComponentFireFilter>>,
    pub entity_component_extinguisher: Vec<Option<EntityComponentExtinguisher>>,
    pub extinguish_type_spray: Vec<Option<ExtinguishType_Spray>>,
    pub entity_component_fire_repairer: Vec<Option<EntityComponentFireRepairer>>,
    pub fire_repairer_type_entity_pos: Vec<Option<FireRepairerType_EntityPos>>,
    pub entity_temperature_state_modifier: Vec<Option<EntityTemperatureStateModifier>>,
    pub item_resource_dynamic_amount_life_support:
        Vec<Option<ItemResourceDynamicAmountLifeSupport>>,
    pub sentity_component_room_group_params: Vec<Option<SEntityComponentRoomGroupParams>>,
    pub entity_component_room_fade_volume_params: Vec<Option<EntityComponentRoomFadeVolumeParams>>,
    pub volume_shape_sphere: Vec<Option<VolumeShape_Sphere>>,
    pub satmospheric_composition_inherit: Vec<Option<SAtmosphericCompositionInherit>>,
    pub asteroid_state: Vec<Option<AsteroidState>>,
}
