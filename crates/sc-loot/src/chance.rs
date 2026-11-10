//! Drop chances: what one fill of a container yields (see [`Odds`]).

use std::collections::{HashMap, HashSet};

use sc_extract::Guid;
use sc_extract::generated::{ELootFullnessMode, ELootPruningLevel};
use serde::{Deserialize, Serialize};

use crate::{Generation, LootConfig, LootTables, RecordCollection, SecondaryGroup, Selector};

/// Results a fill stops at when nothing else bounds it (no limit, no
/// capacity, dynamic pruning).
pub const MAX_RESULTS: u32 = 256;

/// One item's odds in a fill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropChance {
    pub item: Guid,
    /// The chance a fill yields at least one.
    pub chance: f64,
    /// How many a fill yields on average, stacks counted.
    pub expected: f64,
}

/// [`LootTables::simulate`]'s result: what fills of a container yielded,
/// rolled as the client fills it (4.10.197, `LootGenerationDatabase.cpp`,
/// `LootGenerationHelpers.cpp`).
///
/// A fill (`FUN_143cee260`) rolls `chanceToGenerate`, draws the container's
/// target fullness from `fullnessFactorRange` and generates results until
/// the table runs dry, `totalResultsLimit` is reached or the container is
/// full:
/// - **v2** (`GenerateLootForContainers` `FUN_143ced1c0`): every archetype
///   under its minimum (`numberOfResultsConstraints`) first, then archetypes
///   by weight; one reaching its maximum leaves.
/// - **v3** (`GetContainersDecisionTree` `FUN_143cf12b0` and its nodes):
///   each table entry exists with `chanceToExist`; entries with a
///   `priorityCount` go first, in order, until chosen that often; then
///   entries by weight; one chosen `choiceLimit` times leaves; an item
///   dropped `dupeLimit` times is no longer picked by an entry with that
///   limit.
///
/// A result walks the decision tree (see [`SecondaryGroup`]): an archetype
/// entry by weight, a stack size uniform in its range, one entry of each
/// layer by weight, then an item uniformly; empty branches are skipped.
/// Below `dynamic` pruning the item must fit — the room left up to the
/// capacity (`stopAfterExceed`) or the target (`preventExceed`) — and the
/// stack is cut to what fits; a result that does not fit ends the fill, and
/// so does reaching the target. v3 spawn-withs roll their chance and add
/// their amount of a uniform pick.
///
/// [`LootTables::shares`] gives each item's chance to be one pick, from the
/// weights alone; [`LootTables::simulate`] rolls whole fills. Not modelled:
/// the client's RNG stream (only its distributions), a container's grid
/// layout (only its volume), the harvestable spawner (which slot spawns
/// where: [`LootConfig::slot`]) and fills of several containers at once.
/// An actor or corpse fills its loadout's inventories together
/// (`GenerateLootForLoadoutInventories` `FUN_143cee900`: each inventory's
/// free room times the fullness; one picked by capacity, each other kept
/// with `chanceToGenerateAdditionalAttachedInventories`), so its odds here
/// are those of one container of the capacity passed (`None`: unbounded).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Odds {
    /// Fills rolled.
    pub runs: u32,
    /// The share of fills that generated anything.
    pub generated: f64,
    /// Results (entry picks, spawn-withs not counted) per fill, on average.
    pub results: f64,
    /// Every item that dropped, most likely first.
    pub drops: Vec<DropChance>,
}

impl LootTables {
    /// Each item's chance to be one pick of `config`'s table, from the
    /// weights of the table, archetype and layers alone (no minimums,
    /// limits, priorities, existence chances or capacity), times
    /// `chanceToGenerate`. Sorted most likely first; the shares of a table
    /// that can pick add up to `chanceToGenerate`.
    pub fn shares(&self, config: &LootConfig) -> Vec<(Guid, f64)> {
        let Some(plan) = Plan::new(self, config) else {
            return Vec::new();
        };
        let mut share: HashMap<u32, f64> = HashMap::new();
        let total: f64 = plan.tops.iter().map(|t| t.weight).sum();
        if total <= 0.0 {
            return Vec::new();
        }
        for top in &plan.tops {
            let p = top.weight / total * f64::from(config.chance_to_generate.clamp(0.0, 1.0));
            plan.spread(top.root, p, &mut share);
        }
        let mut out: Vec<(Guid, f64)> = share
            .into_iter()
            .map(|(i, p)| (plan.items[i as usize], p))
            .collect();
        out.sort_by(|a, b| {
            b.1.total_cmp(&a.1)
                .then(a.0.to_string().cmp(&b.0.to_string()))
        });
        out
    }

