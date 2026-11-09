//! Re-proves [`sc_extract::AnimationClips`] on LIVE 4.10.2.
//!
//! The base standing pose must reproduce the clip style-citizen.com plays as
//! its idle (`anim/stand_male.json`) to the digit, and the male skeleton its
//! bind pose. Then every human `.dba` and `.caf` is decoded: what the reader
//! refuses is listed and must be nothing.
//!
//! Rotation format 0x83 has no reference, so it is checked against itself: the
//! female `zero_gravity.dba` stores a clip in it that the male pack stores in
//! the 48-bit format, and only the right component order lines the two up.
//!
//! ```bash
//! cargo run -p sc-extract --release --example animation_clips_probe
//! ```

use std::collections::{BTreeMap, HashSet};

use sc_extract::{AnimationClip, AnimationClips, AssetSource, Skeleton};

const SKELETON: &str = "objects/characters/human/male_v7/export/bhm_skeleton_v7.chr";
const IDLE: &str = "Animations/Characters/Human/male_v7/locomotion_sets/noweapon/nw_neutral_stand_idle_base_static.caf";

fn near(got: &[f32], want: &[f32]) -> bool {
    got.len() == want.len() && got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-6)
}

/// `(rotation keys, position keys, sum of absolute values)`.
fn totals(clip: &AnimationClip) -> (usize, usize, f64) {
    let (mut rotations, mut positions, mut sum) = (0, 0, 0f64);
    for channel in &clip.channels {
        rotations += channel.rotations.len();
        positions += channel.positions.len();
        let r = channel.rotations.iter().flat_map(|k| k.value);
        let p = channel.positions.iter().flat_map(|k| k.value);
        sum += r.chain(p).map(|v| f64::from(v.abs())).sum::<f64>();
    }
    (rotations, positions, sum)
}

#[derive(Default)]
struct Census {
    clips: usize,
    male_channels: usize,
    joined: usize,
    past_declared: usize,
    refused: BTreeMap<String, Vec<String>>,
}

fn angle(a: [f32; 4], b: [f32; 4]) -> f64 {
    let dot: f32 = a.iter().zip(&b).map(|(x, y)| x * y).sum();
    2.0 * f64::from(dot.abs().min(1.0)).acos().to_degrees()
}

/// The quaternion with its first two stored components exchanged.
fn swapped(mut q: [f32; 4]) -> [f32; 4] {
    let largest = (0..4)
        .max_by(|&a, &b| q[a].abs().total_cmp(&q[b].abs()))
        .expect("four components");
    let stored: Vec<usize> = (0..4).filter(|&i| i != largest).collect();
    q.swap(stored[0], stored[1]);
    q
}

