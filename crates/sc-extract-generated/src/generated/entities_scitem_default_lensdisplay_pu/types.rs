// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-default_lensdisplay_pu`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SChatChannelBlackList`
/// Inherits from: `SChatChannelFilterBase`
pub struct SChatChannelBlackList {
    /// `chatChannelType` (StrongPointer (array))
    pub chat_channel_type: Vec<SChatChannelTypeBasePtr>,
}

impl Pooled for SChatChannelBlackList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_default_lensdisplay_pu.schat_channel_black_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_default_lensdisplay_pu.schat_channel_black_list }
}

impl<'a> Extract<'a> for SChatChannelBlackList {
    const TYPE_NAME: &'static str = "SChatChannelBlackList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            chat_channel_type: inst.get_array("chatChannelType")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(SChatChannelTypeBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentRttAspectBoxoutParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentRttAspectBoxoutParams {
    /// `targetRttSlot` (EnumChoice)
    pub target_rtt_slot: ERuntimeImageSourceType,
    /// `aspectRatio` (Single)
    pub aspect_ratio: f32,
    /// `maximumScreenSizeRatio` (Single)
    pub maximum_screen_size_ratio: f32,
    /// `noShieldScaleAdjustment` (Single)
    pub no_shield_scale_adjustment: f32,
}

impl Pooled for EntityComponentRttAspectBoxoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_default_lensdisplay_pu.entity_component_rtt_aspect_boxout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_default_lensdisplay_pu.entity_component_rtt_aspect_boxout_params }
}

impl<'a> Extract<'a> for EntityComponentRttAspectBoxoutParams {
    const TYPE_NAME: &'static str = "EntityComponentRttAspectBoxoutParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            target_rtt_slot: ERuntimeImageSourceType::from_dcb_str(inst.get_str("targetRttSlot").unwrap_or("")),
            aspect_ratio: inst.get_f32("aspectRatio").unwrap_or_default(),
            maximum_screen_size_ratio: inst.get_f32("maximumScreenSizeRatio").unwrap_or_default(),
            no_shield_scale_adjustment: inst.get_f32("noShieldScaleAdjustment").unwrap_or_default(),
        }
    }
}

