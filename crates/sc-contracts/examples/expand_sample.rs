//! Sample run of [`sc_contracts::expand_all`].
//!
//! Prints summary counts + a handful of representative `ExpandedContract`
//! entries per handler kind. Used to spot-check the expansion pipeline
//! against SC 4.7 LIVE.
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example expand_sample
//! cargo run -p sc-contracts --release --example expand_sample -- --filter TarPits
//! ```
#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::time::Instant;

use sc_contracts::{
    expand_all, BlueprintPoolRegistry, ExpandedContract, HandlerKind, LocalityRegistry,
    LocationRegistry, RewardAmount, RewardCurrencyCatalog, ShipRegistry,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let filter: Option<String> = args
        .iter()
        .position(|a| a == "--filter")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_lowercase());
    let detail = args.iter().any(|a| a == "--detail");

    let install = sc_installs::discover_primary()?;
    println!(
        "Found {} v{} at {}",
        install.channel,
        install.short_version(),
        install.root.display()
    );
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let ships = ShipRegistry::build(&datacore);
    let blueprints = BlueprintPoolRegistry::build(&datacore, &asset_data.locale);
    let currency = RewardCurrencyCatalog::build(&datacore);
    let locations = LocationRegistry::build(&datacore, &asset_data.locale);
    let localities = LocalityRegistry::build(&datacore, &locations);
    println!(
        "ShipRegistry: {} entities. BlueprintPoolRegistry: {} pools. RewardCurrencyCatalog: {} currencies. LocationRegistry: {} StarMapObjects. LocalityRegistry: {} localities.",
        ships.len(),
        blueprints.len(),
        currency.len(),
        locations.len(),
        localities.len(),
    );

    let t0 = Instant::now();
    let expansions = expand_all(
        &datacore,
        &asset_data.locale,
        &ships,
        &blueprints,
        &currency,
        &localities,
    );
    println!(
        "expand_all: {} ExpandedContract(s) in {:.2}s\n",
        expansions.len(),
        t0.elapsed().as_secs_f64(),
    );

    print_summary(&expansions);

    // Spot-check rows.
    println!();
    if let Some(f) = filter {
        let matching: Vec<&ExpandedContract> = expansions
            .iter()
            .filter(|e| {
                e.debug_name.to_lowercase().contains(&f)
                    || e.handler_debug_name.to_lowercase().contains(&f)
                    || e
                        .title
                        .as_deref()
                        .map(|t| t.to_lowercase().contains(&f))
                        .unwrap_or(false)
            })
            .collect();
        println!(
            "─── Filter '{f}' matches {} of {} expansions ───",
            matching.len(),
            expansions.len()
        );
        for e in matching.iter().take(25) {
            print_expansion(e);
            if detail {
                print_detail(e);
                print_locality_detail(e, &datacore);
            }
        }
        if detail {
            print_delta(&matching);
        }
    } else {
        println!("─── Sample expansions (first from each handler kind) ───");
        for kind in [
            HandlerKind::Legacy,
            HandlerKind::Career,
            HandlerKind::List,
            HandlerKind::LinearSeries,
            HandlerKind::Tutorial,
        ] {
            for e in expansions.iter().filter(|e| e.handler_kind == kind).take(2) {
                print_expansion(e);
            }
        }

        // One blueprint-bearing row.
        println!();
        println!("─── Sample rows with blueprint rewards ───");
        for e in expansions.iter().filter(|e| e.blueprint_reward.is_some()).take(3) {
            print_expansion(e);
        }
    }

    Ok(())
}

