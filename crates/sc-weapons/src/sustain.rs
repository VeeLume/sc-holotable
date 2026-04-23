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

    /// Total seconds of firing within a `window_seconds`-long engagement
    /// starting from a cold weapon. Accounts for cold-start burst, first
    /// lockout, and subsequent warm cycles. Weapons that never overheat
    /// (`heat_rate_per_second == 0`) return the full window.
    ///
    /// Cold-start model: `[0, t_cold]` fire, `[t_cold, t_cold + t_lockout]`
    /// lockout, then repeating `[t_warm fire, t_lockout lock]` cycles.
    pub fn fire_time_in_window(&self, window_seconds: f32) -> f32 {
        if window_seconds <= 0.0 {
            return 0.0;
        }
        let Some(t_cold) = self.time_to_overheat_cold() else {
            return window_seconds;
        };
        let t_lockout = self.overheat_fix_time.max(0.0);
        let Some(t_warm) = self.time_to_overheat_warm() else {
            return window_seconds.min(t_cold);
        };

        if window_seconds <= t_cold {
            return window_seconds;
        }
        let after_cold = window_seconds - t_cold;
        if after_cold <= t_lockout {
            return t_cold;
        }
        let after_first_lockout = after_cold - t_lockout;
        let warm_cycle = t_warm + t_lockout;
        if warm_cycle <= 0.0 {
            return t_cold;
        }
        let full_cycles = (after_first_lockout / warm_cycle).floor();
        let remainder = after_first_lockout - full_cycles * warm_cycle;
        let warm_fire = full_cycles * t_warm + remainder.min(t_warm);
        t_cold + warm_fire
    }
}

