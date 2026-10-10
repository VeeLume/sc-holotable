//! Export every main DCB record to XML under a directory, for grepping the
//! raw data when the typed bindings don't cover it yet (a new patch).
//!
//! ```bash
//! cargo run -p sc-extract --release --example dcb_xml_export -- <out_dir>
//! ```

use sc_extract::svarog_datacore::XmlExporter;
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args()
        .nth(1)
        .ok_or("usage: dcb_xml_export <out_dir>")?;
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let n = XmlExporter::new(datacore.db()).export_all(&out, |_, _| {})?;
    println!("exported {n} records");
    Ok(())
}
