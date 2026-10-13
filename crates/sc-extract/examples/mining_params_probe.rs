//! Mining fracture inputs as the DCB holds them: every `MiningGlobalParams`
//! record, every `MineableElement` with its resource's density, and the
//! `MineableParams.filledFactor` distribution per global-params record.
//!
//! These are the inputs of the client's fracture maths (power capacity per
//! mass, decay, optimal window, resistance curve) — the probe that re-proves
//! the values a mining model is built on.
//!
//! ```bash
//! cargo run -p sc-extract --release --example mining_params_probe
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource};
use svarog_datacore::{DataCoreDatabase, Instance, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let db = datacore.db();

    println!("\n== MiningGlobalParams ==");
    for rec in db.records_by_type("MiningGlobalParams") {
        println!("\n# {}", short(rec.name().unwrap_or("?")));
        dump(db, &rec.as_instance(), 1, 2);
    }

    println!("\n== MineableElement (+ resource density) ==");
    for rec in db.records_by_type("MineableElement") {
        let inst = rec.as_instance();
        let mut line = format!("{:<44}", short(rec.name().unwrap_or("?")));
        for p in inst.properties() {
            if let Some(f) = p.value.as_f32() {
                line.push_str(&format!(" {}={f}", p.name));
            }
        }
        println!("{line}");
        if let Some(r) = inst.get("resourceType").and_then(|v| v.as_record_ref())
            && let Some(res) = db.record(&r.guid)
            && let Some(dt) = res.as_instance().get("densityType")
            && let Some(di) = to_instance(db, &dt)
        {
            println!("    density <{}>:", di.type_name().unwrap_or("?"));
            dump(db, &di, 3, 2);
        }
    }

    println!("\n== MineableParams.filledFactor per globalParams ==");
    let mut by_gp: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    for rec in db.records_by_type("EntityClassDefinition") {
        let Some(comps) = rec.as_instance().get_array("Components") else {
            continue;
        };
        for c in comps {
            let Some(ci) = to_instance(db, &c) else {
                continue;
            };
            if ci.type_name() != Some("MineableParams") {
                continue;
            }
            let gp = ci
                .get("globalParams")
                .and_then(|v| v.as_record_ref())
                .and_then(|r| db.record(&r.guid))
                .and_then(|r| r.name().map(short))
                .unwrap_or_else(|| "?".into());
            let ff = ci
                .get("filledFactor")
                .and_then(|v| v.as_f32())
                .unwrap_or(f32::NAN);
            *by_gp
                .entry(gp)
                .or_default()
                .entry(format!("{ff}"))
                .or_default() += 1;
        }
    }
    for (gp, hist) in &by_gp {
        let total: usize = hist.values().sum();
        let parts: Vec<String> = hist.iter().map(|(v, n)| format!("{v} x{n}")).collect();
        println!(
            "  {gp:<40} {total:>4} rocks   filledFactor: {}",
            parts.join(", ")
        );
    }

    // The client sums additive modifiers as `add = (add + v) / 100` per active
    // entry (CEntityComponentMineable modifier aggregation, LIVE 4.10), so it
    // matters which items carry them at all.
    println!("\n== entities with non-zero FloatModifierAdditive (field: value) ==");
    let mut n_mult = 0usize;
    for rec in db.records_by_type("EntityClassDefinition") {
        let mut hits = Vec::new();
        let mut stack = vec![(rec.as_instance(), String::new(), 0u32)];
        while let Some((inst, field, depth)) = stack.pop() {
            match inst.type_name() {
                Some("FloatModifierAdditive") => {
                    let v = inst.get("value").and_then(|v| v.as_f32()).unwrap_or(0.0);
                    if v != 0.0 {
                        hits.push(format!("{field}: {v}"));
                    }
                }
                Some("FloatModifierMultiplicative") => n_mult += 1,
                _ => {}
            }
            if depth >= 8 {
                continue;
            }
            for p in inst.properties() {
                let children: Vec<Value> = match p.value {
                    Value::Array(_) => inst
                        .get_array(p.name)
                        .map(|a| a.into_iter().collect())
                        .unwrap_or_default(),
                    Value::Class { .. }
                    | Value::ClassRef(_)
                    | Value::StrongPointer(Some(_))
                    | Value::WeakPointer(Some(_)) => vec![p.value],
                    _ => Vec::new(),
                };
                for c in children {
                    if let Some(ci) = to_instance(db, &c) {
                        stack.push((ci, p.name.to_string(), depth + 1));
                    }
                }
            }
        }
        if !hits.is_empty() {
            println!(
                "  {:<48} {}",
                short(rec.name().unwrap_or("?")),
                hits.join(", ")
            );
        }
    }
    println!("  (FloatModifierMultiplicative instances seen on entities: {n_mult})");

    // Laser side: what feeds the per-hit energy. Mining lasers are the
    // entities with SEntityComponentMiningLaserParams; their beam fire action
    // carries damagePerSecond (the in-game "Laser Power") and range falloff.
    println!("\n== mining lasers (SEntityComponentMiningLaserParams + beam fire action) ==");
    for rec in db.records_by_type("EntityClassDefinition") {
        let comps = components(db, &rec.as_instance());
        let Some(laser) = comps
            .iter()
            .find(|c| c.type_name() == Some("SEntityComponentMiningLaserParams"))
        else {
            continue;
        };
        println!("\n# {}", short(rec.name().unwrap_or("?")));
        dump(db, laser, 1, 3);
        for w in comps
            .iter()
            .filter(|c| c.type_name() == Some("SCItemWeaponComponentParams"))
        {
            for fa in w.get_array("fireActions").into_iter().flatten() {
                let Some(fi) = to_instance(db, &fa) else {
                    continue;
                };
                println!(
                    "  fireAction '{}' <{}>:",
                    fi.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
                    fi.type_name().unwrap_or("?")
                );
                for key in [
                    "fullDamageRange",
                    "zeroDamageRange",
                    "minEnergyDraw",
                    "maxEnergyDraw",
                ] {
                    if let Some(v) = fi.get(key).and_then(|v| v.as_f32()) {
                        println!("    {key} = {v}");
                    }
                }
                if let Some(d) = fi.get("damagePerSecond").and_then(|v| to_instance(db, &v)) {
                    println!("    damagePerSecond <{}>:", d.type_name().unwrap_or("?"));
                    dump(db, &d, 3, 2);
                }
            }
        }
    }

    // Modules and gadgets nest their modifiers (attachable-modifier lists), so
    // search the whole entity tree. SWeaponStats.damageMultiplier is the
    // weapon-side power multiplier ("DPS x" in community tools).
    println!("\n== mining modules / gadgets (nested modifier params) ==");
    const WANT: [&str; 3] = [
        "ItemMiningModifierParams",
        "ItemMineableRockModifierParams",
        "ItemMiningBoosterParams",
    ];
    for rec in db.records_by_type("EntityClassDefinition") {
        let root = rec.as_instance();
        if components(db, &root)
            .iter()
            .any(|c| c.type_name() == Some("SEntityComponentMiningLaserParams"))
        {
            continue;
        }
        let mut found: Vec<(String, Instance)> = Vec::new();
        let mut stack = vec![(root, String::new(), 0u32)];
        while let Some((inst, field, depth)) = stack.pop() {
            let ty = inst.type_name().unwrap_or("");
            let dmg = (ty == "SWeaponStats")
                .then(|| inst.get("damageMultiplier").and_then(|v| v.as_f32()))
                .flatten();
            if WANT.contains(&ty) || dmg.is_some_and(|d| d != 1.0) {
                found.push((field.clone(), inst));
                continue;
            }
            if depth >= 10 {
                continue;
            }
            for p in inst.properties() {
                let children: Vec<Value> = match p.value {
                    Value::Array(_) => inst
                        .get_array(p.name)
                        .map(|a| a.into_iter().collect())
                        .unwrap_or_default(),
                    Value::Class { .. }
                    | Value::ClassRef(_)
                    | Value::StrongPointer(Some(_))
                    | Value::WeakPointer(Some(_)) => vec![p.value],
                    _ => Vec::new(),
                };
                for c in children {
                    if let Some(ci) = to_instance(db, &c) {
                        stack.push((ci, p.name.to_string(), depth + 1));
                    }
                }
            }
        }
        if found.is_empty() {
            continue;
        }
        println!("\n# {}", short(rec.name().unwrap_or("?")));
        for (field, inst) in &found {
            println!("  {field} <{}>:", inst.type_name().unwrap_or("?"));
            dump(db, inst, 2, 3);
        }
    }
    Ok(())
}

