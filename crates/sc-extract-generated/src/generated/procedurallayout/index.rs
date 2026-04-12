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

/// Record index for the `procedurallayout` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcedurallayoutIndex {
    #[serde(default)]
    pub faction_palettes: HashMap<CigGuid, Handle<FactionPalettes>>,
    #[serde(default)]
    pub faction_palette: HashMap<CigGuid, Handle<FactionPalette>>,
    #[serde(default)]
    pub procedural_layout_graph: HashMap<CigGuid, Handle<ProceduralLayoutGraph>>,
}

impl ProcedurallayoutIndex {
    pub fn len(&self) -> usize {
        self.faction_palettes.len()
            + self.faction_palette.len()
            + self.procedural_layout_graph.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
