// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalshopparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SAutoLoadingBoxSizePrices`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAutoLoadingBoxSizePrices {
    /// `one_eighthSCU` (Int32)
    #[serde(default)]
    pub one_eighth_scu: i32,
    /// `one_quarterSCU` (Int32)
    #[serde(default)]
    pub one_quarter_scu: i32,
    /// `one_halfSCU` (Int32)
    #[serde(default)]
    pub one_half_scu: i32,
    /// `oneSCU` (Int32)
    #[serde(default)]
    pub one_scu: i32,
    /// `twoSCU` (Int32)
    #[serde(default)]
    pub two_scu: i32,
    /// `fourSCU` (Int32)
    #[serde(default)]
    pub four_scu: i32,
    /// `eightSCU` (Int32)
    #[serde(default)]
    pub eight_scu: i32,
    /// `sixteenSCU` (Int32)
    #[serde(default)]
    pub sixteen_scu: i32,
    /// `twentyFourSCU` (Int32)
    #[serde(default)]
    pub twenty_four_scu: i32,
    /// `thirtyTwoSCU` (Int32)
    #[serde(default)]
    pub thirty_two_scu: i32,
}

impl Pooled for SAutoLoadingBoxSizePrices {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.sauto_loading_box_size_prices }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.sauto_loading_box_size_prices }
}

impl<'a> Extract<'a> for SAutoLoadingBoxSizePrices {
    const TYPE_NAME: &'static str = "SAutoLoadingBoxSizePrices";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            one_eighth_scu: inst.get_i32("one_eighthSCU").unwrap_or_default(),
            one_quarter_scu: inst.get_i32("one_quarterSCU").unwrap_or_default(),
            one_half_scu: inst.get_i32("one_halfSCU").unwrap_or_default(),
            one_scu: inst.get_i32("oneSCU").unwrap_or_default(),
            two_scu: inst.get_i32("twoSCU").unwrap_or_default(),
            four_scu: inst.get_i32("fourSCU").unwrap_or_default(),
            eight_scu: inst.get_i32("eightSCU").unwrap_or_default(),
            sixteen_scu: inst.get_i32("sixteenSCU").unwrap_or_default(),
            twenty_four_scu: inst.get_i32("twentyFourSCU").unwrap_or_default(),
            thirty_two_scu: inst.get_i32("thirtyTwoSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopCommodityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopCommodityParams {
    /// `MaxKioskCargoGridDisplaySize` (Int32)
    #[serde(default)]
    pub max_kiosk_cargo_grid_display_size: i32,
    /// `autoLoadingBasePrice` (Int32)
    #[serde(default)]
    pub auto_loading_base_price: i32,
    /// `autoLoadingBoxSizePrices` (Class)
    #[serde(default)]
    pub auto_loading_box_size_prices: Option<Handle<SAutoLoadingBoxSizePrices>>,
    /// `noSupplyLevel` (Single)
    #[serde(default)]
    pub no_supply_level: f32,
    /// `VeryLowSupplyLevel` (Single)
    #[serde(default)]
    pub very_low_supply_level: f32,
    /// `LowSupplyLevel` (Single)
    #[serde(default)]
    pub low_supply_level: f32,
    /// `MediumSupplyLevel` (Single)
    #[serde(default)]
    pub medium_supply_level: f32,
    /// `HighSupplyLevel` (Single)
    #[serde(default)]
    pub high_supply_level: f32,
    /// `VeryHighSupplyLevel` (Single)
    #[serde(default)]
    pub very_high_supply_level: f32,
    /// `noDemandLevel` (Single)
    #[serde(default)]
    pub no_demand_level: f32,
    /// `VeryLowDemandLevel` (Single)
    #[serde(default)]
    pub very_low_demand_level: f32,
    /// `LowDemandLevel` (Single)
    #[serde(default)]
    pub low_demand_level: f32,
    /// `MediumDemandLevel` (Single)
    #[serde(default)]
    pub medium_demand_level: f32,
    /// `HighDemandLevel` (Single)
    #[serde(default)]
    pub high_demand_level: f32,
    /// `VeryHighDemandLevel` (Single)
    #[serde(default)]
    pub very_high_demand_level: f32,
    /// `transactionSupportedResourceContainerTypes` (Class (array))
    #[serde(default)]
    pub transaction_supported_resource_container_types: Vec<Handle<SItemPortDefTypes>>,
    /// `RMC_ResourceType` (Reference)
    #[serde(default)]
    pub rmc_resource_type: Option<CigGuid>,
    /// `RMC_SalvageCannisterEntity` (Reference)
    #[serde(default)]
    pub rmc_salvage_cannister_entity: Option<CigGuid>,
    /// `genericCrates` (Class)
    #[serde(default)]
    pub generic_crates: Option<Handle<SResourceTypeDefaultCargoContainers>>,
    /// `Location_Select` (Locale)
    #[serde(default)]
    pub location_select: LocaleKey,
    /// `subLocation_All` (Locale)
    #[serde(default)]
    pub sub_location_all: LocaleKey,
    /// `subLocation_CargoGrid` (Locale)
    #[serde(default)]
    pub sub_location_cargo_grid: LocaleKey,
    /// `subLocation_GeneralStorage` (Locale)
    #[serde(default)]
    pub sub_location_general_storage: LocaleKey,
    /// `subLocation_ResourceContainers` (Locale)
    #[serde(default)]
    pub sub_location_resource_containers: LocaleKey,
    /// `subLocationItems_All` (Locale)
    #[serde(default)]
    pub sub_location_items_all: LocaleKey,
}

impl Pooled for GlobalShopCommodityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.global_shop_commodity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.global_shop_commodity_params }
}

