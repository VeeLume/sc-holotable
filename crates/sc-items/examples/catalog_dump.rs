//! Diagnostic dump of the two-tier `ItemCatalog` (collections → models →
//! members) against the live DCB.
//!
//! Surfaces the grouping signals — entity stem, display name, typed item
//! type/sub-type, derived design, and (for context only) the raw ECD tag paths
//! the library no longer classifies by. Heuristic flags on models:
//! - **mixed-type**: members disagree on `item_type` (should be 0).
//! - **mixed-brand**: members' display names start with different words (0).
//! - **giant**: unusually large models.
//!
//! ```bash
//! cargo run -p sc-items --release --example catalog_dump                     # summary + flagged
//! cargo run -p sc-items --release --example catalog_dump -- --all            # every multi-member model
//! cargo run -p sc-items --release --example catalog_dump -- --collection Geist  # collections matching
//! ```

use std::collections::BTreeMap;

use sc_extract::generated::{EntityClassDefinition, RecordLookup};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap, RecordPaths, RecordStore};
use sc_items::{Item, ItemCatalog, Items};
use sc_tags::Tags;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = match args.first().map(String::as_str) {
        Some("--all") => Mode::All,
        Some("--collection") => Mode::Collection(args.get(1).cloned().unwrap_or_default()),
        _ => Mode::Summary,
    };

    let install = sc_discovery::discover_primary()?;
    println!("{} v{}\n", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;

    let store = datacore.records();
    let paths = RecordPaths::build(&datacore);
    let tags = Tags::build(store); // diagnostic context only
    let items = Items::build(store);
    let locale = &asset_data.locale;
    let catalog = ItemCatalog::build(&items, &paths, locale);

    // ── Collection-spotcheck mode ────────────────────────────────────────
    if let Mode::Collection(filter) = &mode {
        let mut cols: Vec<_> = catalog
            .collections()
            .filter(|c| c.name.to_lowercase().contains(&filter.to_lowercase()))
            .collect();
        cols.sort_by(|a, b| a.name.cmp(&b.name));
        println!("-- {} collection(s) matching {filter:?} --\n", cols.len());
        for col in cols {
            println!("██ {}  ({} models)  [{}]", col.name, col.model_count(), col.id);
            for m in catalog.models_in(col) {
                println!("  ▸ ({}) {}/{}", m.len(), m.item_type, m.item_sub_type);
                for (i, g) in m.members.iter().enumerate() {
                    let role = if i == 0 { "base" } else { "var " };
                    println!(
                        "      {role} {:<44} {}",
                        member_name(*g, &paths),
                        member_display(*g, &items, locale)
                    );
                }
            }
            println!();
        }
        return Ok(());
    }

    // ── Summary ──────────────────────────────────────────────────────────
    let mut size_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut multi = 0usize;
    for m in catalog.models() {
        *size_hist.entry(m.len()).or_default() += 1;
        if m.len() > 1 {
            multi += 1;
        }
    }
    println!("items            : {}", items.len());
    println!(
        "inventory items  : {}",
        items.iter().filter(|(_, it)| it.is_inventory_item()).count()
    );
    println!("models           : {}  ({multi} multi-member)", catalog.model_count());
    println!("collections      : {}", catalog.collection_count());
    println!("-- model size histogram (size: count) --");
    for (sz, n) in &size_hist {
        println!("  {sz:>4}: {n}");
    }

    let mut rows: Vec<ModelRow> = catalog
        .models()
        .filter(|m| m.len() > 1)
        .map(|m| ModelRow::new(m, &items, locale, store, &tags, &paths))
        .collect();

    // ── Aggregate contamination stats ────────────────────────────────────
    let mut mixed_type = 0usize;
    let mut any_noninv = 0usize;
    for m in catalog.models() {
        if m.len() < 2 {
            continue;
        }
        let types: std::collections::BTreeSet<_> = m
            .members
            .iter()
            .filter_map(|g| items.get(g).map(|i| format!("{:?}", i.item_type)))
            .collect();
        if types.len() > 1 {
            mixed_type += 1;
        }
        if m
            .members
            .iter()
            .any(|g| !items.get(g).map(|i| i.is_inventory_item()).unwrap_or(false))
        {
            any_noninv += 1;
        }
    }
    println!("\n-- contamination (multi-member models) --");
    println!("  mixed-type             : {mixed_type}   (target 0)");
    println!("  contains non-inventory : {any_noninv}   (target 0)");

    rows.sort_by(|a, b| {
        b.flags
            .len()
            .cmp(&a.flags.len())
            .then(b.size.cmp(&a.size))
            .then(a.id.cmp(&b.id))
    });
    let flagged = rows.iter().filter(|r| !r.flags.is_empty()).count();
    println!("\n-- {flagged} / {multi} multi-member models flagged --\n");

    let show: Vec<&ModelRow> = match mode {
        Mode::Summary => rows.iter().filter(|r| !r.flags.is_empty()).take(40).collect(),
        Mode::All => rows.iter().collect(),
        Mode::Collection(_) => unreachable!(),
    };
    for r in show {
        r.print();
    }

    Ok(())
}

