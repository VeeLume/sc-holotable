// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-usables`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ProceduralConnectionLinkController`
/// Inherits from: `NavigationLinkController`
pub struct ProceduralConnectionLinkController {
    /// `zOffsetForRaycastCheck` (Single)
    pub z_offset_for_raycast_check: f32,
}

impl Pooled for ProceduralConnectionLinkController {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .procedural_connection_link_controller
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .procedural_connection_link_controller
    }
}

impl<'a> Extract<'a> for ProceduralConnectionLinkController {
    const TYPE_NAME: &'static str = "ProceduralConnectionLinkController";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            z_offset_for_raycast_check: inst.get_f32("zOffsetForRaycastCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `ClosestOrientationHandholdAttachSpotChoiceParams`
/// Inherits from: `HandholdAttachPointChoiceParams`
pub struct ClosestOrientationHandholdAttachSpotChoiceParams {
    /// `desiredUpDirection` (Class)
    pub desired_up_direction: Option<Handle<Vec3>>,
    /// `space` (EnumChoice)
    pub space: EHandholdAttachOrientationSpace,
}

impl Pooled for ClosestOrientationHandholdAttachSpotChoiceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .closest_orientation_handhold_attach_spot_choice_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .closest_orientation_handhold_attach_spot_choice_params
    }
}

impl<'a> Extract<'a> for ClosestOrientationHandholdAttachSpotChoiceParams {
    const TYPE_NAME: &'static str = "ClosestOrientationHandholdAttachSpotChoiceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            desired_up_direction: match inst.get("desiredUpDirection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            space: EHandholdAttachOrientationSpace::from_dcb_str(
                inst.get_str("space").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `SpecificHandholdAttachSpotChoiceParams`
/// Inherits from: `HandholdAttachPointChoiceParams`
pub struct SpecificHandholdAttachSpotChoiceParams {
    /// `attachSpotName` (String)
    pub attach_spot_name: String,
    /// `fallbackParams` (StrongPointer)
    pub fallback_params: Option<Handle<ClosestOrientationHandholdAttachSpotChoiceParams>>,
}

impl Pooled for SpecificHandholdAttachSpotChoiceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .specific_handhold_attach_spot_choice_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .specific_handhold_attach_spot_choice_params
    }
}

impl<'a> Extract<'a> for SpecificHandholdAttachSpotChoiceParams {
    const TYPE_NAME: &'static str = "SpecificHandholdAttachSpotChoiceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attach_spot_name: inst
                .get_str("attachSpotName")
                .map(String::from)
                .unwrap_or_default(),
            fallback_params: match inst.get("fallbackParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(
                    b.alloc_nested::<ClosestOrientationHandholdAttachSpotChoiceParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ),
                ),
                _ => None,
            },
        }
    }
}

/// DCB type: `HandholdAttachmentTriggerParams`
pub struct HandholdAttachmentTriggerParams {
    /// `name` (String)
    pub name: String,
    /// `entityLinkName` (String)
    pub entity_link_name: String,
    /// `handholdName` (String)
    pub handhold_name: String,
    /// `attachPointChoiceParams` (StrongPointer)
    pub attach_point_choice_params: Option<HandholdAttachPointChoiceParamsPtr>,
}

impl Pooled for HandholdAttachmentTriggerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .handhold_attachment_trigger_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .handhold_attachment_trigger_params
    }
}

impl<'a> Extract<'a> for HandholdAttachmentTriggerParams {
    const TYPE_NAME: &'static str = "HandholdAttachmentTriggerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            entity_link_name: inst
                .get_str("entityLinkName")
                .map(String::from)
                .unwrap_or_default(),
            handhold_name: inst
                .get_str("handholdName")
                .map(String::from)
                .unwrap_or_default(),
            attach_point_choice_params: match inst.get("attachPointChoiceParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(HandholdAttachPointChoiceParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `HandholdSharedInteractionLink`
/// Inherits from: `HandholdInteractionLink`
pub struct HandholdSharedInteractionLink {
    /// `ignoreInteractionOnFail` (Boolean)
    pub ignore_interaction_on_fail: bool,
    /// `attachmentTrigger` (WeakPointer)
    pub attachment_trigger: Option<Handle<HandholdAttachmentTriggerParams>>,
    /// `sourceSharedInteraction` (WeakPointer)
    pub source_shared_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for HandholdSharedInteractionLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .handhold_shared_interaction_link
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .handhold_shared_interaction_link
    }
}

impl<'a> Extract<'a> for HandholdSharedInteractionLink {
    const TYPE_NAME: &'static str = "HandholdSharedInteractionLink";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ignore_interaction_on_fail: inst
                .get_bool("ignoreInteractionOnFail")
                .unwrap_or_default(),
            attachment_trigger: match inst.get("attachmentTrigger") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<HandholdAttachmentTriggerParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            source_shared_interaction: match inst.get("sourceSharedInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `HandholdInteractionPointLink`
/// Inherits from: `HandholdInteractionLink`
pub struct HandholdInteractionPointLink {
    /// `ignoreInteractionOnFail` (Boolean)
    pub ignore_interaction_on_fail: bool,
    /// `attachmentTrigger` (WeakPointer)
    pub attachment_trigger: Option<Handle<HandholdAttachmentTriggerParams>>,
    /// `sourceInteractionPoint` (WeakPointer)
    pub source_interaction_point: Option<Handle<SInteractionPointParams>>,
}

impl Pooled for HandholdInteractionPointLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .handhold_interaction_point_link
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .handhold_interaction_point_link
    }
}

impl<'a> Extract<'a> for HandholdInteractionPointLink {
    const TYPE_NAME: &'static str = "HandholdInteractionPointLink";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ignore_interaction_on_fail: inst
                .get_bool("ignoreInteractionOnFail")
                .unwrap_or_default(),
            attachment_trigger: match inst.get("attachmentTrigger") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<HandholdAttachmentTriggerParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            source_interaction_point: match inst.get("sourceInteractionPoint") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionPointParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `HandholdLinkComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct HandholdLinkComponentParams {
    /// `attachmentTriggers` (Class (array))
    pub attachment_triggers: Vec<Handle<HandholdAttachmentTriggerParams>>,
    /// `sharedInteractionLinks` (Class (array))
    pub shared_interaction_links: Vec<Handle<HandholdSharedInteractionLink>>,
    /// `interactionPointLinks` (Class (array))
    pub interaction_point_links: Vec<Handle<HandholdInteractionPointLink>>,
}

impl Pooled for HandholdLinkComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.handhold_link_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.handhold_link_component_params
    }
}

impl<'a> Extract<'a> for HandholdLinkComponentParams {
    const TYPE_NAME: &'static str = "HandholdLinkComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attachment_triggers: inst
                .get_array("attachmentTriggers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HandholdAttachmentTriggerParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<HandholdAttachmentTriggerParams>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            shared_interaction_links: inst
                .get_array("sharedInteractionLinks")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HandholdSharedInteractionLink>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<HandholdSharedInteractionLink>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            interaction_point_links: inst
                .get_array("interactionPointLinks")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<HandholdInteractionPointLink>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<HandholdInteractionPointLink>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SPlayerUsableSearchRouteUsable`
pub struct SPlayerUsableSearchRouteUsable {
    /// `searchUsableTags` (Class)
    pub search_usable_tags: Option<Handle<TagList>>,
    /// `searchUsableItemProviderTags` (Class)
    pub search_usable_item_provider_tags: Option<Handle<TagList>>,
    /// `routingSettings` (Class)
    pub routing_settings: Option<Handle<SUsableRoutingSettings>>,
}

impl Pooled for SPlayerUsableSearchRouteUsable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .splayer_usable_search_route_usable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .splayer_usable_search_route_usable
    }
}

