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

/// Record index for the `motionstatemachine` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MotionstatemachineIndex {
    #[serde(default)]
    pub motion_graph: HashMap<CigGuid, Handle<MotionGraph>>,
    #[serde(default)]
    pub scprone_motion_graph_def: HashMap<CigGuid, Handle<SCProneMotionGraphDef>>,
    #[serde(default)]
    pub smannequin_action_def_record: HashMap<CigGuid, Handle<SMannequinActionDefRecord>>,
}

impl MotionstatemachineIndex {
    pub fn len(&self) -> usize {
        self.motion_graph.len()
            + self.scprone_motion_graph_def.len()
            + self.smannequin_action_def_record.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
