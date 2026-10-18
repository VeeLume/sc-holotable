//! A vehicle's power network and how the client shares its power out
//! (LIVE 4.10.197).
//!
//! Amounts are the client's integer micro-units: 1 segment (pip) =
//! 1,000,000.

use sc_extract::generated::{EItemType, EMasterMode, ResourceNetworkResource};
use sc_extract::{Guid, RecordCollection};
use sc_items::Items;
use serde::{Deserialize, Serialize};

use crate::network::item_type_serde;
use crate::resource::modes_list_serde;
use crate::{
    DynamicAmount, Mounted, NetworkParams, PoolSize, PortContainer, PowerRange, ResourceProfiles,
    ResourceUnits, Vehicles, network_items, plant_output, signature_ratio,
};

/// Micro-units per segment.
const SEGMENT: i64 = 1_000_000;

/// The power side of a vehicle's resource network for one loadout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerNetwork {
    pub plants: Vec<Plant>,
    /// Every item that draws power, in loadout order.
    pub consumers: Vec<Consumer>,
    /// The vehicle's power pools, by item type as the client orders them.
    pub pools: Vec<Pool>,
    /// [`PortContainer::max_default_distribution`].
    pub max_default_distribution: bool,
    /// [`NetworkParams::distribution`](crate::NetworkParams::distribution).
    pub defaults: crate::DistributionDefaults,
}

/// A power plant on the network.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plant {
    pub path: String,
    pub item: Guid,
    /// Its `Online` Power generation, segments.
    pub units: i32,
    /// [`NetworkParams::plant_modifier`](crate::NetworkParams::plant_modifier).
    pub modifier: i32,
    /// Switched on (every item is, after [`PowerNetwork::build`]: a ship
    /// spawns with most items off and the crew powers it up).
    pub on: bool,
    /// Its `Online` state's signatures, power ranges and heat.
    pub signals: Signals,
}

/// What an item's `Online` state emits and how it heats, for the
/// [`Readout`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Signals {
    /// `EMSignature.nominalSignature`.
    pub em: f32,
    /// `IRSignature.nominalSignature`.
    pub ir: f32,
    /// `powerRanges`.
    pub ranges: Vec<PowerRange>,
    /// It heats and draws coolant for it (a Coolant consumption with the
    /// `ItemResourceDynamicAmountCoolant` override): power plants, shields,
    /// radars, life support, quantum drives.
    pub heats: bool,
}

/// An item that draws power.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consumer {
    pub path: String,
    pub item: Guid,
    #[serde(with = "item_type_serde")]
    pub item_type: EItemType,
    /// Its `Online` Power request, micro-units (standard units round up).
    pub request: i64,
    /// `request × minimumConsumptionFraction`, micro-units (truncated, as
    /// the client does).
    pub minimum: i64,
    /// Coolant it makes per second at full power (coolers; 0 for the rest).
    pub coolant: f64,
    /// The pool it draws from, an index into [`PowerNetwork::pools`].
    pub pool: Option<usize>,
    /// Switched on (every item is, after [`PowerNetwork::build`]: a ship
    /// spawns with most items off and the crew powers it up).
    pub on: bool,
    /// The master modes it is switched off in.
    #[serde(with = "modes_list_serde")]
    pub excluded_in: Vec<EMasterMode>,
    /// Its `Online` state's signatures, power ranges and heat.
    pub signals: Signals,
}

impl Consumer {
    /// It takes supply in `mode`: switched on and not excluded (client
    /// item-control slot +0x728).
    pub fn draws(&self, mode: &EMasterMode) -> bool {
        self.on && self.allowed(mode)
    }

    /// It may run in `mode` at all (slot +0x730: not excluded).
    pub fn allowed(&self, mode: &EMasterMode) -> bool {
        !self.excluded_in.contains(mode)
    }

    /// It is a cooler: power-on tops coolers up first.
    pub fn is_cooler(&self) -> bool {
        self.coolant > 0.0
    }

