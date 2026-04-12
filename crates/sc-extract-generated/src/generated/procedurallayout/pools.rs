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

/// Pool storage for the `procedurallayout` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcedurallayoutPools {
    #[serde(default)]
    pub faction_palettes: Vec<Option<FactionPalettes>>,
    #[serde(default)]
    pub faction_palette: Vec<Option<FactionPalette>>,
    #[serde(default)]
    pub procedural_layout_node_base: Vec<Option<ProceduralLayoutNode_Base>>,
    #[serde(default)]
    pub procedural_layout_supplementary_element_tags_options: Vec<Option<ProceduralLayout_SupplementaryElementTagsOptions>>,
    #[serde(default)]
    pub procedural_layout_tag_filter: Vec<Option<ProceduralLayout_TagFilter>>,
    #[serde(default)]
    pub procedural_layout_graph: Vec<Option<ProceduralLayoutGraph>>,
}
