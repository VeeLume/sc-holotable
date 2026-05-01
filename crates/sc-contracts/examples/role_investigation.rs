//! §2 EncounterRole investigation — does the live DCB carry typed
//! signals on ship-spawn descriptions sufficient to derive
//! `EncounterGroup.role` / `sub_role` without falling back to substring
//! matching on `mission_variable_name`?
//!
//! Outputs seven sections:
//!
//! 1. **Spawn-variant coverage** — how many `MissionProperty.value`
//!    instances carry each spawn-shape variant (Ship / NPC / Entity vs
//!    everything else). Quantifies the "ship-only encounters today" gap.
//! 2. **Variable-name population** — top mission_variable_name values
//!    on ship spawns, classified with sc-langpatch's substring rules.
//!    Shows the long tail and how many fall through to Unknown.
//! 3. **`extended_text_token` histogram** — sibling field to
//!    `mission_variable_name`. Is it useful or always empty?
//! 4. **Tag-tree role candidates under `AI`** — tags whose names hint
//!    at role (Target / Defenders / Bounty / Allied / Hostile / …).
//!    Counts entity carriers + spawn references.
//! 5. **`initial_damage_settings` correlation** — for each mission_
//!    variable_name, what fraction of its options spawn pre-damaged?
//!    Validates the "salvage" hypothesis.
//! 6. **AI-role tag presence per variable_name** — cross-correlation
//!    of substring labels vs typed AI-tag membership. Shows whether
//!    typed tags align with substring-derived roles.
//! 7. **Markup / entity-tag / embedded-name spot checks** — sample
//!    inspection of currently-unused fields.
//!
//! Run:
//!
//! ```bash
//! cargo run -p sc-contracts --release --example role_investigation
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::generated::{
    BaseMissionPropertyValuePtr, MissionPropertyValue_ShipSpawnDescriptions,
    SpawnDescription_Ship, SpawnDescription_ShipOptions, TagList,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, DatacoreConfig, Guid};

/// Sub-string classifier matching sc-langpatch's current logic. Returns
/// the role label, or "Unknown" for anything that doesn't match. We
/// drop sc-langpatch's "default Hostile" fallback so we can measure how
/// much the substring approach actually covers.
fn classify_role_substring(var_name: &str) -> &'static str {
    let lower = var_name.to_lowercase();
    if lower.contains("salvage") {
        "Salvage"
    } else if lower.contains("hostile")
        || lower.contains("missiontargets")
        || lower.contains("waveships")
    {
        "Hostile"
    } else if lower.contains("attacked")
        || lower.contains("allied")
        || lower.contains("escort")
        || lower.contains("defend")
    {
        "Allied"
    } else {
        "Unknown"
    }
}

/// Sub-role detection (Wave N / Reinforcement) matching sc-langpatch's
/// current logic.
fn classify_sub_role(var_name: &str) -> Option<String> {
    let lower = var_name.to_lowercase();
    // Find a "wave\d+" pattern manually (no regex dep).
    if let Some(idx) = lower.find("wave") {
        let rest = &lower[idx + 4..];
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty()
            && let Ok(n) = digits.parse::<u8>()
        {
            return Some(format!("Wave({n})"));
        }
    }
    if lower.contains("reinforce") {
        return Some("Reinforcement".into());
    }
    None
}

