//! Multi-sample crack: pair each seed DataSet-CRC with its StarMapObject (by
//! display name) to get (dataset_crc, field19=class_crc, guid) triples, then hunt
//! for a relationship between the two id namespaces across ALL pairs (a single
//! pair can't reveal a fixed XOR/offset or a crc-of-the-other).
//!
//! ```bash
//! cargo run -p sc-locations --release --example loc_pairs -- <loc_resolved.tsv>
//! ```
use std::collections::HashMap;

use sc_extract::generated::StarMapObject;
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap, class_crc};

fn table(poly: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ poly } else { c >> 1 };
        }
        t[i as usize] = c;
    }
    t
}
fn crc(b: &[u8], t: &[u32; 256]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &x in b {
        c = t[((c ^ x as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // seed: dataset_crc \t name \t bracket
    let seed: Vec<(u32, String)> = std::fs::read_to_string(&args[0])?
        .lines()
        .filter_map(|l| {
            let mut it = l.split('\t');
            let crc = it.next()?.trim().parse().ok()?;
            let name = it.next().unwrap_or("").to_string();
            Some((crc, name))
        })
        .collect();

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let locale: &LocaleMap = &asset_data.locale;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let pools = &store.pools;
    let c_tbl = table(0x82F6_3B78);
    let i_tbl = table(0xEDB8_8320);

    // name -> (guid, field19=class_crc) for every StarMapObject.
    let mut by_name: HashMap<String, (Guid, u32)> = HashMap::new();
    for (&guid, &handle) in store.records.multi_feature.star_map_object.iter() {
        if let Some(obj) = handle.get(pools) {
            if let Some(name) = locale.resolve(&obj.name) {
                if !name.is_empty() {
                    by_name.insert(name.to_string(), (guid, class_crc(&guid)));
                }
            }
        }
    }
    eprintln!("StarMapObjects with names: {}", by_name.len());

    // Build triples for the seed entries we can match by exact display name.
    let mut triples: Vec<(u32, u32, Guid, String)> = Vec::new();
    for (dcrc, name) in &seed {
        if let Some((guid, f19)) = by_name.get(name) {
            triples.push((*dcrc, *f19, *guid, name.clone()));
        }
    }
    eprintln!(
        "matched {} / {} seed entries to a StarMapObject\n",
        triples.len(),
        seed.len()
    );

    // Show a few triples.
    for (d, f, g, n) in triples.iter().take(12) {
        eprintln!(
            "  dataset={d:<11} field19={f:<11} xor={:<11} guid={g}  {n}",
            d ^ f
        );
    }

    // Relationship hunt across ALL matched pairs.
    let n = triples.len().max(1);
    // (1) constant XOR / additive offset?
    let xors: Vec<u32> = triples.iter().map(|(d, f, _, _)| d ^ f).collect();
    let adds: Vec<u32> = triples
        .iter()
        .map(|(d, f, _, _)| d.wrapping_sub(*f))
        .collect();
    let xor_const = xors.iter().all(|x| Some(x) == xors.first());
    let add_const = adds.iter().all(|x| Some(x) == adds.first());
    eprintln!(
        "\nconstant XOR(dataset,field19)? {xor_const}  (first={:#x})",
        xors.first().copied().unwrap_or(0)
    );
    eprintln!(
        "constant dataset-field19 offset? {add_const}  (first={:#x})",
        adds.first().copied().unwrap_or(0)
    );

    // (2) dataset == crc(field19 bytes LE/BE) ? (count how many pairs satisfy)
    let mut hit_fns: HashMap<&str, usize> = HashMap::new();
    for (d, f, g, _) in &triples {
        let f_le = f.to_le_bytes();
        let f_be = f.to_be_bytes();
        let checks: [(&str, u32); 6] = [
            ("crc32c(field19_le)", crc(&f_le, &c_tbl)),
            ("ieee(field19_le)", crc(&f_le, &i_tbl)),
            ("crc32c(field19_be)", crc(&f_be, &c_tbl)),
            ("ieee(field19_be)", crc(&f_be, &i_tbl)),
            ("crc32c(guid)", crc(g.as_bytes(), &c_tbl)),
            ("ieee(guid)", crc(g.as_bytes(), &i_tbl)),
        ];
        for (name, v) in checks {
            if v == *d {
                *hit_fns.entry(name).or_default() += 1;
            }
        }
    }
    eprintln!("\nper-function match counts (out of {n} pairs — a real relation hits ~all):");
    if hit_fns.is_empty() {
        eprintln!("  (none)");
    }
    for (k, c) in &hit_fns {
        eprintln!("  {k}: {c}");
    }

    // (3) Dump the full triple table for offline pattern analysis.
    let mut out = String::from("dataset_crc\tfield19\tguid\tname\n");
    for (d, f, g, name) in &triples {
        out.push_str(&format!("{d}\t{f}\t{g}\t{name}\n"));
    }
    std::fs::write("loc_triples_field19.tsv", &out)?;
    eprintln!(
        "\nwrote loc_triples_field19.tsv ({} triples)",
        triples.len()
    );
    Ok(())
}