    fn request_segments(&self) -> f64 {
        self.request as f64 / SEGMENT as f64
    }
}

/// A power pool with its members.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(with = "item_type_serde")]
    pub item_type: EItemType,
    pub size: PoolSize,
    /// Its members, indices into [`PowerNetwork::consumers`], sorted as the
    /// client sorts them: switched-on first, then by ascending minimum with
    /// minimums of 0 last.
    pub members: Vec<usize>,
}

/// How much power each item gets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    /// What all online plants make, segments.
    pub supply: i64,
    /// Segments granted per [`PowerNetwork::consumers`] entry. A consumer
    /// that does not [`draw`](Consumer::draws) in the mode keeps its grant
    /// on record but takes no supply and does not run.
    pub granted: Vec<f64>,
    /// The target per [`PowerNetwork::pools`] entry, segments.
    pub pool_targets: Vec<i64>,
    /// Supply left over, segments.
    pub remaining: i64,
}

/// [`PowerNetwork::readout`]: the network's state as the engineering
/// screen shows it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Readout {
    /// Plants first, then consumers, in network order.
    pub items: Vec<ItemReadout>,
    /// Coolant the coolers make, per second.
    pub coolant_made: f64,
    /// Coolant the items ask for, per second.
    pub coolant_asked: f64,
    /// The items' EM and IR, summed.
    pub em: f64,
    pub ir: f64,
}

impl Readout {
    /// The cooling bar: coolant asked for over coolant made (above 1 the
    /// items heat up); `None` without coolers.
    pub fn cooling(&self) -> Option<f64> {
        (self.coolant_made > 0.0).then(|| self.coolant_asked / self.coolant_made)
    }
}

/// One item of a [`Readout`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemReadout {
    pub path: String,
    pub item: Guid,
    /// Its power ratio with its range modifier.
    pub ratio: f32,
    pub em: f32,
    pub ir: f32,
    /// Coolant it makes per second (coolers).
    pub coolant_made: f64,
    /// Coolant it asks for per second at equilibrium.
    pub coolant_asked: f64,
}

impl PowerNetwork {
    /// The network of `loadout` (a vehicle's [`stock`](crate::Vehicle::stock)
    /// tree or an edited one) on the vehicle whose port container is
    /// `container`: its own items ([`network_items`]), each item's `Online`
    /// state, every item switched on.
    pub fn build(
        loadout: &[Mounted],
        container: Option<&PortContainer>,
        vehicles: &Vehicles,
        items: &Items,
        profiles: &ResourceProfiles,
        params: &NetworkParams,
    ) -> Self {
        let mut plants = Vec::new();
        let mut consumers = Vec::new();
        for (path, item) in network_items(loadout, vehicles) {
            let (Some(profile), Some(info)) = (profiles.get(&item), items.get(&item)) else {
                continue;
            };
            let Some(state) = profile.online() else {
                continue;
            };
            let signals = Signals {
                em: state.em.as_ref().map_or(0.0, |e| e.nominal),
                ir: state.ir.as_ref().map_or(0.0, |e| e.nominal),
                ranges: state.power_ranges.clone(),
                heats: state
                    .consumption(&ResourceNetworkResource::Coolant)
                    .is_some_and(|d| d.dynamic == Some(DynamicAmount::Coolant)),
            };
            if let Some(ResourceUnits::Segments(units)) = state
                .generation(&ResourceNetworkResource::Power)
                .and_then(|d| d.generates.as_ref())
                .map(|a| a.per_second)
            {
                plants.push(Plant {
                    path: path.clone(),
                    item,
                    units,
                    modifier: params.plant_modifier(info.size, profile.plant_modifier),
                    on: true,
                    signals: signals.clone(),
                });
            }
            let Some(delta) = state.consumption(&ResourceNetworkResource::Power) else {
                continue;
            };
            let Some(amount) = delta.consumes.as_ref() else {
                continue;
            };
            let request = micro(amount.per_second);
            let excluded_in = [
                EMasterMode::Navigation,
                EMasterMode::SCM,
                EMasterMode::Stealth,
                EMasterMode::SMP,
            ]
            .into_iter()
            .filter(|m| params.excluded(&info.item_type, profile.mode_exclusions.as_deref(), m))
            .collect();
            consumers.push(Consumer {
                path,
                item,
                item_type: info.item_type.clone(),
                request,
                minimum: (request as f32 * delta.minimum_fraction) as i64,
                coolant: state
                    .generation(&ResourceNetworkResource::Coolant)
                    .and_then(|d| d.generates.as_ref())
                    .map_or(0.0, |a| a.per_second.standard()),
                pool: None,
                on: true,
                excluded_in,
                signals,
            });
        }

        // Pools sorted by item type, as the client keys them.
        let mut pools: Vec<Pool> = container
            .map(|c| &c.power_pools[..])
            .unwrap_or_default()
            .iter()
            .map(|p| Pool {
                item_type: p.item_type.clone(),
                size: p.size,
                members: Vec::new(),
            })
            .collect();
        pools.sort_by_key(|p| params.item_type_rank(&p.item_type));
        for (i, c) in consumers.iter_mut().enumerate() {
            if let Some(p) = pools.iter().position(|p| p.item_type == c.item_type) {
                c.pool = Some(p);
                pools[p].members.push(i);
            }
        }
        let mut network = Self {
            plants,
            consumers,
            pools,
            max_default_distribution: container.is_some_and(|c| c.max_default_distribution),
            defaults: params.distribution,
        };
        for p in 0..network.pools.len() {
            network.sort_members(p);
        }
        network
    }

