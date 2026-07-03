//! Dump every DCB record as `guid<TAB>record_name<TAB>path` (raw dashed GUID,
//! not the class_crc). For running an offline multi-hash battery over record
//! GUID BYTES in several byte orders (the location_id hashfn probe).
//!
//! ```bash
//! cargo run -p sc-locations --release --example guid_dump > guids.tsv
//! ```
use sc_extract::{AssetConfig, AssetData, AssetSource, RecordPaths};

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
        println!("{}\t{}\t{}", r.guid, name, path);
    }
    Ok(())
}