impl<'a> Extract<'a> for SPlayerUsableSearchRouteUsable {
    const TYPE_NAME: &'static str = "SPlayerUsableSearchRouteUsable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            search_usable_tags: match inst.get("searchUsableTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            search_usable_item_provider_tags: match inst.get("searchUsableItemProviderTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            routing_settings: match inst.get("routingSettings") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SUsableRoutingSettings>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SSequencerPlayerUsableTaskParams`
/// Inherits from: `SSequencerDefTaskParams`
pub struct SSequencerPlayerUsableTaskParams {
    /// `name` (String)
    pub name: String,
}

impl Pooled for SSequencerPlayerUsableTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_player_usable_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_player_usable_task_params
    }
}

impl<'a> Extract<'a> for SSequencerPlayerUsableTaskParams {
    const TYPE_NAME: &'static str = "SSequencerPlayerUsableTaskParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSequencerPlayerUsableSwitchChannelTaskParams`
/// Inherits from: `SSequencerPlayerUsableTaskParams`
pub struct SSequencerPlayerUsableSwitchChannelTaskParams {
    /// `name` (String)
    pub name: String,
    /// `useChannelName` (String)
    pub use_channel_name: String,
    /// `userOnChannel` (Reference)
    pub user_on_channel: Option<CigGuid>,
}

impl Pooled for SSequencerPlayerUsableSwitchChannelTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_player_usable_switch_channel_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_player_usable_switch_channel_task_params
    }
}

impl<'a> Extract<'a> for SSequencerPlayerUsableSwitchChannelTaskParams {
    const TYPE_NAME: &'static str = "SSequencerPlayerUsableSwitchChannelTaskParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            use_channel_name: inst
                .get_str("useChannelName")
                .map(String::from)
                .unwrap_or_default(),
            user_on_channel: inst
                .get("userOnChannel")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SSequencerPlayerUsableScoochTaskParams`
/// Inherits from: `SSequencerPlayerUsableTaskParams`
pub struct SSequencerPlayerUsableScoochTaskParams {
    /// `name` (String)
    pub name: String,
    /// `searchUsableRoute` (Class)
    pub search_usable_route: Option<Handle<SPlayerUsableSearchRouteUsable>>,
}

impl Pooled for SSequencerPlayerUsableScoochTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_player_usable_scooch_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_player_usable_scooch_task_params
    }
}

impl<'a> Extract<'a> for SSequencerPlayerUsableScoochTaskParams {
    const TYPE_NAME: &'static str = "SSequencerPlayerUsableScoochTaskParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            search_usable_route: match inst.get("searchUsableRoute") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SPlayerUsableSearchRouteUsable>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SSequencerPlayerUsableUseChannelTaskParams`
/// Inherits from: `SSequencerPlayerUsableTaskParams`
pub struct SSequencerPlayerUsableUseChannelTaskParams {
    /// `name` (String)
    pub name: String,
    /// `playerUsablePort` (WeakPointer)
    pub player_usable_port: Option<Handle<PlayerUsableItemPort>>,
}

impl Pooled for SSequencerPlayerUsableUseChannelTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_player_usable_use_channel_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_player_usable_use_channel_task_params
    }
}

impl<'a> Extract<'a> for SSequencerPlayerUsableUseChannelTaskParams {
    const TYPE_NAME: &'static str = "SSequencerPlayerUsableUseChannelTaskParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            player_usable_port: match inst.get("playerUsablePort") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<PlayerUsableItemPort>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SSequencerPlayerUsableReserveSlotTaskParams`
/// Inherits from: `SSequencerPlayerUsableTaskParams`
pub struct SSequencerPlayerUsableReserveSlotTaskParams {
    /// `name` (String)
    pub name: String,
    /// `searchUsableRoute` (Class)
    pub search_usable_route: Option<Handle<SPlayerUsableSearchRouteUsable>>,
    /// `reserveSlot` (Boolean)
    pub reserve_slot: bool,
}

impl Pooled for SSequencerPlayerUsableReserveSlotTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_player_usable_reserve_slot_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_player_usable_reserve_slot_task_params
    }
}

impl<'a> Extract<'a> for SSequencerPlayerUsableReserveSlotTaskParams {
    const TYPE_NAME: &'static str = "SSequencerPlayerUsableReserveSlotTaskParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            search_usable_route: match inst.get("searchUsableRoute") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SPlayerUsableSearchRouteUsable>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            reserve_slot: inst.get_bool("reserveSlot").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerUsableInteractionPoint`
pub struct PlayerUsableInteractionPoint {
    /// `interactionPoint` (WeakPointer)
    pub interaction_point: Option<Handle<SInteractionPointParams>>,
    /// `useInteraction` (WeakPointer)
    pub use_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `fragmentTag` (String)
    pub fragment_tag: String,
}

impl Pooled for PlayerUsableInteractionPoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .player_usable_interaction_point
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .player_usable_interaction_point
    }
}

