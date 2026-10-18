//! A mining setup (lasers with their modules, gadgets on the rock) and the
//! rock as that setup works on it.

use std::collections::BTreeMap;
use std::sync::Arc;

use sc_items_mining::{FloatModifier, MiningGadget, MiningModifiers, MiningModule, ModifierKind};
use sc_items_ship_weapons::{MiningBeam, MiningLaserStats};
use serde::{Deserialize, Serialize};
use tracing::warn;

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
    /// The modules whose effect is on for the whole attempt: passive ones,
    /// and active ones taken as switched on throughout. Activations at given
    /// times go in [`Setup::activations`].
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
        self.beam_power_in_range(beam) * beam.falloff(self.distance_m)
    }

    /// The beam's energy times every module's multiplier, before falloff.
    fn beam_power_in_range(&self, beam: &MiningBeam) -> f32 {
        let modules: f32 = self
            .modules
            .iter()
            .map(|m| m.damage_multiplier(beam.fire_action))
            .product();
        beam.energy_per_second * modules
    }

    fn laser_power(&self) -> LaserPower {
        let beam = self.stats.fracture_beam();
        LaserPower {
            power: self.fracture_power(),
            throttle_minimum: self.stats.throttle_minimum,
            throttle_lerp_speed: self.stats.throttle_lerp_speed,
            distance_m: self.distance_m,
            power_in_range: beam.map_or(0.0, |b| self.beam_power_in_range(b)),
            full_damage_range_m: beam.map_or(0.0, |b| b.full_damage_range),
            zero_damage_range_m: beam.map_or(0.0, |b| b.zero_damage_range),
        }
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
    /// Active modules switched on during the attempt. Only [`Timeline`]
    /// (and so [`simulate_timeline`](crate::simulate_timeline)) applies
    /// them; [`ModifiedRock::new`] takes the setup as fitted.
    #[serde(default)]
    pub activations: Vec<ModuleActivation>,
}

/// An active module switched on at `start_s` on one laser: its modifiers
/// (power multiplier included) work for its lifetime from then.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleActivation {
    /// Index into [`Setup::lasers`].
    pub laser: usize,
    pub module: MiningModule,
    /// Seconds into the attempt.
    pub start_s: f32,
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
    /// The shatter-damage modifier (`shatterdamageModifier`): scales the
    /// rock's explosion multiplier in its [`FractureBlast`](crate::FractureBlast).
    pub shatter_damage: Combined,
    /// The bare rock and every modifier entry, so the rock can be taken
    /// again with only some lasers hitting ([`with_lasers`](Self::with_lasers)).
    #[serde(skip)]
    sources: Option<Arc<Sources>>,
}

/// What a [`ModifiedRock`] was built from.
#[derive(Debug, Clone, PartialEq)]
struct Sources {
    rock: Rock,
    /// Per laser, in setup order: its owner and its modifier blocks (its own
    /// and its modules').
    lasers: Vec<(u32, Vec<MiningModifiers>)>,
    gadgets: Vec<MiningModifiers>,
}

/// One laser's contribution to [`ModifiedRock`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LaserPower {
    /// Fracturing energy per second at full throttle, at
    /// [`distance_m`](Self::distance_m).
    pub power: f32,
    pub throttle_minimum: f32,
    /// `throttleLerpSpeed`, per second: how fast the throttle follows its
    /// setting (see [`next_throttle`](Self::next_throttle)).
    pub throttle_lerp_speed: f32,
    /// The [`Laser::distance_m`] the power is taken at.
    pub distance_m: f32,
    /// Fracturing energy per second at full throttle inside the optimal
    /// range (the beam's energy times the modules' multipliers).
    pub power_in_range: f32,
    /// The fracturing beam's `fullDamageRange` / `zeroDamageRange`, m.
    pub full_damage_range_m: f32,
    pub zero_damage_range_m: f32,
}

impl LaserPower {
    /// Fracturing energy per second at full throttle with the beam's start
    /// `distance_m` from the rock surface: full power inside the optimal
    /// range, falling linearly to 0 at the maximum range (the falloff of
    /// [`MiningBeam::falloff`]).
    pub fn power_at(&self, distance_m: f32) -> f32 {
        if distance_m == self.distance_m {
            return self.power;
        }
        let falloff = if self.full_damage_range_m >= self.zero_damage_range_m {
            1.0
        } else {
            (1.0 - (distance_m - self.full_damage_range_m)
                / (self.zero_damage_range_m - self.full_damage_range_m))
                .clamp(0.0, 1.0)
        };
        self.power_in_range * falloff
    }