/// The entity's `Components[]` as instances.
fn components<'a>(db: &'a DataCoreDatabase, ent: &Instance<'a>) -> Vec<Instance<'a>> {
    ent.get_array("Components")
        .map(|a| a.into_iter().filter_map(|c| to_instance(db, &c)).collect())
        .unwrap_or_default()
}

/// Print scalar properties, then descend into nested classes / pointers.
fn dump(db: &DataCoreDatabase, inst: &Instance, indent: usize, depth: u32) {
    let pad = "  ".repeat(indent);
    for p in inst.properties() {
        match &p.value {
            Value::Class { .. } | Value::StrongPointer(Some(_)) | Value::WeakPointer(Some(_)) => {
                if let Some(ci) = to_instance(db, &p.value) {
                    println!("{pad}{} <{}>:", p.name, ci.type_name().unwrap_or("?"));
                    if depth > 0 {
                        dump(db, &ci, indent + 1, depth - 1);
                    }
                }
            }
            Value::Reference(Some(r)) => {
                let n = db.record(&r.guid).and_then(|r| r.name().map(short));
                println!("{pad}{} -> {}", p.name, n.unwrap_or_else(|| "?".into()));
            }
            Value::Array(_) => {
                let n = inst
                    .get_array(p.name)
                    .map(|a| a.into_iter().count())
                    .unwrap_or(0);
                println!("{pad}{} [{n}]", p.name);
            }
            v => {
                if let Some(s) = scalar(v) {
                    println!("{pad}{} = {s}", p.name);
                }
            }
        }
    }
}

fn scalar(v: &Value) -> Option<String> {
    Some(match v {
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Double(f) => f.to_string(),
        Value::Int32(i) => i.to_string(),
        Value::UInt32(i) => i.to_string(),
        Value::Int64(i) => i.to_string(),
        Value::Enum(s) | Value::String(s) => (*s).to_string(),
        _ => return None,
    })
}

fn to_instance<'a>(db: &'a DataCoreDatabase, v: &Value<'a>) -> Option<Instance<'a>> {
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

fn short(name: &str) -> String {
    name.rsplit('.').next().unwrap_or(name).to_string()
}
