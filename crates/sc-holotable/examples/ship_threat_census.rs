//! Census of what the ships in mission encounters bring: for every ship a
//! mission can spawn, walk its DCB default loadout (recursively — turrets,
//! racks) and collect the threat and capacity signals a pool summary would
//! need: gun alpha (per-shot, explosion included), distortion guns,
//! missiles, armor deflection, EMP / QED devices, shield HP, and hydrogen /
//! quantum fuel capacity. Then aggregate per encounter pool.
//!
//! Probe for the "summarize ship pools by danger" design: how complete is
//! the loadout data, and does a pool summary (size range, max armor, max
//! alpha, EMP) tell a player more than the ship names do?
//!
//! ```bash
//! cargo run -p sc-holotable --release --features full --example ship_threat_census
//! # optional: CENSUS_CSV=<path> writes the per-ship table
//! ```
use std::collections::{BTreeMap, HashMap, HashSet};

use sc_holotable::asset::generated::{
    DataForgeComponentParamsPtr, EItemType, EntityClassDefinition, SBaseCargoUnitPtr,
    SItemPortLoadoutBaseParamsPtr,
};
use sc_holotable::asset::{AssetConfig, AssetData, AssetSource, DataPools, Datacore, Guid};
use sc_holotable::missions::{Encounter, Missions};
use sc_holotable::prelude::RecordCollection;
use sc_holotable::ship_components::{ShipComponentKind, ShipComponents};
use sc_holotable::weapons::Weapons;

#[derive(Debug, Default, Clone)]
struct ShipStats {
    name: String,
    size: i32,
    role: String,
    loadout: &'static str,
    equipped: usize,
    guns: usize,
    /// Highest single-gun alpha (per shot × pellets), all damage types.
    max_alpha: f32,
    /// Sum of every gun's alpha — a full broadside.
    total_alpha: f32,
    distortion_guns: usize,
    missiles: usize,
    max_missile: f32,
    /// Armor deflection threshold per damage type (max over armor items).
    deflect_physical: f32,
    deflect_energy: f32,
    emp: bool,
    qed: bool,
    shield_hp: f32,
    h2_scu: f32,
    qt_scu: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_holotable::install::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let pools = &datacore.records().pools;
    let foundations = sc_holotable::build_foundations(&datacore);
    let items = &foundations.items;
    let missions = Missions::build(&datacore);
    let tree = &missions.tag_tree;
    let weapons = Weapons::build(&datacore, items);
    let guns: HashMap<Guid, &sc_holotable::weapons::ShipWeapon> =
        weapons.ships.iter().map(|w| (w.guid, w)).collect();
    let missiles: HashMap<Guid, &sc_holotable::weapons::Missile> =
        weapons.missiles.iter().map(|m| (m.guid, m)).collect();
    let components = ShipComponents::build(&datacore, items);

    // ── Every ship candidate any mission encounter can spawn ────────────
    // slot = (category, faction set, candidate set)
    let mut slots: Vec<(String, Vec<String>, Vec<Guid>)> = Vec::new();
    let mut seen_slots: HashSet<(Vec<String>, Vec<Guid>)> = HashSet::new();
    let mut ship_guids: HashSet<Guid> = HashSet::new();
    for (_, m) in missions.iter() {
        let category = m
            .category
            .and_then(|c| missions.mission_types.get(&c))
            .and_then(|t| locale.resolve(&t.name_key))
            .unwrap_or("?")
            .to_string();
        for e in &m.encounters {
            let Encounter::Ships(s) = e else { continue };
            for phase in &s.phases {
                for group in &phase.groups {
                    let mut cands: Vec<Guid> = Vec::new();
                    let mut factions: Vec<String> = Vec::new();
                    for opt in &group.options {
                        cands.extend(opt.candidates.iter().map(|c| c.entity_guid));
                        factions.extend(opt.positive.factions(tree).map(str::to_owned));
                    }
                    // CigGuid has no Ord; its string form orders stably.
                    cands.sort_by_key(|g| g.to_string());
                    cands.dedup();
                    factions.sort();
                    factions.dedup();
                    if cands.is_empty() {
                        continue;
                    }
                    ship_guids.extend(cands.iter().copied());
                    if seen_slots.insert((factions.clone(), cands.clone())) {
                        slots.push((category.clone(), factions, cands));
                    }
                }
            }
        }
    }

