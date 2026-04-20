use sc_extract::generated::*;
use sc_extract::DataPools;

/// Ballistic heat model — extracted from `SWeaponSimplifiedHeatParams`.
///
/// Note: `heat_per_shot` is NOT here — it lives on the fire action
/// (`SWeaponActionFireRapidParams.heatPerShot`). Sustain calculations must
/// combine fire action heat-per-shot with these cooling/overheat params.
#[derive(Debug, Clone, Copy)]
pub struct HeatModel {
    pub overheat_temperature: f32,
    pub cooling_per_second: f32,
    pub overheat_fix_time: f32,
    pub temperature_after_overheat_fix: f32,
    pub time_till_cooling_starts: f32,
}

impl HeatModel {
    /// Seconds of continuous fire from a cold start until heat reaches
    /// `overheat_temperature`. Assumes cooling does not occur while firing
    /// (consistent with the `time_till_cooling_starts` semantics — cooling
    /// begins only after firing stops).
    ///
    /// Returns `None` if `heat_per_shot` or `rpm` are zero (no heat
    /// generated) or `overheat_temperature` is non-positive.
    pub fn time_to_overheat(&self, heat_per_shot: f32, rpm: f32) -> Option<f32> {
        if heat_per_shot <= 0.0 || rpm <= 0.0 || self.overheat_temperature <= 0.0 {
            return None;
        }
        let heat_per_sec = heat_per_shot * (rpm / 60.0);
        Some(self.overheat_temperature / heat_per_sec)
    }

    /// Long-run duty cycle: fraction of wall-clock time spent firing during
    /// a cold-start → overheat → lockout cycle, ignoring the initial-fire
    /// advantage of a cold capacitor. Used to scale `alpha_dps` into a
    /// sustained rate.
    ///
    /// Returns `None` when `time_to_overheat` is undefined.
    pub fn duty_cycle(&self, heat_per_shot: f32, rpm: f32) -> Option<f32> {
        let t_fire = self.time_to_overheat(heat_per_shot, rpm)?;
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
        return SustainKind::Heat(HeatModel {
            overheat_temperature: heat.overheat_temperature,
            cooling_per_second: heat.cooling_per_second,
            overheat_fix_time: heat.overheat_fix_time,
            temperature_after_overheat_fix: heat.temperature_after_overheat_fix,
            time_till_cooling_starts: heat.time_till_cooling_starts,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// GATS_BallisticGatling_S1 values from `D:/Obsidian/Star Citizen/Mechanics/Weapons.md`:
    /// overheat=100, cool/s=44.1, fix_time=2.2s, heat_per_shot=0.32, fire_rate=1600rpm.
    fn gats_s1_heat() -> HeatModel {
        HeatModel {
            overheat_temperature: 100.0,
            cooling_per_second: 44.1,
            overheat_fix_time: 2.2,
            temperature_after_overheat_fix: 50.0,
            time_till_cooling_starts: 1.0,
        }
    }

    #[test]
    fn gats_s1_time_to_overheat() {
        // heat/sec = 0.32 * 1600/60 = 8.533...; 100 / 8.533 = ~11.72 s
        let t = gats_s1_heat().time_to_overheat(0.32, 1600.0).unwrap();
        assert!((t - 11.72).abs() < 0.05, "expected ~11.72s, got {t}");
    }

    #[test]
    fn gats_s1_duty_cycle() {
        // duty = 11.72 / (11.72 + 2.2) = ~0.842
        let d = gats_s1_heat().duty_cycle(0.32, 1600.0).unwrap();
        assert!((d - 0.842).abs() < 0.01, "expected ~0.842, got {d}");
    }

    #[test]
    fn zero_heat_per_shot_returns_none() {
        assert!(gats_s1_heat().time_to_overheat(0.0, 1600.0).is_none());
        assert!(gats_s1_heat().duty_cycle(0.0, 1600.0).is_none());
    }

    #[test]
    fn zero_rpm_returns_none() {
        assert!(gats_s1_heat().time_to_overheat(0.32, 0.0).is_none());
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
