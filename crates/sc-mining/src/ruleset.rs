//! Rulesets — the `MiningGlobalParams` records (one per gathering mode).

use std::collections::HashMap;

use sc_extract::generated::DamageBasePtr;
use sc_extract::{Datacore, Guid, RecordCollection};
use sc_items_ship_weapons::Damage;
use sc_resources::{DensityUnit, Resources};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::BlastParams;

/// One `MiningGlobalParams` record: the constants every rock pointing at it
/// is mined under (ship, ground-vehicle and FPS rocks each reference their own,
/// through `MineableParams.globalParams`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ruleset {
    pub guid: Guid,
    /// `powerCapacityPerMass`: the charge a rock holds per unit of mass.
    pub power_capacity_per_mass: f32,
    /// `decayPerMass`: the charge a rock loses per unit of mass and second.
    pub decay_per_mass: f32,
    /// `optimalWindowSize`: the window width before thinness and modifiers.
    pub optimal_window_size: f32,
    /// `optimalWindowFactor`: how far thinness can narrow the window.
    pub optimal_window_factor: f32,
    /// `optimalWindowMaxSize`: the cap on the modified window width.
    pub optimal_window_max_size: f32,
    /// `resistanceCurveFactor`: exponent on each element's fraction.
    pub resistance_curve_factor: f32,
    /// `optimalWindowThinnessCurveFactor`.
    pub optimal_window_thinness_curve_factor: f32,
    /// `mineableInstabilityParams.instabilityCurveFactor`.
    pub instability_curve_factor: f32,
    /// `mineableInstabilityParams.instabilityWavePeriod`, seconds.
    pub instability_wave_period: f32,
    /// `mineableInstabilityParams.instabilityWaveVariance`, seconds.
    pub instability_wave_variance: f32,
    /// `mineableExplosionParams.defaultVolume`, m³: instability and the
    /// fracture explosion scale with the rock's volume over this.
    pub default_volume: f32,
    /// `mineableExplosionParams.dangerPoolFactor`: how much a full danger
    /// pool adds to the fracture explosion's strength factor
    /// (see [`FractureBlast`](crate::FractureBlast)).
    pub danger_pool_factor: f32,
    /// `mineableExplosionParams.defaultExplosionParams`.
    pub blast: BlastParams,
    /// `defaultMass`: the mass the breaking pools' speed scale is relative to.
    pub default_mass: f32,
    /// `controlledBreakingFillRate`.
    pub controlled_breaking_fill_rate: f32,
    /// `controlledBreakingFillRateDanger`.
    pub controlled_breaking_fill_rate_danger: f32,
    /// `controlledBreakingDecayRate`.
    pub controlled_breaking_decay_rate: f32,
    /// `dangerBreakingFillRate`.
    pub danger_breaking_fill_rate: f32,
    /// `dangerBreakingFillRateExponent`.
    pub danger_breaking_fill_rate_exponent: f32,
    /// `dangerBreakingDecayRate`.
    pub danger_breaking_decay_rate: f32,
    /// `absorbableVolumeThreshold`, m³: rocks below it are not fractured
    /// further and are the only ones the extraction beam works on.
    pub absorbable_volume_threshold: f32,
    /// `modifierPersistenceTime`, seconds: how long a laser's modifiers stay
    /// on the rock after its last hit.
    pub modifier_persistence_time: f32,
    /// SCU per m³ of rock (`cSCUPerVolume`, stored in centi-SCU, / 100).
    pub scu_per_volume: f32,
    pub hit_consistency: HitConsistency,
    /// `wasteResourceType`: the resource standing for the inert part of a rock.
    pub waste_resource: Option<Guid>,
    /// Density of [`waste_resource`](Self::waste_resource), g/cm³ (t/m³).
    pub inert_density: Option<f32>,
}

/// `hitConsistencyParams`: how steady aim is scored (see
/// [`hit_consistency`](crate::hit_consistency)) and what it does.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HitConsistency {
    /// Seconds of hit history the score looks at.
    pub history_window: f32,
    pub standard_deviation_multiplier: f32,
    pub time_exponent: f32,
    /// Metres; the hit-point scatter is never counted as smaller than this.
    pub min_deviation: f32,
    /// How much a perfect score speeds up extraction (`x(1 + c * this)`).
    pub extraction_magnitude: f32,
    /// The most a perfect score can damp the instability wave.
    pub max_effect_on_instability: f32,
}

impl Default for HitConsistency {
    /// No effect: a score of 0 always, no extraction or wave effect.
    fn default() -> Self {
        Self {
            history_window: 1.0,
            standard_deviation_multiplier: 1.0,
            time_exponent: 1.0,
            min_deviation: 1.0,
            extraction_magnitude: 0.0,
            max_effect_on_instability: 0.0,
        }
    }
}

impl Ruleset {
    /// Volume of a rock that holds `scu` SCU (the scanner's SCU readout).
    pub fn volume_from_scu(&self, scu: f32) -> f32 {
        scu / self.scu_per_volume
    }