enum Mode {
    Summary,
    All,
    Collection(String),
}

fn member_name(guid: Guid, paths: &RecordPaths) -> String {
    paths
        .get(&guid)
        .map(|rp| {
            rp.name
                .strip_prefix("EntityClassDefinition.")
                .unwrap_or(&rp.name)
                .to_string()
        })
        .unwrap_or_else(|| "<no-path>".into())
}

fn member_display(guid: Guid, items: &Items, locale: &LocaleMap) -> String {
    items
        .get(&guid)
        .and_then(|i| i.display_name(locale))
        .unwrap_or("<no-name>")
        .to_string()
}

struct Member {
    name: String,
    display: String,
    item_type: String,
    sub_type: String,
    tag_paths: Vec<String>,
}

struct ModelRow {
    id: String,
    collection: Option<String>,
    size: usize,
    members: Vec<Member>,
    flags: Vec<&'static str>,
}

impl ModelRow {
    fn new(
        m: &sc_items::Model,
        items: &Items,
        locale: &LocaleMap,
        store: &RecordStore,
        tags: &Tags,
        paths: &RecordPaths,
    ) -> Self {
        let members: Vec<Member> = m
            .members
            .iter()
            .map(|g| member(*g, items, locale, store, tags, paths))
            .collect();

        let mut flags = Vec::new();
        let distinct_types: std::collections::BTreeSet<&str> =
            members.iter().map(|m| m.item_type.as_str()).collect();
        if distinct_types.len() > 1 {
            flags.push("mixed-type");
        }
        if m.len() >= 20 {
            flags.push("giant");
        }

        ModelRow {
            id: m.id.clone(),
            collection: m.collection.clone(),
            size: m.len(),
            members,
            flags,
        }
    }

    fn print(&self) {
        let flag_str = if self.flags.is_empty() {
            String::new()
        } else {
            format!("  [{}]", self.flags.join(", "))
        };
        let col = self.collection.as_deref().unwrap_or("<no collection>");
        println!("━━ ({}) {}{}", self.size, self.id, flag_str);
        println!("     collection: {col}");
        for (i, m) in self.members.iter().enumerate() {
            let role = if i == 0 { "base" } else { "var " };
            println!(
                "   {role} {:<44} {:<28} {}/{}",
                m.name,
                truncate(&m.display, 28),
                m.item_type,
                m.sub_type
            );
            for tp in &m.tag_paths {
                println!("        tag: {tp}");
            }
        }
        println!();
    }
}

fn member(
    guid: Guid,
    items: &Items,
    locale: &LocaleMap,
    store: &RecordStore,
    tags: &Tags,
    paths: &RecordPaths,
) -> Member {
    let it: Option<&Item> = items.get(&guid);
    let name = member_name(guid, paths);
    let display = member_display(guid, items, locale);
    let item_type = it
        .map(|i| format!("{:?}", i.item_type))
        .unwrap_or_else(|| "?".into());
    let sub_type = it
        .map(|i| format!("{:?}", i.item_sub_type))
        .unwrap_or_else(|| "?".into());

    let mut tag_paths = Vec::new();
    if let Some(handle) = EntityClassDefinition::lookup(&store.records, &guid)
        && let Some(ecd) = handle.get(&store.pools)
    {
        for tg in &ecd.tags {
            let segs = tags.path(tg);
            if !segs.is_empty() {
                tag_paths.push(segs.join(" / "));
            }
        }
    }

    Member { name, display, item_type, sub_type, tag_paths }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let t: String = s.chars().take(n.saturating_sub(1)).collect();
        format!("{t}…")
    }
}
