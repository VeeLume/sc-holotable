//! Where a character's motion lives in the game data (LIVE 4.10.2):
//!
//! - the skeleton's `.chrparams` names its clip packs (`$TracksDatabase`, `.dba`)
//!   and event table (`$AnimEventDatabase`);
//! - a `.dba` lists the `.caf` clips it holds;
//! - Mannequin `.adb` files choose clips by state tags;
//! - `.animevents` times the moments inside a clip;
//! - `ActorStanceSpeedsInfo` records hold the movement speeds.
//!
//! Reference values are the ones style-citizen.com publishes for draw timing
//! and gait speeds; a mismatch fails the run. No clip is decoded here.
//!
//! ```bash
//! cargo run -p sc-extract --release --example character_animation_sources
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, XmlNode, object_container};

const SKELETON: &str = "Objects/Characters/Human/male_v7/export/bhm_skeleton_v7.chrparams";
const IDLE_PACK: &str =
    "Animations/Characters/Human/male_v7/weapons/no_weapon/locomotion/stand.dba";
const EVENTS: &str = "Animations/Characters/Human/male_v7/events.animevents";
const IDLE_ADB: &str = "Animations/Mannequin/ADB/Players/playerIdleAnims.adb";
const EMOTE_ADB: &str = "Animations/Mannequin/ADB/Players/playerEmoteAnims.adb";
const DESELECT: &str = "animations/characters/human/male_v7/weapons/select_deselect_v2/fire_extinguisher/fire_extinguisher_alerted_stand_deselect_01.caf";

fn xml(assets: &AssetSource, path: &str) -> Result<XmlNode, Box<dyn std::error::Error>> {
    let bytes = assets.read_data(path)?;
    object_container::decode(&bytes)?.ok_or_else(|| format!("{path}: no XML").into())
}

fn clip_names(root: &XmlNode) -> Vec<&str> {
    root.find_all("Animation")
        .filter_map(|a| a.attr("name"))
        .collect()
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

    println!("\n{SKELETON}");
    let skeleton = xml(&assets, SKELETON)?;
    let entries: Vec<(&str, &str)> = skeleton
        .find_all("Animation")
        .filter_map(|a| Some((a.attr("name")?, a.attr("path")?)))
        .collect();
    for (name, path) in &entries {
        println!("  {name} -> {path}");
    }
    check(
        "names a clip pack",
        entries.iter().any(|(n, _)| *n == "$TracksDatabase"),
    );
    check(
        "names the event table",
        entries
            .iter()
            .any(|(n, p)| *n == "$AnimEventDatabase" && p.ends_with("events.animevents")),
    );

    println!("\n{IDLE_PACK}");
    let pack = assets.read_data(IDLE_PACK)?;
    let walk = b"nw_neutral_stand_walkslow_forward.caf";
    let breathe = b"nw_neutral_stand_idle_breathe_tp_add.caf";
    check(
        "lists the walk clip",
        pack.windows(walk.len()).any(|w| w == walk),
    );
    check(
        "lists the breathing clip",
        pack.windows(breathe.len()).any(|w| w == breathe),
    );

    println!("\nMannequin");
    let idle = xml(&assets, IDLE_ADB)?;
    let emote = xml(&assets, EMOTE_ADB)?;
    check(
        "idle database picks the fidget clips",
        clip_names(&idle).contains(&"nw_neutral_stand_idlefidget_01"),
    );
    check(
        "emote database picks the thumbs-up clip",
        clip_names(&emote).contains(&"emote_agree_m_stand_thumbsup_generic"),
    );

    println!("\n{EVENTS}");
    let events = xml(&assets, EVENTS)?;
    let clip = events
        .find_all("animation")
        .find(|a| a.attr("name") == Some(DESELECT))
        .ok_or("no event block for the deselect clip")?;
    let time = |event: &str| -> Option<f32> {
        clip.children_named("event")
            .find(|e| e.attr("name") == Some(event))?
            .attr("time")?
            .parse()
            .ok()
    };
    let near = |got: Option<f32>, want: f32| got.is_some_and(|g| (g - want).abs() < 1e-4);
    check(
        "customDetach at the site's swap 0.5953",
        near(time("customDetach"), 0.5953),
    );
    check(
        "deselect at the site's ready 0.7691",
        near(time("deselect"), 0.7691),
    );

    println!("\nActorStanceSpeedsInfo.Stand_NoWeapon");
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let record = datacore
        .db()
        .all_records()
        .find(|r| r.name() == Some("ActorStanceSpeedsInfo.Stand_NoWeapon"))
        .ok_or("no Stand_NoWeapon record")?;
    let instance = record.as_instance();
    let speeds = instance.get_instance("speeds").ok_or("no speeds")?;
    for (field, want) in [
        ("walkSlowSpeed", 0.9),
        ("walkMidSpeed", 1.4),
        ("runSlowSpeed", 3.4),
        ("runFastSpeed", 5.58),
        ("sprintSpeed", 7.2),
    ] {
        check(
            &format!("{field} = {want}"),
            near(speeds.get_f32(field), want),
        );
    }

    if failures > 0 {
        return Err(format!("{failures} check(s) failed").into());
    }
    Ok(())
}
