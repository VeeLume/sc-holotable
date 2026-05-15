// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `cargomanifest`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CargoFillCapacityValue_Random`
/// Inherits from: `BaseCargoFillCapacityValue`
pub struct CargoFillCapacityValue_Random {
    /// `resources` (Class (array))
    pub resources: Vec<Handle<CargoResource>>,
    /// `minCapacityRange` (Single)
    pub min_capacity_range: f32,
    /// `maxCapacityRange` (Single)
    pub max_capacity_range: f32,
}

impl Pooled for CargoFillCapacityValue_Random {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_fill_capacity_value_random
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_fill_capacity_value_random
    }
}

impl<'a> Extract<'a> for CargoFillCapacityValue_Random {
    const TYPE_NAME: &'static str = "CargoFillCapacityValue_Random";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            resources: inst
                .get_array("resources")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<CargoResource>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<CargoResource>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            min_capacity_range: inst.get_f32("minCapacityRange").unwrap_or_default(),
            max_capacity_range: inst.get_f32("maxCapacityRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoFillCapacityValue_Custom`
/// Inherits from: `BaseCargoFillCapacityValue`
pub struct CargoFillCapacityValue_Custom {
    /// `resources` (Class (array))
    pub resources: Vec<Handle<CargoResourceAllocation>>,
}

impl Pooled for CargoFillCapacityValue_Custom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_fill_capacity_value_custom
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_fill_capacity_value_custom
    }
}

impl<'a> Extract<'a> for CargoFillCapacityValue_Custom {
    const TYPE_NAME: &'static str = "CargoFillCapacityValue_Custom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            resources: inst
                .get_array("resources")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<CargoResourceAllocation>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<CargoResourceAllocation>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoResourceAllocation`
pub struct CargoResourceAllocation {
    /// `resources` (Class)
    pub resources: Option<Handle<CargoResource>>,
    /// `minResourceAllocationPercentage` (Single)
    pub min_resource_allocation_percentage: f32,
    /// `maxResourceAllocationPercentage` (Single)
    pub max_resource_allocation_percentage: f32,
}

impl Pooled for CargoResourceAllocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_resource_allocation
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_resource_allocation
    }
}

impl<'a> Extract<'a> for CargoResourceAllocation {
    const TYPE_NAME: &'static str = "CargoResourceAllocation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            resources: match inst.get("resources") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoResource>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            min_resource_allocation_percentage: inst
                .get_f32("minResourceAllocationPercentage")
                .unwrap_or_default(),
            max_resource_allocation_percentage: inst
                .get_f32("maxResourceAllocationPercentage")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoManifestFixedQuantityDef`
/// Inherits from: `BaseCargoManifestQuantityTypeDef`
pub struct CargoManifestFixedQuantityDef {
    /// `quantity` (Int32)
    pub quantity: i32,
}

impl Pooled for CargoManifestFixedQuantityDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_manifest_fixed_quantity_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_manifest_fixed_quantity_def
    }
}

impl<'a> Extract<'a> for CargoManifestFixedQuantityDef {
    const TYPE_NAME: &'static str = "CargoManifestFixedQuantityDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            quantity: inst.get_i32("quantity").unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoManifestProportionQuantityDef`
/// Inherits from: `BaseCargoManifestQuantityTypeDef`
pub struct CargoManifestProportionQuantityDef {
    /// `proportion` (Single)
    pub proportion: f32,
}

impl Pooled for CargoManifestProportionQuantityDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_manifest_proportion_quantity_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_manifest_proportion_quantity_def
    }
}

impl<'a> Extract<'a> for CargoManifestProportionQuantityDef {
    const TYPE_NAME: &'static str = "CargoManifestProportionQuantityDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            proportion: inst.get_f32("proportion").unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoSpawnEntityDef`
/// Inherits from: `BaseCargoManifestSpawnRuleDef`
pub struct CargoSpawnEntityDef {
    /// `probability` (Single)
    pub probability: f32,
    /// `entities` (Reference (array))
    pub entities: Vec<CigGuid>,
    /// `quantityAlgorithmType` (StrongPointer)
    pub quantity_algorithm_type: Option<BaseCargoManifestQuantityTypeDefPtr>,
}

impl Pooled for CargoSpawnEntityDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_spawn_entity_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_spawn_entity_def
    }
}

impl<'a> Extract<'a> for CargoSpawnEntityDef {
    const TYPE_NAME: &'static str = "CargoSpawnEntityDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability: inst.get_f32("probability").unwrap_or_default(),
            entities: inst
                .get_array("entities")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            quantity_algorithm_type: match inst.get("quantityAlgorithmType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BaseCargoManifestQuantityTypeDefPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CargoSpawnResourceDef`
/// Inherits from: `BaseCargoManifestSpawnRuleDef`
pub struct CargoSpawnResourceDef {
    /// `probability` (Single)
    pub probability: f32,
    /// `resources` (Reference (array))
    pub resources: Vec<CigGuid>,
    /// `quantityAlgorithmType` (StrongPointer)
    pub quantity_algorithm_type: Option<BaseCargoManifestQuantityTypeDefPtr>,
}

impl Pooled for CargoSpawnResourceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.cargomanifest.cargo_spawn_resource_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.cargomanifest.cargo_spawn_resource_def
    }
}

impl<'a> Extract<'a> for CargoSpawnResourceDef {
    const TYPE_NAME: &'static str = "CargoSpawnResourceDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probability: inst.get_f32("probability").unwrap_or_default(),
            resources: inst
                .get_array("resources")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            quantity_algorithm_type: match inst.get("quantityAlgorithmType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BaseCargoManifestQuantityTypeDefPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}
