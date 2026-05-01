//! Targeted investigation for the Foxwell Enforcement Ship Ambush
//! contracts.  Three sections:
//!
//! 1. **`PU_Human_PrivateSecurity_Light_wTurret` census** — every slot
//!    across the index whose *positive* tag bag contains this crew-
//!    manifest archetype. Shows the contracts that use it and the
//!    encounter variable_names that host it.
//! 2. **`NotWantedDuringShipCombat` census** — every slot whose
//!    *negative* tag bag carries this filter. Cross-correlated with
//!    §1 to confirm the "civilian-pilot + private-security gunner +
//!    must not be a combat-suitable ship" pattern.
//! 3. **Foxwell ambush deep dive** — the Stanton + Pyro variants of
//!    `FoxwellEnforcement_ShipAmbush_*_VeryEasy`:
//!      - Resolve every Locality prereq's `available_locations` to a
//!        list of LocationRefs (planet / station / asteroid base).
//!      - Walk the raw `SpawnDescription_Ship.name_property` chain so
//!        we can see what trigger / dispatch property each slot is
//!        wired to (the `nameProperty` field is dropped at extraction
//!        in v2 phase 6 — investigated raw here).
//!
//! Run:
//! ```
//! cargo run -p sc-contracts --release --example ambush_dig
//! ```

use sc_contracts::{Encounter, MissionIndex};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore, &asset_data.locale);

    section_1_security_turret(&index);
    section_2_not_wanted_during_combat(&index);
    section_3_foxwell_dig(&index, &datacore);

    Ok(())
}

// ── §1 PU_Human_PrivateSecurity_Light_wTurret census ─────────────────────────

fn section_1_security_turret(index: &MissionIndex) {
    println!("=== §1 PU_Human_PrivateSecurity_Light_wTurret usage ===");
    let trait_name = "PU_Human_PrivateSecurity_Light_wTurret";

    // (handler_kind, contract_debug_name, variable_name, phase_name, slot_idx)
    let mut hits: Vec<(String, String, String, String, usize)> = Vec::new();
    for c in &index.contracts {
        for enc in &c.encounters {
            if let Encounter::Ships(s) = enc {
                for phase in &s.phases {
                    for (i, slot) in phase.slots.iter().enumerate() {
                        if slot.positive.contains_name(trait_name) {
                            hits.push((
                                format!("{:?}", c.origin.kind),
                                c.debug_name.clone(),
                                s.variable_name.clone(),
                                phase.name.clone(),
                                i,
                            ));
                        }
                    }
                }
            }
        }
    }
    println!("  {} ship slots use this crew-manifest archetype.", hits.len());

    // Group by variable_name to see the host pattern.
    let mut by_var: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for h in &hits {
        *by_var.entry(h.2.clone()).or_default() += 1;
    }
    println!("  hosting variable_names:");
    for (n, c) in &by_var {
        println!("    {c:>4}  {n}");
    }

    // Distinct contract families (collapse on debug-name common prefix).
    let mut families: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for h in &hits {
        let prefix = family_prefix(&h.1);
        *families.entry(prefix).or_default() += 1;
    }
    println!("  hosting contract families:");
    for (n, c) in &families {
        println!("    {c:>4}  {n}");
    }

    // First handful of full rows for spot checks.
    println!("  sample slots (first 10):");
    for (kind, dn, var, phase, idx) in hits.iter().take(10) {
        println!("    [{kind}] {dn} → {var} / phase \"{phase}\" / slot {idx}");
    }
    println!();
}

// ── §2 NotWantedDuringShipCombat census ──────────────────────────────────────

