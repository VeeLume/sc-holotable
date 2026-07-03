//! Dump the `GameMode.SC_Default` record (libs/foundry/records/gamemode/sc_default)
//! from the raw DCB — to find the `uecCurve` reward constants (k, m) and any
//! economy multiplier / reputation-bonus config the contract-reward formula uses.
//!
//! ```bash
//! cargo run -p sc-missions --release --example gamemode_dump
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordPaths};
use sc_missions::raw::svarog_datacore::{DataCoreDatabase, Instance, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();
    let paths = RecordPaths::build(&datacore);

    let target = paths
        .iter()
        .find(|r| r.name == "GameMode.SC_Default")
        .or_else(|| {
            paths
                .iter()
                .find(|r| r.path.to_lowercase().contains("gamemode/sc_default"))
        });
    let Some(rp) = target else {
        eprintln!("GameMode.SC_Default not found. GameMode.* records:");
        for r in paths.iter().filter(|r| r.name.starts_with("GameMode.")) {
            eprintln!("  {}  ::  {}", r.name, r.path);
        }
        return Ok(());
    };
    println!("=== {} :: {} ===", rp.name, rp.path);
    let Some(rec) = db.record(&rp.guid) else {
        eprintln!("no record for guid");
        return Ok(());
    };
    dump(db, &rec.as_instance(), 0, 0, &paths);
    Ok(())
}

fn dump(db: &DataCoreDatabase, inst: &Instance, indent: usize, depth: usize, paths: &RecordPaths) {
    let pad = "  ".repeat(indent);
    if depth > 5 {
        println!("{pad}…(max depth)");
        return;
    }
    for prop in inst.properties() {
        let n = prop.name;
        match prop.value {
            Value::Bool(v) => println!("{pad}{n} = {v}"),
            Value::Int8(v) => println!("{pad}{n} = {v}"),
            Value::Int16(v) => println!("{pad}{n} = {v}"),
            Value::Int32(v) => println!("{pad}{n} = {v}"),
            Value::Int64(v) => println!("{pad}{n} = {v}"),
            Value::UInt8(v) => println!("{pad}{n} = {v}"),
            Value::UInt16(v) => println!("{pad}{n} = {v}"),
            Value::UInt32(v) => println!("{pad}{n} = {v}"),
            Value::UInt64(v) => println!("{pad}{n} = {v}"),
            Value::Float(v) => println!("{pad}{n} = {v}f32"),
            Value::Double(v) => println!("{pad}{n} = {v}f64"),
            Value::String(s) | Value::Locale(s) | Value::Enum(s) => println!("{pad}{n} = \"{s}\""),
            Value::Guid(g) => {
                let r = paths.get(&g).map(|r| r.name.as_str()).unwrap_or("?");
                println!("{pad}{n} = guid -> {r}");
            }
            Value::Reference(Some(rr)) => {
                let r = paths.get(&rr.guid).map(|r| r.name.as_str()).unwrap_or("?");
                println!("{pad}{n} = ref -> {r}");
            }
            Value::Reference(None) => println!("{pad}{n} = ref(null)"),
            Value::StrongPointer(None) | Value::WeakPointer(None) => {
                println!("{pad}{n} = ptr(null)")
            }
            Value::Class { .. }
            | Value::ClassRef(_)
            | Value::StrongPointer(Some(_))
            | Value::WeakPointer(Some(_)) => {
                if let Some(child) = inst.get_instance(n) {
                    println!("{pad}{n} {{");
                    dump(db, &child, indent + 1, depth + 1, paths);
                    println!("{pad}}}");
                } else {
                    println!("{pad}{n} {{ ? }}");
                }
            }
            Value::Array(_) => {
                let cnt = inst.get_array(n).map(|a| a.len()).unwrap_or(0);
                println!("{pad}{n} [{cnt}]");
                if let Some(arr) = inst.get_array(n) {
                    for (i, el) in arr.enumerate().take(30) {
                        match el {
                            Value::ClassRef(r)
                            | Value::StrongPointer(Some(r))
                            | Value::WeakPointer(Some(r)) => {
                                println!("{pad}  [{i}] {{");
                                dump(
                                    db,
                                    &db.instance(r.struct_index, r.instance_index),
                                    indent + 2,
                                    depth + 1,
                                    paths,
                                );
                                println!("{pad}  }}");
                            }
                            Value::Class { struct_index, data } => {
                                println!("{pad}  [{i}] {{");
                                dump(
                                    db,
                                    &Instance::from_inline_data(db, struct_index, data),
                                    indent + 2,
                                    depth + 1,
                                    paths,
                                );
                                println!("{pad}  }}");
                            }
                            Value::Float(v) => println!("{pad}  [{i}] = {v}f32"),
                            Value::Int32(v) => println!("{pad}  [{i}] = {v}"),
                            Value::String(s) | Value::Locale(s) | Value::Enum(s) => {
                                println!("{pad}  [{i}] = \"{s}\"")
                            }
                            Value::Guid(g) => {
                                let r = paths.get(&g).map(|r| r.name.as_str()).unwrap_or("?");
                                println!("{pad}  [{i}] = guid -> {r}");
                            }
                            other => println!("{pad}  [{i}] = {other:?}"),
                        }
                    }
                }
            }
            Value::Null => println!("{pad}{n} = null"),
        }
    }
}
