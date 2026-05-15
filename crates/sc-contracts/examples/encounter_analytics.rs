//! Targeted analytics on encounter slots, following up on
//! `role_investigation`. Driven by patterns the user spotted while
//! browsing the explorer:
//!
//! 1. **`AvailableToSalvage` mission-tag distribution** — what hosts
//!    it, what co-occurs, is `[pre-damaged]` perfectly correlated.
//! 2. **CargoShip_BP vs SalvageSpawnDescription_BP fingerprint** —
//!    same `[pre-damaged]` flag, different intent. Compare the
//!    typed-tag fingerprints.
//! 3. **Power-state AI traits** — `PoweredOff`, `EngineOff`,
//!    `ItemPortsUnlocked`, `DisablePowerInteractions`,
//!    `EnableInteractions`. Real game mechanics; do they cluster?
//! 4. **Wave-name catalogue** — what wave names exist, are any
//!    misleading vs their typed tags (the Ambush "Allies" case).
//! 5. **Faction × variable_name cross-tab** — which factions
//!    populate which encounter buckets.
//! 6. **`extended_text_token` per variable_name** — for the 3.7%
//!    of slots with a non-empty token, what's the host distribution.
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example encounter_analytics
//! ```

use std::collections::{BTreeMap, BTreeSet, HashMap};

use sc_contracts::{Encounter, MissionIndex, ShipSlot};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig};

/// One row in the analytics worklist — flatten contract → group →
/// wave → slot into a single record so every section can iterate
/// linearly without re-walking the contract graph.
///
/// Classified tag projections are precomputed at row construction so
/// the analytics sections don't have to re-thread the [`TagTree`]
/// through every helper. The projections come from the slot's
/// [`sc_contracts::TagBag`] classifier methods.
struct Row<'a> {
    /// Contract `debug_name` — kept for ad-hoc debugging when adding a
    /// new section that wants to spot-print which contracts a row came
    /// from. Suppressed when no section currently reads it.
    #[allow(dead_code)]
    contract_debug_name: &'a str,
    /// Contract handler kind, formatted via Debug.
    handler_kind: String,
    /// Encounter group's `mission_variable_name`.
    var_name: &'a str,
    /// Encounter group's `extended_text_token`.
    ext_token: &'a str,
    /// Wave name (`SpawnDescription_ShipGroup.Name`).
    wave_name: &'a str,
    /// Slot reference for raw access (initial_damage_settings, candidates, etc.).
    slot: &'a ShipSlot,
    // Precomputed classifications of the positive tag bag.
    factions: Vec<String>,
    cargo: Vec<String>,
    spawn_identifiers: Vec<String>,
    ai_traits: Vec<String>,
    mission_tags: Vec<String>,
    directives: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let index = MissionIndex::build(&datacore);

    // Flatten everything into a single Row vec for cheap multi-pass
    // analysis. Classifier projections are materialised once here so
    // section bodies read them as plain Vec<String>s.
    let tree = &index.tag_tree;
    let mut rows: Vec<Row<'_>> = Vec::new();
    for c in &index.contracts {
        let handler_kind = format!("{:?}", c.origin.kind);
        // Phase 6 of v2 widened encounters to ship + npc + entity.
        // The analytics here are ship-specific (cargo recovery vs
        // salvage, power-state traits) so we filter to Encounter::Ships.
        for enc in &c.encounters {
            let Encounter::Ships(s) = enc else { continue };
            for phase in &s.phases {
                for slot in &phase.slots {
                    let to_owned = |it: &mut dyn Iterator<Item = &str>| -> Vec<String> {
                        it.map(String::from).collect()
                    };
                    let factions = to_owned(&mut slot.positive.factions(tree));
                    let cargo = to_owned(&mut slot.positive.cargo(tree));
                    let spawn_identifiers = to_owned(&mut slot.positive.spawn_identifiers(tree));
                    let ai_traits = to_owned(&mut slot.positive.ai_traits(tree));
                    let mission_tags = to_owned(&mut slot.positive.mission_tags(tree));
                    let directives = to_owned(&mut slot.positive.directives());
                    rows.push(Row {
                        contract_debug_name: &c.debug_name,
                        handler_kind: handler_kind.clone(),
                        var_name: &s.variable_name,
                        ext_token: &s.extended_text_token,
                        wave_name: &phase.name,
                        slot,
                        factions,
                        cargo,
                        spawn_identifiers,
                        ai_traits,
                        mission_tags,
                        directives,
                    });
                }
            }
        }
    }
    eprintln!(
        "[index] {} contracts, {} encounter slots",
        index.contracts.len(),
        rows.len()
    );
    println!();

    section_1_available_to_salvage(&rows);
    section_2_cargoship_vs_salvage(&rows);
    section_3_power_state_traits(&rows);
    section_4_wave_names(&rows);
    section_5_faction_var_crosstab(&rows);
    section_6_ext_token_distribution(&rows);

    println!();
    println!("[done]");
    Ok(())
}

