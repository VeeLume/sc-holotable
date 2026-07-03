//! Find where a location's GUID lives in the socpaks — to diff positioned vs
//! missing places and identify placeholder records.
//!
//!   cargo run -p sc-locations --example placement_probe --release
//!       → dump occurrences for the hardcoded "investigate" targets.
//!   cargo run -p sc-locations --example placement_probe --release -- --placeholders
//!       → list PLACEHOLDER/UNINITIALIZED/unnamed locations with their raw
//!         name-key + any socpak entity Name/path (spot legit sub-locations).
//!
//! One socpak scan builds a `guid → occurrences` map for ALL locations; both
//! reports read from it.

use std::collections::HashMap;

use sc_extract::object_container::{Socpak, XmlNode, decode};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap, RecordCollection};
use sc_locations::Locations;

type Key = [u8; 16];
const MAP: [usize; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8];

fn hex16(s: &str) -> Option<[u8; 16]> {
    let h: Vec<u8> = s.bytes().filter(u8::is_ascii_hexdigit).collect();
    if h.len() != 32 {
        return None;
    }
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[i] = ((h[i * 2] as char).to_digit(16)? * 16 + (h[i * 2 + 1] as char).to_digit(16)?) as u8;
    }
    Some(b)
}
fn cry_key(s: &str) -> Option<Key> {
    let db = hex16(s)?;
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[MAP[i]] = db[i];
    }
    Some(b)
}
fn std_key(s: &str) -> Option<Key> {
    let d = hex16(s)?;
    Some([
        d[3], d[2], d[1], d[0], d[5], d[4], d[7], d[6], d[8], d[9], d[10], d[11], d[12], d[13],
        d[14], d[15],
    ])
}
fn keys_of(s: &str) -> Vec<Key> {
    let mut v = Vec::new();
    if let Some(k) = std_key(s) {
        v.push(k);
    }
    if let Some(k) = cry_key(s) {
        v.push(k);
    }
    v
}

/// One socpak occurrence of a target GUID.
#[derive(Clone)]
struct Occ {
    socpak: String,
    member: String,
    class: String,
    name: String,
    pos: String,
    where_: String, // tag.key that held the guid
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("warn").init();
    let placeholders = std::env::args().any(|a| a == "--placeholders");

