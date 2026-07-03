//! Resolve the cargo manifest (commodity + SCU + box) per contract, keyed by the
//! gRPC contract_definition_id. Demonstrates clean per-mission cargo resolution
//! incl. MIXED contracts (multiple HaulingOrderContent_Resource entries).
//!
//! ```bash
//! cargo run -p sc-missions --release --example mission_cargo
//! ```
use std::collections::HashMap;
use std::str::FromStr;

use sc_extract::generated::{
    BaseMissionPropertyValuePtr, ContractParamOverrides, Handle, HaulingOrderContentBasePtr,
    MissionProperty, RecordLookup, ResourceType, SubContract,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid, LocaleMap, RecordStore};

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

#[derive(Clone)]
struct Leg {
    commodity: String,
    min_scu: f32,
    max_scu: f32,
    max_box: f32,
}

fn resource_name(store: &RecordStore, locale: &LocaleMap, g: &Guid) -> String {
    ResourceType::lookup(&store.records, g)
        .and_then(|h| h.get(&store.pools))
        .and_then(|rt| locale.resolve(&rt.display_name))
        .unwrap_or("<unresolved>")
        .to_string()
}

fn legs_from_props(
    store: &RecordStore,
    locale: &LocaleMap,
    props: &[Handle<MissionProperty>],
    out: &mut Vec<Leg>,
) {
    let pools = &store.pools;
    for ph in props {
        let Some(prop) = ph.get(pools) else { continue };
        let Some(BaseMissionPropertyValuePtr::MissionPropertyValue_HaulingOrders(h)) =
            prop.value.as_ref()
        else {
            continue;
        };
        let Some(ho) = h.get(pools) else { continue };
        for c in &ho.hauling_order_content {
            if let HaulingOrderContentBasePtr::HaulingOrderContent_Resource(rh) = c {
                if let Some(r) = rh.get(pools) {
                    let commodity = r
                        .resource
                        .map(|g| resource_name(store, locale, &g))
                        .unwrap_or("<none>".into());
                    out.push(Leg {
                        commodity,
                        min_scu: r.min_scu,
                        max_scu: r.max_scu,
                        max_box: r.max_container_size,
                    });
                }
            }
        }
    }
}

fn collect(
    store: &RecordStore,
    locale: &LocaleMap,
    po: Option<&Handle<ContractParamOverrides>>,
    subs: &[Handle<SubContract>],
    template: Option<Guid>,
) -> Vec<Leg> {
    let pools = &store.pools;
    let mut out = Vec::new();
    // Template layer (where procedural-haul manifests live).
    if let Some(t) = template {
        if let Some(tmpl) = store
            .records
            .multi_feature
            .contract_template
            .get(&t)
            .and_then(|h| h.get(pools))
        {
            legs_from_props(store, locale, &tmpl.contract_properties, &mut out);
        }
    }
    // Contract + sub override layers.
    if let Some(po) = po.and_then(|h| h.get(pools)) {
        legs_from_props(store, locale, &po.property_overrides, &mut out);
    }
    for s in subs {
        if let Some(s) = s.get(pools) {
            legs_from_props(store, locale, &s.property_overrides, &mut out);
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let locale: &LocaleMap = &asset_data.locale;

    // contract_id -> (debug_name, cargo legs) across all three contract subclasses.
    let mut map: HashMap<Guid, (String, Vec<Leg>)> = HashMap::new();
    for c in store.pools.multi_feature.contract.iter().flatten() {
        map.insert(
            c.id,
            (
                c.debug_name.clone(),
                collect(
                    store,
                    locale,
                    c.param_overrides.as_ref(),
                    &c.sub_contracts,
                    c.template,
                ),
            ),
        );
    }
    for c in store.pools.contracts.contract_legacy.iter().flatten() {
        map.insert(
            c.id,
            (
                c.debug_name.clone(),
                collect(
                    store,
                    locale,
                    c.param_overrides.as_ref(),
                    &c.sub_contracts,
                    c.template,
                ),
            ),
        );
    }
    for c in store.pools.contracts.career_contract.iter().flatten() {
        map.insert(
            c.id,
            (
                c.debug_name.clone(),
                collect(
                    store,
                    locale,
                    c.param_overrides.as_ref(),
                    &c.sub_contracts,
                    c.template,
                ),
            ),
        );
    }
    eprintln!("indexed {} contract records", map.len());

    let mut with_cargo = 0;
    for s in DEFIDS {
        let g = Guid::from_str(s).unwrap();
        match map.get(&g) {
            Some((debug, legs)) => {
                if !legs.is_empty() {
                    with_cargo += 1;
                }
                let kinds: std::collections::BTreeSet<&str> =
                    legs.iter().map(|l| l.commodity.as_str()).collect();
                let mixed = if kinds.len() > 1 { "  [MIXED]" } else { "" };
                println!("\n{s}{mixed}\n  {debug}");
                for l in legs {
                    println!(
                        "    {:<18} SCU {}–{}  box {}",
                        l.commodity, l.min_scu, l.max_scu, l.max_box
                    );
                }
                if legs.is_empty() {
                    println!("    (no HaulingOrders at contract/sub level)");
                }
            }
            None => println!("\n{s}  <<not in contract pools>>"),
        }
    }
    eprintln!(
        "\n{with_cargo}/{} contracts resolved cargo (contract+sub level)",
        DEFIDS.len()
    );
    Ok(())
}
