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

/// DCB type: `Vec2`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    /// DCB field: `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// DCB field: `y` (Single)
    #[serde(default)]
    pub y: f32,
}

impl Pooled for Vec2 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vec2 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vec2 }
}

impl<'a> Extract<'a> for Vec2 {
    const TYPE_NAME: &'static str = "Vec2";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
        }
    }
}

/// DCB type: `Vec3`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    /// DCB field: `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// DCB field: `y` (Single)
    #[serde(default)]
    pub y: f32,
    /// DCB field: `z` (Single)
    #[serde(default)]
    pub z: f32,
}

impl Pooled for Vec3 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vec3 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vec3 }
}

impl<'a> Extract<'a> for Vec3 {
    const TYPE_NAME: &'static str = "Vec3";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
            z: inst.get_f32("z").unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleDifficultyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleDifficultyParams {
    /// DCB field: `damageModifierToAiVehicles` (Single)
    #[serde(default)]
    pub damage_modifier_to_ai_vehicles: f32,
    /// DCB field: `damageModifierFromAiVehicles` (Single)
    #[serde(default)]
    pub damage_modifier_from_ai_vehicles: f32,
}

impl Pooled for VehicleDifficultyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_difficulty_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_difficulty_params }
}

impl<'a> Extract<'a> for VehicleDifficultyParams {
    const TYPE_NAME: &'static str = "VehicleDifficultyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damage_modifier_to_ai_vehicles: inst.get_f32("damageModifierToAiVehicles").unwrap_or_default(),
            damage_modifier_from_ai_vehicles: inst.get_f32("damageModifierFromAiVehicles").unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleSerialNumberCharacterType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSerialNumberCharacterType {
    /// DCB field: `possibleCharacters` (String)
    #[serde(default)]
    pub possible_characters: String,
}

impl Pooled for VehicleSerialNumberCharacterType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_serial_number_character_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_serial_number_character_type }
}

impl<'a> Extract<'a> for VehicleSerialNumberCharacterType {
    const TYPE_NAME: &'static str = "VehicleSerialNumberCharacterType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            possible_characters: inst.get_str("possibleCharacters").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleSerialNumberFormat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSerialNumberFormat {
    /// DCB field: `characterTypes` (Reference (array))
    #[serde(default)]
    pub character_types: Vec<CigGuid>,
    /// DCB field: `format` (Reference (array))
    #[serde(default)]
    pub format: Vec<CigGuid>,
}

impl Pooled for VehicleSerialNumberFormat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_serial_number_format }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_serial_number_format }
}

impl<'a> Extract<'a> for VehicleSerialNumberFormat {
    const TYPE_NAME: &'static str = "VehicleSerialNumberFormat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            character_types: inst.get_array("characterTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            format: inst.get_array("format")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleLandingGearSpring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGearSpring {
    /// DCB field: `lengthTarget` (Single)
    #[serde(default)]
    pub length_target: f32,
    /// DCB field: `lengthMin` (Single)
    #[serde(default)]
    pub length_min: f32,
    /// DCB field: `lengthMax` (Single)
    #[serde(default)]
    pub length_max: f32,
    /// DCB field: `stiffness` (Single)
    #[serde(default)]
    pub stiffness: f32,
    /// DCB field: `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// DCB field: `springBone` (String)
    #[serde(default)]
    pub spring_bone: String,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `rotationalOffset` (Class)
    #[serde(default)]
    pub rotational_offset: Option<Handle<Vec3>>,
}

impl Pooled for VehicleLandingGearSpring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_landing_gear_spring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_landing_gear_spring }
}

impl<'a> Extract<'a> for VehicleLandingGearSpring {
    const TYPE_NAME: &'static str = "VehicleLandingGearSpring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            length_target: inst.get_f32("lengthTarget").unwrap_or_default(),
            length_min: inst.get_f32("lengthMin").unwrap_or_default(),
            length_max: inst.get_f32("lengthMax").unwrap_or_default(),
            stiffness: inst.get_f32("stiffness").unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            spring_bone: inst.get_str("springBone").map(String::from).unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotational_offset: match inst.get("rotationalOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleSalvageGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSalvageGlobalParams {
    /// DCB field: `vehicleFractureForceScale` (Single)
    #[serde(default)]
    pub vehicle_fracture_force_scale: f32,
    /// DCB field: `structuralVFX` (Class)
    #[serde(default)]
    pub structural_vfx: Option<Handle<SSalvageGlobalStructuralVFXParams>>,
    /// DCB field: `structuralHighlights` (Class)
    #[serde(default)]
    pub structural_highlights: Option<Handle<SSalvageGlobalStructuralHighlightParams>>,
}

impl Pooled for VehicleSalvageGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_salvage_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_salvage_global_params }
}

