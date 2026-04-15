// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_pointofinterestprovider`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EntityComponentPointOfInterestProvider`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponentPointOfInterestProvider {
    /// `pointsOfInterestList` (Reference)
    #[serde(default)]
    pub points_of_interest_list: Option<CigGuid>,
}

impl Pooled for EntityComponentPointOfInterestProvider {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_pointofinterestprovider.entity_component_point_of_interest_provider }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_pointofinterestprovider.entity_component_point_of_interest_provider }
}

impl<'a> Extract<'a> for EntityComponentPointOfInterestProvider {
    const TYPE_NAME: &'static str = "EntityComponentPointOfInterestProvider";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            points_of_interest_list: inst.get("pointsOfInterestList").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