impl<'a> Extract<'a> for GlobalShopCommodityParams {
    const TYPE_NAME: &'static str = "GlobalShopCommodityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_kiosk_cargo_grid_display_size: inst.get_i32("MaxKioskCargoGridDisplaySize").unwrap_or_default(),
            auto_loading_base_price: inst.get_i32("autoLoadingBasePrice").unwrap_or_default(),
            auto_loading_box_size_prices: match inst.get("autoLoadingBoxSizePrices") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAutoLoadingBoxSizePrices>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            no_supply_level: inst.get_f32("noSupplyLevel").unwrap_or_default(),
            very_low_supply_level: inst.get_f32("VeryLowSupplyLevel").unwrap_or_default(),
            low_supply_level: inst.get_f32("LowSupplyLevel").unwrap_or_default(),
            medium_supply_level: inst.get_f32("MediumSupplyLevel").unwrap_or_default(),
            high_supply_level: inst.get_f32("HighSupplyLevel").unwrap_or_default(),
            very_high_supply_level: inst.get_f32("VeryHighSupplyLevel").unwrap_or_default(),
            no_demand_level: inst.get_f32("noDemandLevel").unwrap_or_default(),
            very_low_demand_level: inst.get_f32("VeryLowDemandLevel").unwrap_or_default(),
            low_demand_level: inst.get_f32("LowDemandLevel").unwrap_or_default(),
            medium_demand_level: inst.get_f32("MediumDemandLevel").unwrap_or_default(),
            high_demand_level: inst.get_f32("HighDemandLevel").unwrap_or_default(),
            very_high_demand_level: inst.get_f32("VeryHighDemandLevel").unwrap_or_default(),
            transaction_supported_resource_container_types: inst.get_array("transactionSupportedResourceContainerTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rmc_resource_type: inst.get("RMC_ResourceType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rmc_salvage_cannister_entity: inst.get("RMC_SalvageCannisterEntity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            generic_crates: match inst.get("genericCrates") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SResourceTypeDefaultCargoContainers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            location_select: inst.get_str("Location_Select").map(LocaleKey::from).unwrap_or_default(),
            sub_location_all: inst.get_str("subLocation_All").map(LocaleKey::from).unwrap_or_default(),
            sub_location_cargo_grid: inst.get_str("subLocation_CargoGrid").map(LocaleKey::from).unwrap_or_default(),
            sub_location_general_storage: inst.get_str("subLocation_GeneralStorage").map(LocaleKey::from).unwrap_or_default(),
            sub_location_resource_containers: inst.get_str("subLocation_ResourceContainers").map(LocaleKey::from).unwrap_or_default(),
            sub_location_items_all: inst.get_str("subLocationItems_All").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopTerminalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopTerminalParams {
    /// `displayedItemsPerPage` (Int32)
    #[serde(default)]
    pub displayed_items_per_page: i32,
    /// `confirmationAutoCloseTime` (Single)
    #[serde(default)]
    pub confirmation_auto_close_time: f32,
    /// `maxBuyingMultiplier` (Int32)
    #[serde(default)]
    pub max_buying_multiplier: i32,
    /// `inventoryQueryItemsType` (EnumChoice (array))
    #[serde(default)]
    pub inventory_query_items_type: Vec<EItemType>,
    /// `all_items_category` (Locale)
    #[serde(default)]
    pub all_items_category: LocaleKey,
    /// `shopErrors` (Class)
    #[serde(default)]
    pub shop_errors: Option<Handle<SGlobalShopErrors>>,
}

impl Pooled for GlobalShopTerminalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.global_shop_terminal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.global_shop_terminal_params }
}

impl<'a> Extract<'a> for GlobalShopTerminalParams {
    const TYPE_NAME: &'static str = "GlobalShopTerminalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            displayed_items_per_page: inst.get_i32("displayedItemsPerPage").unwrap_or_default(),
            confirmation_auto_close_time: inst.get_f32("confirmationAutoCloseTime").unwrap_or_default(),
            max_buying_multiplier: inst.get_i32("maxBuyingMultiplier").unwrap_or_default(),
            inventory_query_items_type: inst.get_array("inventoryQueryItemsType")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EItemType::from_dcb_str)).collect())
                .unwrap_or_default(),
            all_items_category: inst.get_str("all_items_category").map(LocaleKey::from).unwrap_or_default(),
            shop_errors: match inst.get("shopErrors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGlobalShopErrors>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGlobalShopErrors`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGlobalShopErrors {
    /// `confirmation_success` (Locale)
    #[serde(default)]
    pub confirmation_success: LocaleKey,
    /// `confirmation_fail` (Locale)
    #[serde(default)]
    pub confirmation_fail: LocaleKey,
    /// `confirmation_fail_AuthorityError` (Locale)
    #[serde(default)]
    pub confirmation_fail_authority_error: LocaleKey,
    /// `confirmation_fail_TransactionServiceError` (Locale)
    #[serde(default)]
    pub confirmation_fail_transaction_service_error: LocaleKey,
    /// `confirmation_fail_InvalidLocation` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_location: LocaleKey,
    /// `confirmation_fail_InvalidPlayerInventoryId` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_player_inventory_id: LocaleKey,
    /// `confirmation_fail_InventoryContainerRequestFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_container_request_fail: LocaleKey,
    /// `confirmation_fail_InventoryItemFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_item_fail: LocaleKey,
    /// `confirmation_fail_InventoryItemContentFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_inventory_item_content_fail: LocaleKey,
    /// `confirmation_fail_InvalidQuantityError` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_quantity_error: LocaleKey,
    /// `confirmation_fail_QuickBuyRestockingError` (Locale)
    #[serde(default)]
    pub confirmation_fail_quick_buy_restocking_error: LocaleKey,
    /// `confirmation_fail_InvalidTransactionFlow` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_transaction_flow: LocaleKey,
    /// `confirmation_fail_InvalidLocationSource` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_location_source: LocaleKey,
    /// `confirmation_fail_InvalidShop` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_shop: LocaleKey,
    /// `confirmation_fail_InvalidShopType` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_shop_type: LocaleKey,
    /// `confirmation_fail_InternalError` (Locale)
    #[serde(default)]
    pub confirmation_fail_internal_error: LocaleKey,
    /// `confirmation_fail_InvalidRentalOption` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_rental_option: LocaleKey,
    /// `confirmation_fail_ShipNotInValidLocation` (Locale)
    #[serde(default)]
    pub confirmation_fail_ship_not_in_valid_location: LocaleKey,
    /// `confirmation_fail_NoItemsInSaleError` (Locale)
    #[serde(default)]
    pub confirmation_fail_no_items_in_sale_error: LocaleKey,
    /// `confirmation_fail_WaitingForPendingResult` (Locale)
    #[serde(default)]
    pub confirmation_fail_waiting_for_pending_result: LocaleKey,
    /// `confirmation_fail_ActorDoesNotOwnSaleItem` (Locale)
    #[serde(default)]
    pub confirmation_fail_actor_does_not_own_sale_item: LocaleKey,
    /// `confirmation_fail_TransactionCostMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_transaction_cost_mismatch: LocaleKey,
    /// `confirmation_fail_ItemMaxStockError` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_max_stock_error: LocaleKey,
    /// `confirmation_fail_ItemNotSellable` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_not_sellable: LocaleKey,
    /// `confirmation_fail_ItemNotBuyable` (Locale)
    #[serde(default)]
    pub confirmation_fail_item_not_buyable: LocaleKey,
    /// `confirmation_fail_TimedOut` (Locale)
    #[serde(default)]
    pub confirmation_fail_timed_out: LocaleKey,
    /// `confirmation_fail_InsuffientStock` (Locale)
    #[serde(default)]
    pub confirmation_fail_insuffient_stock: LocaleKey,
    /// `confirmation_fail_ServiceError` (Locale)
    #[serde(default)]
    pub confirmation_fail_service_error: LocaleKey,
    /// `confirmation_fail_DatabaseError` (Locale)
    #[serde(default)]
    pub confirmation_fail_database_error: LocaleKey,
    /// `confirmation_fail_InvalidBuyer` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_buyer: LocaleKey,
    /// `confirmation_fail_InvalidItem` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_item: LocaleKey,
    /// `confirmation_fail_InvalidRequest` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_request: LocaleKey,
    /// `confirmation_fail_InsufficentFunds` (Locale)
    #[serde(default)]
    pub confirmation_fail_insufficent_funds: LocaleKey,
    /// `confirmation_fail_InvalidEntityClassGUID` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_entity_class_guid: LocaleKey,
    /// `confirmation_fail_InvalidKioskId` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_kiosk_id: LocaleKey,
    /// `confirmation_fail_InvalidSellPrice` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_sell_price: LocaleKey,
    /// `confirmation_fail_InvalidMineableEntry` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_mineable_entry: LocaleKey,
    /// `confirmation_fail_PlayerIdMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_player_id_mismatch: LocaleKey,
    /// `confirmation_fail_CargoCreationFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_cargo_creation_failed: LocaleKey,
    /// `confirmation_fail_WalletNotFound` (Locale)
    #[serde(default)]
    pub confirmation_fail_wallet_not_found: LocaleKey,
    /// `confirmation_fail_MissingResourceDataType` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_resource_data_type: LocaleKey,
    /// `confirmation_fail_PlayerInVehicleDuringCargoTransaction` (Locale)
    #[serde(default)]
    pub confirmation_fail_player_in_vehicle_during_cargo_transaction: LocaleKey,
    /// `confirmation_fail_InvalidParentState` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_parent_state: LocaleKey,
    /// `confirmation_fail_InvalidResourceTypeGUID` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_resource_type_guid: LocaleKey,
    /// `confirmation_fail_CargoRemovalFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_cargo_removal_failed: LocaleKey,
    /// `confirmation_fail_WalletUpdateFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_wallet_update_failed: LocaleKey,
    /// `confirmation_fail_ResourceContainerQueryFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_resource_container_query_failed: LocaleKey,
    /// `confirmation_fail_PricePerUnitMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_price_per_unit_mismatch: LocaleKey,
    /// `confirmation_fail_InvalidContainer` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_container: LocaleKey,
    /// `confirmation_fail_EntityQueryFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_entity_query_failed: LocaleKey,
    /// `confirmation_fail_MissingSnapshot` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_snapshot: LocaleKey,
    /// `confirmation_fail_MissingSnapshotData` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_snapshot_data: LocaleKey,
    /// `confirmation_fail_SnapshotGetFail` (Locale)
    #[serde(default)]
    pub confirmation_fail_snapshot_get_fail: LocaleKey,
    /// `confirmation_fail_ExceededBuyLimit` (Locale)
    #[serde(default)]
    pub confirmation_fail_exceeded_buy_limit: LocaleKey,
    /// `confirmation_fail_LicenseError` (Locale)
    #[serde(default)]
    pub confirmation_fail_license_error: LocaleKey,
    /// `confirmation_fail_InvalidPlayerState` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_player_state: LocaleKey,
    /// `confirmation_fail_InvalidBoxSize` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_box_size: LocaleKey,
    /// `confirmation_fail_InvalidBoxClass` (Locale)
    #[serde(default)]
    pub confirmation_fail_invalid_box_class: LocaleKey,
    /// `confirmation_fail_MissingAutoLoadRate` (Locale)
    #[serde(default)]
    pub confirmation_fail_missing_auto_load_rate: LocaleKey,
    /// `confirmation_fail_AutoLoadPriceMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_price_mismatch: LocaleKey,
    /// `confirmation_fail_AutoLoadTimeMismatch` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_time_mismatch: LocaleKey,
    /// `confirmation_fail_AutoLoadStartFailed` (Locale)
    #[serde(default)]
    pub confirmation_fail_auto_load_start_failed: LocaleKey,
}