fn print_summary(expansions: &[ExpandedContract]) {
    let mut by_kind: BTreeMap<HandlerKind, usize> = BTreeMap::new();
    for e in expansions {
        *by_kind.entry(e.handler_kind).or_default() += 1;
    }

    let with_title = expansions.iter().filter(|e| e.title.is_some()).count();
    let with_desc = expansions.iter().filter(|e| e.description.is_some()).count();
    let with_subst = expansions
        .iter()
        .filter(|e| e.has_runtime_substitution)
        .count();
    let shareable = expansions.iter().filter(|e| e.shareable).count();
    let once_only = expansions.iter().filter(|e| e.availability.once_only).count();
    let illegal = expansions.iter().filter(|e| e.illegal_flag).count();
    let with_bp = expansions.iter().filter(|e| e.blueprint_reward.is_some()).count();
    let uec_calc = expansions
        .iter()
        .filter(|e| matches!(e.reward_uec, RewardAmount::Calculated))
        .count();
    let uec_fixed = expansions
        .iter()
        .filter(|e| matches!(e.reward_uec, RewardAmount::Fixed(_)))
        .count();
    let with_scrip = expansions.iter().filter(|e| !e.reward_scrip.is_empty()).count();
    let with_rep = expansions.iter().filter(|e| !e.reward_rep.is_empty()).count();
    let with_items = expansions.iter().filter(|e| !e.reward_items.is_empty()).count();
    let with_prereq = expansions
        .iter()
        .filter(|e| !e.prerequisites.is_empty())
        .count();
    let with_cooldown = expansions
        .iter()
        .filter(|e| {
            e.availability.cooldowns.completion.is_some()
                || e.availability.cooldowns.abandon.is_some()
        })
        .count();

    println!("─── Summary ───────────────────────────────────────");
    println!("  Total expansions:           {}", expansions.len());
    println!("  With resolved title:        {with_title}");
    println!("  With resolved description:  {with_desc}");
    println!("  With runtime substitution:  {with_subst}");
    println!("  Shareable:                  {shareable}");
    println!("  Once-only:                  {once_only}");
    println!("  Illegal (bool param):       {illegal}");
    println!("  Reward UEC calculated:      {uec_calc}");
    println!("  Reward UEC fixed:           {uec_fixed}");
    println!("  With scrip reward:          {with_scrip}");
    println!("  With rep reward:            {with_rep}");
    println!("  With item reward:           {with_items}");
    println!("  With prerequisites:         {with_prereq}");
    println!("  With cooldown:              {with_cooldown}");
    println!("  With blueprint reward:      {with_bp}");
    println!();
    println!("  By handler kind:");
    for (kind, count) in &by_kind {
        println!("    {kind:?} — {count}");
    }
}

