// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `squantumdriveeffecttagstemplate`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SQuantumDriveEffectParams_LEGACY`
/// Inherits from: `SQuantumDriveEffectBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumDriveEffectParams_LEGACY {
    /// `alignEffectTag` (Reference)
    #[serde(default)]
    pub align_effect_tag: Option<CigGuid>,
    /// `pinchEffectTag` (Reference)
    #[serde(default)]
    pub pinch_effect_tag: Option<CigGuid>,
    /// `travelEffectTag` (Reference)
    #[serde(default)]
    pub travel_effect_tag: Option<CigGuid>,
    /// `enterFlashEffectTag` (Reference)
    #[serde(default)]
    pub enter_flash_effect_tag: Option<CigGuid>,
    /// `exitFlashEffectTag` (Reference)
    #[serde(default)]
    pub exit_flash_effect_tag: Option<CigGuid>,
    /// `spoolEffectTag` (Reference)
    #[serde(default)]
    pub spool_effect_tag: Option<CigGuid>,
    /// `pinchStrengthTag` (Reference)
    #[serde(default)]
    pub pinch_strength_tag: Option<CigGuid>,
    /// `spoolStrengthTag` (Reference)
    #[serde(default)]
    pub spool_strength_tag: Option<CigGuid>,
    /// `trailsTag` (Reference)
    #[serde(default)]
    pub trails_tag: Option<CigGuid>,
    /// `trailsStrTag` (Reference)
    #[serde(default)]
    pub trails_str_tag: Option<CigGuid>,
    /// `interdictionEffectTag` (Reference)
    #[serde(default)]
    pub interdiction_effect_tag: Option<CigGuid>,
    /// `interdictionExitFlashTag` (Reference)
    #[serde(default)]
    pub interdiction_exit_flash_tag: Option<CigGuid>,
}

impl Pooled for SQuantumDriveEffectParams_LEGACY {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.squantumdriveeffecttagstemplate.squantum_drive_effect_params_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.squantumdriveeffecttagstemplate.squantum_drive_effect_params_legacy }
}

impl<'a> Extract<'a> for SQuantumDriveEffectParams_LEGACY {
    const TYPE_NAME: &'static str = "SQuantumDriveEffectParams_LEGACY";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            align_effect_tag: inst.get("alignEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pinch_effect_tag: inst.get("pinchEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            travel_effect_tag: inst.get("travelEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            enter_flash_effect_tag: inst.get("enterFlashEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            exit_flash_effect_tag: inst.get("exitFlashEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            spool_effect_tag: inst.get("spoolEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pinch_strength_tag: inst.get("pinchStrengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            spool_strength_tag: inst.get("spoolStrengthTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trails_tag: inst.get("trailsTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trails_str_tag: inst.get("trailsStrTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interdiction_effect_tag: inst.get("interdictionEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interdiction_exit_flash_tag: inst.get("interdictionExitFlashTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SQuantumDriveEffectTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQuantumDriveEffectTemplate {
    /// `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<SQuantumDriveEffectParams_LEGACY>>,
}

impl Pooled for SQuantumDriveEffectTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.squantumdriveeffecttagstemplate.squantum_drive_effect_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.squantumdriveeffecttagstemplate.squantum_drive_effect_template }
}

impl<'a> Extract<'a> for SQuantumDriveEffectTemplate {
    const TYPE_NAME: &'static str = "SQuantumDriveEffectTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SQuantumDriveEffectParams_LEGACY>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SQuantumDriveEffectParams_LEGACY>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

