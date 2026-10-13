//! A mining setup (lasers with their modules, gadgets on the rock) and the
//! rock as that setup works on it.

use std::collections::BTreeMap;

use sc_items_mining::{FloatModifier, MiningGadget, MiningModifiers, MiningModule, ModifierKind};
use sc_items_ship_weapons::{MiningBeam, MiningLaserStats};
use serde::{Deserialize, Serialize};

use crate::rock::volume_scale;
use crate::{Rock, Ruleset, Window};

/// Rock volume (m³) one unit of extraction energy collects: the client adds
/// `(1 + c * extractionMagnitude) * 0.013 * energy / V` to the extracted
/// fraction per hit (a literal in the client code, LIVE 4.10).
const EXTRACTION_PER_ENERGY: f32 = 0.013;

/// One mining laser as fitted: its stats, the modules in it, how far it is
/// from the rock and which ship operates it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Laser {
    pub stats: MiningLaserStats,
    /// The modules whose effect is on: passive ones always, active ones only
    /// while activated.
    pub modules: Vec<MiningModule>,
    /// Distance from the beam's start to the rock surface, m. Inside the
    /// beam's optimal range it makes no difference.
    pub distance_m: f32,
    /// Which ship (or player) the laser belongs to. Lasers sharing an owner
    /// split the instability wave between them; see [`ModifiedRock`].
    pub owner: u32,
}

impl Laser {
    /// A laser with no modules, at the rock, on ship 0.
    pub fn new(stats: MiningLaserStats) -> Self {
        Self {
            stats,
            modules: Vec::new(),
            distance_m: 0.0,
            owner: 0,
        }
    }

    /// Fracturing energy per second at full throttle: the beam's energy times
    /// every module's multiplier for that beam, times the range falloff.
    pub fn fracture_power(&self) -> f32 {
        self.stats
            .fracture_beam()
            .map_or(0.0, |beam| self.beam_power(beam))
    }

    /// Extraction energy per second (extraction ignores the throttle).
    pub fn extraction_power(&self) -> f32 {
        self.stats
            .extraction_beam()
            .map_or(0.0, |beam| self.beam_power(beam))
    }

    /// SCU per second the extraction beam collects from an absorbable rock,
    /// whatever its size, at hit consistency `consistency` (0..1, see
    /// [`hit_consistency`](crate::hit_consistency)). A rock takes its SCU
    /// divided by this.
    pub fn extraction_scu_per_s(&self, ruleset: &Ruleset, consistency: f32) -> f32 {
        let boost =
            1.0 + consistency.clamp(0.0, 1.0) * ruleset.hit_consistency.extraction_magnitude;
        boost * EXTRACTION_PER_ENERGY * self.extraction_power() * ruleset.scu_per_volume
    }

    fn beam_power(&self, beam: &MiningBeam) -> f32 {
        let modules: f32 = self
            .modules
            .iter()
            .map(|m| m.damage_multiplier(beam.fire_action))
            .product();
        beam.energy_per_second * modules * beam.falloff(self.distance_m)
    }

    /// Every modifier block this laser puts on the rock: its own and its
    /// modules'.
    fn modifier_blocks(&self) -> impl Iterator<Item = &MiningModifiers> {
        std::iter::once(&self.stats.modifiers).chain(self.modules.iter().map(|m| &m.modifiers))
    }
}

/// Everything working on one rock.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Setup {
    pub lasers: Vec<Laser>,
    /// Gadgets attached to the rock.
    pub gadgets: Vec<MiningGadget>,
}

/// How the setup's modifiers on one stat combine: `value * factor + add`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Combined {
    pub factor: f32,
    pub add: f32,
}

impl Combined {
    pub const NONE: Self = Self {
        factor: 1.0,
        add: 0.0,
    };

    pub fn apply(&self, value: f32) -> f32 {
        value * self.factor + self.add
    }

