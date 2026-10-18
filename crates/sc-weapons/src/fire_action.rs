use sc_extract::DataPools;
use sc_extract::generated::*;

use crate::damage::DamageSummary;

/// Primary fire action type with extracted scalars.
///
/// Covers all 9 `SWeaponActionParamsPtr` variants. Combat-irrelevant types
/// (HealingBeam, Parallel) map to [`Unknown`](FireActionKind::Unknown).
#[derive(Debug, Clone)]
pub enum FireActionKind {
    /// Continuous automatic fire (gatlings, LMGs).
    Rapid {
        fire_rate: i32,
        heat_per_shot: f32,
        spin_up: f32,
        spin_down: f32,
    },
    /// Semi-automatic (cannons, scatter guns, pistols).
    Single { fire_rate: i32, heat_per_shot: f32 },
    /// Multi-barrel round-robin (repeaters, cannons).
    /// `effective_rpm`: entries per pass of the sequence, see
    /// `sequence_timing` (erkul's rule, the client's per-barrel gating).
    Sequence {
        effective_rpm: f32,
        barrel_count: usize,
    },
    /// Burst fire (Klaus & Werner energy weapons).
    Burst {
        fire_rate: i32,
        shot_count: u32,
        cooldown: f32,
    },
    /// Charge-then-fire (mass drivers, tachyon cannons, railgun).
    Charged {
        charge_time: f32,
        overcharge_time: f32,
        cooldown: f32,
        auto_fire: bool,
        full_only: bool,
        /// Modifiers applied at full charge (damage/speed/pellet/etc.).
        /// `None` if `maxChargeModifier` is unset.
        max_modifier: Option<ChargeModifier>,
    },
    /// Continuous beam (ship beams, mining lasers).
    Beam {
        dps: DamageSummary,
        full_range: f32,
        zero_range: f32,
        heat_per_sec: f32,
    },
    /// Fire action type not relevant to combat (HealingBeam, Parallel, etc.)
    /// or a variant the generator doesn't know about.
    Unknown,
}

/// Per-shot scalar modifiers applied at full charge.
///
/// Extracted from `SWeaponActionFireChargedParams.maxChargeModifier`
/// (a `SWeaponStats` instance). Multipliers are unitless factors over the
/// base ammo / fire-action stats; absolute fields (`pellets`, `burst_shots`)
/// override their counterparts when non-zero.
#[derive(Debug, Clone, Copy)]
pub struct ChargeModifier {
    pub damage_multiplier: f32,
    pub projectile_speed_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub charge_time_multiplier: f32,
    pub heat_generation_multiplier: f32,
    pub ammo_cost_multiplier: f32,
    /// Absolute pellet override (0 = no override; use base pellet count).
    pub pellets: i32,
    /// Absolute burst-shot override (0 = no override).
    pub burst_shots: i32,
}

impl FireActionKind {
    /// Effective rounds per minute, if applicable.
    ///
    /// Returns `None` for beam weapons (continuous, no RPM concept) and
    /// charged weapons (RPM depends on charge time + user behavior).
    pub fn effective_rpm(&self) -> Option<f32> {
        match self {
            Self::Rapid { fire_rate, .. } => Some(*fire_rate as f32),
            Self::Single { fire_rate, .. } => Some(*fire_rate as f32),
            Self::Sequence { effective_rpm, .. } => Some(*effective_rpm),
            Self::Burst { fire_rate, .. } => Some(*fire_rate as f32),
            Self::Charged { .. } | Self::Beam { .. } | Self::Unknown => None,
        }
    }
}

/// Extract a `FireActionKind` from a `SWeaponActionParamsPtr` poly enum variant.
pub(crate) fn extract_fire_action(
    action: &SWeaponActionParamsPtr,
    pools: &DataPools,
) -> FireActionKind {
    match action {
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
            let Some(r) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            FireActionKind::Rapid {
                fire_rate: r.fire_rate,
                heat_per_shot: r.heat_per_shot,
                spin_up: r.spin_up_time,
                spin_down: r.spin_down_time,
            }
        }

        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
            let Some(s) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            FireActionKind::Single {
                fire_rate: s.fire_rate,
                heat_per_shot: s.heat_per_shot,
            }
        }

        SWeaponActionParamsPtr::SWeaponActionSequenceParams(h) => {
            let Some(seq) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            extract_sequence(seq, pools)
        }

        SWeaponActionParamsPtr::SWeaponActionFireBurstParams(h) => {
            let Some(b) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            FireActionKind::Burst {
                fire_rate: b.fire_rate,
                shot_count: b.shot_count,
                cooldown: b.cooldown_time,
            }
        }

        SWeaponActionParamsPtr::SWeaponActionFireChargedParams(h) => {
            let Some(c) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            let max_modifier =
                c.max_charge_modifier
                    .and_then(|h| h.get(pools))
                    .map(|s| ChargeModifier {
                        damage_multiplier: s.damage_multiplier,
                        projectile_speed_multiplier: s.projectile_speed_multiplier,
                        fire_rate_multiplier: s.fire_rate_multiplier,
                        charge_time_multiplier: s.charge_time_multiplier,
                        heat_generation_multiplier: s.heat_generation_multiplier,
                        ammo_cost_multiplier: s.ammo_cost_multiplier,
                        pellets: s.pellets,
                        burst_shots: s.burst_shots,
                    });
            FireActionKind::Charged {
                charge_time: c.charge_time,
                overcharge_time: c.overcharge_time,
                cooldown: c.cooldown_time,
                auto_fire: c.fire_automatically_on_full_charge,
                full_only: c.fire_only_on_full_charge,
                max_modifier,
            }
        }

        SWeaponActionParamsPtr::SWeaponActionFireBeamParams(h) => {
            let Some(beam) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            let dps = beam
                .damage_per_second
                .as_ref()
                .map(|ptr| extract_beam_dps(ptr, pools))
                .unwrap_or_default();
            FireActionKind::Beam {
                dps,
                full_range: beam.full_damage_range,
                zero_range: beam.zero_damage_range,
                heat_per_sec: beam.heat_per_second,
            }
        }

        SWeaponActionParamsPtr::SWeaponActionDynamicConditionParams(h) => {
            // Volt heat-ramp wrapper — unwrap to the default fire action.
            let Some(dc) = h.get(pools) else {
                return FireActionKind::Unknown;
            };
            dc.default_weapon_action
                .as_ref()
                .map(|inner| extract_fire_action(inner, pools))
                .unwrap_or(FireActionKind::Unknown)
        }

        // HealingBeam, Parallel, TractorBeam, etc. — not combat-relevant.
        _ => FireActionKind::Unknown,
    }
}

