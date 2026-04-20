use sc_extract::generated::*;
use sc_extract::DataPools;

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
    Single {
        fire_rate: i32,
        heat_per_shot: f32,
    },
    /// Multi-barrel round-robin (repeaters, cannons).
    /// `effective_rpm = min(sequence_delay, inner_rpm)` — verified with spviewer.
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
            FireActionKind::Charged {
                charge_time: c.charge_time,
                overcharge_time: c.overcharge_time,
                cooldown: c.cooldown_time,
                auto_fire: c.fire_automatically_on_full_charge,
                full_only: c.fire_only_on_full_charge,
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
    if barrel_count == 0 {
        return FireActionKind::Unknown;
    }

    // Get the first entry's delay and inner action RPM.
    let Some(first_entry) = seq.sequence_entries[0].get(pools) else {
        return FireActionKind::Unknown;
    };

    // Sequence delay — can be in RPM or Seconds.
    let delay_rpm = match first_entry.unit {
        EDelayUnit::RPM => first_entry.delay,
        EDelayUnit::Seconds => {
            if first_entry.delay > 0.0 {
                60.0 / first_entry.delay
            } else {
                // delay=0 Seconds means "as fast as possible" — RPODs use this.
                // The inner action's RPM is the limiter.
                f32::INFINITY
            }
        }
        _ => first_entry.delay, // Unrecognized unit — treat as RPM
    };

    // Inner action's RPM (if it has one).
    let inner_rpm = first_entry
        .weapon_action
        .as_ref()
        .and_then(|inner| match inner {
            SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
                h.get(pools).map(|s| s.fire_rate as f32)
            }
            SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
                h.get(pools).map(|r| r.fire_rate as f32)
            }
            SWeaponActionParamsPtr::SWeaponActionFireBurstParams(h) => {
                // For burst inner actions, effective shots/min = sequence_rate × burst_count
                h.get(pools).map(|b| b.fire_rate as f32)
            }
            _ => None,
        })
        .unwrap_or(delay_rpm);

    // Effective RPM = min(sequence_delay, inner_action_rpm) — verified with spviewer.
    let effective_rpm = delay_rpm.min(inner_rpm);

    FireActionKind::Sequence {
        effective_rpm,
        barrel_count,
    }
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
