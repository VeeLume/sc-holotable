// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-entityclassdefinition_test_gadget`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SCItemDeployableShieldParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCItemDeployableShieldParams {
    /// `shield` (String)
    pub shield: String,
    /// `startupVFX` (String)
    pub startup_vfx: String,
    /// `destroyedVFX` (String)
    pub destroyed_vfx: String,
    /// `openAnim` (String)
    pub open_anim: String,
    /// `closedAnim` (String)
    pub closed_anim: String,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
}

impl Pooled for SCItemDeployableShieldParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_entityclassdefinition_test_gadget.scitem_deployable_shield_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_entityclassdefinition_test_gadget.scitem_deployable_shield_params }
}

impl<'a> Extract<'a> for SCItemDeployableShieldParams {
    const TYPE_NAME: &'static str = "SCItemDeployableShieldParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shield: inst.get_str("shield").map(String::from).unwrap_or_default(),
            startup_vfx: inst.get_str("startupVFX").map(String::from).unwrap_or_default(),
            destroyed_vfx: inst.get_str("destroyedVFX").map(String::from).unwrap_or_default(),
            open_anim: inst.get_str("openAnim").map(String::from).unwrap_or_default(),
            closed_anim: inst.get_str("closedAnim").map(String::from).unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