// ── §1 AvailableToSalvage ──────────────────────────────────────────────────

fn section_1_available_to_salvage(rows: &[Row<'_>]) {
    println!("=== Section 1: mission_tags='AvailableToSalvage' distribution ===");
    let hits: Vec<&Row<'_>> = rows
        .iter()
        .filter(|r| r.mission_tags.iter().any(|t| t == "AvailableToSalvage"))
        .collect();

    println!("  total slots tagged AvailableToSalvage: {}", hits.len());

    // Per variable_name
    let mut by_var: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &hits {
        *by_var.entry(r.var_name).or_default() += 1;
    }
    println!("  hosting variable_names:");
    for (n, c) in &by_var {
        println!("    {c:>4}  {n}");
    }

    // Pre-damage correlation
    let with_damage = hits
        .iter()
        .filter(|r| r.slot.initial_damage_settings.is_some())
        .count();
    println!(
        "  initial_damage_settings populated: {with_damage}/{} ({:.0}%)",
        hits.len(),
        if hits.is_empty() {
            0.0
        } else {
            100.0 * with_damage as f32 / hits.len() as f32
        }
    );

    // Co-occurring tags (mission_tags + cargo + ai_traits)
    let mut co_mission: BTreeMap<&str, usize> = BTreeMap::new();
    let mut co_cargo: BTreeMap<&str, usize> = BTreeMap::new();
    let mut co_ai: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &hits {
        for t in &r.mission_tags {
            if t != "AvailableToSalvage" {
                *co_mission.entry(t.as_str()).or_default() += 1;
            }
        }
        for t in &r.cargo {
            *co_cargo.entry(t.as_str()).or_default() += 1;
        }
        for t in &r.ai_traits {
            *co_ai.entry(t.as_str()).or_default() += 1;
        }
    }
    println!("  co-occurring mission_tags:");
    for (n, c) in top_by_count(&co_mission, 10) {
        println!("    {c:>4}  {n}");
    }
    println!("  co-occurring cargo:");
    for (n, c) in top_by_count(&co_cargo, 10) {
        println!("    {c:>4}  {n}");
    }
    println!("  co-occurring ai_traits:");
    for (n, c) in top_by_count(&co_ai, 10) {
        println!("    {c:>4}  {n}");
    }
    println!();
}

// ── §2 CargoShip_BP vs SalvageSpawnDescription_BP fingerprint ─────────────

fn section_2_cargoship_vs_salvage(rows: &[Row<'_>]) {
    println!("=== Section 2: CargoShip_BP vs SalvageSpawnDescription_BP fingerprint ===");
    print_fingerprint("CargoShip_BP", rows, |r| r.var_name == "CargoShip_BP");
    print_fingerprint("SalvageSpawnDescription_BP", rows, |r| {
        r.var_name == "SalvageSpawnDescription_BP"
    });
}

fn print_fingerprint(label: &str, rows: &[Row<'_>], pred: impl Fn(&Row<'_>) -> bool) {
    let hits: Vec<&Row<'_>> = rows.iter().filter(|r| pred(r)).collect();
    if hits.is_empty() {
        println!("  [{label}] no hits");
        return;
    }
    println!("  [{label}] {} slots", hits.len());

    let damaged = hits
        .iter()
        .filter(|r| r.slot.initial_damage_settings.is_some())
        .count();
    println!(
        "    pre-damaged: {damaged}/{} ({:.0}%)",
        hits.len(),
        100.0 * damaged as f32 / hits.len() as f32
    );

    let handlers = top_strings(hits.iter().map(|r| r.handler_kind.as_str()), 4);
    println!("    handler_kind:    {}", handlers);

    let waves = top_strings(hits.iter().map(|r| r.wave_name), 5);
    println!("    wave_name:       {}", waves);

    let mtags = top_strings(
        hits.iter()
            .flat_map(|r| r.mission_tags.iter().map(|s| s.as_str())),
        6,
    );
    println!("    mission_tags:    {}", mtags);

    let cargo = top_strings(
        hits.iter().flat_map(|r| r.cargo.iter().map(|s| s.as_str())),
        6,
    );
    println!("    cargo:           {}", cargo);

    let ai = top_strings(
        hits.iter()
            .flat_map(|r| r.ai_traits.iter().map(|s| s.as_str())),
        6,
    );
    println!("    ai_traits:       {}", ai);

    let dirs = top_strings(
        hits.iter()
            .flat_map(|r| r.directives.iter().map(|s| s.as_str())),
        4,
    );
    println!("    directives:      {}", dirs);

    let factions = top_strings(
        hits.iter()
            .flat_map(|r| r.factions.iter().map(|s| s.as_str())),
        4,
    );
    println!("    factions:        {}", factions);
    println!();
}

// ── §3 power-state AI traits ───────────────────────────────────────────────

fn section_3_power_state_traits(rows: &[Row<'_>]) {
    println!("=== Section 3: power-state AI traits ===");
    let power_traits = [
        "PoweredOff",
        "EngineOff",
        "ItemPortsUnlocked",
        "ItemPortsLocked",
        "DisablePowerInteractions",
        "EnableInteractions",
        "EmptyCrew",
    ];

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut pattern_counts: BTreeMap<String, usize> = BTreeMap::new();
    for trait_name in &power_traits {
        counts.insert(*trait_name, 0);
    }

    for r in rows {
        let mut present: Vec<&str> = Vec::new();
        for trait_name in &power_traits {
            if r.ai_traits.iter().any(|t| t == trait_name) {
                *counts.entry(trait_name).or_default() += 1;
                present.push(*trait_name);
            }
        }
        if !present.is_empty() {
            present.sort();
            *pattern_counts.entry(present.join(" + ")).or_default() += 1;
        }
    }

    let total = rows.len();
    println!("  per-trait totals (out of {total} slots):");
    let mut entries: Vec<_> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (n, c) in &entries {
        let pct = 100.0 * **c as f32 / total as f32;
        println!("    {c:>5}  ({pct:>4.1}%)  {n}");
    }
    println!();

    println!("  top patterns (combinations seen on the same slot):");
    let mut pe: Vec<_> = pattern_counts.iter().collect();
    pe.sort_by(|a, b| b.1.cmp(a.1));
    for (pat, c) in pe.iter().take(15) {
        println!("    {c:>5}  {pat}");
    }
    println!();

    // Pre-damage × power-pattern: are pre-damaged ships always also
    // powered-off?
    let damaged_with_power: usize = rows
        .iter()
        .filter(|r| {
            r.slot.initial_damage_settings.is_some()
                && r.ai_traits.iter().any(|t| {
                    matches!(
                        t.as_str(),
                        "PoweredOff" | "EngineOff" | "DisablePowerInteractions"
                    )
                })
        })
        .count();
    let damaged_total: usize = rows
        .iter()
        .filter(|r| r.slot.initial_damage_settings.is_some())
        .count();
    println!(
        "  of {damaged_total} pre-damaged slots, {damaged_with_power} ({:.0}%) also carry one of \
         PoweredOff / EngineOff / DisablePowerInteractions",
        if damaged_total == 0 {
            0.0
        } else {
            100.0 * damaged_with_power as f32 / damaged_total as f32
        }
    );
    println!();
}

// ── §4 wave names ──────────────────────────────────────────────────────────

fn section_4_wave_names(rows: &[Row<'_>]) {
    println!("=== Section 4: wave-name catalogue ===");
    let mut by_wave: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    let mut wave_count: BTreeMap<&str, usize> = BTreeMap::new();
    for r in rows {
        let name = if r.wave_name.is_empty() {
            "(unnamed)"
        } else {
            r.wave_name
        };
        *wave_count.entry(name).or_default() += 1;
        by_wave.entry(name).or_default().insert(r.var_name);
    }
    let mut entries: Vec<_> = wave_count.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("  top 20 wave names (count, distinct host variable_names):");
    for (n, c) in entries.iter().take(20) {
        let hosts = by_wave.get(*n).map(|s| s.len()).unwrap_or(0);
        println!("    {c:>5}  hosts={hosts:<3}  {n}");
    }
    println!("  (total distinct wave names: {})", entries.len());

    // Mismatch hunt: waves named like "Allies" / "Friendly" / "Defenders"
    // but whose slots' typed tags carry Target / Bounty.
    println!();
    println!("  wave-name vs typed-tag mismatch hunt:");
    let allied_names = ["Allies", "Friendly", "Defenders", "Escort", "Allied"];
    let mut mismatches: BTreeMap<&str, Vec<&Row<'_>>> = BTreeMap::new();
    for r in rows {
        let allied_label = allied_names
            .iter()
            .any(|n| r.wave_name.eq_ignore_ascii_case(n) || r.wave_name.contains(n));
        let typed_hostile = r.spawn_identifiers.iter().any(|t| t == "Target")
            || r.cargo.iter().any(|t| t == "Bounty");
        if allied_label && typed_hostile {
            mismatches.entry(r.wave_name).or_default().push(r);
        }
    }
    if mismatches.is_empty() {
        println!("    (none — typed tags align with allied-style wave names)");
    } else {
        for (wave, rs) in &mismatches {
            let var_names: BTreeSet<&str> = rs.iter().map(|r| r.var_name).collect();
            println!(
                "    {wave:<24} {} slots across var_names: {}",
                rs.len(),
                var_names
                    .iter()
                    .take(4)
                    .copied()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    println!();
}

// ── §5 faction × variable_name cross-tab ───────────────────────────────────

fn section_5_faction_var_crosstab(rows: &[Row<'_>]) {
    println!("=== Section 5: faction × variable_name cross-tab ===");
    let mut counts: HashMap<(&str, &str), usize> = HashMap::new();
    let mut var_totals: BTreeMap<&str, usize> = BTreeMap::new();
    let mut faction_totals: BTreeMap<&str, usize> = BTreeMap::new();
    for r in rows {
        for f in &r.factions {
            *counts.entry((r.var_name, f.as_str())).or_default() += 1;
            *faction_totals.entry(f.as_str()).or_default() += 1;
        }
        if !r.factions.is_empty() {
            *var_totals.entry(r.var_name).or_default() += 1;
        }
    }
    println!("  faction totals:");
    let mut fes: Vec<_> = faction_totals.iter().collect();
    fes.sort_by(|a, b| b.1.cmp(a.1));
    for (n, c) in fes.iter().take(12) {
        println!("    {c:>5}  {n}");
    }
    println!();
    println!("  per variable_name (top 15 by faction-tagged slot count): top 3 factions");
    let mut ves: Vec<_> = var_totals.iter().collect();
    ves.sort_by(|a, b| b.1.cmp(a.1));
    for (var, total) in ves.iter().take(15) {
        let mut row: Vec<(&str, usize)> = counts
            .iter()
            .filter(|((v, _), _)| v == *var)
            .map(|((_, f), c)| (*f, *c))
            .collect();
        row.sort_by(|a, b| b.1.cmp(&a.1));
        let formatted: Vec<String> = row
            .iter()
            .take(3)
            .map(|(f, c)| format!("{f}×{c}"))
            .collect();
        println!("    {total:>4}  {var:<48}  {}", formatted.join(", "));
    }
    println!();
}

// ── §6 extended_text_token distribution ────────────────────────────────────

fn section_6_ext_token_distribution(rows: &[Row<'_>]) {
    println!("=== Section 6: extended_text_token where populated ===");
    // Roll up by (token, var_name) since the token lives on the group
    // and is shared across all slots in that group — count groups, not
    // slots, by deduplicating on (var_name, token) within a single
    // contract is overkill for a sample; we'll just count slots.
    let mut by_token: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    for r in rows {
        if r.ext_token.is_empty() {
            continue;
        }
        *by_token
            .entry(r.ext_token)
            .or_default()
            .entry(r.var_name)
            .or_default() += 1;
    }
    if by_token.is_empty() {
        println!("  (no slots with non-empty extended_text_token)");
    } else {
        for (token, vars) in &by_token {
            let total: usize = vars.values().sum();
            println!("  {total:>5}  ext_token=\"{token}\"");
            for (var, c) in vars {
                println!("           {c:>5}  {var}");
            }
        }
    }
    println!();
}

// ── helpers ────────────────────────────────────────────────────────────────

fn top_by_count<'a, K>(map: &'a BTreeMap<K, usize>, n: usize) -> Vec<(&'a K, &'a usize)>
where
    K: Ord,
{
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    entries.into_iter().take(n).collect()
}

/// Top-N most frequent strings in an iterator, formatted as
/// `name×count, name×count, ...`.
fn top_strings<'a, I: Iterator<Item = &'a str>>(it: I, n: usize) -> String {
    let mut counts: BTreeMap<&'a str, usize> = BTreeMap::new();
    for s in it {
        *counts.entry(s).or_default() += 1;
    }
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries
        .into_iter()
        .take(n)
        .map(|(s, c)| format!("{s}×{c}"))
        .collect::<Vec<_>>()
        .join(", ")
}