/// Per clip block of a `.dba`, the bones whose rotations use format 0x83. The
/// reader does not expose key formats, so this walks the block headers itself.
fn channels_in_format_83(file: &[u8]) -> Vec<HashSet<u32>> {
    let word16 = |d: &[u8], at: usize| u16::from_le_bytes([d[at], d[at + 1]]);
    let word32 = |d: &[u8], at: usize| u32::from_le_bytes([d[at], d[at + 1], d[at + 2], d[at + 3]]);
    let (count, table) = (word32(file, 8) as usize, word32(file, 12) as usize);
    let data = (0..count)
        .map(|i| table + i * 16)
        .find(|&entry| word32(file, entry) == 0x194F_BC50)
        .map_or(0, |entry| word32(file, entry + 8) as usize);
    let d = &file[data..];
    let mut blocks = Vec::new();
    let mut at = 4;
    while d.get(at..at + 4) == Some(b"#dba".as_slice()) {
        let bones = usize::from(word16(d, at + 4));
        let controllers = at + 12 + bones * 4;
        blocks.push(
            (0..bones)
                .filter(|i| {
                    let controller = controllers + i * 24;
                    word16(d, controller) > 0 && word16(d, controller + 2) >> 8 == 0x83
                })
                .map(|i| word32(d, at + 12 + i * 4))
                .collect(),
        );
        at = controllers + bones * 24;
    }
    blocks
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let mut failures = 0;
    let mut check = |what: &str, ok: bool| {
        failures += usize::from(!ok);
        println!("  {} {what}", if ok { "ok      " } else { "MISMATCH" });
    };

    println!("\nskeleton against the site's dump");
    let skeleton = Skeleton::read(&assets, SKELETON)?;
    let ids: HashSet<u32> = skeleton.bones.iter().map(|b| b.controller_id).collect();
    check("220 bones", skeleton.bones.len() == 220);
    let head = skeleton.bone("Head").ok_or("no Head bone")?;
    check(
        "Head local position",
        near(&head.local_position, &[0.071_329_47, -1.862_645_1e-8, 0.0]),
    );
    check(
        "Head local rotation",
        near(
            &head.local_rotation,
            &[-7.264_316e-8, 5.960_464_5e-8, -0.017_375_365, 0.999_848_96],
        ),
    );
    check(
        "Head model position",
        near(
            &head.position,
            &[3.069_475_2e-10, -0.000_842_098_8, 1.705_663],
        ),
    );

    println!("\n{IDLE}");
    let pack = AnimationClips::read(&assets, IDLE)?;
    let idle = pack.clips.first().ok_or("no clip")?;
    let (rotations, positions, sum) = totals(idle);
    check(
        "30 fps, 112 channels",
        idle.fps == 30 && idle.channels.len() == 112,
    );
    check("1461 rotation keys", rotations == 1461);
    check("80 position keys", positions == 80);
    check("absolute sum 2209.9461", (sum - 2209.9461).abs() < 1e-3);
    let first = &idle.channels[0];
    check(
        "first channel is LeftHand_WADIKTarget",
        skeleton
            .bones
            .iter()
            .find(|b| b.controller_id == first.bone_hash)
            .is_some_and(|b| b.name == "LeftHand_WADIKTarget"),
    );
    check(
        "its keys sit on frames 0, 20, 40, 60",
        first.rotations.iter().map(|k| k.frame).eq([0, 20, 40, 60]),
    );
    check(
        "its first rotation",
        near(
            &first.rotations[0].value,
            &[0.458_063_66, 0.620_445_4, -0.617_551_3, -0.154_452_5],
        ),
    );

    let entries: Vec<_> = assets
        .find(|name| {
            let lower = name.to_ascii_lowercase().replace('\\', "/");
            lower.contains("animations/characters/human/")
                && (lower.ends_with(".dba") || lower.ends_with(".caf"))
        })
        .collect();
    let mut census = Census::default();
    for entry in &entries {
        let parsed = assets
            .read_entry(entry)
            .and_then(|bytes| AnimationClips::parse(&bytes));
        let pack = match parsed {
            Ok(pack) => pack,
            Err(e) => {
                census
                    .refused
                    .entry(e.to_string())
                    .or_default()
                    .push(entry.name.to_string());
                continue;
            }
        };
        let male = entry.name.to_ascii_lowercase().contains("male_v7");
        for clip in &pack.clips {
            census.clips += 1;
            if male {
                census.male_channels += clip.channels.len();
                census.joined += clip
                    .channels
                    .iter()
                    .filter(|c| ids.contains(&c.bone_hash))
                    .count();
            }
            let last = clip
                .channels
                .iter()
                .flat_map(|c| {
                    let r = c.rotations.iter().map(|k| k.frame);
                    r.chain(c.positions.iter().map(|k| k.frame))
                })
                .max()
                .map_or(0, u32::from);
            census.past_declared += usize::from(clip.declared_frames.is_some_and(|d| last > d));
        }
    }

    println!("\n{} files, {} clips", entries.len(), census.clips);
    println!(
        "male channels joining the skeleton: {} of {}",
        census.joined, census.male_channels
    );
    println!(
        "clips with a key past their declared frames: {}",
        census.past_declared
    );
    for (error, files) in &census.refused {
        println!("refused, {error}:");
        for file in files {
            println!("  {file}");
        }
    }
    check("no key past its declared frames", census.past_declared == 0);
    check("nothing is refused", census.refused.is_empty());

    println!();
    println!("rotation format 0x83 against the male pack's 48-bit clip");
    let female = assets.read_data("animations/characters/human/female_v2/zero_gravity.dba")?;
    let stored = AnimationClips::parse(&female)?;
    let male = AnimationClips::read(
        &assets,
        "animations/characters/human/male_v7/zero_gravity.dba",
    )?;
    let (mut same, mut swap, mut keys) = (0f64, 0f64, 0u64);
    for (clip, hashes) in stored.clips.iter().zip(channels_in_format_83(&female)) {
        let stem = clip
            .name
            .rsplit('/')
            .next()
            .and_then(|f| f.strip_suffix(".caf"));
        let Some(twin) = stem.and_then(|s| male.clip(s)) else {
            continue;
        };
        for channel in clip
            .channels
            .iter()
            .filter(|c| hashes.contains(&c.bone_hash))
        {
            let Some(other) = twin
                .channels
                .iter()
                .find(|c| c.bone_hash == channel.bone_hash)
            else {
                continue;
            };
            for key in &channel.rotations {
                if let Some(q) = other.rotation_at(f32::from(key.frame)) {
                    same += angle(key.value, q);
                    swap += angle(swapped(key.value), q);
                    keys += 1;
                }
            }
        }
    }
    let (same, swap) = (same / keys.max(1) as f64, swap / keys.max(1) as f64);
    println!(
        "  {keys} keys; mean angle to the twin {same:.3} deg, first two components swapped {swap:.3} deg"
    );
    check(
        "the stored component order lines up",
        keys > 0 && same < 10.0 && swap > 20.0,
    );

    if failures > 0 {
        return Err(format!("{failures} check(s) failed").into());
    }
    Ok(())
}
