// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobaltractorbeamparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `STractorBeamHoloVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STractorBeamHoloVisualParams {
    /// `minAlignmentValidHolo` (Single)
    #[serde(default)]
    pub min_alignment_valid_holo: f32,
    /// `validHoloColor` (Class)
    #[serde(default)]
    pub valid_holo_color: Option<Handle<RGB>>,
    /// `warningHoloColor` (Class)
    #[serde(default)]
    pub warning_holo_color: Option<Handle<RGB>>,
    /// `invalidHoloColor` (Class)
    #[serde(default)]
    pub invalid_holo_color: Option<Handle<RGB>>,
    /// `cargoHoloPreviewColor` (Class)
    #[serde(default)]
    pub cargo_holo_preview_color: Option<Handle<RGBA>>,
    /// `holoOpacity` (Single)
    #[serde(default)]
    pub holo_opacity: f32,
    /// `holoMaterial` (Class)
    #[serde(default)]
    pub holo_material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for STractorBeamHoloVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobaltractorbeamparams.stractor_beam_holo_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobaltractorbeamparams.stractor_beam_holo_visual_params }
}

impl<'a> Extract<'a> for STractorBeamHoloVisualParams {
    const TYPE_NAME: &'static str = "STractorBeamHoloVisualParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_alignment_valid_holo: inst.get_f32("minAlignmentValidHolo").unwrap_or_default(),
            valid_holo_color: match inst.get("validHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            warning_holo_color: match inst.get("warningHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            invalid_holo_color: match inst.get("invalidHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cargo_holo_preview_color: match inst.get("cargoHoloPreviewColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            holo_opacity: inst.get_f32("holoOpacity").unwrap_or_default(),
            holo_material: match inst.get("holoMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STractorBeamOutlineVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STractorBeamOutlineVisualParams {
    /// `minOutlineWidth` (Single)
    #[serde(default)]
    pub min_outline_width: f32,
    /// `maxOutlineWidth` (Single)
    #[serde(default)]
    pub max_outline_width: f32,
    /// `outlineOpacity` (Single)
    #[serde(default)]
    pub outline_opacity: f32,
}

impl Pooled for STractorBeamOutlineVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobaltractorbeamparams.stractor_beam_outline_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobaltractorbeamparams.stractor_beam_outline_visual_params }
}

impl<'a> Extract<'a> for STractorBeamOutlineVisualParams {
    const TYPE_NAME: &'static str = "STractorBeamOutlineVisualParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_outline_width: inst.get_f32("minOutlineWidth").unwrap_or_default(),
            max_outline_width: inst.get_f32("maxOutlineWidth").unwrap_or_default(),
            outline_opacity: inst.get_f32("outlineOpacity").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGlobalTractorBeamParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalTractorBeamParams {
    /// `holoVisualParams` (Class)
    #[serde(default)]
    pub holo_visual_params: Option<Handle<STractorBeamHoloVisualParams>>,
    /// `outlineVisualParams` (Class)
    #[serde(default)]
    pub outline_visual_params: Option<Handle<STractorBeamOutlineVisualParams>>,
    /// `beingTractorBeamedTag` (Reference)
    #[serde(default)]
    pub being_tractor_beamed_tag: Option<CigGuid>,
    /// `contractedTag` (Reference)
    #[serde(default)]
    pub contracted_tag: Option<CigGuid>,
    /// `showLifetimeTag` (Reference)
    #[serde(default)]
    pub show_lifetime_tag: Option<CigGuid>,
    /// `ignoreEntityTag` (Reference)
    #[serde(default)]
    pub ignore_entity_tag: Option<CigGuid>,
    /// `magLockedTag` (Reference)
    #[serde(default)]
    pub mag_locked_tag: Option<CigGuid>,
    /// `checkParentForIgnoreTag` (Boolean)
    #[serde(default)]
    pub check_parent_for_ignore_tag: bool,
}

impl Pooled for SGlobalTractorBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sglobaltractorbeamparams.sglobal_tractor_beam_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sglobaltractorbeamparams.sglobal_tractor_beam_params }
}

impl<'a> Extract<'a> for SGlobalTractorBeamParams {
    const TYPE_NAME: &'static str = "SGlobalTractorBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            holo_visual_params: match inst.get("holoVisualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STractorBeamHoloVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            outline_visual_params: match inst.get("outlineVisualParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STractorBeamOutlineVisualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            being_tractor_beamed_tag: inst.get("beingTractorBeamedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contracted_tag: inst.get("contractedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            show_lifetime_tag: inst.get("showLifetimeTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ignore_entity_tag: inst.get("ignoreEntityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mag_locked_tag: inst.get("magLockedTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            check_parent_for_ignore_tag: inst.get_bool("checkParentForIgnoreTag").unwrap_or_default(),
        }
    }
}

