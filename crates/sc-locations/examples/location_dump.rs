//! Diagnostic dump of the [`Locations`] index against the live DCB.
//!
//! Prints a kind histogram, then demonstrates the EntityGraph entry point:
//! resolve a location `subject_id` CRC to a typed location, its localized name,
//! and its hierarchy chain up to the system root.
//!
//! ```bash
//! cargo run -p sc-locations --release --example location_dump
//! # resolve a specific wire CRC:
//! cargo run -p sc-locations --release --example location_dump -- 3723364946
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, LocaleMap, class_crc};
use sc_locations::{Location, Locations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Default demo CRC: Nyx_Levski's subject_id.
    let crc: u32 = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3_723_364_946);

    let install = sc_discovery::discover_primary()?;
    println!("{} v{}\n", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;

    let locations = Locations::build(datacore.records());
    println!("locations: {}", locations.len());

    // ── kind histogram ───────────────────────────────────────────────────
    let mut hist: BTreeMap<String, usize> = BTreeMap::new();
    for (_g, loc) in locations.iter() {
        *hist.entry(loc.kind.as_dcb_str().to_string()).or_default() += 1;
    }
    println!("\n-- by kind --");
    let mut rows: Vec<_> = hist.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    for (kind, n) in rows {
        println!("  {kind:<28} {n}");
    }

    // ── CRC resolution demo ──────────────────────────────────────────────
    println!("\n-- resolve subject_id crc {crc} --");
    match locations.by_crc(crc) {
        Some(loc) => {
            let guid = loc.guid;
            println!("  guid   : {guid}");
            println!("  name   : {}", display(loc, locale));
            println!("  kind   : {}", loc.kind.as_dcb_str());
            println!("  icon   : {}", loc.nav_icon.as_dcb_str());
            print!("  parents: ");
            let chain: Vec<String> = locations
                .ancestors(&guid)
                .map(|a| display(a, locale))
                .collect();
            println!(
                "{}",
                if chain.is_empty() {
                    "<root>".into()
                } else {
                    chain.join(" → ")
                }
            );
            // round-trip sanity: the CRC must come back from the guid.
            assert_eq!(class_crc(&guid), crc, "crc round-trip mismatch");
        }
        None => println!("  (no location hashes to {crc})"),
    }

    // ── a sample of named landing zones with their hierarchy ─────────────
    println!("\n-- landing zones --");
    let mut lzs: Vec<&Location> = locations
        .iter()
        .map(|(_g, l)| l)
        .filter(|l| l.kind.as_dcb_str() == "LandingZone")
        .collect();
    lzs.sort_by_key(|l| display(l, locale));
    for l in lzs {
        let chain: Vec<String> = locations
            .ancestors(&l.guid)
            .map(|a| display(a, locale))
            .collect();
        println!("  {:<20} {}", display(l, locale), chain.join(" → "));
    }

    Ok(())
}

fn display(loc: &Location, locale: &LocaleMap) -> String {
    loc.display_name(locale).unwrap_or("<no-name>").to_string()
}
