//! `.dba` and `.caf` animation clips: per-bone rotation and position keys.
//!
//! A `.dba` packs many clips, a `.caf` holds one. Both are IVO files of the
//! same clip blocks: `#dba`/`#caf`, bone count `u16`, a `u16`, size `u32`, one
//! `u32` hash per bone, then one 24-byte controller per bone. A controller
//! gives each of its two tracks a key count, format flags, and time and value
//! offsets relative to the controller itself. Block headers sit back to back,
//! with the keys after them; a `.dba`'s catalogue chunk lists fps, frame count
//! and path per block, in block order. Layout after StarBreaker's reader (MIT).
//!
//! Which clip plays when is not in these files: Mannequin `.adb` databases and
//! the skeleton's `.chrparams` decide that. Keys are as stored, in CryEngine
//! space and quaternions `(x, y, z, w)`, with no blending or interpolation. A
//! channel joins a skeleton through [`Bone::controller_id`](crate::Bone).
//!
//! An unknown key format is an error for the file, because guessing one would
//! return plausible motion that is wrong.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let pack = sc_extract::AnimationClips::read(
//!     &assets,
//!     "animations/characters/human/male_v7/weapons/no_weapon/locomotion/stand.dba",
//! )?;
//! for clip in pack.clips.iter().take(3) {
//!     println!("{}: {} bones at {} fps", clip.name, clip.channels.len(), clip.fps);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::mesh::{chunk, chunk_to_end, read_data};
use crate::{AssetSource, Error, Result};

const DBA_DATA: u32 = 0x194F_BC50;
const DBA_META: u32 = 0xF735_1608;
const CAF_DATA: u32 = 0xA949_6CB5;
const ANIM_INFO: u32 = 0x4733_C6ED;

const CONTROLLER_SIZE: usize = 24;
const CATALOGUE_ENTRY_SIZE: usize = 48;
const DEFAULT_FPS: u16 = 30;
const ADDITIVE_FLAG: u32 = 1;

/// The clips of one `.dba` or `.caf`, in file order.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationClips {
    pub clips: Vec<AnimationClip>,
}

/// One clip: a track pair per animated bone.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationClip {
    /// The `.caf` path a `.dba` lists; for a `.caf`, the path it was read from
    /// (empty through [`AnimationClips::parse`]).
    pub name: String,
    pub fps: u16,
    /// Frame count a `.dba` catalogue declares; `None` for a `.caf`.
    pub declared_frames: Option<u32>,
    /// Stored as a change on top of another pose, not as a pose of its own.
    /// From the file's header, which does not always agree with an `_add` name.
    pub additive: bool,
    pub channels: Vec<BoneChannel>,
}

/// One bone's keys.
#[derive(Debug, Clone, PartialEq)]
pub struct BoneChannel {
    /// Matches [`Bone::controller_id`](crate::Bone) of the skeleton it animates.
    pub bone_hash: u32,
    /// Quaternions `(x, y, z, w)`.
    pub rotations: Vec<Keyframe<[f32; 4]>>,
    pub positions: Vec<Keyframe<[f32; 3]>>,
}

/// A value at a frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keyframe<T> {
    pub frame: u16,
    pub value: T,
}

impl AnimationClips {
    /// Read a `.dba` or `.caf` (with or without a `Data/` prefix).
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        let mut out = Self::parse(&read_data(assets, path)?)?;
        for clip in out.clips.iter_mut().filter(|c| c.name.is_empty()) {
            clip.name = path.to_string();
        }
        Ok(out)
    }

    /// Parse `.dba` or `.caf` bytes.
    pub fn parse(file: &[u8]) -> Result<Self> {
        if let Ok(meta) = chunk(file, DBA_META) {
            return parse_dba(file, meta);
        }
        parse_caf(file)
    }

    /// The clip whose file stem is `stem` (case-insensitive).
    pub fn clip(&self, stem: &str) -> Option<&AnimationClip> {
        self.clips.iter().find(|c| {
            let file = c.name.rsplit(['/', '\\']).next().unwrap_or(&c.name);
            file.strip_suffix(".caf")
                .unwrap_or(file)
                .eq_ignore_ascii_case(stem)
        })
    }
}

fn bad(what: impl std::fmt::Display) -> Error {
    Error::Mesh(format!("animation: {what}"))
}

fn u16_at(d: &[u8], at: usize) -> Result<u16> {
    d.get(at..at + 2)
        .map(|b| u16::from_le_bytes(b.try_into().expect("2 bytes")))
        .ok_or_else(|| bad("truncated"))
}

fn u32_at(d: &[u8], at: usize) -> Result<u32> {
    d.get(at..at + 4)
        .map(|b| u32::from_le_bytes(b.try_into().expect("4 bytes")))
        .ok_or_else(|| bad("truncated"))
}

