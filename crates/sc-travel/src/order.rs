//! The best order for a set of stops: [`Tour`] describes them,
//! [`Travel::order`] plans them into a [`Plan`].
//!
//! # The model
//!
//! - A stop is one place, or any one of several ([`Tour::visit_any`]); the
//!   planner picks the choice together with the order.
//! - A stop has a dwell (the work done there). Arriving at a place costs an
//!   overhead (landing, docking, launching), paid once per arrival:
//!   consecutive stops at the same place pay it once. The drive's cooldown
//!   runs during it, so an arrival takes at least the cooldown.
//! - Ordering rules ([`Tour::require`], [`TourStop::after`], [`Tour::then`])
//!   say which stop must come before which. A stop without one floats.
//! - The start is a place, or open ([`Tour::anywhere`]: the first stop costs
//!   no travel). The tour ends anywhere, back at the start, or at a place.
//!
//! The objective is total time. This is the travelling-salesman problem with
//! precedence and choices: up to [`EXACT_STOPS`] stops it is solved exactly
//! by dynamic programming over (visited set, last choice) (Held–Karp), and
//! [`Plan::exact`] says so; beyond, a greedy order is improved by moving,
//! reversing and re-choosing stops until nothing gains, which is good but
//! not proven best.

use std::collections::HashMap;

use sc_extract::Guid;
use sc_locations::JumpLink;
use sc_vehicles::Quantum;
use serde::{Deserialize, Serialize};

use crate::{HopKind, Route, Travel};

/// Up to this many stops, [`Travel::order`] finds the proven best order.
pub const EXACT_STOPS: usize = 12;

/// Random restarts of the local search beyond [`EXACT_STOPS`].
const KICKS: usize = 200;

/// A stop in a [`Tour`], as [`Tour::visit`] returned it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StopId(usize);

impl StopId {
    /// The stop's position in the order it was added.
    pub fn index(self) -> usize {
        self.0
    }
}

/// Where a tour ends.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TourEnd {
    /// After the last stop.
    Anywhere,
    /// Back at the start. With an open start there is none, and the tour
    /// ends anywhere.
    Start,
    /// At a place.
    At(Guid),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Spec {
    choices: Vec<Guid>,
    dwell: f64,
    overhead: Option<f64>,
}

/// What to plan: a start, the stops and their rules, and the end.
///
/// ```no_run
/// # fn demo(travel: &sc_travel::Travel, drive: &sc_vehicles::Quantum,
/// #         start: sc_extract::Guid, site: sc_extract::Guid,
/// #         refineries: &[sc_extract::Guid], shops: &[sc_extract::Guid]) {
/// use sc_travel::Tour;
///
/// let mut tour = Tour::new(start);
/// tour.overhead(120.0); // landing and launching, per arrival
/// let mine = tour.visit(site).dwell(3.0 * 3600.0).id();
/// tour.visit_any(refineries).dwell(600.0).after(mine);
/// tour.visit_any(shops).dwell(300.0);
/// tour.end_at_start();
///
/// if let Some(plan) = travel.order(&tour, drive, |_| true) {
///     for v in &plan.visits {
///         println!("stop {} at {:.0} s: {}", v.stop.index(), v.at, v.place);
///     }
///     println!("{:.0} s, proven best: {}", plan.seconds(), plan.exact);
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tour {
    start: Option<Guid>,
    stops: Vec<Spec>,
    rules: Vec<(usize, usize)>,
    end: TourEnd,
    overhead: f64,
}

/// A stop just added to a [`Tour`]: set its dwell, overhead and rules.
pub struct TourStop<'a> {
    tour: &'a mut Tour,
    id: StopId,
}

impl TourStop<'_> {
    /// Seconds of work at the stop (default 0). Consecutive stops at one
    /// place add up their dwells.
    pub fn dwell(self, seconds: f64) -> Self {
        self.tour.stops[self.id.0].dwell = seconds;
        self
    }

    /// The arrival overhead at this stop's place, s, instead of the tour's
    /// ([`Tour::overhead`]).
    pub fn overhead(self, seconds: f64) -> Self {
        self.tour.stops[self.id.0].overhead = Some(seconds);
        self
    }

    /// Only after `before` ([`Tour::require`]).
    pub fn after(self, before: StopId) -> Self {
        self.tour.require(before, self.id);
        self
    }

    /// The stop's handle.
    pub fn id(self) -> StopId {
        self.id
    }
}

