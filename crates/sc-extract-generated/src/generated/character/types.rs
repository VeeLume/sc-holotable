// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `character`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DefaultPlayerLoadoutEntitlementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementParams {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `LoadoutId` (UInt32)
    #[serde(default)]
    pub loadout_id: u32,
    /// `Entitlement` (EnumChoice)
    #[serde(default)]
    pub entitlement: EDefaultEntitlement,
    /// `Loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.default_player_loadout_entitlement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.default_player_loadout_entitlement_params }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementParams {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            loadout_id: inst.get_u32("LoadoutId").unwrap_or_default(),
            entitlement: EDefaultEntitlement::from_dcb_str(inst.get_str("Entitlement").unwrap_or("")),
            loadout: match inst.get("Loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultPlayerLoadoutEntitlementRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPlayerLoadoutEntitlementRecord {
    /// `Loadouts` (Class (array))
    #[serde(default)]
    pub loadouts: Vec<Handle<DefaultPlayerLoadoutEntitlementParams>>,
}

impl Pooled for DefaultPlayerLoadoutEntitlementRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.character.default_player_loadout_entitlement_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.character.default_player_loadout_entitlement_record }
}

impl<'a> Extract<'a> for DefaultPlayerLoadoutEntitlementRecord {
    const TYPE_NAME: &'static str = "DefaultPlayerLoadoutEntitlementRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadouts: inst.get_array("Loadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<DefaultPlayerLoadoutEntitlementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