fn f32_at(d: &[u8], at: usize) -> Result<f32> {
    Ok(f32::from_bits(u32_at(d, at)?))
}

fn vec3_at(d: &[u8], at: usize) -> Result<[f32; 3]> {
    Ok([f32_at(d, at)?, f32_at(d, at + 4)?, f32_at(d, at + 8)?])
}

struct CatalogueEntry {
    name: String,
    fps: u16,
    controllers: u16,
    frames: u32,
    additive: bool,
}

fn parse_dba(file: &[u8], meta: &[u8]) -> Result<AnimationClips> {
    let catalogue = parse_catalogue(meta)?;
    // Controller offsets can reach past the data chunk's end.
    let blocks = parse_blocks(chunk_to_end(file, DBA_DATA)?)?;
    if blocks.len() != catalogue.len() {
        return Err(bad(format!(
            "{} blocks but {} catalogue entries",
            blocks.len(),
            catalogue.len()
        )));
    }
    let clips = catalogue
        .into_iter()
        .zip(blocks)
        .map(|(entry, channels)| {
            if usize::from(entry.controllers) != channels.len() {
                return Err(bad(format!(
                    "{}: catalogue says {} controllers, block has {}",
                    entry.name,
                    entry.controllers,
                    channels.len()
                )));
            }
            Ok(AnimationClip {
                name: entry.name,
                fps: if entry.fps == 0 {
                    DEFAULT_FPS
                } else {
                    entry.fps
                },
                declared_frames: Some(entry.frames),
                additive: entry.additive,
                channels,
            })
        })
        .collect::<Result<_>>()?;
    Ok(AnimationClips { clips })
}

fn parse_caf(file: &[u8]) -> Result<AnimationClips> {
    let fps = chunk(file, ANIM_INFO)
        .ok()
        .and_then(|d| u16_at(d, 4).ok())
        .filter(|&f| f != 0)
        .unwrap_or(DEFAULT_FPS);
    let additive = chunk(file, ANIM_INFO)
        .ok()
        .and_then(|d| u32_at(d, 0).ok())
        .is_some_and(|flags| flags & ADDITIVE_FLAG != 0);
    let clips = parse_blocks(chunk_to_end(file, CAF_DATA)?)?
        .into_iter()
        .map(|channels| AnimationClip {
            name: String::new(),
            fps,
            declared_frames: None,
            additive,
            channels,
        })
        .collect();
    Ok(AnimationClips { clips })
}

/// Count, 48-byte entries, padding NULs, then one NUL-terminated path each.
fn parse_catalogue(d: &[u8]) -> Result<Vec<CatalogueEntry>> {
    let count = u32_at(d, 0)? as usize;
    let names_from = count
        .checked_mul(CATALOGUE_ENTRY_SIZE)
        .and_then(|n| n.checked_add(4))
        .filter(|&end| end <= d.len())
        .ok_or_else(|| bad("catalogue larger than its chunk"))?;
    let mut names = d[names_from..]
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).into_owned());
    (0..count)
        .map(|i| {
            let e = 4 + i * CATALOGUE_ENTRY_SIZE;
            Ok(CatalogueEntry {
                name: names.next().ok_or_else(|| bad("fewer names than clips"))?,
                fps: u16_at(d, e + 8)?,
                controllers: u16_at(d, e + 10)?,
                frames: u32_at(d, e + 20)?,
                additive: u32_at(d, e + 4)? & ADDITIVE_FLAG != 0,
            })
        })
        .collect()
}

fn parse_blocks(d: &[u8]) -> Result<Vec<Vec<BoneChannel>>> {
    // A `.dba`'s data starts with its total size, a `.caf`'s with a signature.
    let mut at = if is_block(d, 0) { 0 } else { 4 };
    let mut blocks = Vec::new();
    while is_block(d, at) {
        let bones = usize::from(u16_at(d, at + 4)?);
        let hashes = at + 12;
        let controllers = hashes + bones * 4;
        let channels = (0..bones)
            .map(|i| {
                parse_channel(
                    d,
                    u32_at(d, hashes + i * 4)?,
                    controllers + i * CONTROLLER_SIZE,
                )
            })
            .collect::<Result<_>>()?;
        blocks.push(channels);
        at = controllers + bones * CONTROLLER_SIZE;
    }
    Ok(blocks)
}

fn is_block(d: &[u8], at: usize) -> bool {
    matches!(d.get(at..at + 4), Some(b"#dba" | b"#caf"))
}