    /// Multiplicative values multiply as `1 + v/100`; additive ones add as
    /// `v / 100`. The client folds additive entries one at a time as
    /// `(add + v) / 100`; that equals this sum for the single additive item in
    /// the data (the ROC-DS module) and is not modelled for more.
    fn of<'a>(modifiers: impl Iterator<Item = Option<&'a FloatModifier>>) -> Self {
        let mut combined = Self::NONE;
        for m in modifiers.flatten() {
            match m.kind {
                ModifierKind::Multiplicative => combined.factor *= m.factor(),
                ModifierKind::Additive => combined.add += m.value / 100.0,
            }
        }
        combined
    }
}

/// The rock as a [`Setup`] works on it: its values with every laser's and
/// gadget's modifiers applied (what the scanner shows once the beams land),
/// and the setup's fracturing power.
///
/// Modifiers (client code, LIVE 4.10): each laser (with its modules) and
/// each gadget puts one entry on the rock, refreshed by every hit and dropped
/// [`modifier_persistence_time`](Ruleset::modifier_persistence_time) after
/// the last, so a laser that stops hitting stops modifying. Resistance is
/// `clamp(R * factor + add, 0, 1)`; instability `max(0, (I * factor + add) *
/// V / defaultVolume)`, its factor also times `owners / Π lasers per owner`
/// (two lasers on one ship halve the wave, two ships double it; "owner" read
/// as the ship is inferred, not traced); the window's width scales around
/// its centre, capped at [`optimal_window_max_size`](Ruleset::optimal_window_max_size).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifiedRock {
    pub mass: f32,
    pub volume: f32,
    pub resistance: f32,
    pub instability: f32,
    pub window: Window,
    /// Per laser: fracturing energy per second at full throttle, and the
    /// lowest throttle it can be set to.
    pub lasers: Vec<LaserPower>,
    /// The controlled-breaking fill inside the window (`optimalChargeWindowRate`).
    pub optimal_rate: Combined,
    /// The fill above the window (`catastrophicChargeWindowRate`).
    pub catastrophic_rate: Combined,
    /// The cluster-factor modifier (`clusterFactorModifier`): how strongly a
    /// fracture concentrates each element (see [`Spawn::fracture`](crate::Spawn::fracture)).
    pub cluster_factor: Combined,
}

/// One laser's contribution to [`ModifiedRock`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LaserPower {
    pub power: f32,
    pub throttle_minimum: f32,
}

impl ModifiedRock {
    pub fn new(ruleset: &Ruleset, rock: &Rock, setup: &Setup) -> Self {
        let blocks: Vec<&MiningModifiers> = setup
            .lasers
            .iter()
            .flat_map(Laser::modifier_blocks)
            .chain(setup.gadgets.iter().map(|g| &g.modifiers))
            .collect();
        let combine = |pick: fn(&MiningModifiers) -> Option<&FloatModifier>| {
            Combined::of(blocks.iter().map(|b| pick(b)))
        };

        let resistance = combine(|m| m.resistance.as_ref());
        let mut instability = combine(|m| m.laser_instability.as_ref());
        instability.factor *= owner_factor(&setup.lasers);
        let window = combine(|m| m.optimal_charge_window_size.as_ref());

        Self {
            mass: rock.mass,
            volume: rock.volume,
            resistance: resistance.apply(rock.resistance).clamp(0.0, 1.0),
            instability: (instability.apply(rock.instability_unscaled)
                * volume_scale(ruleset, rock.volume))
            .max(0.0),
            window: rock
                .window
                .scaled(window.factor + window.add, ruleset.optimal_window_max_size),
            lasers: setup
                .lasers
                .iter()
                .map(|l| LaserPower {
                    power: l.fracture_power(),
                    throttle_minimum: l.stats.throttle_minimum,
                })
                .collect(),
            optimal_rate: combine(|m| m.optimal_charge_window_rate.as_ref()),
            catastrophic_rate: combine(|m| m.catastrophic_charge_window_rate.as_ref()),
            cluster_factor: combine(|m| m.cluster_factor.as_ref()),
        }
    }

    /// The setup's fracturing energy per second at full throttle.
    pub fn power(&self) -> f32 {
        self.lasers.iter().map(|l| l.power).sum()
    }

