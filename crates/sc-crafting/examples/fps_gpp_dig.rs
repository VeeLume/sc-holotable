//! Investigation: from the FPS-weapon *entity* side, what stat-bearing
//! structure exists, and is there ANY data-driven (non-name, non-GUID-table)
//! relation to the crafting gameplay-property defs (GPP) that recipes modify?
//!
//! Five sections:
//!   1. GPP catalog — every CraftingGameplayPropertyDef + ALL its raw fields
//!      (confirms whether a hidden "property path / target field" exists).
//!   2. Reverse references — ReferenceGraph.incoming() for each GPP, grouped
//!      by the referring record's TYPE. Authoritative for "does anything
//!      outside the crafting recipe tree reference a GPP?".
//!   3. Craftable FPS weapons — blueprints whose crafted entity is a
//!      WeaponPersonal (non-Gadget).
//!   4. Entity stat-field dump — deep raw dump of the weapon/health/ammo/
//!      attachable components of a few sample weapons: every scalar leaf with
//!      its STABLE structural path. This is what a T1 crate would read.
//!   5. Recipe GPPs per weapon + entity-side GPP-reference scan (catches
//!      Guid/pointer edges the Reference-only graph in §2 would miss).
//!
//! Read-only. Run:
//! ```bash
//! cargo run -p sc-crafting --release --example fps_gpp_dig > /tmp/fps_gpp_dig.txt
//! ```

#![allow(clippy::too_many_arguments)] // throwaway investigation probe — recursive dumpers thread several params

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_crafting::{Blueprints, Cost, CostContext, GameplayProperties, GameplayPropertyModifier};
use sc_extract::generated::{EItemSubType, EItemType};
use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataCoreDatabase, Datacore, Guid, Instance, LocaleMap,
    ReferenceGraph, Value,
};
use sc_items::{Items, RecordCollection};