fn extract_sequence(seq: &SWeaponActionSequenceParams, pools: &DataPools) -> FireActionKind {
    let barrel_count = seq.sequence_entries.len();
    match sequence_timing(seq, pools) {
        Some(t) => FireActionKind::Sequence {
            effective_rpm: t.rpm,
            barrel_count,
        },
        None => FireActionKind::Unknown,
    }
}

/// A sequence fire action's pace.
pub(crate) struct SequenceTiming {
    /// Shots per minute across the whole sequence.
    pub rpm: f32,
    /// The inner actions' mean `heatPerShot` (0 when none carries one).
    pub heat_per_shot: f32,
}

/// The pace of a sequence (client `CWeaponActionSequence::Update`): each
/// entry waits its `delay` (`Seconds`, or `RPM` as 60 / rpm) before the
/// next fires, and each inner action keeps its own cooldown (60 /
/// `fireRate`). One pass of the sequence fires one shot per entry and
/// takes the longer of the summed delays and the slowest inner cooldown,
/// so `rpm = entries / max(Σ delay, max cooldown) × 60` (erkul's rule; the
/// BEHR ballistic repeater: 2 entries at 750 rpm over inner Singles at 500
/// fires 750).
pub(crate) fn sequence_timing(
    seq: &SWeaponActionSequenceParams,
    pools: &DataPools,
) -> Option<SequenceTiming> {
    let entries: Vec<_> = seq
        .sequence_entries
        .iter()
        .filter_map(|e| e.get(pools))
        .collect();
    if entries.is_empty() {
        return None;
    }
    let mut delays = 0.0f32;
    let mut slowest = 0.0f32;
    let (mut heat, mut heated) = (0.0f32, 0u32);
    for e in &entries {
        delays += match e.unit {
            EDelayUnit::Seconds => e.delay.max(0.0),
            _ if e.delay > 0.0 => 60.0 / e.delay,
            _ => 0.0,
        };
        let (rate, hps) = match e.weapon_action.as_ref() {
            Some(SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h)) => h
                .get(pools)
                .map_or((0, 0.0), |s| (s.fire_rate, s.heat_per_shot)),
            Some(SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h)) => h
                .get(pools)
                .map_or((0, 0.0), |r| (r.fire_rate, r.heat_per_shot)),
            Some(SWeaponActionParamsPtr::SWeaponActionFireBurstParams(h)) => {
                h.get(pools).map_or((0, 0.0), |b| (b.fire_rate, 0.0))
            }
            _ => (0, 0.0),
        };
        if rate > 0 {
            slowest = slowest.max(60.0 / rate as f32);
        }
        if hps > 0.0 {
            heat += hps;
            heated += 1;
        }
    }
    let pass = delays.max(slowest);
    if pass <= 0.0 {
        return None;
    }
    Some(SequenceTiming {
        rpm: entries.len() as f32 / pass * 60.0,
        heat_per_shot: if heated > 0 {
            heat / heated as f32
        } else {
            0.0
        },
    })
}

fn extract_beam_dps(ptr: &DamageBasePtr, pools: &DataPools) -> DamageSummary {
    match ptr {
        DamageBasePtr::DamageInfo(h) => {
            let Some(d) = h.get(pools) else {
                return DamageSummary::default();
            };
            DamageSummary {
                physical: d.damage_physical,
                energy: d.damage_energy,
                distortion: d.damage_distortion,
                thermal: d.damage_thermal,
                biochemical: d.damage_biochemical,
                stun: d.damage_stun,
            }
        }
        _ => DamageSummary::default(),
    }
}
