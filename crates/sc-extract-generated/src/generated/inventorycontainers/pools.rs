// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `inventorycontainers` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InventorycontainersPools {
    #[serde(default)]
    pub open_inventory_occupant_item_type_properties: Vec<Option<OpenInventoryOccupantItemTypeProperties>>,
    #[serde(default)]
    pub inventory_container_manager: Vec<Option<InventoryContainerManager>>,
    #[serde(default)]
    pub landing_zone_inventory_redirect: Vec<Option<LandingZoneInventoryRedirect>>,
    #[serde(default)]
    pub inventory_location: Vec<Option<InventoryLocation>>,
    #[serde(default)]
    pub landing_zone_inventory: Vec<Option<LandingZoneInventory>>,
    #[serde(default)]
    pub cargo_grid_occupant_face: Vec<Option<CargoGridOccupantFace>>,
    #[serde(default)]
    pub cargo_grid_occupant_properties: Vec<Option<CargoGridOccupantProperties>>,
}