impl<'a> Extract<'a> for PlayerUsableInteractionPoint {
    const TYPE_NAME: &'static str = "PlayerUsableInteractionPoint";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interaction_point: match inst.get("interactionPoint") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionPointParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            use_interaction: match inst.get("useInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            fragment_tag: inst
                .get_str("fragmentTag")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerUsableItemPort`
pub struct PlayerUsableItemPort {
    /// `itemPort` (WeakPointer)
    pub item_port: Option<Handle<SItemPortDef>>,
    /// `useInteraction` (WeakPointer)
    pub use_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for PlayerUsableItemPort {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.player_usable_item_port
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.player_usable_item_port
    }
}

impl<'a> Extract<'a> for PlayerUsableItemPort {
    const TYPE_NAME: &'static str = "PlayerUsableItemPort";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_port: match inst.get("itemPort") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SItemPortDef>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            use_interaction: match inst.get("useInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerUsableSlot`
pub struct PlayerUsableSlot {
    /// `useSlot` (WeakPointer)
    pub use_slot: Option<Handle<UsableUseSlot>>,
    /// `playerUsableInteractionPoints` (Class (array))
    pub player_usable_interaction_points: Vec<Handle<PlayerUsableInteractionPoint>>,
    /// `playerUsableItemPorts` (Class (array))
    pub player_usable_item_ports: Vec<Handle<PlayerUsableItemPort>>,
}

impl Pooled for PlayerUsableSlot {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.player_usable_slot
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.player_usable_slot
    }
}

impl<'a> Extract<'a> for PlayerUsableSlot {
    const TYPE_NAME: &'static str = "PlayerUsableSlot";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_slot: match inst.get("useSlot") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<UsableUseSlot>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            player_usable_interaction_points: inst
                .get_array("playerUsableInteractionPoints")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerUsableInteractionPoint>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerUsableInteractionPoint>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            player_usable_item_ports: inst
                .get_array("playerUsableItemPorts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerUsableItemPort>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerUsableItemPort>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerUsableUseChannelInstance`
pub struct PlayerUsableUseChannelInstance {
    /// `useChannelInstance` (WeakPointer)
    pub use_channel_instance: Option<Handle<UsableUseChannelInstance>>,
    /// `delinkOnEnterComplete` (EnumChoice)
    pub delink_on_enter_complete: EDelinkMode,
    /// `availableActionGroups` (Class (array))
    pub available_action_groups: Vec<Handle<UsableChannelInputActionGroup>>,
}

impl Pooled for PlayerUsableUseChannelInstance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .player_usable_use_channel_instance
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .player_usable_use_channel_instance
    }
}

impl<'a> Extract<'a> for PlayerUsableUseChannelInstance {
    const TYPE_NAME: &'static str = "PlayerUsableUseChannelInstance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_channel_instance: match inst.get("useChannelInstance") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<UsableUseChannelInstance>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            delink_on_enter_complete: EDelinkMode::from_dcb_str(
                inst.get_str("delinkOnEnterComplete").unwrap_or(""),
            ),
            available_action_groups: inst
                .get_array("availableActionGroups")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<UsableChannelInputActionGroup>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<UsableChannelInputActionGroup>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerUsableView`
pub struct PlayerUsableView {
    /// `focusOnUser` (Boolean)
    pub focus_on_user: bool,
    /// `cameraView` (Reference)
    pub camera_view: Option<CigGuid>,
    /// `transitionParams` (StrongPointer)
    pub transition_params: Option<Handle<CameraTransitionParams>>,
}

impl Pooled for PlayerUsableView {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.player_usable_view
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.player_usable_view
    }
}

impl<'a> Extract<'a> for PlayerUsableView {
    const TYPE_NAME: &'static str = "PlayerUsableView";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            focus_on_user: inst.get_bool("focusOnUser").unwrap_or_default(),
            camera_view: inst
                .get("cameraView")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            transition_params: match inst.get("transitionParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<CameraTransitionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SPlayerUsableParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SPlayerUsableParams {
    /// `playerUsableSlots` (Class (array))
    pub player_usable_slots: Vec<Handle<PlayerUsableSlot>>,
    /// `playerUseChannels` (Class (array))
    pub player_use_channels: Vec<Handle<PlayerUsableUseChannelInstance>>,
    /// `sequencerTasks` (StrongPointer (array))
    pub sequencer_tasks: Vec<SSequencerPlayerUsableTaskParamsPtr>,
    /// `specializedData` (Class)
    pub specialized_data: Option<Handle<SSpecializedData>>,
    /// `views` (Class (array))
    pub views: Vec<Handle<PlayerUsableView>>,
}

impl Pooled for SPlayerUsableParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.splayer_usable_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.splayer_usable_params
    }
}

impl<'a> Extract<'a> for SPlayerUsableParams {
    const TYPE_NAME: &'static str = "SPlayerUsableParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            player_usable_slots: inst
                .get_array("playerUsableSlots")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerUsableSlot>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerUsableSlot>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            player_use_channels: inst
                .get_array("playerUseChannels")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerUsableUseChannelInstance>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<PlayerUsableUseChannelInstance>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            sequencer_tasks: inst
                .get_array("sequencerTasks")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(SSequencerPlayerUsableTaskParamsPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            specialized_data: match inst.get("specializedData") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SSpecializedData>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            views: inst
                .get_array("views")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerUsableView>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerUsableView>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MedBedProviderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MedBedProviderParams {}

impl Pooled for MedBedProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.med_bed_provider_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.med_bed_provider_params
    }
}

impl<'a> Extract<'a> for MedBedProviderParams {
    const TYPE_NAME: &'static str = "MedBedProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `MedBedSurgeryNamesInjurySeverity`
pub struct MedBedSurgeryNamesInjurySeverity {
    /// `injuryName` (Locale)
    pub injury_name: LocaleKey,
    /// `majorInjuryName` (Locale)
    pub major_injury_name: LocaleKey,
    /// `deadlyInjuryName` (Locale)
    pub deadly_injury_name: LocaleKey,
}

impl Pooled for MedBedSurgeryNamesInjurySeverity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .med_bed_surgery_names_injury_severity
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .med_bed_surgery_names_injury_severity
    }
}

impl<'a> Extract<'a> for MedBedSurgeryNamesInjurySeverity {
    const TYPE_NAME: &'static str = "MedBedSurgeryNamesInjurySeverity";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            injury_name: inst
                .get_str("injuryName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            major_injury_name: inst
                .get_str("majorInjuryName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            deadly_injury_name: inst
                .get_str("deadlyInjuryName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MedBedSurgeryNames`
