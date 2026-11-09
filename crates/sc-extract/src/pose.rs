//! Evaluating animation clips on a skeleton.
//!
//! A [`Pose`] starts as the skeleton's bind pose. [`Pose::apply`] layers one
//! clip at a frame: a bone the clip animates takes the clip's value, and an
//! [additive](crate::AnimationClip::additive) clip multiplies its rotation in
//! front and adds its position instead. Bones a clip does not name keep what
//! they had, so order the calls base first, additive layers after. The front
//! multiply is measured: on pairs of a clip and its additive version, only that
//! order leaves a constant base (`pose_probe`).
//!
//! Between keys a rotation is normalised-lerped along the shorter arc and a
//! position is lerped. Choosing which clips to play and at what weight is the
//! game's Mannequin logic and is not modelled here.
//!
//! ```no_run
//! use sc_extract::{AnimationClips, AssetSource, Pose, Skeleton};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! let skeleton = Skeleton::read(
//!     &assets,
//!     "objects/characters/human/male_v7/export/bhm_skeleton_v7.chr",
//! )?;
//! let clips = AnimationClips::read(
//!     &assets,
//!     "animations/characters/human/male_v7/locomotion_sets/noweapon/nw_neutral_stand_idle_base_static.caf",
//! )?;
//!
//! let mut pose = Pose::new(&skeleton);
//! pose.apply(&clips.clips[0], 0.0, 1.0);
//! let model = pose.model_space();
//! println!("{:?}", model[0].position);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;

use crate::{AnimationClip, BoneChannel, Keyframe, Skeleton};

const IDENTITY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// A rotation, quaternion `(x, y, z, w)`, and a position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoneTransform {
    pub rotation: [f32; 4],
    pub position: [f32; 3],
}

/// Parent-relative transforms for every bone of one skeleton, in its bone order.
#[derive(Debug, Clone)]
pub struct Pose {
    bones: Vec<BoneTransform>,
    bind: Vec<BoneTransform>,
    parents: Vec<Option<usize>>,
    parents_first: Vec<usize>,
    by_controller: HashMap<u32, usize>,
}

impl Pose {
    /// The skeleton's bind pose.
    pub fn new(skeleton: &Skeleton) -> Self {
        let bind: Vec<BoneTransform> = skeleton
            .bones
            .iter()
            .map(|b| BoneTransform {
                rotation: b.local_rotation,
                position: b.local_position,
            })
            .collect();
        let parents: Vec<Option<usize>> = skeleton.bones.iter().map(|b| b.parent).collect();
        let mut by_controller = HashMap::with_capacity(skeleton.bones.len());
        for (i, bone) in skeleton.bones.iter().enumerate() {
            by_controller.entry(bone.controller_id).or_insert(i);
        }
        Self {
            bones: bind.clone(),
            parents_first: parents_first(&parents),
            bind,
            parents,
            by_controller,
        }
    }

    /// Back to the bind pose.
    pub fn reset(&mut self) {
        self.bones.copy_from_slice(&self.bind);
    }

    /// Layer `clip` at `frame` (clamped to its keys) with `weight` in `0..=1`.
    pub fn apply(&mut self, clip: &AnimationClip, frame: f32, weight: f32) {
        for channel in &clip.channels {
            let Some(&bone) = self.by_controller.get(&channel.bone_hash) else {
                continue;
            };
            let current = &mut self.bones[bone];
            if let Some(rotation) = channel.rotation_at(frame) {
                current.rotation = if clip.additive {
                    multiply(nlerp(IDENTITY, rotation, weight), current.rotation)
                } else {
                    nlerp(current.rotation, rotation, weight)
                };
            }
            if let Some(position) = channel.position_at(frame) {
                current.position = if clip.additive {
                    std::array::from_fn(|i| current.position[i] + position[i] * weight)
                } else {
                    lerp(current.position, position, weight)
                };
            }
        }
    }

    /// Parent-relative transforms, in the skeleton's bone order.
    pub fn local(&self) -> &[BoneTransform] {
        &self.bones
    }

