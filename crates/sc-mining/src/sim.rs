//! The charge bar tick by tick, and a difficulty measured from many runs.

use std::collections::VecDeque;
use std::f32::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::ruleset::HitConsistency;
use crate::setup::{breaking_scale, decay_rate};
use crate::{ModifiedRock, Ruleset};

/// The laser's hit consistency `c` (0..1) after `firing_s` seconds of
/// unbroken hits whose hit points scatter by `scatter_m` (standard deviation,
/// metres).
///
/// Client code (LIVE 4.10): the laser keeps the hits of the last
/// `history_window` seconds and scores `c = clamp((hit time in the window /
/// window)^time_exponent / (standard_deviation_multiplier * max(σ,
/// min_deviation)), 0, 1)`, σ being the sample standard deviation of the hit
/// points. Steady aim on one spot raises it; it damps the instability wave by
/// up to [`max_effect_on_instability`](HitConsistency::max_effect_on_instability)
/// and speeds extraction. Whether the hit points are taken in world or rock
/// space (which decides how much a spinning rock scatters them) is not read.
pub fn hit_consistency(params: &HitConsistency, firing_s: f32, scatter_m: f32) -> f32 {
    let window = params.history_window.max(f32::EPSILON);
    let covered = (firing_s.clamp(0.0, window) / window).powf(params.time_exponent);
    let spread = scatter_m.max(params.min_deviation) * params.standard_deviation_multiplier;
    if spread <= 0.0 {
        return 1.0;
    }
    (covered / spread).clamp(0.0, 1.0)
}

/// Hit time the rock takes from a laser's queue per tick, as a multiple of
/// the tick (client code, LIVE 4.10: a literal 1.2).
pub const HIT_DRAIN: f32 = 1.2;

/// A laser's hit queue holds at most this much hit time; a hit arriving at a
/// full queue is dropped (client code, LIVE 4.10).
pub const HIT_QUEUE_CAP_S: f32 = 1.0;

/// How a simulation runs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimConfig {
    /// Seconds per rock update: the frame time of the machine that owns the
    /// rock, a runtime value the data does not give.
    pub tick_s: f32,
    /// Seconds between a laser's hits. The beam ticks at a fixed 1/30 s and
    /// keeps one raycast in flight; each answer delivers the hit time since
    /// the previous one, so this is 1/30 s times the ticks a raycast takes.
    pub hit_interval_s: f32,
    /// Give up after this long.
    pub max_time_s: f32,
    /// Standard deviation of the hit points, m (see [`hit_consistency`]).
    /// The default 0.25 m is a modelling choice, not data.
    pub aim_scatter_m: f32,
    /// Seeds the instability wave (each period draws a length and a height).
    pub seed: u64,
    /// Keep every tick in [`Trace::samples`].
    pub record: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            tick_s: 1.0 / 30.0,
            hit_interval_s: 1.0 / 30.0,
            max_time_s: 180.0,
            aim_scatter_m: 0.25,
            seed: 0,
            record: true,
        }
    }
}

/// What a throttle controller sees each tick.
#[derive(Debug, Clone, Copy)]
pub struct View<'a> {
    pub time_s: f32,
    /// The charge bar now, 0..1.
    pub power: f32,
    pub rock: &'a ModifiedRock,
    pub ruleset: &'a Ruleset,
}

/// Chooses the throttle (0..1, every laser at the same setting) each tick.
pub trait Throttle {
    fn throttle(&mut self, view: &View<'_>) -> f32;
}

/// One throttle, held.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fixed(pub f32);

impl Throttle for Fixed {
    fn throttle(&mut self, _: &View<'_>) -> f32 {
        self.0
    }
}

/// A player model (ours, not the game's): aims for the window's centre,
/// seeing the bar `reaction_s` late, and corrects a gap from the centre over
/// about `correction_s` seconds around the balance throttle. Full throttle
/// when the rock cannot be held.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub reaction_s: f32,
    pub correction_s: f32,
    #[serde(skip)]
    seen: VecDeque<(f32, f32)>,
}

impl Player {
    pub fn new(reaction_s: f32, correction_s: f32) -> Self {
        Self {
            reaction_s,
            correction_s,
            seen: VecDeque::new(),
        }
    }
}

impl Default for Player {
    /// 0.3 s reaction, 1 s correction.
    fn default() -> Self {
        Self::new(0.3, 1.0)
    }
}

impl Throttle for Player {
    fn throttle(&mut self, view: &View<'_>) -> f32 {
        self.seen.push_back((view.time_s, view.power));
        while self
            .seen
            .get(1)
            .is_some_and(|&(t, _)| t <= view.time_s - self.reaction_s)
        {
            self.seen.pop_front();
        }
        let power = self.seen.front().map_or(0.0, |&(_, p)| p);
        let Some(balance) = view.rock.balance_throttle(view.ruleset) else {
            return 1.0;
        };
        let full = view.rock.gain_rate(view.ruleset, 1.0);
        let gap = view.rock.window.centre() - power;
        balance + gap / (full * self.correction_s.max(f32::EPSILON))
    }
}