impl Tour {
    /// A tour from `start`, ending anywhere.
    pub fn new(start: Guid) -> Self {
        Self {
            start: Some(start),
            stops: Vec::new(),
            rules: Vec::new(),
            end: TourEnd::Anywhere,
            overhead: 0.0,
        }
    }

    /// A tour with an open start: the first stop costs no travel and no
    /// arrival overhead.
    pub fn anywhere() -> Self {
        Self {
            start: None,
            ..Self::new(Guid::from_bytes([0; 16]))
        }
    }

    /// Add a stop at `place`.
    pub fn visit(&mut self, place: Guid) -> TourStop<'_> {
        self.visit_any(&[place])
    }

    /// Add a stop at any one of `places`; the planner picks one.
    pub fn visit_any(&mut self, places: &[Guid]) -> TourStop<'_> {
        self.stops.push(Spec {
            choices: places.to_vec(),
            dwell: 0.0,
            overhead: None,
        });
        let id = StopId(self.stops.len() - 1);
        TourStop { tour: self, id }
    }

    /// `before` must come before `after`. Either may be added first.
    pub fn require(&mut self, before: StopId, after: StopId) -> &mut Self {
        if !self.rules.contains(&(before.0, after.0)) {
            self.rules.push((before.0, after.0));
        }
        self
    }

    /// A stage: every stop in `stops` comes after every stop added before
    /// the earliest of them.
    pub fn then(&mut self, stops: &[StopId]) -> &mut Self {
        let first = stops.iter().map(|s| s.0).min().unwrap_or(0);
        for before in 0..first {
            for &after in stops {
                self.require(StopId(before), after);
            }
        }
        self
    }

    /// The default arrival overhead, s (0 unless set).
    pub fn overhead(&mut self, seconds: f64) -> &mut Self {
        self.overhead = seconds;
        self
    }

    /// End at `place`.
    pub fn end_at(&mut self, place: Guid) -> &mut Self {
        self.end = TourEnd::At(place);
        self
    }

    /// End back at the start.
    pub fn end_at_start(&mut self) -> &mut Self {
        self.end = TourEnd::Start;
        self
    }

    /// End after the last stop (the default).
    pub fn end_anywhere(&mut self) -> &mut Self {
        self.end = TourEnd::Anywhere;
        self
    }

    /// The number of stops.
    pub fn len(&self) -> usize {
        self.stops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stops.is_empty()
    }
}

/// One visit of a [`Plan`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visit {
    pub stop: StopId,
    /// The place chosen for the stop.
    pub place: Guid,
    /// How it is reached from the previous visit or the start (empty at the
    /// same place, or as the first stop of an open start).
    pub route: Route,
    /// Seconds from the start to arriving here, the route done.
    pub at: f64,
    /// Arrival overhead paid here, s: the stop's overhead or the drive's
    /// cooldown, whichever is longer; 0 when already at the place.
    pub overhead: f64,
    /// The stop's dwell, s.
    pub dwell: f64,
}

/// A planned [`Tour`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub visits: Vec<Visit>,
    /// The route to the end place, when the tour ends somewhere.
    pub finish: Option<Route>,
    /// True when the order is proven best ([`EXACT_STOPS`]).
    pub exact: bool,
}

impl Plan {
    /// Total time, s: travel, overheads and dwells.
    pub fn seconds(&self) -> f64 {
        let visits = self
            .visits
            .iter()
            .fold(0.0, |t, v| t + v.route.seconds() + v.overhead + v.dwell);
        visits + self.finish.as_ref().map_or(0.0, Route::seconds)
    }

    /// Distance, m.
    pub fn metres(&self) -> f64 {
        self.routes().fold(0.0, |t, r| t + r.metres())
    }

    /// Quantum fuel, SCU.
    pub fn fuel(&self) -> f64 {
        self.routes().fold(0.0, |t, r| t + r.fuel())
    }

    /// True when any route is an estimate.
    pub fn is_estimate(&self) -> bool {
        self.routes().any(Route::is_estimate)
    }