    /// What the online plants make together, segments
    /// ([`plant_output`] for each).
    pub fn supply(&self) -> i64 {
        let online = self.plants.iter().filter(|p| p.on).count() as i32;
        self.plants
            .iter()
            .filter(|p| p.on)
            .map(|p| i64::from(plant_output(p.units, online, p.modifier)))
            .sum()
    }

    /// A pool's working size, segments: a fixed pool's `poolSize` capped by
    /// what its active members ask, a dynamic pool the sum of their requests
    /// (client `FUN_1429416a0`).
    pub fn pool_size(&self, pool: usize) -> i64 {
        let asked: f64 = self
            .active_members(pool)
            .map(|c| self.consumers[c].request_segments())
            .sum();
        match self.pools[pool].size {
            PoolSize::Fixed(size) => (asked.ceil() as i64).min(i64::from(size)),
            _ => asked as i64,
        }
    }

    /// A pool's minimum, segments: its active members' smallest minimum, 1
    /// when one of them has none, 0 for an empty pool (`FUN_142941740`).
    pub fn pool_minimum(&self, pool: usize) -> i64 {
        let floor = i64::from(self.pool_size(pool) > 0);
        let mut least: Option<f64> = None;
        for c in self.active_members(pool) {
            let m = self.member_minimum(pool, c);
            if m <= 0.0 {
                return floor;
            }
            least = Some(least.map_or(m, |l| l.min(m)));
        }
        floor.max(least.unwrap_or(0.0) as i64)
    }