    /// Roll `runs` fills of `config` into a container of `capacity` SCU
    /// (`None`: unbounded), each item taking `volume(item)` SCU (`None`:
    /// no room). Deterministic for a `seed`.
    pub fn simulate(
        &self,
        config: &LootConfig,
        capacity: Option<f64>,
        volume: impl Fn(&Guid) -> Option<f64>,
        runs: u32,
        seed: u64,
    ) -> Odds {
        let mut odds = Odds {
            runs,
            generated: 0.0,
            results: 0.0,
            drops: Vec::new(),
        };
        let Some(plan) = Plan::new(self, config) else {
            return odds;
        };
        let volumes: Vec<f64> = plan
            .items
            .iter()
            .map(|g| volume(g).unwrap_or(0.0))
            .collect();
        let mut rng = Rng(seed);
        let mut seen: Vec<u32> = vec![0; plan.items.len()];
        let mut total: Vec<u64> = vec![0; plan.items.len()];
        let (mut generated, mut results) = (0u64, 0u64);
        for _ in 0..runs {
            let fill = plan.fill(config, capacity, &volumes, &mut rng);
            if fill.generated {
                generated += 1;
            }
            results += u64::from(fill.results);
            for (i, n) in fill.counts {
                seen[i as usize] += 1;
                total[i as usize] += u64::from(n);
            }
        }
        let runs_f = f64::from(runs.max(1));
        odds.generated = generated as f64 / runs_f;
        odds.results = results as f64 / runs_f;
        odds.drops = (0..plan.items.len())
            .filter(|&i| seen[i] > 0)
            .map(|i| DropChance {
                item: plan.items[i],
                chance: f64::from(seen[i]) / runs_f,
                expected: total[i] as f64 / runs_f,
            })
            .collect();
        odds.drops.sort_by(|a, b| {
            b.chance
                .total_cmp(&a.chance)
                .then(a.item.to_string().cmp(&b.item.to_string()))
        });
        odds
    }
}

/// A node of a result's decision tree: weighted children, or a leaf of
/// items.
struct Node {
    weight: f64,
    children: Vec<usize>,
    items: Vec<u32>,
    /// The archetype entry's stack range (set on its root node).
    stack: (i32, i32),
    /// Indices into [`Plan::spawns`] (on the entry's root node).
    spawns: Vec<usize>,
}

/// A v3 spawn-with, resolved.
struct Spawn {
    chance: f64,
    amount: (i32, i32),
    items: Vec<u32>,
}

/// A table-level pick: a v2 archetype or a v3 table entry.
struct Top {
    weight: f64,
    /// The archetype's node (children: its entries).
    root: usize,
    min: u32,
    max: u32,
    exist: f64,
    priority: u32,
    choice_limit: u32,
    dupe_limit: u32,
}

/// A config's table as decision trees over interned items.
struct Plan {
    generation: Generation,
    items: Vec<Guid>,
    nodes: Vec<Node>,
    spawns: Vec<Spawn>,
    tops: Vec<Top>,
}

/// What one fill yielded.
struct Fill {
    generated: bool,
    results: u32,
    counts: HashMap<u32, u32>,
}

