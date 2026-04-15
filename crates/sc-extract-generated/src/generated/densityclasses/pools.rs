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

/// Pool storage for the `densityclasses` feature.
#[derive(Default)]
pub struct DensityclassesPools {
    pub sdensity_class_lifetime_override_entry: Vec<Option<SDensityClassLifetimeOverrideEntry>>,
    pub sdefault_density_class_lifetime_overrides: Vec<Option<SDefaultDensityClassLifetimeOverrides>>,
    pub sentity_density_class_overrides_record: Vec<Option<SEntityDensityClassOverridesRecord>>,
    pub sentity_density_class_overrides_manual: Vec<Option<SEntityDensityClassOverridesManual>>,
}