    // ── Per-ship stats via the default loadout ──────────────────────────
    let mut stats: HashMap<Guid, ShipStats> = HashMap::new();
    for guid in &ship_guids {
        let mut s = ShipStats {
            name: missions
                .ships
                .display_name(guid, items, locale)
                .unwrap_or("?")
                .to_string(),
            size: items.get(guid).map_or(0, |i| i.size),
            loadout: "none",
            ..Default::default()
        };
        let Some(ecd) = datacore.resolve::<EntityClassDefinition>(guid) else {
            stats.insert(*guid, s);
            continue;
        };
        for c in &ecd.components {
            if let DataForgeComponentParamsPtr::VehicleComponentParams(h) = c
                && let Some(v) = h.get(pools)
            {
                s.role = locale.resolve(&v.vehicle_role).unwrap_or("").to_string();
            }
        }
        let mut equipped: Vec<Guid> = Vec::new();
        let mut visited: HashSet<Guid> = HashSet::new();
        s.loadout = default_loadout(&datacore, pools, ecd, 0, &mut equipped, &mut visited);
        s.equipped = equipped.len();
        for g in &equipped {
            if let Some(w) = guns.get(g) {
                // Alpha: one trigger pull — per-shot damage × pellets.
                // (Not `volley_damage`, which is a whole burst-until-stop.)
                let pellets = w.pellet_count.unwrap_or(1).max(1) as f32;
                let alpha = w.damage.as_ref().map_or(0.0, |d| d.total() * pellets);
                s.guns += 1;
                s.max_alpha = s.max_alpha.max(alpha);
                s.total_alpha += alpha;
                if w.damage.as_ref().is_some_and(|d| d.distortion > 0.0) {
                    s.distortion_guns += 1;
                }
            }
            if let Some(m) = missiles.get(g) {
                s.missiles += 1;
                s.max_missile = s
                    .max_missile
                    .max(m.damage.as_ref().map_or(0.0, |d| d.total()));
            }
            if let Some(c) = components.get(g)
                && matches!(c.kind, ShipComponentKind::Shield)
            {
                s.shield_hp += c.shield_max_health.unwrap_or(0.0);
            }
            let item_type = items.get(g).map(|i| &i.item_type);
            match item_type {
                Some(EItemType::EMP) => s.emp = true,
                Some(EItemType::QuantumInterdictionGenerator) => s.qed = true,
                Some(EItemType::FuelTank) => s.h2_scu += capacity_scu(&datacore, pools, g),
                Some(EItemType::QuantumFuelTank) => s.qt_scu += capacity_scu(&datacore, pools, g),
                _ => {}
            }
            if let Some(ecd) = datacore.resolve::<EntityClassDefinition>(g) {
                for c in &ecd.components {
                    if let DataForgeComponentParamsPtr::SCItemVehicleArmorParams(h) = c
                        && let Some(a) = h.get(pools)
                        && let Some(d) = a.armor_deflection.as_ref().and_then(|h| h.get(pools))
                        && let Some(v) = d.deflection_value.as_ref().and_then(|h| h.get(pools))
                    {
                        s.deflect_physical = s.deflect_physical.max(v.damage_physical);
                        s.deflect_energy = s.deflect_energy.max(v.damage_energy);
                    }
                }
            }
        }
        stats.insert(*guid, s);
    }

    // ── 1. Coverage ─────────────────────────────────────────────────────
    let all: Vec<&ShipStats> = stats.values().collect();
    let count = |f: &dyn Fn(&ShipStats) -> bool| all.iter().filter(|s| f(s)).count();
    println!("== 1. coverage over {} distinct mission ships", all.len());
    for kind in ["manual", "xml", "none"] {
        println!("  loadout {kind:<7} {}", count(&|s| s.loadout == kind));
    }
    println!("  with guns            {}", count(&|s| s.guns > 0));
    println!(
        "  with armor deflect   {}",
        count(&|s| s.deflect_physical > 0.0)
    );
    println!("  with shields         {}", count(&|s| s.shield_hp > 0.0));
    println!("  with H2 capacity     {}", count(&|s| s.h2_scu > 0.0));
    println!("  with QT capacity     {}", count(&|s| s.qt_scu > 0.0));
    println!(
        "  with EMP / QED       {} / {}",
        count(&|s| s.emp),
        count(&|s| s.qed)
    );
    println!(
        "  with distortion guns {}",
        count(&|s| s.distortion_guns > 0)
    );