pub struct MedBedSurgeryNames {
    /// `headInjuries` (Class)
    pub head_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
    /// `torsoInjuries` (Class)
    pub torso_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
    /// `leftArmInjuries` (Class)
    pub left_arm_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
    /// `rightArmInjuries` (Class)
    pub right_arm_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
    /// `leftLegInjuries` (Class)
    pub left_leg_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
    /// `rightLegInjuries` (Class)
    pub right_leg_injuries: Option<Handle<MedBedSurgeryNamesInjurySeverity>>,
}

impl Pooled for MedBedSurgeryNames {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.med_bed_surgery_names
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.med_bed_surgery_names
    }
}

impl<'a> Extract<'a> for MedBedSurgeryNames {
    const TYPE_NAME: &'static str = "MedBedSurgeryNames";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            head_injuries: match inst.get("headInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            torso_injuries: match inst.get("torsoInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            left_arm_injuries: match inst.get("leftArmInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            right_arm_injuries: match inst.get("rightArmInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            left_leg_injuries: match inst.get("leftLegInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            right_leg_injuries: match inst.get("rightLegInjuries") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNamesInjurySeverity>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `MedBedRespawnRangeOverride`
pub struct MedBedRespawnRangeOverride {
    /// `respawnRange` (Int64)
    pub respawn_range: i64,
}

impl Pooled for MedBedRespawnRangeOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.med_bed_respawn_range_override
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.med_bed_respawn_range_override
    }
}

impl<'a> Extract<'a> for MedBedRespawnRangeOverride {
    const TYPE_NAME: &'static str = "MedBedRespawnRangeOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            respawn_range: inst.get_i64("respawnRange").unwrap_or_default(),
        }
    }
}

/// DCB type: `MedBedComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MedBedComponentParams {
    /// `healInteraction` (WeakPointer)
    pub heal_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `setRespawnInteraction` (WeakPointer)
    pub set_respawn_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `cancelRespawnInteraction` (WeakPointer)
    pub cancel_respawn_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `cancelAllRespawnsInteraction` (WeakPointer)
    pub cancel_all_respawns_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `respawnInteraction` (WeakPointer)
    pub respawn_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `respawnInteractionPoint` (WeakPointer)
    pub respawn_interaction_point: Option<Handle<SInteractionPointParams>>,
    /// `useChannelToHealActor` (WeakPointer)
    pub use_channel_to_heal_actor: Option<Handle<UsableUseChannelInstance>>,
    /// `surgerySequenceState` (WeakPointer)
    pub surgery_sequence_state: Option<Handle<SInteractionState>>,
    /// `respawnRangeOverride` (StrongPointer)
    pub respawn_range_override: Option<Handle<MedBedRespawnRangeOverride>>,
    /// `timeToHeal` (Single)
    pub time_to_heal: f32,
    /// `delayBeforeHeal` (Single)
    pub delay_before_heal: f32,
    /// `medBedTier` (EnumChoice)
    pub med_bed_tier: MedBedTier,
    /// `medicalItemTierConfig` (Reference)
    pub medical_item_tier_config: Option<CigGuid>,
    /// `canRespawnHere` (Boolean)
    pub can_respawn_here: bool,
    /// `invulnerableUser` (Boolean)
    pub invulnerable_user: bool,
    /// `invulnerableDuration` (Single)
    pub invulnerable_duration: f32,
    /// `surgeryNames` (Class)
    pub surgery_names: Option<Handle<MedBedSurgeryNames>>,
    /// `resourceRegenerationPerMinute` (StrongPointer)
    pub resource_regeneration_per_minute: Option<SBaseCargoUnitPtr>,
}

impl Pooled for MedBedComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.med_bed_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.med_bed_component_params
    }
}

impl<'a> Extract<'a> for MedBedComponentParams {
    const TYPE_NAME: &'static str = "MedBedComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            heal_interaction: match inst.get("healInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            set_respawn_interaction: match inst.get("setRespawnInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            cancel_respawn_interaction: match inst.get("cancelRespawnInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            cancel_all_respawns_interaction: match inst.get("cancelAllRespawnsInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            respawn_interaction: match inst.get("respawnInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            respawn_interaction_point: match inst.get("respawnInteractionPoint") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionPointParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            use_channel_to_heal_actor: match inst.get("useChannelToHealActor") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<UsableUseChannelInstance>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            surgery_sequence_state: match inst.get("surgerySequenceState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            respawn_range_override: match inst.get("respawnRangeOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MedBedRespawnRangeOverride>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            time_to_heal: inst.get_f32("timeToHeal").unwrap_or_default(),
            delay_before_heal: inst.get_f32("delayBeforeHeal").unwrap_or_default(),
            med_bed_tier: MedBedTier::from_dcb_str(inst.get_str("medBedTier").unwrap_or("")),
            medical_item_tier_config: inst
                .get("medicalItemTierConfig")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            can_respawn_here: inst.get_bool("canRespawnHere").unwrap_or_default(),
            invulnerable_user: inst.get_bool("invulnerableUser").unwrap_or_default(),
            invulnerable_duration: inst.get_f32("invulnerableDuration").unwrap_or_default(),
            surgery_names: match inst.get("surgeryNames") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MedBedSurgeryNames>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            resource_regeneration_per_minute: match inst.get("resourceRegenerationPerMinute") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SBaseCargoUnitPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `MedicalSkeletonUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MedicalSkeletonUIProviderParams {}

impl Pooled for MedicalSkeletonUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .medical_skeleton_uiprovider_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .medical_skeleton_uiprovider_params
    }
}

impl<'a> Extract<'a> for MedicalSkeletonUIProviderParams {
    const TYPE_NAME: &'static str = "MedicalSkeletonUIProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `LinkedCloneLocationMedicalTier`
/// Inherits from: `CloneLocationMedicalTier`
pub struct LinkedCloneLocationMedicalTier {}

impl Pooled for LinkedCloneLocationMedicalTier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .linked_clone_location_medical_tier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .linked_clone_location_medical_tier
    }
}

impl<'a> Extract<'a> for LinkedCloneLocationMedicalTier {
    const TYPE_NAME: &'static str = "LinkedCloneLocationMedicalTier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SetCloneLocationMedicalTier`
/// Inherits from: `CloneLocationMedicalTier`
pub struct SetCloneLocationMedicalTier {
    /// `locationMedicalTier` (EnumChoice)
    pub location_medical_tier: MedBedTier,
}

impl Pooled for SetCloneLocationMedicalTier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .set_clone_location_medical_tier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .set_clone_location_medical_tier
    }
}