impl Plan {
    fn new(tables: &LootTables, config: &LootConfig) -> Option<Self> {
        let table = tables.get(&config.table_used(tables)?)?;
        let pooled = |i: &Guid| {
            config
                .pool_filter
                .as_ref()
                .is_none_or(|f| tables.admits(f, i))
        };
        let mut plan = Self {
            generation: table.generation,
            items: Vec::new(),
            nodes: Vec::new(),
            spawns: Vec::new(),
            tops: Vec::new(),
        };
        let mut index: HashMap<Guid, u32> = HashMap::new();
        let mut intern = |plan: &mut Self, g: &Guid| -> u32 {
            *index.entry(*g).or_insert_with(|| {
                plan.items.push(*g);
                (plan.items.len() - 1) as u32
            })
        };
        for ta in &table.archetypes {
            let layers: &[SecondaryGroup] = match table.generation {
                Generation::V2 => &ta.archetype.secondary,
                Generation::V3 => &config.secondary,
            };
            let mut entries = Vec::new();
            for entry in &ta.archetype.entries {
                let items: Vec<&Guid> = entry.items.iter().filter(|i| pooled(i)).collect();
                let Some(root) = plan.branch(tables, &items, layers, &mut intern) else {
                    continue;
                };
                plan.nodes[root].weight = f64::from(entry.weight.max(0.0));
                plan.nodes[root].stack = entry.stack.unwrap_or((1, 1));
                for s in &entry.spawn_with {
                    let items: Vec<u32> = s
                        .items
                        .iter()
                        .filter(|i| pooled(i))
                        .map(|i| intern(&mut plan, i))
                        .collect();
                    if !items.is_empty() {
                        plan.spawns.push(Spawn {
                            chance: f64::from(s.chance),
                            amount: s.amount.unwrap_or((1, 1)),
                            items,
                        });
                        let at = plan.spawns.len() - 1;
                        plan.nodes[root].spawns.push(at);
                    }
                }
                if plan.nodes[root].weight > 0.0 {
                    entries.push(root);
                }
            }
            if entries.is_empty() {
                continue;
            }
            plan.nodes.push(Node {
                weight: 0.0,
                children: entries,
                items: Vec::new(),
                stack: (1, 1),
                spawns: Vec::new(),
            });
            let (min, max) = ta.results.unwrap_or((0, 0));
            plan.tops.push(Top {
                weight: f64::from(ta.weight.max(0.0)),
                root: plan.nodes.len() - 1,
                min: min.max(0) as u32,
                max: max.max(0) as u32,
                exist: f64::from(ta.chance_to_exist.unwrap_or(1.0)),
                priority: ta.priority_count.unwrap_or(0).max(0) as u32,
                choice_limit: ta.choice_limit.unwrap_or(0).max(0) as u32,
                dupe_limit: ta.dupe_limit.unwrap_or(0).max(0) as u32,
            });
        }
        (!plan.tops.is_empty()).then_some(plan)
    }

    /// The node for `items` split by `layers` (the first layer here, the
    /// rest below); `None` when nothing is left.
    fn branch(
        &mut self,
        tables: &LootTables,
        items: &[&Guid],
        layers: &[SecondaryGroup],
        intern: &mut impl FnMut(&mut Self, &Guid) -> u32,
    ) -> Option<usize> {
        if items.is_empty() {
            return None;
        }
        let Some((layer, rest)) = layers.split_first() else {
            let leaf: Vec<u32> = items.iter().map(|i| intern(self, i)).collect();
            self.nodes.push(Node {
                weight: 0.0,
                children: Vec::new(),
                items: leaf,
                stack: (1, 1),
                spawns: Vec::new(),
            });
            return Some(self.nodes.len() - 1);
        };
        let mut children = Vec::new();
        for e in &layer.entries {
            let kept: Vec<&Guid> = items
                .iter()
                .copied()
                .filter(|i| match &e.selector {
                    Selector::Tags(term) => tables.selects(term, i),
                    Selector::Entities(list) => list.contains(i),
                    Selector::Other { .. } => true,
                })
                .collect();
            if e.weight <= 0.0 {
                continue;
            }
            if let Some(child) = self.branch(tables, &kept, rest, intern) {
                self.nodes[child].weight = f64::from(e.weight);
                children.push(child);
            }
        }
        if children.is_empty() {
            return None;
        }
        self.nodes.push(Node {
            weight: 0.0,
            children,
            items: Vec::new(),
            stack: (1, 1),
            spawns: Vec::new(),
        });
        Some(self.nodes.len() - 1)
    }

    /// Spread probability `p` of reaching `node` over its items.
    fn spread(&self, node: usize, p: f64, out: &mut HashMap<u32, f64>) {
        let n = &self.nodes[node];
        if n.children.is_empty() {
            let each = p / n.items.len() as f64;
            for &i in &n.items {
                *out.entry(i).or_default() += each;
            }
            return;
        }
        let total: f64 = n.children.iter().map(|&c| self.nodes[c].weight).sum();
        for &c in &n.children {
            self.spread(c, p * self.nodes[c].weight / total, out);
        }
    }