    /// Charge-bar change per second at `throttle` (every laser at the same
    /// setting, held at its minimum), without the instability wave: gain
    /// `(1 - R) * E * throttle / (mass * powerCapacityPerMass)` minus decay
    /// `decayPerMass / powerCapacityPerMass`.
    pub fn net_rate(&self, ruleset: &Ruleset, throttle: f32) -> f32 {
        self.gain_rate(ruleset, throttle) - decay_rate(ruleset)
    }

    /// The heaviest rock of this resistance the setup can charge at all:
    /// `(1 - R) * E / decayPerMass`. Heavier rocks lose charge faster than
    /// the lasers add it (the game's IMPOSSIBLE).
    pub fn max_mass(&self, ruleset: &Ruleset) -> f32 {
        if ruleset.decay_per_mass > 0.0 {
            (1.0 - self.resistance) * self.power() / ruleset.decay_per_mass
        } else {
            f32::INFINITY
        }
    }

    pub fn is_breakable(&self, ruleset: &Ruleset) -> bool {
        self.mass < self.max_mass(ruleset)
    }

    /// The throttle at which the bar holds still (gain equals decay); `None`
    /// when even full throttle cannot hold it. Above it the bar rises and the
    /// instability wave amplifies the rise; below it the bar falls at the
    /// plain decay rate.
    pub fn balance_throttle(&self, ruleset: &Ruleset) -> Option<f32> {
        let full = self.gain_rate(ruleset, 1.0);
        let t = decay_rate(ruleset) / full;
        (full > 0.0 && t <= 1.0).then_some(t)
    }

    /// Whether the bar still rises with every laser at its throttle minimum,
    /// so the charge cannot be held and the rock will overcharge.
    pub fn overcharges_at_minimum(&self, ruleset: &Ruleset) -> bool {
        self.net_rate(ruleset, 0.0) > 0.0
    }

    /// Seconds from an empty bar to the window at full throttle, without the
    /// wave (which only shortens it); `None` when the bar cannot rise.
    pub fn time_to_window_s(&self, ruleset: &Ruleset) -> Option<f32> {
        let rate = self.net_rate(ruleset, 1.0);
        (rate > 0.0).then(|| self.window.low / rate)
    }

    /// Seconds inside the window that fill the controlled-breaking pool (a
    /// clean fracture when the bar never went above the window).
    pub fn time_in_window_s(&self, ruleset: &Ruleset) -> Option<f32> {
        let rate = self
            .optimal_rate
            .apply(breaking_scale(ruleset, self.mass) * ruleset.controlled_breaking_fill_rate);
        (rate > 0.0).then(|| 1.0 / rate)
    }

    /// Charge-bar gain per second at `throttle`, before decay and the wave.
    pub fn gain_rate(&self, ruleset: &Ruleset, throttle: f32) -> f32 {
        let capacity = self.mass * ruleset.power_capacity_per_mass;
        if capacity <= 0.0 {
            return 0.0;
        }
        let energy: f32 = self
            .lasers
            .iter()
            .map(|l| l.power * throttle.max(l.throttle_minimum).min(1.0))
            .sum();
        (1.0 - self.resistance) * energy / capacity
    }
}

/// Bar lost per second (mass cancels out).
pub(crate) fn decay_rate(ruleset: &Ruleset) -> f32 {
    if ruleset.power_capacity_per_mass > 0.0 {
        ruleset.decay_per_mass / ruleset.power_capacity_per_mass
    } else {
        0.0
    }
}

/// The breaking pools' speed scale `max(0.1, (defaultMass - mass) /
/// defaultMass + 1)`: 0.1 for every rock heavier than 1.9 default masses.
pub(crate) fn breaking_scale(ruleset: &Ruleset, mass: f32) -> f32 {
    if ruleset.default_mass > 0.0 {
        ((ruleset.default_mass - mass) / ruleset.default_mass + 1.0).max(0.1)
    } else {
        0.1
    }
}

/// `owners / Π lasers per owner`.
fn owner_factor(lasers: &[Laser]) -> f32 {
    let mut per_owner: BTreeMap<u32, u32> = BTreeMap::new();
    for laser in lasers {
        *per_owner.entry(laser.owner).or_default() += 1;
    }
    if per_owner.is_empty() {
        return 1.0;
    }
    per_owner.len() as f32 / per_owner.values().map(|&n| n as f32).product::<f32>()
}
