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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `RefineryNotificationConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefineryNotificationConfiguration {
    /// `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// `textMultiple` (Locale)
    #[serde(default)]
    pub text_multiple: String,
    /// `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// `refineryServiceError` (Locale)
    #[serde(default)]
    pub refinery_service_error: String,
    /// `refineryDeliveryFailed` (Locale)
    #[serde(default)]
    pub refinery_delivery_failed: String,
    /// `refineryJobCreationFailed` (Locale)
    #[serde(default)]
    pub refinery_job_creation_failed: String,
    /// `refinerySetupError` (Locale)
    #[serde(default)]
    pub refinery_setup_error: String,
}

impl Pooled for RefineryNotificationConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.refinerynotificationconfiguration.refinery_notification_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.refinerynotificationconfiguration.refinery_notification_configuration }
}

impl<'a> Extract<'a> for RefineryNotificationConfiguration {
    const TYPE_NAME: &'static str = "RefineryNotificationConfiguration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            text_multiple: inst.get_str("textMultiple").map(String::from).unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
            refinery_service_error: inst.get_str("refineryServiceError").map(String::from).unwrap_or_default(),
            refinery_delivery_failed: inst.get_str("refineryDeliveryFailed").map(String::from).unwrap_or_default(),
            refinery_job_creation_failed: inst.get_str("refineryJobCreationFailed").map(String::from).unwrap_or_default(),
            refinery_setup_error: inst.get_str("refinerySetupError").map(String::from).unwrap_or_default(),
        }
    }
}