impl Pooled for SGlobalShopErrors {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.sglobal_shop_errors }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.sglobal_shop_errors }
}

impl<'a> Extract<'a> for SGlobalShopErrors {
    const TYPE_NAME: &'static str = "SGlobalShopErrors";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            confirmation_success: inst.get_str("confirmation_success").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail: inst.get_str("confirmation_fail").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_authority_error: inst.get_str("confirmation_fail_AuthorityError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_transaction_service_error: inst.get_str("confirmation_fail_TransactionServiceError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_location: inst.get_str("confirmation_fail_InvalidLocation").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_player_inventory_id: inst.get_str("confirmation_fail_InvalidPlayerInventoryId").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_inventory_container_request_fail: inst.get_str("confirmation_fail_InventoryContainerRequestFail").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_inventory_item_fail: inst.get_str("confirmation_fail_InventoryItemFail").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_inventory_item_content_fail: inst.get_str("confirmation_fail_InventoryItemContentFail").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_quantity_error: inst.get_str("confirmation_fail_InvalidQuantityError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_quick_buy_restocking_error: inst.get_str("confirmation_fail_QuickBuyRestockingError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_transaction_flow: inst.get_str("confirmation_fail_InvalidTransactionFlow").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_location_source: inst.get_str("confirmation_fail_InvalidLocationSource").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_shop: inst.get_str("confirmation_fail_InvalidShop").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_shop_type: inst.get_str("confirmation_fail_InvalidShopType").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_internal_error: inst.get_str("confirmation_fail_InternalError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_rental_option: inst.get_str("confirmation_fail_InvalidRentalOption").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_ship_not_in_valid_location: inst.get_str("confirmation_fail_ShipNotInValidLocation").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_no_items_in_sale_error: inst.get_str("confirmation_fail_NoItemsInSaleError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_waiting_for_pending_result: inst.get_str("confirmation_fail_WaitingForPendingResult").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_actor_does_not_own_sale_item: inst.get_str("confirmation_fail_ActorDoesNotOwnSaleItem").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_transaction_cost_mismatch: inst.get_str("confirmation_fail_TransactionCostMismatch").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_item_max_stock_error: inst.get_str("confirmation_fail_ItemMaxStockError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_item_not_sellable: inst.get_str("confirmation_fail_ItemNotSellable").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_item_not_buyable: inst.get_str("confirmation_fail_ItemNotBuyable").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_timed_out: inst.get_str("confirmation_fail_TimedOut").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_insuffient_stock: inst.get_str("confirmation_fail_InsuffientStock").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_service_error: inst.get_str("confirmation_fail_ServiceError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_database_error: inst.get_str("confirmation_fail_DatabaseError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_buyer: inst.get_str("confirmation_fail_InvalidBuyer").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_item: inst.get_str("confirmation_fail_InvalidItem").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_request: inst.get_str("confirmation_fail_InvalidRequest").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_insufficent_funds: inst.get_str("confirmation_fail_InsufficentFunds").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_entity_class_guid: inst.get_str("confirmation_fail_InvalidEntityClassGUID").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_kiosk_id: inst.get_str("confirmation_fail_InvalidKioskId").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_sell_price: inst.get_str("confirmation_fail_InvalidSellPrice").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_mineable_entry: inst.get_str("confirmation_fail_InvalidMineableEntry").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_player_id_mismatch: inst.get_str("confirmation_fail_PlayerIdMismatch").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_cargo_creation_failed: inst.get_str("confirmation_fail_CargoCreationFailed").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_wallet_not_found: inst.get_str("confirmation_fail_WalletNotFound").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_missing_resource_data_type: inst.get_str("confirmation_fail_MissingResourceDataType").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_player_in_vehicle_during_cargo_transaction: inst.get_str("confirmation_fail_PlayerInVehicleDuringCargoTransaction").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_parent_state: inst.get_str("confirmation_fail_InvalidParentState").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_resource_type_guid: inst.get_str("confirmation_fail_InvalidResourceTypeGUID").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_cargo_removal_failed: inst.get_str("confirmation_fail_CargoRemovalFailed").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_wallet_update_failed: inst.get_str("confirmation_fail_WalletUpdateFailed").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_resource_container_query_failed: inst.get_str("confirmation_fail_ResourceContainerQueryFailed").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_price_per_unit_mismatch: inst.get_str("confirmation_fail_PricePerUnitMismatch").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_container: inst.get_str("confirmation_fail_InvalidContainer").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_entity_query_failed: inst.get_str("confirmation_fail_EntityQueryFailed").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_missing_snapshot: inst.get_str("confirmation_fail_MissingSnapshot").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_missing_snapshot_data: inst.get_str("confirmation_fail_MissingSnapshotData").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_snapshot_get_fail: inst.get_str("confirmation_fail_SnapshotGetFail").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_exceeded_buy_limit: inst.get_str("confirmation_fail_ExceededBuyLimit").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_license_error: inst.get_str("confirmation_fail_LicenseError").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_player_state: inst.get_str("confirmation_fail_InvalidPlayerState").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_box_size: inst.get_str("confirmation_fail_InvalidBoxSize").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_invalid_box_class: inst.get_str("confirmation_fail_InvalidBoxClass").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_missing_auto_load_rate: inst.get_str("confirmation_fail_MissingAutoLoadRate").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_auto_load_price_mismatch: inst.get_str("confirmation_fail_AutoLoadPriceMismatch").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_auto_load_time_mismatch: inst.get_str("confirmation_fail_AutoLoadTimeMismatch").map(LocaleKey::from).unwrap_or_default(),
            confirmation_fail_auto_load_start_failed: inst.get_str("confirmation_fail_AutoLoadStartFailed").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeModifier {
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: EItemType,
    /// `SubType` (EnumChoice)
    #[serde(default)]
    pub sub_type: EItemSubType,
    /// `matchPercentage` (Single)
    #[serde(default)]
    pub match_percentage: f32,
}

impl Pooled for ItemTypeModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.item_type_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.item_type_modifier }
}

