//! Gathering mode — which tool a resource is gathered with.

use serde::{Deserialize, Serialize};

/// The tool family a group's resources are gathered with.
///
/// Derived (Tier 2) from the rock's `MiningGlobalParams` family for mineables —
/// the per-rock ground truth — falling back to the group's `groupName` for
/// plants/salvage (which carry no `MiningGlobalParams`). `MiningGlobalParams`
/// has no typed mode field, so the family is read from its record name; the same
/// [`classify`](GatheringMode::classify) handles either source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatheringMode {
    /// Ship mining.
    Ship,
    /// Ground-vehicle (ROC) mining.
    GroundVehicle,
    /// FPS / handheld mining.
    Fps,
    /// Plants / flora harvesting.
    Plant,
    /// Salvage / debris.
    Salvage,
    /// Unrecognized — the source name matched no known family.
    Other,
}

impl GatheringMode {
    /// Classify from a name token — a `MiningGlobalParams` record name
    /// (`miningglobalparams_ship`/`_fps`/`_groundvehicle`) for mineables, or a
    /// `groupName` (`SpaceShip_Mineables`, `Salvage_*`, `Harvestables`) otherwise.
    pub fn classify(name: &str) -> Self {
        let n = name.to_ascii_lowercase();
        if n.contains("groundvehicle") {
            Self::GroundVehicle
        } else if n.contains("fps") {
            Self::Fps
        } else if n.contains("ship") {
            Self::Ship
        } else if n.contains("salvage") {
            Self::Salvage
        } else if n.contains("harvestable") || n.contains("plant") {
            Self::Plant
        } else {
            Self::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_global_params_and_group_names() {
        assert_eq!(
            GatheringMode::classify("miningglobalparamsship"),
            GatheringMode::Ship
        );
        assert_eq!(
            GatheringMode::classify("MiningGlobalParamsGroundVehicle"),
            GatheringMode::GroundVehicle
        );
        assert_eq!(
            GatheringMode::classify("miningglobalparamsfps"),
            GatheringMode::Fps
        );
        assert_eq!(
            GatheringMode::classify("SpaceShip_Mineables"),
            GatheringMode::Ship
        );
        assert_eq!(
            GatheringMode::classify("Salvage_FreshDerelicts"),
            GatheringMode::Salvage
        );
        assert_eq!(
            GatheringMode::classify("Harvestables"),
            GatheringMode::Plant
        );
        assert_eq!(
            GatheringMode::classify("Mystery_Group"),
            GatheringMode::Other
        );
    }
}
