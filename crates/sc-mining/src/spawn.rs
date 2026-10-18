//! Rolling rocks of a type, and how a setup fares on them.

use std::collections::HashMap;

use sc_extract::{AssetSource, Mesh, RecordCollection};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::sim::Rng;
use crate::{
    Compositions, Elements, ModifiedRock, Rock, RockType, RockTypes, Ruleset, Setup, Shape,
};

/// The fracture trees of every rock mesh, keyed by geometry path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Shapes {
    by_path: HashMap<String, Shape>,
}

impl Shapes {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the mesh of every geometry the rock types list. A mesh that
    /// cannot be read or has no `main` node is skipped with a warning.
    pub fn build(assets: &AssetSource, rock_types: &RockTypes) -> Self {
        let mut by_path = HashMap::new();
        for geometry in rock_types.values().flat_map(|r| &r.geometries) {
            if by_path.contains_key(&geometry.path) {
                continue;
            }
            match Mesh::read(assets, &geometry.path) {
                Ok(mesh) => {
                    if let Some(shape) = Shape::from_mesh(&geometry.path, &mesh) {
                        by_path.insert(geometry.path.clone(), shape);
                    }
                }
                Err(error) => warn!(path = geometry.path, %error, "rock mesh unreadable"),
            }
        }
        Self { by_path }
    }

    /// The fracture tree of the geometry file `path` (lower-case).
    pub fn get(&self, path: &str) -> Option<&Shape> {
        self.by_path.get(path)
    }

    pub fn len(&self) -> usize {
        self.by_path.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_path.is_empty()
    }

    pub fn insert(&mut self, shape: Shape) {
        self.by_path.insert(shape.path.clone(), shape);
    }
}

/// One rolled rock: its scale, its fracture tree at that scale, and its
/// properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spawn {
    pub scale: f32,
    pub shape: Shape,
    pub rock: Rock,
}

/// What a rock type spawns from, resolved once.
#[derive(Debug, Clone, Copy)]
pub struct Spawner<'a> {
    pub rock_type: &'a RockType,
    pub ruleset: &'a Ruleset,
    pub elements: &'a Elements,
    pub compositions: &'a Compositions,
    /// The meshes to pick from, uniformly (the client picks one of the
    /// provider entry's geometries uniformly).
    pub shapes: &'a [&'a Shape],
}

impl Spawner<'_> {
    /// Roll one rock as the client does at spawn: a scale uniform in the
    /// type's range, a mesh, a composition. `None` without a mesh or a
    /// composition, or when an element lacks a density.
    ///
    /// One draw, picked by `seed` (not the game's seed, which comes from the
    /// spawn location at runtime); for odds use [`survey`](Self::survey).
    pub fn spawn(&self, seed: u64) -> Option<Spawn> {
        let mut rng = Rng::new(seed);
        let t = self.rock_type;
        let scale = t.min_scale + rng.unit() * (t.max_scale - t.min_scale);
        let pick = (rng.unit() * self.shapes.len() as f32) as usize;
        let shape = self
            .shapes
            .get(pick.min(self.shapes.len().saturating_sub(1)))?
            .scaled(scale);
        let composition = self.compositions.get(&t.composition?)?;
        let entries = composition.roll(seed.wrapping_mul(0x9E37_79B9).wrapping_add(1));
        let rock = Rock::from_volume(self.ruleset, self.elements, &entries, shape.volume)?;
        Some(Spawn { scale, shape, rock })
    }

    /// `runs` rocks (draws `0..runs`): a deterministic sample of the type.
    pub fn spawns(&self, runs: u32) -> Vec<Spawn> {
        (0..u64::from(runs)).filter_map(|i| self.spawn(i)).collect()
    }

    /// How `setup` fares on this rock type, over `runs` rolled rocks (a few
    /// thousand give shares to about a percent).
    pub fn survey(&self, setup: &Setup, runs: u32) -> Survey {
        Survey::of(&self.spawns(runs), self.ruleset, self.elements, setup)
    }
}

/// How a setup fares on a set of rolled rocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Survey {
    pub rocks: u32,
    /// Share the setup can break at all (mass under its limit).
    pub breakable: f32,
    /// Share collectable without any fracture.
    pub collectable_whole: f32,
    pub mass: Quantiles,
    pub scu: Quantiles,
    /// Fractures until everything is collectable, over the rocks where that
    /// is possible (see [`Shape::cracks_to_collect`]).
    pub cracks: Quantiles,
    /// Over the breakable rocks: their balance throttle.
    pub balance_throttle: Quantiles,
    /// Over the pieces a fracture of a breakable rock leaves that are still
    /// too big to collect: the share the setup can break in turn (pieces get
    /// their composition by [`Spawn::fracture`], from a clean fracture).
    pub pieces_breakable: f32,
}

/// 10th / 50th / 90th percentile; `NaN` when empty.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantiles {
    pub p10: f32,
    pub p50: f32,
    pub p90: f32,
}

impl Quantiles {
    pub fn of(mut values: Vec<f32>) -> Self {
        values.retain(|v| v.is_finite());
        values.sort_by(f32::total_cmp);
        let at = |q: f32| {
            if values.is_empty() {
                f32::NAN
            } else {
                values[((values.len() - 1) as f32 * q).round() as usize]
            }
        };
        Self {
            p10: at(0.1),
            p50: at(0.5),
            p90: at(0.9),
        }
    }
}

impl Survey {
    pub fn of(spawns: &[Spawn], ruleset: &Ruleset, elements: &Elements, setup: &Setup) -> Self {
        let n = spawns.len().max(1) as f32;
        let mined: Vec<ModifiedRock> = spawns
            .iter()
            .map(|s| ModifiedRock::new(ruleset, &s.rock, setup))
            .collect();
        let share = |count: usize| count as f32 / n;
        let (mut pieces, mut pieces_ok) = (0usize, 0usize);
        for (i, (spawn, m)) in spawns.iter().zip(&mined).enumerate() {
            if !m.is_breakable(ruleset) || spawn.rock.is_absorbable(ruleset) {
                continue;
            }
            for piece in spawn.fracture(ruleset, elements, m.cluster_factor, 0.0, i as u64) {
                if piece.rock.is_absorbable(ruleset) {
                    continue;
                }
                pieces += 1;
                if ModifiedRock::new(ruleset, &piece.rock, setup).is_breakable(ruleset) {
                    pieces_ok += 1;
                }
            }
        }
        Self {
            rocks: spawns.len() as u32,
            breakable: share(mined.iter().filter(|m| m.is_breakable(ruleset)).count()),
            collectable_whole: share(
                spawns
                    .iter()
                    .filter(|s| s.rock.is_absorbable(ruleset))
                    .count(),
            ),
            mass: Quantiles::of(spawns.iter().map(|s| s.rock.mass).collect()),
            scu: Quantiles::of(spawns.iter().map(|s| s.rock.scu).collect()),
            cracks: Quantiles::of(
                spawns
                    .iter()
                    .filter_map(|s| s.shape.cracks_to_collect(ruleset))
                    .map(|c| c as f32)
                    .collect(),
            ),
            balance_throttle: Quantiles::of(
                mined
                    .iter()
                    .filter_map(|m| m.balance_throttle(ruleset))
                    .collect(),
            ),
            pieces_breakable: if pieces > 0 {
                pieces_ok as f32 / pieces as f32
            } else {
                1.0
            },
        }
    }
}
