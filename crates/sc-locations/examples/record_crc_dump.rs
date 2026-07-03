//! Dump every DCB record as `class_crc(guid)<TAB>record_name<TAB>path` — to
//! CRC-test the gRPC Contractor / location CRCs against record class-CRCs and
//! record-name CRCs.
//!
//! ```bash
//! cargo run -p sc-locations --release --example record_crc_dump > records.tsv
//! ```
use sc_extract::{AssetConfig, AssetData, AssetSource, RecordPaths, class_crc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let paths = RecordPaths::build(&datacore);
    eprintln!("records: {}", paths.len());
    for r in paths.iter() {
        let name = r.name.replace(['\t', '\n', '\r'], " ");
        let path = r.path.replace(['\t', '\n', '\r'], " ");
        println!("{}\t{}\t{}", class_crc(&r.guid), name, path);
    }
    Ok(())
}
