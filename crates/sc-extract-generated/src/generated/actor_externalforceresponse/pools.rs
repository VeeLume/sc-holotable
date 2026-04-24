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

/// Pool storage for the `actor-externalforceresponse` feature.
#[derive(Default)]
pub struct ActorExternalforceresponsePools {
    pub sactor_force_reaction_procedural_xian_lean_pose:
        Vec<Option<SActorForceReactionProceduralXianLeanPose>>,
    pub sactor_force_reaction_procedural_xian_lean_pose_list:
        Vec<Option<SActorForceReactionProceduralXianLeanPoseList>>,
    pub sactor_force_reactions_procedural_lean_override:
        Vec<Option<SActorForceReactionsProceduralLeanOverride>>,
    pub sactor_force_reactions_preset_record: Vec<Option<SActorForceReactionsPresetRecord>>,
}