fn parse_channel(d: &[u8], bone_hash: u32, base: usize) -> Result<BoneChannel> {
    let track = |at: usize| -> Result<(usize, u16, usize, usize)> {
        Ok((
            usize::from(u16_at(d, at)?),
            u16_at(d, at + 2)?,
            u32_at(d, at + 4)? as usize,
            u32_at(d, at + 8)? as usize,
        ))
    };
    let (n_rot, rot_flags, rot_time, rot_data) = track(base)?;
    let (n_pos, pos_flags, pos_time, pos_data) = track(base + 12)?;

    let mut rotations = Vec::with_capacity(n_rot);
    if n_rot > 0 {
        let frames = read_frames(d, base, rot_time, n_rot, rot_flags)?;
        let values = read_rotations(d, base + rot_data, n_rot, rot_flags)?;
        rotations.extend(
            frames
                .into_iter()
                .zip(values)
                .map(|(frame, value)| Keyframe { frame, value }),
        );
    }
    let mut positions = Vec::with_capacity(n_pos);
    if n_pos > 0 {
        let frames = read_frames(d, base, pos_time, n_pos, pos_flags)?;
        let values = read_positions(d, base + pos_data, n_pos, pos_flags)?;
        positions.extend(
            frames
                .into_iter()
                .zip(values)
                .map(|(frame, value)| Keyframe { frame, value }),
        );
    }
    Ok(BoneChannel {
        bone_hash,
        rotations,
        positions,
    })
}

/// Frame of each key. A time offset of 0 means the keys are consecutive.
fn read_frames(d: &[u8], base: usize, offset: usize, n: usize, flags: u16) -> Result<Vec<u16>> {
    if offset == 0 {
        return (0..n)
            .map(|i| u16::try_from(i).map_err(|_| bad("too many keys")))
            .collect();
    }
    let at = base + offset;
    match flags & 0x0F {
        0 => Ok(d
            .get(at..at + n)
            .ok_or_else(|| bad("truncated"))?
            .iter()
            .map(|&b| u16::from(b))
            .collect()),
        1 => (0..n).map(|i| u16_at(d, at + i * 2)).collect(),
        2 => read_frame_bitmap(d, at, n),
        other => Err(bad(format!("unknown time format {other:#x}"))),
    }
}

/// First frame, last frame, then one bit per frame, LSB first.
fn read_frame_bitmap(d: &[u8], at: usize, n: usize) -> Result<Vec<u16>> {
    let start = u16_at(d, at)?;
    let end = u16_at(d, at + 2)?;
    let bits = usize::from(
        end.checked_sub(start)
            .ok_or_else(|| bad("frame bitmap ends before it starts"))?,
    ) + 1;
    let map = d
        .get(at + 4..at + 4 + bits.div_ceil(8))
        .ok_or_else(|| bad("truncated"))?;
    let frames: Vec<u16> = (0..bits)
        .filter(|i| map[i / 8] >> (i % 8) & 1 == 1)
        .map(|i| start + i as u16)
        .collect();
    if frames.len() != n {
        return Err(bad(format!(
            "frame bitmap has {} keys, track has {n}",
            frames.len()
        )));
    }
    Ok(frames)
}

fn read_rotations(d: &[u8], at: usize, n: usize, flags: u16) -> Result<Vec<[f32; 4]>> {
    match flags >> 8 {
        0x80 => (0..n)
            .map(|i| {
                let o = at + i * 16;
                Ok([
                    f32_at(d, o)?,
                    f32_at(d, o + 4)?,
                    f32_at(d, o + 8)?,
                    f32_at(d, o + 12)?,
                ])
            })
            .collect(),
        0x82 => (0..n)
            .map(|i| {
                let o = at + i * 6;
                Ok(small_tree_quat(
                    u16_at(d, o)?,
                    u16_at(d, o + 2)?,
                    u16_at(d, o + 4)?,
                ))
            })
            .collect(),
        other => Err(bad(format!("unknown rotation format {other:#x}"))),
    }
}

/// Three 15-bit components crossing the `u16` borders, and two bits naming the
/// largest component, which is rebuilt from unit length.
fn small_tree_quat(s0: u16, s1: u16, s2: u16) -> [f32; 4] {
    const SCALE: f32 = 1.0 / 23170.0;
    const RANGE: f32 = std::f32::consts::FRAC_1_SQRT_2;
    let component = |bits: u32| (bits & 0x7FFF) as f32 * SCALE - RANGE;

    let a = component(u32::from(s0));
    let b = component(u32::from(s1) * 2 + u32::from(s0 >> 15));
    let c = component(u32::from(s1 >> 14).wrapping_add((s2 as i16 as u32).wrapping_mul(4)));
    let largest = (1.0 - a * a - b * b - c * c).max(0.0).sqrt();

    let dropped = usize::from(s2 >> 14);
    let mut q = [a, b, c, 0.0];
    q.copy_within(dropped..3, dropped + 1);
    q[dropped] = largest;
    q
}

