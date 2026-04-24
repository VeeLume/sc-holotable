// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalcargoloadingparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SAutoLoadingBoxSizeLoadingTime`
pub struct SAutoLoadingBoxSizeLoadingTime {
    /// `one_eighthSCU` (Single)
    pub one_eighth_scu: f32,
    /// `one_quarterSCU` (Single)
    pub one_quarter_scu: f32,
    /// `one_halfSCU` (Single)
    pub one_half_scu: f32,
    /// `oneSCU` (Single)
    pub one_scu: f32,
    /// `twoSCU` (Single)
    pub two_scu: f32,
    /// `fourSCU` (Single)
    pub four_scu: f32,
    /// `eightSCU` (Single)
    pub eight_scu: f32,
    /// `sixteenSCU` (Single)
    pub sixteen_scu: f32,
    /// `twentyFourSCU` (Single)
    pub twenty_four_scu: f32,
    /// `thirtyTwoSCU` (Single)
    pub thirty_two_scu: f32,
}

impl Pooled for SAutoLoadingBoxSizeLoadingTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .globalcargoloadingparams
            .sauto_loading_box_size_loading_time
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .globalcargoloadingparams
            .sauto_loading_box_size_loading_time
    }
}

impl<'a> Extract<'a> for SAutoLoadingBoxSizeLoadingTime {
    const TYPE_NAME: &'static str = "SAutoLoadingBoxSizeLoadingTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            one_eighth_scu: inst.get_f32("one_eighthSCU").unwrap_or_default(),
            one_quarter_scu: inst.get_f32("one_quarterSCU").unwrap_or_default(),
            one_half_scu: inst.get_f32("one_halfSCU").unwrap_or_default(),
            one_scu: inst.get_f32("oneSCU").unwrap_or_default(),
            two_scu: inst.get_f32("twoSCU").unwrap_or_default(),
            four_scu: inst.get_f32("fourSCU").unwrap_or_default(),
            eight_scu: inst.get_f32("eightSCU").unwrap_or_default(),
            sixteen_scu: inst.get_f32("sixteenSCU").unwrap_or_default(),
            twenty_four_scu: inst.get_f32("twentyFourSCU").unwrap_or_default(),
            thirty_two_scu: inst.get_f32("thirtyTwoSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalCargoLoadingParams`
pub struct GlobalCargoLoadingParams {
    /// `vehicleVelocityEpsilon` (Single)
    pub vehicle_velocity_epsilon: f32,
    /// `vehicleMovementTimeLimit` (Single)
    pub vehicle_movement_time_limit: f32,
    /// `uiDisplayTimeLimit` (Single)
    pub ui_display_time_limit: f32,
    /// `revokeTimeDelay` (Single)
    pub revoke_time_delay: f32,
    /// `uiTimeRemainingForTimeOutWarning` (Single)
    pub ui_time_remaining_for_time_out_warning: f32,
    /// `forfeitTimeBuffer` (Single)
    pub forfeit_time_buffer: f32,
    /// `initialMovementHintTimeBuffer` (Single)
    pub initial_movement_hint_time_buffer: f32,
    /// `cargoDeckLoadingTimePerSCU` (Single)
    pub cargo_deck_loading_time_per_scu: f32,
    /// `autoLoadingBaseLoadingTime` (Single)
    pub auto_loading_base_loading_time: f32,
    /// `autoLoadingBaseUnloadingTime` (Single)
    pub auto_loading_base_unloading_time: f32,
    /// `autoLoadingBoxSizeLoadingTime` (Class)
    pub auto_loading_box_size_loading_time: Option<Handle<SAutoLoadingBoxSizeLoadingTime>>,
    /// `autoLoadingBoxSizeUnloadingTime` (Class)
    pub auto_loading_box_size_unloading_time: Option<Handle<SAutoLoadingBoxSizeLoadingTime>>,
    /// `warningCargoRemovalNotification` (Class)
    pub warning_cargo_removal_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyCargoRemovalNotification` (Class)
    pub notify_cargo_removal_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// `reminderRetrieveCargoFromLoadingAreaNotification` (Class)
    pub reminder_retrieve_cargo_from_loading_area_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyCargoTransferredNotification` (Class)
    pub notify_cargo_transferred_notification: Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyCargoTransferInterruptedObstructionNotification` (Class)
    pub notify_cargo_transfer_interrupted_obstruction_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyCargoTransferInterruptedShipMovingNotification` (Class)
    pub notify_cargo_transfer_interrupted_ship_moving_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyCargoTransferInterruptedGenericNotification` (Class)
    pub notify_cargo_transfer_interrupted_generic_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `initialMovementHintTimeBufferNotification` (Class)
    pub initial_movement_hint_time_buffer_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyLoadingAreaRevokedTimeoutNotification` (Class)
    pub notify_loading_area_revoked_timeout_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
    /// `notifyLoadingAreaRevokedGenericNotification` (Class)
    pub notify_loading_area_revoked_generic_notification:
        Option<Handle<CargoLoadingNotificationParams>>,
}

impl Pooled for GlobalCargoLoadingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.globalcargoloadingparams.global_cargo_loading_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.globalcargoloadingparams.global_cargo_loading_params
    }
}

