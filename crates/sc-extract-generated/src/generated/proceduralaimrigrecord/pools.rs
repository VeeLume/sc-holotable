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

/// Pool storage for the `proceduralaimrigrecord` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProceduralaimrigrecordPools {
    #[serde(default)]
    pub proc_aim_base_joint_type_config: Vec<Option<ProcAimBaseJointTypeConfig>>,
    #[serde(default)]
    pub proc_aim_rig_config: Vec<Option<ProcAimRigConfig>>,
    #[serde(default)]
    pub procedural_aim_rig_record: Vec<Option<ProceduralAimRigRecord>>,
}
