//! Probe: is `BlueprintCategoryRecord`'s empty `pub struct {}` in
//! generated code a generator bug, or does the DCB schema genuinely
//! define it with zero attributes? Lists schema attributes (including
//! inherited) for the type and dumps a sample record's raw bytes.

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, svarog_datacore::Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let _ = asset_data;
    let datacore = Datacore::parse(&assets, &assets_to_asset_data(&assets)?)?;
    let db = datacore.db();

    for type_name in [
        "BlueprintCategoryRecord",
        "BlueprintCategoryDatabaseRecord",
        "CraftingGameplayPropertyDef",
    ] {
        println!("\n=== {type_name} ===");
        // Find the struct_index by scanning struct_definitions
        let mut idx = None;
        for (i, _def) in db.struct_definitions().iter().enumerate() {
            if db.struct_name(i) == Some(type_name) {
                idx = Some(i);
                break;
            }
        }
        let Some(idx) = idx else {
            println!("  (type not found in schema)");
            continue;
        };
        let def = &db.struct_definitions()[idx];
        let parent = { def.parent_type_index };
        let own_attrs = { def.attribute_count };
        let size = { def.struct_size };
        println!("  struct_index : {idx}");
        println!("  parent_type  : {parent} (-1 = no parent)");
        println!("  attribute_count (own) : {own_attrs}");
        println!("  struct_size           : {size}");

        let props = db.get_struct_properties(idx);
        println!("  full property list (own + inherited): {}", props.len());
        for p in &props {
            let name_offset = { p.name_offset };
            let data_type = { p.data_type };
            let name = db.get_string2(&name_offset);
            println!(
                "    {:<32} (type tag {:?})",
                name.unwrap_or("?"),
                data_type
            );
        }

        // sample one record of this type if any
        if let Some(rec) = db.records_by_type(type_name).next() {
            println!("  sample record: name={:?} guid={}", rec.name(), rec.id());
            let inst = rec.as_instance();
            let mut shown = 0;
            for prop in inst.properties() {
                let v = match &prop.value {
                    Value::String(s) | Value::Locale(s) | Value::Enum(s) => format!("{:?}", s),
                    Value::Int32(n) => format!("i32({n})"),
                    Value::Float(f) => format!("f32({f})"),
                    Value::Bool(b) => format!("Bool({b})"),
                    Value::Reference(Some(r)) => format!("Ref→{}", r.guid),
                    Value::Reference(None) => "Ref(none)".into(),
                    Value::Array(a) => format!("Array(len={}, elem={:?})", a.count, a.element_type),
                    other => format!("{:?}", std::mem::discriminant(other)),
                };
                println!("    field {:<32} = {}", prop.name, v);
                shown += 1;
            }
            if shown == 0 {
                println!("    (no instance-level fields)");
            }
        }
    }
    Ok(())
}

// boilerplate to thread AssetData through; tiny helper to mirror the pattern
fn assets_to_asset_data(assets: &AssetSource) -> Result<AssetData, Box<dyn std::error::Error>> {
    Ok(AssetData::extract(assets, &AssetConfig::standard())?)
}
