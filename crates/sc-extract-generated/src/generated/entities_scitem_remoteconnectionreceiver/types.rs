// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-remoteconnectionreceiver`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SCItemRemoteConnectionParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCItemRemoteConnectionParams {
    /// `availableOperatorModes` (Reference)
    pub available_operator_modes: Option<CigGuid>,
}

impl Pooled for SCItemRemoteConnectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_remoteconnectionreceiver
            .scitem_remote_connection_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_remoteconnectionreceiver
            .scitem_remote_connection_params
    }
}

impl<'a> Extract<'a> for SCItemRemoteConnectionParams {
    const TYPE_NAME: &'static str = "SCItemRemoteConnectionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            available_operator_modes: inst
                .get("availableOperatorModes")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
