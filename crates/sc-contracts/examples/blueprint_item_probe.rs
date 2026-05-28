//! Probe for diagnosing blueprint-item display-name dropouts.
//!
//! Background: [`crate::BlueprintItem::display_name`] tries two
//! sources (the crafted entity's `SAttachableComponentParams →
//! AttachDef → Localization.Name`, then the `CraftingBlueprint.blueprintName`
//! fallback). Some items return `None` from both, so renderers drop
//! them. SCMDB shows these items with polished names, so it must read
//! a third source we don't.
//!
//! This probe dumps every BlueprintItem in a matching mission's pools
//! with full DCB metadata so we can see exactly which path fails and
//! what other fields are available on the crafted entity.
//!
//! Run:
//! ```bash
//! cargo run -p sc-contracts --release --example blueprint_item_probe -- --mission "Tactical Strike Group"
//! ```
#![allow(non_snake_case)]

use sc_contracts::{BlueprintPool, MissionIndex};
use sc_extract::svarog_datacore::{DataCoreDatabase, Instance, Value};
use sc_extract::{
    AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, LocaleMap, LocalizedItemCache,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mission_filter: String = args
        .iter()
        .position(|a| a == "--mission")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .ok_or("missing --mission <substring>")?
        .to_lowercase();

    let install = sc_installs::discover_primary()?;
    println!(
        "Found {} v{} at {}",
        install.channel,
        install.short_version(),
        install.root.display(),
    );

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let locale = &asset_data.locale;
    let cache = &datacore.snapshot().localized_items;
    let db = datacore.db();

    let index = MissionIndex::build(&datacore);
    println!("MissionIndex: {} missions\n", index.len());

    let matching: Vec<_> = index
        .iter()
        .filter(|m| {
            let title = m.title(locale).unwrap_or(&m.debug_name).to_lowercase();
            title.contains(&mission_filter) || m.debug_name.to_lowercase().contains(&mission_filter)
        })
        .collect();

    if matching.is_empty() {
        println!("(no missions matched '{mission_filter}')");
        return Ok(());
    }

    println!(
        "═══ {} mission(s) matching '{mission_filter}' ═══",
        matching.len(),
    );
    for mission in matching {
        probe_mission(mission, &index, db, cache, locale);
    }

    Ok(())
}

fn probe_mission(
    mission: &sc_contracts::Mission,
    index: &MissionIndex,
    db: &DataCoreDatabase,
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
) {
    let title = mission.title(locale).unwrap_or("<no title>");
    println!();
    println!("── Mission ──");
    println!("  title:        {title}");
    println!("  debug_name:   {}", mission.debug_name);
    println!("  id:           {}", mission.id);
    println!("  origin:       {:?}", mission.origin.kind);

    if mission.rewards.blueprints.is_empty() {
        println!("  (no blueprint rewards)");
        return;
    }

    for (pi, reward) in mission.rewards.blueprints.iter().enumerate() {
        println!();
        println!("  ─ Pool [{pi}] ─");
        println!("    pool_guid:    {}", reward.pool_guid);
        println!("    chance:       {:.0}%", reward.chance * 100.0);
        let Some(pool) = index.blueprints.get(&reward.pool_guid) else {
            println!("    !! pool not in BlueprintPoolRegistry");
            continue;
        };
        println!(
            "    pool_name:    {} ({} item(s))",
            pool.name,
            pool.items.len()
        );
        probe_pool(pool, db, cache, locale);
    }
}