/// Energy capacitor model — extracted from `SWeaponRegenConsumerParams`.
///
/// # Unit interpretation (validated 2026-04-20 against spviewer reference
/// values for M5A, Attrition-3, Panther, XJ3, PyroBurst, Singe S3)
///
/// - `max_ammo_load` is **shot capacity** (not ammo units divided by cost).
///   Attrition-3: 75 shots; M5A: 25 shots. A full capacitor fires exactly
///   `max_ammo_load` shots at `burst_rpm` before depleting.
/// - `max_regen_per_sec` is **shots per second regen** during the refill
///   phase. Attrition-3: 15 shots/s.
/// - `regeneration_cost_per_bullet` is the **ship-level** ammo-pool cost
///   per shot (drain on the shared ship weapon energy pool), NOT the
///   weapon's own capacitor cost. Irrelevant to the standalone weapon
///   cycle; relevant when modelling ship-level power contention.
/// - `requested_regen_per_sec` / `requested_ammo_load` are what the weapon
///   *asks for* from the ship's power network — caps under starvation,
///   peak when ship is well-powered. Also ship-level, not weapon-level.
///
/// The weapon-level cycle is therefore determined entirely by
/// `max_ammo_load`, `max_regen_per_sec`, `regeneration_cooldown`, and the
/// primary fire action's burst RPM.
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
    /// Seconds of continuous fire at `burst_rpm` before the capacitor
    /// depletes (`max_ammo_load / (burst_rpm / 60)`).
    pub fn burst_seconds(&self, burst_rpm: f32) -> Option<f32> {
        if burst_rpm <= 0.0 || self.max_ammo_load <= 0.0 {
            return None;
        }
        Some(self.max_ammo_load / (burst_rpm / 60.0))
    }

    /// Seconds to refill the capacitor from empty
    /// (`max_ammo_load / max_regen_per_sec`).
    pub fn refill_seconds(&self) -> Option<f32> {
        if self.max_regen_per_sec <= 0.0 || self.max_ammo_load <= 0.0 {
            return None;
        }
        Some(self.max_ammo_load / self.max_regen_per_sec)
    }

    /// Full fire/cooldown/refill cycle time.
    pub fn cycle_seconds(&self, burst_rpm: f32) -> Option<f32> {
        let burst = self.burst_seconds(burst_rpm)?;
        let refill = self.refill_seconds()?;
        Some(burst + self.regeneration_cooldown.max(0.0) + refill)
    }

    /// Long-run (asymptotic) fraction of `burst_dps` the weapon sustains —
    /// unitless `[0.0, 1.0]`. `burst_seconds / cycle_seconds`.
    pub fn asymptotic_dps_fraction(&self, burst_rpm: f32) -> Option<f32> {
        let burst = self.burst_seconds(burst_rpm)?;
        let cycle = self.cycle_seconds(burst_rpm)?;
        if cycle <= 0.0 {
            return None;
        }
        Some(burst / cycle)
    }

    /// Total shots fired in `seconds` starting from a full capacitor,
    /// accounting for fire/cooldown/refill cycles.
    pub fn shots_in_window(&self, burst_rpm: f32, seconds: f32) -> Option<f32> {
        let burst = self.burst_seconds(burst_rpm)?;
        let cycle = self.cycle_seconds(burst_rpm)?;
        if seconds <= 0.0 || cycle <= 0.0 {
            return Some(0.0);
        }
        let shots_per_sec = burst_rpm / 60.0;
        let full_cycles = (seconds / cycle).floor();
        let remainder = seconds - full_cycles * cycle;
        let mut shots = full_cycles * self.max_ammo_load;
        if remainder <= burst {
            shots += remainder * shots_per_sec;
        } else {
            // In cooldown or refill phase — fired the full capacitor, no
            // partial extra shots (refill doesn't mid-fire).
            shots += self.max_ammo_load;
        }
        Some(shots)
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
    fn gats_s1_fire_time_in_window() {
        let h = gats_s1_heat();
        // Cold-start phase — fires for the full T
        assert!((h.fire_time_in_window(5.0) - 5.0).abs() < 0.01);
        assert!((h.fire_time_in_window(11.72) - 11.72).abs() < 0.01);
        // Within first lockout — fire_time stays at t_cold
        assert!((h.fire_time_in_window(13.0) - 11.72).abs() < 0.01);
        assert!((h.fire_time_in_window(13.92) - 11.72).abs() < 0.01);
        // Second burst starts at 13.92, fires 11.72s warm → 25.64 at overheat
        assert!((h.fire_time_in_window(25.64) - 23.44).abs() < 0.05);
        // T=60 → expected 51.2s fire → retention = 85.3%
        let fire_60 = h.fire_time_in_window(60.0);
        assert!((fire_60 - 51.2).abs() < 0.1, "expected ~51.2s, got {fire_60}");
    }

    #[test]
    fn non_overheating_weapon_fires_whole_window() {
        let h = HeatModel {
            overheat_temperature: 100.0,
            cooling_per_second: 20.0,
            overheat_fix_time: 2.0,
            temperature_after_overheat_fix: 0.0,
            time_till_cooling_starts: 0.5,
            heat_rate_per_second: 0.0, // scatter gun
        };
        assert!((h.fire_time_in_window(60.0) - 60.0).abs() < 0.01);
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

    // ------------------------------------------------------------------
    // Energy model — validated against spviewer reference values for
    // 4.7 LIVE. See docs/sc-weapons.md §Planned v2 phase 3 for the full
    // model derivation.
    // ------------------------------------------------------------------

    /// HRST Attrition-3 S3 (LaserRepeater). Live DCB values.
    fn attrition3_s3_energy() -> EnergyModel {
        EnergyModel {
            max_ammo_load: 75.0,          // 75 shots capacity
            max_regen_per_sec: 15.0,      // 15 shots/sec regen
            regeneration_cooldown: 0.7,
            regeneration_cost_per_bullet: 84.0, // ship-level ammo-pool cost, not weapon-cycle
            requested_regen_per_sec: 4853.0,
            requested_ammo_load: 29120.0,
        }
    }

    #[test]
    fn attrition3_cycle_times() {
        let e = attrition3_s3_energy();
        let rpm = 350.0;
        assert!((e.burst_seconds(rpm).unwrap() - 12.857).abs() < 0.01);
        assert!((e.refill_seconds().unwrap() - 5.0).abs() < 0.01);
        assert!((e.cycle_seconds(rpm).unwrap() - 18.557).abs() < 0.01);
    }

    #[test]
    fn attrition3_asymptotic_fraction() {
        // 12.857 / 18.557 = 0.693 → 69.3% of burst
        let f = attrition3_s3_energy().asymptotic_dps_fraction(350.0).unwrap();
        assert!((f - 0.693).abs() < 0.005, "expected ~0.693, got {f}");
    }

    #[test]
    fn attrition3_60s_matches_spviewer() {
        // spviewer reports sustain_60s = 561.1 for Attrition-3 (burst=786.2, alpha=134.8).
        let e = attrition3_s3_energy();
        let rpm = 350.0;
        let alpha = 134.8;
        let shots = e.shots_in_window(rpm, 60.0).unwrap();
        let dps_60 = shots * alpha / 60.0;
        assert!((dps_60 - 561.1).abs() < 2.0, "expected ~561.1 DPS, got {dps_60}");
    }

    #[test]
    fn panther_s3_60s_matches_spviewer() {
        // CF-337 Panther (KLWE_LaserRepeater_S3): spviewer sustain_60s = 306.9.
        let e = EnergyModel {
            max_ammo_load: 75.0,
            max_regen_per_sec: 15.0,
            regeneration_cooldown: 0.2,
            regeneration_cost_per_bullet: 48.5,
            requested_regen_per_sec: 3031.0,
            requested_ammo_load: 18187.0,
        };
        let shots = e.shots_in_window(750.0, 60.0).unwrap();
        let dps_60 = shots * 43.7 / 60.0;
        assert!((dps_60 - 306.9).abs() < 3.0, "expected ~306.9 DPS, got {dps_60}");
    }

    #[test]
    fn m5a_cannon_60s_matches_spviewer() {
        // M5A Cannon (BEHR_LaserCannon_S3): spviewer sustain_60s = 463.4.
        let e = EnergyModel {
            max_ammo_load: 25.0,
            max_regen_per_sec: 3.0,
            regeneration_cooldown: 1.3,
            regeneration_cost_per_bullet: 253.0,
            requested_regen_per_sec: 4220.0,
            requested_ammo_load: 25320.0,
        };
        let shots = e.shots_in_window(100.0, 60.0).unwrap();
        let dps_60 = shots * 410.2 / 60.0;
        assert!((dps_60 - 463.4).abs() < 2.0, "expected ~463.4 DPS, got {dps_60}");
    }

    #[test]
    fn pyroburst_60s_no_depletion_matches_spviewer() {
        // PyroBurst Scattergun (AMRS_ScatterGun_S3): burst_seconds (64.3s)
        // exceeds 60s window, so sustain = burst = 462.
        let e = EnergyModel {
            max_ammo_load: 75.0,
            max_regen_per_sec: 15.0,
            regeneration_cooldown: 0.4,
            regeneration_cost_per_bullet: 440.0,
            requested_regen_per_sec: 2566.7,
            requested_ammo_load: 15400.0,
        };
        // Slow-fire scattergun: 70 rpm × 8 pellets × 49.5 alpha = 462 DPS.
        let burst_rpm = 70.0;
        let alpha_per_shot = 49.5 * 8.0;
        let shots = e.shots_in_window(burst_rpm, 60.0).unwrap();
        let dps_60 = shots * alpha_per_shot / 60.0;
        assert!((dps_60 - 462.0).abs() < 1.0, "expected 462 DPS, got {dps_60}");
    }

    #[test]
    fn zero_regen_returns_none() {
        let e = EnergyModel {
            max_ammo_load: 100.0,
            max_regen_per_sec: 0.0,
            regeneration_cooldown: 0.0,
            regeneration_cost_per_bullet: 1.0,
            requested_regen_per_sec: 0.0,
            requested_ammo_load: 0.0,
        };
        assert!(e.refill_seconds().is_none());
        assert!(e.cycle_seconds(100.0).is_none());
        assert!(e.asymptotic_dps_fraction(100.0).is_none());
    }
}
