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

/// Pool storage for the `tintpalettes` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TintpalettesPools {
    #[serde(default)]
    pub tint_entry: Vec<Option<TintEntry>>,
    #[serde(default)]
    pub tint_palette: Vec<Option<TintPalette>>,
    #[serde(default)]
    pub tint_palette_tree: Vec<Option<TintPaletteTree>>,
}
