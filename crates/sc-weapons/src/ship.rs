use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{DataPools, Guid};

use crate::classify::{classify, WeaponCategory};
use crate::damage::{self, DamageSummary};
use crate::fire_action::{self, FireActionKind};
use crate::sustain::{self, SustainKind};

/// A ship-mounted weapon with all data materialized into owned fields.
///
/// Constructed via [`ShipWeapon::try_new`] which resolves all handles from
/// the flat pool. After construction every accessor is a plain field read.
#[derive(Debug, Clone)]
pub struct ShipWeapon {
    /// Record GUID — stable across game patches.
    pub guid: Guid,
    /// DCB record name (e.g. `"GATS_BallisticGatling_S1"`).
    pub record_name: String,
    /// Weapon size (1-12).
    pub size: i32,
    /// Weapon grade (always 1 in 4.7).
    pub grade: i32,
    /// Item type enum.
    pub item_type: EItemType,
    /// Item subtype enum (`Gun`, `Rocket`, `NoseMounted`).
    pub item_sub_type: EItemSubType,
    /// Manufacturer GUID — look up via `ManufacturerRegistry::get`.
    pub manufacturer_guid: Option<Guid>,
    /// All fire actions in declaration order. `fire_actions[0]` is the
    /// primary mode; later entries are alternate modes. Ship weapons in
    /// 4.7 are uniformly single-mode, but the field is plural for symmetry
    /// with [`crate::FpsWeapon`] (where multi-mode is common).
    pub fire_actions: Vec<FireActionKind>,
    /// Sustain model: Heat, Energy, or None.
    pub sustain: SustainKind,
    /// Per-shot damage (direct + explosion, all 6 types). `None` if ammo
    /// could not be resolved (mining lasers, dummies).
    pub damage: Option<DamageSummary>,
    /// Ammo projectile speed in m/s.
    pub ammo_speed: Option<f32>,
    /// Ammo lifetime in seconds (range ≈ speed × lifetime).
    pub ammo_lifetime: Option<f32>,
    /// Pellet count from `SProjectileLauncher` (scatter guns: 8 or 12).
    /// `None` for single-projectile weapons.
    pub pellet_count: Option<i32>,
    /// Magazine size from `SAmmoContainerComponentParams.maxAmmoCount`.
    pub magazine_size: Option<i32>,
    /// Power draw per second when active (from `ItemResourceComponentParams`).
    pub power_draw: Option<f32>,
    /// Weapon health points (from `SHealthComponentParams`).
    pub health: Option<f32>,
    /// Raw entity handle — escape hatch for consumers with `&DataPools`.
    pub entity_handle: Handle<EntityClassDefinition>,
}

impl ShipWeapon {
    /// Attempt to construct a `ShipWeapon` from an entity handle.
    ///
    /// Returns `None` if:
    /// - The entity doesn't have `SCItemWeaponComponentParams`
    /// - The entity doesn't have `SAttachableComponentParams` with an `AttachDef`
    /// - Classification doesn't match [`WeaponCategory::Ship`]
    pub fn try_new(
        handle: Handle<EntityClassDefinition>,
        guid: Guid,
        pools: &DataPools,
        ecd_map: &HashMap<Guid, Handle<EntityClassDefinition>>,
        ammo_map: &HashMap<Guid, Handle<AmmoParams>>,
        record_names: &HashMap<Guid, &str>,
    ) -> Option<Self> {
        let ecd = handle.get(pools)?;

        // Must have weapon component
        let wp = damage::find_component::<SCItemWeaponComponentParams>(ecd, pools)?;

        // Must have attachable with item definition
        let attachable = damage::find_component::<SAttachableComponentParams>(ecd, pools)?;
        let item_def = attachable.attach_def.and_then(|h| h.get(pools))?;

        // Must classify as ship weapon
        if classify(&item_def.r#type, &item_def.sub_type) != Some(WeaponCategory::Ship) {
            return None;
        }

        // Fire actions — extract every declared mode in order.
        let fire_actions: Vec<FireActionKind> = wp
            .fire_actions
            .iter()
            .map(|a| fire_action::extract_fire_action(a, pools))
            .collect();

        // Sustain
        let sustain = sustain::extract_sustain(wp, pools);

        // Ammo
        let ammo = damage::resolve_ammo(ecd, wp, pools, ecd_map, ammo_map);

        // Pellets
        let pellet_count = damage::extract_pellet_count(wp, pools);

        // Magazine
        let magazine_size = damage::extract_magazine_size(ecd, pools);

        // Power draw
        let power_draw = extract_power_draw(ecd, pools);

        // Health
        let health = damage::find_component::<SHealthComponentParams>(ecd, pools)
            .map(|h| h.health);

        // Record name
        let record_name = record_names
            .get(&guid)
            .map(|n| n.strip_prefix("EntityClassDefinition.").unwrap_or(n))
            .unwrap_or("")
            .to_string();

        Some(ShipWeapon {
            guid,
            record_name,
            size: item_def.size,
            grade: item_def.grade,
            item_type: item_def.r#type.clone(),
            item_sub_type: item_def.sub_type.clone(),
            manufacturer_guid: item_def.manufacturer,
            fire_actions,
            sustain,
            damage: ammo.as_ref().map(|a| a.damage),
            ammo_speed: ammo.as_ref().map(|a| a.speed),
            ammo_lifetime: ammo.as_ref().map(|a| a.lifetime),
            pellet_count,
            magazine_size,
            power_draw,
            health,
            entity_handle: handle,
        })
    }

