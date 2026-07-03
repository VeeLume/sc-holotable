//! Throwaway probe: where do ship cargo grids live in the DCB?
//!
//! Phase-2 reality check for the cargo-grid exploration. Answers:
//!   1. How many `SCItemCargoGridParams` instances actually exist (the schema
//!      marks the type `dormant` = "never observed" — verify/disprove)?
//!   2. Which entity classes carry a cargo-grid component, and what are its
//!      `dimensions` (the physical grid box, in metres)?
//!   3. What cargo-related component types appear across all entity classes?
//!   4. Full field dump of one named ship's cargo structure (--entity <substr>).
//!
//! ```bash
//! cargo run -p sc-extract --release --example cargo_grid_probe
//! cargo run -p sc-extract --release --example cargo_grid_probe -- --entity Freelancer
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entity_filter: Option<String> = None;
    let mut guid_filter: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--entity" => entity_filter = args.next(),
            "--guid" => {
                if let Some(g) = args.next() {
                    guid_filter.push(g.to_lowercase());
                }
            }
            _ => {}
        }
    }

    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    // (1) top-level record count for the cargo-grid params type.
    let top_level = db.records_by_type("SCItemCargoGridParams").count();
    println!("\n== SCItemCargoGridParams as top-level record: {top_level} ==");

    // (2)+(3) walk every EntityClassDefinition component tree.
    let mut cargo_type_tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut grid_bearers: Vec<(String, String)> = Vec::new(); // (entity, dims)
    let mut n_entities = 0usize;

    for rec in db.records_by_type("EntityClassDefinition") {
        n_entities += 1;
        let name = rec.name().unwrap_or("?").to_string();
        let inst = rec.as_instance();
        let mut seen_grid = false;
        let mut local_types: BTreeMap<String, usize> = BTreeMap::new();

        walk(db, &inst, 8, &mut 40_000, &mut |ci| {
            if let Some(tn) = ci.type_name() {
                if tn.contains("Cargo") {
                    *local_types.entry(tn.to_string()).or_default() += 1;
                }
                if tn == "SCItemCargoGridParams" && !seen_grid {
                    seen_grid = true;
                    let dims = ci
                        .get_instance("dimensions")
                        .map(|d| {
                            format!(
                                "({}, {}, {})",
                                d.get_f32("x").unwrap_or(0.0),
                                d.get_f32("y").unwrap_or(0.0),
                                d.get_f32("z").unwrap_or(0.0)
                            )
                        })
                        .unwrap_or_else(|| "<none>".into());
                    let mining = ci.get_bool("miningOnly").unwrap_or(false);
                    grid_bearers.push((name.clone(), format!("dims={dims} miningOnly={mining}")));
                }
            }
        });
        for (t, n) in local_types {
            *cargo_type_tally.entry(t).or_default() += n;
        }
    }

    println!("\n== entity classes scanned: {n_entities} ==");
    println!("\n== cargo-related component types (type -> #entities carrying) ==");
    for (t, n) in &cargo_type_tally {
        println!("  {n:>5}  {t}");
    }

    println!(
        "\n== entities carrying SCItemCargoGridParams: {} ==",
        grid_bearers.len()
    );
    for (e, d) in grid_bearers.iter().take(60) {
        println!("  {e:<48} {d}");
    }

    // (4) full dump of one named entity's cargo structure.
    if let Some(filter) = &entity_filter {
        println!("\n== entity dump: names containing '{filter}' ==");
        for rec in db.records_by_type("EntityClassDefinition") {
            let name = rec.name().unwrap_or("?");
            if !name.to_lowercase().contains(&filter.to_lowercase()) {
                continue;
            }
            println!("\n--- {name}  (guid {}) ---", rec.id());
            let inst = rec.as_instance();

            // Top-level Components array: list every component type name.
            if let Some(comps) = inst.get_array("Components") {
                println!("  [top-level Components]");
                for elem in comps {
                    if let Some(ci) = value_to_instance(db, &elem) {
                        println!("    - {}", ci.type_name().unwrap_or("?"));
                    }
                }
            }

            // Geometry: any string field that looks like a .cga/.cgf path.
            {
                let mut paths: std::collections::BTreeSet<String> =
                    std::collections::BTreeSet::new();
                walk(db, &inst, 12, &mut 400_000, &mut |ci| {
                    for p in ci.properties() {
                        if let Some(s) = p.value.as_str() {
                            let l = s.to_lowercase();
                            if l.ends_with(".cga") || l.ends_with(".cgf") {
                                paths.insert(s.to_string());
                            }
                        }
                    }
                });
                for pth in paths.iter().take(12) {
                    println!("  [geometry] {pth}");
                }
            }

            // Loadout: which cargo-grid entity classes mount at which item ports.
            walk(db, &inst, 14, &mut 400_000, &mut |ci| {
                if ci.type_name() == Some("SItemPortLoadoutEntryParams") {
                    let cls = ci.get_str("entityClassName").unwrap_or("");
                    if cls.to_lowercase().contains("cargogrid")
                        || cls.to_lowercase().contains("cargo_grid")
                    {
                        let port = ci.get_str("itemPortName").unwrap_or("?");
                        println!("  [loadout] port '{port}' -> {cls}");
                    }
                }
            });

            // Anywhere in the tree: dump full fields of any component whose type
            // name hints at cargo / grid / inventory / capacity / volume / room.
            walk(db, &inst, 12, &mut 400_000, &mut |ci| {
                if let Some(tn) = ci.type_name() {
                    let l = tn.to_lowercase();
                    let hit = ["cargo", "grid", "inventory", "capacit", "volume", "room"]
                        .iter()
                        .any(|k| l.contains(k));
                    if hit {
                        println!("  <{tn}>");
                        for p in ci.properties() {
                            println!("      {} = {}", p.name, short_val(&p.value));
                        }
                    }
                }
            });
        }
    }

    // (4b) InventoryContainer census — every grid record matching --containers <substr>.
    if let Some(sub) = std::env::args().skip_while(|a| a != "--containers").nth(1) {
        let sub = sub.to_lowercase();
        println!("\n== InventoryContainer records matching '{sub}' ==");
        for rec in db.records_by_type("InventoryContainer") {
            let name = rec.name().unwrap_or("?");
            if !name.to_lowercase().contains(&sub) {
                continue;
            }
            let inst = rec.as_instance();
            let dims = inst
                .get_instance("interiorDimensions")
                .map(|d| {
                    let (x, y, z) = (
                        d.get_f32("x").unwrap_or(0.0),
                        d.get_f32("y").unwrap_or(0.0),
                        d.get_f32("z").unwrap_or(0.0),
                    );
                    let scu = (x / 1.25).round() * (y / 1.25).round() * (z / 1.25).round();
                    format!("{x:.2}x{y:.2}x{z:.2}  = {scu:.0} SCU")
                })
                .unwrap_or_else(|| "<no dims>".into());
            let open = inst
                .get_instance("inventoryType")
                .and_then(|it| it.type_name().map(|s| s.to_string()))
                .unwrap_or_else(|| "?".into());
            println!(
                "  {:<48} {dims:<28} {open}",
                name.rsplit('.').next().unwrap_or(name)
            );
        }
    }

    // (5) resolve arbitrary record GUIDs and dump their full field tree.
    if !guid_filter.is_empty() {
        println!("\n== record dumps for {} GUID(s) ==", guid_filter.len());
        for rec in db.all_records() {
            let id = rec.id().to_string().to_lowercase();
            if !guid_filter.iter().any(|g| id.contains(g)) {
                continue;
            }
            println!(
                "\n### {}  type={}  name={}",
                rec.id(),
                rec.struct_index(),
                rec.name().unwrap_or("?"),
            );
            let inst = rec.as_instance();
            println!("   (type: {})", inst.type_name().unwrap_or("?"));
            dump_fields(db, &inst, 0, 4);
        }
    }

    Ok(())
}