fn print_expansion(e: &ExpandedContract) {
    let origin = match e.origin {
        sc_contracts::ContractOrigin::Contract => "Contract",
        sc_contracts::ContractOrigin::ContractLegacy => "Legacy",
        sc_contracts::ContractOrigin::CareerContract => "Career",
        sc_contracts::ContractOrigin::SubContract { .. } => "SubContract",
    };
    let title = e.title.as_deref().unwrap_or("<none>");
    let subst = if e.has_runtime_substitution { " *" } else { "" };
    let mut flags = Vec::new();
    if e.shareable {
        flags.push("shareable");
    } else {
        flags.push("solo");
    }
    if e.availability.once_only {
        flags.push("once-only");
    }
    if e.availability.can_reaccept_after_failing {
        flags.push("retry-after-fail");
    }
    if e.availability.hide_in_mobi_glas {
        flags.push("hidden");
    }
    if e.illegal_flag {
        flags.push("illegal");
    }

    println!();
    println!(
        "  [{kind:?}/{origin}] {hd} / {cd}",
        kind = e.handler_kind,
        hd = e.handler_debug_name,
        cd = e.debug_name,
    );
    println!("      title:    {title}{subst}");
    if let Some(desc) = &e.description {
        let trimmed: String = desc.chars().take(120).collect();
        let tail = if desc.chars().count() > 120 { "…" } else { "" };
        println!("      desc:     {trimmed}{tail}");
    }
    println!("      flags:    {}", flags.join(", "));
    if !e.encounters.is_empty() {
        let wave_count: usize = e.encounters.iter().map(|g| g.waves.len()).sum();
        let slot_count: usize = e
            .encounters
            .iter()
            .flat_map(|g| g.waves.iter())
            .map(|w| w.slots.len())
            .sum();
        let sample_ships: Vec<String> = e
            .encounters
            .iter()
            .flat_map(|g| g.waves.iter())
            .flat_map(|w| w.slots.iter())
            .flat_map(|s| s.candidates.iter())
            .map(|c| c.display_name.clone())
            .filter(|n| !n.is_empty())
            .take(5)
            .collect();
        println!(
            "      enc:      {groups} group(s), {wave_count} wave(s), {slot_count} slot(s){ships}",
            groups = e.encounters.len(),
            ships = if sample_ships.is_empty() {
                String::new()
            } else {
                format!(" — sample: {}", sample_ships.join(", "))
            },
        );
    }
    if let Some(bp) = &e.blueprint_reward {
        let first_items: Vec<&str> = bp
            .items
            .iter()
            .filter_map(|i| {
                if i.display_name.is_empty() {
                    None
                } else {
                    Some(i.display_name.as_str())
                }
            })
            .take(4)
            .collect();
        println!(
            "      bp:       pool='{pool}' chance={c:.0}% items=[{items}{etc}]",
            pool = bp.pool_name,
            c = bp.chance * 100.0,
            items = first_items.join(", "),
            etc = if bp.items.len() > 4 {
                format!(", … +{} more", bp.items.len() - 4)
            } else {
                String::new()
            },
        );
    }

    // Rewards.
    let mut reward_bits = Vec::new();
    match e.reward_uec {
        sc_contracts::RewardAmount::None => {}
        sc_contracts::RewardAmount::Calculated => reward_bits.push("UEC: calculated".to_string()),
        sc_contracts::RewardAmount::Fixed(n) => reward_bits.push(format!("UEC: {n}")),
    }
    for s in &e.reward_scrip {
        let name = if s.display_name.is_empty() { &s.record_name } else { &s.display_name };
        reward_bits.push(format!("{} ×{}", name, s.amount));
    }
    for r in &e.reward_rep {
        let val = r
            .amount
            .map(|n| n.to_string())
            .unwrap_or_else(|| "calc".into());
        reward_bits.push(format!("rep {val}"));
    }
    if !e.reward_items.is_empty() {
        let names: Vec<String> = e
            .reward_items
            .iter()
            .map(|i| {
                let n = if i.display_name.is_empty() { "?" } else { i.display_name.as_str() };
                format!("{n}×{}", i.amount)
            })
            .take(3)
            .collect();
        let etc = if e.reward_items.len() > 3 {
            format!(", …+{}", e.reward_items.len() - 3)
        } else {
            String::new()
        };
        reward_bits.push(format!("items: [{}{}]", names.join(", "), etc));
    }
    if !e.reward_other.is_empty() {
        let kinds: Vec<String> = e
            .reward_other
            .iter()
            .map(|o| format!("{:?}", o).split('(').next().unwrap_or("").to_string())
            .collect();
        reward_bits.push(format!("other: [{}]", kinds.join(", ")));
    }
    if !reward_bits.is_empty() {
        println!("      rewards:  {}", reward_bits.join("; "));
    }

    // Cooldowns.
    let mut cd_bits = Vec::new();
    if let Some(c) = &e.availability.cooldowns.completion {
        cd_bits.push(format!("completion {:.0}s", c.mean_seconds));
    }
    if let Some(c) = &e.availability.cooldowns.abandon {
        cd_bits.push(format!("abandon {:.0}s", c.mean_seconds));
    }
    if let Some(c) = &e.availability.cooldowns.fail {
        cd_bits.push(format!("fail {:.0}s", c.mean_seconds));
    }
    if !cd_bits.is_empty() {
        println!("      cooldown: {}", cd_bits.join(", "));
    }

    // Prerequisites — count by kind.
    if !e.prerequisites.is_empty() {
        let mut histogram: BTreeMap<&str, usize> = BTreeMap::new();
        for p in &e.prerequisites {
            let k = match p {
                sc_contracts::PrereqView::Locality { .. } => "Locality",
                sc_contracts::PrereqView::Location { .. } => "Location",
                sc_contracts::PrereqView::LocationProperty { .. } => "LocationProperty",
                sc_contracts::PrereqView::CrimeStat { .. } => "CrimeStat",
                sc_contracts::PrereqView::Reputation { .. } => "Reputation",
                sc_contracts::PrereqView::CompletedContractTags { .. } => "CompletedContractTags",
                sc_contracts::PrereqView::Unknown { .. } => "Unknown",
            };
            *histogram.entry(k).or_default() += 1;
        }
        let summary: Vec<String> = histogram
            .iter()
            .map(|(k, v)| format!("{k}×{v}"))
            .collect();
        println!("      prereq:   {}", summary.join(", "));
    }
}

