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

    // =========================================================
    // Tier 2 — burst stats (rpm-coupled)
    // =========================================================

    /// RPM of the primary fire action, generalised across types:
    /// - Rapid/Single: `fire_rate`
    /// - Sequence: `effective_rpm`
    /// - Charged: `60 / cycle_seconds` under the always-fully-charged model
    ///
    /// `None` for Beam (no per-shot concept), Burst, Unknown.
    pub fn burst_rpm(&self) -> Option<f32> {
        match self.fire_actions.first()? {
            FireActionKind::Rapid { fire_rate, .. }
            | FireActionKind::Single { fire_rate, .. } => Some(*fire_rate as f32),
            FireActionKind::Sequence { effective_rpm, .. } => Some(*effective_rpm),
            FireActionKind::Charged { charge_time, overcharge_time, cooldown, .. } => {
                let cycle = charge_time + overcharge_time + cooldown;
                if cycle > 0.0 { Some(60.0 / cycle) } else { None }
            }
            _ => None,
        }
    }

    /// Peak (burst) damage per second — the weapon's ceiling rate.
    ///
    /// - **Rapid/Single**: `(rpm / 60) × alpha_per_shot × pellets`
    /// - **Sequence**: `(effective_rpm / 60) × alpha_per_shot × pellets`
    /// - **Beam**: returns the base `dps` directly (already per-second).
    /// - **Charged**: `(alpha × damage_multiplier × pellets) / cycle_seconds`
    ///   under the "always fully charged, no user downtime" model. Cycle =
    ///   `charge_time + overcharge_time + cooldown_time`. **Known caveat**:
    ///   spviewer reports lower DPS for some charged weapons (Banu Singe S3:
    ///   spviewer 318.9 vs computed 405) — a ~0.675s per-shot interval
    ///   exists in the game engine that isn't in the fire-action data.
    ///
    /// Returns `None` for Burst/Unknown and when damage did not resolve.
    pub fn burst_dps(&self) -> Option<f32> {
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
            FireActionKind::Charged {
                charge_time,
                overcharge_time,
                cooldown,
                max_modifier,
                ..
            } => {
                let alpha = self.damage.as_ref()?.total();
                let dmg_mult = max_modifier.map(|m| m.damage_multiplier).unwrap_or(1.0);
                let cycle = charge_time + overcharge_time + cooldown;
                if cycle <= 0.0 {
                    return None;
                }
                Some((alpha * dmg_mult * pellets) / cycle)
            }
            FireActionKind::Burst { .. } | FireActionKind::Unknown => None,
        }
    }

    /// Seconds of continuous fire before the first forced stop (overheat
    /// for heat weapons, capacitor depletion for energy). `None` for
    /// non-overheating weapons, Charged (no burst concept — cycle-paced),
    /// Burst, Unknown, Beam, and weapons with no sustain cap (RPODs).
    pub fn burst_seconds(&self) -> Option<f32> {
        let primary = self.fire_actions.first()?;
        match primary {
            FireActionKind::Charged { .. }
            | FireActionKind::Burst { .. }
            | FireActionKind::Unknown => return None,
            _ => {}
        }
        match &self.sustain {
            SustainKind::Heat(h) => h.time_to_overheat_cold(),
            SustainKind::Energy(e) => e.burst_seconds(self.burst_rpm()?),
            SustainKind::None => None,
        }
    }

    /// Total damage delivered during one full burst-until-first-stop phase
    /// (`burst_dps × burst_seconds`). Useful for "how much damage can this
    /// weapon deliver in one trigger-hold?"
    pub fn volley_damage(&self) -> Option<f32> {
        Some(self.burst_dps()? * self.burst_seconds()?)
    }

    /// Seconds from forced stop to ready-to-fire-again state:
    /// - Heat: `overheat_fix_time`
    /// - Energy: `regeneration_cooldown + refill_seconds`
    /// - None/Charged: `None`
    pub fn recovery_seconds(&self) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => Some(h.overheat_fix_time),
            SustainKind::Energy(e) => {
                let refill = e.refill_seconds()?;
                Some(e.regeneration_cooldown + refill)
            }
            SustainKind::None => None,
        }
    }

    /// Full fire-then-recover cycle duration
    /// (`burst_seconds + recovery_seconds`).
    pub fn cycle_seconds(&self) -> Option<f32> {
        Some(self.burst_seconds()? + self.recovery_seconds()?)
    }

    // =========================================================
    // Legacy / transitional accessors
    // =========================================================

    /// Seconds of continuous primary-mode fire before overheat from a cold
    /// start. Alias for heat-specific branch of [`burst_seconds`].
    pub fn time_to_overheat(&self) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => h.time_to_overheat_cold(),
            _ => None,
        }
    }

    /// Seconds of forced lockout after an overheat event (heat weapons
    /// only). Alias for the heat-specific branch of [`recovery_seconds`].
    pub fn overheat_lockout_time(&self) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => Some(h.overheat_fix_time),
            _ => None,
        }
    }

    /// Long-run sustained damage per second (asymptotic) = `burst_dps ×
    /// firing_time_fraction`. For heat weapons uses
    /// `duty_cycle_long_run` (warm-restart); for energy uses
    /// `asymptotic_dps_fraction` (capacitor cycle). Beam returns burst
    /// directly; charged weapons' cycle is self-contained so burst = sust.
    ///
    /// **Energy note**: uses `max_regen_per_sec` — a power-starved floor
    /// when the ship's power network would otherwise supply
    /// `requested_regen_per_sec`. For loadout-aware DPS see
    /// [`effective_dps`](Self::effective_dps).
    pub fn sustained_dps(&self) -> Option<f32> {
        let burst = self.burst_dps()?;
        match &self.sustain {
            SustainKind::Heat(h) => match h.duty_cycle_long_run() {
                Some(dc) => Some(burst * dc),
                None => Some(burst), // non-overheating
            },
            SustainKind::Energy(e) => {
                let primary = self.fire_actions.first()?;
                let burst_rpm = match primary {
                    FireActionKind::Rapid { fire_rate, .. }
                    | FireActionKind::Single { fire_rate, .. } => *fire_rate as f32,
                    FireActionKind::Sequence { effective_rpm, .. } => *effective_rpm,
                    FireActionKind::Beam { .. } => return Some(burst),
                    FireActionKind::Charged { .. } => return Some(burst), // cycle-paced, no extra degradation
                    _ => return None,
                };
                let fraction = e.asymptotic_dps_fraction(burst_rpm)?;
                Some(burst * fraction)
            }
            SustainKind::None => Some(burst),
        }
    }

    // =========================================================
    // Tier 3 — normalised, cross-rpm comparable
    // =========================================================

    /// Total damage delivered during a `window_seconds`-long engagement
    /// starting from full/cold state, accounting for the weapon's sustain
    /// cycles. Core primitive powering [`dps_retention_pct`].
    pub fn damage_in_window(&self, window_seconds: f32) -> Option<f32> {
        if window_seconds <= 0.0 {
            return Some(0.0);
        }
        let primary = self.fire_actions.first()?;
        let pellets = self.pellet_count.unwrap_or(1).max(1) as f32;
        match primary {
            FireActionKind::Rapid { fire_rate, .. } | FireActionKind::Single { fire_rate, .. } => {
                let rpm = *fire_rate as f32;
                let alpha = self.damage.as_ref()?.total();
                let alpha_per_shot = alpha * pellets;
                self.damage_in_window_continuous(window_seconds, rpm, alpha_per_shot)
            }
            FireActionKind::Sequence { effective_rpm, .. } => {
                let alpha = self.damage.as_ref()?.total();
                let alpha_per_shot = alpha * pellets;
                self.damage_in_window_continuous(window_seconds, *effective_rpm, alpha_per_shot)
            }
            FireActionKind::Beam { dps, .. } => Some(dps.total() * window_seconds),
            FireActionKind::Charged {
                charge_time,
                overcharge_time,
                cooldown,
                max_modifier,
                ..
            } => {
                let alpha = self.damage.as_ref()?.total();
                let dmg_mult = max_modifier.map(|m| m.damage_multiplier).unwrap_or(1.0);
                let cycle = charge_time + overcharge_time + cooldown;
                if cycle <= 0.0 {
                    return None;
                }
                // Shots fire at t = cycle, 2×cycle, ... (auto-fire at full charge)
                let shots = (window_seconds / cycle).floor();
                Some(shots * alpha * dmg_mult * pellets)
            }
            FireActionKind::Burst { .. } | FireActionKind::Unknown => None,
        }
    }

    fn damage_in_window_continuous(
        &self,
        window_seconds: f32,
        rpm: f32,
        alpha_per_shot: f32,
    ) -> Option<f32> {
        match &self.sustain {
            SustainKind::Heat(h) => {
                let fire_time = h.fire_time_in_window(window_seconds);
                let shots_per_sec = rpm / 60.0;
                Some(fire_time * shots_per_sec * alpha_per_shot)
            }
            SustainKind::Energy(e) => {
                let shots = e.shots_in_window(rpm, window_seconds)?;
                Some(shots * alpha_per_shot)
            }
            SustainKind::None => {
                let shots_per_sec = rpm / 60.0;
                Some(window_seconds * shots_per_sec * alpha_per_shot)
            }
        }
    }

    /// Percentage of peak (burst) DPS retained averaged over a
    /// `window_seconds`-long engagement from full/cold start. Cross-rpm
    /// comparable — two weapons with very different sizes land on the same
    /// 0-100 scale. Capped at 100%.
    pub fn dps_retention_pct(&self, window_seconds: f32) -> Option<f32> {
        let burst = self.burst_dps()?;
        if burst <= 0.0 || window_seconds <= 0.0 {
            return None;
        }
        let damage = self.damage_in_window(window_seconds)?;
        let avg_dps = damage / window_seconds;
        Some((avg_dps / burst * 100.0).min(100.0))
    }

    /// Percentage of wall-clock time the weapon is actually firing in
    /// long-run steady state. Equivalent to `firing_duration / cycle_duration`.
    /// `100%` for non-overheating / always-ready weapons.
    pub fn firing_time_pct(&self) -> Option<f32> {
        let primary = self.fire_actions.first()?;
        match primary {
            FireActionKind::Burst { .. } | FireActionKind::Unknown => return None,
            _ => {}
        }
        match &self.sustain {
            SustainKind::Heat(h) => match h.duty_cycle_long_run() {
                Some(f) => Some(f * 100.0),
                None => Some(100.0), // non-overheating
            },
            SustainKind::Energy(e) => {
                let rpm = self.burst_rpm()?;
                e.asymptotic_dps_fraction(rpm).map(|f| f * 100.0)
            }
            SustainKind::None => Some(100.0),
        }
    }

    /// Long-run (asymptotic) DPS as a percentage of `burst_dps`. This is
    /// the floor of `dps_retention_pct` as the window grows.
    pub fn long_run_dps_pct(&self) -> Option<f32> {
        let burst = self.burst_dps()?;
        if burst <= 0.0 {
            return None;
        }
        let sustained = self.sustained_dps()?;
        Some((sustained / burst * 100.0).min(100.0))
    }

    /// Thermal efficiency — damage delivered per unit of heat generated.
    /// Cross-rpm comparable: strips fire rate entirely. `burst_dps /
    /// heat_rate_per_second`. `None` for non-heat weapons.
    pub fn thermal_efficiency(&self) -> Option<f32> {
        let SustainKind::Heat(h) = &self.sustain else {
            return None;
        };
        if h.heat_rate_per_second <= 0.0 {
            return None;
        }
        let burst = self.burst_dps()?;
        Some(burst / h.heat_rate_per_second)
    }

    /// Power efficiency — burst DPS per unit of power draw. Exposes the
    /// ship-loadout tradeoff: "how much damage do I get per power pip?"
    /// `None` when `power_draw` is unknown or zero.
    pub fn power_efficiency(&self) -> Option<f32> {
        let power = self.power_draw?;
        if power <= 0.0 {
            return None;
        }
        Some(self.burst_dps()? / power)
    }

    // =========================================================
    // Single composite — loadout-aware DPS score
    // =========================================================

    /// Effective DPS under loadout constraints — the canonical default
    /// sort key for weapon comparison.
    ///
    /// `effective_dps = burst_dps × power_factor × sustain_factor` where:
    /// - `power_factor = min(1.0, ctx.power_per_slot / power_draw)` when
    ///   `ctx.power_per_slot` is set (ballistic weapons with token
    ///   `power_draw = 0.1` are effectively unconstrained; energy and
    ///   mass-driver weapons throttle linearly when starved).
    /// - `sustain_factor = dps_retention_pct(window_seconds) / 100`.
    ///
    /// Both `window_seconds` and `power_per_slot` are caller-provided
    /// with no defaults — consumer apps wire them to UI state / ship
    /// loadouts.
    pub fn effective_dps(&self, ctx: &LoadoutContext) -> Option<f32> {
        let burst = self.burst_dps()?;
        let power_factor = match ctx.power_per_slot {
            Some(pps) => match self.power_draw {
                Some(pd) if pd > 0.0 => (pps / pd).clamp(0.0, 1.0),
                _ => 1.0,
            },
            None => 1.0,
        };
        let sustain_factor = self
            .dps_retention_pct(ctx.window_seconds)
            .map(|p| p / 100.0)
            .unwrap_or(1.0);
        Some(burst * power_factor * sustain_factor)
    }
}

/// Loadout context for computing [`ShipWeapon::effective_dps`]. Both
/// fields are caller-provided with no baked-in defaults — consumer apps
/// wire them to runtime state (UI engagement-window slider, ship power
/// plant / pip allocation).
#[derive(Debug, Clone, Copy)]
pub struct LoadoutContext {
    /// Engagement window in seconds. Typical values: 10-15s for dogfight,
    /// 30-60s for typical skirmish, 120s+ for capital-ship engagements.
    pub window_seconds: f32,
    /// Power units allocated to this weapon's slot (from the ship's
    /// shared power pool, in the same units as `ShipWeapon::power_draw`).
    /// `None` = peak performance (unconstrained).
    pub power_per_slot: Option<f32>,
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
