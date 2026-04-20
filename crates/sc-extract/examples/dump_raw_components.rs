//! Use the raw svarog DB to list component type names on weapon entities.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons \
//!     --example dump_raw_components
//! ```

use sc_extract::{
    AssetConfig, AssetData, AssetSource, DatacoreConfig,
    svarog_datacore::Value,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let install = sc_installs::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let db = datacore.db();

    // First: find how records are named
    let mut found = 0;
    for record in db.records() {
        let struct_name = db.struct_name(record.struct_index as usize).unwrap_or("?");
        if struct_name != "EntityClassDefinition" { continue; }
        let name = db.record_name(record).unwrap_or("");
        if name.contains("LaserCannon_S7") || name.contains("BallisticGatling_S1") || name.contains("LaserRepeater_S1") {
            if found < 5 {
                println!("Record name format: '{}'", name);
                found += 1;
            }
        }
    }

    for record in db.records() {
        let name = db.record_name(record).unwrap_or("");
        if !name.contains("BEHR_LaserCannon_S7") && !name.contains("GATS_BallisticGatling_S1") { continue; }

        let struct_name = db.struct_name(record.struct_index as usize).unwrap_or("?");
        if struct_name != "EntityClassDefinition" { continue; }

        println!("=== {} (type: {}) ===", name, struct_name);

        let inst = db.instance(record.struct_index as u32, record.instance_index as u32);
        if let Some(components) = inst.get_array("Components") {
            for (i, val) in components.enumerate() {
                let type_name = match &val {
                    Value::StrongPointer(Some(r)) => {
                        db.struct_name(r.struct_index as usize).unwrap_or("?")
                    }
                    Value::Class { struct_index, .. } => {
                        db.struct_name(*struct_index as usize).unwrap_or("?")
                    }
                    _ => "non-struct",
                };
                println!("  [{}] {}", i, type_name);
            }
        }
        println!();
    }

    Ok(())
}
