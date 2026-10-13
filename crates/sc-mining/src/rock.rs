//! A rock's own properties, from its composition (no laser applied).

use sc_extract::{Guid, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::{Elements, Ruleset};

/// One line of a rock's composition, as the scanner lists it: an element and
/// its share of the rock's volume (0..1). The inert part is not an entry; it
/// is whatever the entries leave.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub element: Guid,
    pub fraction: f32,
}

/// The optimal charge window, as fractions of the charge bar.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Window {
    pub low: f32,
    pub high: f32,
}

impl Window {
    /// The window `centre ∓ size / 2`, clamped to the bar.
    pub fn around(centre: f32, size: f32) -> Self {
        Self {
            low: (centre - size / 2.0).clamp(0.0, 1.0),
            high: (centre + size / 2.0).clamp(0.0, 1.0),
        }
    }

    pub fn size(&self) -> f32 {
        self.high - self.low
    }

    pub fn centre(&self) -> f32 {
        (self.low + self.high) / 2.0
    }

    pub fn contains(&self, power: f32) -> bool {
        (self.low..=self.high).contains(&power)
    }

    /// The window with its half-width scaled by `factor` around its centre,
    /// capped at `max_size` wide (how the client applies the window modifier).
    pub fn scaled(&self, factor: f32, max_size: f32) -> Self {
        let size = (self.size() * factor).clamp(0.0, max_size);
        Self::around(self.centre(), size)
    }
}

/// A rock's own properties: what the scanner shows before a laser hits it.
///
/// Built from the composition the scanner lists. Formulas (client code, LIVE
/// 4.10, checked against in-game scans): `x` is an entry's fraction,
/// `ruleset` the rock's [`Ruleset`].
///
/// - resistance: entries in order of falling element resistance, each
///   `w = clamp(x^resistanceCurveFactor * r, -1, 1)`, then `R += w * (1 - R)`
///   for `w > 0` and `R += w * R` otherwise, clamped to 0..1;
/// - instability: `max(0, Σ x^instabilityCurveFactor * i) * V / defaultVolume`;
/// - window: thinness `T = clamp(Σ x^thinnessCurveFactor * t, 0, 1)`, size
///   `(1 - optimalWindowFactor * T) * optimalWindowSize`, around the midpoint
///   `Σ x * m / Σ x` plus a per-rock random shift of up to
///   [`window_midpoint_spread`](Self::window_midpoint_spread);
/// - mass: `V * (Σ x * ρ + inert * ρ_inert)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rock {
    pub entries: Vec<Entry>,
    /// m³.
    pub volume: f32,
    /// Tonnes (m³ times g/cm³), the scanner's mass readout.
    pub mass: f32,
    /// 0..1. The scanner shows it truncated to whole percent.
    pub resistance: f32,
    /// `Σ x^c * i` before the volume scaling; modifiers apply to this.
    pub instability_unscaled: f32,
    /// The scanner's instability readout (volume-scaled, no laser).
    pub instability: f32,
    /// The window without a laser's modifier.
    pub window: Window,
    /// The largest shift the per-rock random can move the window midpoint.
    pub window_midpoint_spread: f32,
    pub explosion_multiplier: f32,
    pub scu: f32,
}

impl Rock {
    /// A rock of `volume_m3` with this composition. `None` when an entry's
    /// element is unknown or lacks a density, or the ruleset has no inert
    /// density.
    pub fn from_volume(
        ruleset: &Ruleset,
        elements: &Elements,
        entries: &[Entry],
        volume_m3: f32,
    ) -> Option<Self> {
        let density = mean_density(ruleset, elements, entries)?;
        Self::build(ruleset, elements, entries, volume_m3, volume_m3 * density)
    }

