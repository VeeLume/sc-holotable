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

/// Pool storage for the `globallogoutparams` feature.
#[derive(Default)]
pub struct GloballogoutparamsPools {
    pub sdclogout_behaviour_def: Vec<Option<SDCLogoutBehaviourDef>>,
    pub sdclogout_rule_entry: Vec<Option<SDCLogoutRuleEntry>>,
    pub sdclogout_rule_description_location: Vec<Option<SDCLogoutRuleDescription_Location>>,
    pub sdclogout_rule_description_location_selector:
        Vec<Option<SDCLogoutRuleDescription_LocationSelector>>,
    pub sdclogout_entity_selector_class_list: Vec<Option<SDCLogoutEntitySelector_ClassList>>,
}
