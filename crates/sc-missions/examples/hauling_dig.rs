//! Navigate a (procedural) contract's objective tree → `ObjectiveHandler_Hauling`
//! → `HaulingOrder*` legs, dumping each leg's commodity/SCU/box + pickup/dropoff
//! SLOT (the `mission_variable_name`, joinable to the gRPC DataSet). Proves the
//! path before productionizing into `Mission.cargo`.
//!
//! ```bash
//! cargo run -p sc-missions --release --example hauling_dig -- <contract_definition_id>
//! ```
use std::collections::HashSet;

use sc_extract::generated::{
    BaseMissionPropertyValuePtr as PV, ChildMissionPhase, DataPools, Handle, HaulingOrderBasePtr,
    HaulingOrderContentBasePtr, MissionProperty, ObjectiveHandlerBasePtr, ObjectivePropertyBasePtr,
    ObjectiveToken,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid};

/// An ObjectiveProperty → (slot name, the backing MissionProperty handle).
fn op_resolve(
    pools: &DataPools,
    op: &ObjectivePropertyBasePtr,
) -> (String, Option<Handle<MissionProperty>>) {
    use ObjectivePropertyBasePtr::*;
    match op {
        ObjectiveProperty_Referenced(h) => h
            .get(pools)
            .map(|r| (r.mission_variable_name.clone(), r.property))
            .unwrap_or_default(),
        ObjectiveProperty_Embedded(h) => h
            .get(pools)
            .map(|r| (String::new(), r.property))
            .unwrap_or_default(),
        ObjectiveProperty_Output(h) => h
            .get(pools)
            .map(|r| (String::new(), r.property))
            .unwrap_or_default(),
        ObjectiveProperty_Input(h) => h
            .get(pools)
            .map(|r| (String::new(), r.property))
            .unwrap_or_default(),
        _ => (String::new(), None),
    }
}
fn slot_of(pools: &DataPools, op: &Option<ObjectivePropertyBasePtr>) -> String {
    op.as_ref()
        .map(|o| op_resolve(pools, o).0)
        .unwrap_or_default()
}

fn dump_ho(pools: &DataPools, ho: &HaulingOrderBasePtr) {
    use HaulingOrderBasePtr::*;
    match ho {
        HaulingOrder_Resource(h) => {
            if let Some(r) = h.get(pools) {
                println!(
                    "    Resource  res={:?} scu {}-{} box {}  pickup='{}' dropoff='{}'",
                    r.resource,
                    r.min_scu,
                    r.max_scu,
                    r.max_container_size,
                    slot_of(pools, &r.pick_up_location),
                    slot_of(pools, &r.drop_off_location)
                );
            }
        }
        HaulingOrder_Property(h) => {
            if let Some(p) = h.get(pools) {
                let (pu, dp) = (
                    slot_of(pools, &p.pick_up_location),
                    slot_of(pools, &p.drop_off_location),
                );
                let mp = p
                    .hauling_orders_property
                    .as_ref()
                    .and_then(|op| op_resolve(pools, op).1);
                let val = mp
                    .and_then(|h| h.get(pools))
                    .and_then(|prop| prop.value.as_ref());
                if let Some(PV::MissionPropertyValue_HaulingOrders(hh)) = val {
                    if let Some(hov) = hh.get(pools) {
                        for c in &hov.hauling_order_content {
                            if let HaulingOrderContentBasePtr::HaulingOrderContent_Resource(rh) = c
                                && let Some(rr) = rh.get(pools)
                            {
                                println!(
                                    "    Property  res={:?} scu {}-{} box {}  pickup='{pu}' dropoff='{dp}'",
                                    rr.resource, rr.min_scu, rr.max_scu, rr.max_container_size
                                );
                            }
                        }
                    }
                } else {
                    println!(
                        "    Property  (no HaulingOrders behind hauling_orders_property)  pickup='{pu}' dropoff='{dp}'"
                    );
                }
            }
        }
        _ => println!("    (other HaulingOrder variant)"),
    }
}

fn dump_handler(pools: &DataPools, oh: &Option<ObjectiveHandlerBasePtr>) {
    if let Some(ObjectiveHandlerBasePtr::ObjectiveHandler_Hauling(h)) = oh
        && let Some(handler) = h.get(pools)
    {
        println!(
            "  ObjectiveHandler_Hauling: {} order(s)",
            handler.hauling_orders.len()
        );
        for ho in &handler.hauling_orders {
            dump_ho(pools, ho);
        }
    }
}
fn walk_token(pools: &DataPools, t: &ObjectiveToken, seen: &mut HashSet<Guid>) {
    if !seen.insert(t.id) {
        return;
    }
    dump_handler(pools, &t.objective_handler);
    for cp in &t.child_mission_phases {
        if let Some(c) = cp.get(pools) {
            walk_phase(pools, c, seen);
        }
    }
}
fn walk_phase(pools: &DataPools, p: &ChildMissionPhase, seen: &mut HashSet<Guid>) {
    if !seen.insert(p.id) {
        return;
    }
    dump_handler(pools, &p.objective_handler);
    for cp in &p.child_mission_phases {
        if let Some(c) = cp.get(pools) {
            walk_phase(pools, c, seen);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let store = datacore.records();
    let pools = &store.pools;
    let g: Guid = std::env::args()
        .nth(1)
        .expect("contract_def guid")
        .parse()?;

    for c in pools.contracts.contract_legacy.iter().flatten() {
        if c.id != g {
            continue;
        }
        println!(
            "ContractLegacy {}  template={:?} broker={:?}",
            c.debug_name,
            c.template.is_some(),
            c.mission_broker_entry.is_some()
        );
        let mut tokens: Vec<Handle<ObjectiveToken>> = Vec::new();
        if let Some(t) = c.template
            && let Some(tmpl) = store
                .records
                .multi_feature
                .contract_template
                .get(&t)
                .and_then(|h| h.get(pools))
        {
            tokens.extend(tmpl.objective_tokens.iter().copied());
        }
        if let Some(b) = c.mission_broker_entry
            && let Some(be) = store
                .records
                .multi_feature
                .mission_broker_entry
                .get(&b)
                .and_then(|h| h.get(pools))
        {
            tokens.extend(be.objective_tokens.iter().copied());
        }
        println!("objective_tokens: {}", tokens.len());
        let mut seen = HashSet::new();
        for th in &tokens {
            if let Some(t) = th.get(pools) {
                walk_token(pools, t, &mut seen);
            }
        }
        return Ok(());
    }
    println!("{g} not found as a ContractLegacy");
    Ok(())
}
