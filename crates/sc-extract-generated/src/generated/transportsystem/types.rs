// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `transportsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `TransportEventItemSpawnerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportEventItemSpawnerParams {
    /// `splineEvent` (String)
    pub spline_event: String,
    /// `spawnEntityClass` (String)
    pub spawn_entity_class: String,
}

impl Pooled for TransportEventItemSpawnerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_event_item_spawner_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_event_item_spawner_params
    }
}

impl<'a> Extract<'a> for TransportEventItemSpawnerParams {
    const TYPE_NAME: &'static str = "TransportEventItemSpawnerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            spline_event: inst
                .get_str("splineEvent")
                .map(String::from)
                .unwrap_or_default(),
            spawn_entity_class: inst
                .get_str("spawnEntityClass")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportDestinationCategory`
pub struct TransportDestinationCategory {
    /// `categoryName` (String)
    pub category_name: String,
    /// `locName` (Locale)
    pub loc_name: LocaleKey,
    /// `layout` (EnumChoice)
    pub layout: TransportDestinationCategoryLayout,
    /// `icon` (Reference)
    pub icon: Option<CigGuid>,
    /// `priority` (Int32)
    pub priority: i32,
}

impl Pooled for TransportDestinationCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_destination_category
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_destination_category
    }
}

impl<'a> Extract<'a> for TransportDestinationCategory {
    const TYPE_NAME: &'static str = "TransportDestinationCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            category_name: inst
                .get_str("categoryName")
                .map(String::from)
                .unwrap_or_default(),
            loc_name: inst
                .get_str("locName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            layout: TransportDestinationCategoryLayout::from_dcb_str(
                inst.get_str("layout").unwrap_or(""),
            ),
            icon: inst
                .get("icon")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            priority: inst.get_i32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportDestinationCategories`
pub struct TransportDestinationCategories {
    /// `categories` (Reference (array))
    pub categories: Vec<CigGuid>,
}

impl Pooled for TransportDestinationCategories {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_destination_categories
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_destination_categories
    }
}

impl<'a> Extract<'a> for TransportDestinationCategories {
    const TYPE_NAME: &'static str = "TransportDestinationCategories";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst
                .get_array("categories")
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

/// DCB type: `TransportIconType`
pub struct TransportIconType {
    /// `iconTypeName` (String)
    pub icon_type_name: String,
    /// `value` (Int32)
    pub value: i32,
}

impl Pooled for TransportIconType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_icon_type
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_icon_type
    }
}

