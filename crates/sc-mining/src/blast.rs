//! The explosion a rock makes when it fractures.
//!
//! Every fracture explodes, a clean one included: once the child rocks have
//! spawned, the client's `SvDestroy` queues an explosion at the rock's centre
//! (client code, LIVE 4.10). Only a fully extracted rock makes none. How big
//! it is depends on the rock's volume and composition, the shatter-damage
//! modifiers on it, and how full the danger pool is at that moment, so an
//! overcharge grows it a lot. The mining HUD's striped zone on the distance
//! gauge is [`FractureBlast::radius_max`] with the current danger pool.

use sc_items_ship_weapons::Damage;
use serde::{Deserialize, Serialize};

use crate::{Combined, Rock, Ruleset};

/// `mineableExplosionParams.defaultExplosionParams`: the explosion a rock of
/// [`default_volume`](Ruleset::default_volume) with a strength factor of 1
/// makes. [`FractureBlast`] scales it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct BlastParams {
    /// `minRadius`, m.
    pub min_radius: f32,
    /// `maxRadius`, m.
    pub max_radius: f32,
    /// `pressure`: the physics push.
    pub pressure: f32,
    /// `damage` (`DamageInfo`).
    pub damage: Damage,
}

/// One fracture's explosion, as the client queues it (`FUN_144511510`):
///
/// ```text
/// u = V / defaultVolume
/// F = max(0, explosionMultiplier * shatterFactor + shatterAdd + 1 + dangerPoolFactor * dangerPool)
/// radii    = cbrt(u) * F * {minRadius, maxRadius}
/// damage   = u * F * damage
/// pressure = u * F * pressure
/// ```
///
/// Radii are measured from the rock's centre (the explosion's origin), not
/// from its surface. Elements with a negative explosion multiplier (copper,
/// tin, aluminium, …) can pull `F` to 0: such a rock's clean fracture makes
/// no explosion at all. [`damage_at`](Self::damage_at) and
/// [`pressure_at`](Self::pressure_at) give what reaches a target at a
/// distance.
///
/// ```no_run
/// # fn demo(ruleset: &sc_mining::Ruleset, rock: &sc_mining::Rock) {
/// use sc_mining::Combined;
/// let clean = rock.fracture_blast(ruleset, Combined::NONE, 0.0);
/// let overcharged = rock.fracture_blast(ruleset, Combined::NONE, 1.0);
/// println!(
///     "a clean crack reaches {:.0} m, a full overcharge {:.0} m; at 30 m: {:.1} physical",
///     clean.radius_max,
///     overcharged.radius_max,
///     clean.damage_at(30.0).physical
/// );
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FractureBlast {
    /// The radius of full effect, m.
    pub radius_min: f32,
    /// The outer edge, m: what the HUD's striped zone shows.
    pub radius_max: f32,
    pub damage: Damage,
    pub pressure: f32,
}

impl FractureBlast {
    /// The explosion of a rock of `volume` m³ and `explosion_multiplier`
    /// ([`Rock::explosion_multiplier`]) fracturing with the danger pool at
    /// `danger_pool` (0..1; 0 is a clean fracture). `shatter` is the
    /// setup's shatter-damage modifier ([`ModifiedRock::shatter_damage`](crate::ModifiedRock::shatter_damage),
    /// [`Combined::NONE`] without one).
    pub fn new(
        ruleset: &Ruleset,
        volume: f32,
        explosion_multiplier: f32,
        shatter: Combined,
        danger_pool: f32,
    ) -> Self {
        let u = if ruleset.default_volume > 0.0 {
            volume / ruleset.default_volume
        } else {
            0.0
        };
        let factor = (shatter.apply(explosion_multiplier)
            + 1.0
            + ruleset.danger_pool_factor * danger_pool.clamp(0.0, 1.0))
        .max(0.0);
        let size = u.cbrt() * factor;
        let strength = u * factor;
        let params = &ruleset.blast;
        let d = params.damage;
        Self {
            radius_min: size * params.min_radius,
            radius_max: size * params.max_radius,
            damage: Damage {
                physical: d.physical * strength,
                energy: d.energy * strength,
                distortion: d.distortion * strength,
                thermal: d.thermal * strength,
                biochemical: d.biochemical * strength,
                stun: d.stun * strength,
            },
            pressure: params.pressure * strength,
        }
    }

    /// Whether the fracture makes an explosion at all.
    pub fn is_none(&self) -> bool {
        self.radius_max <= 0.0
    }

    /// The share of [`damage`](Self::damage) a target takes at `distance`
    /// m from the rock's centre to the point the blast hits on the target's
    /// surface: 1 inside [`radius_min`](Self::radius_min), then falling with
    /// the squared distance, `1 - clamp((d² - min²) / (max² - min²), 0, 1)`,
    /// to 0 at [`radius_max`](Self::radius_max) (client 4.10, the game
    /// rules' explosion damage handling).
    ///
    /// This is the falloff for items and other non-vehicle entities. **A
    /// vehicle's own hull takes no distance falloff**: anywhere the explosion
    /// reaches it, the hull takes the full [`damage`](Self::damage) times its
    /// exposure; parts mounted on the ship are separate entities and do fall
    /// off. Exposure (how much of the target is occluded from the blast) is
    /// not modelled here and taken as 1.
    pub fn damage_falloff(&self, distance: f32) -> f32 {
        let (min, max) = (self.radius_min, self.radius_max);
        let d2 = distance * distance;
        if max <= min {
            return if d2 < max * max { 1.0 } else { 0.0 };
        }
        1.0 - ((d2 - min * min) / (max * max - min * min)).clamp(0.0, 1.0)
    }

    /// [`damage`](Self::damage) times [`damage_falloff`](Self::damage_falloff)
    /// at `distance`: what an unoccluded item or other non-vehicle target
    /// takes. Not for a ship's hull, which takes the full `damage` in reach.
    pub fn damage_at(&self, distance: f32) -> Damage {
        let k = self.damage_falloff(distance);
        let d = self.damage;
        Damage {
            physical: d.physical * k,
            energy: d.energy * k,
            distortion: d.distortion * k,
            thermal: d.thermal * k,
            biochemical: d.biochemical * k,
            stun: d.stun * k,
        }
    }

    /// The push at `distance`: [`pressure`](Self::pressure) times
    /// `1 - clamp((d - min) / (max - min), 0, 1)`, linear in the distance
    /// (client 4.10).
    pub fn pressure_at(&self, distance: f32) -> f32 {
        let (min, max) = (self.radius_min, self.radius_max);
        if max <= min {
            return if distance < max { self.pressure } else { 0.0 };
        }
        self.pressure * (1.0 - ((distance - min) / (max - min)).clamp(0.0, 1.0))
    }
}

impl Rock {
    /// This rock's [`FractureBlast`] with the danger pool at `danger_pool`
    /// and the setup's shatter-damage modifier `shatter`.
    pub fn fracture_blast(
        &self,
        ruleset: &Ruleset,
        shatter: Combined,
        danger_pool: f32,
    ) -> FractureBlast {
        FractureBlast::new(
            ruleset,
            self.volume,
            self.explosion_multiplier,
            shatter,
            danger_pool,
        )
    }
}
