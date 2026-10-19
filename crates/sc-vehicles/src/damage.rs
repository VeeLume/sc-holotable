//! A projectile hit on a loadout, as the client resolves it (LIVE 4.10.197,
//! `CVehicle::OnHit`): shield face, armour, hull part, penetration into the
//! components behind it.

use std::collections::HashMap;

use sc_extract::generated::{EItemSubType, EItemType};
use sc_extract::{Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_ship_components::{ComponentHealth, PerDamageType, component_health};
use serde::{Deserialize, Serialize};

use crate::raycast::{dot, sub};
use crate::{Defense, HullGeometry, Raycaster, Vehicle, apply};

/// Every entity's `SHealthComponentParams`, keyed by
/// `EntityClassDefinition` GUID: what a penetrating hit can damage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentHealths {
    by_guid: HashMap<Guid, ComponentHealth>,
}

impl ComponentHealths {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let by_guid = store
            .records
            .multi_feature
            .entity_class_definition
            .iter()
            .filter_map(|(&g, h)| Some((g, component_health(h.get(&store.pools)?, &store.pools)?)))
            .collect();
        Self { by_guid }
    }
}

impl RecordCollection for ComponentHealths {
    type Item = ComponentHealth;

    fn get(&self, guid: &Guid) -> Option<&ComponentHealth> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ComponentHealth)> + '_ {
        self.by_guid.iter()
    }
}

/// The client's damage constants (console variables, shipped defaults; a
/// server can override them).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DamageRules {
    /// `v_armor_max_effectiveness`: a hull part takes `1 − this × armour
    /// health ratio` of what gets past the armour.
    pub armor_max_effectiveness: f32,
}

impl Default for DamageRules {
    fn default() -> Self {
        Self {
            armor_max_effectiveness: 0.75,
        }
    }
}

/// One projectile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shot {
    /// Where it comes from and where it flies, ship frame.
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    /// Damage per type (`sc_weapons::ShipWeapon::damage`).
    pub damage: PerDamageType<f32>,
    /// `penetrationParams`: `basePenetrationDistance` and the near / far
    /// radius, m; `None` for ammo without.
    pub penetration: Option<(f32, [f32; 2])>,
}

/// The ship's state when the shot lands.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// The armour's current HP over its maximum (1 = intact).
    pub armor_ratio: f32,
    /// The struck shield face's HP over its maximum; 0 = down.
    pub shield_ratio: f32,
    /// Where each shield resistance sits between its min and max (the
    /// client lerps by the emitter's power; INFERRED); 1 = max.
    pub shield_resistance_ratio: f32,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            armor_ratio: 1.0,
            shield_ratio: 1.0,
            shield_resistance_ratio: 1.0,
        }
    }
}

/// What one shot does.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HitReport {
    /// The hull triangle the shot reaches: point and part (index into
    /// [`HullGeometry::collision`] and its name); `None` for a miss.
    pub point: Option<[f32; 3]>,
    pub part: Option<(usize, String)>,
    /// Shield: what the face takes, and what passes through to the hull
    /// (the projectile's second collision). Without a shield the
    /// pass-through is the full damage.
    pub shield_taken: f32,
    pub passed: PerDamageType<f32>,
    /// After the armour (deflection, then its damage multiplier).
    pub after_armor: PerDamageType<f32>,
    /// What the armour item loses.
    pub armor_wear: f32,
    /// What the struck hull part takes.
    pub hull_damage: f32,
    /// Penetration depth, m.
    pub depth: f32,
    /// Components the penetration reaches, with their damage.
    pub components: Vec<ComponentHit>,
}

/// A component a penetration reaches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentHit {
    /// Index into [`HullGeometry::placements`].
    pub placement: usize,
    pub item: Guid,
    /// Distance along the shot from the impact point, m.
    pub depth: f32,
    /// Damage after falloff, multipliers and the item's resistances.
    pub damage: f32,
}