impl<'a> Extract<'a> for SetCloneLocationMedicalTier {
    const TYPE_NAME: &'static str = "SetCloneLocationMedicalTier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_medical_tier: MedBedTier::from_dcb_str(
                inst.get_str("locationMedicalTier").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `CloneLocationUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CloneLocationUIProviderParams {
    /// `medicalTier` (StrongPointer)
    pub medical_tier: Option<CloneLocationMedicalTierPtr>,
}

impl Pooled for CloneLocationUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .clone_location_uiprovider_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .clone_location_uiprovider_params
    }
}

impl<'a> Extract<'a> for CloneLocationUIProviderParams {
    const TYPE_NAME: &'static str = "CloneLocationUIProviderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            medical_tier: match inst.get("medicalTier") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(CloneLocationMedicalTierPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SEntityComponentPersistentEntityEntitlementSpawnerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentPersistentEntityEntitlementSpawnerParams {}

impl Pooled for SEntityComponentPersistentEntityEntitlementSpawnerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .sentity_component_persistent_entity_entitlement_spawner_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .sentity_component_persistent_entity_entitlement_spawner_params
    }
}

impl<'a> Extract<'a> for SEntityComponentPersistentEntityEntitlementSpawnerParams {
    const TYPE_NAME: &'static str = "SEntityComponentPersistentEntityEntitlementSpawnerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SSequencerImmediateDespawnDespawnerTaskParams`
/// Inherits from: `SSequencerDespawnerTaskParams`
pub struct SSequencerImmediateDespawnDespawnerTaskParams {
    /// `name` (String)
    pub name: String,
    /// `immediateDespawnPorts` (WeakPointer (array))
    pub immediate_despawn_ports: Vec<Handle<SItemPortDef>>,
}

impl Pooled for SSequencerImmediateDespawnDespawnerTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_immediate_despawn_despawner_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_immediate_despawn_despawner_task_params
    }
}

impl<'a> Extract<'a> for SSequencerImmediateDespawnDespawnerTaskParams {
    const TYPE_NAME: &'static str = "SSequencerImmediateDespawnDespawnerTaskParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            immediate_despawn_ports: inst
                .get_array("immediateDespawnPorts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(b.alloc_nested::<SItemPortDef>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityEffectSystem_EnableSequencerTask`
/// Inherits from: `EntityEffectSystem_BaseSequencerTask`
pub struct EntityEffectSystem_EnableSequencerTask {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
}

impl Pooled for EntityEffectSystem_EnableSequencerTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .entity_effect_system_enable_sequencer_task
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .entity_effect_system_enable_sequencer_task
    }
}

impl<'a> Extract<'a> for EntityEffectSystem_EnableSequencerTask {
    const TYPE_NAME: &'static str = "EntityEffectSystem_EnableSequencerTask";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `EntityEffectSystem_DisableSequencerTask`
/// Inherits from: `EntityEffectSystem_BaseSequencerTask`
pub struct EntityEffectSystem_DisableSequencerTask {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
}

impl Pooled for EntityEffectSystem_DisableSequencerTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .entity_effect_system_disable_sequencer_task
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .entity_effect_system_disable_sequencer_task
    }
}

impl<'a> Extract<'a> for EntityEffectSystem_DisableSequencerTask {
    const TYPE_NAME: &'static str = "EntityEffectSystem_DisableSequencerTask";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `HospitalCheckinScreenComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct HospitalCheckinScreenComponentParams {}

impl Pooled for HospitalCheckinScreenComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .hospital_checkin_screen_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .hospital_checkin_screen_component_params
    }
}

impl<'a> Extract<'a> for HospitalCheckinScreenComponentParams {
    const TYPE_NAME: &'static str = "HospitalCheckinScreenComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `InteractionConditionEmptyUsableItemPort`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionEmptyUsableItemPort {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `itemPortTag` (String)
    pub item_port_tag: String,
}

impl Pooled for InteractionConditionEmptyUsableItemPort {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interaction_condition_empty_usable_item_port
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interaction_condition_empty_usable_item_port
    }
}

impl<'a> Extract<'a> for InteractionConditionEmptyUsableItemPort {
    const TYPE_NAME: &'static str = "InteractionConditionEmptyUsableItemPort";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            item_port_tag: inst
                .get_str("itemPortTag")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractionConditionUsableAlignmentSlotEmpty`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionUsableAlignmentSlotEmpty {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `alignmentSlot` (WeakPointer)
    pub alignment_slot: Option<AlignmentSlotBasePtr>,
}

impl Pooled for InteractionConditionUsableAlignmentSlotEmpty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interaction_condition_usable_alignment_slot_empty
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interaction_condition_usable_alignment_slot_empty
    }
}

impl<'a> Extract<'a> for InteractionConditionUsableAlignmentSlotEmpty {
    const TYPE_NAME: &'static str = "InteractionConditionUsableAlignmentSlotEmpty";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            alignment_slot: match inst.get("alignmentSlot") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(AlignmentSlotBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractionConditionAvailableSpaceInLinkedUsableItem`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionAvailableSpaceInLinkedUsableItem {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `usablePortTag` (String)
    pub usable_port_tag: String,
    /// `itemPortTag` (String)
    pub item_port_tag: String,
    /// `itemType` (EnumChoice)
    pub item_type: EItemType,
    /// `itemSubType` (EnumChoice)
    pub item_sub_type: EItemSubType,
}

impl Pooled for InteractionConditionAvailableSpaceInLinkedUsableItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interaction_condition_available_space_in_linked_usable_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interaction_condition_available_space_in_linked_usable_item
    }
}

