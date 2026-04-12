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

/// Pool storage for the `zerogtraversalgraph` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZerogtraversalgraphPools {
    #[serde(default)]
    pub zero_gtraversal_connection: Vec<Option<ZeroGTraversalConnection>>,
    #[serde(default)]
    pub zero_gtraversal_state: Vec<Option<ZeroGTraversalState>>,
    #[serde(default)]
    pub zero_gtraversal_graph: Vec<Option<ZeroGTraversalGraph>>,
}
