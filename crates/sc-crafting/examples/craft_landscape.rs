//! Scope the craftable landscape: every blueprint grouped by crafted-item
//! type, with the gameplay properties its recipes touch and the component
//! types a sample entity carries. Drives which per-itemtype T1 stat crates to
//! build next (armor, components, …).
//!
//! ```bash
//! cargo run -p sc-crafting --release --example craft_landscape > target/craft_landscape.txt
//! ```

use std::collections::{BTreeMap, BTreeSet};

use sc_crafting::{Blueprints, GameplayProperties};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, Value};
use sc_items::{Items, RecordCollection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let gp = GameplayProperties::build(&datacore);
    let blueprints = Blueprints::build(&datacore, &items);
    let db = datacore.db();

    // item_type -> (count, GPP suffixes touched, sub_types, sample entity guid)
    struct Bucket {
        count: usize,
        gpps: BTreeSet<String>,
        sub_types: BTreeSet<String>,
        sample: Option<Guid>,
    }
    let mut by_type: BTreeMap<String, Bucket> = BTreeMap::new();

    for bp in blueprints.values() {
        let Some(ent) = bp.crafted_entity_guid() else {
            continue;
        };
        let ty = items
            .item_type(&ent)
            .map(|t| t.as_dcb_str().to_string())
            .unwrap_or_else(|| "<unknown>".into());
        let sub = items
            .item_sub_type(&ent)
            .map(|t| t.as_dcb_str().to_string())
            .unwrap_or_default();
        let b = by_type.entry(ty).or_insert_with(|| Bucket {
            count: 0,
            gpps: BTreeSet::new(),
            sub_types: BTreeSet::new(),
            sample: None,
        });
        b.count += 1;
        if !sub.is_empty() {
            b.sub_types.insert(sub);
        }
        if b.sample.is_none() {
            b.sample = Some(ent);
        }
        for tier in &bp.tiers {
            if let Some(recipe) = &tier.recipe
                && let Some(costs) = &recipe.costs
                && let Some(mc) = &costs.mandatory
            {
                for m in mc.gameplay_property_modifiers() {
                    if let Some(g) = m.gameplay_property
                        && let Some(p) = gp.get(&g)
                    {
                        let suffix = p.record_name.rsplit('.').next().unwrap_or(&p.record_name);
                        b.gpps.insert(suffix.to_string());
                    }
                }
            }
        }
    }

    println!("=== craftable item types ({} total) ===", by_type.len());
    for (ty, b) in &by_type {
        println!("\n▼ {ty}  ({} blueprints)", b.count);
        println!("  sub_types: {:?}", b.sub_types);
        println!("  GPPs touched ({}):", b.gpps.len());
        for g in &b.gpps {
            println!("    - {g}");
        }
        // Sample entity's component types (where base stats would live).
        if let Some(ent) = b.sample
            && let Some(rec) = db.record(&ent)
        {
            let mut comps: BTreeSet<String> = BTreeSet::new();
            for p in rec.as_instance().properties() {
                if let Value::Array(_) = p.value
                    && let Some(arr) = rec.get_array(p.name)
                {
                    for elem in arr {
                        if let Some(si) = elem.struct_index()
                            && let Some(tn) = db.struct_name(si as usize)
                            && tn.contains("Component")
                        {
                            comps.insert(tn.to_string());
                        }
                    }
                }
            }
            println!(
                "  sample {} components: {:?}",
                rec.name().unwrap_or("?"),
                comps
            );
        }
    }

    // Cross-check: which GPPs are NOT used by any recipe (the unused 9).
    let used: BTreeSet<&String> = by_type.values().flat_map(|b| b.gpps.iter()).collect();
    println!("\n=== GPPs defined but UNUSED by any recipe ===");
    for p in gp.iter() {
        let suffix = p.record_name.rsplit('.').next().unwrap_or(&p.record_name);
        if !used.contains(&suffix.to_string()) {
            println!("  - {suffix}");
        }
    }
    Ok(())
}