fn section_2_not_wanted_during_combat(index: &MissionIndex) {
    println!("=== §2 NotWantedDuringShipCombat (negative tag) usage ===");
    let trait_name = "NotWantedDuringShipCombat";

    let mut hits = 0usize;
    let mut also_security = 0usize;
    let mut by_var: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut families: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();

    for c in &index.contracts {
        for enc in &c.encounters {
            if let Encounter::Ships(s) = enc {
                for phase in &s.phases {
                    for slot in &phase.slots {
                        if slot.negative.contains_name(trait_name) {
                            hits += 1;
                            *by_var.entry(s.variable_name.clone()).or_default() += 1;
                            *families.entry(family_prefix(&c.debug_name)).or_default() += 1;
                            if slot
                                .positive
                                .contains_name("PU_Human_PrivateSecurity_Light_wTurret")
                            {
                                also_security += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("  {hits} ship slots filter this trait out as a negative tag.");
    println!(
        "  of those, {also_security} also carry PU_Human_PrivateSecurity_Light_wTurret as a positive — \
         the two are paired."
    );
    println!("  hosting variable_names:");
    for (n, c) in &by_var {
        println!("    {c:>4}  {n}");
    }
    println!("  hosting contract families:");
    for (n, c) in &families {
        println!("    {c:>4}  {n}");
    }
    println!();
}

// ── §3 Foxwell ambush deep dive ──────────────────────────────────────────────

fn section_3_foxwell_dig(index: &MissionIndex, datacore: &Datacore) {
    println!("=== §3 Foxwell Enforcement Ship Ambush — deep dive ===");
    let pattern = "foxwellenforcement_shipambush_";

    let matches: Vec<_> = index
        .contracts
        .iter()
        .filter(|c| c.debug_name.to_lowercase().contains(pattern))
        .collect();
    println!("  {} matching contracts.\n", matches.len());

    for c in &matches {
        println!("── {}", c.debug_name);
        println!("    title:        {}", c.title.as_deref().unwrap_or("?"));
        println!("    handler:      {}", c.origin.source_debug_name);
        println!("    sub-of:       {:?}", c.origin.subcontract_of);
        println!("    crime_stat prereq:");
        for p in &c.prerequisites {
            if let sc_contracts::PrereqView::CrimeStat { min, max, .. } = p {
                println!("      min={min} max={max}");
            }
        }

        // Resolve every Locality prereq's available_locations.
        let locality_prereqs: Vec<&Guid> = c
            .prerequisites
            .iter()
            .filter_map(|p| {
                if let sc_contracts::PrereqView::Locality { locality: Some(g) } = p {
                    Some(g)
                } else {
                    None
                }
            })
            .collect();
        if locality_prereqs.is_empty() {
            println!("    (no Locality prereqs)");
        } else {
            println!("    Locality prereqs ({}):", locality_prereqs.len());
            for g in &locality_prereqs {
                if let Some(view) = index.localities.get(g) {
                    println!(
                        "      - {} :: {}  ({} location(s))",
                        view.name,
                        view.region_label,
                        view.locations.len(),
                    );
                    for loc in view.locations.iter().take(8) {
                        let body = if !loc.body_display_name.is_empty() {
                            loc.body_display_name.as_str()
                        } else {
                            loc.body.as_deref().unwrap_or("")
                        };
                        println!(
                            "          {} ({:?})  body={}  display={}",
                            loc.record_name, loc.system, body, loc.display_name,
                        );
                    }
                    if view.locations.len() > 8 {
                        println!("          ... +{} more", view.locations.len() - 8);
                    }
                } else {
                    println!("      - <unresolved {g}>");
                }
            }
        }

        println!();
    }
    let _ = datacore; // raw-walk path was investigative-only; sub-contract
                     // record lookup via db.record() returns None for the
                     // parent CareerContract id, so the typed encounter list
                     // (already dumped by contract_dump) is what we trust.
}


/// Trim numeric / suffix tail off a debug_name to group by family.
/// `FoxwellEnforcement_ShipAmbush_Stanton_VeryEasy` →
/// `FoxwellEnforcement_ShipAmbush_*`.
fn family_prefix(s: &str) -> String {
    // Strip common difficulty / region suffixes.
    let parts: Vec<&str> = s.split('_').collect();
    if parts.len() <= 2 {
        return s.to_string();
    }
    let drop_tail = parts
        .iter()
        .rev()
        .take_while(|p| {
            matches!(
                **p,
                "VeryEasy" | "Easy" | "Moderate" | "Medium" | "High" | "VeryHigh" | "Extreme"
            ) || matches!(**p, "Stanton" | "Pyro" | "Nyx")
        })
        .count();
    parts[..parts.len() - drop_tail].join("_")
}