    // ── 2. Per-ship table, by size ──────────────────────────────────────
    let mut sorted: Vec<&ShipStats> = all.clone();
    sorted.sort_by(|a, b| a.size.cmp(&b.size).then(a.name.cmp(&b.name)));
    println!(
        "\n== 2. ships (size · name · role · guns · max alpha · broadside · defl phys/energy · shield · H2/QT · flags)"
    );
    for s in sorted.iter().step_by((sorted.len() / 45).max(1)) {
        println!(
            "  S{} {:<30} {:<20} {:>2} {:>7.0} {:>7.0} {:>5.0}/{:<5.0} {:>7.0} {:>6.1}/{:<5.1} {}{}{}",
            s.size,
            trunc(&s.name, 30),
            trunc(&s.role, 20),
            s.guns,
            s.max_alpha,
            s.total_alpha,
            s.deflect_physical,
            s.deflect_energy,
            s.shield_hp,
            s.h2_scu,
            s.qt_scu,
            if s.emp { "EMP " } else { "" },
            if s.qed { "QED " } else { "" },
            if s.distortion_guns > 0 { "DIST" } else { "" },
        );
    }
    if let Ok(path) = std::env::var("CENSUS_CSV") {
        let mut csv = String::from(
            "name,size,role,loadout,guns,max_alpha,total_alpha,distortion_guns,missiles,max_missile,deflect_phys,deflect_energy,emp,qed,shield_hp,h2_scu,qt_scu\n",
        );
        for s in &sorted {
            csv.push_str(&format!(
                "\"{}\",{},\"{}\",{},{},{:.0},{:.0},{},{},{:.0},{:.0},{:.0},{},{},{:.0},{:.2},{:.2}\n",
                s.name, s.size, s.role, s.loadout, s.guns, s.max_alpha, s.total_alpha,
                s.distortion_guns, s.missiles, s.max_missile, s.deflect_physical,
                s.deflect_energy, s.emp, s.qed, s.shield_hp, s.h2_scu, s.qt_scu
            ));
        }
        std::fs::write(&path, csv)?;
        println!("  (full table written to {path})");
    }

    // ── 3. Pools ────────────────────────────────────────────────────────
    println!("\n== 3. {} distinct encounter pools", slots.len());
    let mut by_spread: BTreeMap<i32, usize> = BTreeMap::new();
    let mut by_len: BTreeMap<usize, usize> = BTreeMap::new();
    let (mut with_emp, mut with_dist, mut civ) = (0, 0, 0);
    for (_, factions, cands) in &slots {
        let ss: Vec<&ShipStats> = cands.iter().filter_map(|g| stats.get(g)).collect();
        let lo = ss.iter().map(|s| s.size).min().unwrap_or(0);
        let hi = ss.iter().map(|s| s.size).max().unwrap_or(0);
        *by_spread.entry(hi - lo).or_default() += 1;
        *by_len.entry(bucket(cands.len())).or_default() += 1;
        with_emp += ss.iter().any(|s| s.emp) as usize;
        with_dist += ss.iter().any(|s| s.distortion_guns > 0) as usize;
        civ += factions.iter().any(|f| f == "Civilians") as usize;
    }
    println!("  candidates per pool (bucketed): {by_len:?}");
    println!("  size spread (max-min) per pool: {by_spread:?}");
    println!(
        "  pools with an EMP ship: {with_emp}, with a distortion gun: {with_dist}, civilian pools: {civ}"
    );
    println!("\n  sample pool summaries (largest pools first):");
    let mut big: Vec<&(String, Vec<String>, Vec<Guid>)> = slots.iter().collect();
    big.sort_by_key(|s| std::cmp::Reverse(s.2.len()));
    for (category, factions, cands) in big.iter().take(14) {
        let ss: Vec<&ShipStats> = cands.iter().filter_map(|g| stats.get(g)).collect();
        let lo = ss.iter().map(|s| s.size).min().unwrap_or(0);
        let hi = ss.iter().map(|s| s.size).max().unwrap_or(0);
        let fmax = |f: fn(&ShipStats) -> f32| ss.iter().map(|s| f(s)).fold(0.0f32, f32::max);
        println!(
            "  {:<16} {:<22} {:>2} ships S{lo}-S{hi} · max armor deflect {:.0} phys / {:.0} energy · max gun alpha {:.0} · max broadside {:.0} · max shield {:.0}{}{}",
            trunc(category, 16),
            trunc(&factions.join("+"), 22),
            cands.len(),
            fmax(|s| s.deflect_physical),
            fmax(|s| s.deflect_energy),
            fmax(|s| s.max_alpha),
            fmax(|s| s.total_alpha),
            fmax(|s| s.shield_hp),
            if ss.iter().any(|s| s.emp) {
                " · EMP"
            } else {
                ""
            },
            if ss.iter().any(|s| s.distortion_guns > 0) {
                " · distortion"
            } else {
                ""
            },
        );
    }