/// Which breaking pool filled first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pool {
    /// The controlled-breaking pool: fills inside the window (and, more
    /// slowly, above it).
    Controlled,
    /// Filled above the window: an overcharge.
    Danger,
}

/// How a simulated attempt ended.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    /// A pool reached 1 and the rock fractured. `danger` is the danger pool
    /// then (the client spawns fewer, and more explosive, pieces the fuller it is).
    Fractured {
        time_s: f32,
        pool: Pool,
        danger: f32,
    },
    /// Neither pool filled within [`SimConfig::max_time_s`].
    TimedOut,
}

/// One tick.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    pub time_s: f32,
    pub throttle: f32,
    /// The charge bar after the tick.
    pub power: f32,
    /// The wave's factor this tick: a rising bar was multiplied by `1 + wave`
    /// (or divided by `1 - wave` when negative). Zero effect while falling.
    pub wave: f32,
    pub controlled: f32,
    pub danger: f32,
}

/// A simulated attempt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    pub samples: Vec<Sample>,
    pub outcome: Outcome,
    pub time_in_window_s: f32,
}

/// Run one attempt on `rock` with `throttle` choosing the setting.
///
/// Per tick, as the client does (LIVE 4.10): the wave `w = sin(2π t / T) *
/// amp` (each period draws `T = period ± variance` and `amp` in 0..1); the
/// hits that arrived join the queue, and up to [`HIT_DRAIN`] ticks of hit
/// time leave it, at the throttle set now; their gain minus the decay; if
/// that is a gain, it is multiplied by `1 + f` (or divided by `1 - f`) with
/// `f = instability * w * (1 - min(c, maxEffectOnInstability))`; then the
/// bar, then the breaking pools.
///
/// When hits arrive once per tick ([`SimConfig::hit_interval_s`] equal to
/// [`SimConfig::tick_s`], the default) every tick drains exactly one hit and
/// the gain is even. Otherwise ticks alternate between more and less than
/// the mean, and the wave amplifies the surplus of the strong ones even at
/// the balance throttle. Every laser hits in step. Not modelled: the
/// throttle's lerp toward the setting, and rotation changing the distance.
pub fn simulate(
    rock: &ModifiedRock,
    ruleset: &Ruleset,
    throttle: &mut impl Throttle,
    config: &SimConfig,
) -> Trace {
    let dt = config.tick_s.max(1e-4);
    let interval = config.hit_interval_s.max(1e-4);
    let decay = decay_rate(ruleset) * dt;
    let scale = breaking_scale(ruleset, rock.mass);
    let mut rng = Rng::new(config.seed);
    let mut wave = Wave::new(ruleset, &mut rng);

    let mut samples = Vec::new();
    let (mut power, mut controlled, mut danger) = (0.0f32, 0.0f32, 0.0f32);
    let mut time_in_window = 0.0;
    let mut time = 0.0f32;
    // Hits are counted on an exact clock so that equal tick and hit intervals
    // never drift into a tick with two hits or none.
    let (mut ticks, mut hits, mut queued) = (0u64, 0u64, 0.0f32);
    let outcome = loop {
        if time >= config.max_time_s {
            break Outcome::TimedOut;
        }
        let w = wave.advance(ruleset, &mut rng, dt);
        let view = View {
            time_s: time,
            power,
            rock,
            ruleset,
        };
        let setting = throttle.throttle(&view).clamp(0.0, 1.0);

        ticks += 1;
        let end = ticks as f64 * f64::from(dt);
        while (hits + 1) as f64 * f64::from(interval) <= end + 1e-9 {
            hits += 1;
            if queued < HIT_QUEUE_CAP_S {
                queued += interval;
            }
        }
        let drained = queued.min(HIT_DRAIN * dt);
        queued -= drained;

        let mut change = rock.gain_rate(ruleset, setting) * drained - decay;
        let consistency =
            hit_consistency(&ruleset.hit_consistency, time + dt, config.aim_scatter_m);
        let damping = 1.0 - consistency.min(ruleset.hit_consistency.max_effect_on_instability);
        let f = rock.instability * w * damping;
        if change > 0.0 {
            change = if f < 0.0 {
                change / (1.0 - f)
            } else {
                change * (1.0 + f)
            };
        }
        power = (power + change).clamp(0.0, 1.0);
        time += dt;

        let window = rock.window;
        if window.contains(power) {
            time_in_window += dt;
            controlled += rock
                .optimal_rate
                .apply(scale * ruleset.controlled_breaking_fill_rate)
                * dt;
        } else if power > window.high {
            controlled += rock
                .catastrophic_rate
                .apply(scale * ruleset.controlled_breaking_fill_rate_danger)
                * dt;
        } else {
            controlled =
                (controlled - ruleset.controlled_breaking_decay_rate * scale * dt).max(0.0);
        }
        if power <= window.high {
            danger = (danger - ruleset.danger_breaking_decay_rate * scale * dt).max(0.0);
        } else {
            let over = if window.high < 1.0 {
                (power - window.high) / (1.0 - window.high)
            } else {
                0.0
            };
            // The additive catastrophic term is added per tick, not per second
            // (client code).
            danger += (over + 1.0).powf(ruleset.danger_breaking_fill_rate_exponent)
                * ruleset.danger_breaking_fill_rate
                * scale
                * dt
                * rock.catastrophic_rate.factor
                + rock.catastrophic_rate.add;
        }

        if config.record {
            samples.push(Sample {
                time_s: time,
                throttle: setting,
                power,
                wave: f,
                controlled,
                danger,
            });
        }
        if controlled >= 1.0 || danger >= 1.0 {
            break Outcome::Fractured {
                time_s: time,
                pool: if controlled >= 1.0 {
                    Pool::Controlled
                } else {
                    Pool::Danger
                },
                danger: danger.min(1.0),
            };
        }
    };
    Trace {
        samples,
        outcome,
        time_in_window_s: time_in_window,
    }
}