    /// The power-on distribution in `mode` (client `FUN_14297cb30`, with
    /// the `item_resource.power_dist_*` defaults from the DCB):
    ///
    /// 1. every item outside a pool gets its minimum (rounded up), if that
    ///    is less than what is left;
    /// 2. every pool its minimum, if it fits;
    /// 3. coolers are topped up to their full request;
    /// 4. every pool its default share of its size (half for the weapon,
    ///    engine and shield pools, all for the rest), snapped down to whole
    ///    member minimums;
    /// 5. the items outside a pool are topped up to their full request.
    ///
    /// Items that do not [`draw`](Consumer::draws) in `mode` keep their
    /// grant without taking supply. The client walks steps 1 and 5 in the
    /// order of a hash map of runtime entity ids; this takes loadout order,
    /// so when supply runs out in step 5 the last segments may land on a
    /// different item than in game.
    pub fn power_on(&self, mode: &EMasterMode) -> Allocation {
        let supply = self.supply();
        let mut remaining = supply;
        let mut granted = vec![0.0; self.consumers.len()];
        let loose: Vec<usize> = (0..self.consumers.len())
            .filter(|&i| self.consumers[i].pool.is_none())
            .collect();

        // 1. Minimums.
        for &i in &loose {
            let c = &self.consumers[i];
            if c.request <= 0 {
                continue;
            }
            let grant = ceil_segments(c.minimum).min(c.request / SEGMENT);
            if !c.draws(mode) {
                if grant > 0 {
                    granted[i] = grant as f64;
                }
            } else if grant > 0 && grant < remaining {
                remaining -= grant;
                granted[i] = grant as f64;
            }
        }

        // 2. Pool minimums.
        let total_size: i64 = (0..self.pools.len()).map(|p| self.pool_size(p)).sum();
        let mut pool_min = vec![0; self.pools.len()];
        if self.defaults.pools_fill_min && total_size > 0 {
            for (p, min) in pool_min.iter_mut().enumerate() {
                if self.pools[p].members.is_empty() {
                    continue;
                }
                let want = self.pool_minimum(p).max(1);
                if want <= remaining.min(self.pool_size(p)) {
                    *min = want;
                    if self.pool_allowed(p, mode) {
                        remaining -= want;
                    }
                }
            }
        }

        // 3. Coolers.
        if self.defaults.cooler_min > 0.0 {
            for &i in &loose {
                let c = &self.consumers[i];
                if !c.is_cooler() {
                    continue;
                }
                let target = ((c.request / SEGMENT) as f32 * self.defaults.cooler_min.min(1.0))
                    .ceil() as i64;
                let add = target - granted[i] as i64;
                if add > 0 {
                    let add = if c.draws(mode) {
                        let add = add.min(remaining);
                        remaining -= add;
                        add
                    } else {
                        0
                    };
                    granted[i] += add as f64;
                }
            }
        }

        // 4. Pool shares.
        let mut pool_targets = vec![0; self.pools.len()];
        if total_size > 0 {
            for p in 0..self.pools.len() {
                if self.pools[p].members.is_empty() {
                    continue;
                }
                let size = self.pool_size(p);
                let share = if self.max_default_distribution {
                    1.0
                } else {
                    self.default_share(p)
                };
                let mut target = ((size as f32 * share).ceil() as i64)
                    .max(1)
                    .min(remaining)
                    .min(size);
                target = (target - pool_min[p]).max(0).min(remaining) + pool_min[p];
                if target < size && !matches!(self.pools[p].size, PoolSize::Fixed(_)) {
                    target = self.snap(p, target);
                }
                if self.pool_allowed(p, mode) {
                    remaining += pool_min[p] - target;
                }
                pool_targets[p] = target;
            }
        }

        // 5. Top-up.
        for &i in &loose {
            let c = &self.consumers[i];
            let full = c.request / SEGMENT;
            if full <= 0 {
                continue;
            }
            let mut add = full - granted[i] as i64;
            if c.draws(mode) {
                add = add.min(remaining);
                remaining -= add;
            } else {
                add = 0;
            }
            granted[i] += add as f64;
        }

        let mut allocation = Allocation {
            supply,
            granted,
            pool_targets,
            remaining,
        };
        self.split_pools(&mut allocation);
        allocation
    }