    /// Whether a rock of `volume_m3` can be collected directly.
    pub fn is_absorbable(&self, volume_m3: f32) -> bool {
        volume_m3 < self.absorbable_volume_threshold
    }
}

/// Every `MiningGlobalParams` record, keyed by GUID. A rock's
/// `MineableParams.globalParams` (sc-gathering's
/// `GatherableElement::global_params`) is the key.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rulesets {
    by_guid: HashMap<Guid, Ruleset>,
}

impl Rulesets {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read every `MiningGlobalParams` record; the inert density joins through
    /// [`Resources`].
    pub fn build(datacore: &Datacore, resources: &Resources) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut by_guid = HashMap::new();
        for (&guid, handle) in &store.records.multi_feature.mining_global_params {
            let Some(gp) = handle.get(pools) else {
                continue;
            };
            let instability = gp.mineable_instability_params.and_then(|h| h.get(pools));
            let explosion = gp.mineable_explosion_params.and_then(|h| h.get(pools));
            let hit_consistency = gp
                .hit_consistency_params
                .and_then(|h| h.get(pools))
                .map(|h| HitConsistency {
                    history_window: h.hit_history_window,
                    standard_deviation_multiplier: h.standard_deviation_multiplier,
                    time_exponent: h.time_exponent,
                    min_deviation: h.min_deviation,
                    extraction_magnitude: h.extraction_magnitude,
                    max_effect_on_instability: h.max_effect_on_instability,
                })
                .unwrap_or_default();
            let inert_density = gp.waste_resource_type.and_then(|g| density(resources, &g));
            if inert_density.is_none() {
                warn!(%guid, "mining ruleset without an inert density");
            }
            let ruleset = Ruleset {
                guid,
                power_capacity_per_mass: gp.power_capacity_per_mass,
                decay_per_mass: gp.decay_per_mass,
                optimal_window_size: gp.optimal_window_size,
                optimal_window_factor: gp.optimal_window_factor,
                optimal_window_max_size: gp.optimal_window_max_size,
                resistance_curve_factor: gp.resistance_curve_factor,
                optimal_window_thinness_curve_factor: gp.optimal_window_thinness_curve_factor,
                instability_curve_factor: instability.map_or(1.0, |i| i.instability_curve_factor),
                instability_wave_period: instability.map_or(0.0, |i| i.instability_wave_period),
                instability_wave_variance: instability.map_or(0.0, |i| i.instability_wave_variance),
                default_volume: explosion.map_or(0.0, |e| e.default_volume),
                danger_pool_factor: explosion.map_or(0.0, |e| e.danger_pool_factor),
                blast: explosion
                    .and_then(|e| e.default_explosion_params)
                    .and_then(|h| h.get(pools))
                    .map(|e| BlastParams {
                        min_radius: e.min_radius,
                        max_radius: e.max_radius,
                        pressure: e.pressure,
                        damage: match e.damage {
                            Some(DamageBasePtr::DamageInfo(h)) => {
                                h.get(pools).map_or_else(Damage::default, |d| Damage {
                                    physical: d.damage_physical,
                                    energy: d.damage_energy,
                                    distortion: d.damage_distortion,
                                    thermal: d.damage_thermal,
                                    biochemical: d.damage_biochemical,
                                    stun: d.damage_stun,
                                })
                            }
                            _ => Damage::default(),
                        },
                    })
                    .unwrap_or_default(),
                default_mass: gp.default_mass,
                controlled_breaking_fill_rate: gp.controlled_breaking_fill_rate,
                controlled_breaking_fill_rate_danger: gp.controlled_breaking_fill_rate_danger,
                controlled_breaking_decay_rate: gp.controlled_breaking_decay_rate,
                danger_breaking_fill_rate: gp.danger_breaking_fill_rate,
                danger_breaking_fill_rate_exponent: gp.danger_breaking_fill_rate_exponent,
                danger_breaking_decay_rate: gp.danger_breaking_decay_rate,
                absorbable_volume_threshold: gp.absorbable_volume_threshold,
                modifier_persistence_time: gp.modifier_persistence_time,
                scu_per_volume: gp.c_scuper_volume / 100.0,
                hit_consistency,
                waste_resource: gp.waste_resource_type,
                inert_density,
            };
            debug!(%guid, ?ruleset, "mining ruleset");
            by_guid.insert(guid, ruleset);
        }
        Self { by_guid }
    }
}

impl RecordCollection for Rulesets {
    type Item = Ruleset;

    fn get(&self, guid: &Guid) -> Option<&Ruleset> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Ruleset)> + '_ {
        self.by_guid.iter()
    }
}

/// A resource's density in g/cm³, if it has one in that unit.
pub(crate) fn density(resources: &Resources, guid: &Guid) -> Option<f32> {
    match resources.get(guid)?.density.as_ref()?.unit.as_ref()? {
        DensityUnit::GramsPerCm3(v) => Some(*v),
        DensityUnit::Other { .. } => None,
    }
}
