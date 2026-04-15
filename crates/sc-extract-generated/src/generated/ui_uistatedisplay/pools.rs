// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `ui-uistatedisplay` feature.
#[derive(Default)]
pub struct UiUistatedisplayPools {
    pub uistate_display_threshold: Vec<Option<UIStateDisplay_Threshold>>,
    pub uistate_display: Vec<Option<UIStateDisplay>>,
}
