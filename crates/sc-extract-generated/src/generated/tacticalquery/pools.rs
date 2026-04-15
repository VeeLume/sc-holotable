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

/// Pool storage for the `tacticalquery` feature.
#[derive(Default)]
pub struct TacticalqueryPools {
    pub tqsinput_int_value: Vec<Option<TQSInputIntValue>>,
    pub tqsinput_dynamic_variable_value: Vec<Option<TQSInputDynamicVariableValue>>,
    pub tqsweight_input_float_value: Vec<Option<TQSWeightInputFloatValue>>,
    pub tqsweight_input_tag_value: Vec<Option<TQSWeightInputTagValue>>,
    pub tqsoption_content_record: Vec<Option<TQSOptionContentRecord>>,
    pub tqsoption_reference: Vec<Option<TQSOptionReference>>,
}