    fn fill(
        &self,
        config: &LootConfig,
        capacity: Option<f64>,
        volumes: &[f64],
        rng: &mut Rng,
    ) -> Fill {
        let mut fill = Fill {
            generated: false,
            results: 0,
            counts: HashMap::new(),
        };
        let chance = f64::from(config.chance_to_generate);
        if chance < 1.0 && rng.unit() > chance {
            return fill;
        }
        fill.generated = true;
        let (lo, hi) = config.fullness;
        let fullness = f64::from(lo) + rng.unit() * f64::from(hi - lo);
        let mut room = Room {
            capacity,
            target: capacity.map(|c| c * fullness),
            used: 0.0,
            checks: !matches!(config.pruning, ELootPruningLevel::dynamic),
            prevent: matches!(config.fullness_mode, ELootFullnessMode::preventExceed),
            misfit: false,
        };
        let limit = if config.total_results_limit > 0 {
            config.total_results_limit as u32
        } else {
            MAX_RESULTS
        };
        let mut state = State {
            dead: HashSet::new(),
            banned: HashSet::new(),
        };
        let mut chosen: Vec<u32> = vec![0; self.tops.len()];
        let mut alive: Vec<bool> = self
            .tops
            .iter()
            .map(|t| t.exist >= 1.0 || rng.unit() <= t.exist)
            .collect();
        while fill.results < limit && fill.results < MAX_RESULTS && !room.full() {
            let Some((t, first)) = self.next_top(&alive, &chosen, rng) else {
                break;
            };
            let top = &self.tops[t];
            if top.dupe_limit > 0 {
                state.banned = fill
                    .counts
                    .iter()
                    .filter(|&(_, &n)| n >= top.dupe_limit)
                    .map(|(&i, _)| i)
                    .collect();
            } else {
                state.banned.clear();
            }
            match self.result(top.root, &mut room, volumes, &mut state, rng) {
                Some(picked) => {
                    fill.results += 1;
                    for (i, n) in picked {
                        *fill.counts.entry(i).or_default() += n;
                    }
                    chosen[t] += 1;
                    // A v2 maximum applies to weighted picks only.
                    let done = match self.generation {
                        Generation::V2 => {
                            !first && top.max > 0 && (top.max <= top.min || chosen[t] >= top.max)
                        }
                        Generation::V3 => {
                            top.choice_limit > 0
                                && chosen[t] >= top.priority
                                && (top.choice_limit <= top.priority
                                    || chosen[t] >= top.choice_limit)
                        }
                    };
                    if done {
                        alive[t] = false;
                    }
                }
                // Nothing left that can be picked: the archetype is spent;
                // a result that does not fit ends the fill.
                None if room.misfit => break,
                None => alive[t] = false,
            }
        }
        fill
    }

    /// The table-level pick and whether it is a first-pass one: a v2
    /// archetype under its minimum or a v3 entry under its priority count
    /// first, else one by weight.
    fn next_top(&self, alive: &[bool], chosen: &[u32], rng: &mut Rng) -> Option<(usize, bool)> {
        let first = (0..self.tops.len()).find(|&t| {
            alive[t]
                && match self.generation {
                    Generation::V2 => chosen[t] < self.tops[t].min,
                    Generation::V3 => chosen[t] < self.tops[t].priority,
                }
        });
        if let Some(t) = first {
            return Some((t, true));
        }
        let total: f64 = (0..self.tops.len())
            .filter(|&t| alive[t])
            .map(|t| self.tops[t].weight)
            .sum();
        if total <= 0.0 {
            return None;
        }
        let r = rng.unit() * total;
        let mut acc = 0.0;
        let mut last = None;
        for t in (0..self.tops.len()).filter(|&t| alive[t]) {
            acc += self.tops[t].weight;
            last = Some(t);
            if r < acc {
                return Some((t, false));
            }
        }
        last.map(|t| (t, false))
    }