const MAX_DEPTH: usize = 9;
const MAX_ELEMS: usize = 12;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let locale = &asset_data.locale;
    let db = datacore.db();

    // ── GPP catalog: guid -> (record_name, display, unit_key, raw_field_dump) ──
    let gp = GameplayProperties::build(&datacore);
    let mut gpp_set: HashSet<Guid> = HashSet::new();
    let mut gpp_name: HashMap<Guid, String> = HashMap::new(); // guid -> record name
    let mut gpp_disp: HashMap<Guid, String> = HashMap::new(); // guid -> display name

    println!("=========================================================");
    println!(
        "§1  GPP CATALOG  ({} CraftingGameplayPropertyDef records)",
        gp.len()
    );
    println!("=========================================================");
    for prop in gp.iter() {
        gpp_set.insert(prop.guid);
        let rec_name = db
            .record(&prop.guid)
            .and_then(|r| r.name())
            .unwrap_or("?")
            .to_string();
        let disp = locale
            .resolve(&prop.property_name_key)
            .unwrap_or_else(|| prop.property_name_key.as_str())
            .to_string();
        gpp_name.insert(prop.guid, rec_name.clone());
        gpp_disp.insert(prop.guid, disp.clone());

        println!("\n  {rec_name}");
        println!("    display      : {disp:?}");
        println!("    propertyName : {}", prop.property_name_key.as_str());
        println!("    unitFormat   : {}", prop.unit_format_key.as_str());
        println!("    transform    : {:?}", prop.display_transformation);
        // RAW field dump straight off the record instance — exhaustive, in
        // case the typed model dropped a field (e.g. a target-property path).
        if let Some(rec) = db.record(&prop.guid) {
            println!("    -- raw fields --");
            for p in rec.as_instance().properties() {
                println!("      {} : {}", p.name, render_value(db, &p.value));
            }
        }
    }

    // ── §2  Reverse references to each GPP ──
    println!("\n\n=========================================================");
    println!("§2  REVERSE REFERENCES TO GPPs  (ReferenceGraph.incoming)");
    println!("    cross-record Value::Reference edges; grouped by referrer type");
    println!("=========================================================");
    let graph = ReferenceGraph::from_database(db);
    // Aggregate: referrer-type -> count, across all GPPs.
    let mut by_referrer_type: BTreeMap<String, usize> = BTreeMap::new();
    let mut total_incoming = 0usize;
    for guid in gpp_set.iter() {
        let incoming = graph.incoming(guid);
        total_incoming += incoming.len();
        for src in incoming {
            let t = db
                .record(src)
                .and_then(|r| r.type_name())
                .unwrap_or("?")
                .to_string();
            *by_referrer_type.entry(t).or_default() += 1;
        }
    }
    println!(
        "\n  total incoming Reference edges across {} GPPs: {total_incoming}",
        gpp_set.len()
    );
    println!("  distinct referrer record TYPES:");
    for (t, n) in &by_referrer_type {
        println!("    {t:<52} : {n}");
    }
    // Per-GPP, list any referrer type that is NOT a crafting type — the
    // interesting discovery, if any.
    println!("\n  per-GPP referrer types that are NOT 'Crafting*':");
    let mut found_non_crafting = false;
    for guid in gpp_set.iter() {
        let mut non_crafting: BTreeMap<String, usize> = BTreeMap::new();
        for src in graph.incoming(guid) {
            if let Some(t) = db.record(src).and_then(|r| r.type_name())
                && !t.starts_with("Crafting")
            {
                *non_crafting.entry(t.to_string()).or_default() += 1;
            }
        }
        if !non_crafting.is_empty() {
            found_non_crafting = true;
            println!(
                "    {} -> {non_crafting:?}",
                gpp_name.get(guid).map(|s| s.as_str()).unwrap_or("?")
            );
        }
    }
    if !found_non_crafting {
        println!("    (none — every GPP referrer is a Crafting* record)");
    }

    // ── §3  Craftable FPS weapons ──
    let blueprints = Blueprints::build(&datacore, &items);
    let mut fps: Vec<(Guid, Guid, String)> = Vec::new(); // (bp_guid, entity_guid, name)
    for bp in blueprints.values() {
        let Some(ent) = bp.crafted_entity_guid() else {
            continue;
        };
        let Some(ty) = items.item_type(&ent) else {
            continue;
        };
        if *ty != EItemType::WeaponPersonal {
            continue;
        }
        if matches!(items.item_sub_type(&ent), Some(EItemSubType::Gadget)) {
            continue;
        }
        let name = bp
            .display_name(locale)
            .map(|s| s.to_string())
            .or_else(|| {
                db.record(&ent)
                    .and_then(|r| r.name())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "?".into());
        fps.push((bp.blueprint_record_guid, ent, name));
    }
    fps.sort_by(|a, b| a.2.cmp(&b.2));
    println!("\n\n=========================================================");
    println!("§3  CRAFTABLE FPS WEAPONS  ({} blueprints)", fps.len());
    println!("=========================================================");
    for (_, ent, name) in &fps {
        let rn = db.record(ent).and_then(|r| r.name()).unwrap_or("?");
        println!("  {name:<34} {rn}");
    }

    // ── §4  Entity stat-field dump (samples) ──
    let focus = |t: &str| {
        t.contains("Weapon")
            || t.contains("Health")
            || t.contains("Ammo")
            || t.contains("Attachable")
            || t.contains("ItemResource")
            || t.contains("Heat")
            || t.contains("Durability")
    };
    println!("\n\n=========================================================");
    println!("§4  ENTITY STAT-FIELD DUMP  (first 3 craftable FPS weapons)");
    println!("    full component-type list + deep dump of focus components");
    println!("=========================================================");
    for (_, ent, name) in fps.iter().take(3) {
        let Some(rec) = db.record(ent) else { continue };
        println!("\n────────────────────────────────────────────────────────");
        println!("ENTITY  {name}   [{}]", rec.name().unwrap_or("?"));
        println!("────────────────────────────────────────────────────────");
        let inst = rec.as_instance();
        let comps = collect_components(db, &inst);
        println!("  components ({}):", comps.len());
        for c in &comps {
            println!("    - {}", c.type_name().unwrap_or("?"));
        }
        for c in &comps {
            let tn = c.type_name().unwrap_or("?");
            if !focus(tn) {
                continue;
            }
            println!("\n  ╔═ {tn} ═══");
            let mut lines = Vec::new();
            let mut budget = 60_000usize;
            dump(db, c, tn, 0, &gpp_set, &mut lines, &mut budget);
            for l in &lines {
                println!("  {l}");
            }
        }
    }

    // ── §5  Recipe GPPs per weapon + entity-side GPP-reference scan ──
    println!("\n\n=========================================================");
    println!("§5  RECIPE GPPs PER WEAPON  +  entity-side GPP-ref scan");
    println!("=========================================================");
    let mut any_entity_gpp_hit = false;
    for (bp_guid, ent, name) in &fps {
        let Some(bp) = blueprints.get(bp_guid) else {
            continue;
        };
        // Collect (slot, modifiers) over the typed cost tree.
        let mut slots: Vec<(String, Vec<&GameplayPropertyModifier>)> = Vec::new();
        for tier in &bp.tiers {
            if let Some(recipe) = &tier.recipe
                && let Some(costs) = &recipe.costs
                && let Some(mc) = &costs.mandatory
            {
                walk_cost(mc, "(top)", locale, &mut slots);
            }
        }
        println!("\n  {name}");
        if slots.is_empty() {
            println!("    (no gameplay-property modifiers)");
        }
        for (slot, mods) in &slots {
            for m in mods {
                let gname = m
                    .gameplay_property
                    .and_then(|g| gpp_disp.get(&g).cloned())
                    .unwrap_or_else(|| "?".into());
                let grec = m
                    .gameplay_property
                    .and_then(|g| gpp_name.get(&g).cloned())
                    .unwrap_or_else(|| "?".into());
                let bands: Vec<String> = m
                    .value_ranges
                    .iter()
                    .filter_map(|vr| vr.quality_band())
                    .map(|(s, e)| format!("Q{s}-{e}"))
                    .collect();
                println!("    [{slot}] {gname}  ({grec})  bands={}", bands.join(","));
            }
        }
        // Entity-side: does this weapon's entity tree reference any GPP guid
        // (Reference / Guid / pointer-to-GPP-instance)?
        if let Some(rec) = db.record(ent) {
            let inst = rec.as_instance();
            let mut hits: Vec<String> = Vec::new();
            let mut budget = 400_000usize;
            scan_gpp_refs(
                db,
                &inst,
                "",
                0,
                &gpp_set,
                &gpp_name,
                &mut hits,
                &mut budget,
            );
            if !hits.is_empty() {
                any_entity_gpp_hit = true;
                println!("    !! ENTITY REFERENCES GPP:");
                for h in hits.iter().take(20) {
                    println!("       {h}");
                }
            }
        }
    }
    println!(
        "\n  entity-side GPP references found in any FPS weapon entity: {}",
        if any_entity_gpp_hit { "YES" } else { "NO" }
    );

    Ok(())
}

