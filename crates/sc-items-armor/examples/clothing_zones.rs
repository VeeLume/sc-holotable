//! What a worn item covers and hides, read from `SCItemClothingParams`:
//! `Chunks` (body mesh zones with their layer), `HiddenParts` (item ports the
//! piece hides) and `Flight.gForceResistance`.
//!
//! Prints a census over every entity carrying the component. `--dump` adds one
//! line per item. The reference table holds values style-citizen.com publishes
//! for items that exist on LIVE; a mismatch is reported and fails the run.
//!
//! ```bash
//! cargo run -p sc-items-armor --release --example clothing_zones -- [--dump]
//! ```

use std::collections::{BTreeMap, BTreeSet};

use sc_extract::{
    AssetConfig, AssetData, AssetSource, DataCoreDatabase, Datacore, Instance, Value,
};

struct Chunk {
    zone: String,
    layer: i32,
    visible_layers: Vec<i32>,
    conditions: usize,
    condition_ports: Vec<String>,
    hide_in_first_person: bool,
}

struct Row {
    name: String,
    item_type: String,
    chunks: Vec<Chunk>,
    hides: Vec<String>,
    g_force: Option<f32>,
}

/// `(record, covered zones, G-force resistance)`.
type Reference = (&'static str, &'static [&'static str], f32);

/// Published by style-citizen.com (`data/outfit_data.json`, PTU 5.0.0) and
/// matching LIVE 4.10.2. Layers and hidden ports are left out on purpose: the
/// site's differ from LIVE (armor layer 2 vs 4, `Torso_2`, which LIVE lacks)
/// and without a PTU install a build change can't be told from its export.
const REFERENCE: &[Reference] = &[
    ("987_hat_01_01_01", &[], 0.0),
    ("fta_medium_helmet_01_01_12", &["theta_head_zone"], -0.0625),
    (
        "cds_combat_heavy_arms_03_05_01",
        &["l_hand_zone", "omega_arms_zone", "r_hand_zone"],
        -0.125,
    ),
    (
        "outlaw_legacy_heavy_armor_core_02_01_10",
        &["omega_core_zone"],
        -0.5,
    ),
    (
        "rrs_specialist_heavy_legs_01_01_01",
        &["omega_legs_zone"],
        -0.25,
    ),
    (
        "dmc_pants_04_01_19",
        &[
            "hips_zone",
            "l_leg01_zone",
            "l_leg02_zone",
            "l_leg03_zone",
            "omega_legs_zone",
            "r_leg01_zone",
            "r_leg02_zone",
            "r_leg03_zone",
            "underwear_zone",
        ],
        0.0,
    ),
];

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

fn instances<'a>(db: &'a DataCoreDatabase, inst: &Instance<'a>, field: &str) -> Vec<Instance<'a>> {
    inst.get_array(field)
        .map(|arr| arr.filter_map(|v| value_to_instance(db, &v)).collect())
        .unwrap_or_default()
}

fn component<'a>(
    db: &'a DataCoreDatabase,
    entity: &Instance<'a>,
    type_name: &str,
) -> Option<Instance<'a>> {
    instances(db, entity, "Components")
        .into_iter()
        .find(|c| c.type_name() == Some(type_name))
}

fn read_chunk<'a>(db: &'a DataCoreDatabase, c: &Instance<'a>) -> Chunk {
    let conditions = instances(db, c, "VisibilityConditions");
    let mut condition_ports = Vec::new();
    for condition in &conditions {
        for port in instances(db, condition, "PortVisibility") {
            if let Some(ports) = port.get_array("Ports") {
                condition_ports.extend(ports.filter_map(|v| v.as_str().map(String::from)));
            }
        }
    }
    Chunk {
        zone: c.get_str("MeshChunk").unwrap_or_default().to_string(),
        layer: c.get_i32("Layer").unwrap_or_default(),
        visible_layers: c
            .get_array("VisibleLayers")
            .map(|a| a.filter_map(|v| v.as_i32()).collect())
            .unwrap_or_default(),
        conditions: conditions.len(),
        condition_ports,
        hide_in_first_person: c.get_bool("HideInFirstPerson").unwrap_or_default(),
    }
}