impl<'a> Extract<'a> for VehicleSalvageGlobalParams {
    const TYPE_NAME: &'static str = "VehicleSalvageGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_fracture_force_scale: inst.get_f32("vehicleFractureForceScale").unwrap_or_default(),
            structural_vfx: match inst.get("structuralVFX") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageGlobalStructuralVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageGlobalStructuralVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            structural_highlights: match inst.get("structuralHighlights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSalvageGlobalStructuralHighlightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSalvageGlobalStructuralHighlightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleLandingGear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGear {
    /// DCB field: `scopeContext` (String)
    #[serde(default)]
    pub scope_context: String,
    /// DCB field: `vehicleScopeContext` (String)
    #[serde(default)]
    pub vehicle_scope_context: String,
    /// DCB field: `bone` (String)
    #[serde(default)]
    pub bone: String,
    /// DCB field: `mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// DCB field: `alwaysVisible` (Boolean)
    #[serde(default)]
    pub always_visible: bool,
    /// DCB field: `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `spring` (StrongPointer)
    #[serde(default)]
    pub spring: Option<Handle<VehicleLandingGearSpring>>,
}

impl Pooled for VehicleLandingGear {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_landing_gear }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_landing_gear }
}

impl<'a> Extract<'a> for VehicleLandingGear {
    const TYPE_NAME: &'static str = "VehicleLandingGear";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope_context: inst.get_str("scopeContext").map(String::from).unwrap_or_default(),
            vehicle_scope_context: inst.get_str("vehicleScopeContext").map(String::from).unwrap_or_default(),
            bone: inst.get_str("bone").map(String::from).unwrap_or_default(),
            mass: inst.get_f32("mass").unwrap_or_default(),
            always_visible: inst.get_bool("alwaysVisible").unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spring: match inst.get("spring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VehicleLandingGearSpring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VehicleLandingGearSpring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleLandingGearSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGearSystem {
    /// DCB field: `fragment` (String)
    #[serde(default)]
    pub fragment: String,
    /// DCB field: `fragmentDeploy` (String)
    #[serde(default)]
    pub fragment_deploy: String,
    /// DCB field: `fragmentRetract` (String)
    #[serde(default)]
    pub fragment_retract: String,
    /// DCB field: `fragmentCompress` (String)
    #[serde(default)]
    pub fragment_compress: String,
    /// DCB field: `altitudeToExtraGears` (Single)
    #[serde(default)]
    pub altitude_to_extra_gears: f32,
    /// DCB field: `gears` (StrongPointer (array))
    #[serde(default)]
    pub gears: Vec<Handle<VehicleLandingGear>>,
}

impl Pooled for VehicleLandingGearSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_landing_gear_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_landing_gear_system }
}

impl<'a> Extract<'a> for VehicleLandingGearSystem {
    const TYPE_NAME: &'static str = "VehicleLandingGearSystem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragment: inst.get_str("fragment").map(String::from).unwrap_or_default(),
            fragment_deploy: inst.get_str("fragmentDeploy").map(String::from).unwrap_or_default(),
            fragment_retract: inst.get_str("fragmentRetract").map(String::from).unwrap_or_default(),
            fragment_compress: inst.get_str("fragmentCompress").map(String::from).unwrap_or_default(),
            altitude_to_extra_gears: inst.get_f32("altitudeToExtraGears").unwrap_or_default(),
            gears: inst.get_array("gears")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VehicleLandingGear>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VehicleLandingGear>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleRole`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRole {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for VehicleRole {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_role }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_role }
}

impl<'a> Extract<'a> for VehicleRole {
    const TYPE_NAME: &'static str = "VehicleRole";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleCareer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleCareer {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `roleList` (Reference (array))
    #[serde(default)]
    pub role_list: Vec<CigGuid>,
}

impl Pooled for VehicleCareer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_career }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_career }
}

impl<'a> Extract<'a> for VehicleCareer {
    const TYPE_NAME: &'static str = "VehicleCareer";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            role_list: inst.get_array("roleList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleCareerList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleCareerList {
    /// DCB field: `careerList` (Reference (array))
    #[serde(default)]
    pub career_list: Vec<CigGuid>,
}

impl Pooled for VehicleCareerList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ve.vehicle_career_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ve.vehicle_career_list }
}

impl<'a> Extract<'a> for VehicleCareerList {
    const TYPE_NAME: &'static str = "VehicleCareerList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            career_list: inst.get_array("careerList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