    /// One result from the archetype at `root`: the item, its stack and
    /// its spawn-withs; `None` when nothing can be picked or it does not
    /// fit (then `room.misfit`).
    fn result(
        &self,
        root: usize,
        room: &mut Room,
        volumes: &[f64],
        state: &mut State,
        rng: &mut Rng,
    ) -> Option<Vec<(u32, u32)>> {
        state.dead.clear();
        let (entry, item) = loop {
            let entry = self.pick_child(root, state, rng)?;
            match self.descend(entry, state, rng) {
                Some(item) => break (entry, item),
                None => {
                    state.dead.insert(entry);
                }
            }
        };
        let (lo, hi) = self.nodes[entry].stack;
        let stack = rng.range(lo.max(1), hi.max(lo.max(1)));
        let fits = room.take(volumes[item as usize], stack);
        if fits == 0 {
            room.misfit = true;
            return None;
        }
        let mut out = vec![(item, fits)];
        for &s in &self.nodes[entry].spawns {
            let spawn = &self.spawns[s];
            if spawn.chance < 1.0 && rng.unit() > spawn.chance {
                continue;
            }
            let (lo, hi) = spawn.amount;
            let n = rng.range(lo.max(1), hi.max(lo.max(1)));
            let i = spawn.items[rng.index(spawn.items.len())];
            let took = room.take(volumes[i as usize], n);
            if took > 0 {
                out.push((i, took));
            }
        }
        Some(out)
    }

    /// A live child of `node` by weight.
    fn pick_child(&self, node: usize, state: &State, rng: &mut Rng) -> Option<usize> {
        let live: Vec<usize> = self.nodes[node]
            .children
            .iter()
            .copied()
            .filter(|c| !state.dead.contains(c))
            .collect();
        let total: f64 = live.iter().map(|&c| self.nodes[c].weight).sum();
        if total <= 0.0 {
            return None;
        }
        let r = rng.unit() * total;
        let mut acc = 0.0;
        for &c in &live {
            acc += self.nodes[c].weight;
            if r < acc {
                return Some(c);
            }
        }
        live.last().copied()
    }

    /// An item under `node`, marking spent branches dead; `None` when none
    /// is left.
    fn descend(&self, node: usize, state: &mut State, rng: &mut Rng) -> Option<u32> {
        let n = &self.nodes[node];
        if n.children.is_empty() {
            let live: Vec<u32> = n
                .items
                .iter()
                .copied()
                .filter(|i| !state.banned.contains(i))
                .collect();
            if live.is_empty() {
                state.dead.insert(node);
                return None;
            }
            return Some(live[rng.index(live.len())]);
        }
        loop {
            let c = self.pick_child(node, state, rng);
            let Some(c) = c else {
                state.dead.insert(node);
                return None;
            };
            if let Some(i) = self.descend(c, state, rng) {
                return Some(i);
            }
        }
    }
}

/// Per-result pruning: spent branches and items over a dupe limit.
struct State {
    dead: HashSet<usize>,
    banned: HashSet<u32>,
}

/// The container being filled, SCU.
struct Room {
    capacity: Option<f64>,
    target: Option<f64>,
    used: f64,
    checks: bool,
    prevent: bool,
    misfit: bool,
}

impl Room {
    /// Reached the target (only counts below dynamic pruning).
    fn full(&self) -> bool {
        self.checks && self.target.is_some_and(|t| self.used >= t)
    }

    /// Take up to `n` of an item of `volume`; how many fit.
    fn take(&mut self, volume: f64, n: i32) -> u32 {
        let n = n.max(0) as u32;
        if !self.checks || volume <= 0.0 {
            self.used += volume * f64::from(n);
            return n;
        }
        let bound = if self.prevent {
            self.target
        } else {
            self.capacity
        };
        let Some(bound) = bound else {
            self.used += volume * f64::from(n);
            return n;
        };
        let room = ((bound - self.used) / volume + 1e-9).floor().max(0.0) as u32;
        let took = n.min(room);
        self.used += volume * f64::from(took);
        took
    }
}

