use sc_extract::generated::*;
use sc_extract::DataPools;

/// Ballistic heat model — extracted from `SWeaponSimplifiedHeatParams` +
/// the weapon's primary fire action / `SWeaponConnectionParams`.
///
/// `heat_rate_per_second` unifies two data paths observed in 4.7 LIVE:
/// - **Rapid / Single** fire actions carry `heatPerShot`; heat rate =
///   `heat_per_shot × rpm / 60`.
/// - **Sequence** fire actions carry no per-shot heat; heat rate is read
///   directly from `SWeaponConnectionParams.heatRateOnline`.
///
/// Weapons with no heat source at all (ship ballistic scatter guns in 4.7)
/// have `heat_rate_per_second == 0.0` and never overheat — sustain methods
/// return `None`.
#[derive(Debug, Clone, Copy)]
pub struct HeatModel {
    pub overheat_temperature: f32,
    pub cooling_per_second: f32,
    pub overheat_fix_time: f32,
    pub temperature_after_overheat_fix: f32,
    pub time_till_cooling_starts: f32,
    /// Heat generated per second of continuous primary-mode fire.
    /// Unified across Rapid/Single/Sequence weapons; see struct docs.
    pub heat_rate_per_second: f32,
}

impl HeatModel {
    /// Seconds of continuous fire from a **cold start** until heat reaches
    /// `overheat_temperature`. Assumes cooling does not occur while firing
    /// (consistent with the `time_till_cooling_starts` semantics — cooling
    /// begins only after firing stops).
    ///
    /// Returns `None` if `heat_rate_per_second == 0.0` (non-overheating
    /// weapon) or `overheat_temperature <= 0.0`.
    pub fn time_to_overheat_cold(&self) -> Option<f32> {
        if self.heat_rate_per_second <= 0.0 || self.overheat_temperature <= 0.0 {
            return None;
        }
        Some(self.overheat_temperature / self.heat_rate_per_second)
    }

    /// Seconds of continuous fire from a **warm restart** (immediately
    /// after an `overheat_fix`) until the next overheat. Uses the reduced
    /// headroom `overheat_temperature - temperature_after_overheat_fix`.
    ///
    /// Critical for long-run duty-cycle calculations on weapons with
    /// non-zero `temperature_after_overheat_fix` (e.g. BEHR Revenant S6
    /// Gatling has `after_fix = 99`, leaving only 1 unit of headroom per
    /// warm cycle). For cold-reset weapons (`after_fix = 0`) this equals
    /// [`time_to_overheat_cold`](Self::time_to_overheat_cold).
    pub fn time_to_overheat_warm(&self) -> Option<f32> {
        if self.heat_rate_per_second <= 0.0 {
            return None;
        }
        let headroom = self.overheat_temperature - self.temperature_after_overheat_fix;
        if headroom <= 0.0 {
            return None;
        }
        Some(headroom / self.heat_rate_per_second)
    }

    /// Long-run (asymptotic) duty cycle: fraction of wall-clock time spent
    /// firing across repeated warm-restart cycles. Uses
    /// [`time_to_overheat_warm`](Self::time_to_overheat_warm) because the
    /// cold-start advantage is a one-time bonus that vanishes in the limit.
    ///
    /// Returns `None` when the weapon doesn't overheat.
    pub fn duty_cycle_long_run(&self) -> Option<f32> {
        let t_fire = self.time_to_overheat_warm()?;
        let t_lockout = self.overheat_fix_time.max(0.0);
        if t_fire + t_lockout <= 0.0 {
            return None;
        }
        Some(t_fire / (t_fire + t_lockout))
    }
}

/// Energy capacitor model — extracted from `SWeaponRegenConsumerParams`.
///
/// The actual regen rate depends on ship-level power allocation (shared
/// weapon energy pool). These are the weapon's own params; the ship
/// determines how much power it actually receives.
#[derive(Debug, Clone, Copy)]
pub struct EnergyModel {
    pub max_ammo_load: f32,
    pub max_regen_per_sec: f32,
    pub regeneration_cooldown: f32,
    pub regeneration_cost_per_bullet: f32,
    pub requested_regen_per_sec: f32,
    pub requested_ammo_load: f32,
}

impl EnergyModel {
    /// Shots available from a full capacitor before regen becomes the
    /// bottleneck (`max_ammo_load / regeneration_cost_per_bullet`).
    ///
    /// Note: in 4.7 LIVE, several weapons report `cost_per_bullet > max_load`
    /// (Singe S1: 300 vs 25) — the ammo-unit scale is not a literal 1-shot
    /// unit and the in-game behaviour presumably uses fractional
    /// accumulation. This method returns the raw ratio; consumers should
    /// treat `< 1.0` results as "regen-gated from the first shot."
    ///
    /// Returns `None` when `cost_per_bullet <= 0`.
    pub fn burst_shot_count(&self) -> Option<f32> {
        if self.regeneration_cost_per_bullet <= 0.0 {
            return None;
        }
        Some(self.max_ammo_load / self.regeneration_cost_per_bullet)
    }

