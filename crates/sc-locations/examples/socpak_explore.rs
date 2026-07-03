//! Explore a socpak's internal structure: members, entity-class histogram, all
//! component tags + attribute keys (to surface undecoded data), and the
//! OrbitingObjectContainer (starmapRecord, objectContainer) bridge pairs.
//!
//! ```bash
//! cargo run -p sc-locations --release --example socpak_explore -- stantonsystem
//! cargo run -p sc-locations --release --example socpak_explore -- sunsetmesa --members
//! ```
use std::collections::BTreeMap;

use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, XmlNode, decode};

fn is_entity_file(n: &str) -> bool {
    let n = n.to_ascii_lowercase();
    n.ends_with(".soc") || n.ends_with(".pla") || n.ends_with(".entxml") || n.ends_with("pivot.xml")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let needle = args.first().expect("needle").to_lowercase();
    let show_members = args.iter().any(|a| a == "--members");
    let max_paks: usize = args
        .iter()
        .find_map(|a| a.strip_prefix("--max=").and_then(|s| s.parse().ok()))
        .unwrap_or(2);
    let dump_tag: Option<String> = args
        .iter()
        .find_map(|a| a.strip_prefix("--dump=").map(|s| s.to_string()));
    let find_guid: Option<String> = args
        .iter()
        .find_map(|a| a.strip_prefix("--findguid=").map(|s| s.to_lowercase()));
    let dump_n: usize = args
        .iter()
        .find_map(|a| a.strip_prefix("--dumpn=").and_then(|s| s.parse().ok()))
        .unwrap_or(4);

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;

    let entries: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase();
            l.ends_with(".socpak") && l.contains(&needle)
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("matched {} socpaks for {needle:?}", entries.len());
    for e in entries.iter().take(8) {
        eprintln!("  - {e}");
    }

    for path in entries.iter().take(max_paks) {
        let bytes = assets.read(path)?;
        let mut pak = match Socpak::open(bytes) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("open {path}: {e}");
                continue;
            }
        };
        println!(
            "\n================ {path}  ({} members) ================",
            pak.len()
        );

        let mut ext_hist: BTreeMap<String, usize> = BTreeMap::new();
        let mut entity_members = Vec::new();
        for i in 0..pak.len() {
            let name = pak.name(i).unwrap_or_default();
            let ext = name.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
            *ext_hist.entry(ext).or_default() += 1;
            if show_members {
                println!("  [{i}] {name}");
            }
            if is_entity_file(&name) {
                entity_members.push((i, name));
            }
        }
        println!("member extensions: {ext_hist:?}");
        println!("entity-bearing members: {}", entity_members.len());

        let mut class_hist: BTreeMap<String, usize> = BTreeMap::new();
        let mut tag_hist: BTreeMap<String, usize> = BTreeMap::new();
        let mut attr_hist: BTreeMap<String, usize> = BTreeMap::new();
        let mut ooc_pairs: Vec<(String, String, String)> = Vec::new(); // name, objectContainer, starmapRecord
        let mut total_entities = 0usize;
        let mut decode_fail = 0usize;
        let mut sample_dumped = false;

        for (i, name) in &entity_members {
            let bytes = match pak.read(*i) {
                Ok(b) => b,
                Err(_) => {
                    decode_fail += 1;
                    continue;
                }
            };
            let root = match decode(&bytes) {
                Ok(Some(r)) => r,
                Ok(None) => continue,
                Err(_) => {
                    decode_fail += 1;
                    continue;
                }
            };
            for n in root.descendants() {
                *tag_hist.entry(n.tag.clone()).or_default() += 1;
                if n.tag == "Entity" {
                    total_entities += 1;
                    let class = n.attr("EntityClass").unwrap_or("<none>").to_string();
                    *class_hist.entry(class.clone()).or_default() += 1;
                    for (k, _) in &n.attrs {
                        *attr_hist.entry(format!("Entity@{k}")).or_default() += 1;
                    }
                    if class == "OrbitingObjectContainer" {
                        let oc = n
                            .find_all("EntityComponentObjectContainer")
                            .next()
                            .and_then(|c| c.attr("objectContainer"))
                            .unwrap_or("")
                            .to_string();
                        let meta = n.find_all("SObjectMetadataParams").next();
                        let sr = meta
                            .and_then(|c| c.attr("starmapRecord"))
                            .unwrap_or("")
                            .to_string();
                        let laa = meta
                            .and_then(|c| c.attr("locationActionArea"))
                            .unwrap_or("")
                            .to_string();
                        ooc_pairs.push((
                            n.attr("Name").unwrap_or("").to_string(),
                            oc,
                            format!("star={sr} laa={laa}"),
                        ));
                    }
                    if !sample_dumped && class == "OrbitingObjectContainer" {
                        sample_dumped = true;
                        eprintln!("\n--- sample OrbitingObjectContainer entity in {name} ---");
                        dump(n, 0, 3);
                    }
                }
            }
        }
        println!("total entities: {total_entities}  (decode failures: {decode_fail})");
        println!("\n-- EntityClass histogram (top 25) --");
        let mut cv: Vec<_> = class_hist.iter().collect();
        cv.sort_by(|a, b| b.1.cmp(a.1));
        for (k, n) in cv.iter().take(25) {
            println!("  {n:>5}  {k}");
        }
        println!(
            "\n-- OrbitingObjectContainer pairs: {} (name | objectContainer | starmapRecord) --",
            ooc_pairs.len()
        );
        for (nm, oc, sr) in ooc_pairs.iter().take(40) {
            println!("  {nm:<34} | {oc} | {sr}");
        }
        println!("\n-- component tags (top 40, for undecoded-data inventory) --");
        let mut tv: Vec<_> = tag_hist.iter().collect();
        tv.sort_by(|a, b| b.1.cmp(a.1));
        for (k, n) in tv.iter().take(40) {
            println!("  {n:>6}  {k}");
        }
        println!("\n-- Entity attribute keys --");
        for (k, n) in &attr_hist {
            println!("  {n:>6}  {k}");
        }

        // Find an entity by a GUID appearing in ANY of its attrs (CryGUID/actionArea/…).
        if let Some(g) = &find_guid {
            println!("\n######## entities referencing GUID {g} ########");
            let mut found = 0;
            for (i, _name) in &entity_members {
                if found >= 4 {
                    break;
                }
                let Ok(bytes) = pak.read(*i) else { continue };
                let Ok(Some(root)) = decode(&bytes) else {
                    continue;
                };
                for e in root.find_all("Entity") {
                    let hit = e.descendants().any(|n| {
                        n.attrs
                            .iter()
                            .any(|(_, v)| v.to_ascii_lowercase().contains(g.as_str()))
                    });
                    if hit {
                        println!("\n---- entity ----");
                        dump(e, 0, 8);
                        found += 1;
                        if found >= 4 {
                            break;
                        }
                    }
                }
            }
            println!("(found {found})");
        }

        // Full-subtree dump of entities containing a given component tag.
        if let Some(tag) = &dump_tag {
            println!("\n######## full dump of entities containing <{tag}> (max {dump_n}) ########");
            let mut dumped = 0usize;
            for (i, _name) in &entity_members {
                if dumped >= dump_n {
                    break;
                }
                let Ok(bytes) = pak.read(*i) else { continue };
                let Ok(Some(root)) = decode(&bytes) else {
                    continue;
                };
                for e in root.find_all("Entity") {
                    if dumped >= dump_n {
                        break;
                    }
                    if e.find_all(tag).next().is_some() {
                        println!("\n---- entity ----");
                        dump(e, 0, 8);
                        dumped += 1;
                    }
                }
            }
        }
    }
    Ok(())
}

fn dump(n: &XmlNode, depth: usize, maxd: usize) {
    if depth > maxd {
        return;
    }
    let attrs: Vec<String> = n
        .attrs
        .iter()
        .map(|(k, v)| format!("{k}={:?}", v))
        .collect();
    eprintln!("{}{} {}", "  ".repeat(depth), n.tag, attrs.join(" "));
    for c in &n.children {
        dump(c, depth + 1, maxd);
    }
}
