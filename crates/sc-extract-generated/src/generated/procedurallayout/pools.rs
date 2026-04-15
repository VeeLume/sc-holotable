// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `procedurallayout` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcedurallayoutPools {
    #[serde(default)]
    pub faction_palettes: Vec<Option<FactionPalettes>>,
    #[serde(default)]
    pub faction_palette: Vec<Option<FactionPalette>>,
    #[serde(default)]
    pub procedural_layout_node_start: Vec<Option<ProceduralLayoutNode_Start>>,
    #[serde(default)]
    pub procedural_layout_node_element_properties: Vec<Option<ProceduralLayoutNode_ElementProperties>>,
    #[serde(default)]
    pub procedural_layout_graph_node_element: Vec<Option<ProceduralLayoutGraphNode_Element>>,
    #[serde(default)]
    pub procedural_layout_graph_node_multi_element: Vec<Option<ProceduralLayoutGraphNode_MultiElement>>,
    #[serde(default)]
    pub procedural_layout_supplementary_element_tags_options: Vec<Option<ProceduralLayout_SupplementaryElementTagsOptions>>,
    #[serde(default)]
    pub procedural_layout_tag_filter: Vec<Option<ProceduralLayout_TagFilter>>,
    #[serde(default)]
    pub procedural_layout_graph: Vec<Option<ProceduralLayoutGraph>>,
}
