//! Map the `libs/foundry/records/crafting/` subtree via RecordPaths.
//! Lists every record's type + path + first-line of fields where reachable
//! under the `crafting` feature, with extra focus on
//! `craftingglobalparams.xml` (flagged as load-bearing).
//!
//! ```bash
//! cargo run -p sc-crafting --release --example crafting_tree
//! ```
#![allow(non_snake_case)]

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordPaths};

const ROOT: &str = "libs/foundry/records/crafting";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let _asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &_asset_data)?;
    let paths = RecordPaths::build(&datacore);

    // 1. tree shape: subdirectories + record counts per subdirectory
    println!("\n=== {ROOT}/ subdirectory map ===");
    let children: Vec<&str> = paths.children(ROOT).collect();
    if children.is_empty() {
        println!("  (no children directly under {ROOT})");
    }
    for child in &children {
        let sub = format!("{ROOT}/{child}");
        let n = paths.under(&sub).count();
        println!("  {child:<28} ({n} records under)");
    }

    // 2. file leaves + their main records' types
    println!("\n=== files (record_type counts under each direct child) ===");
    for child in &children {
        let sub = format!("{ROOT}/{child}");
        let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
        for guid in paths.under(&sub) {
            let Some(rp) = paths.get(guid) else { continue };
            let t = paths.type_name(rp.struct_index).unwrap_or("?").to_string();
            *by_type.entry(t).or_default() += 1;
        }
        println!("  {child}/");
        for (t, n) in by_type {
            println!("    {t:<44} {n}");
        }
    }

    // 3. enumerate every file path with its main record(s)
    println!("\n=== every record file (path → (type, name)) ===");
    // Collect distinct paths from records under root.
    let mut by_path: BTreeMap<String, Vec<&sc_extract::RecordPath>> = BTreeMap::new();
    for guid in paths.under(ROOT) {
        if let Some(rp) = paths.get(guid) {
            by_path.entry(rp.path.clone()).or_default().push(rp);
        }
    }
    for (p, recs) in &by_path {
        println!("  {p}");
        for r in recs {
            let t = paths.type_name(r.struct_index).unwrap_or("?");
            let mark = if r.is_main { "★" } else { " " };
            println!("    {mark} {t:<40} name={:?} guid={}", r.name, r.guid);
        }
    }

    // 4. focused dig: craftingglobalparams.xml — print every typed field we
    //    can reach via the `crafting` feature.
    println!("\n=== craftingglobalparams.xml deep dig ===");
    let gp_path = format!("{ROOT}/globalparams/craftingglobalparams.xml");
    let gp_guids = paths.at(&gp_path);
    if gp_guids.is_empty() {
        println!("  (no records at {gp_path}) — try other casing / variant filename?");
        // fallback: search globalparams dir
        let gp_dir = format!("{ROOT}/globalparams");
        for guid in paths.under(&gp_dir) {
            if let Some(rp) = paths.get(guid) {
                println!(
                    "  found under globalparams/: {} ({})",
                    rp.path,
                    paths.type_name(rp.struct_index).unwrap_or("?")
                );
            }
        }
    } else {
        for guid in gp_guids {
            let Some(rp) = paths.get(guid) else { continue };
            let t = paths.type_name(rp.struct_index).unwrap_or("?");
            println!("  ★ record: type={t} name={:?} guid={}", rp.name, rp.guid);
            dump_record_raw(&datacore, *guid, t);
        }
    }

    Ok(())
}

/// Walk the raw svarog instance for a record and print attribute names +
/// value kinds. Generic — works for any record regardless of feature gating.
fn dump_record_raw(datacore: &Datacore, guid: sc_extract::Guid, type_name: &str) {
    use sc_extract::svarog_datacore::Value;
    let Some(record) = datacore.db().record(&guid) else {
        println!("    (record not in db?)");
        return;
    };
    let inst = record.as_instance();
    println!("    fields (raw svarog attributes on {type_name}):");
    for prop in inst.properties() {
        let name = prop.name;
        let value = prop.value;
        let kind = match &value {
            Value::Bool(b) => format!("Bool({b})"),
            Value::Int8(v) => format!("i8({v})"),
            Value::Int16(v) => format!("i16({v})"),
            Value::Int32(v) => format!("i32({v})"),
            Value::Int64(v) => format!("i64({v})"),
            Value::UInt8(v) => format!("u8({v})"),
            Value::UInt16(v) => format!("u16({v})"),
            Value::UInt32(v) => format!("u32({v})"),
            Value::UInt64(v) => format!("u64({v})"),
            Value::Float(v) => format!("f32({v})"),
            Value::Double(v) => format!("f64({v})"),
            Value::String(s) => format!("Str({:?})", s),
            Value::Locale(s) => format!("Locale({:?})", s),
            Value::Enum(s) => format!("Enum({:?})", s),
            Value::Guid(g) => format!("Guid({})", g),
            Value::StrongPointer(Some(r)) => {
                let inner = datacore.db().instance(r.struct_index, r.instance_index);
                let tn = inner.type_name().unwrap_or("?");
                format!("StrongPtr→{}", tn)
            }
            Value::StrongPointer(None) => "StrongPtr(none)".into(),
            Value::WeakPointer(Some(r)) => {
                let inner = datacore.db().instance(r.struct_index, r.instance_index);
                let tn = inner.type_name().unwrap_or("?");
                format!("WeakPtr→{}", tn)
            }
            Value::WeakPointer(None) => "WeakPtr(none)".into(),
            Value::Reference(Some(r)) => format!("Ref→{}", r.guid),
            Value::Reference(None) => "Ref(none)".into(),
            Value::Class { struct_index, .. } => {
                let n = datacore
                    .db()
                    .struct_name(*struct_index as usize)
                    .unwrap_or("?");
                format!("Class({})", n)
            }
            Value::ClassRef(r) => {
                let si = r.struct_index;
                let n = datacore.db().struct_name(si as usize).unwrap_or("?");
                format!("ClassRef({})", n)
            }
            Value::Array(arr) => format!("Array(len={}, elem={:?})", arr.count, arr.element_type),
            other => format!("{:?}", std::mem::discriminant(other)),
        };
        println!("      {:<32} : {}", name, kind);
        // For Class arrays, resolve the struct index to a typename for clarity.
        if let Value::Array(arr) = &value {
            let struct_name = datacore
                .db()
                .struct_name(arr.struct_index as usize)
                .unwrap_or("");
            if !struct_name.is_empty() {
                println!("      {:<32}   elem struct: {}", "", struct_name);
            }
        }
    }
}

#[allow(dead_code)]
fn describe_value(datacore: &Datacore, v: &sc_extract::svarog_datacore::Value) -> String {
    use sc_extract::svarog_datacore::{InstanceRef, Value};
    match v {
        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            let inst = datacore.db().instance(r.struct_index, r.instance_index);
            inst.type_name().unwrap_or("?").into()
        }
        Value::Class { struct_index, .. } | Value::ClassRef(InstanceRef { struct_index, .. }) => {
            datacore
                .db()
                .struct_name(*struct_index as usize)
                .unwrap_or("?")
                .into()
        }
        Value::Reference(Some(r)) => format!("Ref→{}", r.guid),
        Value::String(s) | Value::Locale(s) | Value::Enum(s) => format!("{:?}", s),
        Value::Int32(n) => format!("i32({})", n),
        Value::Float(f) => format!("f32({})", f),
        other => format!("{:?}", std::mem::discriminant(other)),
    }
}
