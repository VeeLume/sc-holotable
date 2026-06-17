//! Show the full structure of a crafting recipe — slot tree, materials, and
//! every gameplay-property modifier with its quality value-ranges — for a
//! couple of FPS weapons. Read-only.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example recipe_anatomy > target/recipe_anatomy.txt
//! ```

use sc_crafting::{
    Blueprints, Cost, CostContext, GameplayProperties, ModifierValue, Process, ValueRange,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, DataCoreDatabase, Datacore, LocaleMap};
use sc_items::Items;
use sc_resources::Resources;

const TARGETS: &[&str] = &["behr_sniper_ballistic_01", "klwe_pistol_energy_01"];
const Q: i32 = 750;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let resources = Resources::build(datacore.records());
    let gp = GameplayProperties::build(&datacore);
    let blueprints = Blueprints::build(&datacore, &items);
    let locale = &asset_data.locale;
    let db = datacore.db();

    for target in TARGETS {
        let Some(ent) = db
            .records_by_type("EntityClassDefinition")
            .find(|r| r.name().map(|n| n.ends_with(target)).unwrap_or(false))
        else {
            println!("!! {target} not found\n");
            continue;
        };
        let ent_guid = ent.id();
        let Some(bp) = blueprints.for_crafted_entity(ent_guid) else {
            println!("!! no blueprint for {target}\n");
            continue;
        };

        println!("══════════════════════════════════════════════════════════");
        println!("RECIPE  {}", bp.display_name(locale).unwrap_or(target));
        println!(
            "  crafted entity : {} ({ent_guid})",
            ent.name().unwrap_or("?")
        );
        println!("  blueprint GUID : {}", bp.blueprint_record_guid);
        println!("  category GUID  : {:?}", bp.category);
        match &bp.process {
            Process::Creation { entity_class } => {
                println!("  process        : Creation -> {entity_class:?}")
            }
            Process::Other { type_name, .. } => println!("  process        : Other({type_name})"),
        }
        println!("  tiers          : {}", bp.tiers.len());
        for (ti, tier) in bp.tiers.iter().enumerate() {
            let Some(recipe) = &tier.recipe else {
                println!("  tier[{ti}] (no recipe)");
                continue;
            };
            println!(
                "  tier[{ti}]: craft_time={:?}  results={}  is_shared={}  research={}",
                recipe.craft_time,
                recipe.results.len(),
                recipe.is_shared,
                tier.research.is_some(),
            );
            if let Some(costs) = &recipe.costs {
                println!("  COST TREE (mandatory):");
                if let Some(mc) = &costs.mandatory {
                    print_cost(mc, 2, &gp, &resources, &items, locale, db);
                }
                if !costs.optional.is_empty() {
                    println!("  optional costs: {}", costs.optional.len());
                }
            }
        }
        println!();
    }
    Ok(())
}

fn print_cost(
    cost: &Cost,
    indent: usize,
    gp: &GameplayProperties,
    resources: &Resources,
    items: &Items,
    locale: &LocaleMap,
    db: &DataCoreDatabase,
) {
    let pad = " ".repeat(indent);
    match cost {
        Cost::Select {
            name_info,
            count,
            options,
            context,
        } => {
            let slot = name_info
                .as_ref()
                .map(|n| {
                    locale
                        .resolve(&n.display_name)
                        .filter(|s| !s.is_empty() && !s.contains("PLACEHOLDER"))
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("<{}>", n.debug_name))
                })
                .unwrap_or_else(|| "(unnamed)".into());
            println!(
                "{pad}● Select  slot=\"{slot}\"  count={count}  options={}",
                options.len()
            );
            print_context(context, indent + 2, gp, locale, db);
            for o in options {
                print_cost(o, indent + 4, gp, resources, items, locale, db);
            }
        }
        Cost::Resource(rc) => {
            let name = rc
                .resource
                .and_then(|g| resources.get(&g))
                .and_then(|r| locale.resolve(&r.name_key))
                .unwrap_or("?");
            let qty = rc.quantity.as_ref().and_then(|q| q.to_scu());
            println!(
                "{pad}└─ Resource: {name:<16} min_quality={} qty_scu={:?}",
                rc.min_quality, qty
            );
            print_context(&rc.context, indent + 4, gp, locale, db);
        }
        Cost::Item(ic) => {
            let name = ic
                .entity_class
                .and_then(|g| items.name_key(&g))
                .and_then(|k| locale.resolve(k))
                .unwrap_or("?");
            println!(
                "{pad}└─ Item: {name:<16} qty={} min_quality={}",
                ic.quantity, ic.min_quality
            );
            print_context(&ic.context, indent + 4, gp, locale, db);
        }
        Cost::Other { type_name, .. } => println!("{pad}└─ Other({type_name})"),
    }
}