impl<'a> Extract<'a> for InteractionConditionAvailableSpaceInLinkedUsableItem {
    const TYPE_NAME: &'static str = "InteractionConditionAvailableSpaceInLinkedUsableItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            usable_port_tag: inst
                .get_str("usablePortTag")
                .map(String::from)
                .unwrap_or_default(),
            item_port_tag: inst
                .get_str("itemPortTag")
                .map(String::from)
                .unwrap_or_default(),
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            item_sub_type: EItemSubType::from_dcb_str(inst.get_str("itemSubType").unwrap_or("")),
        }
    }
}

/// DCB type: `InteractionConditionLinkedUsableHasTag`
/// Inherits from: `InteractionConditionParams`
pub struct InteractionConditionLinkedUsableHasTag {
    /// `conditionDisplay` (StrongPointer)
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `usableTags` (Class)
    pub usable_tags: Option<Handle<TagsDNFTerm>>,
}

impl Pooled for InteractionConditionLinkedUsableHasTag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interaction_condition_linked_usable_has_tag
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interaction_condition_linked_usable_has_tag
    }
}

impl<'a> Extract<'a> for InteractionConditionLinkedUsableHasTag {
    const TYPE_NAME: &'static str = "InteractionConditionLinkedUsableHasTag";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ConditionDisplayParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            usable_tags: match inst.get("usableTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SResourceContainerStateModifier`
/// Inherits from: `SRangeStateModifier`
pub struct SResourceContainerStateModifier {
    /// `stateRanges` (Class (array))
    pub state_ranges: Vec<Handle<SRangeStateLevel>>,
    /// `containerResource` (Reference)
    pub container_resource: Option<CigGuid>,
    /// `useActualOccupancyValue` (Boolean)
    pub use_actual_occupancy_value: bool,
}

impl Pooled for SResourceContainerStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .sresource_container_state_modifier
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .sresource_container_state_modifier
    }
}

impl<'a> Extract<'a> for SResourceContainerStateModifier {
    const TYPE_NAME: &'static str = "SResourceContainerStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_ranges: inst
                .get_array("stateRanges")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SRangeStateLevel>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SRangeStateLevel>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            container_resource: inst
                .get("containerResource")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            use_actual_occupancy_value: inst
                .get_bool("useActualOccupancyValue")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SItemPortRule_TriggerSequenceDef`
/// Inherits from: `SItemPortRuleDef`
pub struct SItemPortRule_TriggerSequenceDef {
    /// `triggerSequence` (WeakPointer)
    pub trigger_sequence: Option<SSequencerDefSequenceParamsPtr>,
}

impl Pooled for SItemPortRule_TriggerSequenceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .sitem_port_rule_trigger_sequence_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .sitem_port_rule_trigger_sequence_def
    }
}

impl<'a> Extract<'a> for SItemPortRule_TriggerSequenceDef {
    const TYPE_NAME: &'static str = "SItemPortRule_TriggerSequenceDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_sequence: match inst.get("triggerSequence") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SSequencerDefSequenceParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `UsableChannelInputActionGroup`
pub struct UsableChannelInputActionGroup {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `hintDescription` (Locale)
    pub hint_description: LocaleKey,
    /// `activationMode` (EnumChoice)
    pub activation_mode: ActivationMode,
    /// `actions` (StrongPointer (array))
    pub actions: Vec<UsableChannelInputActionPtr>,
}

impl Pooled for UsableChannelInputActionGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .usable_channel_input_action_group
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .usable_channel_input_action_group
    }
}

impl<'a> Extract<'a> for UsableChannelInputActionGroup {
    const TYPE_NAME: &'static str = "UsableChannelInputActionGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            hint_description: inst
                .get_str("hintDescription")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            activation_mode: ActivationMode::from_dcb_str(
                inst.get_str("activationMode").unwrap_or(""),
            ),
            actions: inst
                .get_array("actions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(UsableChannelInputActionPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UsableChannelInputAction`
pub struct UsableChannelInputAction {
    /// `inputAction` (Class)
    pub input_action: Option<Handle<InputAction>>,
    /// `interactionOnAction` (WeakPointer)
    pub interaction_on_action: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for UsableChannelInputAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.usable_channel_input_action
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.usable_channel_input_action
    }
}

impl<'a> Extract<'a> for UsableChannelInputAction {
    const TYPE_NAME: &'static str = "UsableChannelInputAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_action: match inst.get("inputAction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            interaction_on_action: match inst.get("interactionOnAction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `UsableChannelInputAction_ControlInteractive`
/// Inherits from: `UsableChannelInputAction`
pub struct UsableChannelInputAction_ControlInteractive {
    /// `inputAction` (Class)
    pub input_action: Option<Handle<InputAction>>,
    /// `interactionOnAction` (WeakPointer)
    pub interaction_on_action: Option<Handle<SSharedInteractionParams>>,
    /// `controlVariable` (WeakPointer)
    pub control_variable: Option<ControlInteractiveVariablePtr>,
}

impl Pooled for UsableChannelInputAction_ControlInteractive {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .usable_channel_input_action_control_interactive
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .usable_channel_input_action_control_interactive
    }
}

impl<'a> Extract<'a> for UsableChannelInputAction_ControlInteractive {
    const TYPE_NAME: &'static str = "UsableChannelInputAction_ControlInteractive";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_action: match inst.get("inputAction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            interaction_on_action: match inst.get("interactionOnAction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            control_variable: match inst.get("controlVariable") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ControlInteractiveVariablePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractiveVariable_BackToDefaultIntParams`
/// Inherits from: `InteractiveVariable_BackToDefaultParams`
pub struct InteractiveVariable_BackToDefaultIntParams {
    /// `waitTime` (Single)
    pub wait_time: f32,
}

impl Pooled for InteractiveVariable_BackToDefaultIntParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interactive_variable_back_to_default_int_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interactive_variable_back_to_default_int_params
    }
}

impl<'a> Extract<'a> for InteractiveVariable_BackToDefaultIntParams {
    const TYPE_NAME: &'static str = "InteractiveVariable_BackToDefaultIntParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wait_time: inst.get_f32("waitTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `InteractiveVariable_BackToDefaultFloatParams`
/// Inherits from: `InteractiveVariable_BackToDefaultParams`
pub struct InteractiveVariable_BackToDefaultFloatParams {
    /// `waitTime` (Single)
    pub wait_time: f32,
}

impl Pooled for InteractiveVariable_BackToDefaultFloatParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .interactive_variable_back_to_default_float_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .interactive_variable_back_to_default_float_params
    }
}