    /// The throttle one update of `dt` seconds later, moving from `current`
    /// toward the setting `desired`.
    ///
    /// Client code (LIVE 4.10.197, `CEntityComponentMiningLaser`): `current
    /// += clamp(throttleLerpSpeed * dt, throttleMinimum, 1) * (desired -
    /// current)`, snapping to `desired` within 1e-5. At 30 updates a second
    /// a lerp speed of 10 follows a step with a time constant of about
    /// 0.08 s, 6.5 with about 0.14 s.
    pub fn next_throttle(&self, current: f32, desired: f32, dt: f32) -> f32 {
        let step = self.throttle_lerp_speed * dt;
        let alpha = if self.throttle_minimum <= step {
            step.min(1.0)
        } else {
            self.throttle_minimum
        };
        let next = current + alpha * (desired - current);
        if (next - desired).abs() < 1e-5 {
            desired
        } else {
            next
        }
    }

    /// The throttle the rock works with: held between the laser's minimum
    /// and 1.
    pub(crate) fn effective(&self, throttle: f32) -> f32 {
        throttle.max(self.throttle_minimum).min(1.0)
    }
}

impl ModifiedRock {
    pub fn new(ruleset: &Ruleset, rock: &Rock, setup: &Setup) -> Self {
        let sources = Arc::new(Sources {
            rock: rock.clone(),
            lasers: setup
                .lasers
                .iter()
                .map(|l| (l.owner, l.modifier_blocks().copied().collect()))
                .collect(),
            gadgets: setup.gadgets.iter().map(|g| g.modifiers).collect(),
        });
        let lasers = setup.lasers.iter().map(Laser::laser_power).collect();
        Self::from_sources(ruleset, sources, lasers, None)
    }

    /// The rock with only the lasers marked in `contributing` (setup order)
    /// modifying it, as when the others have stopped hitting for longer
    /// than [`modifier_persistence_time`](Ruleset::modifier_persistence_time).
    /// Gadgets always modify it, and [`lasers`](Self::lasers) still lists
    /// every laser; a laser missing from `contributing` counts as hitting.
    pub fn with_lasers(
        ruleset: &Ruleset,
        rock: &Rock,
        setup: &Setup,
        contributing: &[bool],
    ) -> Self {
        Self::new(ruleset, rock, setup).subset(ruleset, contributing)
    }

    /// This rock with only the lasers marked in `contributing` modifying it
    /// (see [`with_lasers`](Self::with_lasers)); itself when it does not
    /// know its sources (a deserialized rock).
    pub(crate) fn subset(&self, ruleset: &Ruleset, contributing: &[bool]) -> Self {
        match &self.sources {
            Some(sources) => Self::from_sources(
                ruleset,
                Arc::clone(sources),
                self.lasers.clone(),
                Some(contributing),
            ),
            None => self.clone(),
        }
    }

    fn from_sources(
        ruleset: &Ruleset,
        sources: Arc<Sources>,
        lasers: Vec<LaserPower>,
        contributing: Option<&[bool]>,
    ) -> Self {
        let on = |i: usize| contributing.and_then(|c| c.get(i).copied()).unwrap_or(true);
        let hitting: Vec<&(u32, Vec<MiningModifiers>)> = sources
            .lasers
            .iter()
            .enumerate()
            .filter(|&(i, _)| on(i))
            .map(|(_, l)| l)
            .collect();
        let blocks: Vec<&MiningModifiers> = hitting
            .iter()
            .flat_map(|(_, blocks)| blocks)
            .chain(&sources.gadgets)
            .collect();
        let combine = |pick: fn(&MiningModifiers) -> Option<&FloatModifier>| {
            Combined::of(blocks.iter().map(|b| pick(b)))
        };

        let resistance = combine(|m| m.resistance.as_ref());
        let mut instability = combine(|m| m.laser_instability.as_ref());
        instability.factor *= owner_factor(hitting.iter().map(|(owner, _)| *owner));
        let window = combine(|m| m.optimal_charge_window_size.as_ref());

        let rock = &sources.rock;
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
            lasers,
            optimal_rate: combine(|m| m.optimal_charge_window_rate.as_ref()),
            catastrophic_rate: combine(|m| m.catastrophic_charge_window_rate.as_ref()),
            cluster_factor: combine(|m| m.cluster_factor.as_ref()),
            shatter_damage: combine(|m| m.shatter_damage.as_ref()),
            sources: Some(sources),
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
        let energy: f32 = self
            .lasers
            .iter()
            .map(|l| l.power * l.effective(throttle))
            .sum();
        self.gain_per_energy(ruleset) * energy
    }

