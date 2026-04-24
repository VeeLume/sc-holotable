// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `procedurallayout` feature.
#[derive(Default)]
pub struct ProcedurallayoutPools {
    pub faction_palettes: Vec<Option<FactionPalettes>>,
    pub faction_palette: Vec<Option<FactionPalette>>,
    pub procedural_layout_node_start: Vec<Option<ProceduralLayoutNode_Start>>,
    pub procedural_layout_node_element_properties: Vec<Option<ProceduralLayoutNode_ElementProperties>>,
    pub procedural_layout_graph_node_element: Vec<Option<ProceduralLayoutGraphNode_Element>>,
    pub procedural_layout_graph_node_multi_element: Vec<Option<ProceduralLayoutGraphNode_MultiElement>>,
    pub procedural_layout_supplementary_element_tags_options: Vec<Option<ProceduralLayout_SupplementaryElementTagsOptions>>,
    pub procedural_layout_tag_filter: Vec<Option<ProceduralLayout_TagFilter>>,
    pub procedural_layout_graph: Vec<Option<ProceduralLayoutGraph>>,
}