impl<'a> Extract<'a> for ItemTypeModifier {
    const TYPE_NAME: &'static str = "ItemTypeModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("Type").unwrap_or("")),
            sub_type: EItemSubType::from_dcb_str(inst.get_str("SubType").unwrap_or("")),
            match_percentage: inst.get_f32("matchPercentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopSellingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopSellingParams {
    /// `matchPercentage` (Single)
    #[serde(default)]
    pub match_percentage: f32,
    /// `noMatchPercentage` (Single)
    #[serde(default)]
    pub no_match_percentage: f32,
    /// `missionItemSellPriceReductionPercentage` (Single)
    #[serde(default)]
    pub mission_item_sell_price_reduction_percentage: f32,
    /// `maxInventoryCurve` (Single (array))
    #[serde(default)]
    pub max_inventory_curve: Vec<f32>,
    /// `wearCurve` (Single (array))
    #[serde(default)]
    pub wear_curve: Vec<f32>,
    /// `itemTypeModifiers` (Class (array))
    #[serde(default)]
    pub item_type_modifiers: Vec<Handle<ItemTypeModifier>>,
}

impl Pooled for GlobalShopSellingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.global_shop_selling_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.global_shop_selling_params }
}

impl<'a> Extract<'a> for GlobalShopSellingParams {
    const TYPE_NAME: &'static str = "GlobalShopSellingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_percentage: inst.get_f32("matchPercentage").unwrap_or_default(),
            no_match_percentage: inst.get_f32("noMatchPercentage").unwrap_or_default(),
            mission_item_sell_price_reduction_percentage: inst.get_f32("missionItemSellPriceReductionPercentage").unwrap_or_default(),
            max_inventory_curve: inst.get_array("maxInventoryCurve")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
            wear_curve: inst.get_array("wearCurve")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
            item_type_modifiers: inst.get_array("itemTypeModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ItemTypeModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LicensedItemModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensedItemModifier {
    /// `LicensedItem` (Reference)
    #[serde(default)]
    pub licensed_item: Option<CigGuid>,
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: ELicenseType,
    /// `PercentageModifier` (Single)
    #[serde(default)]
    pub percentage_modifier: f32,
    /// `DisableDuplicateBadgeCheck` (Boolean)
    #[serde(default)]
    pub disable_duplicate_badge_check: bool,
}

impl Pooled for LicensedItemModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.licensed_item_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.licensed_item_modifier }
}