fn read_positions(d: &[u8], at: usize, n: usize, flags: u16) -> Result<Vec<[f32; 3]>> {
    match flags >> 8 {
        0xC0 => (0..n).map(|i| vec3_at(d, at + i * 12)).collect(),
        0xC1 => {
            let (scale, offset) = (vec3_at(d, at)?, vec3_at(d, at + 12)?);
            (0..n)
                .map(|i| {
                    let o = at + 24 + i * 6;
                    let mut p = [0.0; 3];
                    for (axis, v) in p.iter_mut().enumerate() {
                        *v = f32::from(u16_at(d, o + axis * 2)?) * scale[axis] + offset[axis];
                    }
                    Ok(p)
                })
                .collect()
        }
        0xC2 => read_planar_positions(d, at, n),
        other => Err(bad(format!("unknown position format {other:#x}"))),
    }
}

/// Scale and offset per axis, then one `u16` run per axis that moves. An axis
/// with a scale of about `f32::MAX` has no run and stays at its offset.
fn read_planar_positions(d: &[u8], at: usize, n: usize) -> Result<Vec<[f32; 3]>> {
    const CONSTANT_AXIS: f32 = 3.0e38;
    let (scale, offset) = (vec3_at(d, at)?, vec3_at(d, at + 12)?);
    let mut run = [None; 3];
    let mut next = at + 24;
    for axis in 0..3 {
        if scale[axis].abs() < CONSTANT_AXIS {
            run[axis] = Some(next);
            next += n * 2;
        }
    }
    (0..n)
        .map(|i| {
            let mut p = offset;
            for axis in 0..3 {
                if let Some(start) = run[axis] {
                    p[axis] = f32::from(u16_at(d, start + i * 2)?) * scale[axis] + offset[axis];
                }
            }
            Ok(p)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_tree_rebuilds_the_dropped_component() {
        for (top, slot) in [(0x0000u16, 0), (0x4000, 1), (0x8000, 2), (0xC000, 3)] {
            let q = small_tree_quat(0x4000, 0x2000, top | 0x1000);
            for (i, v) in q.iter().enumerate() {
                let want = if i == slot { 1.0 } else { 0.0 };
                assert!((v - want).abs() < 1e-3, "slot {slot}: {q:?}");
            }
        }
    }

    #[test]
    fn bitmap_keeps_uneven_frames() {
        let mut d = Vec::new();
        d.extend_from_slice(&2u16.to_le_bytes());
        d.extend_from_slice(&11u16.to_le_bytes());
        d.extend_from_slice(&[0b0000_0101, 0b0000_0010]);
        assert_eq!(read_frame_bitmap(&d, 0, 3).unwrap(), vec![2, 4, 11]);
        assert!(read_frame_bitmap(&d, 0, 4).is_err());
    }

    #[test]
    fn planar_positions_skip_constant_axes() {
        let mut d = Vec::new();
        for v in [f32::MAX, 2.0, 1.0, 7.0, 10.0, 20.0] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        for v in [1u16, 2, 3, 4] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        assert_eq!(
            read_planar_positions(&d, 0, 2).unwrap(),
            vec![[7.0, 12.0, 23.0], [7.0, 14.0, 24.0]]
        );
    }

    #[test]
    fn unknown_formats_are_errors() {
        let d = [0u8; 64];
        assert!(read_rotations(&d, 0, 1, 0x8100).is_err());
        assert!(read_positions(&d, 0, 1, 0xC300).is_err());
        assert!(read_frames(&d, 0, 4, 1, 0x0003).is_err());
    }

    #[test]
    fn clip_block_routes_keys_to_their_bone() {
        let mut d = Vec::new();
        d.extend_from_slice(b"#caf");
        d.extend_from_slice(&1u16.to_le_bytes());
        d.extend_from_slice(&0xAA55u16.to_le_bytes());
        d.extend_from_slice(&0u32.to_le_bytes());
        d.extend_from_slice(&0x1234u32.to_le_bytes());
        // Controller: no rotations, one float position right after it.
        d.extend_from_slice(&0u16.to_le_bytes());
        d.extend_from_slice(&0u16.to_le_bytes());
        d.extend_from_slice(&[0; 8]);
        d.extend_from_slice(&1u16.to_le_bytes());
        d.extend_from_slice(&0xC000u16.to_le_bytes());
        d.extend_from_slice(&0u32.to_le_bytes());
        d.extend_from_slice(&24u32.to_le_bytes());
        for v in [1.0f32, 2.0, 3.0] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        let blocks = parse_blocks(&d).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0][0].bone_hash, 0x1234);
        assert_eq!(
            blocks[0][0].positions,
            vec![Keyframe {
                frame: 0,
                value: [1.0, 2.0, 3.0]
            }]
        );
    }
}
