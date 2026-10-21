//! What an item holds: its inventory container.
//!
//! An entity with `SCItemInventoryContainerComponentParams` names an
//! `InventoryContainer` record (`containerParams`); a vehicle names one in
//! `VehicleComponentParams.inventoryContainerParams`. The record's
//! `inventoryType` says what it is: an open container is a cargo grid
//! whose size is its `interiorDimensions` in 1.25 m cells, a closed one
//! (lockers, personal storage, a ship's own inventory) states its
//! `capacity`.

use sc_extract::generated::{
    InventoryContainer, InventoryContainerTypeBasePtr, RecordLookup, SBaseCargoUnitPtr,
};
use sc_extract::{DataPools, Guid, RecordStore};
use serde::{Deserialize, Serialize};

/// Edge of one SCU cell, m.
const CELL: f32 = 1.25;

/// An `InventoryContainer` record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub kind: InventoryKind,
    /// `interiorDimensions`, m.
    pub dimensions: Option<[f32; 3]>,
    /// A closed container's `capacity`, SCU.
    pub capacity: Option<f64>,
}

/// `InventoryContainer.inventoryType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InventoryKind {
    /// `InventoryOpenContainerType`: a cargo grid, placed in the world;
    /// `external` when it sits outside the hull (`isExternalContainer`).
    Open { external: bool },
    /// `InventoryOpenAlwaysContainerType`.
    OpenAlways,
    /// `InventoryClosedContainerType`: a container opened through a menu.
    Closed,
    /// No type, or one this crate does not read.
    Other,
}

impl Inventory {
    /// The `InventoryContainer` record `guid` names.
    pub fn from_record(guid: &Guid, store: &RecordStore) -> Option<Self> {
        let pools = &store.pools;
        let c = InventoryContainer::lookup(&store.records, guid)?.get(pools)?;
        let (kind, capacity) = match &c.inventory_type {
            Some(InventoryContainerTypeBasePtr::InventoryOpenContainerType(h)) => (
                InventoryKind::Open {
                    external: h.get(pools).is_some_and(|t| t.is_external_container),
                },
                None,
            ),
            Some(InventoryContainerTypeBasePtr::InventoryOpenAlwaysContainerType(_)) => {
                (InventoryKind::OpenAlways, None)
            }
            Some(InventoryContainerTypeBasePtr::InventoryClosedContainerType(h)) => (
                InventoryKind::Closed,
                h.get(pools)
                    .and_then(|t| t.capacity.as_ref())
                    .and_then(|p| scu(p, pools)),
            ),
            _ => (InventoryKind::Other, None),
        };
        Some(Self {
            kind,
            dimensions: c
                .interior_dimensions
                .and_then(|h| h.get(pools))
                .map(|v| [v.x, v.y, v.z]),
            capacity,
        })
    }

    /// What it holds, SCU: a closed container's capacity, an open one's
    /// whole 1.25 m cells along each axis multiplied (the Freelancer's
    /// rear grid, 2.5 × 11.25 × 3.75 m, holds 2 × 9 × 3 = 54).
    pub fn scu(&self) -> Option<f64> {
        match self.kind {
            InventoryKind::Closed => self.capacity,
            InventoryKind::Open { .. } | InventoryKind::OpenAlways => self.dimensions.map(|d| {
                d.iter()
                    .map(|&m| (m / CELL + 1e-3).floor() as f64)
                    .product()
            }),
            InventoryKind::Other => None,
        }
    }
}

/// A cargo amount in SCU (`SStandardCargoUnit`, `SCentiCargoUnit`,
/// `SMicroCargoUnit`); standard units are `f32`, kept to the µSCU so 10.28
/// reads as such.
fn scu(p: &SBaseCargoUnitPtr, pools: &DataPools) -> Option<f64> {
    match p {
        SBaseCargoUnitPtr::SStandardCargoUnit(h) => h
            .get(pools)
            .map(|u| (f64::from(u.standard_cargo_units) * 1e6).round() / 1e6),
        SBaseCargoUnitPtr::SCentiCargoUnit(h) => {
            h.get(pools).map(|u| f64::from(u.centi_scu) / 100.0)
        }
        SBaseCargoUnitPtr::SMicroCargoUnit(h) => {
            h.get(pools).map(|u| f64::from(u.micro_scu) / 1_000_000.0)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_cells() {
        let grid = |d: [f32; 3]| Inventory {
            kind: InventoryKind::Open { external: false },
            dimensions: Some(d),
            capacity: None,
        };
        // Freelancer rear + two mids: 54 + 6 + 6 = 66.
        assert_eq!(grid([2.5, 11.25, 3.75]).scu(), Some(54.0));
        assert_eq!(grid([2.5, 1.25, 3.75]).scu(), Some(6.0));
        // Constellation Mk V main grid: erkul's 240.
        assert_eq!(grid([7.5, 12.5, 5.0]).scu(), Some(240.0));
        let locker = Inventory {
            kind: InventoryKind::Closed,
            dimensions: Some([1.0, 1.0, 2.0]),
            capacity: Some(0.39),
        };
        assert_eq!(locker.scu(), Some(0.39));
    }
}