impl<'a> Extract<'a> for TransportIconType {
    const TYPE_NAME: &'static str = "TransportIconType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon_type_name: inst
                .get_str("iconTypeName")
                .map(String::from)
                .unwrap_or_default(),
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportIconTypes`
pub struct TransportIconTypes {
    /// `icons` (Reference (array))
    pub icons: Vec<CigGuid>,
}

impl Pooled for TransportIconTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_icon_types
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_icon_types
    }
}

impl<'a> Extract<'a> for TransportIconTypes {
    const TYPE_NAME: &'static str = "TransportIconTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icons: inst
                .get_array("icons")
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

/// DCB type: `TransportCarriageGenericAnnouncement`
pub struct TransportCarriageGenericAnnouncement {
    /// `postDepartureAnnouncement` (Class)
    pub post_departure_announcement: Option<Handle<GlobalResourceAudio>>,
    /// `preArrivalAnnouncement` (Class)
    pub pre_arrival_announcement: Option<Handle<GlobalResourceAudio>>,
    /// `preDepartureAnnouncement` (Class)
    pub pre_departure_announcement: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for TransportCarriageGenericAnnouncement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .transportsystem
            .transport_carriage_generic_announcement
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .transportsystem
            .transport_carriage_generic_announcement
    }
}

impl<'a> Extract<'a> for TransportCarriageGenericAnnouncement {
    const TYPE_NAME: &'static str = "TransportCarriageGenericAnnouncement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            post_departure_announcement: match inst.get("postDepartureAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pre_arrival_announcement: match inst.get("preArrivalAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pre_departure_announcement: match inst.get("preDepartureAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `TransportCarriageDestinationAnnouncement`
/// Inherits from: `TransportCarriageGenericAnnouncement`
pub struct TransportCarriageDestinationAnnouncement {
    /// `postDepartureAnnouncement` (Class)
    pub post_departure_announcement: Option<Handle<GlobalResourceAudio>>,
    /// `preArrivalAnnouncement` (Class)
    pub pre_arrival_announcement: Option<Handle<GlobalResourceAudio>>,
    /// `preDepartureAnnouncement` (Class)
    pub pre_departure_announcement: Option<Handle<GlobalResourceAudio>>,
    /// `destinationIdentifierTagFilter` (Class)
    pub destination_identifier_tag_filter: Option<Handle<TagsDNFTerm>>,
}

impl Pooled for TransportCarriageDestinationAnnouncement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .transportsystem
            .transport_carriage_destination_announcement
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .transportsystem
            .transport_carriage_destination_announcement
    }
}

impl<'a> Extract<'a> for TransportCarriageDestinationAnnouncement {
    const TYPE_NAME: &'static str = "TransportCarriageDestinationAnnouncement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            post_departure_announcement: match inst.get("postDepartureAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pre_arrival_announcement: match inst.get("preArrivalAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pre_departure_announcement: match inst.get("preDepartureAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            destination_identifier_tag_filter: match inst.get("destinationIdentifierTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `TransportCarriageAnnouncements`
pub struct TransportCarriageAnnouncements {
    /// `preArrivalAlertTime` (Single)
    pub pre_arrival_alert_time: f32,
    /// `preDepartureAlertTime` (Single)
    pub pre_departure_alert_time: f32,
    /// `postDepartureAlertTime` (Single)
    pub post_departure_alert_time: f32,
    /// `genericAnnouncement` (Class)
    pub generic_announcement: Option<Handle<TransportCarriageGenericAnnouncement>>,
    /// `destinationAnnouncements` (Class (array))
    pub destination_announcements: Vec<Handle<TransportCarriageDestinationAnnouncement>>,
}

impl Pooled for TransportCarriageAnnouncements {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_carriage_announcements
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_carriage_announcements
    }
}

impl<'a> Extract<'a> for TransportCarriageAnnouncements {
    const TYPE_NAME: &'static str = "TransportCarriageAnnouncements";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pre_arrival_alert_time: inst.get_f32("preArrivalAlertTime").unwrap_or_default(),
            pre_departure_alert_time: inst.get_f32("preDepartureAlertTime").unwrap_or_default(),
            post_departure_alert_time: inst.get_f32("postDepartureAlertTime").unwrap_or_default(),
            generic_announcement: match inst.get("genericAnnouncement") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<TransportCarriageGenericAnnouncement>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            destination_announcements: inst
                .get_array("destinationAnnouncements")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<TransportCarriageDestinationAnnouncement>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<TransportCarriageDestinationAnnouncement>(
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

/// DCB type: `TransportManagerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportManagerParams {
    /// `automated` (Boolean)
    pub automated: bool,
    /// `carriageClass` (Reference)
    pub carriage_class: Option<CigGuid>,
    /// `permissionsSource` (StrongPointer)
    pub permissions_source: Option<TransportPermissionsInterfacePtr>,
    /// `carriageAnnouncements` (Reference)
    pub carriage_announcements: Option<CigGuid>,
    /// `gatewayAnnouncements` (Reference)
    pub gateway_announcements: Option<CigGuid>,
    /// `manufacturer` (Reference)
    pub manufacturer: Option<CigGuid>,
    /// `ordering` (EnumChoice)
    pub ordering: TransportDestinationOrderingMethod,
    /// `tagFilter` (Class)
    pub tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `maxSpeed` (Single)
    pub max_speed: f32,
    /// `acceleration` (Single)
    pub acceleration: f32,
    /// `maxTeleportDuration` (Single)
    pub max_teleport_duration: f32,
    /// `invalidTeleportDuration` (Single)
    pub invalid_teleport_duration: f32,
    /// `automatedCarriageInterval` (Single)
    pub automated_carriage_interval: f32,
    /// `doorCloseDelay` (Single)
    pub door_close_delay: f32,
    /// `departureDelay` (Single)
    pub departure_delay: f32,
    /// `holdDoorsDelay` (Single)
    pub hold_doors_delay: f32,
    /// `reservationTime` (Single)
    pub reservation_time: f32,
}

impl Pooled for TransportManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_manager_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_manager_params
    }
}