    /// Alpha damage per second assuming maximum burst fire rate, no sustain
    /// limits. Returns `None` for Charged/Burst/Unknown primary actions
    /// (user-paced or requiring a cooldown model) and for weapons where
    /// damage did not resolve.
    ///
    /// - **Rapid/Single**: `(rpm / 60) × alpha_per_shot × pellets`
    /// - **Sequence**: `(effective_rpm / 60) × alpha_per_shot × pellets`
    /// - **Beam**: returns the base `dps` directly (already per-second).
    ///
    /// Pellet count is included when set; otherwise treated as 1.
    pub fn alpha_dps(&self) -> Option<f32> {
        let primary = self.fire_actions.first()?;
        let pellets = self.pellet_count.unwrap_or(1).max(1) as f32;
        match primary {
            FireActionKind::Rapid { fire_rate, .. } | FireActionKind::Single { fire_rate, .. } => {
                let alpha = self.damage.as_ref()?.total();
                Some((*fire_rate as f32 / 60.0) * alpha * pellets)
            }
            FireActionKind::Sequence { effective_rpm, .. } => {
                let alpha = self.damage.as_ref()?.total();
                Some((effective_rpm / 60.0) * alpha * pellets)
            }
            FireActionKind::Beam { dps, .. } => Some(dps.total()),
            FireActionKind::Burst { .. }
            | FireActionKind::Charged { .. }
            | FireActionKind::Unknown => None,
        }
    }

    /// Seconds of continuous primary-mode fire before the weapon overheats
    /// **from a cold start**. Covers Rapid/Single (heat from fire action)
    /// and Sequence (heat from `SWeaponConnectionParams.heatRateOnline`).
    /// `None` for non-heat weapons and non-overheating weapons (scatter
    /// guns with `heat_rate_per_second = 0`).
    pub fn time_to_overheat(&self) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => h.time_to_overheat_cold(),
            _ => None,
        }
    }

    /// Seconds of forced lockout after an overheat event. Direct read of
    /// `HeatModel::overheat_fix_time`. `None` for non-heat weapons.
    pub fn overheat_lockout_time(&self) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => Some(h.overheat_fix_time),
            _ => None,
        }
    }

    /// Long-run sustained damage per second, scaling `alpha_dps` by the
    /// relevant sustain bottleneck:
    ///
    /// - **Heat**: `alpha_dps × duty_cycle_long_run` which uses warm-
    ///   restart `time_to_overheat` (weapons with high
    ///   `temperature_after_overheat_fix` converge to much lower sustained
    ///   rates than their first-burst cold cycle suggests).
    /// - **Energy**: `alpha_per_shot × min(burst_rpm, max_regen_per_sec × 60 / cost_per_bullet)`.
    ///   **Power-starved floor** — uses `max_regen_per_sec` (the hard rate
    ///   cap) which under-reports effective in-game DPS when the ship's
    ///   power network fully supplies `requested_regen_per_sec`. Consumers
    ///   modelling a specific loadout should compute their own sustained
    ///   rate from the raw `EnergyModel` fields and the ship's allocated
    ///   power. Ignores `regeneration_cooldown` amortisation.
    /// - **None** (RPODs): `alpha_dps` (brief single-burst fire).
    ///
    /// Returns `None` when `alpha_dps` is undefined (Charged/Burst/Unknown).
    /// Non-overheating heat weapons (scatter guns) fall through to
    /// `alpha_dps` since they have no sustain cap.
    pub fn sustained_dps(&self) -> Option<f32> {
        let alpha = self.alpha_dps()?;
        match &self.sustain {
            SustainKind::Heat(h) => match h.duty_cycle_long_run() {
                Some(dc) => Some(alpha * dc),
                // heat_rate_per_second == 0 → weapon generates no heat → sustained
                None => Some(alpha),
            },
            SustainKind::Energy(e) => {
                let primary = self.fire_actions.first()?;
                let burst_rpm = match primary {
                    FireActionKind::Rapid { fire_rate, .. }
                    | FireActionKind::Single { fire_rate, .. } => *fire_rate as f32,
                    FireActionKind::Sequence { effective_rpm, .. } => *effective_rpm,
                    // Beam: alpha is already DPS; energy regen caps duration
                    // but not instantaneous rate.
                    FireActionKind::Beam { .. } => return Some(alpha),
                    _ => return None,
                };
                let sustained_rpm = e.sustained_rpm()?.min(burst_rpm);
                let pellets = self.pellet_count.unwrap_or(1).max(1) as f32;
                let per_shot_alpha = self.damage.as_ref()?.total() * pellets;
                Some((sustained_rpm / 60.0) * per_shot_alpha)
            }
            SustainKind::None => Some(alpha),
        }
    }
}

/// Extract power draw from `ItemResourceComponentParams`.
fn extract_power_draw(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<f32> {
    let irc = damage::find_component::<ItemResourceComponentParams>(ecd, pools)?;

    for state_h in &irc.states {
        let state = state_h.get(pools)?;
        for delta in &state.deltas {
            if let ItemResourceDeltaBasePtr::ItemResourceDeltaConsumption(h) = delta {
                let cons = h.get(pools)?;
                let amount = cons.consumption.and_then(|h| h.get(pools))?;
                if matches!(amount.resource, ResourceNetworkResource::Power) {
                    let val = amount.resource_amount_per_second.as_ref().and_then(|u| {
                        match u {
                            SBaseResourceUnitPtr::SStandardResourceUnit(h) => {
                                h.get(pools).map(|v| v.standard_resource_units)
                            }
                            SBaseResourceUnitPtr::SPowerSegmentResourceUnit(h) => {
                                h.get(pools).map(|v| v.units as f32)
                            }
                            _ => None,
                        }
                    });
                    if let Some(v) = val {
                        return Some(v);
                    }
                }
            }
        }
    }
    None
}