/// Recursively print scalar + nested fields (bounded depth), for record inspection.
fn dump_fields<'a>(db: &'a DataCoreDatabase, inst: &Instance<'a>, depth: u32, max: u32) {
    let pad = "  ".repeat(depth as usize + 1);
    for p in inst.properties() {
        match &p.value {
            Value::Array(a) => {
                println!("{pad}{} : <Array len={}>", p.name, a.count);
                if depth < max
                    && let Some(arr) = inst.get_array(p.name)
                {
                    for (i, elem) in arr.enumerate().take(8) {
                        if let Some(ci) = value_to_instance(db, &elem) {
                            println!("{pad}  [{i}] <{}>", ci.type_name().unwrap_or("?"));
                            dump_fields(db, &ci, depth + 1, max);
                        } else {
                            println!("{pad}  [{i}] {}", short_val(&elem));
                        }
                    }
                }
            }
            Value::Class { .. }
            | Value::ClassRef(_)
            | Value::StrongPointer(Some(_))
            | Value::WeakPointer(Some(_)) => {
                if let Some(ci) = value_to_instance(db, &p.value) {
                    println!("{pad}{} : <{}>", p.name, ci.type_name().unwrap_or("?"));
                    if depth < max {
                        dump_fields(db, &ci, depth + 1, max);
                    }
                } else {
                    println!("{pad}{} : {}", p.name, short_val(&p.value));
                }
            }
            other => println!("{pad}{} : {}", p.name, short_val(other)),
        }
    }
}

/// Iterative component-tree walk with depth + node budget. Calls `visit` on
/// every reachable nested instance (Class / ClassRef / Strong / Weak / Array
/// elements).
fn walk<'a>(
    db: &'a DataCoreDatabase,
    root: &Instance<'a>,
    max_depth: u32,
    budget: &mut u32,
    visit: &mut dyn FnMut(&Instance<'a>),
) {
    let mut stack: Vec<(Instance<'a>, u32)> = vec![(*root, 0)];
    while let Some((inst, depth)) = stack.pop() {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        visit(&inst);
        if depth >= max_depth {
            continue;
        }
        for p in inst.properties() {
            match p.value {
                Value::Array(_) => {
                    if let Some(arr) = inst.get_array(p.name) {
                        for elem in arr {
                            if let Some(ci) = value_to_instance(db, &elem) {
                                stack.push((ci, depth + 1));
                            }
                        }
                    }
                }
                Value::Class { .. }
                | Value::ClassRef(_)
                | Value::StrongPointer(Some(_))
                | Value::WeakPointer(Some(_)) => {
                    if let Some(ci) = value_to_instance(db, &p.value) {
                        stack.push((ci, depth + 1));
                    }
                }
                _ => {}
            }
        }
    }
}

fn value_to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
    match v {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}

fn short_val(v: &Value) -> String {
    match v {
        Value::Class { .. } => "<Class>".into(),
        Value::ClassRef(_) => "<ClassRef>".into(),
        Value::StrongPointer(_) => "<StrongPtr>".into(),
        Value::WeakPointer(_) => "<WeakPtr>".into(),
        Value::Array(a) => format!("<Array len={}>", a.count),
        other => format!("{other:?}"),
    }
}
