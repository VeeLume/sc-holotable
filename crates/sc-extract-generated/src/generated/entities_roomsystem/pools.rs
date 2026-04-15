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

/// Pool storage for the `entities-roomsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesRoomsystemPools {
    #[serde(default)]
    pub fire_area_hazards: Vec<Option<FireAreaHazards>>,
    #[serde(default)]
    pub cigaudio_params: Vec<Option<CIGAudioParams>>,
    #[serde(default)]
    pub entity_component_fire_area: Vec<Option<EntityComponentFireArea>>,
    #[serde(default)]
    pub fire_voxel_selection_shape_box: Vec<Option<FireVoxelSelectionShape_Box>>,
    #[serde(default)]
    pub entity_component_fire_filter: Vec<Option<EntityComponentFireFilter>>,
    #[serde(default)]
    pub entity_component_extinguisher: Vec<Option<EntityComponentExtinguisher>>,
    #[serde(default)]
    pub extinguish_type_spray: Vec<Option<ExtinguishType_Spray>>,
    #[serde(default)]
    pub entity_component_fire_repairer: Vec<Option<EntityComponentFireRepairer>>,
    #[serde(default)]
    pub fire_repairer_type_entity_pos: Vec<Option<FireRepairerType_EntityPos>>,
    #[serde(default)]
    pub entity_temperature_state_modifier: Vec<Option<EntityTemperatureStateModifier>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_life_support: Vec<Option<ItemResourceDynamicAmountLifeSupport>>,
    #[serde(default)]
    pub sentity_component_room_group_params: Vec<Option<SEntityComponentRoomGroupParams>>,
    #[serde(default)]
    pub entity_component_room_fade_volume_params: Vec<Option<EntityComponentRoomFadeVolumeParams>>,
    #[serde(default)]
    pub volume_shape_sphere: Vec<Option<VolumeShape_Sphere>>,
    #[serde(default)]
    pub satmospheric_composition_inherit: Vec<Option<SAtmosphericCompositionInherit>>,
    #[serde(default)]
    pub asteroid_state: Vec<Option<AsteroidState>>,
}