    /// Set every pool member's grant from `allocation.pool_targets`, as the
    /// client shares a pool out each frame (`FUN_1429417b0`): a fixed pool
    /// gives each member the same fraction of its request; a dynamic pool
    /// gives every member its full request when the target covers them all,
    /// else each its minimum in member order while it fits, and what is
    /// left goes to members in order unless a minimum did not fit.
    ///
    /// Call it after changing a pool target (a player moving pips); grants
    /// of items outside pools are left as they are.
    pub fn split_pools(&self, allocation: &mut Allocation) {
        for p in 0..self.pools.len() {
            let pool = &self.pools[p];
            let active: Vec<usize> = self.active_members(p).collect();
            for &c in &pool.members {
                allocation.granted[c] = 0.0;
            }
            let all_asked: i64 = pool.members.iter().fold(0.0, |sum, &c| {
                (sum + self.consumers[c].request_segments()).trunc()
            }) as i64;
            let target = allocation.pool_targets[p]
                .min(self.pool_size(p))
                .min(all_asked) as f64;
            match pool.size {
                PoolSize::Fixed(_) => {
                    let asked: f64 = active
                        .iter()
                        .map(|&c| self.consumers[c].request_segments())
                        .sum();
                    if asked <= 0.0 {
                        continue;
                    }
                    let ratio = target / asked;
                    for &c in &active {
                        allocation.granted[c] = ratio * self.consumers[c].request_segments();
                    }
                }
                _ => {
                    if all_asked <= 0 {
                        continue;
                    }
                    if target / all_asked as f64 >= 1.0 {
                        for &c in &active {
                            allocation.granted[c] = self.consumers[c].request_segments();
                        }
                        continue;
                    }
                    let mut rest = target;
                    let mut short = false;
                    for &c in &active {
                        let m = self.member_minimum(p, c);
                        if m <= rest {
                            allocation.granted[c] = m;
                            rest -= m;
                        } else {
                            short = true;
                        }
                    }
                    if !short {
                        for &c in &active {
                            let more = (self.consumers[c].request_segments()
                                - allocation.granted[c])
                                .min(rest);
                            allocation.granted[c] += more;
                            rest -= more;
                        }
                    }
                }
            }
        }
    }

    /// Each consumer's power ratio under `allocation` in `mode`: granted
    /// over requested, 0 when it does not [`draw`](Consumer::draws).
    pub fn ratios(&self, allocation: &Allocation, mode: &EMasterMode) -> Vec<f32> {
        self.consumers
            .iter()
            .zip(&allocation.granted)
            .map(|(c, &g)| {
                if c.draws(mode) && c.request > 0 {
                    (g / c.request_segments()) as f32
                } else {
                    0.0
                }
            })
            .collect()
    }

