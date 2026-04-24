// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-restricted_areas`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `RestrictedAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct RestrictedAreaComponentParams {
    /// `state` (EnumChoice)
    pub state: RestrictedAreaState,
    /// `timeToDespawn` (Single)
    pub time_to_despawn: f32,
    /// `affectShips` (Boolean)
    pub affect_ships: bool,
    /// `affectActorsOnFoot` (Boolean)
    pub affect_actors_on_foot: bool,
    /// `affectGroundVehicles` (Boolean)
    pub affect_ground_vehicles: bool,
    /// `allowTagsOnLoad` (Reference (array))
    pub allow_tags_on_load: Vec<CigGuid>,
}

impl Pooled for RestrictedAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_restricted_areas
            .restricted_area_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_restricted_areas
            .restricted_area_component_params
    }
}

impl<'a> Extract<'a> for RestrictedAreaComponentParams {
    const TYPE_NAME: &'static str = "RestrictedAreaComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state: RestrictedAreaState::from_dcb_str(inst.get_str("state").unwrap_or("")),
            time_to_despawn: inst.get_f32("timeToDespawn").unwrap_or_default(),
            affect_ships: inst.get_bool("affectShips").unwrap_or_default(),
            affect_actors_on_foot: inst.get_bool("affectActorsOnFoot").unwrap_or_default(),
            affect_ground_vehicles: inst.get_bool("affectGroundVehicles").unwrap_or_default(),
            allow_tags_on_load: inst
                .get_array("allowTagsOnLoad")
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
        }
    }
}