    let install = sc_discovery::discover_primary()?;
    println!("-> {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let ad = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &ad)?;
    let locs = Locations::build(datacore.records());

    // key → guid, for EVERY location. (both GUID renderings)
    let mut key_guid: HashMap<Key, Guid> = HashMap::new();
    for (g, _) in locs.iter() {
        for k in keys_of(&format!("{g}")) {
            key_guid.insert(k, *g);
        }
    }

    // Scan once, collect occurrences per guid (capped).
    let mut occ: HashMap<Guid, Vec<Occ>> = HashMap::new();
    let socpaks: Vec<String> = assets
        .find(|n| {
            let l = n.to_ascii_lowercase().replace('\\', "/");
            l.ends_with(".socpak") && l.contains("objectcontainers")
        })
        .map(|e| e.name.to_string())
        .collect();
    eprintln!("-> scanning {} socpaks…", socpaks.len());

    for sp in &socpaks {
        let base = sp.rsplit(['\\', '/']).next().unwrap_or(sp).to_string();
        let allow_xml = sp
            .to_ascii_lowercase()
            .replace('\\', "/")
            .contains("/system/");
        let Ok(bytes) = assets.read(sp) else { continue };
        let Ok(mut pak) = Socpak::open(bytes) else {
            continue;
        };
        for m in 0..pak.len() {
            let mem = pak.name(m).unwrap_or_default();
            let ml = mem.to_ascii_lowercase();
            let ok = ml.ends_with(".soc")
                || ml.ends_with(".pla")
                || ml.ends_with(".entxml")
                || (allow_xml && ml.ends_with(".xml"));
            if !ok {
                continue;
            }
            let Ok(b) = pak.read(m) else { continue };
            let Ok(Some(root)) = decode(&b) else { continue };
            let member = mem.rsplit(['\\', '/']).next().unwrap_or(&mem).to_string();

            for e in root.find_all("Entity") {
                let class = e.attr("EntityClass").unwrap_or("").to_string();
                let name = e.attr("Name").unwrap_or("").to_string();
                let pos = e.attr("Pos").unwrap_or("<none>").to_string();
                for n in e.descendants() {
                    for (k, v) in &n.attrs {
                        if v.chars().filter(|c| c.is_ascii_hexdigit()).count() < 32 {
                            continue;
                        }
                        for kk in keys_of(v) {
                            if let Some(g) = key_guid.get(&kk) {
                                let list = occ.entry(*g).or_default();
                                if list.len() < 6 {
                                    list.push(Occ {
                                        socpak: base.clone(),
                                        member: member.clone(),
                                        class: class.clone(),
                                        name: name.clone(),
                                        pos: pos.clone(),
                                        where_: format!("{}.{k}", n.tag),
                                    });
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = placeholders;
    report_targets(&locs, &ad.locale, &occ);
    report_name_contains(&locs, &ad.locale, &occ, "QV Breaker");
    report_placeholders(&locs, &ad.locale, &occ);
    Ok(())
}

/// Enumerate every location whose display name contains `needle`, with its
/// position status — for "how many QV Breaker Stations, and which have a pos".
fn report_name_contains(
    locs: &Locations,
    locale: &LocaleMap,
    occ: &HashMap<Guid, Vec<Occ>>,
    needle: &str,
) {
    let mut placed: Vec<String> = Vec::new();
    let mut unplaced = 0u32;
    let mut socpaks: HashMap<String, u32> = HashMap::new();
    for (g, l) in locs.iter() {
        let Some(name) = l.display_name(locale) else {
            continue;
        };
        if !name.contains(needle) {
            continue;
        }
        match occ.get(g) {
            Some(v) if !v.is_empty() => {
                let o = &v[0];
                *socpaks.entry(o.socpak.clone()).or_default() += 1;
                placed.push(format!(
                    "  {name:<26} {} via {} ({})",
                    o.pos, o.where_, o.socpak
                ));
            }
            _ => unplaced += 1,
        }
    }
    placed.sort();
    println!(
        "\n═══ \"{needle}\": {} total — {} positioned, {unplaced} unplaced ═══",
        placed.len() + unplaced as usize,
        placed.len()
    );
    println!("  positioned by socpak: {socpaks:?}");
    for r in placed.iter().take(6) {
        println!("{r}");
    }
    if placed.len() > 6 {
        println!("  … +{} more positioned", placed.len() - 6);
    }
}

fn report_targets(locs: &Locations, locale: &LocaleMap, occ: &HashMap<Guid, Vec<Occ>>) {
    let targets = [
        "MIC-L2 Long Forest Station", // ● working reference (SObjectMetadataParams)
        "MIC-L1 Shallow Frontier Station", // ○ starmapRecord on <Elem>
        "Checkmate",                  // ingame name for the old PYAM-FARSTAT-2-0
        "PYAM-FARSTAT-2-0",
    ];
    for name in targets {
        let hit = locs
            .iter()
            .find(|(_, l)| l.display_name(locale) == Some(name));
        let Some((g, _)) = hit else {
            println!("\n══ {name} — NOT A LOCATION");
            continue;
        };
        println!("\n══ {name}  guid={g} ══");
        match occ.get(g) {
            None => println!("  (GUID never appears in any scanned socpak)"),
            Some(v) => {
                for o in v {
                    println!(
                        "  [{}] Name={} Pos={}\n      via {}  ({}/{})",
                        o.class, o.name, o.pos, o.where_, o.socpak, o.member
                    );
                }
            }
        }
    }
}

fn report_placeholders(locs: &Locations, locale: &LocaleMap, occ: &HashMap<Guid, Vec<Occ>>) {
    // Placeholder/unnamed = display resolves to "<= …" or nothing.
    let mut with_occ: Vec<String> = Vec::new();
    let mut without = 0u32;
    let mut total = 0u32;
    for (g, l) in locs.iter() {
        let disp = l.display_name(locale);
        let is_ph = disp.is_none() || disp.map(|d| d.starts_with("<=")).unwrap_or(false);
        if !is_ph {
            continue;
        }
        total += 1;
        let key = l
            .name_key
            .as_ref()
            .map(|k| k.as_ref())
            .unwrap_or("<no key>");
        let kind = format!("{:?}", l.kind);
        match occ.get(g) {
            Some(v) if !v.is_empty() => {
                // The socpak entity Name is the real identity of a placeholder.
                let o = &v[0];
                with_occ.push(format!(
                    "  [{kind:<24}] key={key:<34} → Name={} Pos={} ({}/{})",
                    o.name, o.pos, o.socpak, o.member
                ));
            }
            _ => without += 1,
        }
    }
    with_occ.sort();
    println!("═══ PLACEHOLDER / UNINITIALIZED / unnamed locations ═══");
    println!("  total placeholder records : {total}");
    println!(
        "  with a socpak placement   : {} (potentially legit)",
        with_occ.len()
    );
    println!("  no socpak placement       : {without} (pure dev/empty records)\n");
    for line in with_occ.iter().take(120) {
        println!("{line}");
    }
    if with_occ.len() > 120 {
        println!("  … +{} more", with_occ.len() - 120);
    }
}

// (helper kept for symmetry with other examples)
#[allow(dead_code)]
fn norm(raw: &str) -> String {
    raw.replace('\\', "/").to_ascii_lowercase()
}
#[allow(dead_code)]
fn _x(_: &XmlNode) {}
