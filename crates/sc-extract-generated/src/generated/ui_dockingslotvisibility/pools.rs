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

/// Pool storage for the `ui-dockingslotvisibility` feature.
#[derive(Default)]
pub struct UiDockingslotvisibilityPools {
    pub docking_slot_visibility_tag_set: Vec<Option<DockingSlotVisibilityTagSet>>,
    pub docking_slot_visibility_rule: Vec<Option<DockingSlotVisibilityRule>>,
    pub docking_slot_visibility: Vec<Option<DockingSlotVisibility>>,
}