    /// Model-space transforms, in the skeleton's bone order.
    pub fn model_space(&self) -> Vec<BoneTransform> {
        let mut model = self.bones.clone();
        for &bone in &self.parents_first {
            if let Some(parent) = self.parents[bone] {
                let (p, local) = (model[parent], self.bones[bone]);
                let moved = rotate(p.rotation, local.position);
                model[bone] = BoneTransform {
                    rotation: multiply(p.rotation, local.rotation),
                    position: std::array::from_fn(|i| p.position[i] + moved[i]),
                };
            }
        }
        model
    }
}

/// Bones ordered so a parent precedes its children. A bone caught in a parent
/// cycle is left out and stays parent-relative.
fn parents_first(parents: &[Option<usize>]) -> Vec<usize> {
    let mut children = vec![Vec::new(); parents.len()];
    let mut todo = Vec::new();
    for (bone, parent) in parents.iter().enumerate() {
        match parent {
            Some(p) => children[*p].push(bone),
            None => todo.push(bone),
        }
    }
    let mut order = Vec::with_capacity(parents.len());
    while let Some(bone) = todo.pop() {
        order.push(bone);
        todo.extend_from_slice(&children[bone]);
    }
    order
}

impl AnimationClip {
    /// The last frame any key sits on.
    pub fn last_frame(&self) -> u16 {
        self.channels
            .iter()
            .flat_map(|c| {
                let rotations = c.rotations.last().map(|k| k.frame);
                rotations
                    .into_iter()
                    .chain(c.positions.last().map(|k| k.frame))
            })
            .max()
            .unwrap_or(0)
    }
}

impl BoneChannel {
    /// The rotation at `frame`, held at the first and last keys.
    pub fn rotation_at(&self, frame: f32) -> Option<[f32; 4]> {
        sample(&self.rotations, frame, nlerp)
    }

    /// The position at `frame`, held at the first and last keys.
    pub fn position_at(&self, frame: f32) -> Option<[f32; 3]> {
        sample(&self.positions, frame, lerp)
    }
}

fn sample<T: Copy>(keys: &[Keyframe<T>], frame: f32, mix: impl Fn(T, T, f32) -> T) -> Option<T> {
    let first = keys.first()?;
    let next = keys.partition_point(|k| f32::from(k.frame) <= frame);
    if next == 0 {
        return Some(first.value);
    }
    let a = keys[next - 1];
    let Some(b) = keys.get(next) else {
        return Some(a.value);
    };
    let t = (frame - f32::from(a.frame)) / (f32::from(b.frame) - f32::from(a.frame));
    Some(mix(a.value, b.value, t))
}

fn lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t)
}

fn nlerp(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let dot: f32 = (0..4).map(|i| a[i] * b[i]).sum();
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    let q: [f32; 4] = std::array::from_fn(|i| a[i] + (sign * b[i] - a[i]) * t);
    let length = q.iter().map(|v| v * v).sum::<f32>().sqrt();
    if length > 0.0 {
        q.map(|v| v / length)
    } else {
        a
    }
}

fn multiply(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let [ax, ay, az, aw] = a;
    let [bx, by, bz, bw] = b;
    [
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
        aw * bw - ax * bx - ay * by - az * bz,
    ]
}