/// Inputs every hit on one loadout shares.
pub struct Target<'a> {
    pub vehicle: &'a Vehicle,
    pub geometry: &'a HullGeometry,
    /// Built from [`HullGeometry::collision`].
    pub raycaster: &'a Raycaster,
    pub defense: &'a Defense,
    pub items: &'a Items,
    pub healths: &'a ComponentHealths,
    pub rules: DamageRules,
}

impl Target<'_> {
    /// Resolve `shot` (client `CVehicle::OnHit`, LIVE 4.10.197):
    ///
    /// 1. **Shield** (face up): the face takes `D(1−R)A`, `D(1−R)(1−A)`
    ///    passes to the hull, `D·R` is lost (`ProcessHitOnShieldFaces`),
    ///    with `A` between its min and max by the face's HP.
    /// 2. **Armour**, per type: a hit at or below `deflection × h` does
    ///    nothing; otherwise × `damageMultiplier` (while `h > 0`). The
    ///    armour wears by the physical + energy left, after its own
    ///    resistances.
    /// 3. **Hull part**: the part the shot reaches takes the non-distortion
    ///    damage × (1 − 0.75 h) (`v_armor_max_effectiveness`).
    /// 4. **Penetration** (projectiles with `penetrationParams`): per type,
    ///    depth = `basePenetrationDistance × (1 − absorption × reduction ×
    ///    h)` if that type got past the armour; a cylinder from the impact
    ///    point along the shot reaches every item with health (doors
    ///    excepted, no occlusion) whose box, centre `c` and radius = its
    ///    smallest half-extent, it touches. Damage falls off linearly with
    ///    depth (`1 − s / depth`) and outside the item's radius; a relay
    ///    passes `fuse_multiplier` of it to a fuse, fuses are skipped,
    ///    everything else takes `component_multiplier`, then its own
    ///    resistances.
    ///
    /// `h` is the armour's health ratio: before this shot's wear for the
    /// deflection, after it for the hull and the penetration. With intact armour every ship's
    /// armour in LIVE 4.10 absorbs all penetration (depth 0). The hull is
    /// its ray-colliding physics proxies (INFERRED: the projectile query
    /// tests those), item boxes are the render bounds (they match the
    /// items' box proxies).
    pub fn hit(&self, shot: &Shot, condition: &Condition) -> HitReport {
        let mut report = HitReport::default();
        let dir = crate::raycast::normalize(shot.direction);
        let Some(ray) = self.raycaster.cast(shot.origin, dir) else {
            return report;
        };
        report.point = Some(ray.point);
        report.part = Some((ray.part, self.geometry.collision[ray.part].part.clone()));

        // 1. Shield.
        report.passed = shot.damage;
        if condition.shield_ratio > 0.0
            && let Some(shield) = &self.defense.shield
        {
            let lerp = |m: sc_items_ship_components::MinMax, t: f32| m.min + (m.max - m.min) * t;
            let mut taken = 0.0;
            let mut passed = shot.damage;
            for_each_type(&mut passed, &shot.damage, |i, d| {
                let r = shield.resistance.map_or(0.0, |r| {
                    lerp(pick(&r, i), condition.shield_resistance_ratio)
                });
                let a = shield
                    .absorption
                    .map_or(1.0, |a| lerp(pick(&a, i), condition.shield_ratio));
                taken += d * (1.0 - r) * a;
                d * (1.0 - r) * (1.0 - a)
            });
            report.shield_taken = taken;
            report.passed = passed;
        }

        // 2. Armour.
        let h = condition.armor_ratio.clamp(0.0, 1.0);
        let mut after = report.passed;
        if let Some(armor) = &self.defense.armor {
            for_each_type(&mut after, &report.passed, |i, d| {
                let deflect = armor.deflection.map_or(0.0, |v| at(&v, i));
                if d <= deflect * h {
                    0.0
                } else if h > 0.0 {
                    d * armor.damage_multiplier.map_or(1.0, |v| at(&v, i))
                } else {
                    d
                }
            });
            let own = self
                .vehicle_armor_item()
                .and_then(|g| self.healths.get(&g))
                .and_then(|c| c.damage_multiplier);
            report.armor_wear = after.physical * own.map_or(1.0, |m| m.physical)
                + after.energy * own.map_or(1.0, |m| m.energy);
        }
        report.after_armor = after;
        // The hull and the penetration see the armour after this shot's wear.
        let h = match self.defense.armor_health {
            Some(max) if max > 0.0 => (h - report.armor_wear / max).max(0.0),
            _ => h,
        };

        // 3. Hull part.
        let non_distortion =
            after.physical + after.energy + after.thermal + after.biochemical + after.stun;
        report.hull_damage = non_distortion * (1.0 - self.rules.armor_max_effectiveness * h);

        // 4. Penetration.
        let Some((base, [near, far])) = shot.penetration else {
            return report;
        };
        if base <= 0.0 {
            return report;
        }
        let pen = self
            .defense
            .armor
            .as_ref()
            .map(|a| (a.penetration_reduction, a.penetration_absorption));
        let depth_of = |i: usize, d: f32| -> f32 {
            if d <= 0.0 {
                return 0.0;
            }
            let (reduction, absorption) = pen.unwrap_or((0.0, None));
            let absorb = absorption.map_or(0.0, |a| at(&a, i));
            base * (1.0 - absorb * reduction * h).max(0.0)
        };
        let depths: [f32; 6] = std::array::from_fn(|i| depth_of(i, at(&after, i)));
        let length = depths.iter().copied().fold(0.0, f32::max);
        report.depth = length;
        if length <= 0.0 {
            return report;
        }
        let radius = near + (length / base) * (far - near);
        let rules = self.vehicle.penetration;
        for (index, pl) in self.geometry.placements.iter().enumerate() {
            let Some(bounds) = pl.bounds else {
                continue;
            };
            let item = self.items.get(&pl.item);
            let kind = item.map(|i| &i.item_type);
            if matches!(kind, Some(EItemType::Door))
                || item.is_some_and(|i| i.item_sub_type == EItemSubType::Fuse)
            {
                continue;
            }
            let Some(health) = self.healths.get(&pl.item) else {
                continue;
            };
            let centre = apply(
                &pl.transform,
                std::array::from_fn(|k| 0.5 * (bounds.min[k] + bounds.max[k])),
            );
            let r = (0..3)
                .map(|k| 0.5 * (bounds.max[k] - bounds.min[k]))
                .fold(f32::MAX, f32::min);
            let to = sub(centre, ray.point);
            let s = dot(to, dir);
            if s < 0.0 || s > length {
                continue;
            }
            let lateral = sub(to, dir.map(|c| c * s));
            let l = dot(lateral, lateral).sqrt();
            if l >= r + radius {
                continue;
            }
            let edge = if l > r { 1.0 - (l - r) / radius } else { 1.0 };
            let multiplier = match kind {
                Some(EItemType::Relay) => rules.fuse_multiplier,
                _ => rules.component_multiplier,
            };
            let resist = health.damage_multiplier;
            let damage: f32 = (0..6)
                .filter(|&i| i != 2) // distortion is not health damage
                .map(|i| {
                    let k = if depths[i] > 0.0 && s <= depths[i] {
                        1.0 - s / depths[i]
                    } else {
                        0.0
                    };
                    at(&after, i) * k * edge * multiplier * resist.map_or(1.0, |m| at(&m, i))
                })
                .sum();
            if damage > 0.0 {
                report.components.push(ComponentHit {
                    placement: index,
                    item: pl.item,
                    depth: s,
                    damage,
                });
            }
        }
        report
            .components
            .sort_by(|a, b| a.depth.total_cmp(&b.depth));
        report
    }

    /// How exposed component `placement` is: over `samples` directions
    /// spread evenly over the sphere, a shot aimed at its centre meets the
    /// hull at some point; the distance from there to the centre is the
    /// penetration depth that reaches it on the shot's axis. Directions
    /// sorted shallowest first (the best attack angles); a component the
    /// shot meets before any hull (mounted outside) has depth 0. The hull
    /// is [`HullGeometry::collision`]: closed, so an internal component
    /// always has depth.
    pub fn exposure(&self, placement: usize, samples: usize) -> Vec<Exposure> {
        let Some(pl) = self.geometry.placements.get(placement) else {
            return Vec::new();
        };
        let Some(b) = pl.bounds else {
            return Vec::new();
        };
        let centre = apply(
            &pl.transform,
            std::array::from_fn(|k| 0.5 * (b.min[k] + b.max[k])),
        );
        let reach = self.reach();
        let mut out: Vec<Exposure> = fibonacci_sphere(samples)
            .into_iter()
            .map(|d| {
                let origin = std::array::from_fn(|k| centre[k] - d[k] * reach);
                let hit = self.raycaster.cast(origin, d);
                let (depth, part) = match hit {
                    Some(h) if h.distance < reach => (
                        reach - h.distance,
                        Some(self.geometry.collision[h.part].part.clone()),
                    ),
                    _ => (0.0, None),
                };
                Exposure {
                    direction: d,
                    depth,
                    part,
                }
            })
            .collect();
        out.sort_by(|a, b| a.depth.total_cmp(&b.depth));
        out
    }

    /// A distance from which every shot starts outside the hull.
    fn reach(&self) -> f32 {
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for p in self.geometry.collision.iter().flat_map(|p| &p.positions) {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
        let d = sub(hi, lo);
        dot(d, d).sqrt() + 1.0
    }

    /// The hull armour item on the loadout.
    fn vehicle_armor_item(&self) -> Option<Guid> {
        self.geometry.placements.iter().map(|p| p.item).find(|g| {
            self.items
                .get(g)
                .is_some_and(|i| i.item_type == EItemType::Armor)
        })
    }
}