impl<'a> Extract<'a> for TransportManagerParams {
    const TYPE_NAME: &'static str = "TransportManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            automated: inst.get_bool("automated").unwrap_or_default(),
            carriage_class: inst
                .get("carriageClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            permissions_source: match inst.get("permissionsSource") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(TransportPermissionsInterfacePtr::from_ref(b, r))
                }
                _ => None,
            },
            carriage_announcements: inst
                .get("carriageAnnouncements")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            gateway_announcements: inst
                .get("gatewayAnnouncements")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            manufacturer: inst
                .get("manufacturer")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            ordering: TransportDestinationOrderingMethod::from_dcb_str(
                inst.get_str("ordering").unwrap_or(""),
            ),
            tag_filter: match inst.get("tagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            acceleration: inst.get_f32("acceleration").unwrap_or_default(),
            max_teleport_duration: inst.get_f32("maxTeleportDuration").unwrap_or_default(),
            invalid_teleport_duration: inst.get_f32("invalidTeleportDuration").unwrap_or_default(),
            automated_carriage_interval: inst
                .get_f32("automatedCarriageInterval")
                .unwrap_or_default(),
            door_close_delay: inst.get_f32("doorCloseDelay").unwrap_or_default(),
            departure_delay: inst.get_f32("departureDelay").unwrap_or_default(),
            hold_doors_delay: inst.get_f32("holdDoorsDelay").unwrap_or_default(),
            reservation_time: inst.get_f32("reservationTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportDestinationParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportDestinationParams {
    /// `name` (Locale)
    pub name: LocaleKey,
    /// `autoNumberedName` (Boolean)
    pub auto_numbered_name: bool,
    /// `category` (Reference)
    pub category: Option<CigGuid>,
    /// `iconOverride` (Reference)
    pub icon_override: Option<CigGuid>,
    /// `iconString` (Locale)
    pub icon_string: LocaleKey,
    /// `priority` (Int32)
    pub priority: i32,
    /// `useDoorTags` (Boolean)
    pub use_door_tags: bool,
    /// `isDefaultDestination` (Boolean)
    pub is_default_destination: bool,
    /// `tagFilter` (Class)
    pub tag_filter: Option<Handle<TagsDNFTerm>>,
}

impl Pooled for TransportDestinationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_destination_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_destination_params
    }
}

impl<'a> Extract<'a> for TransportDestinationParams {
    const TYPE_NAME: &'static str = "TransportDestinationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst
                .get_str("name")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            auto_numbered_name: inst.get_bool("autoNumberedName").unwrap_or_default(),
            category: inst
                .get("category")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            icon_override: inst
                .get("iconOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            icon_string: inst
                .get_str("iconString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            use_door_tags: inst.get_bool("useDoorTags").unwrap_or_default(),
            is_default_destination: inst.get_bool("isDefaultDestination").unwrap_or_default(),
            tag_filter: match inst.get("tagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `TransportGatewayParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportGatewayParams {
    /// `tagFilter` (Class)
    pub tag_filter: Option<Handle<TagsDNFTerm>>,
}

impl Pooled for TransportGatewayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_gateway_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_gateway_params
    }
}

impl<'a> Extract<'a> for TransportGatewayParams {
    const TYPE_NAME: &'static str = "TransportGatewayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag_filter: match inst.get("tagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `TransportNavSplineParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportNavSplineParams {
    /// `startTags` (Class)
    pub start_tags: Option<Handle<TagList>>,
    /// `startTagFilter` (Class)
    pub start_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `endTags` (Class)
    pub end_tags: Option<Handle<TagList>>,
    /// `endTagFilter` (Class)
    pub end_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `reversible` (Boolean)
    pub reversible: bool,
    /// `invertForward` (Boolean)
    pub invert_forward: bool,
    /// `invertForwardOnStart` (Boolean)
    pub invert_forward_on_start: bool,
}

impl Pooled for TransportNavSplineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_nav_spline_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_nav_spline_params
    }
}

impl<'a> Extract<'a> for TransportNavSplineParams {
    const TYPE_NAME: &'static str = "TransportNavSplineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_tags: match inst.get("startTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            start_tag_filter: match inst.get("startTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tags: match inst.get("endTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tag_filter: match inst.get("endTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            reversible: inst.get_bool("reversible").unwrap_or_default(),
            invert_forward: inst.get_bool("invertForward").unwrap_or_default(),
            invert_forward_on_start: inst.get_bool("invertForwardOnStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportPausePointParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportPausePointParams {
    /// `startTags` (Class)
    pub start_tags: Option<Handle<TagList>>,
    /// `startTagFilter` (Class)
    pub start_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `endTags` (Class)
    pub end_tags: Option<Handle<TagList>>,
    /// `endTagFilter` (Class)
    pub end_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `invertForwardOnReverse` (Boolean)
    pub invert_forward_on_reverse: bool,
    /// `pauseDuration` (Single)
    pub pause_duration: f32,
    /// `trigger` (String)
    pub trigger: String,
}

impl Pooled for TransportPausePointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_pause_point_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_pause_point_params
    }
}

impl<'a> Extract<'a> for TransportPausePointParams {
    const TYPE_NAME: &'static str = "TransportPausePointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_tags: match inst.get("startTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            start_tag_filter: match inst.get("startTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tags: match inst.get("endTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tag_filter: match inst.get("endTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            invert_forward_on_reverse: inst.get_bool("invertForwardOnReverse").unwrap_or_default(),
            pause_duration: inst.get_f32("pauseDuration").unwrap_or_default(),
            trigger: inst
                .get_str("trigger")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportGatewayTimerPanelParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportGatewayTimerPanelParams {}

impl Pooled for TransportGatewayTimerPanelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_gateway_timer_panel_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_gateway_timer_panel_params
    }
}