/// Dump every prereq as a structured one-liner so consumers can see which
/// tag GUIDs and which min/max rep standings drive apparent differences.
fn print_detail(e: &ExpandedContract) {
    println!("      id:       {}", e.id);
    if let sc_contracts::ContractOrigin::SubContract { parent } = e.origin {
        println!("      parent:   {parent}");
    }
    for (i, p) in e.prerequisites.iter().enumerate() {
        match p {
            sc_contracts::PrereqView::Locality { locality } => {
                println!(
                    "      pre#{i}:   Locality {{ locality: {} }}",
                    locality.map(|g| g.to_string()).unwrap_or("<none>".into())
                );
            }
            sc_contracts::PrereqView::Location { location } => {
                println!(
                    "      pre#{i}:   Location {{ location: {} }}",
                    location.map(|g| g.to_string()).unwrap_or("<none>".into())
                );
            }
            sc_contracts::PrereqView::LocationProperty { variable_name, extended_text_token, level } => {
                println!(
                    "      pre#{i}:   LocationProperty {{ var='{variable_name}', token='{extended_text_token}', level={level:?} }}"
                );
            }
            sc_contracts::PrereqView::CrimeStat { min, max, jurisdiction_override, include_when_sharing } => {
                println!(
                    "      pre#{i}:   CrimeStat {{ min={min}, max={max}, jurisdiction={}, share={include_when_sharing} }}",
                    jurisdiction_override.map(|g| g.to_string()).unwrap_or("<none>".into())
                );
            }
            sc_contracts::PrereqView::Reputation { faction, scope, exclude, min_standing, max_standing, include_when_sharing } => {
                println!(
                    "      pre#{i}:   Reputation {{ faction={}, scope={}, min_standing={}, max_standing={}, exclude={exclude}, share={include_when_sharing} }}",
                    faction.map(|g| g.to_string()).unwrap_or("<none>".into()),
                    scope.map(|g| g.to_string()).unwrap_or("<none>".into()),
                    min_standing.map(|g| g.to_string()).unwrap_or("<none>".into()),
                    max_standing.map(|g| g.to_string()).unwrap_or("<none>".into()),
                );
            }
            sc_contracts::PrereqView::CompletedContractTags { required_tags, required_count, excluded_tags, excluded_count, include_when_sharing } => {
                println!(
                    "      pre#{i}:   CompletedContractTags {{ req=[{}] count≥{required_count}, excl=[{}] count≥{excluded_count}, share={include_when_sharing} }}",
                    required_tags.iter().take(3).map(|g| g.to_string()).collect::<Vec<_>>().join(", "),
                    excluded_tags.iter().take(3).map(|g| g.to_string()).collect::<Vec<_>>().join(", "),
                );
            }
            sc_contracts::PrereqView::Unknown { struct_index, instance_index } => {
                println!("      pre#{i}:   Unknown {{ si={struct_index}, ii={instance_index} }}");
            }
        }
    }
}