/// One attack direction on a component ([`Target::exposure`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exposure {
    /// The shot's direction (unit, ship frame).
    pub direction: [f32; 3],
    /// Hull surface to the component's centre along the shot, m: the
    /// penetration depth that reaches it.
    pub depth: f32,
    /// The hull part the shot meets first.
    pub part: Option<String>,
}

/// `n` unit vectors spread evenly over the sphere.
fn fibonacci_sphere(n: usize) -> Vec<[f32; 3]> {
    let golden = std::f32::consts::PI * (3.0 - 5f32.sqrt());
    (0..n)
        .map(|i| {
            let z = 1.0 - 2.0 * (i as f32 + 0.5) / n as f32;
            let r = (1.0 - z * z).sqrt();
            let a = golden * i as f32;
            [r * a.cos(), r * a.sin(), z]
        })
        .collect()
}

/// The value for damage type `i` (physical, energy, distortion, thermal,
/// biochemical, stun).
fn at(v: &PerDamageType<f32>, i: usize) -> f32 {
    match i {
        0 => v.physical,
        1 => v.energy,
        2 => v.distortion,
        3 => v.thermal,
        4 => v.biochemical,
        _ => v.stun,
    }
}

fn pick<T: Copy>(v: &PerDamageType<T>, i: usize) -> T {
    match i {
        0 => v.physical,
        1 => v.energy,
        2 => v.distortion,
        3 => v.thermal,
        4 => v.biochemical,
        _ => v.stun,
    }
}

/// `out[i] = f(i, input[i])` for every damage type.
fn for_each_type(
    out: &mut PerDamageType<f32>,
    input: &PerDamageType<f32>,
    mut f: impl FnMut(usize, f32) -> f32,
) {
    out.physical = f(0, input.physical);
    out.energy = f(1, input.energy);
    out.distortion = f(2, input.distortion);
    out.thermal = f(3, input.thermal);
    out.biochemical = f(4, input.biochemical);
    out.stun = f(5, input.stun);
}
