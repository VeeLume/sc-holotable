// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `refinerynotificationconfiguration`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RefineryNotificationConfiguration`
pub struct RefineryNotificationConfiguration {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `textMultiple` (Locale)
    pub text_multiple: LocaleKey,
    /// `duration` (Single)
    pub duration: f32,
    /// `refineryServiceError` (Locale)
    pub refinery_service_error: LocaleKey,
    /// `refineryDeliveryFailed` (Locale)
    pub refinery_delivery_failed: LocaleKey,
    /// `refineryJobCreationFailed` (Locale)
    pub refinery_job_creation_failed: LocaleKey,
    /// `refinerySetupError` (Locale)
    pub refinery_setup_error: LocaleKey,
}

impl Pooled for RefineryNotificationConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.refinerynotificationconfiguration.refinery_notification_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.refinerynotificationconfiguration.refinery_notification_configuration }
}

impl<'a> Extract<'a> for RefineryNotificationConfiguration {
    const TYPE_NAME: &'static str = "RefineryNotificationConfiguration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            text_multiple: inst.get_str("textMultiple").map(LocaleKey::from).unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            refinery_service_error: inst.get_str("refineryServiceError").map(LocaleKey::from).unwrap_or_default(),
            refinery_delivery_failed: inst.get_str("refineryDeliveryFailed").map(LocaleKey::from).unwrap_or_default(),
            refinery_job_creation_failed: inst.get_str("refineryJobCreationFailed").map(LocaleKey::from).unwrap_or_default(),
            refinery_setup_error: inst.get_str("refinerySetupError").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