/// Resolve every `Locality` / `Location` prereq GUID against the raw
/// DCB and print its record name + the locations it points at. Surfaces
/// the system / planet spread a contract covers — specifically useful
/// for Pyro / Nyx sub-contract variations where the "radius" is really
/// a set of offered pickup sites.
fn print_locality_detail(e: &ExpandedContract, datacore: &Datacore) {
    let db = datacore.db();
    for p in &e.prerequisites {
        match p {
            sc_contracts::PrereqView::Locality { locality: Some(g) } => {
                dump_locality(g, datacore);
            }
            sc_contracts::PrereqView::Location { location: Some(g) } => {
                dump_location(g, db, "      loca(p):");
            }
            _ => {}
        }
    }
}

fn dump_locality(
    guid: &Guid,
    datacore: &Datacore,
) {
    let db = datacore.db();
    let pools = &datacore.records().pools;
    let records = &datacore.records().records;

    let record_name = db
        .record(guid)
        .and_then(|r| r.name().map(|s| s.to_string()))
        .unwrap_or_default();
    let short = record_name
        .strip_prefix("MissionLocality.")
        .unwrap_or(&record_name);
    let short = strip_noise(short);

    // Look up the typed MissionLocality via the record index.
    let locality = records
        .multi_feature
        .mission_locality
        .get(guid)
        .and_then(|h| h.get(pools));

    if let Some(loc) = locality {
        let count = loc.available_locations.len();
        // Aggregate by system and body.
        let mut bodies: std::collections::BTreeMap<(&str, &str), usize> =
            std::collections::BTreeMap::new();
        for loc_guid in &loc.available_locations {
            let loc_name = db
                .record(loc_guid)
                .and_then(|r| r.name().map(|s| s.to_string()))
                .unwrap_or_default();
            let short_strip = loc_name
                .strip_prefix("StarMapObject.")
                .or_else(|| loc_name.strip_prefix("MissionLocationTemplate."))
                .unwrap_or(&loc_name)
                .to_string();
            *bodies.entry(classify_location(&short_strip)).or_default() += 1;
        }
        let body_summary: Vec<String> = bodies
            .iter()
            .map(|((sys, body), v)| {
                if body.is_empty() {
                    format!("{sys}×{v}")
                } else {
                    format!("{sys}/{body}×{v}")
                }
            })
            .collect();
        println!(
            "      locality: {short} ({count} loc{s}; {body_summary})",
            s = if count == 1 { "" } else { "s" },
            body_summary = body_summary.join(", "),
        );
        for (i, loc_guid) in loc.available_locations.iter().take(12).enumerate() {
            let loc_name = db
                .record(loc_guid)
                .and_then(|r| r.name().map(|s| s.to_string()))
                .unwrap_or_default();
            let short_strip = loc_name
                .strip_prefix("StarMapObject.")
                .or_else(|| loc_name.strip_prefix("MissionLocationTemplate."))
                .unwrap_or(&loc_name);
            let (sys, body) = classify_location(short_strip);
            let body_tag = if body.is_empty() { sys.to_string() } else { format!("{sys}/{body}") };
            println!("         {i:>2}. [{body_tag}] {}", strip_noise(short_strip));
        }
        if count > 12 {
            println!("         ... +{} more", count - 12);
        }
    } else {
        println!("      locality: {short} (unresolved)");
    }
}

fn dump_location(
    guid: &Guid,
    db: &sc_extract::svarog_datacore::DataCoreDatabase,
    prefix: &str,
) {
    let record_name = db
        .record(guid)
        .and_then(|r| r.name().map(|s| s.to_string()))
        .unwrap_or_default();
    let short = record_name
        .strip_prefix("MissionLocationTemplate.")
        .unwrap_or(&record_name);
    let system = classify_system(short);
    println!("{prefix} [{system}] {}", strip_noise(short));
}

