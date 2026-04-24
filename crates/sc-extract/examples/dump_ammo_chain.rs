//! Trace the full ammo resolution chain for every weapon.
//!
//! ```bash
//! cargo run -p sc-extract --release --features entities-scitem-weapons,ammoparams \
//!     --example dump_ammo_chain
//! ```

use std::collections::HashMap;

use sc_extract::generated::*;
use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let install = sc_installs::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;

    let snap = datacore.snapshot();
    let db = datacore.db();
    let pools = &snap.records.pools;

    let record_names: HashMap<Guid, &str> = db
        .records()
        .iter()
        .filter_map(|r| Some((r.id, db.record_name(r)?)))
        .collect();

    let ecd_map = &snap.records.records.multi_feature.entity_class_definition;
    let ammo_map = &snap.records.records.multi_feature.ammo_params;

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Stats
    let mut no_ref = 0u32;
    let mut direct_ammo = 0u32;
    let mut two_hop_ok = 0u32;
    let mut two_hop_no_container = 0u32;
    let mut two_hop_no_ammo_comp = 0u32;
    let mut two_hop_no_ammo_ref = 0u32;
    let mut two_hop_no_ammo_record = 0u32;
    let mut local_container_ok = 0u32;
    let mut local_container_fail = 0u32;
    let mut guid_not_found = 0u32;

    // Detailed output for interesting cases
    let mut failures: Vec<String> = Vec::new();

    for (&guid, &handle) in ecd_map {
        let Some(ecd) = handle.get(pools) else {
            continue;
        };
        let name = record_names
            .get(&guid)
            .copied()
            .unwrap_or("?")
            .replace("EntityClassDefinition.", "");

        let Some(wp) = ecd.components.iter().find_map(|c| match c {
            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(h) => h.get(pools),
            _ => None,
        }) else {
            continue;
        };

        // Dedup
        let base = dedup_name(&name);
        if seen.contains(&base) {
            continue;
        }
        seen.insert(base);

        // === Path 1: ammoContainerRecord on weapon component ===
        let container_ref = wp.ammo_container_record;

        if container_ref.is_none() {
            // No ammo ref on weapon component — check local SAmmoContainerComponentParams
            let local_ac = ecd.components.iter().find_map(|c| match c {
                DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
                _ => None,
            });

            if let Some(ac) = local_ac {
                if let Some(ammo_guid) = ac.ammo_params_record {
                    if ammo_map
                        .get(&ammo_guid)
                        .and_then(|h| h.get(pools))
                        .is_some()
                    {
                        local_container_ok += 1;
                    } else {
                        local_container_fail += 1;
                        failures.push(format!(
                            "LOCAL_NO_AMMO_RECORD {} ac.ammoParamsRecord={}",
                            name,
                            record_names.get(&ammo_guid).unwrap_or(&"?")
                        ));
                    }
                } else {
                    no_ref += 1;
                    failures.push(format!("LOCAL_NO_AMMO_REF {} (has SAmmoContainerComponentParams but ammoParamsRecord is None)", name));
                }
            } else {
                no_ref += 1;
                failures.push(format!(
                    "NO_REF {} (no ammoContainerRecord, no local SAmmoContainerComponentParams)",
                    name
                ));
            }
            continue;
        }

        let ref_guid = container_ref.unwrap();
        let ref_name = record_names.get(&ref_guid).copied().unwrap_or("?");

        // === Try direct AmmoParams ===
        if let Some(&ammo_h) = ammo_map.get(&ref_guid)
            && ammo_h.get(pools).is_some() {
                direct_ammo += 1;
                continue;
            }

        // === Try two-hop: ref → EntityClassDefinition → SAmmoContainerComponentParams → AmmoParams ===
        if let Some(&container_h) = ecd_map.get(&ref_guid) {
            if let Some(container_ecd) = container_h.get(pools) {
                let ammo_comp = container_ecd.components.iter().find_map(|c| match c {
                    DataForgeComponentParamsPtr::SAmmoContainerComponentParams(h) => h.get(pools),
                    _ => None,
                });

                if let Some(ac) = ammo_comp {
                    if let Some(ammo_guid) = ac.ammo_params_record {
                        if ammo_map
                            .get(&ammo_guid)
                            .and_then(|h| h.get(pools))
                            .is_some()
                        {
                            two_hop_ok += 1;
                        } else {
                            two_hop_no_ammo_record += 1;
                            failures.push(format!("TWO_HOP_NO_AMMO_RECORD {} -> {} -> ac.ammoParamsRecord={} (not in ammo pool)",
                                name, ref_name, record_names.get(&ammo_guid).unwrap_or(&"?")));
                        }
                    } else {
                        two_hop_no_ammo_ref += 1;
                        failures.push(format!("TWO_HOP_NO_AMMO_REF {} -> {} (SAmmoContainerComponentParams.ammoParamsRecord is None)",
                            name, ref_name));
                    }
                } else {
                    // Container entity has no SAmmoContainerComponentParams — check what it does have
                    let comp_types: Vec<&str> = container_ecd
                        .components
                        .iter()
                        .map(|c| match c {
                            DataForgeComponentParamsPtr::SCItemWeaponComponentParams(_) => {
                                "WeaponComp"
                            }
                            DataForgeComponentParamsPtr::SAttachableComponentParams(_) => {
                                "Attachable"
                            }
                            DataForgeComponentParamsPtr::SAmmoContainerComponentParams(_) => {
                                "AmmoContainer"
                            }
                            DataForgeComponentParamsPtr::SHealthComponentParams(_) => "Health",
                            _ => "other",
                        })
                        .filter(|t| *t != "other")
                        .collect();

                    two_hop_no_ammo_comp += 1;
                    failures.push(format!(
                        "TWO_HOP_NO_AMMO_COMP {} -> {} (comps: {}, has: [{}])",
                        name,
                        ref_name,
                        container_ecd.components.len(),
                        comp_types.join(", ")
                    ));
                }
            } else {
                two_hop_no_container += 1;
                failures.push(format!(
                    "TWO_HOP_POOL_FAIL {} -> {} (entity in map but not in pool)",
                    name, ref_name
                ));
            }
        } else {
            // GUID not in ecd_map either — could be a different record type
            guid_not_found += 1;
            failures.push(format!(
                "GUID_NOT_FOUND {} -> {} (not in entity_class_definition or ammo_params maps)",
                name, ref_name
            ));
        }
    }

    println!("=== AMMO CHAIN RESOLUTION SUMMARY (unique base weapons) ===\n");
    println!("Direct AmmoParams hit:          {direct_ammo}");
    println!("Two-hop resolved:               {two_hop_ok}");
    println!("Local container resolved:        {local_container_ok}");
    println!("---");
    println!("No ammo reference at all:        {no_ref}");
    println!("Two-hop: no ammo component:      {two_hop_no_ammo_comp}");
    println!("Two-hop: no ammo ref on comp:    {two_hop_no_ammo_ref}");
    println!("Two-hop: ammo not in pool:       {two_hop_no_ammo_record}");
    println!("Two-hop: container pool fail:     {two_hop_no_container}");
    println!("GUID not found anywhere:         {guid_not_found}");
    println!("Local container fail:            {local_container_fail}");

    let total_ok = direct_ammo + two_hop_ok + local_container_ok;
    let total_fail = no_ref
        + two_hop_no_ammo_comp
        + two_hop_no_ammo_ref
        + two_hop_no_ammo_record
        + two_hop_no_container
        + guid_not_found
        + local_container_fail;
    println!("\nTotal resolved: {total_ok}");
    println!("Total unresolved: {total_fail}");

    if !failures.is_empty() {
        println!("\n=== FAILURE DETAILS ===\n");
        for f in &failures {
            println!("{f}");
        }
    }

    Ok(())
}

fn dedup_name(name: &str) -> String {
    if name.starts_with(|c: char| c.is_uppercase()) {
        return name.to_string();
    }
    if let Some(pos) = name.find("_01_") {
        return name[..pos + 3].to_string();
    }
    if let Some(pos) = name.find("_02_") {
        return name[..pos + 3].to_string();
    }
    name.to_string()
}