fn print_context(
    ctx: &[CostContext],
    indent: usize,
    gp: &GameplayProperties,
    locale: &LocaleMap,
    db: &DataCoreDatabase,
) {
    let pad = " ".repeat(indent);
    for c in ctx {
        match c {
            CostContext::GameplayPropertyModifiers(mods) => {
                for m in mods {
                    let (disp, rec_name, unit, transform) = match m.gameplay_property {
                        Some(g) => {
                            let p = gp.get(&g);
                            let disp = p
                                .and_then(|p| locale.resolve(&p.property_name_key))
                                .unwrap_or("?")
                                .to_string();
                            let rec = db
                                .record(&g)
                                .and_then(|r| r.name())
                                .unwrap_or("?")
                                .to_string();
                            let unit = p
                                .and_then(|p| locale.resolve(&p.unit_format_key))
                                .filter(|s| !s.is_empty())
                                .unwrap_or("(none)")
                                .to_string();
                            let tf = p
                                .map(|p| format!("{:?}", p.display_transformation))
                                .unwrap_or_else(|| "?".into());
                            (disp, rec, unit, tf)
                        }
                        None => ("?".into(), "?".into(), "?".into(), "?".into()),
                    };
                    println!("{pad}⊳ GPP \"{disp}\"  [{rec_name}]");
                    println!("{pad}    unit={unit}  transform={transform}");
                    for vr in &m.value_ranges {
                        match vr {
                            ValueRange::Linear {
                                start_quality,
                                end_quality,
                                modifier_at_start,
                                modifier_at_end,
                            } => {
                                let ev = m.evaluate(Q);
                                println!(
                                    "{pad}    Linear  Q{start_quality}-{end_quality}  ×{modifier_at_start}→{modifier_at_end}   eval@Q{Q}={}",
                                    fmt_mv(ev)
                                );
                            }
                            ValueRange::LinearIntegerAdditive {
                                start_quality,
                                end_quality,
                                additive_at_start,
                                additive_at_end,
                            } => {
                                let ev = m.evaluate(Q);
                                println!(
                                    "{pad}    LinearAdditive  Q{start_quality}-{end_quality}  +{additive_at_start}→+{additive_at_end}   eval@Q{Q}={}",
                                    fmt_mv(ev)
                                );
                            }
                            ValueRange::Other { type_name, .. } => {
                                println!("{pad}    Other({type_name})");
                            }
                        }
                    }
                }
            }
            CostContext::QuantityMultiplier(f) => println!("{pad}⊳ QuantityMultiplier({f})"),
            CostContext::ResultCompositionInclusion(ci) => {
                println!("{pad}⊳ ResultCompositionInclusion({ci:?})")
            }
            CostContext::Other { type_name, .. } => println!("{pad}⊳ Other({type_name})"),
        }
    }
}

fn fmt_mv(mv: Option<ModifierValue>) -> String {
    match mv {
        Some(ModifierValue::Multiplier(f)) => format!("×{f:.4}"),
        Some(ModifierValue::Additive(a)) => format!("+{a:.4}"),
        None => "?".into(),
    }
}