/// Our difficulty for one rock and setup: how a [`Player`] fares over many
/// attempts with different waves.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Difficulty {
    pub runs: u32,
    /// Share of attempts whose controlled-breaking pool filled first. That
    /// pool also fills (more slowly) above the window, so read it with
    /// [`mean_danger`](Self::mean_danger).
    pub controlled: f32,
    /// Share whose danger pool filled first (an overcharge).
    pub overcharged: f32,
    /// Share that neither fractured nor overcharged in time.
    pub timed_out: f32,
    /// Median time to a fracture of either kind.
    pub median_time_s: Option<f32>,
    /// Mean danger pool at fracture, over the attempts that fractured: 0 is a
    /// clean fracture, and the fuller it is the fewer (and more explosive)
    /// the pieces.
    pub mean_danger: f32,
    /// Mean share of the attempt spent inside the window.
    pub in_window: f32,
}

/// Run `runs` attempts (seeds `config.seed`, `config.seed + 1`, …) with a
/// fresh copy of `player` each and summarise them.
pub fn difficulty(
    rock: &ModifiedRock,
    ruleset: &Ruleset,
    player: &Player,
    config: &SimConfig,
    runs: u32,
) -> Difficulty {
    let mut times = Vec::new();
    let (mut controlled, mut over, mut timed_out) = (0u32, 0u32, 0u32);
    let (mut danger_sum, mut in_window_sum) = (0.0f32, 0.0f32);
    for i in 0..runs {
        let config = SimConfig {
            seed: config.seed.wrapping_add(u64::from(i)),
            record: false,
            ..*config
        };
        let trace = simulate(rock, ruleset, &mut player.clone(), &config);
        let duration = match trace.outcome {
            Outcome::Fractured {
                time_s,
                pool,
                danger,
            } => {
                match pool {
                    Pool::Controlled => controlled += 1,
                    Pool::Danger => over += 1,
                }
                times.push(time_s);
                danger_sum += danger;
                time_s
            }
            Outcome::TimedOut => {
                timed_out += 1;
                config.max_time_s
            }
        };
        if duration > 0.0 {
            in_window_sum += trace.time_in_window_s / duration;
        }
    }
    times.sort_by(f32::total_cmp);
    let share = |n: u32| {
        if runs > 0 {
            n as f32 / runs as f32
        } else {
            0.0
        }
    };
    Difficulty {
        runs,
        controlled: share(controlled),
        overcharged: share(over),
        timed_out: share(timed_out),
        median_time_s: times.get(times.len() / 2).copied(),
        mean_danger: if times.is_empty() {
            0.0
        } else {
            danger_sum / times.len() as f32
        },
        in_window: if runs > 0 {
            in_window_sum / runs as f32
        } else {
            0.0
        },
    }
}

/// The instability wave's phase and current period.
struct Wave {
    t: f32,
    period: f32,
    amplitude: f32,
}

impl Wave {
    fn new(ruleset: &Ruleset, rng: &mut Rng) -> Self {
        let mut wave = Self {
            t: 0.0,
            period: 0.0,
            amplitude: 0.0,
        };
        wave.draw(ruleset, rng);
        wave
    }

    fn draw(&mut self, ruleset: &Ruleset, rng: &mut Rng) {
        self.period = (ruleset.instability_wave_period
            + (2.0 * rng.unit() - 1.0) * ruleset.instability_wave_variance)
            .max(1.19e-7);
        self.amplitude = rng.unit();
    }

    /// The wave value now, then step `dt`.
    fn advance(&mut self, ruleset: &Ruleset, rng: &mut Rng, dt: f32) -> f32 {
        if self.t >= self.period {
            self.t -= self.period;
            self.draw(ruleset, rng);
        }
        let w = (TAU * self.t / self.period).sin() * self.amplitude;
        self.t += dt;
        w
    }
}

/// SplitMix64: small, seedable, good enough for wave and spawn draws.
pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub(crate) fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
}
