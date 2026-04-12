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

/// DCB type: `ElectricalStateTemplateInternal`
///
/// Inherits from: `ElectricalState` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalStateTemplateInternal {
    /// DCB field: `chargeMod` (EnumChoice)
    #[serde(default)]
    pub charge_mod: String,
    /// DCB field: `charge` (Single)
    #[serde(default)]
    pub charge: f32,
}

impl Pooled for ElectricalStateTemplateInternal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_el.electrical_state_template_internal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_el.electrical_state_template_internal }
}

impl<'a> Extract<'a> for ElectricalStateTemplateInternal {
    const TYPE_NAME: &'static str = "ElectricalStateTemplateInternal";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            charge_mod: inst.get_str("chargeMod").map(String::from).unwrap_or_default(),
            charge: inst.get_f32("charge").unwrap_or_default(),
        }
    }
}

/// DCB type: `ElectricalStateTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalStateTemplate {
    /// DCB field: `state` (Class)
    #[serde(default)]
    pub state: Option<Handle<ElectricalStateTemplateInternal>>,
}

impl Pooled for ElectricalStateTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_el.electrical_state_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_el.electrical_state_template }
}

impl<'a> Extract<'a> for ElectricalStateTemplate {
    const TYPE_NAME: &'static str = "ElectricalStateTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ElectricalStateTemplateInternal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ElectricalStateTemplateInternal>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ElectricalCalculationPropertyRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalCalculationPropertyRange {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `range` (Class)
    #[serde(default)]
    pub range: Option<Handle<Range>>,
}

impl Pooled for ElectricalCalculationPropertyRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_el.electrical_calculation_property_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_el.electrical_calculation_property_range }
}

impl<'a> Extract<'a> for ElectricalCalculationPropertyRange {
    const TYPE_NAME: &'static str = "ElectricalCalculationPropertyRange";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            range: match inst.get("range") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ElectricalBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalBehavior {
    /// DCB field: `lightning` (Class)
    #[serde(default)]
    pub lightning: Option<Handle<LightningBehavior>>,
    /// DCB field: `vehicleEffects` (Class)
    #[serde(default)]
    pub vehicle_effects: Option<Handle<Behavior_ElectricalVehicleEffectParams>>,
}

impl Pooled for ElectricalBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_el.electrical_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_el.electrical_behavior }
}

impl<'a> Extract<'a> for ElectricalBehavior {
    const TYPE_NAME: &'static str = "ElectricalBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lightning: match inst.get("lightning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LightningBehavior>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LightningBehavior>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_effects: match inst.get("vehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_ElectricalVehicleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_ElectricalVehicleEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

