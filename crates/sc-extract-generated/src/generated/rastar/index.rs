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

/// Record index for the `rastar` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RastarIndex {
    #[serde(default)]
    pub hologram_params: HashMap<CigGuid, Handle<HologramParams>>,
    #[serde(default)]
    pub ra_sta_rlibrary_element: HashMap<CigGuid, Handle<RaSTaRLibraryElement>>,
    #[serde(default)]
    pub ra_sta_rlibrary: HashMap<CigGuid, Handle<RaSTaRLibrary>>,
}

impl RastarIndex {
    pub fn len(&self) -> usize {
        self.hologram_params.len()
            + self.ra_sta_rlibrary_element.len()
            + self.ra_sta_rlibrary.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