    fn routes(&self) -> impl Iterator<Item = &Route> + '_ {
        self.visits.iter().map(|v| &v.route).chain(&self.finish)
    }
}

/// The cooldown the drive still owes after `route`.
fn cooldown(route: &Route) -> f64 {
    route
        .hops
        .last()
        .filter(|h| h.kind != HopKind::Tunnel)
        .map_or(0.0, |h| h.cooldown)
}

impl Travel {
    /// Plan `tour` for `drive` through the jump links `usable` accepts: the
    /// order and choices with the least total time. `None` when a stop has
    /// no reachable choice, the ordering rules contradict each other, or a
    /// rule names a stop of another tour.
    pub fn order(
        &self,
        tour: &Tour,
        drive: &Quantum,
        usable: impl Fn(&JumpLink) -> bool,
    ) -> Option<Plan> {
        let n = tour.stops.len();
        if tour.rules.iter().any(|&(a, b)| a >= n || b >= n || a == b) {
            return None;
        }
        // Every (stop, choice) is a node.
        let nodes: Vec<(usize, Guid)> = tour
            .stops
            .iter()
            .enumerate()
            .flat_map(|(s, spec)| spec.choices.iter().map(move |&g| (s, g)))
            .collect();
        let end = match tour.end {
            TourEnd::Anywhere => None,
            TourEnd::Start => tour.start,
            TourEnd::At(g) => Some(g),
        };
        let mut routes: HashMap<(Guid, Guid), Option<Route>> = HashMap::new();
        let mut route = |a: Guid, b: Guid| -> Option<Route> {
            if a == b {
                return Some(Route::default());
            }
            routes
                .entry((a, b))
                .or_insert_with(|| self.path(&a, &b, drive, &usable))
                .clone()
        };
        let overhead = |s: usize| tour.stops[s].overhead.unwrap_or(tour.overhead);
        // Cost of arriving at node `to` and doing its work, from `from`.
        let mut arrive = |from: Option<Guid>, to: usize| -> Option<f64> {
            let (s, g) = nodes[to];
            let dwell = tour.stops[s].dwell;
            match from {
                None => Some(dwell),
                Some(f) if f == g => Some(dwell),
                Some(f) => {
                    let r = route(f, g)?;
                    Some(r.seconds() + overhead(s).max(cooldown(&r)) + dwell)
                }
            }
        };
        let problem = Problem {
            stop_of: nodes.iter().map(|&(s, _)| s).collect(),
            stops: n,
            preds: (0..n)
                .map(|b| {
                    tour.rules
                        .iter()
                        .filter(|r| r.1 == b)
                        .map(|r| r.0)
                        .collect()
                })
                .collect(),
            start: (0..nodes.len()).map(|v| arrive(tour.start, v)).collect(),
            edge: (0..nodes.len())
                .map(|v| {
                    (0..nodes.len())
                        .map(|w| arrive(Some(nodes[v].1), w))
                        .collect()
                })
                .collect(),
            finish: Vec::new(),
        };
        let finish: Vec<Option<f64>> = nodes
            .iter()
            .map(|&(_, g)| match end {
                None => Some(0.0),
                Some(e) => route(g, e).map(|r| r.seconds()),
            })
            .collect();
        let problem = Problem { finish, ..problem };
        let (sequence, exact) = if n <= EXACT_STOPS {
            (problem.exact()?, true)
        } else {
            (problem.heuristic()?, false)
        };

        let mut visits = Vec::with_capacity(n);
        let mut at = 0.0;
        let mut here = tour.start;
        for v in sequence {
            let (s, g) = nodes[v];
            let r = match here {
                Some(f) => route(f, g)?,
                None => Route::default(),
            };
            at += r.seconds();
            let paid = if here.is_some_and(|f| f != g) {
                overhead(s).max(cooldown(&r))
            } else {
                0.0
            };
            let dwell = tour.stops[s].dwell;
            visits.push(Visit {
                stop: StopId(s),
                place: g,
                route: r,
                at,
                overhead: paid,
                dwell,
            });
            at += paid + dwell;
            here = Some(g);
        }
        let finish = match (end, here) {
            (Some(e), Some(f)) => Some(route(f, e)?),
            _ => None,
        };
        Some(Plan {
            visits,
            finish,
            exact,
        })
    }
}