impl<'a> Extract<'a> for TransportGatewayTimerPanelParams {
    const TYPE_NAME: &'static str = "TransportGatewayTimerPanelParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `TransportGatewayControlPanelParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportGatewayControlPanelParams {}

impl Pooled for TransportGatewayControlPanelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_gateway_control_panel_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_gateway_control_panel_params
    }
}

impl<'a> Extract<'a> for TransportGatewayControlPanelParams {
    const TYPE_NAME: &'static str = "TransportGatewayControlPanelParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `TransportCarriageControlPanelParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportCarriageControlPanelParams {}

impl Pooled for TransportCarriageControlPanelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .transportsystem
            .transport_carriage_control_panel_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .transportsystem
            .transport_carriage_control_panel_params
    }
}

impl<'a> Extract<'a> for TransportCarriageControlPanelParams {
    const TYPE_NAME: &'static str = "TransportCarriageControlPanelParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `TransportCarriageTimerPanelParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportCarriageTimerPanelParams {}

impl Pooled for TransportCarriageTimerPanelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_carriage_timer_panel_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_carriage_timer_panel_params
    }
}

impl<'a> Extract<'a> for TransportCarriageTimerPanelParams {
    const TYPE_NAME: &'static str = "TransportCarriageTimerPanelParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `TransportCarriageAudio`
pub struct TransportCarriageAudio {
    /// `startTriggerID` (Class)
    pub start_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `startTriggerIDOneShot` (Class)
    pub start_trigger_idone_shot: Option<Handle<GlobalResourceAudio>>,
    /// `stoppingTriggerID` (Class)
    pub stopping_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `stopTriggerID` (Class)
    pub stop_trigger_id: Option<Handle<GlobalResourceAudio>>,
    /// `stopTriggerIDOneShot` (Class)
    pub stop_trigger_idone_shot: Option<Handle<GlobalResourceAudio>>,
    /// `speedRTPC` (Class)
    pub speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `turnRTPC` (Class)
    pub turn_rtpc: Option<Handle<AudioRtpc>>,
    /// `turnRTPCScaler` (Single)
    pub turn_rtpcscaler: f32,
}

impl Pooled for TransportCarriageAudio {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_carriage_audio
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_carriage_audio
    }
}

impl<'a> Extract<'a> for TransportCarriageAudio {
    const TYPE_NAME: &'static str = "TransportCarriageAudio";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_trigger_id: match inst.get("startTriggerID") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            start_trigger_idone_shot: match inst.get("startTriggerIDOneShot") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            stopping_trigger_id: match inst.get("stoppingTriggerID") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            stop_trigger_id: match inst.get("stopTriggerID") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            stop_trigger_idone_shot: match inst.get("stopTriggerIDOneShot") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            speed_rtpc: match inst.get("speedRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            turn_rtpc: match inst.get("turnRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            turn_rtpcscaler: inst.get_f32("turnRTPCScaler").unwrap_or_default(),
        }
    }
}

/// DCB type: `TransportCarriageEffects`
pub struct TransportCarriageEffects {
    /// `inTransitTag` (Reference)
    pub in_transit_tag: Option<CigGuid>,
}

impl Pooled for TransportCarriageEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_carriage_effects
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_carriage_effects
    }
}

impl<'a> Extract<'a> for TransportCarriageEffects {
    const TYPE_NAME: &'static str = "TransportCarriageEffects";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            in_transit_tag: inst
                .get("inTransitTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `TransportCarriageParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportCarriageParams {
    /// `responsive` (Boolean)
    pub responsive: bool,
    /// `reverseDirectionWithSpline` (Boolean)
    pub reverse_direction_with_spline: bool,
    /// `alignmentPositionalOffset` (Class)
    pub alignment_positional_offset: Option<Handle<Vec3>>,
    /// `alignmentRotationalOffset` (Class)
    pub alignment_rotational_offset: Option<Handle<Vec3>>,
    /// `audio` (Class)
    pub audio: Option<Handle<TransportCarriageAudio>>,
    /// `effects` (Class)
    pub effects: Option<Handle<TransportCarriageEffects>>,
}

impl Pooled for TransportCarriageParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_carriage_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_carriage_params
    }
}

impl<'a> Extract<'a> for TransportCarriageParams {
    const TYPE_NAME: &'static str = "TransportCarriageParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            responsive: inst.get_bool("responsive").unwrap_or_default(),
            reverse_direction_with_spline: inst
                .get_bool("reverseDirectionWithSpline")
                .unwrap_or_default(),
            alignment_positional_offset: match inst.get("alignmentPositionalOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            alignment_rotational_offset: match inst.get("alignmentRotationalOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<TransportCarriageAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            effects: match inst.get("effects") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<TransportCarriageEffects>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
