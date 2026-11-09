//! Re-proves [`sc_extract::AnimationClips`] on LIVE 4.10.2.
//!
//! The base standing pose must reproduce the clip style-citizen.com plays as
//! its idle (`anim/stand_male.json`) to the digit, and the male skeleton its
//! bind pose. Then every human `.dba` and `.caf` is decoded: what the reader
//! refuses is listed (5 files use rotation format 0x83, which no reference
//! documents; they are refused rather than guessed).
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
    check(
        "only the 5 known files are refused",
        census.refused.values().map(Vec::len).sum::<usize>() == 5,
    );

    if failures > 0 {
        return Err(format!("{failures} check(s) failed").into());
    }
    Ok(())
}
