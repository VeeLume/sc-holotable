// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `scitemdisplayscreenpreset`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCItemDisplayScreenPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemDisplayScreenPreset {
    /// `material` (String)
    #[serde(default)]
    pub material: String,
    /// `geometryPath` (String)
    #[serde(default)]
    pub geometry_path: String,
}

impl Pooled for SCItemDisplayScreenPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scitemdisplayscreenpreset.scitem_display_screen_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scitemdisplayscreenpreset.scitem_display_screen_preset }
}

impl<'a> Extract<'a> for SCItemDisplayScreenPreset {
    const TYPE_NAME: &'static str = "SCItemDisplayScreenPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            material: inst.get_str("material").map(String::from).unwrap_or_default(),
            geometry_path: inst.get_str("geometryPath").map(String::from).unwrap_or_default(),
        }
    }
}

