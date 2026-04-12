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

/// Pool storage for the `ui-graphs` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiGraphsPools {
    #[serde(default)]
    pub ctx_graph_node: Vec<Option<CtxGraph_Node>>,
    #[serde(default)]
    pub ctx_graph_dependency: Vec<Option<CtxGraph_Dependency>>,
    #[serde(default)]
    pub ctx_graph_group: Vec<Option<CtxGraph_Group>>,
    #[serde(default)]
    pub ctx_graph_context: Vec<Option<CtxGraph_Context>>,
    #[serde(default)]
    pub ctx_graph_component: Vec<Option<CtxGraph_Component>>,
    #[serde(default)]
    pub ctx_graph: Vec<Option<CtxGraph>>,
}
