//! Full extension census of the p4k: count files by extension + total bytes.
//! Used to enumerate ALL shipped file formats (find untested containers).
//!
//! ```bash
//! cargo run -p sc-locations --release --example ext_census
//! ```
use sc_extract::AssetSource;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let archive = assets.archive().expect("live");
    let mut counts: HashMap<String, (u64, u64)> = HashMap::new(); // ext -> (count, bytes)
    let mut total = 0u64;
    for e in archive.iter() {
        total += 1;
        let name = e.name;
        let ext = name
            .rsplit('.')
            .next()
            .filter(|x: &&str| x.len() <= 8 && !x.contains('/') && !x.contains('\\'))
            .unwrap_or("<noext>")
            .to_ascii_lowercase();
        let slot = counts.entry(ext).or_insert((0, 0));
        slot.0 += 1;
        slot.1 += e.uncompressed_size;
    }
    let mut v: Vec<_> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    println!("total entries: {total}");
    for (ext, (c, b)) in v {
        println!("{c:>8}  {b:>15}  .{ext}");
    }
    Ok(())
}