    /// Charge-bar gain per second per unit of fracturing energy per second:
    /// `(1 - R) / (mass * powerCapacityPerMass)`.
    pub(crate) fn gain_per_energy(&self, ruleset: &Ruleset) -> f32 {
        let capacity = self.mass * ruleset.power_capacity_per_mass;
        if capacity <= 0.0 {
            return 0.0;
        }
        (1.0 - self.resistance) / capacity
    }
}

/// The rock as a [`Setup`] works on it over an attempt: one [`ModifiedRock`]
/// per stretch between [`Setup::activations`] switching on and off.
///
/// An activation works over `[start, start + lifetime)`; a module without a
/// lifetime stays on from its start. Client code (LIVE 4.10.197,
/// `CEntityComponentAttachableModifier`): every modifier of an active module
/// carries the same `ItemModifierTimedLife`, power multiplier included, and
/// is removed at the first check after it runs out, so within a frame; the
/// rock takes the new laser values with the next hit. Activations of one
/// module on one laser beyond its [`charges`](MiningModule::charges) are
/// ignored (no charge count, or 0, means unlimited), and so is one starting
/// while the same module is still on: whether the client refuses, restarts
/// or stacks a second activation is not read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    /// `(start_s, rock)`, by start; the first starts at 0.
    pub spans: Vec<(f32, ModifiedRock)>,
}

impl Timeline {
    pub fn new(ruleset: &Ruleset, rock: &Rock, setup: &Setup) -> Self {
        let windows = activation_windows(setup);
        let mut edges: Vec<f32> = windows
            .iter()
            .flat_map(|w| [w.start, w.end])
            .filter(|t| t.is_finite() && *t > 0.0)
            .collect();
        edges.push(0.0);
        edges.sort_by(f32::total_cmp);
        edges.dedup();
        let spans = edges
            .into_iter()
            .map(|t| {
                let mut live = setup.clone();
                live.activations.clear();
                for w in windows.iter().filter(|w| w.start <= t && t < w.end) {
                    live.lasers[w.laser].modules.push(w.module.clone());
                }
                (t, ModifiedRock::new(ruleset, rock, &live))
            })
            .collect();
        Self { spans }
    }

    /// The rock at `time_s` into the attempt.
    pub fn at(&self, time_s: f32) -> &ModifiedRock {
        &self.spans[self.span_index(time_s)].1
    }

    /// Index into [`spans`](Self::spans) of the stretch holding `time_s`.
    pub(crate) fn span_index(&self, time_s: f32) -> usize {
        self.spans
            .partition_point(|(start, _)| *start <= time_s)
            .saturating_sub(1)
    }
}

impl From<ModifiedRock> for Timeline {
    /// A setup with nothing switching: the same rock throughout.
    fn from(rock: ModifiedRock) -> Self {
        Self {
            spans: vec![(0.0, rock)],
        }
    }
}

/// One accepted activation.
struct Live {
    laser: usize,
    module: MiningModule,
    start: f32,
    end: f32,
}

/// The activations that take effect, with their end times.
fn activation_windows(setup: &Setup) -> Vec<Live> {
    let mut sorted: Vec<&ModuleActivation> = setup.activations.iter().collect();
    sorted.sort_by(|a, b| a.start_s.total_cmp(&b.start_s));
    let mut windows: Vec<Live> = Vec::new();
    for a in sorted {
        if a.laser >= setup.lasers.len() {
            warn!(
                laser = a.laser,
                "module activation on a laser the setup lacks"
            );
            continue;
        }
        let same: Vec<&Live> = windows
            .iter()
            .filter(|w| w.laser == a.laser && w.module.guid == a.module.guid)
            .collect();
        let used_up = a
            .module
            .charges()
            .is_some_and(|c| c > 0 && same.len() >= c as usize);
        if used_up || same.iter().any(|w| a.start_s < w.end) {
            continue;
        }
        let start = a.start_s.max(0.0);
        windows.push(Live {
            laser: a.laser,
            module: a.module.clone(),
            start,
            end: a.module.lifetime_s().map_or(f32::INFINITY, |l| start + l),
        });
    }
    windows
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
fn owner_factor(owners: impl Iterator<Item = u32>) -> f32 {
    let mut per_owner: BTreeMap<u32, u32> = BTreeMap::new();
    for owner in owners {
        *per_owner.entry(owner).or_default() += 1;
    }
    if per_owner.is_empty() {
        return 1.0;
    }
    per_owner.len() as f32 / per_owner.values().map(|&n| n as f32).product::<f32>()
}
