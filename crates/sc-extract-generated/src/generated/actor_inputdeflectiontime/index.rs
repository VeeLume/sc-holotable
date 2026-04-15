// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `actor-inputdeflectiontime` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorInputdeflectiontimeIndex {
    #[serde(default)]
    pub ifcs_input_deflection_time_params: HashMap<CigGuid, Handle<IfcsInputDeflectionTimeParams>>,
    #[serde(default)]
    pub turret_input_deflection_time_params: HashMap<CigGuid, Handle<TurretInputDeflectionTimeParams>>,
}

impl ActorInputdeflectiontimeIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.ifcs_input_deflection_time_params.len();
        total += self.turret_input_deflection_time_params.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