impl<'a> Extract<'a> for InteractiveVariable_BackToDefaultFloatParams {
    const TYPE_NAME: &'static str = "InteractiveVariable_BackToDefaultFloatParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wait_time: inst.get_f32("waitTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `IntInteractiveVariable`
/// Inherits from: `InteractiveVariable`
pub struct IntInteractiveVariable {
    /// `name` (String)
    pub name: String,
    /// `defaultValue` (Int32)
    pub default_value: i32,
    /// `minValue` (Int32)
    pub min_value: i32,
    /// `maxValue` (Int32)
    pub max_value: i32,
    /// `loopType` (EnumChoice)
    pub loop_type: InteractiveVariableLoopType,
    /// `backToDefaultParams` (StrongPointer)
    pub back_to_default_params: Option<Handle<InteractiveVariable_BackToDefaultIntParams>>,
}

impl Pooled for IntInteractiveVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.int_interactive_variable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.int_interactive_variable
    }
}

impl<'a> Extract<'a> for IntInteractiveVariable {
    const TYPE_NAME: &'static str = "IntInteractiveVariable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            default_value: inst.get_i32("defaultValue").unwrap_or_default(),
            min_value: inst.get_i32("minValue").unwrap_or_default(),
            max_value: inst.get_i32("maxValue").unwrap_or_default(),
            loop_type: InteractiveVariableLoopType::from_dcb_str(
                inst.get_str("loopType").unwrap_or(""),
            ),
            back_to_default_params: match inst.get("backToDefaultParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(
                    b.alloc_nested::<InteractiveVariable_BackToDefaultIntParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ),
                ),
                _ => None,
            },
        }
    }
}

/// DCB type: `FloatInteractiveVariable`
/// Inherits from: `InteractiveVariable`
pub struct FloatInteractiveVariable {
    /// `name` (String)
    pub name: String,
    /// `defaultValue` (Single)
    pub default_value: f32,
    /// `minValue` (Single)
    pub min_value: f32,
    /// `maxValue` (Single)
    pub max_value: f32,
    /// `loopType` (EnumChoice)
    pub loop_type: InteractiveVariableLoopType,
    /// `backToDefaultParams` (StrongPointer)
    pub back_to_default_params: Option<InteractiveVariable_BackToDefaultFloatParamsPtr>,
}

impl Pooled for FloatInteractiveVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.float_interactive_variable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.float_interactive_variable
    }
}

impl<'a> Extract<'a> for FloatInteractiveVariable {
    const TYPE_NAME: &'static str = "FloatInteractiveVariable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            default_value: inst.get_f32("defaultValue").unwrap_or_default(),
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
            loop_type: InteractiveVariableLoopType::from_dcb_str(
                inst.get_str("loopType").unwrap_or(""),
            ),
            back_to_default_params: match inst.get("backToDefaultParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(
                    InteractiveVariable_BackToDefaultFloatParamsPtr::from_ref(b, r),
                ),
                _ => None,
            },
        }
    }
}

/// DCB type: `ControlInteractiveVariable`
pub struct ControlInteractiveVariable {
    /// `name` (String)
    pub name: String,
    /// `fragmentTag` (String)
    pub fragment_tag: String,
}

impl Pooled for ControlInteractiveVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.control_interactive_variable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.control_interactive_variable
    }
}

impl<'a> Extract<'a> for ControlInteractiveVariable {
    const TYPE_NAME: &'static str = "ControlInteractiveVariable";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            fragment_tag: inst
                .get_str("fragmentTag")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlIntInteractiveVariable`
/// Inherits from: `ControlInteractiveVariable`
pub struct ControlIntInteractiveVariable {
    /// `name` (String)
    pub name: String,
    /// `fragmentTag` (String)
    pub fragment_tag: String,
    /// `variable` (WeakPointer)
    pub variable: Option<Handle<IntInteractiveVariable>>,
    /// `amountToChange` (Int32)
    pub amount_to_change: i32,
}

impl Pooled for ControlIntInteractiveVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .control_int_interactive_variable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .control_int_interactive_variable
    }
}

impl<'a> Extract<'a> for ControlIntInteractiveVariable {
    const TYPE_NAME: &'static str = "ControlIntInteractiveVariable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            fragment_tag: inst
                .get_str("fragmentTag")
                .map(String::from)
                .unwrap_or_default(),
            variable: match inst.get("variable") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<IntInteractiveVariable>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            amount_to_change: inst.get_i32("amountToChange").unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlFloatInteractiveVariable`
/// Inherits from: `ControlInteractiveVariable`
pub struct ControlFloatInteractiveVariable {
    /// `name` (String)
    pub name: String,
    /// `fragmentTag` (String)
    pub fragment_tag: String,
    /// `variable` (WeakPointer)
    pub variable: Option<Handle<FloatInteractiveVariable>>,
    /// `useAnimationEffectiveSection` (Boolean)
    pub use_animation_effective_section: bool,
    /// `amountToChange` (Single)
    pub amount_to_change: f32,
    /// `animationCycle` (Single)
    pub animation_cycle: f32,
}

impl Pooled for ControlFloatInteractiveVariable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .control_float_interactive_variable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .control_float_interactive_variable
    }
}

impl<'a> Extract<'a> for ControlFloatInteractiveVariable {
    const TYPE_NAME: &'static str = "ControlFloatInteractiveVariable";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            fragment_tag: inst
                .get_str("fragmentTag")
                .map(String::from)
                .unwrap_or_default(),
            variable: match inst.get("variable") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<FloatInteractiveVariable>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            use_animation_effective_section: inst
                .get_bool("useAnimationEffectiveSection")
                .unwrap_or_default(),
            amount_to_change: inst.get_f32("amountToChange").unwrap_or_default(),
            animation_cycle: inst.get_f32("animationCycle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCLinkedInteractiveControllerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCLinkedInteractiveControllerParams {
    /// `ints` (Class (array))
    pub ints: Vec<Handle<IntInteractiveVariable>>,
    /// `controlIntVariables` (Class (array))
    pub control_int_variables: Vec<Handle<ControlIntInteractiveVariable>>,
    /// `floats` (Class (array))
    pub floats: Vec<Handle<FloatInteractiveVariable>>,
    /// `controlFloatVariables` (Class (array))
    pub control_float_variables: Vec<Handle<ControlFloatInteractiveVariable>>,
}

impl Pooled for SCLinkedInteractiveControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .sclinked_interactive_controller_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .sclinked_interactive_controller_params
    }
}

