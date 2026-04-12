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

/// Pool storage for the `awardservice` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwardservicePools {
    #[serde(default)]
    pub award_service_award: Vec<Option<AwardService_Award>>,
    #[serde(default)]
    pub award_service_config: Vec<Option<AwardService_Config>>,
}
