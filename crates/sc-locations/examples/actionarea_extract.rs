//! Extract the mission-location binding fields from every location entity in the
//! station socpaks: `starmapRecord`, `template`, `actionArea`, `locationActionArea`
//! (the last two are STRINGs holding `"<EntityCryGUID>,<per-location GUID>"`), plus
//! the entity Name + EntityCryGUID. Resolve `starmapRecord` -> StarMapObject name.
//!
//! Output TSV (-> ./actionarea.tsv): name, starmap, cryguid, template, actionArea,
//! locationActionArea, socpak. Feed to a hash test of the per-location 2nd GUID.
//!
//! ```bash
//! cargo run -p sc-locations --release --example actionarea_extract
//! ```
use std::collections::BTreeMap;

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap};
use sc_locations::Locations;

fn first_attr<'a>(e: &'a XmlNode, key: &str) -> Option<&'a str> {
    e.descendants()
        .find_map(|n| n.attr(key))
        .filter(|s| !s.is_empty())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let needles = [
        "system/stanton",
        "system/pyro",
        "/loc/mod/",
        "reststop",
        "lagrange",
        "outpost",
        "landingzone",
        "/dc_",
        "distribution",
        "junksite",
        "asteroidbase",
    ];
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let locale: &LocaleMap = &asset_data.locale;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let locs = Locations::build(store);

    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && needles.iter().any(|nd| l.contains(nd))
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("scanning {} location socpaks ...", socpaks.len());

    // name -> row (dedup by resolved name + cryguid)
    let mut rows: BTreeMap<String, String> = BTreeMap::new();
    let mut n_ent = 0usize;
    for (i, sp) in socpaks.iter().enumerate() {
        if i % 500 == 0 {
            eprintln!("  [{i}/{}] {} entities", socpaks.len(), n_ent);
        }
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let nm = pak.name(m).unwrap_or_default().to_ascii_lowercase();
            if !(nm.ends_with(".soc") || nm.ends_with(".pla") || nm.ends_with(".entxml")) {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for e in root.find_all("Entity") {
                let class = e.attr("EntityClass").unwrap_or("");
                if !class.contains("ObjectContainer") {
                    continue;
                }
                let action = first_attr(e, "actionArea");
                let locaction = first_attr(e, "locationActionArea");
                let starmap = first_attr(e, "starmapRecord");
                let template = first_attr(e, "template");
                if action.is_none() && locaction.is_none() {
                    continue; // only entities that actually carry an action area
                }
                n_ent += 1;
                let cry = e.attr("EntityCryGUID").unwrap_or("");
                let name = e.attr("Name").unwrap_or("");
                // Resolve starmapRecord -> StarMapObject display name.
                let resolved = starmap
                    .and_then(|s| s.trim().trim_matches(['{', '}']).parse::<Guid>().ok())
                    .and_then(|g| locs.by_crc(sc_extract::class_crc(&g)).map(|_| g))
                    .and_then(|_g| {
                        starmap
                            .and_then(|s| s.trim().trim_matches(['{', '}']).parse::<Guid>().ok())
                            .and_then(|g| {
                                // by_crc keyed on class_crc(guid); look up the Location and its name
                                let cc = sc_extract::class_crc(&g);
                                locs.by_crc(cc).and_then(|l| l.display_name(locale))
                            })
                    })
                    .unwrap_or("");
                let key = format!("{resolved}|{cry}|{name}");
                rows.entry(key).or_insert_with(|| {
                    format!(
                        "{resolved}\t{}\t{cry}\t{}\t{}\t{}\t{sp}",
                        starmap.unwrap_or(""),
                        template.unwrap_or(""),
                        action.unwrap_or(""),
                        locaction.unwrap_or(""),
                    )
                });
            }
        }
    }

    let mut out =
        String::from("name\tstarmap\tcryguid\ttemplate\tactionArea\tlocationActionArea\tsocpak\n");
    for v in rows.values() {
        out.push_str(v);
        out.push('\n');
    }
    std::fs::write("actionarea.tsv", &out)?;
    eprintln!(
        "\n[+] {} location entities ({} unique rows) -> actionarea.tsv",
        n_ent,
        rows.len()
    );
    // show a few with a resolved name
    let mut shown = 0;
    for v in rows.values() {
        if !v.starts_with('\t') && shown < 15 {
            eprintln!(
                "  {}",
                v.split('\t').take(5).collect::<Vec<_>>().join(" | ")
            );
            shown += 1;
        }
    }
    Ok(())
}