    /// What the ship's engineering screen shows under `allocation` in
    /// `mode`, once temperatures have settled.
    ///
    /// Each item's power ratio is granted over requested (a plant's, what
    /// is drawn from it over its output; consumers drain the plants in
    /// network order), times the modifier of its power range
    /// ([`signature_ratio`]); its EM settles at nominal × that ratio
    /// (`UpdatePowerRatio`, `UpdateIdealSignature`). A cooler makes its
    /// coolant × that ratio (`GetGenerationFromDelta`); its IR is nominal ×
    /// what is drawn from it over its full output, the coolers drained in
    /// network order (other items emit no IR below 323 K). Signature
    /// functionality curves are not applied (none in LIVE 4.10). An item that heats
    /// asks for coolant to carry its heat away; the client requests
    /// `min(cap, 2.5 × P × dt)` each step, where `cap` is what returns it to
    /// its minimum cooling temperature (`ItemResourceDynamicAmountCoolant`,
    /// `FUN_1428ac000`), so at equilibrium it asks `P × ratio` per second
    /// with `P` its power segments (a plant's, its share of the output). The
    /// cooling bar is requested over generated (the UI's
    /// `coolantIndicator`, smoothed by a tenth per update).
    ///
    /// Thrusters heat with thrust and ask nothing at rest; they are not
    /// counted.
    pub fn readout(&self, allocation: &Allocation, mode: &EMasterMode) -> Readout {
        let ratios = self.ratios(allocation, mode);
        let online = self.plants.iter().filter(|p| p.on).count() as i32;
        // Consumers drain the plants in network order (UpdateConsumeFromPool).
        let mut power_left: f64 = self
            .consumers
            .iter()
            .zip(&allocation.granted)
            .filter(|(c, _)| c.draws(mode))
            .map(|(_, g)| g)
            .sum();
        let mut items = Vec::new();
        for p in self.plants.iter().filter(|p| p.on) {
            let output = plant_output(p.units, online, p.modifier);
            let drawn = power_left.min(f64::from(output)).max(0.0);
            power_left -= drawn;
            let load = if output > 0 {
                (drawn / f64::from(output)) as f32
            } else {
                0.0
            };
            let ratio = signature_ratio(&p.signals.ranges, output, load);
            items.push(ItemReadout {
                path: p.path.clone(),
                item: p.item,
                ratio,
                em: p.signals.em * ratio,
                ir: 0.0,
                coolant_made: 0.0,
                coolant_asked: if p.signals.heats {
                    f64::from(output) * f64::from(ratio)
                } else {
                    0.0
                },
            });
        }
        let mut coolers = Vec::new();
        for (c, &raw) in self.consumers.iter().zip(&ratios) {
            let units = (c.request / SEGMENT) as i32;
            let ratio = signature_ratio(&c.signals.ranges, units, raw);
            if c.is_cooler() {
                coolers.push((items.len(), c));
            }
            items.push(ItemReadout {
                path: c.path.clone(),
                item: c.item,
                ratio,
                em: c.signals.em * ratio,
                ir: 0.0,
                coolant_made: c.coolant * f64::from(ratio),
                coolant_asked: if c.signals.heats {
                    c.request_segments() * f64::from(ratio)
                } else {
                    0.0
                },
            });
        }
        // The coolant asked for drains the coolers in network order; a
        // cooler's IR follows what is drawn from it over its full output.
        let coolant_asked: f64 = items.iter().map(|i| i.coolant_asked).sum();
        let mut coolant_left = coolant_asked;
        for (i, c) in coolers {
            let drawn = coolant_left.min(items[i].coolant_made).max(0.0);
            coolant_left -= drawn;
            items[i].ir = c.signals.ir * (drawn / c.coolant) as f32;
        }
        Readout {
            coolant_made: items.iter().map(|i| i.coolant_made).sum(),
            coolant_asked,
            em: items.iter().map(|i| f64::from(i.em)).sum(),
            ir: items.iter().map(|i| f64::from(i.ir)).sum(),
            items,
        }
    }

    fn default_share(&self, pool: usize) -> f32 {
        let d = &self.defaults;
        let typed = match self.pools[pool].item_type {
            EItemType::FlightController => d.engines,
            EItemType::Shield => d.shields,
            EItemType::WeaponGun => d.weapons,
            _ => -1.0,
        };
        if typed >= 0.0 {
            typed
        } else if d.pools_proportion >= 0.0 {
            d.pools_proportion
        } else {
            1.0
        }
    }

    /// The largest sum of leading member minimums within `target`; `target`
    /// itself when every minimum fits.
    fn snap(&self, pool: usize, target: i64) -> i64 {
        let mut sum = 0.0;
        for &c in &self.pools[pool].members {
            let m = self.member_minimum(pool, c);
            if (target as f64) < sum + m {
                return sum as i64;
            }
            sum += m;
        }
        target
    }

    /// A member's minimum, segments: rounded in a dynamic pool, as is in a
    /// fixed one.
    fn member_minimum(&self, pool: usize, consumer: usize) -> f64 {
        let m = self.consumers[consumer].minimum as f64 / SEGMENT as f64;
        match self.pools[pool].size {
            PoolSize::Fixed(_) => m,
            _ => m.round(),
        }
    }

