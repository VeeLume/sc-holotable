// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `lawsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LawsystemIndex {
    #[serde(default)]
    pub infraction_definition: HashMap<CigGuid, Handle<InfractionDefinition>>,
    #[serde(default)]
    pub jurisdiction: HashMap<CigGuid, Handle<Jurisdiction>>,
    #[serde(default)]
    pub security_clearance_token: HashMap<CigGuid, Handle<SecurityClearanceToken>>,
    #[serde(default)]
    pub security_network_room_settings: HashMap<CigGuid, Handle<SecurityNetworkRoomSettings>>,
    #[serde(default)]
    pub security_network_manifest: HashMap<CigGuid, Handle<SecurityNetworkManifest>>,
}

impl LawsystemIndex {
    pub fn len(&self) -> usize {
        self.infraction_definition.len()
            + self.jurisdiction.len()
            + self.security_clearance_token.len()
            + self.security_network_room_settings.len()
            + self.security_network_manifest.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
