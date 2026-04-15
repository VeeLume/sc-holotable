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

/// Pool storage for the `rastar` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RastarPools {
    #[serde(default)]
    pub hologram_params: Vec<Option<HologramParams>>,
    #[serde(default)]
    pub ra_sta_rlibrary_element: Vec<Option<RaSTaRLibraryElement>>,
    #[serde(default)]
    pub ra_sta_rlibrary_category: Vec<Option<RaSTaRLibraryCategory>>,
    #[serde(default)]
    pub ra_sta_rlibrary: Vec<Option<RaSTaRLibrary>>,
}
