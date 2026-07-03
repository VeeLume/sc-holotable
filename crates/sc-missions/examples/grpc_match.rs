//! Match captured gRPC contract_definition_id GUIDs against sc-missions, and
//! dump each contract's category + mission span + location-query scope — the
//! offline (DCB) view of the live mission data.
//!
//! ```bash
//! cargo run -p sc-missions --release --features full --example grpc_match
//! ```
use std::str::FromStr;

use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap};
use sc_missions::{MissionVar, Missions, RecordCollection};

const DEFIDS: &[&str] = &[
    "04e20bff-124c-46dc-b4ec-ea706b8d0313",
    "1cecb4cd-886c-4c84-a747-68ead8d2e8f9",
    "2338d0d1-b41c-4a92-8909-47892ab3e7a5",
    "39e7837b-c0ab-4c7f-859c-8964dc9c5158",
    "45e7b8b9-0f1e-42a8-ab67-4b5996a090ae",
    "4ca632b6-a50c-46e8-867f-2a46fc525ff3",
    "4cb22c75-5b48-494d-a44f-29c4fd6dc5b7",
    "68c03147-9e95-4a20-9e35-3a877ae0bc03",
    "698a2696-0c6b-4ab2-be85-370a7782d1fe",
    "84e9fc27-ba0e-4518-8c8d-d404333885dc",
    "8f29a2e2-4f34-4a9e-8335-e0b7e4d32f4c",
    "91aacb3d-2fac-4141-bca4-5b8048c50d10",
    "9bb5529b-54ed-4f46-a37d-634836687ac6",
    "bc4968b1-34b2-4a5b-90f5-b1ffe9aa16c9",
    "cbda5c17-9e2d-47ae-80b2-098693481820",
    "ef9d030a-4f13-49bb-818c-9dd2a7b1a347",
    "fd5615f8-a727-47e2-861e-679188653a33",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let locale: &LocaleMap = &asset_data.locale;
    let index = Missions::build(&datacore);
    eprintln!("missions: {}", index.contracts.len());

    let mut by_id = 0;
    let mut by_tmpl = 0;
    let mut miss = 0;
    for s in DEFIDS {
        let g = Guid::from_str(s).unwrap();
        // Resolve as Mission.id, else as template_id, else as generator_id.
        let m = index
            .get(&g)
            .or_else(|| index.contracts.iter().find(|m| m.template_id == Some(g)))
            .or_else(|| index.contracts.iter().find(|m| m.origin.generator_id == g));
        let Some(m) = m else {
            miss += 1;
            println!("\n{s}  <<NO MATCH in sc-missions>>");
            continue;
        };
        if m.id == g {
            by_id += 1;
        } else {
            by_tmpl += 1;
        }
        let how = if m.id == g {
            "id"
        } else if m.template_id == Some(g) {
            "template_id"
        } else {
            "generator_id"
        };
        let title = m.title(locale).unwrap_or("<none>");
        println!("\n{s}  [{how}]");
        println!("  debug_name: {}", m.debug_name);
        println!("  title:      {title}");
        println!("  handler:    {:?}", m.origin.kind);
        // mission span (localities -> region label + named locations)
        if m.mission_span.is_empty() {
            println!("  span:       <none>");
        } else {
            for sp in &m.mission_span {
                if let Some(lv) = index.localities.get(sp) {
                    let names: Vec<String> = lv
                        .locations
                        .iter()
                        .filter_map(|l| l.display_name(locale).map(|n| n.to_string()))
                        .take(12)
                        .collect();
                    println!(
                        "  span loc:   {} [{}]  -> {}",
                        lv.name,
                        lv.region_label(locale),
                        names.join(", ")
                    );
                }
            }
        }
        // location-type + choice variables (the runtime query scope / dataset slots)
        for (k, v) in &m.variables {
            match v {
                MissionVar::Location { systems, settings } => {
                    println!("  var Loc {k}: systems={systems:?} settings={settings:?}")
                }
                MissionVar::Choice(opts) => println!("  var Choice {k}: {} opts", opts.len()),
                MissionVar::Number(ns) => println!("  var Num {k}: {ns:?}"),
            }
        }
    }
    eprintln!(
        "\nmatched by id: {by_id}, by template/generator: {by_tmpl}, missed: {miss}  (of {})",
        DEFIDS.len()
    );
    Ok(())
}