    /// Switched-on members, up to the pool's `maxItemCount`.
    fn active_members(&self, pool: usize) -> impl Iterator<Item = usize> + '_ {
        let cap = match self.pools[pool].size {
            PoolSize::Dynamic { max_items: Some(n) } => n.max(0) as usize,
            _ => usize::MAX,
        };
        self.pools[pool]
            .members
            .iter()
            .copied()
            .filter(|&c| self.consumers[c].on)
            .take(cap)
    }

    /// Whether the pool takes supply in `mode`: one of its members may run
    /// (`AreAnyItemsInPoolAllowedToBeOnline`).
    fn pool_allowed(&self, pool: usize, mode: &EMasterMode) -> bool {
        self.pools[pool]
            .members
            .iter()
            .any(|&c| self.consumers[c].allowed(mode))
    }

    fn sort_members(&mut self, pool: usize) {
        let mut members = std::mem::take(&mut self.pools[pool].members);
        members.sort_by(|&a, &b| {
            let key = |c: usize| {
                let m = self.member_minimum(pool, c);
                (!self.consumers[c].on, m <= 0.0, m)
            };
            let (ka, kb) = (key(a), key(b));
            ka.0.cmp(&kb.0)
                .then(ka.1.cmp(&kb.1))
                .then(ka.2.total_cmp(&kb.2))
        });
        self.pools[pool].members = members;
    }
}

/// A rate in micro-units, as the client converts them (standard units
/// round up).
fn micro(units: ResourceUnits) -> i64 {
    match units {
        ResourceUnits::Segments(n) => i64::from(n) * SEGMENT,
        ResourceUnits::Standard(v) => (f64::from(v) * SEGMENT as f64).ceil() as i64,
        ResourceUnits::Centi(n) => i64::from(n) * 10_000,
        ResourceUnits::Micro(n) => i64::from(n),
    }
}

