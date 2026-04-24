// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `entities-scitem-ships` feature.
#[derive(Default)]
pub struct EntitiesScitemShipsIndex {
    pub sifcsmodifiers_legacy: HashMap<CigGuid, Handle<SIFCSModifiersLegacy>>,
    pub smfdparams_diagnostics: HashMap<CigGuid, Handle<SMFDParamsDiagnostics>>,
}

impl EntitiesScitemShipsIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.sifcsmodifiers_legacy.len();
        total += self.smfdparams_diagnostics.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
