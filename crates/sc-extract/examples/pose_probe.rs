//! Re-proves [`sc_extract::Pose`] on LIVE 4.10.2: forward kinematics against
//! the skeleton's own model-space bind pose, and the additive flag against how
//! flagged clips actually behave.
//!
//! Also measures the composition order. Where a clip and its additive version
//! both exist, `full = add * base` leaves `add^-1 * full` constant over time,
//! and `full = base * add` leaves `full * add^-1` constant. Finally the
//! standing pose must match forward kinematics run on style-citizen.com's data.
//!
//! ```bash
//! cargo run -p sc-extract --release --example pose_probe
//! ```

use sc_extract::{AnimationClip, AnimationClips, AssetSource, Pose, Skeleton};

const SKELETON: &str = "objects/characters/human/male_v7/export/bhm_skeleton_v7.chr";
const IDLE: &str = "animations/characters/human/male_v7/locomotion_sets/noweapon/nw_neutral_stand_idle_base_static.caf";
const SELECT_DESELECT: &str = "animations/characters/human/male_v7/weapons/select_deselect_v2";

fn inverse(q: [f32; 4]) -> [f32; 4] {
    [-q[0], -q[1], -q[2], q[3]]
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

/// Angle of the rotation between two quaternions, from its vector part: `acos`
/// of a dot product near 1 has no precision left at `f32`.
fn angle_between(a: [f32; 4], b: [f32; 4]) -> f64 {
    let [x, y, z, _] = multiply(inverse(a), b).map(f64::from);
    2.0 * (x * x + y * y + z * z).sqrt().min(1.0).asin().to_degrees()
}

/// Mean over bones of the largest angle `combine(full, add)` strays from its
/// value at the first frame.
fn drift(
    full: &AnimationClip,
    add: &AnimationClip,
    combine: impl Fn([f32; 4], [f32; 4]) -> [f32; 4],
) -> (f64, usize) {
    let (mut total, mut bones) = (0f64, 0usize);
    for channel in &full.channels {
        let Some(other) = add
            .channels
            .iter()
            .find(|c| c.bone_hash == channel.bone_hash)
        else {
            continue;
        };
        if channel.rotations.len() < 8 || other.rotations.len() < 8 {
            continue;
        }
        let at = |frame: f32| -> Option<[f32; 4]> {
            Some(combine(
                channel.rotation_at(frame)?,
                other.rotation_at(frame)?,
            ))
        };
        let Some(start) = at(0.0) else { continue };
        let last = full.last_frame().min(add.last_frame());
        let worst = (0..=last)
            .filter_map(|f| at(f32::from(f)))
            .map(|q| angle_between(q, start))
            .fold(0f64, f64::max);
        total += worst;
        bones += 1;
    }
    (total / bones.max(1) as f64, bones)
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

    println!("\nforward kinematics of the bind pose");
    let skeleton = Skeleton::read(&assets, SKELETON)?;
    let bind = Pose::new(&skeleton).model_space();
    let (mut worst_position, mut worst_angle) = (0f32, 0f64);
    for (bone, model) in skeleton.bones.iter().zip(&bind) {
        for axis in 0..3 {
            worst_position = worst_position.max((bone.position[axis] - model.position[axis]).abs());
        }
        worst_angle = worst_angle.max(angle_between(bone.rotation, model.rotation));
    }
    println!("  worst position error {worst_position} m, worst rotation error {worst_angle} deg");
    check("every bone within 0.1 mm", worst_position < 1e-4);
    check("every bone within 0.1 deg", worst_angle < 0.1);

    println!("\nadditive flag over human clips");
    let entries: Vec<_> = assets
        .find(|name| {
            let lower = name.to_ascii_lowercase().replace(char::from(92u8), "/");
            lower.contains("animations/characters/human/")
                && (lower.ends_with(".dba") || lower.ends_with(".caf"))
        })
        .collect();
    let mut still = [[0u64; 2]; 2];
    for entry in &entries {
        let Ok(pack) = assets
            .read_entry(entry)
            .and_then(|bytes| AnimationClips::parse(&bytes))
        else {
            continue;
        };
        for clip in &pack.clips {
            let row = &mut still[usize::from(clip.additive)];
            for channel in &clip.channels {
                if let Some(k) = channel.positions.first() {
                    row[1] += 1;
                    row[0] += u64::from(k.value.iter().all(|v| v.abs() < 0.005));
                }
            }
        }
    }
    let share = |row: [u64; 2]| 100.0 * row[0] as f64 / row[1].max(1) as f64;
    println!(
        "  positions starting at zero: flagged {:.1}%, unflagged {:.1}%",
        share(still[1]),
        share(still[0])
    );
    check("flagged clips start at zero", share(still[1]) > 90.0);
    check("unflagged clips do not", share(still[0]) < 50.0);

    println!("\ncomposition order, on clip and additive pairs");
    let sep = char::from(92u8);
    let adds: Vec<String> = assets
        .find(|name| {
            let lower = name.to_ascii_lowercase().replace(char::from(92u8), "/");
            lower.contains(SELECT_DESELECT) && lower.ends_with("_add.caf")
        })
        .map(|e| e.name.replace(sep, "/"))
        .collect();
    let (mut front, mut back, mut pairs) = (0f64, 0f64, 0usize);
    for add_path in &adds {
        let full_path = add_path.replace("_add.caf", ".caf");
        let (Ok(add), Ok(full)) = (
            AnimationClips::read(&assets, add_path),
            AnimationClips::read(&assets, &full_path),
        ) else {
            continue;
        };
        let (Some(add), Some(full)) = (add.clips.first(), full.clips.first()) else {
            continue;
        };
        if !add.additive || full.additive {
            continue;
        }
        let (in_front, bones) = drift(full, add, |f, a| multiply(inverse(a), f));
        let (behind, _) = drift(full, add, |f, a| multiply(f, inverse(a)));
        if bones > 0 {
            front += in_front;
            back += behind;
            pairs += 1;
        }
    }
    let (front, back) = (front / pairs.max(1) as f64, back / pairs.max(1) as f64);
    println!("  {pairs} pairs; mean drift, add in front {front:.3} deg, behind {back:.3} deg");
    check("an additive clip goes in front", front < 0.1 && back > 10.0);

    println!("\nstanding pose at frame 0 against the site's data");
    let idle = AnimationClips::read(&assets, IDLE)?;
    let mut pose = Pose::new(&skeleton);
    pose.apply(&idle.clips[0], 0.0, 1.0);
    let model = pose.model_space();
    for (name, want) in [
        ("Hips", [0.0, 0.0, 0.997_681]),
        ("Head", [0.009_255, -0.022_402, 1.700_852]),
        ("LeftHand", [-0.306_625, 0.090_61, 0.961_03]),
        ("RightHand", [0.301_831, 0.043_268, 0.926_199]),
        ("LeftFoot", [-0.238_831, -0.042_669, 0.099_523]),
        ("RightToeBase", [0.181_744, 0.009_526, 0.030_893]),
    ] {
        let position = skeleton
            .bones
            .iter()
            .position(|b| b.name == name)
            .map(|i| model[i].position);
        let ok = position.is_some_and(|p| p.iter().zip(&want).all(|(a, b)| (a - b).abs() < 2e-6));
        check(&format!("{name} at {want:?}"), ok);
    }

    if failures > 0 {
        return Err(format!("{failures} check(s) failed").into());
    }
    Ok(())
}