/// Collect every component instance off an entity record (scans all array
/// props, keeps elements whose type name contains "Component").
fn collect_components<'a>(db: &'a DataCoreDatabase, entity: &Instance<'a>) -> Vec<Instance<'a>> {
    let mut out = Vec::new();
    for p in entity.properties() {
        if let Value::Array(_) = p.value
            && let Some(arr) = entity.get_array(p.name)
        {
            for elem in arr {
                if let Some(ci) = value_to_instance(db, &elem)
                    && ci
                        .type_name()
                        .map(|t| t.contains("Component"))
                        .unwrap_or(false)
                {
                    out.push(ci);
                }
            }
        }
    }
    out
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

/// Recursive dump: every scalar leaf with its structural path. Recurses into
/// Class/pointer children, follows arrays (capped). References are printed
/// with their target record TYPE.
fn dump(
    db: &DataCoreDatabase,
    inst: &Instance<'_>,
    path: &str,
    depth: usize,
    gpp_set: &HashSet<Guid>,
    out: &mut Vec<String>,
    budget: &mut usize,
) {
    if *budget == 0 {
        return;
    }
    for p in inst.properties() {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        let child_path = format!("{path}.{}", p.name);
        match &p.value {
            Value::Array(_) => {
                if let Some(arr) = inst.get_array(p.name) {
                    let total = arr.len();
                    if total == 0 {
                        continue;
                    }
                    for (i, elem) in inst.get_array(p.name).unwrap().enumerate() {
                        if i >= MAX_ELEMS {
                            out.push(format!(
                                "{child_path}[..{total}] (+{} more)",
                                total - MAX_ELEMS
                            ));
                            break;
                        }
                        dump_value(
                            db,
                            &elem,
                            &format!("{child_path}[{i}]"),
                            depth,
                            gpp_set,
                            out,
                            budget,
                        );
                    }
                }
            }
            v => dump_value(db, v, &child_path, depth, gpp_set, out, budget),
        }
    }
}

fn dump_value(
    db: &DataCoreDatabase,
    v: &Value<'_>,
    path: &str,
    depth: usize,
    gpp_set: &HashSet<Guid>,
    out: &mut Vec<String>,
    budget: &mut usize,
) {
    match v {
        Value::Bool(_)
        | Value::Int8(_)
        | Value::Int16(_)
        | Value::Int32(_)
        | Value::Int64(_)
        | Value::UInt8(_)
        | Value::UInt16(_)
        | Value::UInt32(_)
        | Value::UInt64(_)
        | Value::Float(_)
        | Value::Double(_) => out.push(format!("{path} = {v}")),
        Value::String(s) | Value::Locale(s) | Value::Enum(s) => {
            if !s.is_empty() {
                out.push(format!("{path} = {s:?}"));
            }
        }
        Value::Guid(g) => {
            let tag = if gpp_set.contains(g) {
                "  <<< GPP GUID!"
            } else {
                ""
            };
            out.push(format!("{path} = guid {g}{tag}"));
        }
        Value::Reference(Some(r)) => {
            let tt = db
                .record(&r.guid)
                .and_then(|x| x.type_name())
                .unwrap_or("?");
            let tag = if gpp_set.contains(&r.guid) {
                "  <<< GPP GUID!"
            } else {
                ""
            };
            out.push(format!("{path} -> Ref({tt}){tag}"));
        }
        Value::Class { .. }
        | Value::ClassRef(_)
        | Value::StrongPointer(Some(_))
        | Value::WeakPointer(Some(_)) => {
            if depth >= MAX_DEPTH {
                if let Some(ci) = value_to_instance(db, v) {
                    out.push(format!(
                        "{path} : {} (depth cap)",
                        ci.type_name().unwrap_or("?")
                    ));
                }
                return;
            }
            if let Some(ci) = value_to_instance(db, v) {
                let tn = ci.type_name().unwrap_or("?");
                // Tag inline nested type for orientation.
                out.push(format!("{path} : {tn}"));
                dump(db, &ci, path, depth + 1, gpp_set, out, budget);
            }
        }
        _ => {} // Null / null pointers / null refs — skip
    }
}

/// Lightweight scan: walk the entity tree and record any GPP-guid reference
/// (Reference, raw Guid, or pointer/class resolving to a GPP record type).
fn scan_gpp_refs(
    db: &DataCoreDatabase,
    inst: &Instance<'_>,
    path: &str,
    depth: usize,
    gpp_set: &HashSet<Guid>,
    gpp_name: &HashMap<Guid, String>,
    hits: &mut Vec<String>,
    budget: &mut usize,
) {
    if *budget == 0 || depth > MAX_DEPTH {
        return;
    }
    for p in inst.properties() {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        let cp = format!("{path}.{}", p.name);
        match &p.value {
            Value::Guid(g) if gpp_set.contains(g) => {
                hits.push(format!(
                    "{cp} = guid {}",
                    gpp_name.get(g).map(|s| s.as_str()).unwrap_or("?")
                ));
            }
            Value::Reference(Some(r)) if gpp_set.contains(&r.guid) => {
                hits.push(format!(
                    "{cp} -> Ref {}",
                    gpp_name.get(&r.guid).map(|s| s.as_str()).unwrap_or("?")
                ));
            }
            Value::Array(_) => {
                if let Some(arr) = inst.get_array(p.name) {
                    for (i, elem) in arr.enumerate() {
                        scan_value(
                            db,
                            &elem,
                            &format!("{cp}[{i}]"),
                            depth,
                            gpp_set,
                            gpp_name,
                            hits,
                            budget,
                        );
                    }
                }
            }
            v => scan_value(db, v, &cp, depth, gpp_set, gpp_name, hits, budget),
        }
    }
}

fn scan_value(
    db: &DataCoreDatabase,
    v: &Value<'_>,
    path: &str,
    depth: usize,
    gpp_set: &HashSet<Guid>,
    gpp_name: &HashMap<Guid, String>,
    hits: &mut Vec<String>,
    budget: &mut usize,
) {
    match v {
        Value::Guid(g) if gpp_set.contains(g) => {
            hits.push(format!(
                "{path} = guid {}",
                gpp_name.get(g).map(|s| s.as_str()).unwrap_or("?")
            ));
        }
        Value::Reference(Some(r)) if gpp_set.contains(&r.guid) => {
            hits.push(format!(
                "{path} -> Ref {}",
                gpp_name.get(&r.guid).map(|s| s.as_str()).unwrap_or("?")
            ));
        }
        Value::Class { .. }
        | Value::ClassRef(_)
        | Value::StrongPointer(Some(_))
        | Value::WeakPointer(Some(_)) => {
            if let Some(ci) = value_to_instance(db, v) {
                if ci.type_name() == Some("CraftingGameplayPropertyDef") {
                    hits.push(format!("{path} -> ptr to CraftingGameplayPropertyDef"));
                }
                scan_gpp_refs(db, &ci, path, depth + 1, gpp_set, gpp_name, hits, budget);
            }
        }
        _ => {}
    }
}

/// Walk the typed cost tree, attributing each node's gameplay-property
/// modifiers to the nearest named slot.
fn walk_cost<'a>(
    cost: &'a Cost,
    parent_slot: &str,
    locale: &LocaleMap,
    out: &mut Vec<(String, Vec<&'a GameplayPropertyModifier>)>,
) {
    let slot = match cost {
        Cost::Select {
            name_info: Some(n), ..
        } => locale
            .resolve(&n.display_name)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty() && !s.contains("PLACEHOLDER"))
            .unwrap_or_else(|| n.debug_name.clone()),
        _ => parent_slot.to_string(),
    };
    let mut mods: Vec<&GameplayPropertyModifier> = Vec::new();
    for ctx in cost.context() {
        if let CostContext::GameplayPropertyModifiers(ms) = ctx {
            mods.extend(ms.iter());
        }
    }
    if !mods.is_empty() {
        out.push((slot.clone(), mods));
    }
    if let Cost::Select { options, .. } = cost {
        for o in options {
            walk_cost(o, &slot, locale, out);
        }
    }
}

/// Compact one-line render of a raw Value for the §1 field dump.
fn render_value(db: &DataCoreDatabase, v: &Value<'_>) -> String {
    match v {
        Value::Reference(Some(r)) => {
            let tt = db
                .record(&r.guid)
                .and_then(|x| x.type_name())
                .unwrap_or("?");
            format!("Ref({tt} {})", r.guid)
        }
        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) | Value::ClassRef(r) => {
            let tn = db
                .instance(r.struct_index, r.instance_index)
                .type_name()
                .unwrap_or("?");
            format!("ptr({tn})")
        }
        Value::Class { struct_index, .. } => {
            format!(
                "class({})",
                db.struct_name(*struct_index as usize).unwrap_or("?")
            )
        }
        Value::Array(a) => format!("array[{}]", a.count),
        other => format!("{other}"),
    }
}
