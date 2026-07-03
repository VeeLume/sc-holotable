//! Dump every global.ini localization (key<TAB>value) pair — to CRC-test the
//! gRPC uint32 localization ids (ui_display_text_id / short/long_desc_id).
//!
//! ```bash
//! cargo run -p sc-locations --release --example locale_dump > locale.tsv
//! ```
use sc_extract::{AssetConfig, AssetData, AssetSource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let locale = &asset_data.locale;
    eprintln!("locale entries: {}", locale.len());
    for (k, v) in locale.iter() {
        // strip tabs/newlines from the value so the TSV stays one line per key.
        let v = v.replace(['\t', '\n', '\r'], " ");
        println!("{k}\t{v}");
    }
    Ok(())
}
