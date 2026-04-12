// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

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
    pub fn len(&self) -> usize {
        self.ifcs_input_deflection_time_params.len()
            + self.turret_input_deflection_time_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