    // ── 4. Refuel recipients ────────────────────────────────────────────
    println!("\n== 4. refuel-category civilian pools: recipients' fuel capacity");
    for (category, factions, cands) in &slots {
        if !category.to_lowercase().contains("refuel") || !factions.iter().any(|f| f == "Civilians")
        {
            continue;
        }
        let ss: Vec<&ShipStats> = cands.iter().filter_map(|g| stats.get(g)).collect();
        let h2: Vec<f32> = ss.iter().map(|s| s.h2_scu).collect();
        let qt: Vec<f32> = ss.iter().map(|s| s.qt_scu).collect();
        let range = |v: &[f32]| {
            (
                v.iter().copied().fold(f32::MAX, f32::min),
                v.iter().copied().fold(0.0f32, f32::max),
            )
        };
        let (h_lo, h_hi) = range(&h2);
        let (q_lo, q_hi) = range(&qt);
        println!(
            "  {:>2} ships S{}-S{} · H2 {h_lo:.1}-{h_hi:.1} SCU · QT {q_lo:.1}-{q_hi:.1} SCU",
            cands.len(),
            ss.iter().map(|s| s.size).min().unwrap_or(0),
            ss.iter().map(|s| s.size).max().unwrap_or(0),
        );
    }
    Ok(())
}

/// Walk an entity's default loadout, collecting every equipped entity GUID
/// (and, recursively, what those items equip). Returns the top loadout's
/// shape: `manual` (DCB entries), `xml` (a loose XML file — not walkable
/// here) or `none`.
fn default_loadout(
    datacore: &Datacore,
    pools: &DataPools,
    ecd: &EntityClassDefinition,
    depth: usize,
    out: &mut Vec<Guid>,
    visited: &mut HashSet<Guid>,
) -> &'static str {
    let mut shape = "none";
    for c in &ecd.components {
        let DataForgeComponentParamsPtr::SEntityComponentDefaultLoadoutParams(h) = c else {
            continue;
        };
        if let Some(lo) = h.get(pools).and_then(|p| p.loadout.as_ref()) {
            shape = walk_loadout(datacore, pools, lo, depth, out, visited);
        }
    }
    shape
}

fn walk_loadout(
    datacore: &Datacore,
    pools: &DataPools,
    lo: &SItemPortLoadoutBaseParamsPtr,
    depth: usize,
    out: &mut Vec<Guid>,
    visited: &mut HashSet<Guid>,
) -> &'static str {
    match lo {
        SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutManualParams(h) => {
            let Some(manual) = h.get(pools) else {
                return "none";
            };
            for entry in manual.entries.iter().filter_map(|e| e.get(pools)) {
                if let Some(g) = entry.entity_class_reference {
                    out.push(g);
                    // The equipped item's own default loadout (turret guns,
                    // rack missiles) — once per item, bounded depth.
                    if depth < 4
                        && visited.insert(g)
                        && let Some(child) = datacore.resolve::<EntityClassDefinition>(&g)
                    {
                        default_loadout(datacore, pools, child, depth + 1, out, visited);
                    }
                }
                if let Some(sub) = entry.loadout.as_ref() {
                    walk_loadout(datacore, pools, sub, depth + 1, out, visited);
                }
            }
            "manual"
        }
        SItemPortLoadoutBaseParamsPtr::SItemPortLoadoutXMLParams(_) => "xml",
        _ => "none",
    }
}

/// A tank's capacity in SCU, from its `ResourceContainer` component.
fn capacity_scu(datacore: &Datacore, pools: &DataPools, guid: &Guid) -> f32 {
    let Some(ecd) = datacore.resolve::<EntityClassDefinition>(guid) else {
        return 0.0;
    };
    let mut total = 0.0;
    for c in &ecd.components {
        let DataForgeComponentParamsPtr::ResourceContainer(h) = c else {
            continue;
        };
        let Some(rc) = h.get(pools) else { continue };
        total += match rc.capacity.as_ref() {
            Some(SBaseCargoUnitPtr::SStandardCargoUnit(h)) => {
                h.get(pools).map_or(0.0, |u| u.standard_cargo_units)
            }
            Some(SBaseCargoUnitPtr::SCentiCargoUnit(h)) => {
                h.get(pools).map_or(0.0, |u| u.centi_scu as f32 / 100.0)
            }
            Some(SBaseCargoUnitPtr::SMicroCargoUnit(h)) => h
                .get(pools)
                .map_or(0.0, |u| u.micro_scu as f32 / 1_000_000.0),
            _ => 0.0,
        };
    }
    total
}

fn bucket(n: usize) -> usize {
    match n {
        0..=1 => 1,
        2..=3 => 3,
        4..=7 => 7,
        8..=15 => 15,
        16..=31 => 31,
        _ => 99,
    }
}

fn trunc(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}
