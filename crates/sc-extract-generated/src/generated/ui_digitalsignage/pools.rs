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

/// Pool storage for the `ui-digitalsignage` feature.
#[derive(Default)]
pub struct UiDigitalsignagePools {
    pub digital_signage_content: Vec<Option<DigitalSignageContent>>,
    pub digital_signage_content_set: Vec<Option<DigitalSignageContentSet>>,
}
