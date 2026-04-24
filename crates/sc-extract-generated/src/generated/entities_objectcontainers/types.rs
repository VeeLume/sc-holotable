// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-objectcontainers`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SMovableObjectContainerParams`
/// Inherits from: `EntityClassStaticDataParams`
pub struct SMovableObjectContainerParams {
}

impl Pooled for SMovableObjectContainerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.smovable_object_container_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.smovable_object_container_params }
}

impl<'a> Extract<'a> for SMovableObjectContainerParams {
    const TYPE_NAME: &'static str = "SMovableObjectContainerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SOCInstanceSlot`
pub struct SOCInstanceSlot {
    /// `objectContainerRef` (String)
    pub object_container_ref: String,
}

impl Pooled for SOCInstanceSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.socinstance_slot }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.socinstance_slot }
}

impl<'a> Extract<'a> for SOCInstanceSlot {
    const TYPE_NAME: &'static str = "SOCInstanceSlot";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            object_container_ref: inst.get_str("objectContainerRef").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SOCInstanceComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SOCInstanceComponentParams {
    /// `slots` (Class)
    pub slots: Option<Handle<SOCInstanceSlot>>,
    /// `rulesConfig` (Reference)
    pub rules_config: Option<CigGuid>,
}

impl Pooled for SOCInstanceComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.socinstance_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.socinstance_component_params }
}

impl<'a> Extract<'a> for SOCInstanceComponentParams {
    const TYPE_NAME: &'static str = "SOCInstanceComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            slots: match inst.get("slots") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SOCInstanceSlot>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rules_config: inst.get("rulesConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityComponentProceduralOCModifierParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentProceduralOCModifierParams {
    /// `ExteriorWearFactor` (Single)
    pub exterior_wear_factor: f32,
    /// `ExteriorDirtFactor` (Single)
    pub exterior_dirt_factor: f32,
    /// `InteriorWearFactor` (Single)
    pub interior_wear_factor: f32,
    /// `InteriorDirtFactor` (Single)
    pub interior_dirt_factor: f32,
    /// `OverrideTintPalette` (Reference)
    pub override_tint_palette: Option<CigGuid>,
}

impl Pooled for SEntityComponentProceduralOCModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.sentity_component_procedural_ocmodifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.sentity_component_procedural_ocmodifier_params }
}

impl<'a> Extract<'a> for SEntityComponentProceduralOCModifierParams {
    const TYPE_NAME: &'static str = "SEntityComponentProceduralOCModifierParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            exterior_wear_factor: inst.get_f32("ExteriorWearFactor").unwrap_or_default(),
            exterior_dirt_factor: inst.get_f32("ExteriorDirtFactor").unwrap_or_default(),
            interior_wear_factor: inst.get_f32("InteriorWearFactor").unwrap_or_default(),
            interior_dirt_factor: inst.get_f32("InteriorDirtFactor").unwrap_or_default(),
            override_tint_palette: inst.get("OverrideTintPalette").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SOrbitComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SOrbitComponentParams {
    /// `OrbitalRadius` (Double)
    pub orbital_radius: f64,
    /// `OrbitalSpeed` (Double)
    pub orbital_speed: f64,
    /// `OrbitalAngle` (Double)
    pub orbital_angle: f64,
    /// `parentGUID` (String)
    pub parent_guid: String,
}

impl Pooled for SOrbitComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.sorbit_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.sorbit_component_params }
}

impl<'a> Extract<'a> for SOrbitComponentParams {
    const TYPE_NAME: &'static str = "SOrbitComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            orbital_radius: inst.get_f64("OrbitalRadius").unwrap_or_default(),
            orbital_speed: inst.get_f64("OrbitalSpeed").unwrap_or_default(),
            orbital_angle: inst.get_f64("OrbitalAngle").unwrap_or_default(),
            parent_guid: inst.get_str("parentGUID").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityComponentPhysicsGridParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentPhysicsGridParams {
    /// `PhysGridType` (StrongPointer)
    pub phys_grid_type: Option<SEntityBasePhysicsGridParamsPtr>,
}

impl Pooled for SEntityComponentPhysicsGridParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.sentity_component_physics_grid_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.sentity_component_physics_grid_params }
}

impl<'a> Extract<'a> for SEntityComponentPhysicsGridParams {
    const TYPE_NAME: &'static str = "SEntityComponentPhysicsGridParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            phys_grid_type: match inst.get("PhysGridType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SEntityBasePhysicsGridParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityBasePhysicsGridParams`
pub struct SEntityBasePhysicsGridParams {
    /// `inheritGravity` (Boolean)
    pub inherit_gravity: bool,
    /// `gravity` (Class)
    pub gravity: Option<Handle<Vec3>>,
    /// `gridType` (EnumChoice)
    pub grid_type: GRID_TYPE,
    /// `cellSize` (Single)
    pub cell_size: f32,
    /// `gridPartsOnly` (Boolean)
    pub grid_parts_only: bool,
    /// `portalExclusiveMode` (Boolean)
    pub portal_exclusive_mode: bool,
}

impl Pooled for SEntityBasePhysicsGridParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_objectcontainers.sentity_base_physics_grid_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_objectcontainers.sentity_base_physics_grid_params }
}

impl<'a> Extract<'a> for SEntityBasePhysicsGridParams {
    const TYPE_NAME: &'static str = "SEntityBasePhysicsGridParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            inherit_gravity: inst.get_bool("inheritGravity").unwrap_or_default(),
            gravity: match inst.get("gravity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            grid_type: GRID_TYPE::from_dcb_str(inst.get_str("gridType").unwrap_or("")),
            cell_size: inst.get_f32("cellSize").unwrap_or_default(),
            grid_parts_only: inst.get_bool("gridPartsOnly").unwrap_or_default(),
            portal_exclusive_mode: inst.get_bool("portalExclusiveMode").unwrap_or_default(),
        }
    }
}