/// Heuristic system classifier from `StarMapObject` / `MissionLocationTemplate`
/// record-name prefix. Maps numbered bodies to their canonical names:
/// Stanton1=Hurston, 2=Crusader, 3=ArcCorp, 4=microTech; Pyro1=Sirus,
/// 2=Monox, 3=Bloom, 4=Terminus, 5=Ignis, 6=Vuur. Returns `(system, body)`
/// where `body` may be empty for system-level references.
fn classify_system(name: &str) -> &'static str {
    let info = classify_location(name);
    info.0
}

fn classify_location(name: &str) -> (&'static str, &'static str) {
    let lower = name.to_lowercase();
    // Stanton numbered: Stanton1, Stanton2, … with suffixes (_L1..L5, a..z).
    if let Some(rest) = lower.strip_prefix("stanton") {
        if rest.starts_with('1') {
            return ("Stanton", "Hurston");
        }
        if rest.starts_with('2') {
            return ("Stanton", "Crusader");
        }
        if rest.starts_with('3') {
            return ("Stanton", "ArcCorp");
        }
        if rest.starts_with('4') {
            return ("Stanton", "microTech");
        }
        return ("Stanton", "");
    }
    if let Some(rest) = lower.strip_prefix("pyro") {
        let first = rest.chars().next();
        match first {
            Some('1') => return ("Pyro", "Sirus (Pyro I)"),
            Some('2') => return ("Pyro", "Monox (Pyro II)"),
            Some('3') => return ("Pyro", "Bloom (Pyro III)"),
            Some('4') => return ("Pyro", "Terminus (Pyro IV)"),
            Some('5') => return ("Pyro", "Ignis (Pyro V)"),
            Some('6') => return ("Pyro", "Vuur (Pyro VI)"),
            _ => return ("Pyro", ""),
        }
    }
    if lower.starts_with("nyx") || lower.contains("delamar") || lower.contains("levski") {
        return ("Nyx", "");
    }
    // Pyro infix markers: `_P1_`, `_P2_`, … appear in station / asteroid /
    // region-specific record names (`RR_P1_LEO`, `AsteroidCluster_Pyro_RegionD_*`,
    // `*_Pyro_RegionBandD_*`).
    if lower.contains("pyro") || lower.contains("_p1_") || lower.contains("_p2_")
        || lower.contains("_p3_") || lower.contains("_p4_") || lower.contains("_p5_")
        || lower.contains("_p6_") || lower.contains("regionb") || lower.contains("regiond")
    {
        // Try to narrow to a body.
        if lower.contains("_p1") || lower.contains("pyro1") { return ("Pyro", "Sirus (Pyro I)"); }
        if lower.contains("_p2") || lower.contains("pyro2") { return ("Pyro", "Monox (Pyro II)"); }
        if lower.contains("_p3") || lower.contains("pyro3") { return ("Pyro", "Bloom (Pyro III)"); }
        if lower.contains("_p4") || lower.contains("pyro4") { return ("Pyro", "Terminus (Pyro IV)"); }
        if lower.contains("_p5") || lower.contains("pyro5") { return ("Pyro", "Ignis (Pyro V)"); }
        if lower.contains("_p6") || lower.contains("pyro6") { return ("Pyro", "Vuur (Pyro VI)"); }
        return ("Pyro", "");
    }
    // Known Stanton body short-names.
    for (pat, label) in &[
        ("hur", "Hurston"),
        ("crus", "Crusader"),
        ("arc", "ArcCorp"),
        ("mic", "microTech"),
        ("lorville", "Lorville"),
        ("orison", "Orison"),
        ("newbab", "New Babbage"),
        ("area18", "Area18"),
    ] {
        if lower.starts_with(pat) {
            return ("Stanton", label);
        }
    }
    ("?", "")
}

