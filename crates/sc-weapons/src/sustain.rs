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