fn tag_set(pools: &sc_extract::DataPools, h: Option<&sc_extract::generated::Handle<TagList>>) -> HashSet<Guid> {
    let Some(h) = h else { return HashSet::new() };
    let Some(list) = h.get(pools) else {
        return HashSet::new();
    };
    list.tags.iter().copied().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_installs::discover_primary()?;
    eprintln!("[install] {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let pools = &datacore.records().pools;
    let snap = datacore.snapshot();
    let tag_tree = &snap.tag_tree;

    // ── Section 1: spawn-variant coverage ────────────────────────────────
    println!("=== Section 1: MissionProperty.value variant coverage ===");
    let mut variant_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut ship_props_total = 0usize;
    for prop in pools.multi_feature.mission_property.iter().flatten() {
        let label = match &prop.value {
            None => "<none>",
            Some(v) => match v {
                BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(_) => {
                    ship_props_total += 1;
                    "ShipSpawnDescriptions"
                }
                BaseMissionPropertyValuePtr::MissionPropertyValue_NPCSpawnDescriptions(_) => {
                    "NPCSpawnDescriptions"
                }
                BaseMissionPropertyValuePtr::MissionPropertyValue_EntitySpawnDescriptions(_) => {
                    "EntitySpawnDescriptions"
                }
                BaseMissionPropertyValuePtr::MissionPropertyValue_AIName(_) => "AIName",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Tags(_) => "Tags",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Location(_) => "Location",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Locations(_) => "Locations",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Boolean(_) => "Boolean",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Integer(_) => "Integer",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Float(_) => "Float",
                BaseMissionPropertyValuePtr::MissionPropertyValue_StringHash(_) => "StringHash",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Object(_) => "Object",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Organization(_) => "Organization",
                BaseMissionPropertyValuePtr::MissionPropertyValue_Reward(_) => "Reward",
                _ => "<other>",
            },
        };
        *variant_counts.entry(label).or_default() += 1;
    }
    let total_props: usize = variant_counts.values().sum();
    let mut entries: Vec<_> = variant_counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    for (k, v) in &entries {
        let pct = 100.0 * **v as f32 / total_props as f32;
        println!("  {k:<32} {v:>6} ({pct:.1}%)");
    }
    println!("  ─────────────────────────────────────");
    println!("  total MissionProperty records: {total_props}");
    println!();

    // ── Walk every Ship-spawn-bearing MissionProperty once and cache
    //    everything subsequent sections need. ───────────────────────────
    struct Sample {
        var_name: String,
        ext_token: String,
        positive_tags: HashSet<Guid>,
        markup_tags: HashSet<Guid>,
        entity_tags: HashSet<Guid>,
        has_initial_damage: bool,
        embedded_name_present: bool,
    }
    let mut samples: Vec<Sample> = Vec::new();
    for prop in pools.multi_feature.mission_property.iter().flatten() {
        let Some(BaseMissionPropertyValuePtr::MissionPropertyValue_ShipSpawnDescriptions(h)) =
            prop.value.as_ref()
        else {
            continue;
        };
        let Some(val): Option<&MissionPropertyValue_ShipSpawnDescriptions> = h.get(pools) else {
            continue;
        };
        for grp_h in &val.spawn_descriptions {
            let Some(grp) = grp_h.get(pools) else {
                continue;
            };
            for opts_h in &grp.ships {
                let Some(opts): Option<&SpawnDescription_ShipOptions> = opts_h.get(pools) else {
                    continue;
                };
                for opt_h in &opts.options {
                    let Some(opt): Option<&SpawnDescription_Ship> = opt_h.get(pools) else {
                        continue;
                    };
                    samples.push(Sample {
                        var_name: prop.mission_variable_name.clone(),
                        ext_token: prop.extended_text_token.clone(),
                        positive_tags: tag_set(pools, opt.tags.as_ref()),
                        markup_tags: tag_set(pools, opt.markup_tags.as_ref()),
                        entity_tags: tag_set(pools, opt.entity_tags.as_ref()),
                        has_initial_damage: opt.initial_damage_settings.is_some(),
                        embedded_name_present: grp.embedded_name.is_some(),
                    });
                }
            }
        }
    }
    println!(
        "[index] {} ship-spawn samples across {} ship-bearing MissionProperty records",
        samples.len(),
        ship_props_total,
    );
    println!();

    // ── Section 2: variable_name population ──────────────────────────────
    println!("=== Section 2: mission_variable_name on ship spawns ===");
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    let mut role_tally: BTreeMap<&'static str, usize> = BTreeMap::new();
    for s in &samples {
        *name_counts.entry(s.var_name.clone()).or_default() += 1;
        *role_tally
            .entry(classify_role_substring(&s.var_name))
            .or_default() += 1;
    }
    println!("Substring role classification (per spawn option):");
    let mut role_entries: Vec<_> = role_tally.iter().collect();
    role_entries.sort_by(|a, b| b.1.cmp(a.1));
    let total_samples = samples.len();
    for (k, v) in &role_entries {
        let pct = 100.0 * **v as f32 / total_samples as f32;
        println!("  {k:<10} {v:>6} ({pct:.1}%)");
    }
    let unknown = role_tally.get("Unknown").copied().unwrap_or(0);
    println!(
        "  → {:.1}% of ship spawns fall through the substring classifier with no default-Hostile",
        100.0 * unknown as f32 / total_samples as f32
    );
    println!();

    let mut name_entries: Vec<_> = name_counts.iter().collect();
    name_entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("Top 30 mission_variable_name values:");
    for (name, count) in name_entries.iter().take(30) {
        let cls = classify_role_substring(name);
        let sub = classify_sub_role(name)
            .map(|s| format!(" sub={s}"))
            .unwrap_or_default();
        println!("  {count:>5}  {name:<48}  → {cls}{sub}");
    }
    println!("  (distinct names: {})", name_counts.len());
    println!();

    // ── Section 3: extended_text_token histogram ─────────────────────────
    println!("=== Section 3: extended_text_token histogram ===");
    let mut ext_counts: HashMap<String, usize> = HashMap::new();
    for s in &samples {
        *ext_counts.entry(s.ext_token.clone()).or_default() += 1;
    }
    let empty = ext_counts.get("").copied().unwrap_or(0);
    println!(
        "  empty tokens: {empty}/{total_samples} ({:.1}%)",
        100.0 * empty as f32 / total_samples as f32
    );
    let mut ext_entries: Vec<_> = ext_counts.iter().filter(|(k, _)| !k.is_empty()).collect();
    ext_entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("  top 20 non-empty values:");
    for (k, v) in ext_entries.iter().take(20) {
        println!("    {v:>5}  {k}");
    }
    println!("  (distinct non-empty: {})", ext_entries.len());
    println!();

    // ── Section 4: tag-tree role candidates under AI ─────────────────────
    println!("=== Section 4: tag-tree role candidates ===");
    let role_keywords = [
        "target",
        "defender",
        "bounty",
        "allied",
        "hostile",
        "friendly",
        "enemy",
        "ally",
        "salvage",
        "escort",
        "wreck",
        "reinforce",
    ];
    // All candidate role tags anywhere in the tree (not just AI subtree —
    // we want to see whether role markers are scattered beyond AI).
    let mut role_tags: Vec<(String, Guid, Vec<String>)> = Vec::new();
    for node in tag_tree.iter() {
        let lower = node.name.to_lowercase();
        if role_keywords.iter().any(|kw| lower.contains(kw)) {
            role_tags.push((node.name.clone(), node.guid, tag_tree.path(&node.guid).iter().map(|s| s.to_string()).collect()));
        }
    }
    // Count how many ship-spawn samples reference each role tag (positive
    // tags only — negative would invert the meaning).
    let mut spawn_refs: HashMap<Guid, usize> = HashMap::new();
    for s in &samples {
        for guid in &s.positive_tags {
            spawn_refs.entry(*guid).and_modify(|c| *c += 1).or_insert(0);
        }
    }
    let mut entity_carriers: HashMap<Guid, usize> = HashMap::new();
    for ecd in pools.multi_feature.entity_class_definition.iter().flatten() {
        for tag in ecd.tags.iter() {
            *entity_carriers.entry(*tag).or_default() += 1;
        }
    }
    role_tags.sort_by(|a, b| {
        let ar = spawn_refs.get(&a.1).copied().unwrap_or(0);
        let br = spawn_refs.get(&b.1).copied().unwrap_or(0);
        br.cmp(&ar)
    });
    println!(
        "  {:<32} {:>10} {:>10}   path",
        "name", "spawn-refs", "ent-carry"
    );
    for (name, guid, path) in role_tags.iter().take(40) {
        let r = spawn_refs.get(guid).copied().unwrap_or(0);
        let c = entity_carriers.get(guid).copied().unwrap_or(0);
        let path_s = path.join(" ▸ ");
        println!("  {name:<32} {r:>10} {c:>10}   {path_s}");
    }
    println!("  (total role-keyword tags: {})", role_tags.len());
    println!();

    // ── Section 5: initial_damage_settings correlation ───────────────────
    println!("=== Section 5: initial_damage_settings correlation ===");
    // For each variable_name (top by count), report what fraction of its
    // samples have initial_damage_settings populated.
    let mut by_name: HashMap<String, (usize, usize)> = HashMap::new(); // (with_dmg, total)
    for s in &samples {
        let entry = by_name.entry(s.var_name.clone()).or_default();
        entry.1 += 1;
        if s.has_initial_damage {
            entry.0 += 1;
        }
    }
    let mut dmg_entries: Vec<_> = by_name.iter().collect();
    dmg_entries.sort_by(|a, b| b.1.1.cmp(&a.1.1));
    let total_with_damage: usize = samples.iter().filter(|s| s.has_initial_damage).count();
    println!(
        "  ship spawns with initial_damage_settings set: {total_with_damage}/{total_samples} ({:.1}%)",
        100.0 * total_with_damage as f32 / total_samples as f32
    );
    println!();
    println!("  Top variable_names by sample count, with damage-settings fraction:");
    println!("  {:>5} {:>5} {:>5}  {:<48} substring-role", "with", "tot", "%", "name");
    for (name, (with_dmg, total)) in dmg_entries.iter().take(30) {
        let pct = 100.0 * (*with_dmg as f32) / (*total as f32);
        let role = classify_role_substring(name);
        println!("  {with_dmg:>5} {total:>5} {pct:>4.0}%  {name:<48} {role}");
    }
    println!();

    // ── Section 6: AI-role tag presence per variable_name ────────────────
    println!("=== Section 6: typed-tag membership per variable_name (top 20) ===");
    // Build a name → set of role-tag-names-ever-present map.
    let role_tag_guids: HashSet<Guid> = role_tags.iter().map(|(_, g, _)| *g).collect();
    let role_tag_names: HashMap<Guid, String> =
        role_tags.iter().map(|(n, g, _)| (*g, n.clone())).collect();
    let mut per_name: HashMap<String, BTreeMap<String, usize>> = HashMap::new();
    for s in &samples {
        let entry = per_name.entry(s.var_name.clone()).or_default();
        for guid in &s.positive_tags {
            if role_tag_guids.contains(guid) {
                let n = &role_tag_names[guid];
                *entry.entry(n.clone()).or_default() += 1;
            }
        }
    }
    let mut presence_entries: Vec<_> = per_name.iter().collect();
    presence_entries.sort_by(|a, b| {
        let total_a: usize = a.1.values().sum();
        let total_b: usize = b.1.values().sum();
        total_b.cmp(&total_a)
    });
    for (name, tags) in presence_entries.iter().take(20) {
        if tags.is_empty() {
            continue;
        }
        let formatted: Vec<String> = tags.iter().map(|(t, c)| format!("{t}×{c}")).collect();
        println!("  {name:<48} {}", formatted.join(", "));
    }
    let names_with_no_role_tag = per_name.values().filter(|m| m.is_empty()).count();
    println!(
        "  → {} of {} distinct variable_names have ZERO role-keyword tags in any sample",
        names_with_no_role_tag,
        per_name.len()
    );
    println!();

    // ── Section 7: spot checks ───────────────────────────────────────────
    println!("=== Section 7: misc field spot checks ===");
    let with_markup = samples.iter().filter(|s| !s.markup_tags.is_empty()).count();
    let with_entity_tags = samples.iter().filter(|s| !s.entity_tags.is_empty()).count();
    let with_embedded = samples.iter().filter(|s| s.embedded_name_present).count();
    println!(
        "  markup_tags populated:    {with_markup:>5}/{total_samples} ({:.1}%)",
        100.0 * with_markup as f32 / total_samples as f32
    );
    println!(
        "  entity_tags populated:    {with_entity_tags:>5}/{total_samples} ({:.1}%)",
        100.0 * with_entity_tags as f32 / total_samples as f32
    );
    println!(
        "  embedded_name populated:  {with_embedded:>5}/{total_samples} ({:.1}%)",
        100.0 * with_embedded as f32 / total_samples as f32
    );

    // Sample 5 markup_tags entries with their resolved names.
    println!();
    println!("  Sample markup_tags resolutions:");
    let mut shown = 0;
    for s in samples.iter().filter(|s| !s.markup_tags.is_empty()) {
        if shown >= 5 {
            break;
        }
        let names: Vec<String> = s
            .markup_tags
            .iter()
            .filter_map(|g| tag_tree.get(g).map(|n| n.name.clone()))
            .collect();
        println!("    {} → {}", s.var_name, names.join(", "));
        shown += 1;
    }
    println!();
    println!("[done]");
    Ok(())
}