/// The ordering problem over nodes (stop, choice), costs already known.
struct Problem {
    /// The stop each node belongs to.
    stop_of: Vec<usize>,
    stops: usize,
    /// For each stop, the stops that must come before it.
    preds: Vec<Vec<usize>>,
    /// Cost of a node as the first visit; `None` unreachable.
    start: Vec<Option<f64>>,
    /// Cost of node `w` right after node `v`.
    edge: Vec<Vec<Option<f64>>>,
    /// Cost of ending after a node.
    finish: Vec<Option<f64>>,
}

impl Problem {
    /// Held–Karp over (visited stops, last node): the proven best sequence
    /// of nodes, one per stop. `None` when no sequence respects the rules.
    fn exact(&self) -> Option<Vec<usize>> {
        let (n, m) = (self.stops, self.stop_of.len());
        if n == 0 {
            return Some(Vec::new());
        }
        let pred: Vec<u32> = self
            .preds
            .iter()
            .map(|p| p.iter().fold(0u32, |acc, &s| acc | 1 << s))
            .collect();
        let full = (1u32 << n) - 1;
        let mut best = vec![f64::INFINITY; (full as usize + 1) * m];
        let mut prev = vec![usize::MAX; (full as usize + 1) * m];
        let at = |mask: u32, v: usize| mask as usize * m + v;
        for v in 0..m {
            let s = self.stop_of[v];
            if pred[s] == 0
                && let Some(c) = self.start[v]
            {
                best[at(1 << s, v)] = best[at(1 << s, v)].min(c);
            }
        }
        for mask in 1..=full {
            for v in 0..m {
                let here = best[at(mask, v)];
                if !here.is_finite() || mask & (1 << self.stop_of[v]) == 0 {
                    continue;
                }
                for w in 0..m {
                    let s = self.stop_of[w];
                    if mask & (1 << s) != 0 || pred[s] & !mask != 0 {
                        continue;
                    }
                    let Some(c) = self.edge[v][w] else { continue };
                    let next = at(mask | 1 << s, w);
                    if here + c < best[next] {
                        best[next] = here + c;
                        prev[next] = v;
                    }
                }
            }
        }
        let (mut v, _) = (0..m)
            .filter_map(|v| Some((v, best[at(full, v)] + self.finish[v]?)))
            .filter(|(_, c)| c.is_finite())
            .min_by(|a, b| a.1.total_cmp(&b.1))?;
        let mut mask = full;
        let mut out = vec![v];
        while prev[at(mask, v)] != usize::MAX {
            let p = prev[at(mask, v)];
            mask &= !(1 << self.stop_of[v]);
            v = p;
            out.push(v);
        }
        out.reverse();
        Some(out)
    }

    /// Total cost of a node sequence; `None` when it breaks a rule or a
    /// step is unreachable.
    fn cost(&self, seq: &[usize]) -> Option<f64> {
        let mut pos = vec![usize::MAX; self.stops];
        for (i, &v) in seq.iter().enumerate() {
            pos[self.stop_of[v]] = i;
        }
        for (b, preds) in self.preds.iter().enumerate() {
            if preds.iter().any(|&a| pos[a] > pos[b]) {
                return None;
            }
        }
        let (&first, &last) = (seq.first()?, seq.last()?);
        let mut total = self.start[first]? + self.finish[last]?;
        for w in seq.windows(2) {
            total += self.edge[w[0]][w[1]]?;
        }
        Some(total)
    }

