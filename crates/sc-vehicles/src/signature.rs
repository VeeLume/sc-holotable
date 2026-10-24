//! A ship's signatures as its HUD shows them.

use sc_items_ship_components::VehicleArmor;
use serde::{Deserialize, Serialize};

use crate::{Readout, Vehicle};

/// A ship's EM, IR and cross-section: what its items emit and its hull's
/// cross-section, each times its armour's signal multiplier.
///
/// Client (4.10.197): the armour installs itself into the vehicle's
/// signature system as a modifier when it attaches (`FUN_143f5abe0` →
/// `CSCSignatureSystem::InstallSignatureModifier`) and leaves when it
/// detaches; the cross-section is the manual `crossSection` per axis times
/// a multiplier (`FUN_143aa9d60`). That the multiplier is the armour's
/// `signal*` value over the summed items is inferred, not read; it
/// reproduces the F7C-M Super Hornet Mk II's screen (armour 1.13) to the
/// displayed digit. The cross-section a ship generates while firing is not
/// counted.
///
/// ```no_run
/// # fn f(
/// #     network: &sc_vehicles::PowerNetwork,
/// #     allocation: &sc_vehicles::Allocation,
/// #     defense: &sc_vehicles::Defense,
/// #     vehicle: &sc_vehicles::Vehicle,
/// # ) {
/// use sc_extract::generated::EMasterMode;
/// use sc_vehicles::ShipSignature;
///
/// let readout = network.readout(allocation, &EMasterMode::SCM);
/// let ship = ShipSignature::compute(&readout, vehicle, defense.armor.as_ref());
/// println!("EM {:.0} IR {:.0} CS {:?}", ship.em, ship.ir, ship.cs());
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ShipSignature {
    /// [`Readout::em`] × `signalElectromagnetic`.
    pub em: f64,
    /// [`Readout::ir`] × `signalInfrared`.
    pub ir: f64,
    /// [`Vehicle::cross_section`] per axis × `signalCrossSection`; `None`
    /// when the hull's cross-section comes from its geometry.
    pub cross_section: Option<[f32; 3]>,
}

impl ShipSignature {
    /// The signatures of a loadout on `vehicle` whose network reads
    /// `readout`, under `armor` ([`Defense::armor`](crate::Defense::armor);
    /// without one nothing is scaled).
    pub fn compute(readout: &Readout, vehicle: &Vehicle, armor: Option<&VehicleArmor>) -> Self {
        let (em, ir, cs) = armor.map_or((1.0, 1.0, 1.0), |a| {
            (
                a.signal_electromagnetic,
                a.signal_infrared,
                a.signal_cross_section,
            )
        });
        Self {
            em: readout.em * f64::from(em),
            ir: readout.ir * f64::from(ir),
            cross_section: vehicle.cross_section.map(|v| v.map(|x| x * cs)),
        }
    }

    /// The cross-section the HUD shows: the largest axis
    /// (`CSCSignatureSystem::OnUpdateSignatures`, `FUN_143adfba0`).
    pub fn cs(&self) -> Option<f32> {
        self.cross_section.map(|[x, y, z]| x.max(y).max(z))
    }
}