fn strip_noise(s: &str) -> String {
    // Chop off common trailing noise (_PU, _01, numeric suffixes) for readability.
    s.trim_end_matches("_PU").to_string()
}

/// For a list of matching expansions, find every field that differs across
/// them. Prints a compact "what actually differs" report so the user can
/// see sub-contract distinctions that the summary collapses.
fn print_delta(matching: &[&ExpandedContract]) {
    if matching.len() < 2 {
        return;
    }
    println!();
    println!("─── Delta across {} matching expansions ───", matching.len());

    let first = matching[0];
    let all_ids: Vec<String> = matching.iter().map(|e| format!("{}", e.id)).collect();
    println!("  ids:              {}", all_ids.join("  "));
    println!(
        "  all ids unique?   {}",
        all_ids.iter().collect::<std::collections::HashSet<_>>().len() == matching.len()
    );

    let titles_same = matching.iter().all(|e| e.title == first.title);
    let descs_same = matching.iter().all(|e| e.description == first.description);
    let subst_same = matching.iter().all(|e| e.has_runtime_substitution == first.has_runtime_substitution);
    println!("  title identical?  {titles_same}");
    println!("  desc identical?   {descs_same}");
    println!("  subst flag same?  {subst_same}");

    println!("  origins:          {:?}", matching.iter().map(|e| e.origin).collect::<Vec<_>>());

    // Prereq-guid fingerprint per row.
    println!();
    println!("  Prereq fingerprints (row index → prereq guids):");
    for (i, e) in matching.iter().enumerate() {
        let mut guids: Vec<String> = e
            .prerequisites
            .iter()
            .flat_map(|p| match p {
                sc_contracts::PrereqView::CompletedContractTags { required_tags, .. } => {
                    required_tags.iter().map(|g| format!("tag:{}", &g.to_string()[..8])).collect::<Vec<_>>()
                }
                sc_contracts::PrereqView::Locality { locality } => {
                    locality.map(|g| vec![format!("loc:{}", &g.to_string()[..8])]).unwrap_or_default()
                }
                sc_contracts::PrereqView::Location { location } => {
                    location.map(|g| vec![format!("loca:{}", &g.to_string()[..8])]).unwrap_or_default()
                }
                sc_contracts::PrereqView::CrimeStat { min, max, .. } => {
                    vec![format!("cs:{min}-{max}")]
                }
                sc_contracts::PrereqView::Reputation { min_standing, max_standing, .. } => {
                    vec![format!("rep:{}-{}",
                        min_standing.map(|g| g.to_string()[..8].to_string()).unwrap_or_default(),
                        max_standing.map(|g| g.to_string()[..8].to_string()).unwrap_or_default())]
                }
                _ => vec![],
            })
            .collect();
        guids.sort();
        println!("    [{i}] {}", guids.join(", "));
    }

    // Reward fingerprints.
    println!();
    println!("  Reward fingerprints:");
    for (i, e) in matching.iter().enumerate() {
        let rep_fp: Vec<String> = e.reward_rep.iter().map(|r| {
            format!(
                "{}/{}:{}",
                r.faction.map(|g| g.to_string()[..8].to_string()).unwrap_or_default(),
                r.scope.map(|g| g.to_string()[..8].to_string()).unwrap_or_default(),
                r.amount.map(|n| n.to_string()).unwrap_or("calc".into()),
            )
        }).collect();
        let scrip_fp: Vec<String> = e.reward_scrip.iter().map(|s| {
            format!("{}×{}", s.display_name, s.amount)
        }).collect();
        let bp_fp = e.blueprint_reward.as_ref().map(|b| b.pool_name.as_str()).unwrap_or("");
        println!(
            "    [{i}] uec={:?} scrip=[{}] rep=[{}] bp='{}'",
            e.reward_uec,
            scrip_fp.join(", "),
            rep_fp.join(", "),
            bp_fp,
        );
    }
}