fn probe_pool(
    pool: &BlueprintPool,
    db: &DataCoreDatabase,
    cache: &LocalizedItemCache,
    locale: &LocaleMap,
) {
    for (idx, item) in pool.items.iter().enumerate() {
        println!();
        println!("    Item [{idx}/{}]", pool.items.len() - 1);
        println!(
            "      blueprint_record_guid:  {}",
            item.blueprint_record_guid
        );
        let bp_record_name = db
            .record(&item.blueprint_record_guid)
            .and_then(|r| r.name())
            .unwrap_or("<not in db>");
        println!("        record name:          {bp_record_name}");

        match &item.crafted_entity_guid {
            Some(g) => {
                println!("      crafted_entity_guid:    {g}");
                let entity_record = db.record(g);
                let entity_name = entity_record
                    .as_ref()
                    .and_then(|r| r.name())
                    .unwrap_or("<not in db>");
                let entity_type = entity_record
                    .as_ref()
                    .and_then(|r| r.type_name())
                    .unwrap_or("<unknown>");
                println!("        record name:          {entity_name}   (type: {entity_type})");

                // What does the cache hold for this entity?
                match cache.get(g) {
                    Some(li) => {
                        println!("        cache entry:");
                        println!(
                            "          name_key:           {}",
                            li.name_key
                                .as_ref()
                                .map(|k| k.as_str().to_string())
                                .unwrap_or("<none>".into())
                        );
                        if let Some(k) = li.name_key.as_ref() {
                            println!(
                                "          name resolves to:   {:?}",
                                locale.resolve(k).unwrap_or("<not in locale>")
                            );
                        }
                        println!(
                            "          short_name_key:     {}",
                            li.short_name_key
                                .as_ref()
                                .map(|k| k.as_str().to_string())
                                .unwrap_or("<none>".into())
                        );
                        if let Some(k) = li.short_name_key.as_ref() {
                            println!(
                                "          short resolves to:  {:?}",
                                locale.resolve(k).unwrap_or("<not in locale>")
                            );
                        }
                        println!(
                            "          desc_key:           {}",
                            li.desc_key
                                .as_ref()
                                .map(|k| k.as_str().to_string())
                                .unwrap_or("<none>".into())
                        );
                    }
                    None => println!("        cache entry:           <none>"),
                }

                // Walk the entity's components for any extra fields we
                // could read (DisplayName, manufacturer + ticker, …).
                if let Some(rec) = entity_record {
                    dump_entity_extras(&rec.as_instance(), db, locale);
                }
            }
            None => println!("      crafted_entity_guid:    <none>"),
        }

        println!(
            "      blueprint_name_key:     {}",
            item.blueprint_name_key
                .as_ref()
                .map(|k| k.as_str().to_string())
                .unwrap_or("<none>".into())
        );
        if let Some(k) = &item.blueprint_name_key {
            println!(
                "        resolves to:          {:?}",
                locale.resolve(k).unwrap_or("<not in locale>")
            );
        }

        let resolved = item.display_name(cache, locale);
        println!("      display_name() →        {resolved:?}");
        if resolved.is_none() {
            println!("      !! DROPPED by renderer (display_name returned None)");
        }
    }
}

/// Walk the entity's `Components` array and surface anything that
/// might carry a player-facing name we're not currently reading:
/// every component's type_name, plus any `Localization` /
/// `DisplayName` / `Name` fields found inside.
fn dump_entity_extras(inst: &Instance<'_>, db: &DataCoreDatabase, locale: &LocaleMap) {
    let Some(components) = inst.get_array("Components") else {
        println!("        components:            <no Components array>");
        return;
    };
    let comps: Vec<Value<'_>> = components.collect();
    if comps.is_empty() {
        println!("        components:            <empty>");
        return;
    }
    println!("        components ({}):", comps.len());
    for (ci, value) in comps.iter().enumerate() {
        let Some(component) = value_to_instance(value, db) else {
            println!("          [{ci}] (not an instance)");
            continue;
        };
        let ty = component.type_name().unwrap_or("<unknown>");
        println!("          [{ci}] type: {ty}");

        // Direct DisplayName / Name / LocalizedName fields on the
        // component itself.
        for field in &["DisplayName", "Name", "LocalizedName", "ItemName"] {
            if let Some(s) = component.get_str(field) {
                if !s.is_empty() {
                    let resolved = locale.resolve(s).unwrap_or("<not in locale>");
                    println!("              {field} = {s:?}  →  {resolved:?}");
                }
            }
        }

        // Any nested `Localization` block (Name / ShortName / Description).
        if let Some(loc) = component.get_instance("Localization") {
            dump_localization(&loc, locale, "Localization");
        }
        // Some components nest it one level deeper.
        if let Some(attach) = component.get_instance("AttachDef")
            && let Some(loc) = attach.get_instance("Localization")
        {
            dump_localization(&loc, locale, "AttachDef.Localization");
        }
        if let Some(item_def) = component.get_instance("ItemDef")
            && let Some(loc) = item_def.get_instance("Localization")
        {
            dump_localization(&loc, locale, "ItemDef.Localization");
        }
    }
}

fn dump_localization(loc: &Instance<'_>, locale: &LocaleMap, label: &str) {
    for field in &["Name", "ShortName", "Description"] {
        if let Some(s) = loc.get_str(field) {
            if !s.is_empty() {
                let resolved = locale.resolve(s).unwrap_or("<not in locale>");
                println!("              {label}.{field} = {s:?}  →  {resolved:?}");
            }
        }
    }
}

fn value_to_instance<'a>(value: &Value<'a>, db: &'a DataCoreDatabase) -> Option<Instance<'a>> {
    match value {
        Value::Class { struct_index, data } => {
            Some(Instance::from_inline_data(db, *struct_index, data))
        }
        Value::ClassRef(r) | Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
            Some(db.instance(r.struct_index, r.instance_index))
        }
        _ => None,
    }
}