fn collect(dc: &Datacore) -> Vec<Row> {
    let db = dc.db();
    let mut rows = Vec::new();
    for record in db.all_records() {
        let full = record.name().unwrap_or_default();
        let Some(name) = full.strip_prefix("EntityClassDefinition.") else {
            continue;
        };
        let entity = record.as_instance();
        let Some(clothing) = component(db, &entity, "SCItemClothingParams") else {
            continue;
        };
        let item_type = component(db, &entity, "SAttachableComponentParams")
            .and_then(|a| a.get_instance("AttachDef"))
            .and_then(|d| d.get_str("Type").map(String::from))
            .unwrap_or_default();
        rows.push(Row {
            name: name.to_ascii_lowercase(),
            item_type,
            chunks: instances(db, &clothing, "Chunks")
                .iter()
                .map(|c| read_chunk(db, c))
                .collect(),
            hides: instances(db, &clothing, "HiddenParts")
                .iter()
                .filter_map(|h| h.get_str("PortName").map(String::from))
                .collect(),
            g_force: clothing
                .get_instance("Flight")
                .and_then(|f| f.get_f32("gForceResistance")),
        });
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    rows
}

fn census(rows: &[Row]) {
    let mut by_type: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();
    let mut zones: BTreeMap<&str, usize> = BTreeMap::new();
    let mut layers: BTreeMap<i32, usize> = BTreeMap::new();
    let mut hides: BTreeMap<&str, usize> = BTreeMap::new();
    let (mut mixed, mut visible, mut conditional, mut first_person, mut g_nonzero) =
        (0, 0, 0, 0, 0);
    let mut condition_ports: BTreeSet<&str> = BTreeSet::new();
    let mut g_sum = 0f64;
    for row in rows {
        let entry = by_type.entry(&row.item_type).or_default();
        entry.0 += 1;
        entry.1 += usize::from(!row.chunks.is_empty());
        entry.2 += usize::from(!row.hides.is_empty());
        let distinct: BTreeSet<i32> = row.chunks.iter().map(|c| c.layer).collect();
        mixed += usize::from(distinct.len() > 1);
        for chunk in &row.chunks {
            *zones.entry(&chunk.zone).or_default() += 1;
            *layers.entry(chunk.layer).or_default() += 1;
            visible += usize::from(!chunk.visible_layers.is_empty());
            conditional += usize::from(chunk.conditions > 0);
            first_person += usize::from(chunk.hide_in_first_person);
            condition_ports.extend(chunk.condition_ports.iter().map(String::as_str));
        }
        for port in &row.hides {
            *hides.entry(port).or_default() += 1;
        }
        if let Some(g) = row.g_force.filter(|g| *g != 0.0) {
            g_nonzero += 1;
            g_sum += f64::from(g);
        }
    }
    println!("{} entities with SCItemClothingParams", rows.len());
    println!("\ntype: entities / with chunks / with hidden parts");
    for (t, (n, c, h)) in &by_type {
        println!("  {t}: {n} / {c} / {h}");
    }
    println!("\nzones ({}): chunk rows", zones.len());
    for (z, n) in &zones {
        println!("  {z}: {n}");
    }
    println!("\nlayers: chunk rows");
    for (l, n) in &layers {
        println!("  {l}: {n}");
    }
    println!("\nhidden ports ({}): items", hides.len());
    for (p, n) in &hides {
        println!("  {p}: {n}");
    }
    println!("\nitems with chunks on more than one layer: {mixed}");
    println!("chunks with VisibleLayers: {visible}");
    println!("chunks with VisibilityConditions: {conditional}");
    println!("chunks hidden in first person: {first_person}");
    println!("condition ports: {condition_ports:?}");
    println!("items with non-zero gForceResistance: {g_nonzero}, sum {g_sum:.3}");
}

fn check_reference(rows: &[Row]) -> usize {
    let mut failures = 0;
    println!("\nreference (style-citizen.com):");
    for (name, zones, g_force) in REFERENCE {
        let Some(row) = rows.iter().find(|r| r.name == *name) else {
            println!("  {name}: no record");
            failures += 1;
            continue;
        };
        let ours: BTreeSet<&str> = row.chunks.iter().map(|c| c.zone.as_str()).collect();
        let theirs: BTreeSet<&str> = zones.iter().copied().collect();
        let g = row.g_force.unwrap_or_default();
        let ok = ours == theirs && (g - g_force).abs() < 1e-4;
        failures += usize::from(!ok);
        println!(
            "  {name}: {} zones, g {g} -> {}",
            ours.len(),
            if ok { "ok" } else { "MISMATCH" }
        );
    }
    failures
}

/// Zone names join mesh pieces through `zone_hash`: the site's table for nine
/// zones (its `hips_zone` entry disagrees with the client's function and is
/// left out), then every piece of the male body.
fn check_mesh_zones(
    assets: &AssetSource,
    rows: &[Row],
) -> Result<usize, Box<dyn std::error::Error>> {
    const PUBLISHED: &[(&str, u32)] = &[
        ("vneck_zone", 2_432_608_691),
        ("torso01_zone", 3_955_070_026),
        ("torso02_zone", 320_886_823),
        ("torso03_zone", 3_157_836_472),
        ("torso04_zone", 3_044_089_349),
        ("underwear_top_zone", 3_553_630_895),
        ("l_shoulder_zone", 1_353_240_489),
        ("r_shoulder_zone", 1_020_047_223),
        ("l_arm01_zone", 3_734_923_156),
    ];
    let mut failures = PUBLISHED
        .iter()
        .filter(|(name, hash)| sc_extract::zone_hash(name) != *hash)
        .count();
    println!();
    println!(
        "zone hash: {} of {} published values differ",
        failures,
        PUBLISHED.len()
    );

    let known: BTreeSet<u32> = rows
        .iter()
        .flat_map(|r| &r.chunks)
        .map(|c| sc_extract::zone_hash(&c.zone))
        .collect();
    let body = sc_extract::MeshGeometry::read_shape(
        assets,
        "objects/characters/human/male_v7/body/m_body.skin",
    )?;
    let zoned: Vec<u32> = body.submeshes.iter().filter_map(|s| s.zone).collect();
    let resolved = zoned.iter().filter(|z| known.contains(z)).count();
    println!(
        "male body: {resolved} of {} zoned pieces name a DCB zone",
        zoned.len()
    );
    failures += usize::from(zoned.is_empty() || resolved != zoned.len());
    Ok(failures)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dump = std::env::args().any(|a| a == "--dump");
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let dc = Datacore::parse(&assets, &data)?;
    let rows = collect(&dc);
    census(&rows);
    let failures = check_reference(&rows) + check_mesh_zones(&assets, &rows)?;
    if dump {
        println!("\nname|type|g|hides|zone:layer...");
        for row in &rows {
            let zones: Vec<String> = row
                .chunks
                .iter()
                .map(|c| format!("{}:{}", c.zone, c.layer))
                .collect();
            println!(
                "{}|{}|{}|{}|{}",
                row.name,
                row.item_type,
                row.g_force.unwrap_or_default(),
                row.hides.join(","),
                zones.join(" ")
            );
        }
    }
    if failures > 0 {
        return Err(format!("{failures} reference item(s) differ").into());
    }
    Ok(())
}
