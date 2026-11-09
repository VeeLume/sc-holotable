//! Re-proves [`sc_items_visuals::Garments`] on LIVE 4.10.2: the client's layer
//! rule on real outfits, and the join down to the body mesh's own pieces.
//!
//! No game reference exists for an outfit, so the expected counts were reached
//! by reading the client and running the rule, and are held here against drift.
//!
//! ```bash
//! cargo run -p sc-items-visuals --release --example garments_probe
//! ```

use std::collections::{BTreeMap, HashMap};

use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, Guid, MeshGeometry, RecordCollection,
};
use sc_items::Items;
use sc_items_visuals::{Garments, ZoneState};

const BODY: &str = "m_body_01";
const UNDERSUIT: &str = "rsi_odyssey_undersuit_01_01_01";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let items = Items::build(datacore.records());
    let garments = Garments::build(&datacore, &items);
    let mut failures = 0;
    let mut check = |what: &str, ok: bool| {
        failures += usize::from(!ok);
        println!("  {} {what}", if ok { "ok      " } else { "MISMATCH" });
    };

    let mut by_name: HashMap<String, Guid> = HashMap::new();
    for record in datacore.db().all_records() {
        let name = record.name().unwrap_or_default();
        let short = name.split_once('.').map_or(name, |(_, n)| n);
        by_name.insert(short.to_ascii_lowercase(), record.id());
    }

    let mut usage: BTreeMap<(i32, Vec<i32>), usize> = BTreeMap::new();
    let (mut zones, mut conditional, mut first_person) = (0, 0, 0);
    for garment in garments.values() {
        for zone in &garment.zones {
            zones += 1;
            conditional += usize::from(!zone.conditions.is_empty());
            first_person += usize::from(zone.hide_in_first_person);
            *usage
                .entry((zone.layer, zone.visible_layers.clone()))
                .or_default() += 1;
        }
    }
    println!(
        "{} garments, {zones} zones, {conditional} conditional, {first_person} hidden in first person",
        garments.len()
    );
    for ((layer, shows), count) in &usage {
        println!("  layer {layer}, shows {shows:?}: {count}");
    }
    check(
        "VisibleLayers only ever keeps the body",
        usage
            .keys()
            .all(|(_, shows)| shows.is_empty() || shows == &[0]),
    );

    for (outfit, expected) in [
        (vec![BODY, UNDERSUIT], vec![(BODY, 32), (UNDERSUIT, 0)]),
        (
            vec![
                BODY,
                UNDERSUIT,
                "cds_legacy_armor_medium_core_01_01_03",
                "outlaw_legacy_armor_light_legs_01_05_18",
                "fta_medium_helmet_01_01_12",
            ],
            vec![(BODY, 32), (UNDERSUIT, 3)],
        ),
        (
            vec![
                BODY,
                "eld_shirt_04_drake_04",
                "987_jacket_01_01_01",
                "dmc_pants_04_01_19",
            ],
            vec![
                (BODY, 24),
                ("eld_shirt_04_drake_04", 11),
                ("987_jacket_01_01_01", 0),
                ("dmc_pants_04_01_19", 1),
            ],
        ),
    ] {
        let worn: Vec<Guid> = outfit
            .iter()
            .filter_map(|n| by_name.get(*n).copied())
            .collect();
        check(
            &format!("{} items found", outfit.len()),
            worn.len() == outfit.len(),
        );
        let result = garments.outfit(&worn);
        for (name, hidden) in expected {
            let got = by_name
                .get(name)
                .map_or(usize::MAX, |g| result.zones_in(g, ZoneState::Hidden).len());
            check(
                &format!("{name} has {hidden} zones hidden (got {got})"),
                got == hidden,
            );
        }
    }

    println!();
    println!("join to the male body mesh under an undersuit");
    let (body, undersuit) = (by_name[BODY], by_name[UNDERSUIT]);
    let outfit = garments.outfit(&[body, undersuit]);
    let mesh =
        MeshGeometry::read_shape(&assets, "objects/characters/human/male_v7/body/m_body.skin")?;
    let pieces: Vec<u32> = mesh.submeshes.iter().filter_map(|s| s.zone).collect();
    let hidden = pieces
        .iter()
        .filter(|z| outfit.state(&body, **z) == ZoneState::Hidden)
        .count();
    println!("  {hidden} of {} zoned pieces hidden", pieces.len());
    check(
        "every zoned body piece is hidden",
        !pieces.is_empty() && hidden == pieces.len(),
    );

    if failures > 0 {
        return Err(format!("{failures} check(s) failed").into());
    }
    Ok(())
}
