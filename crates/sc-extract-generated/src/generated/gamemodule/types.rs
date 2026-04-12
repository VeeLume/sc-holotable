// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `gamemodule`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `GameModule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModule {
    /// `moduleCode` (String)
    #[serde(default)]
    pub module_code: String,
    /// `rankTexturePrefix` (String)
    #[serde(default)]
    pub rank_texture_prefix: String,
}

impl Pooled for GameModule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemodule.game_module }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemodule.game_module }
}

impl<'a> Extract<'a> for GameModule {
    const TYPE_NAME: &'static str = "GameModule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            module_code: inst.get_str("moduleCode").map(String::from).unwrap_or_default(),
            rank_texture_prefix: inst.get_str("rankTexturePrefix").map(String::from).unwrap_or_default(),
        }
    }
}