/// SplitMix64: a small deterministic source for the rolls.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform in [0, n).
    fn index(&mut self, n: usize) -> usize {
        ((self.unit() * n as f64) as usize).min(n.saturating_sub(1))
    }

    /// Uniform in [lo, hi].
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.index((hi - lo + 1).max(1) as usize) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Archetype, LootEntry, LootTable, TableArchetype};

    fn guid(n: u8) -> Guid {
        format!("{n:08x}-0000-0000-0000-000000000000")
            .parse()
            .unwrap()
    }

    fn entry(item: u8, weight: f32, stack: Option<(i32, i32)>) -> LootEntry {
        LootEntry {
            name: String::new(),
            weight,
            selector: Selector::Entities(vec![guid(item)]),
            items: vec![guid(item)],
            stack,
            spawn_with: Vec::new(),
        }
    }

    fn arch(weight: f32, entries: Vec<LootEntry>) -> TableArchetype {
        TableArchetype {
            name: String::new(),
            weight,
            archetype: Archetype {
                record: None,
                entries,
                secondary: Vec::new(),
            },
            results: None,
            chance_to_exist: None,
            choice_limit: None,
            dupe_limit: None,
            priority_count: None,
        }
    }

    fn tables(generation: Generation, archetypes: Vec<TableArchetype>) -> LootTables {
        let mut t = LootTables::new();
        t.by_guid.insert(
            guid(100),
            LootTable {
                generation,
                archetypes,
            },
        );
        t
    }

    fn config(generation: Generation, limit: i32) -> LootConfig {
        LootConfig {
            table: (generation == Generation::V2).then_some(guid(100)),
            table_v3: (generation == Generation::V3).then_some(guid(100)),
            pool_filter: None,
            pool_filter_record: None,
            chance_to_generate: 1.0,
            total_results_limit: limit,
            fullness: (1.0, 1.0),
            pruning: ELootPruningLevel::none,
            fullness_mode: ELootFullnessMode::stopAfterExceed,
            secondary: Vec::new(),
            container: None,
            slot: None,
        }
    }

    fn odds_of(odds: &Odds, item: u8) -> (f64, f64) {
        odds.drops
            .iter()
            .find(|d| d.item == guid(item))
            .map_or((0.0, 0.0), |d| (d.chance, d.expected))
    }

    /// Archetypes weighted 3 : 1, a fill generating half the time.
    #[test]
    fn shares_follow_the_weights() {
        let t = tables(
            Generation::V2,
            vec![
                arch(3.0, vec![entry(1, 1.0, None)]),
                arch(1.0, vec![entry(2, 1.0, None)]),
            ],
        );
        let mut c = config(Generation::V2, 1);
        c.chance_to_generate = 0.5;
        let s = t.shares(&c);
        assert_eq!(s.len(), 2);
        assert!((s[0].1 - 0.375).abs() < 1e-12 && s[0].0 == guid(1));
        assert!((s[1].1 - 0.125).abs() < 1e-12);
        let odds = t.simulate(&c, None, |_| Some(0.0), 20_000, 7);
        assert!((odds.generated - 0.5).abs() < 0.02);
        assert!((odds_of(&odds, 1).0 - 0.375).abs() < 0.02);
    }

    /// A v2 archetype's minimum is met before the weighted draws.
    #[test]
    fn v2_minimum_first() {
        let mut rare = arch(0.001, vec![entry(2, 1.0, None)]);
        rare.results = Some((2, 2));
        let t = tables(
            Generation::V2,
            vec![arch(1.0, vec![entry(1, 1.0, None)]), rare],
        );
        let odds = t.simulate(&config(Generation::V2, 3), None, |_| Some(0.0), 1000, 1);
        assert_eq!(odds_of(&odds, 2).0, 1.0);
        assert!(odds_of(&odds, 2).1 >= 2.0);
        assert!((odds.results - 3.0).abs() < 1e-9);
    }

    /// A stack is cut to the room left and the fill stops when full.
    #[test]
    fn capacity_cuts_the_stack() {
        let t = tables(
            Generation::V2,
            vec![arch(1.0, vec![entry(1, 1.0, Some((5, 5)))])],
        );
        let mut c = config(Generation::V2, 0);
        c.fullness_mode = ELootFullnessMode::preventExceed;
        let odds = t.simulate(&c, Some(3.0), |_| Some(1.0), 100, 3);
        assert_eq!(odds_of(&odds, 1), (1.0, 3.0));
        assert!((odds.results - 1.0).abs() < 1e-9);
    }

    /// A v3 priority entry comes first; its choice limit then retires it.
    #[test]
    fn v3_priority_then_choice_limit() {
        let mut first = arch(1000.0, vec![entry(1, 1.0, None)]);
        first.priority_count = Some(1);
        first.choice_limit = Some(1);
        let t = tables(
            Generation::V3,
            vec![first, arch(1.0, vec![entry(2, 1.0, None)])],
        );
        let odds = t.simulate(&config(Generation::V3, 3), None, |_| Some(0.0), 500, 5);
        assert_eq!(odds_of(&odds, 1), (1.0, 1.0));
        assert_eq!(odds_of(&odds, 2), (1.0, 2.0));
    }
}
