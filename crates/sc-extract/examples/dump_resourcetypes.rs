//! Probe: dump every `ResourceType.*` record from the live DCB as XML.
//! The on-disk dcbraw corpus collapses all records sharing the file path
//! `resourcetypedatabase/resourcetypedatabase.xml` into one file; this dumps
//! ALL of them (one XML per record name) so we can inspect quality refs.
//!
//! ```bash
//! cargo run -p sc-extract --release --example dump_resourcetypes -- <out-dir> [name-prefix]
//! ```

use sc_extract::svarog_datacore::XmlExporter;
use sc_extract::{AssetConfig, AssetData, AssetSource};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let out_dir = args
        .next()
        .expect("usage: dump_resourcetypes <out-dir> [name-prefix]");
    let prefix = args.next().unwrap_or_else(|| "ResourceType.".to_string());
    std::fs::create_dir_all(&out_dir)?;

    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    let exporter = XmlExporter::new(db);
    let mut n = 0usize;
    for record in db.records() {
        let Some(name) = db.record_name(record) else {
            continue;
        };
        if !name.starts_with(&prefix) {
            continue;
        }
        let xml = exporter.export_record(record)?;
        let fname = format!("{}/{}.xml", out_dir, name.replace([':', '/', '\\'], "_"));
        std::fs::write(&fname, xml)?;
        n += 1;
    }
    println!("wrote {n} records with prefix {prefix} to {out_dir}");
    Ok(())
}