    /// A greedy sequence from every possible first node, each improved by
    /// local search ([`improve`](Self::improve)); the best one.
    fn heuristic(&self) -> Option<Vec<usize>> {
        let (mut best_cost, mut best) = (0..self.stop_of.len())
            .filter(|&first| self.preds[self.stop_of[first]].is_empty())
            .filter_map(|first| self.greedy(first))
            .filter_map(|seq| {
                let seq = self.improve(seq);
                Some((self.cost(&seq)?, seq))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0))?;
        // Iterated local search: kick the best with a few random moves,
        // improve, keep it when it gains. Deterministic seed.
        let mut rng = 0x9E37_79B9_7F4A_7C15u64;
        let mut rand = |below: usize| {
            rng = rng
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((rng >> 33) as usize) % below.max(1)
        };
        let n = best.len();
        for _ in 0..KICKS {
            let mut cand = best.clone();
            for _ in 0..3 {
                let i = rand(n);
                let v = cand.remove(i);
                let s = self.stop_of[v];
                let options: Vec<usize> = (0..self.stop_of.len())
                    .filter(|&w| self.stop_of[w] == s)
                    .collect();
                cand.insert(rand(n), options[rand(options.len())]);
            }
            if self.cost(&cand).is_none() {
                continue;
            }
            let cand = self.improve(cand);
            if let Some(c) = self.cost(&cand)
                && c + 1e-9 < best_cost
            {
                (best_cost, best) = (c, cand);
            }
        }
        Some(best)
    }

    /// From `first`, always the cheapest next node whose rules are met.
    fn greedy(&self, first: usize) -> Option<Vec<usize>> {
        self.start[first]?;
        let mut placed = vec![false; self.stops];
        placed[self.stop_of[first]] = true;
        let mut seq = vec![first];
        while seq.len() < self.stops {
            let last = *seq.last()?;
            let (w, _) = (0..self.stop_of.len())
                .filter(|&w| {
                    let s = self.stop_of[w];
                    !placed[s] && self.preds[s].iter().all(|&p| placed[p])
                })
                .filter_map(|w| Some((w, self.edge[last][w]?)))
                .min_by(|a, b| a.1.total_cmp(&b.1))?;
            placed[self.stop_of[w]] = true;
            seq.push(w);
        }
        Some(seq)
    }