    /// Sustained shots-per-minute allowed by the regen loop, ignoring
    /// `regeneration_cooldown` amortisation.
    /// (`max_regen_per_sec / cost_per_bullet × 60`).
    ///
    /// Returns `None` when `cost_per_bullet <= 0`.
    pub fn sustained_rpm(&self) -> Option<f32> {
        if self.regeneration_cost_per_bullet <= 0.0 {
            return None;
        }
        Some(self.max_regen_per_sec / self.regeneration_cost_per_bullet * 60.0)
    }
}

/// Sustain model for a ship weapon.
///
/// Ship weapons have exactly one of Heat (65 weapons), Energy (108), or
/// None (9 RPODs). No weapon has both in 4.7.
#[derive(Debug, Clone)]
pub enum SustainKind {
    Heat(HeatModel),
    Energy(EnergyModel),
    None,
}

/// Extract the sustain model from weapon component params.
pub(crate) fn extract_sustain(
    weapon_params: &SCItemWeaponComponentParams,
    pools: &DataPools,
) -> SustainKind {
    let connection = weapon_params
        .connection_params
        .and_then(|h| h.get(pools));

    // Check heat model first
    if let Some(heat) = connection
        .and_then(|c| c.simplified_heat_params)
        .and_then(|h| h.get(pools))
    {
        // Compute the unified heat_rate_per_second. Try the fire action
        // first (Rapid/Single carry heat_per_shot), fall back to the
        // connection_params.heat_rate_online (populated for Sequence).
        let heat_rate_per_second = heat_rate_from_fire_action(weapon_params, pools)
            .or_else(|| connection.map(|c| c.heat_rate_online))
            .unwrap_or(0.0);

        return SustainKind::Heat(HeatModel {
            overheat_temperature: heat.overheat_temperature,
            cooling_per_second: heat.cooling_per_second,
            overheat_fix_time: heat.overheat_fix_time,
            temperature_after_overheat_fix: heat.temperature_after_overheat_fix,
            time_till_cooling_starts: heat.time_till_cooling_starts,
            heat_rate_per_second,
        });
    }

    // Check energy model
    if let Some(regen) = weapon_params
        .weapon_regen_consumer_params
        .and_then(|h| h.get(pools))
    {
        return SustainKind::Energy(EnergyModel {
            max_ammo_load: regen.max_ammo_load,
            max_regen_per_sec: regen.max_regen_per_sec,
            regeneration_cooldown: regen.regeneration_cooldown,
            regeneration_cost_per_bullet: regen.regeneration_cost_per_bullet,
            requested_regen_per_sec: regen.requested_regen_per_sec,
            requested_ammo_load: regen.requested_ammo_load,
        });
    }

    SustainKind::None
}

