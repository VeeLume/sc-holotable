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

/// DCB type: `CustomScanProcedureParams`
///
/// Inherits from: `ScanProcedureParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomScanProcedureParams {
    /// DCB field: `requiredScanTag` (Reference)
    #[serde(default)]
    pub required_scan_tag: Option<CigGuid>,
    /// DCB field: `emissionBaseline` (Single)
    #[serde(default)]
    pub emission_baseline: f32,
    /// DCB field: `runtimeDuration` (Single)
    #[serde(default)]
    pub runtime_duration: f32,
    /// DCB field: `allowedInAIAutoScan` (Boolean)
    #[serde(default)]
    pub allowed_in_aiauto_scan: bool,
    /// DCB field: `allowedInFocalPointScan` (Boolean)
    #[serde(default)]
    pub allowed_in_focal_point_scan: bool,
    /// DCB field: `allowedInPingBroadScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_broad_scan: bool,
    /// DCB field: `allowedInPingFocusScan` (Boolean)
    #[serde(default)]
    pub allowed_in_ping_focus_scan: bool,
    /// DCB field: `allowedInPassiveScan` (Boolean)
    #[serde(default)]
    pub allowed_in_passive_scan: bool,
    /// DCB field: `customDatas` (StrongPointer (array))
    #[serde(default)]
    pub custom_datas: Vec<Handle<ScanCustomData>>,
}

impl Pooled for CustomScanProcedureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_cu.custom_scan_procedure_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_cu.custom_scan_procedure_params }
}

impl<'a> Extract<'a> for CustomScanProcedureParams {
    const TYPE_NAME: &'static str = "CustomScanProcedureParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            required_scan_tag: inst.get("requiredScanTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            emission_baseline: inst.get_f32("emissionBaseline").unwrap_or_default(),
            runtime_duration: inst.get_f32("runtimeDuration").unwrap_or_default(),
            allowed_in_aiauto_scan: inst.get_bool("allowedInAIAutoScan").unwrap_or_default(),
            allowed_in_focal_point_scan: inst.get_bool("allowedInFocalPointScan").unwrap_or_default(),
            allowed_in_ping_broad_scan: inst.get_bool("allowedInPingBroadScan").unwrap_or_default(),
            allowed_in_ping_focus_scan: inst.get_bool("allowedInPingFocusScan").unwrap_or_default(),
            allowed_in_passive_scan: inst.get_bool("allowedInPassiveScan").unwrap_or_default(),
            custom_datas: inst.get_array("customDatas")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ScanCustomData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ScanCustomData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