impl<'a> Extract<'a> for GlobalCargoLoadingParams {
    const TYPE_NAME: &'static str = "GlobalCargoLoadingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_velocity_epsilon: inst.get_f32("vehicleVelocityEpsilon").unwrap_or_default(),
            vehicle_movement_time_limit: inst
                .get_f32("vehicleMovementTimeLimit")
                .unwrap_or_default(),
            ui_display_time_limit: inst.get_f32("uiDisplayTimeLimit").unwrap_or_default(),
            revoke_time_delay: inst.get_f32("revokeTimeDelay").unwrap_or_default(),
            ui_time_remaining_for_time_out_warning: inst
                .get_f32("uiTimeRemainingForTimeOutWarning")
                .unwrap_or_default(),
            forfeit_time_buffer: inst.get_f32("forfeitTimeBuffer").unwrap_or_default(),
            initial_movement_hint_time_buffer: inst
                .get_f32("initialMovementHintTimeBuffer")
                .unwrap_or_default(),
            cargo_deck_loading_time_per_scu: inst
                .get_f32("cargoDeckLoadingTimePerSCU")
                .unwrap_or_default(),
            auto_loading_base_loading_time: inst
                .get_f32("autoLoadingBaseLoadingTime")
                .unwrap_or_default(),
            auto_loading_base_unloading_time: inst
                .get_f32("autoLoadingBaseUnloadingTime")
                .unwrap_or_default(),
            auto_loading_box_size_loading_time: match inst.get("autoLoadingBoxSizeLoadingTime") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            auto_loading_box_size_unloading_time: match inst.get("autoLoadingBoxSizeUnloadingTime")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SAutoLoadingBoxSizeLoadingTime>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            warning_cargo_removal_notification: match inst.get("warningCargoRemovalNotification") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_cargo_removal_notification: match inst.get("notifyCargoRemovalNotification") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            reminder_retrieve_cargo_from_loading_area_notification: match inst
                .get("reminderRetrieveCargoFromLoadingAreaNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_cargo_transferred_notification: match inst
                .get("notifyCargoTransferredNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_cargo_transfer_interrupted_obstruction_notification: match inst
                .get("notifyCargoTransferInterruptedObstructionNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_cargo_transfer_interrupted_ship_moving_notification: match inst
                .get("notifyCargoTransferInterruptedShipMovingNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_cargo_transfer_interrupted_generic_notification: match inst
                .get("notifyCargoTransferInterruptedGenericNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            initial_movement_hint_time_buffer_notification: match inst
                .get("initialMovementHintTimeBufferNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_loading_area_revoked_timeout_notification: match inst
                .get("notifyLoadingAreaRevokedTimeoutNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notify_loading_area_revoked_generic_notification: match inst
                .get("notifyLoadingAreaRevokedGenericNotification")
            {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CargoLoadingNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `CargoLoadingNotificationParams`
pub struct CargoLoadingNotificationParams {
    /// `message` (Locale)
    pub message: LocaleKey,
    /// `screenTimer` (Single)
    pub screen_timer: f32,
    /// `hurryScreenTimer` (Single)
    pub hurry_screen_timer: f32,
    /// `disabled` (Boolean)
    pub disabled: bool,
    /// `dockNotificationParamsOverride` (Reference)
    pub dock_notification_params_override: Option<CigGuid>,
}

impl Pooled for CargoLoadingNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .globalcargoloadingparams
            .cargo_loading_notification_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .globalcargoloadingparams
            .cargo_loading_notification_params
    }
}

impl<'a> Extract<'a> for CargoLoadingNotificationParams {
    const TYPE_NAME: &'static str = "CargoLoadingNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst
                .get_str("message")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            disabled: inst.get_bool("disabled").unwrap_or_default(),
            dock_notification_params_override: inst
                .get("dockNotificationParamsOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
