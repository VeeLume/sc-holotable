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

/// Pool storage for the `sglobalsalvagerepairbeamparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SglobalsalvagerepairbeamparamsPools {
    #[serde(default)]
    pub ssalvage_repair_highlight_color_params: Vec<Option<SSalvageRepairHighlightColorParams>>,
    #[serde(default)]
    pub ssalvage_repair_highlight_outline_values: Vec<Option<SSalvageRepairHighlightOutlineValues>>,
    #[serde(default)]
    pub sitem_type_filter: Vec<Option<SItemTypeFilter>>,
    #[serde(default)]
    pub ssalvage_repair_highlight_params: Vec<Option<SSalvageRepairHighlightParams>>,
    #[serde(default)]
    pub ssalvage_repair_card_params: Vec<Option<SSalvageRepairCardParams>>,
    #[serde(default)]
    pub ssalvage_repair_item_type_localization_pair: Vec<Option<SSalvageRepairItemTypeLocalizationPair>>,
    #[serde(default)]
    pub ssalvage_repair_localization_params: Vec<Option<SSalvageRepairLocalizationParams>>,
    #[serde(default)]
    pub ssalvage_repair_material_params: Vec<Option<SSalvageRepairMaterialParams>>,
    #[serde(default)]
    pub ssalvage_repair_audio_params: Vec<Option<SSalvageRepairAudioParams>>,
    #[serde(default)]
    pub sglobal_salvage_repair_beam_params: Vec<Option<SGlobalSalvageRepairBeamParams>>,
}
