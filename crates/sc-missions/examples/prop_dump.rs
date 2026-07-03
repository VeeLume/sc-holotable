//! Dump the full MissionProperty tree of a contract (template + param-override +
//! sub layers), recursing CombinedDataSetEntries, printing each property's value
//! variant — to locate where a split-delivery contract's cargo manifest lives.
//!
//! ```bash
//! cargo run -p sc-missions --release --example prop_dump -- <contract_definition_id>
//! ```
use sc_extract::generated::{
    BaseMissionPropertyValuePtr as V, DataPools, Handle, HaulingOrderContentBasePtr,
    MissionProperty,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Guid};

fn vname(v: &V) -> &'static str {
    match v {
        V::MissionPropertyValue_HaulingOrders(_) => "HaulingOrders",
        V::MissionPropertyValue_CombinedDataSetEntries(_) => "CombinedDataSetEntries",
        V::MissionPropertyValue_DeliveryOrder(_) => "DeliveryOrder",
        V::MissionPropertyValue_MissionItem(_) => "MissionItem",
        V::MissionPropertyValue_Location(_) => "Location",
        V::MissionPropertyValue_Locations(_) => "Locations",
        V::MissionPropertyValue_Integer(_) => "Integer",
        V::MissionPropertyValue_StringHash(_) => "StringHash",
        V::MissionPropertyValue_Organization(_) => "Organization",
        V::MissionPropertyValue_Object(_) => "Object",
        V::MissionPropertyValue_Tags(_) => "Tags",
        V::MissionPropertyValue_Reward(_) => "Reward",
        V::MissionPropertyValue_Float(_) => "Float",
        V::MissionPropertyValue_Boolean(_) => "Boolean",
        V::MissionPropertyValue_AIName(_) => "AIName",
        V::MissionPropertyValue_ShipSpawnDescriptions(_) => "ShipSpawn",
        V::MissionPropertyValue_NPCSpawnDescriptions(_) => "NPCSpawn",
        V::MissionPropertyValue_EntitySpawnDescriptions(_) => "EntitySpawn",
        _ => "?other",
    }
}

fn dump(pools: &DataPools, props: &[Handle<MissionProperty>], d: usize) {
    let pad = "  ".repeat(d);
    for ph in props {
        let Some(prop) = ph.get(pools) else { continue };
        let v = prop.value.as_ref();
        println!(
            "{pad}token='{}'  value={}",
            prop.extended_text_token,
            v.map(vname).unwrap_or("<none>")
        );
        match v {
            Some(V::MissionPropertyValue_CombinedDataSetEntries(h)) => {
                if let Some(c) = h.get(pools) {
                    dump(pools, &c.data_set_entry_properties, d + 1);
                }
            }
            Some(V::MissionPropertyValue_HaulingOrders(h)) => {
                if let Some(ho) = h.get(pools) {
                    for c in &ho.hauling_order_content {
                        if let HaulingOrderContentBasePtr::HaulingOrderContent_Resource(rh) = c
                            && let Some(r) = rh.get(pools)
                        {
                            println!(
                                "{pad}    leg resource={:?} scu {}-{} box {}",
                                r.resource, r.min_scu, r.max_scu, r.max_container_size
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let pools = &store.pools;

    let g: Guid = std::env::args()
        .nth(1)
        .expect("contract_def guid")
        .parse()?;

    // find the contract across the three contract pools; dump template + overrides + subs
    macro_rules! try_pool {
        ($pool:expr, $kind:literal) => {
            for c in $pool.iter().flatten() {
                if c.id != g {
                    continue;
                }
                println!(
                    "== {} {} (template={:?}) ==",
                    $kind,
                    c.debug_name,
                    c.template.map(|t| t.to_string())
                );
                if let Some(t) = c.template {
                    if let Some(tmpl) = store
                        .records
                        .multi_feature
                        .contract_template
                        .get(&t)
                        .and_then(|h| h.get(pools))
                    {
                        println!(
                            "-- template.contract_properties ({}):",
                            tmpl.contract_properties.len()
                        );
                        dump(pools, &tmpl.contract_properties, 1);
                    }
                }
                if let Some(po) = c.param_overrides.as_ref().and_then(|h| h.get(pools)) {
                    println!(
                        "-- contract.param_overrides ({}):",
                        po.property_overrides.len()
                    );
                    dump(pools, &po.property_overrides, 1);
                }
                for (i, s) in c.sub_contracts.iter().enumerate() {
                    if let Some(s) = s.get(pools) {
                        println!(
                            "-- sub_contract[{i}].property_overrides ({}):",
                            s.property_overrides.len()
                        );
                        dump(pools, &s.property_overrides, 1);
                    }
                }
                return Ok(());
            }
        };
    }
    try_pool!(pools.multi_feature.contract, "Contract");
    try_pool!(pools.contracts.contract_legacy, "ContractLegacy");
    try_pool!(pools.contracts.career_contract, "CareerContract");
    println!("contract {g} not found in any pool");
    Ok(())
}
