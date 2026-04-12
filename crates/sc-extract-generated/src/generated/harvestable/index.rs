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

/// Record index for the `harvestable` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarvestableIndex {
    #[serde(default)]
    pub sub_harvestable_config_record: HashMap<CigGuid, Handle<SubHarvestableConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_record: HashMap<CigGuid, Handle<SubHarvestableMultiConfigRecord>>,
    #[serde(default)]
    pub harvestable_preset: HashMap<CigGuid, Handle<HarvestablePreset>>,
    #[serde(default)]
    pub harvestable_setup: HashMap<CigGuid, Handle<HarvestableSetup>>,
    #[serde(default)]
    pub harvestable_cluster_preset: HashMap<CigGuid, Handle<HarvestableClusterPreset>>,
    #[serde(default)]
    pub harvestable_provider_preset: HashMap<CigGuid, Handle<HarvestableProviderPreset>>,
}

impl HarvestableIndex {
    pub fn len(&self) -> usize {
        self.sub_harvestable_config_record.len()
            + self.sub_harvestable_multi_config_record.len()
            + self.harvestable_preset.len()
            + self.harvestable_setup.len()
            + self.harvestable_cluster_preset.len()
            + self.harvestable_provider_preset.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
