// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `densityclasses` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DensityclassesPools {
    #[serde(default)]
    pub sentity_density_class: Vec<Option<SEntityDensityClass>>,
    #[serde(default)]
    pub sdensity_class_lifetime_override_entry: Vec<Option<SDensityClassLifetimeOverrideEntry>>,
    #[serde(default)]
    pub sdefault_density_class_lifetime_overrides: Vec<Option<SDefaultDensityClassLifetimeOverrides>>,
    #[serde(default)]
    pub sentity_density_class_overrides_record: Vec<Option<SEntityDensityClassOverridesRecord>>,
    #[serde(default)]
    pub sentity_density_class_overrides_manual: Vec<Option<SEntityDensityClassOverridesManual>>,
    #[serde(default)]
    pub time_value_base: Vec<Option<TimeValue_Base>>,
}
