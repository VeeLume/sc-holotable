// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sglobalhealingbeamparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SHealingBeamBoneEntryParams`
pub struct SHealingBeamBoneEntryParams {
    /// `boneName` (String)
    pub bone_name: String,
    /// `boneOffset` (Class)
    pub bone_offset: Option<Handle<Vec3>>,
    /// `cardOffset` (Class)
    pub card_offset: Option<Handle<Vec3>>,
}

impl Pooled for SHealingBeamBoneEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalhealingbeamparams
            .shealing_beam_bone_entry_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalhealingbeamparams
            .shealing_beam_bone_entry_params
    }
}

impl<'a> Extract<'a> for SHealingBeamBoneEntryParams {
    const TYPE_NAME: &'static str = "SHealingBeamBoneEntryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone_name: inst
                .get_str("boneName")
                .map(String::from)
                .unwrap_or_default(),
            bone_offset: match inst.get("boneOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            card_offset: match inst.get("cardOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHealingBeamBodyPartHighlightingParams`
pub struct SHealingBeamBodyPartHighlightingParams {
    /// `characterAttachmentName` (String)
    pub character_attachment_name: String,
    /// `zonesToShow` (EnumChoice (array))
    pub zones_to_show: Vec<EMeshChunks>,
}

impl Pooled for SHealingBeamBodyPartHighlightingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalhealingbeamparams
            .shealing_beam_body_part_highlighting_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalhealingbeamparams
            .shealing_beam_body_part_highlighting_params
    }
}

impl<'a> Extract<'a> for SHealingBeamBodyPartHighlightingParams {
    const TYPE_NAME: &'static str = "SHealingBeamBodyPartHighlightingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            character_attachment_name: inst
                .get_str("characterAttachmentName")
                .map(String::from)
                .unwrap_or_default(),
            zones_to_show: inst
                .get_array("zonesToShow")
                .map(|arr| {
                    arr.filter_map(|v| v.as_str().map(EMeshChunks::from_dcb_str))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHealingBeamBodyPartParams`
pub struct SHealingBeamBodyPartParams {
    /// `bodyPart` (Reference)
    pub body_part: Option<CigGuid>,
    /// `displayName` (Locale)
    pub display_name: LocaleKey,
    /// `boneEntry` (Class)
    pub bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// `highlightParams` (Class)
    pub highlight_params: Option<Handle<SHealingBeamBodyPartHighlightingParams>>,
}

impl Pooled for SHealingBeamBodyPartParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .sglobalhealingbeamparams
            .shealing_beam_body_part_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .sglobalhealingbeamparams
            .shealing_beam_body_part_params
    }
}

impl<'a> Extract<'a> for SHealingBeamBodyPartParams {
    const TYPE_NAME: &'static str = "SHealingBeamBodyPartParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            body_part: inst
                .get("bodyPart")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            display_name: inst
                .get_str("displayName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            bone_entry: match inst.get("boneEntry") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            highlight_params: match inst.get("highlightParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SHealingBeamBodyPartHighlightingParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SGlobalHealingBeamParams`
pub struct SGlobalHealingBeamParams {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `medgunTag` (Reference)
    pub medgun_tag: Option<CigGuid>,
    /// `bodyParts` (Class (array))
    pub body_parts: Vec<Handle<SHealingBeamBodyPartParams>>,
    /// `cardDisplayTimeout` (Single)
    pub card_display_timeout: f32,
    /// `limbSwitchTime` (Single)
    pub limb_switch_time: f32,
    /// `cardPosLerpSpeed` (Single)
    pub card_pos_lerp_speed: f32,
    /// `cardClosingLerpSpeedScalar` (Single)
    pub card_closing_lerp_speed_scalar: f32,
    /// `targetModeActorCardBoneEntry` (Class)
    pub target_mode_actor_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// `selfHealModeActorCardBoneEntry` (Class)
    pub self_heal_mode_actor_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// `selfHealModeLimbCardBoneEntry` (Class)
    pub self_heal_mode_limb_card_bone_entry: Option<Handle<SHealingBeamBoneEntryParams>>,
    /// `transparentMaterial` (Class)
    pub transparent_material: Option<Handle<GlobalResourceMaterial>>,
    /// `injuryHighlightColors` (Class)
    pub injury_highlight_colors: Option<Handle<RGB>>,
    /// `highlightOccludedAlpha` (Single)
    pub highlight_occluded_alpha: f32,
    /// `highlightOutlineWidth` (Single)
    pub highlight_outline_width: f32,
    /// `highlightOutlineOnly` (Boolean)
    pub highlight_outline_only: bool,
}

impl Pooled for SGlobalHealingBeamParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.sglobalhealingbeamparams.sglobal_healing_beam_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.sglobalhealingbeamparams.sglobal_healing_beam_params
    }
}

impl<'a> Extract<'a> for SGlobalHealingBeamParams {
    const TYPE_NAME: &'static str = "SGlobalHealingBeamParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            medgun_tag: inst
                .get("medgunTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            body_parts: inst
                .get_array("bodyParts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SHealingBeamBodyPartParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SHealingBeamBodyPartParams>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            card_display_timeout: inst.get_f32("cardDisplayTimeout").unwrap_or_default(),
            limb_switch_time: inst.get_f32("limbSwitchTime").unwrap_or_default(),
            card_pos_lerp_speed: inst.get_f32("cardPosLerpSpeed").unwrap_or_default(),
            card_closing_lerp_speed_scalar: inst
                .get_f32("cardClosingLerpSpeedScalar")
                .unwrap_or_default(),
            target_mode_actor_card_bone_entry: match inst.get("targetModeActorCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            self_heal_mode_actor_card_bone_entry: match inst.get("selfHealModeActorCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            self_heal_mode_limb_card_bone_entry: match inst.get("selfHealModeLimbCardBoneEntry") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SHealingBeamBoneEntryParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            transparent_material: match inst.get("transparentMaterial") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceMaterial>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            injury_highlight_colors: match inst.get("injuryHighlightColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            highlight_occluded_alpha: inst.get_f32("highlightOccludedAlpha").unwrap_or_default(),
            highlight_outline_width: inst.get_f32("highlightOutlineWidth").unwrap_or_default(),
            highlight_outline_only: inst.get_bool("highlightOutlineOnly").unwrap_or_default(),
        }
    }
}