fn rotate(q: [f32; 4], v: [f32; 3]) -> [f32; 3] {
    let [x, y, z, w] = q;
    let t = [
        2.0 * (y * v[2] - z * v[1]),
        2.0 * (z * v[0] - x * v[2]),
        2.0 * (x * v[1] - y * v[0]),
    ];
    [
        v[0] + w * t[0] + y * t[2] - z * t[1],
        v[1] + w * t[1] + z * t[0] - x * t[2],
        v[2] + w * t[2] + x * t[1] - y * t[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bone, Keyframe};

    const QUARTER_Z: [f32; 4] = [
        0.0,
        0.0,
        std::f32::consts::FRAC_1_SQRT_2,
        std::f32::consts::FRAC_1_SQRT_2,
    ];

    fn close<const N: usize>(a: [f32; N], b: [f32; N]) -> bool {
        a.iter().zip(&b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    fn bone(name: &str, id: u32, parent: Option<usize>, position: [f32; 3]) -> Bone {
        Bone {
            name: name.into(),
            controller_id: id,
            parent,
            position,
            rotation: IDENTITY,
            local_position: position,
            local_rotation: IDENTITY,
        }
    }

    fn two_bones() -> Skeleton {
        Skeleton {
            bones: vec![
                bone("root", 1, None, [0.0; 3]),
                bone("tip", 2, Some(0), [1.0, 0.0, 0.0]),
            ],
        }
    }

    fn clip(
        additive: bool,
        id: u32,
        rotation: [f32; 4],
        position: Option<[f32; 3]>,
    ) -> AnimationClip {
        AnimationClip {
            name: String::new(),
            fps: 30,
            declared_frames: None,
            additive,
            channels: vec![BoneChannel {
                bone_hash: id,
                rotations: vec![Keyframe {
                    frame: 0,
                    value: rotation,
                }],
                positions: position
                    .into_iter()
                    .map(|value| Keyframe { frame: 0, value })
                    .collect(),
            }],
        }
    }

    #[test]
    fn a_parent_turn_carries_its_children() {
        let mut pose = Pose::new(&two_bones());
        pose.apply(&clip(false, 1, QUARTER_Z, None), 0.0, 1.0);
        assert!(close(pose.model_space()[1].position, [0.0, 1.0, 0.0]));
    }

    #[test]
    fn parents_come_first_whatever_the_file_order() {
        let order = parents_first(&[Some(2), None, Some(1)]);
        assert_eq!(order, vec![1, 2, 0]);
    }

    #[test]
    fn a_parent_cycle_does_not_hang() {
        assert_eq!(parents_first(&[Some(1), Some(0), None]), vec![2]);
    }

    #[test]
    fn additive_clips_stack_on_the_base() {
        let mut pose = Pose::new(&two_bones());
        pose.apply(&clip(false, 2, QUARTER_Z, Some([1.0, 1.0, 0.0])), 0.0, 1.0);
        pose.apply(&clip(true, 2, QUARTER_Z, Some([0.5, 0.0, 0.0])), 0.0, 1.0);
        let tip = pose.local()[1];
        assert!(close(tip.rotation, [0.0, 0.0, 1.0, 0.0]));
        assert!(close(tip.position, [1.5, 1.0, 0.0]));
    }

    #[test]
    fn zero_weight_changes_nothing() {
        let mut pose = Pose::new(&two_bones());
        let before = pose.local().to_vec();
        pose.apply(&clip(false, 1, QUARTER_Z, Some([9.0; 3])), 0.0, 0.0);
        pose.apply(&clip(true, 1, QUARTER_Z, Some([9.0; 3])), 0.0, 0.0);
        assert_eq!(pose.local(), before);
    }

    #[test]
    fn unnamed_bones_are_left_alone_and_reset_restores() {
        let mut pose = Pose::new(&two_bones());
        pose.apply(&clip(false, 99, QUARTER_Z, None), 0.0, 1.0);
        assert_eq!(pose.local()[0].rotation, IDENTITY);
        pose.apply(&clip(false, 1, QUARTER_Z, None), 0.0, 1.0);
        pose.reset();
        assert_eq!(pose.local()[0].rotation, IDENTITY);
    }

    #[test]
    fn sampling_interpolates_and_holds_the_ends() {
        let channel = BoneChannel {
            bone_hash: 0,
            rotations: vec![],
            positions: vec![
                Keyframe {
                    frame: 10,
                    value: [0.0, 0.0, 0.0],
                },
                Keyframe {
                    frame: 20,
                    value: [10.0, 0.0, 0.0],
                },
            ],
        };
        assert_eq!(channel.position_at(0.0), Some([0.0, 0.0, 0.0]));
        assert_eq!(channel.position_at(15.0), Some([5.0, 0.0, 0.0]));
        assert_eq!(channel.position_at(25.0), Some([10.0, 0.0, 0.0]));
        assert_eq!(channel.rotation_at(15.0), None);
    }

    #[test]
    fn rotations_take_the_short_way_round() {
        let flipped = QUARTER_Z.map(|v| -v);
        let mid = nlerp(IDENTITY, flipped, 0.5);
        let eighth = (std::f32::consts::PI / 8.0).sin();
        assert!(close([mid[2].abs()], [eighth]));
    }
}
