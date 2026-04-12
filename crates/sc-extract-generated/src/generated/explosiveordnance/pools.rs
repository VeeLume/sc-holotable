// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `explosiveordnance` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExplosiveordnancePools {
    #[serde(default)]
    pub explosive_ordnance_ping_vfx: Vec<Option<ExplosiveOrdnancePingVFX>>,
    #[serde(default)]
    pub explosive_ordnance_ping_global_params: Vec<Option<ExplosiveOrdnancePingGlobalParams>>,
}