    /// Local search until no move gains: re-choose a node; move a run of
    /// one to three nodes elsewhere, forwards or reversed (a single node
    /// also with each of its choices); reverse a stretch (2-opt).
    fn improve(&self, mut seq: Vec<usize>) -> Vec<usize> {
        let Some(mut best) = self.cost(&seq) else {
            return seq;
        };
        let choices: Vec<Vec<usize>> = (0..self.stops)
            .map(|s| {
                (0..self.stop_of.len())
                    .filter(|&w| self.stop_of[w] == s)
                    .collect()
            })
            .collect();
        let n = seq.len();
        for _ in 0..500 {
            let mut found: Option<Vec<usize>> = None;
            let mut consider = |cand: Vec<usize>| {
                if let Some(c) = self.cost(&cand)
                    && c + 1e-9 < best
                {
                    best = c;
                    found = Some(cand);
                }
            };
            for i in 0..n {
                for &w in &choices[self.stop_of[seq[i]]] {
                    let mut cand = seq.clone();
                    cand[i] = w;
                    consider(cand);
                }
            }
            for len in 1..=3.min(n) {
                for i in 0..=n - len {
                    let mut rest = seq.clone();
                    let run: Vec<usize> = rest.drain(i..i + len).collect();
                    for k in 0..=rest.len() {
                        if k == i {
                            continue;
                        }
                        let variants: Vec<Vec<usize>> = if len == 1 {
                            choices[self.stop_of[run[0]]]
                                .iter()
                                .map(|&w| vec![w])
                                .collect()
                        } else {
                            let mut back = run.clone();
                            back.reverse();
                            vec![run.clone(), back]
                        };
                        for v in variants {
                            let mut cand = rest.clone();
                            cand.splice(k..k, v);
                            consider(cand);
                        }
                    }
                }
            }
            for i in 0..n {
                for k in i + 2..=n {
                    let mut cand = seq.clone();
                    cand[i..k].reverse();
                    consider(cand);
                }
            }
            match found {
                Some(better) => seq = better,
                None => break,
            }
        }
        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small deterministic generator (no dependency).
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (self.0 >> 33) as f64 / (1u64 << 31) as f64
        }
    }

    /// Random points on a plane; `choices` places per stop; a few rules.
    fn random(seed: u64, stops: usize, choices: usize, rules: usize) -> Problem {
        let mut r = Lcg(seed);
        let stop_of: Vec<usize> = (0..stops).flat_map(|s| vec![s; choices]).collect();
        let pts: Vec<(f64, f64)> = stop_of.iter().map(|_| (r.next(), r.next())).collect();
        let d = |a: (f64, f64), b: (f64, f64)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
        let mut preds = vec![Vec::new(); stops];
        for _ in 0..rules {
            let (a, b) = (
                (r.next() * stops as f64) as usize,
                (r.next() * stops as f64) as usize,
            );
            // Only forward rules, so there is no cycle.
            if a < b && !preds[b].contains(&a) {
                preds[b].push(a);
            }
        }
        Problem {
            start: pts.iter().map(|&p| Some(d((0.5, 0.5), p))).collect(),
            edge: pts
                .iter()
                .map(|&p| pts.iter().map(|&q| Some(d(p, q) + 0.01)).collect())
                .collect(),
            finish: pts.iter().map(|&p| Some(d(p, (0.5, 0.5)))).collect(),
            stop_of,
            stops,
            preds,
        }
    }

    /// Every order of stops and every choice, the slow way.
    fn brute(p: &Problem) -> f64 {
        fn go(p: &Problem, seq: &mut Vec<usize>, used: &mut Vec<bool>, best: &mut f64) {
            if seq.len() == p.stops {
                if let Some(c) = p.cost(seq) {
                    *best = best.min(c);
                }
                return;
            }
            for w in 0..p.stop_of.len() {
                let s = p.stop_of[w];
                if used[s] {
                    continue;
                }
                used[s] = true;
                seq.push(w);
                go(p, seq, used, best);
                seq.pop();
                used[s] = false;
            }
        }
        let mut best = f64::INFINITY;
        go(p, &mut Vec::new(), &mut vec![false; p.stops], &mut best);
        best
    }

    #[test]
    fn exact_matches_brute_force() {
        for seed in 0..40 {
            let p = random(seed, 5, 2, 3);
            let seq = p.exact().expect("a valid order");
            let got = p.cost(&seq).expect("respects the rules");
            assert!(
                (got - brute(&p)).abs() < 1e-9,
                "seed {seed}: {got} vs {}",
                brute(&p)
            );
        }
    }

    #[test]
    fn heuristic_is_valid_and_close() {
        let (mut worst, mut sum, seeds): (f64, f64, u64) = (1.0, 0.0, 12);
        for seed in 0..seeds {
            let p = random(seed, 9, 2, 4);
            let best = p.cost(&p.exact().unwrap()).unwrap();
            let got = p.cost(&p.heuristic().expect("a valid order")).unwrap();
            assert!(got + 1e-9 >= best);
            worst = worst.max(got / best);
            sum += got / best;
        }
        let mean = sum / seeds as f64;
        println!("heuristic: worst {worst:.4}x, mean {mean:.4}x the best");
        assert!(
            worst < 1.05 && mean < 1.01,
            "worst {worst:.4}x, mean {mean:.4}x"
        );
    }

    #[test]
    fn rules_are_kept_and_a_cycle_has_no_order() {
        let mut p = random(7, 4, 1, 0);
        p.preds[0] = vec![3];
        let seq = p.exact().unwrap();
        let pos = |s: usize| seq.iter().position(|&v| p.stop_of[v] == s).unwrap();
        assert!(pos(3) < pos(0));
        p.preds[3] = vec![0];
        assert!(p.exact().is_none());
        assert!(p.heuristic().is_none());
    }

    #[test]
    fn builder_records_stops_and_rules() {
        let g = |b: u8| Guid::from_bytes([b; 16]);
        let mut tour = Tour::new(g(1));
        let a = tour.visit(g(2)).dwell(10.0).id();
        let b = tour.visit_any(&[g(3), g(4)]).overhead(60.0).after(a).id();
        let c = tour.visit(g(5)).id();
        tour.require(c, a).then(&[c]).end_at_start();
        assert_eq!(tour.len(), 3);
        assert_eq!(tour.rules, vec![(0, 1), (2, 0), (0, 2), (1, 2)]);
        assert_eq!(tour.stops[1].choices, vec![g(3), g(4)]);
        assert_eq!(
            (tour.stops[0].dwell, tour.stops[1].overhead),
            (10.0, Some(60.0))
        );
        assert_eq!(b.index(), 1);
        assert_eq!(tour.end, TourEnd::Start);
    }
}
