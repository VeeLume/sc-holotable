// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `rentalnotificationparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `RentalNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalNotificationParams {
    /// `timeToRentalExpireMessage` (Locale)
    #[serde(default)]
    pub time_to_rental_expire_message: String,
    /// `rentedItemNameToken` (String)
    #[serde(default)]
    pub rented_item_name_token: String,
    /// `rentalDurationToken` (String)
    #[serde(default)]
    pub rental_duration_token: String,
}

impl Pooled for RentalNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.rentalnotificationparams.rental_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.rentalnotificationparams.rental_notification_params }
}

impl<'a> Extract<'a> for RentalNotificationParams {
    const TYPE_NAME: &'static str = "RentalNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            time_to_rental_expire_message: inst.get_str("timeToRentalExpireMessage").map(String::from).unwrap_or_default(),
            rented_item_name_token: inst.get_str("rentedItemNameToken").map(String::from).unwrap_or_default(),
            rental_duration_token: inst.get_str("rentalDurationToken").map(String::from).unwrap_or_default(),
        }
    }
}

