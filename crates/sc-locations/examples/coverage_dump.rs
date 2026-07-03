//! Dump every `StarMapObject` location as JSONL for external coverage analysis.
//!
//! One JSON object per line:
//! `{guid, crc, kind, name, record_name, record_path, parent, parent_name, system}`
//! where `crc = class_crc(guid)` (the EntityGraph subject_id namespace),
//! `record_name`/`record_path` come from `RecordPaths` (for bracket-id matching),
//! and `system` is the SolarSystem ancestor's display name.
//!
//! ```bash
//! cargo run -p sc-locations --release --example coverage_dump > locations.jsonl
//! ```
//! (Throwaway research tool for sc-cargo-planner CRC→name coverage analysis.)

use sc_extract::{AssetConfig, AssetData, AssetSource, LocaleMap, RecordPaths, class_crc};
use sc_locations::{Location, LocationKind, Locations, RecordCollection};

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn jstr(s: Option<&str>) -> String {
    match s {
        Some(s) => format!("\"{}\"", esc(s)),
        None => "null".to_string(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("{} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale: &LocaleMap = &asset_data.locale;

    let locs = Locations::build(datacore.records());
    let paths = RecordPaths::build(&datacore);
    eprintln!("locations: {}", locs.len());

    for (guid, loc) in locs.iter() {
        // SolarSystem ancestor (or self).
        let system: Option<String> = if matches!(loc.kind, LocationKind::SolarSystem) {
            loc.display_name(locale).map(str::to_string)
        } else {
            locs.ancestors(guid)
                .find(|a| matches!(a.kind, LocationKind::SolarSystem))
                .and_then(|a| a.display_name(locale))
                .map(str::to_string)
        };
        let parent: Option<&Location> = loc.parent.and_then(|p| locs.get(&p));
        let rp = paths.get(guid);

        println!(
            "{{\"guid\":\"{guid}\",\"crc\":{crc},\"kind\":\"{kind}\",\"name\":{name},\"record_name\":{rn},\"record_path\":{rpth},\"parent\":{parent_g},\"parent_name\":{parent_n},\"system\":{system}}}",
            crc = class_crc(guid),
            kind = loc.kind.as_dcb_str(),
            name = jstr(loc.display_name(locale)),
            rn = jstr(rp.map(|r| r.name.as_str())),
            rpth = jstr(rp.map(|r| r.path.as_str())),
            parent_g = jstr(loc.parent.map(|p| p.to_string()).as_deref()),
            parent_n = jstr(parent.and_then(|p| p.display_name(locale))),
            system = jstr(system.as_deref()),
        );
    }
    Ok(())
}
