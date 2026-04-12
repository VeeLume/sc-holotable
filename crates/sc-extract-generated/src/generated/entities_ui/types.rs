// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-ui`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `FrontendOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendOverrideParams {
    /// `persistentUniverseActive` (Boolean)
    #[serde(default)]
    pub persistent_universe_active: bool,
    /// `arenaCommanderActive` (Boolean)
    #[serde(default)]
    pub arena_commander_active: bool,
    /// `tutorialDisabled` (Boolean)
    #[serde(default)]
    pub tutorial_disabled: bool,
    /// `disableResidenceSelectionWarning` (Boolean)
    #[serde(default)]
    pub disable_residence_selection_warning: bool,
    /// `backgroundVideoPath` (String)
    #[serde(default)]
    pub background_video_path: String,
    /// `disabledSystems` (Reference (array))
    #[serde(default)]
    pub disabled_systems: Vec<CigGuid>,
}

impl Pooled for FrontendOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ui.frontend_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ui.frontend_override_params }
}

impl<'a> Extract<'a> for FrontendOverrideParams {
    const TYPE_NAME: &'static str = "FrontendOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            persistent_universe_active: inst.get_bool("persistentUniverseActive").unwrap_or_default(),
            arena_commander_active: inst.get_bool("arenaCommanderActive").unwrap_or_default(),
            tutorial_disabled: inst.get_bool("tutorialDisabled").unwrap_or_default(),
            disable_residence_selection_warning: inst.get_bool("disableResidenceSelectionWarning").unwrap_or_default(),
            background_video_path: inst.get_str("backgroundVideoPath").map(String::from).unwrap_or_default(),
            disabled_systems: inst.get_array("disabledSystems")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

