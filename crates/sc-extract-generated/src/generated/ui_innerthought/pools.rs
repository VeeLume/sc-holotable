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

/// Pool storage for the `ui-innerthought` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiInnerthoughtPools {
    #[serde(default)]
    pub inner_thought_cycle_anim_base: Vec<Option<InnerThought_CycleAnimBase>>,
    #[serde(default)]
    pub inner_thought_cycle_anim_rotate_x: Vec<Option<InnerThought_CycleAnimRotateX>>,
    #[serde(default)]
    pub inner_thought_cycle_anim_rotate_y: Vec<Option<InnerThought_CycleAnimRotateY>>,
    #[serde(default)]
    pub inner_thought_cycle_anim_rotate_z: Vec<Option<InnerThought_CycleAnimRotateZ>>,
    #[serde(default)]
    pub inner_thought_layout_grid_set_thought: Vec<Option<InnerThought_LayoutGridSetThought>>,
    #[serde(default)]
    pub inner_thought_layout_grid_set: Vec<Option<InnerThought_LayoutGridSet>>,
    #[serde(default)]
    pub inner_thought_layout_grid: Vec<Option<InnerThought_LayoutGrid>>,
    #[serde(default)]
    pub inner_thought_layout_curve: Vec<Option<InnerThought_LayoutCurve>>,
    #[serde(default)]
    pub inner_thought_layout_pit: Vec<Option<InnerThought_LayoutPIT>>,
    #[serde(default)]
    pub inner_thought_legacy_use_system_config: Vec<Option<InnerThought_LegacyUseSystemConfig>>,
}