impl<'a> Extract<'a> for SCLinkedInteractiveControllerParams {
    const TYPE_NAME: &'static str = "SCLinkedInteractiveControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ints: inst
                .get_array("ints")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<IntInteractiveVariable>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<IntInteractiveVariable>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            control_int_variables: inst
                .get_array("controlIntVariables")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ControlIntInteractiveVariable>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<ControlIntInteractiveVariable>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            floats: inst
                .get_array("floats")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<FloatInteractiveVariable>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<FloatInteractiveVariable>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            control_float_variables: inst
                .get_array("controlFloatVariables")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ControlFloatInteractiveVariable>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<ControlFloatInteractiveVariable>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `UsableArchetypes`
pub struct UsableArchetypes {
    /// `archetypes` (Reference (array))
    pub archetypes: Vec<CigGuid>,
}

impl Pooled for UsableArchetypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.usable_archetypes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.usable_archetypes
    }
}

impl<'a> Extract<'a> for UsableArchetypes {
    const TYPE_NAME: &'static str = "UsableArchetypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            archetypes: inst
                .get_array("archetypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SphereAreaAlignmentSlotTypeParams`
/// Inherits from: `AreaAlignmentSlotTypeParams`
pub struct SphereAreaAlignmentSlotTypeParams {
    /// `radius` (Single)
    pub radius: f32,
}

impl Pooled for SphereAreaAlignmentSlotTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .sphere_area_alignment_slot_type_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .sphere_area_alignment_slot_type_params
    }
}

impl<'a> Extract<'a> for SphereAreaAlignmentSlotTypeParams {
    const TYPE_NAME: &'static str = "SphereAreaAlignmentSlotTypeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `SUsableRoutingSettings`
pub struct SUsableRoutingSettings {
    /// `useChannelName` (String)
    pub use_channel_name: String,
    /// `slotGameTags` (Class)
    pub slot_game_tags: Option<Handle<TagList>>,
    /// `selectionType` (EnumChoice)
    pub selection_type: EUsableEntrySelectionType,
    /// `skipEntry` (Boolean)
    pub skip_entry: bool,
    /// `verifyNavMesh` (Boolean)
    pub verify_nav_mesh: bool,
    /// `interactionTag` (Reference)
    pub interaction_tag: Option<CigGuid>,
}

impl Pooled for SUsableRoutingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.susable_routing_settings
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.susable_routing_settings
    }
}

impl<'a> Extract<'a> for SUsableRoutingSettings {
    const TYPE_NAME: &'static str = "SUsableRoutingSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_channel_name: inst
                .get_str("useChannelName")
                .map(String::from)
                .unwrap_or_default(),
            slot_game_tags: match inst.get("slotGameTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            selection_type: EUsableEntrySelectionType::from_dcb_str(
                inst.get_str("selectionType").unwrap_or(""),
            ),
            skip_entry: inst.get_bool("skipEntry").unwrap_or_default(),
            verify_nav_mesh: inst.get_bool("verifyNavMesh").unwrap_or_default(),
            interaction_tag: inst
                .get("interactionTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SSequencerUsableDelinkTask`
/// Inherits from: `SSequencerUsableTask`
pub struct SSequencerUsableDelinkTask {}

impl Pooled for SSequencerUsableDelinkTask {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.ssequencer_usable_delink_task
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.ssequencer_usable_delink_task
    }
}

impl<'a> Extract<'a> for SSequencerUsableDelinkTask {
    const TYPE_NAME: &'static str = "SSequencerUsableDelinkTask";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SSequencerUsableFillConsumableTaskParams`
/// Inherits from: `SSequencerUsableTask`
pub struct SSequencerUsableFillConsumableTaskParams {
    /// `itemPortTag` (String)
    pub item_port_tag: String,
    /// `useReservedContents` (Boolean)
    pub use_reserved_contents: bool,
    /// `contentType` (String)
    pub content_type: String,
    /// `duration` (Single)
    pub duration: f32,
    /// `amountToAdd` (Single)
    pub amount_to_add: f32,
}

impl Pooled for SSequencerUsableFillConsumableTaskParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_usables
            .ssequencer_usable_fill_consumable_task_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_usables
            .ssequencer_usable_fill_consumable_task_params
    }
}

impl<'a> Extract<'a> for SSequencerUsableFillConsumableTaskParams {
    const TYPE_NAME: &'static str = "SSequencerUsableFillConsumableTaskParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_port_tag: inst
                .get_str("itemPortTag")
                .map(String::from)
                .unwrap_or_default(),
            use_reserved_contents: inst.get_bool("useReservedContents").unwrap_or_default(),
            content_type: inst
                .get_str("contentType")
                .map(String::from)
                .unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            amount_to_add: inst.get_f32("amountToAdd").unwrap_or_default(),
        }
    }
}

/// DCB type: `CameraTransitionParams`
pub struct CameraTransitionParams {
    /// `transitionTime` (Single)
    pub transition_time: f32,
    /// `isCinematic` (Boolean)
    pub is_cinematic: bool,
    /// `nearFaceDistanceSq` (Single)
    pub near_face_distance_sq: f32,
    /// `interpolationToPoint` (Reference)
    pub interpolation_to_point: Option<CigGuid>,
}

impl Pooled for CameraTransitionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_usables.camera_transition_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_usables.camera_transition_params
    }
}

impl<'a> Extract<'a> for CameraTransitionParams {
    const TYPE_NAME: &'static str = "CameraTransitionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            transition_time: inst.get_f32("transitionTime").unwrap_or_default(),
            is_cinematic: inst.get_bool("isCinematic").unwrap_or_default(),
            near_face_distance_sq: inst.get_f32("nearFaceDistanceSq").unwrap_or_default(),
            interpolation_to_point: inst
                .get("interpolationToPoint")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