fn ceil_segments(micro: i64) -> i64 {
    (micro as f64 / SEGMENT as f64).ceil() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DistributionDefaults;

    fn consumer(item_type: EItemType, request: i64, fraction: f32) -> Consumer {
        Consumer {
            path: item_type.as_dcb_str().to_string(),
            item: Guid::default(),
            item_type,
            request,
            minimum: (request as f32 * fraction) as i64,
            coolant: 0.0,
            pool: None,
            on: true,
            excluded_in: Vec::new(),
            signals: Signals::default(),
        }
    }

    /// The stock Constellation Mk V Centaurus as the client sees it:
    /// 2 × Sedulity, 2 × Snowfall, Lephari, ComfortAir Plus, Khaos, the
    /// flight controller, 2 × 5CA in a dynamic shield pool, 4 × S5 and
    /// 8 × S3 guns in a fixed weapon pool of 4.
    fn centaurus() -> PowerNetwork {
        let mut consumers = Vec::new();
        for _ in 0..2 {
            let mut c = consumer(EItemType::Cooler, 4 * SEGMENT, 0.75);
            c.coolant = 41.0;
            consumers.push(c);
        }
        consumers.push(consumer(EItemType::Radar, 8 * SEGMENT, 0.25));
        consumers.push(consumer(EItemType::LifeSupportGenerator, 2 * SEGMENT, 0.5));
        let mut qd = consumer(EItemType::QuantumDrive, 3 * SEGMENT, 1.0);
        qd.excluded_in = vec![EMasterMode::SCM];
        consumers.push(qd);
        consumers.push(consumer(
            EItemType::FlightController,
            6 * SEGMENT,
            0.1666667,
        ));
        for _ in 0..2 {
            let mut s = consumer(EItemType::Shield, 4 * SEGMENT, 0.5);
            s.excluded_in = vec![EMasterMode::Navigation];
            consumers.push(s);
        }
        for i in 0..12 {
            let request = if i < 4 { 1_050_000 } else { SEGMENT };
            consumers.push(consumer(EItemType::WeaponGun, request, 0.0));
        }
        let pools = vec![
            Pool {
                item_type: EItemType::FlightController,
                size: PoolSize::Dynamic { max_items: None },
                members: vec![5],
            },
            Pool {
                item_type: EItemType::Shield,
                size: PoolSize::Dynamic { max_items: Some(2) },
                members: vec![6, 7],
            },
            Pool {
                item_type: EItemType::WeaponGun,
                size: PoolSize::Fixed(4),
                members: (8..20).collect(),
            },
        ];
        for (p, pool) in pools.iter().enumerate() {
            for &c in &pool.members {
                consumers[c].pool = Some(p);
            }
        }
        let plant = Plant {
            path: "hardpoint_power_plant".into(),
            item: Guid::default(),
            units: 19,
            modifier: 2,
            on: true,
            signals: Signals::default(),
        };
        PowerNetwork {
            plants: vec![plant.clone(), plant],
            consumers,
            pools,
            max_default_distribution: false,
            defaults: DistributionDefaults {
                pools_fill_min: true,
                cooler_min: 1.0,
                pools_proportion: 1.0,
                weapons: 0.5,
                engines: 0.5,
                shields: 0.5,
            },
        }
    }

    #[test]
    fn pools() {
        let n = centaurus();
        assert_eq!(n.supply(), 24);
        // Flight controller 6 (minimum 1: 6e6 × 0.1666667 truncates),
        // shields 2 × 4 (minimum 2), weapons min(ceil 12.2, 4) (minimum 1).
        assert_eq!((n.pool_size(0), n.pool_minimum(0)), (6, 1));
        assert_eq!((n.pool_size(1), n.pool_minimum(1)), (8, 2));
        assert_eq!((n.pool_size(2), n.pool_minimum(2)), (4, 1));
    }

    #[test]
    fn power_on_scm() {
        let n = centaurus();
        let a = n.power_on(&EMasterMode::SCM);
        // Coolers 4, radar 2 + the last 4 (it comes first here), life
        // support its minimum 1; the quantum drive keeps 3 on record but
        // takes nothing in SCM.
        assert_eq!(&a.granted[..5], &[4.0, 4.0, 6.0, 1.0, 3.0]);
        assert_eq!(a.pool_targets, [3, 4, 2]);
        assert_eq!(a.remaining, 0);
        // Flight controller 3 of 6, each shield 2 of 4, each gun 2 / 12.2 of
        // its request.
        assert_eq!(&a.granted[5..8], &[3.0, 2.0, 2.0]);
        assert!((a.granted[8] - 1.05 * 2.0 / 12.2).abs() < 1e-9);
        assert!((a.granted[19] - 2.0 / 12.2).abs() < 1e-9);
        let ratios = n.ratios(&a, &EMasterMode::SCM);
        assert_eq!(ratios[4], 0.0);
        assert_eq!(ratios[5], 0.5);
    }

    #[test]
    fn power_on_nav() {
        let n = centaurus();
        let a = n.power_on(&EMasterMode::Navigation);
        // The quantum drive draws its 3; the shield pool keeps its target 4
        // without taking supply, so the radar gets the last 5.
        assert_eq!(&a.granted[..5], &[4.0, 4.0, 7.0, 1.0, 3.0]);
        assert_eq!(a.pool_targets, [3, 4, 2]);
        assert_eq!(a.remaining, 0);
        let ratios = n.ratios(&a, &EMasterMode::Navigation);
        assert_eq!((ratios[6], ratios[7]), (0.0, 0.0));
    }

    #[test]
    fn player_pool_target() {
        let n = centaurus();
        let mut a = n.power_on(&EMasterMode::SCM);
        // Shields to 3 of 8: the first gets its minimum 2, the second's
        // does not fit, so it stays off and nobody gets the third segment.
        a.pool_targets[1] = 3;
        n.split_pools(&mut a);
        assert_eq!(&a.granted[6..8], &[2.0, 0.0][..]);
        // 6 of 8: both minimums, then the rest in order.
        a.pool_targets[1] = 6;
        n.split_pools(&mut a);
        assert_eq!(&a.granted[6..8], &[4.0, 2.0][..]);
        // Weapons above their size are capped at 4.
        a.pool_targets[2] = 9;
        n.split_pools(&mut a);
        let guns: f64 = a.granted[8..20].iter().sum();
        assert!((guns - 4.0).abs() < 1e-9);
    }
}