    /// A rock of `mass` tonnes with this composition (the scanner shows mass;
    /// the volume follows from the densities).
    pub fn from_mass(
        ruleset: &Ruleset,
        elements: &Elements,
        entries: &[Entry],
        mass: f32,
    ) -> Option<Self> {
        let density = mean_density(ruleset, elements, entries)?;
        Self::build(ruleset, elements, entries, mass / density, mass)
    }

    /// The same rock with the window centred on `midpoint` instead of the
    /// nominal one, for a rock whose window was seen in game.
    pub fn with_window_midpoint(mut self, midpoint: f32) -> Self {
        self.window = Window::around(midpoint, self.window.size());
        self
    }

    /// Whether the extraction beam can collect it directly.
    pub fn is_absorbable(&self, ruleset: &Ruleset) -> bool {
        ruleset.is_absorbable(self.volume)
    }

    fn build(
        ruleset: &Ruleset,
        elements: &Elements,
        entries: &[Entry],
        volume: f32,
        mass: f32,
    ) -> Option<Self> {
        let mut resolved = Vec::with_capacity(entries.len());
        for entry in entries {
            resolved.push((entry.fraction.max(0.0), elements.get(&entry.element)?));
        }

        let mut by_resistance = resolved.clone();
        by_resistance.sort_by(|a, b| b.1.resistance.total_cmp(&a.1.resistance));
        let mut resistance = 0.0f32;
        for (x, e) in &by_resistance {
            let w = (x.powf(ruleset.resistance_curve_factor) * e.resistance).clamp(-1.0, 1.0);
            resistance += w * if w > 0.0 {
                1.0 - resistance
            } else {
                resistance
            };
            resistance = resistance.clamp(0.0, 1.0);
        }

        let instability_unscaled = resolved
            .iter()
            .map(|(x, e)| x.powf(ruleset.instability_curve_factor) * e.instability)
            .sum::<f32>()
            .max(0.0);

        let thinness = resolved
            .iter()
            .map(|(x, e)| x.powf(ruleset.optimal_window_thinness_curve_factor) * e.window_thinness)
            .sum::<f32>()
            .clamp(0.0, 1.0);
        let size = (1.0 - ruleset.optimal_window_factor * thinness) * ruleset.optimal_window_size;
        let total: f32 = resolved.iter().map(|(x, _)| x).sum();
        let (midpoint, spread) = if total > 0.0 {
            (
                (resolved
                    .iter()
                    .map(|(x, e)| x * e.window_midpoint)
                    .sum::<f32>()
                    / total)
                    .clamp(0.0, 1.0),
                resolved
                    .iter()
                    .map(|(x, e)| x * e.window_midpoint_randomness)
                    .sum::<f32>()
                    / total,
            )
        } else {
            (0.5, 0.0)
        };

        Some(Self {
            entries: entries.to_vec(),
            volume,
            mass,
            resistance,
            instability_unscaled,
            instability: instability_unscaled * volume_scale(ruleset, volume),
            window: Window::around(midpoint, size),
            window_midpoint_spread: spread,
            explosion_multiplier: resolved
                .iter()
                .map(|(x, e)| x * e.explosion_multiplier)
                .sum(),
            scu: volume * ruleset.scu_per_volume,
        })
    }
}

/// `V / defaultVolume`, the factor instability is scaled by.
pub(crate) fn volume_scale(ruleset: &Ruleset, volume: f32) -> f32 {
    if ruleset.default_volume > 0.0 {
        volume / ruleset.default_volume
    } else {
        1.0
    }
}

/// Mass per volume of the composition, inert part included.
fn mean_density(ruleset: &Ruleset, elements: &Elements, entries: &[Entry]) -> Option<f32> {
    let mut density = 0.0;
    let mut filled = 0.0;
    for entry in entries {
        let x = entry.fraction.max(0.0);
        density += x * elements.get(&entry.element)?.density?;
        filled += x;
    }
    let inert = (1.0 - filled).max(0.0);
    if inert > 0.0 {
        density += inert * ruleset.inert_density?;
    }
    (density > 0.0).then_some(density)
}