impl<'a> Extract<'a> for LicensedItemModifier {
    const TYPE_NAME: &'static str = "LicensedItemModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            licensed_item: inst.get("LicensedItem").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            r#type: ELicenseType::from_dcb_str(inst.get_str("Type").unwrap_or("")),
            percentage_modifier: inst.get_f32("PercentageModifier").unwrap_or_default(),
            disable_duplicate_badge_check: inst.get_bool("DisableDuplicateBadgeCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalShopBuyingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShopBuyingParams {
    /// `tutorialItemBuyLimitPerHour` (UInt32)
    #[serde(default)]
    pub tutorial_item_buy_limit_per_hour: u32,
    /// `licensedItemModifiers` (Class (array))
    #[serde(default)]
    pub licensed_item_modifiers: Vec<Handle<LicensedItemModifier>>,
}

impl Pooled for GlobalShopBuyingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalshopparams.global_shop_buying_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalshopparams.global_shop_buying_params }
}

impl<'a> Extract<'a> for GlobalShopBuyingParams {
    const TYPE_NAME: &'static str = "GlobalShopBuyingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tutorial_item_buy_limit_per_hour: inst.get_u32("tutorialItemBuyLimitPerHour").unwrap_or_default(),
            licensed_item_modifiers: inst.get_array("licensedItemModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LicensedItemModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LicensedItemModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

