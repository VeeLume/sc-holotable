// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-holofield`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SHoloFieldComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SHoloFieldComponentParams {
    /// `shape` (EnumChoice)
    pub shape: EHoloFieldShape,
    /// `model` (String)
    pub model: String,
    /// `material` (String)
    pub material: String,
    /// `audioMessageDistance` (Single)
    pub audio_message_distance: f32,
    /// `audioMessageDistanceNoVehicle` (Single)
    pub audio_message_distance_no_vehicle: f32,
    /// `audioMessageDelay` (Single)
    pub audio_message_delay: f32,
    /// `vehicleDestroyDistance` (Single)
    pub vehicle_destroy_distance: f32,
    /// `hitRadius` (Single)
    pub hit_radius: f32,
    /// `hitDuration` (Single)
    pub hit_duration: f32,
    /// `repeatHitTime` (Single)
    pub repeat_hit_time: f32,
    /// `repeatHitDistance` (Single)
    pub repeat_hit_distance: f32,
    /// `damageMultiplier` (Single)
    pub damage_multiplier: f32,
    /// `maxDamage` (Single)
    pub max_damage: f32,
    /// `particleHitScale` (Single)
    pub particle_hit_scale: f32,
}

impl Pooled for SHoloFieldComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_holofield.sholo_field_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_holofield.sholo_field_component_params
    }
}

impl<'a> Extract<'a> for SHoloFieldComponentParams {
    const TYPE_NAME: &'static str = "SHoloFieldComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            shape: EHoloFieldShape::from_dcb_str(inst.get_str("shape").unwrap_or("")),
            model: inst.get_str("model").map(String::from).unwrap_or_default(),
            material: inst
                .get_str("material")
                .map(String::from)
                .unwrap_or_default(),
            audio_message_distance: inst.get_f32("audioMessageDistance").unwrap_or_default(),
            audio_message_distance_no_vehicle: inst
                .get_f32("audioMessageDistanceNoVehicle")
                .unwrap_or_default(),
            audio_message_delay: inst.get_f32("audioMessageDelay").unwrap_or_default(),
            vehicle_destroy_distance: inst.get_f32("vehicleDestroyDistance").unwrap_or_default(),
            hit_radius: inst.get_f32("hitRadius").unwrap_or_default(),
            hit_duration: inst.get_f32("hitDuration").unwrap_or_default(),
            repeat_hit_time: inst.get_f32("repeatHitTime").unwrap_or_default(),
            repeat_hit_distance: inst.get_f32("repeatHitDistance").unwrap_or_default(),
            damage_multiplier: inst.get_f32("damageMultiplier").unwrap_or_default(),
            max_damage: inst.get_f32("maxDamage").unwrap_or_default(),
            particle_hit_scale: inst.get_f32("particleHitScale").unwrap_or_default(),
        }
    }
}
