//! Harvest entity/area/tag GUIDs from socpak(s) and test them against the
//! historical seed: (a) do the seed's SuperGUID constant components appear as
//! socpak entity GUIDs? (b) does class_crc(any harvested GUID) == a locationHash?
//!
//! ```bash
//! cargo run -p sc-locations --release --example guid_harvest -- <triples.tsv> stanton1.socpak
//! ```
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use sc_extract::object_container::{Socpak, decode};
use sc_extract::{AssetSource, Guid, class_crc};

fn is_entity_file(n: &str) -> bool {
    let n = n.to_ascii_lowercase();
    n.ends_with(".soc") || n.ends_with(".pla") || n.ends_with(".entxml")
}
fn norm_guid(s: &str) -> Option<String> {
    let t = s.trim().trim_matches(['{', '}']).to_ascii_lowercase();
    Guid::from_str(&t).ok().map(|_| t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let triples = std::env::args().nth(1).expect("triples.tsv");
    let needle = std::env::args()
        .nth(2)
        .expect("socpak needle")
        .to_lowercase();

    // Load seed: hash -> constant SuperGUID components (present in every row).
    let text = std::fs::read_to_string(&triples)?;
    let mut rows: HashMap<u32, Vec<Vec<String>>> = HashMap::new();
    let mut name: HashMap<u32, String> = HashMap::new();
    for line in text.lines() {
        let mut it = line.split('\t');
        let (Some(h), Some(sg), nm) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let Ok(hash) = h.parse::<u32>() else { continue };
        let comps: Vec<String> = sg.split('.').filter_map(norm_guid).collect();
        if comps.is_empty() {
            continue;
        }
        rows.entry(hash).or_default().push(comps);
        name.entry(hash)
            .or_insert_with(|| nm.unwrap_or("").to_string());
    }
    let mut const_comps: HashMap<u32, HashSet<String>> = HashMap::new();
    for (&h, vecs) in &rows {
        let n = vecs.len();
        let mut cnt: HashMap<&String, usize> = HashMap::new();
        for v in vecs {
            for g in v.iter().collect::<HashSet<_>>() {
                *cnt.entry(g).or_default() += 1;
            }
        }
        const_comps.insert(
            h,
            cnt.into_iter()
                .filter(|(_, c)| *c == n)
                .map(|(g, _)| g.clone())
                .collect(),
        );
    }

    // Harvest GUIDs from matching socpaks.
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase();
            l.ends_with(".socpak") && l.contains(&needle)
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("harvesting {} socpaks for {needle:?}", socpaks.len());

    let mut harvested: HashSet<String> = HashSet::new(); // raw guid strings (lower)
    let attrs_of_interest = [
        "EntityCryGUID",
        "EntityClassGUID",
        "starmapRecord",
        "locationActionArea",
        "actionArea",
        "template",
        "TagId",
        "parentGUID",
        "preset",
    ];
    let mut socpak_count = 0;
    for sp in socpaks.iter().take(6) {
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        socpak_count += 1;
        for i in 0..pak.len() {
            let nm = pak.name(i).unwrap_or_default();
            if !is_entity_file(&nm) {
                continue;
            }
            let Ok(b) = pak.read(i) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            for node in root.descendants() {
                for (k, v) in &node.attrs {
                    if attrs_of_interest.contains(&k.as_str()) {
                        for part in v.split(',') {
                            if let Some(g) = norm_guid(part) {
                                harvested.insert(g);
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!(
        "harvested {} distinct GUIDs from {socpak_count} socpaks",
        harvested.len()
    );

    // class_crc index of harvested GUIDs.
    let mut crc_to_guid: HashMap<u32, String> = HashMap::new();
    for g in &harvested {
        if let Ok(guid) = Guid::from_str(g) {
            crc_to_guid.insert(class_crc(&guid), g.clone());
        }
    }

    // Test (a): seed constant SuperGUID components present as harvested entity GUIDs.
    // Test (b): locationHash == class_crc(harvested GUID).
    let mut comp_hit = 0;
    let mut hash_hit = 0;
    let mut both = 0;
    let mut samples = 0;
    for (&h, comps) in &const_comps {
        let comp_present: Vec<&String> = comps.iter().filter(|c| harvested.contains(*c)).collect();
        let crc_present = crc_to_guid.get(&h);
        if !comp_present.is_empty() {
            comp_hit += 1;
        }
        if crc_present.is_some() {
            hash_hit += 1;
        }
        if !comp_present.is_empty() && crc_present.is_some() {
            both += 1;
        }
        if (!comp_present.is_empty() || crc_present.is_some()) && samples < 25 {
            samples += 1;
            eprintln!(
                "hash {h:<11} {:?}\n   superGUID-comp in socpak: {comp_present:?}\n   class_crc match: {:?}",
                name.get(&h),
                crc_present
            );
        }
    }
    eprintln!(
        "\n==== SUMMARY (against {} seed hashes) ====",
        const_comps.len()
    );
    eprintln!("seed hashes with a SuperGUID constant component found in these socpaks: {comp_hit}");
    eprintln!("seed hashes where class_crc(harvested GUID) == locationHash: {hash_hit}");
    eprintln!("both: {both}");
    Ok(())
}