/// Compute heat-per-second from the primary fire action, if it carries a
/// `heat_per_shot` field with meaningful value. Returns `None` for
/// Sequence variants (no per-shot heat) or Rapid/Single with zero hps
/// (scatter guns) — the caller falls back to `heat_rate_online`.
fn heat_rate_from_fire_action(
    wp: &SCItemWeaponComponentParams,
    pools: &DataPools,
) -> Option<f32> {
    let action = wp.fire_actions.first()?;
    match action {
        SWeaponActionParamsPtr::SWeaponActionFireRapidParams(h) => {
            let r = h.get(pools)?;
            if r.heat_per_shot > 0.0 {
                Some(r.heat_per_shot * (r.fire_rate as f32 / 60.0))
            } else {
                None
            }
        }
        SWeaponActionParamsPtr::SWeaponActionFireSingleParams(h) => {
            let s = h.get(pools)?;
            if s.heat_per_shot > 0.0 {
                Some(s.heat_per_shot * (s.fire_rate as f32 / 60.0))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GATS_BallisticGatling_S1 values from live DCB + `Weapons.md`:
    /// overheat=100, cool/s=44.1, fix_time=2.2s, heat_per_shot=0.32,
    /// fire_rate=1600rpm, temperature_after_overheat_fix=0 (cold reset).
    /// heat_rate_per_second = 0.32 * 1600/60 = 8.533.
    fn gats_s1_heat() -> HeatModel {
        HeatModel {
            overheat_temperature: 100.0,
            cooling_per_second: 44.1,
            overheat_fix_time: 2.2,
            temperature_after_overheat_fix: 0.0,
            time_till_cooling_starts: 0.5,
            heat_rate_per_second: 8.533,
        }
    }

    #[test]
    fn gats_s1_time_to_overheat_cold() {
        // 100 / 8.533 = ~11.72 s
        let t = gats_s1_heat().time_to_overheat_cold().unwrap();
        assert!((t - 11.72).abs() < 0.05, "expected ~11.72s, got {t}");
    }

    #[test]
    fn gats_s1_warm_equals_cold_when_after_fix_is_zero() {
        // temperature_after_overheat_fix = 0 → full headroom on warm restart.
        let h = gats_s1_heat();
        let cold = h.time_to_overheat_cold().unwrap();
        let warm = h.time_to_overheat_warm().unwrap();
        assert!((cold - warm).abs() < 0.01, "expected equal, got cold={cold} warm={warm}");
    }

    #[test]
    fn gats_s1_duty_cycle_long_run() {
        // duty = 11.72 / (11.72 + 2.2) = ~0.842
        let d = gats_s1_heat().duty_cycle_long_run().unwrap();
        assert!((d - 0.842).abs() < 0.01, "expected ~0.842, got {d}");
    }

    /// BEHR Revenant S6 Gatling: after_fix=99 → almost no warm headroom.
    /// heat_rate = 2.37 * 900/60 = 35.55.
    #[test]
    fn behr_revenant_s6_warm_cycle_collapses() {
        let h = HeatModel {
            overheat_temperature: 100.0,
            cooling_per_second: 34.6,
            overheat_fix_time: 3.3,
            temperature_after_overheat_fix: 99.0,  // only 1 unit of headroom!
            time_till_cooling_starts: 0.6,
            heat_rate_per_second: 35.55,
        };
        let cold = h.time_to_overheat_cold().unwrap();
        let warm = h.time_to_overheat_warm().unwrap();
        assert!((cold - 2.81).abs() < 0.02, "expected cold~2.81s, got {cold}");
        assert!((warm - 0.028).abs() < 0.005, "expected warm~0.028s, got {warm}");
        // Long-run duty is dominated by lockout: 0.028 / (0.028 + 3.3) = 0.84%.
        let duty = h.duty_cycle_long_run().unwrap();
        assert!(duty < 0.01, "expected <1% sustained duty, got {duty}");
    }

    #[test]
    fn zero_heat_rate_returns_none() {
        // Ship ballistic scatter guns: heat_rate_per_second = 0 → no overheat.
        let h = HeatModel {
            overheat_temperature: 100.0,
            cooling_per_second: 24.2,
            overheat_fix_time: 0.87,
            temperature_after_overheat_fix: 0.0,
            time_till_cooling_starts: 0.182,
            heat_rate_per_second: 0.0,
        };
        assert!(h.time_to_overheat_cold().is_none());
        assert!(h.time_to_overheat_warm().is_none());
        assert!(h.duty_cycle_long_run().is_none());
    }

    /// HRST_LaserRepeater_S3 values from Weapons.md:
    /// cooldown=0.7s, cost/bullet=84.0, max_load=75, max_regen/s=15.0.
    fn hrst_s3_energy() -> EnergyModel {
        EnergyModel {
            max_ammo_load: 75.0,
            max_regen_per_sec: 15.0,
            regeneration_cooldown: 0.7,
            regeneration_cost_per_bullet: 84.0,
            requested_regen_per_sec: 15.0,
            requested_ammo_load: 75.0,
        }
    }

    #[test]
    fn hrst_s3_burst_shot_count() {
        // 75 / 84 = 0.893 — below one "shot"; fractional-accumulation weapon.
        let n = hrst_s3_energy().burst_shot_count().unwrap();
        assert!((n - 0.893).abs() < 0.01, "expected ~0.893, got {n}");
    }

    #[test]
    fn hrst_s3_sustained_rpm() {
        // 15 / 84 * 60 = 10.71 rpm
        let r = hrst_s3_energy().sustained_rpm().unwrap();
        assert!((r - 10.71).abs() < 0.05, "expected ~10.71 rpm, got {r}");
    }

    #[test]
    fn zero_cost_returns_none() {
        let e = EnergyModel {
            max_ammo_load: 100.0,
            max_regen_per_sec: 10.0,
            regeneration_cooldown: 0.0,
            regeneration_cost_per_bullet: 0.0,
            requested_regen_per_sec: 0.0,
            requested_ammo_load: 0.0,
        };
        assert!(e.burst_shot_count().is_none());
        assert!(e.sustained_rpm().is_none());
    }
}
