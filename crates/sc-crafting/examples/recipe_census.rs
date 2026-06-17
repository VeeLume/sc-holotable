//! Crafting domain census against live DCB — pre-design dig for the
//! sc-crafting future-proofing pass (Hearth-blocker resolution).
//!
//! Tallies the actual shape of `CraftingBlueprintRecord` → tiers → recipes
//! → costs/results, plus raw counts of `dormant`-gated types reached only
//! by `db.records_by_type` (non-Creation processes, `CraftingResult_*`,
//! `CraftingRecipe_Ref`/`_RecordRef`, …). Throwaway — read the numbers,
//! decide the API, delete.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example recipe_census
//! ```
#![allow(non_snake_case)]

use std::collections::BTreeMap;

use sc_extract::generated::{
    CraftingBlueprint_Base_NonRefPtr, CraftingBlueprintTier_BasePtr, CraftingCost_BasePtr,
    CraftingProcess_BasePtr, CraftingRecipe_BasePtr, CraftingRecipeCosts_BasePtr,
    CraftingRecipeResults_BasePtr, CraftingResearch_BasePtr, DataPools, RecordIndex,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore};
use sc_items::Items;
use std::collections::HashSet;

#[derive(Default, Debug)]
struct Census {
    blueprint_records: usize,
    bp_no_blueprint: usize,
    bp_unknown_blueprint_variant: usize,

    process_variants: BTreeMap<&'static str, usize>,
    process_unknown_by_type: BTreeMap<String, usize>,

    tier_count_dist: BTreeMap<usize, usize>,
    tiers_total: usize,
    tiers_no_recipe: usize,
    tiers_no_research: usize,

    recipe_variants: BTreeMap<&'static str, usize>,
    recipe_unknown_by_type: BTreeMap<String, usize>,

    inline_recipes_inspected: usize,
    recipes_no_costs: usize,
    recipes_no_results: usize,
    recipes_with_craft_time: usize,

    cost_variants_mandatory_top: BTreeMap<&'static str, usize>,
    cost_select_options_total: usize,
    cost_select_recursive_variants: BTreeMap<&'static str, usize>,

    optional_costs_total: usize,
    optional_with_effect: usize,

    cost_resource_total: usize,
    cost_item_total: usize,
    cost_item_quantity_dist: BTreeMap<i32, usize>,
    cost_item_minq_dist: BTreeMap<i32, usize>,
    cost_resource_minq_dist: BTreeMap<i32, usize>,

    results_variant_top: BTreeMap<&'static str, usize>,
    results_unknown_by_type: BTreeMap<String, usize>,
    inline_recipe_results_seen: usize,
    inline_recipe_results_empty: usize,

    resource_guids: HashSet<sc_extract::Guid>,
    research_seen: usize,
    research_unlock_some: usize,
    research_unlock_variant_by_struct: BTreeMap<String, usize>,
    research_costs_some: usize,
    research_costs_variant_by_struct: BTreeMap<String, usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());

    let records = &datacore.records().records;
    let pools = &datacore.records().pools;

    let mut c = Census::default();
    let bp_records = &records.multi_feature.crafting_blueprint_record;
    c.blueprint_records = bp_records.len();

    for handle in bp_records.values() {
        let Some(bp_record) = handle.get(pools) else {
            c.bp_no_blueprint += 1;
            continue;
        };
        let Some(bp_ptr) = &bp_record.blueprint else {
            c.bp_no_blueprint += 1;
            continue;
        };
        let bp = match bp_ptr {
            CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(h) => h.get(pools),
            _ => {
                c.bp_unknown_blueprint_variant += 1;
                continue;
            }
        };
        let Some(bp) = bp else {
            c.bp_no_blueprint += 1;
            continue;
        };

        // process variant tally
        match &bp.process_specific_data {
            None => *c.process_variants.entry("(none)").or_default() += 1,
            Some(p) => count_process(p, &mut c, &datacore),
        }

        // tier shape
        *c.tier_count_dist.entry(bp.tiers.len()).or_default() += 1;
        c.tiers_total += bp.tiers.len();

        for tier_ptr in &bp.tiers {
            let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = tier_ptr else {
                continue;
            };
            let Some(tier) = th.get(pools) else { continue };

            if tier.research.is_none() {
                c.tiers_no_research += 1;
            } else if let Some(rsh) = &tier.research {
                count_research(rsh, &mut c, pools, &datacore);
            }
            let Some(recipe_ptr) = &tier.recipe else {
                c.tiers_no_recipe += 1;
                continue;
            };
            count_recipe(recipe_ptr, &mut c, records, pools, &datacore);
        }
    }

    // raw counts for dormant types via svarog (no codegen access)
    let db = datacore.db();
    let dormant_probe = [
        "CraftingProcess_Refining",
        "CraftingProcess_Repair",
        "CraftingProcess_Upgrade",
        "CraftingProcess_Dismantle",
        "CraftingProcess_Base_NonRef",
        "CraftingProcessSpecificRecipeData_Refining",
        "CraftingRecipe_Ref",
        "CraftingRecipe_RecordRef",
        "CraftingRecipeCosts_Ref",
        "CraftingRecipeCosts_RecordRef",
        "CraftingRecipeResults_Ref",
        "CraftingRecipeResults_RecordRef",
        "CraftingResult_Item",
        "CraftingResult_Resource",
        "CraftingCost_RecordRef",
        "CraftingCost_Ref",
        "CraftingResearchUnlock",
        "BlueprintCategoryRecord",
    ];
    let mut dormant_counts = BTreeMap::new();
    for t in dormant_probe {
        let n = db.records_by_type(t).count();
        dormant_counts.insert(t, n);
    }

    println!("\n=== blueprint records ===");
    println!("  total                       : {}", c.blueprint_records);
    println!("  no/empty blueprint payload  : {}", c.bp_no_blueprint);
    println!(
        "  unknown CraftingBlueprint_*  : {}",
        c.bp_unknown_blueprint_variant
    );

    println!("\n=== process_specific_data variants ===");
    for (k, v) in &c.process_variants {
        println!("  {:<32} : {}", k, v);
    }
    if !c.process_unknown_by_type.is_empty() {
        println!("  --- Unknown ptr by type (dormant?) ---");
        for (k, v) in &c.process_unknown_by_type {
            println!("    {:<30} : {}", k, v);
        }
    }

    println!("\n=== tier shape ===");
    println!("  total tiers         : {}", c.tiers_total);
    println!("  tiers w/o recipe    : {}", c.tiers_no_recipe);
    println!("  tiers w/o research  : {}", c.tiers_no_research);
    println!("  tier count distribution (n_tiers → blueprints):");
    for (k, v) in &c.tier_count_dist {
        println!("    {:>2} : {}", k, v);
    }

    println!("\n=== recipe variants ===");
    for (k, v) in &c.recipe_variants {
        println!("  {:<32} : {}", k, v);
    }
    if !c.recipe_unknown_by_type.is_empty() {
        println!("  --- Unknown ptr by type ---");
        for (k, v) in &c.recipe_unknown_by_type {
            println!("    {:<30} : {}", k, v);
        }
    }

    println!("\n=== inline CraftingRecipe shape ===");
    println!("  inspected             : {}", c.inline_recipes_inspected);
    println!("  with craftTime        : {}", c.recipes_with_craft_time);
    println!("  no costs ptr          : {}", c.recipes_no_costs);
    println!("  no results ptr        : {}", c.recipes_no_results);

    println!("\n=== mandatory cost top-level variants (inline recipes) ===");
    for (k, v) in &c.cost_variants_mandatory_top {
        println!("  {:<32} : {}", k, v);
    }
    println!(
        "  CraftingCost_Resource total : {} entries",
        c.cost_resource_total
    );
    println!(
        "  CraftingCost_Item     total : {} entries",
        c.cost_item_total
    );
    if !c.cost_select_recursive_variants.is_empty() {
        println!(
            "  Cost_Select inner option variants ({} total options):",
            c.cost_select_options_total
        );
        for (k, v) in &c.cost_select_recursive_variants {
            println!("    {:<30} : {}", k, v);
        }
    }

    println!("\n=== optional costs (inline recipes) ===");
    println!("  total entries           : {}", c.optional_costs_total);
    println!("  with attached effect    : {}", c.optional_with_effect);

    println!("\n=== Cost_Item quantity distribution ===");
    for (q, n) in &c.cost_item_quantity_dist {
        println!("  qty={:<5} : {}", q, n);
    }
    println!("\n=== Cost_Item minQuality distribution ===");
    for (q, n) in &c.cost_item_minq_dist {
        println!("  minQ={:<5}: {}", q, n);
    }
    println!("=== Cost_Resource minQuality distribution ===");
    for (q, n) in &c.cost_resource_minq_dist {
        println!("  minQ={:<5}: {}", q, n);
    }

    println!("\n=== inline recipe results ===");
    println!(
        "  inline result containers seen : {}",
        c.inline_recipe_results_seen
    );
    println!(
        "  with empty results vec        : {}",
        c.inline_recipe_results_empty
    );
    println!("  CraftingResult_* variants seen (typed):");
    for (k, v) in &c.results_variant_top {
        println!("    {:<30} : {}", k, v);
    }
    if !c.results_unknown_by_type.is_empty() {
        println!("  --- Unknown CraftingResult ptr by type (likely dormant) ---");
        for (k, v) in &c.results_unknown_by_type {
            println!("    {:<30} : {}", k, v);
        }
    }

    println!("\n=== raw db counts for dormant-gated types ===");
    println!("(reachable via db.records_by_type, NOT via typed enum dispatch under `crafting`)");
    for (t, n) in &dormant_counts {
        println!("  {:<44} : {}", t, n);
    }

    println!("\n=== research (probe the unlock_requirements / research_costs payloads) ===");
    println!("  tiers with research present : {}", c.research_seen);
    println!("  research with unlock_req    : {}", c.research_unlock_some);
    println!("  research with research_costs: {}", c.research_costs_some);
    if !c.research_unlock_variant_by_struct.is_empty() {
        println!("  unlock_requirements actual struct types:");
        for (k, v) in &c.research_unlock_variant_by_struct {
            println!("    {:<40} : {}", k, v);
        }
    }
    if !c.research_costs_variant_by_struct.is_empty() {
        println!("  research_costs actual struct types:");
        for (k, v) in &c.research_costs_variant_by_struct {
            println!("    {:<40} : {}", k, v);
        }
    }

    println!("\n=== resource GUID probe ===");
    println!(
        "  unique resource GUIDs in costs : {}",
        c.resource_guids.len()
    );
    let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
    let mut unresolved = 0usize;
    let mut samples: Vec<(sc_extract::Guid, String, Option<String>)> = Vec::new();
    for (i, g) in c.resource_guids.iter().enumerate() {
        match db.record(g) {
            Some(rec) => {
                let tname = rec.type_name().unwrap_or("?").to_string();
                let path = rec.file_name().map(|s| s.to_string());
                *by_type.entry(tname.clone()).or_default() += 1;
                if i < 5 {
                    samples.push((*g, tname, path));
                }
            }
            None => unresolved += 1,
        }
    }
    println!("  unresolved (GUID not in db)    : {}", unresolved);
    println!("  resource record-type distribution:");
    for (t, n) in &by_type {
        println!("    {:<44} : {}", t, n);
    }
    println!("  samples:");
    for (g, t, p) in &samples {
        println!(
            "    {} → type={:<28} path={}",
            g,
            t,
            p.as_deref().unwrap_or("?")
        );
    }

    // sample: pick three blueprints — P4-AR if found, a Select-cost one, one with research
    println!("\n=== sample blueprints (pretty-printed) ===");
    let mut samples_emitted = 0usize;
    let mut p4ar_seen = false;
    let mut select_seen = false;
    let mut research_seen = false;
    for (guid, handle) in bp_records.iter() {
        if samples_emitted >= 3 {
            break;
        }
        let Some(bp_record) = handle.get(pools) else {
            continue;
        };
        let CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(bh) =
            bp_record.blueprint.as_ref().unwrap()
        else {
            continue;
        };
        let Some(bp) = bh.get(pools) else { continue };

        let nm = items
            .name_key(guid)
            .and_then(|k| asset_data.locale.resolve(k))
            .or_else(|| asset_data.locale.resolve(&bp.blueprint_name))
            .unwrap_or("?");

        let is_p4ar = nm.contains("P4-AR");
        let has_select = recipe_has_select(bp, pools);
        let has_research = bp.tiers.iter().any(|t| {
            if let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = t
                && let Some(tier) = th.get(pools)
            {
                return tier.research.is_some();
            }
            false
        });

        let pick = (is_p4ar && !p4ar_seen)
            || (has_select && !select_seen)
            || (has_research && !research_seen);
        if !pick {
            continue;
        }
        p4ar_seen |= is_p4ar;
        select_seen |= has_select;
        research_seen |= has_research;
        samples_emitted += 1;

        println!("\n--- {} ({}) ---", nm, guid);
        dump_blueprint(bp, pools, &items, &asset_data.locale);
    }
    Ok(())
}

fn count_process(p: &CraftingProcess_BasePtr, c: &mut Census, datacore: &Datacore) {
    let label = match p {
        CraftingProcess_BasePtr::CraftingProcess_Base(_) => "CraftingProcess_Base",
        CraftingProcess_BasePtr::CraftingProcess_Creation(_) => "CraftingProcess_Creation",
        CraftingProcess_BasePtr::Unknown { struct_index, .. } => {
            let name = datacore
                .db()
                .struct_name(*struct_index as usize)
                .map(|n| n.to_string())
                .unwrap_or_else(|| format!("struct#{}", struct_index));
            *c.process_unknown_by_type.entry(name).or_default() += 1;
            "Unknown"
        }
    };
    *c.process_variants.entry(label).or_default() += 1;
}

fn count_recipe(
    ptr: &CraftingRecipe_BasePtr,
    c: &mut Census,
    records: &RecordIndex,
    pools: &DataPools,
    datacore: &Datacore,
) {
    let label = match ptr {
        CraftingRecipe_BasePtr::CraftingRecipe(_) => "CraftingRecipe (inline)",
        CraftingRecipe_BasePtr::CraftingRecipe_Base(_) => "CraftingRecipe_Base",
        CraftingRecipe_BasePtr::CraftingRecipe_Base_NonRef(_) => "CraftingRecipe_Base_NonRef",
        CraftingRecipe_BasePtr::Unknown { struct_index, .. } => {
            let name = datacore
                .db()
                .struct_name(*struct_index as usize)
                .map(|n| n.to_string())
                .unwrap_or_else(|| format!("struct#{}", struct_index));
            *c.recipe_unknown_by_type.entry(name).or_default() += 1;
            "Unknown"
        }
    };
    *c.recipe_variants.entry(label).or_default() += 1;

    if let CraftingRecipe_BasePtr::CraftingRecipe(h) = ptr
        && let Some(recipe) = h.get(pools)
    {
        c.inline_recipes_inspected += 1;
        if recipe.costs.is_none() {
            c.recipes_no_costs += 1;
        }
        if recipe.results.is_none() {
            c.recipes_no_results += 1;
        }
        if let Some(costs_ptr) = &recipe.costs {
            count_costs(costs_ptr, c, pools);
        }
        if let Some(results_ptr) = &recipe.results {
            count_results(results_ptr, c, pools, datacore);
        }
        let _ = records; // suppress unused if extended later
    }
}

fn count_costs(ptr: &CraftingRecipeCosts_BasePtr, c: &mut Census, pools: &DataPools) {
    let CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(h) = ptr else {
        return;
    };
    let Some(costs) = h.get(pools) else { return };
    if costs.craft_time.is_some() {
        c.recipes_with_craft_time += 1;
    }
    if let Some(mandatory) = &costs.mandatory_cost {
        let label = cost_label(mandatory);
        *c.cost_variants_mandatory_top.entry(label).or_default() += 1;
        tally_cost(mandatory, c, pools, /*recursive_from_select=*/ false);
    }
    c.optional_costs_total += costs.optional_costs.len();
    for opt_h in &costs.optional_costs {
        if let Some(opt) = opt_h.get(pools) {
            if opt.effect.is_some() {
                c.optional_with_effect += 1;
            }
            if let Some(oc) = &opt.optional_cost {
                tally_cost(oc, c, pools, false);
            }
        }
    }
}

fn cost_label(ptr: &CraftingCost_BasePtr) -> &'static str {
    match ptr {
        CraftingCost_BasePtr::CraftingCost_Resource(_) => "CraftingCost_Resource",
        CraftingCost_BasePtr::CraftingCost_Item(_) => "CraftingCost_Item",
        CraftingCost_BasePtr::CraftingCost_Select(_) => "CraftingCost_Select",
        CraftingCost_BasePtr::CraftingCost_Base(_) => "CraftingCost_Base",
        CraftingCost_BasePtr::Unknown { .. } => "Unknown(dormant/other)",
    }
}

fn tally_cost(ptr: &CraftingCost_BasePtr, c: &mut Census, pools: &DataPools, recursive: bool) {
    match ptr {
        CraftingCost_BasePtr::CraftingCost_Resource(h) => {
            c.cost_resource_total += 1;
            if let Some(r) = h.get(pools) {
                *c.cost_resource_minq_dist.entry(r.min_quality).or_default() += 1;
                if let Some(g) = r.resource {
                    c.resource_guids.insert(g);
                }
            }
        }
        CraftingCost_BasePtr::CraftingCost_Item(h) => {
            c.cost_item_total += 1;
            if let Some(i) = h.get(pools) {
                *c.cost_item_quantity_dist.entry(i.quantity).or_default() += 1;
                *c.cost_item_minq_dist.entry(i.min_quality).or_default() += 1;
            }
        }
        CraftingCost_BasePtr::CraftingCost_Select(h) => {
            if let Some(sel) = h.get(pools) {
                if !recursive {
                    c.cost_select_options_total += sel.options.len();
                }
                for opt in &sel.options {
                    if !recursive {
                        let label = cost_label(opt);
                        *c.cost_select_recursive_variants.entry(label).or_default() += 1;
                    }
                    tally_cost(opt, c, pools, true);
                }
            }
        }
        _ => {}
    }
}

fn count_results(
    ptr: &CraftingRecipeResults_BasePtr,
    c: &mut Census,
    pools: &DataPools,
    datacore: &Datacore,
) {
    let CraftingRecipeResults_BasePtr::CraftingRecipeResults(h) = ptr else {
        return;
    };
    let Some(rr) = h.get(pools) else { return };
    c.inline_recipe_results_seen += 1;
    if rr.results.is_empty() {
        c.inline_recipe_results_empty += 1;
    }
    for r in &rr.results {
        use sc_extract::generated::CraftingResult_BasePtr as R;
        let label = match r {
            R::CraftingResult_Base(_) => "CraftingResult_Base",
            R::Unknown { struct_index, .. } => {
                let name = datacore
                    .db()
                    .struct_name(*struct_index as usize)
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("struct#{}", struct_index));
                *c.results_unknown_by_type.entry(name).or_default() += 1;
                "Unknown"
            }
        };
        *c.results_variant_top.entry(label).or_default() += 1;
    }
}

fn count_research(
    ptr: &CraftingResearch_BasePtr,
    c: &mut Census,
    pools: &DataPools,
    datacore: &Datacore,
) {
    c.research_seen += 1;
    let CraftingResearch_BasePtr::CraftingResearch(h) = ptr else {
        return;
    };
    let Some(research) = h.get(pools) else { return };
    if let Some(unlock) = &research.unlock_requirements {
        c.research_unlock_some += 1;
        use sc_extract::generated::CraftingResearchUnlock_BasePtr as U;
        let label = match unlock {
            U::CraftingResearchUnlock_Base(_) => "CraftingResearchUnlock_Base".to_string(),
            U::Unknown { struct_index, .. } => datacore
                .db()
                .struct_name(*struct_index as usize)
                .map(|n| format!("Unknown({})", n))
                .unwrap_or_else(|| format!("Unknown(struct#{})", struct_index)),
        };
        *c.research_unlock_variant_by_struct
            .entry(label)
            .or_default() += 1;
    }
    if let Some(costs) = &research.research_costs {
        c.research_costs_some += 1;
        let label = match costs {
            CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(_) => {
                "CraftingRecipeCosts (inline)".to_string()
            }
            CraftingRecipeCosts_BasePtr::CraftingRecipeCosts_Base(_) => {
                "CraftingRecipeCosts_Base".to_string()
            }
            CraftingRecipeCosts_BasePtr::CraftingRecipeCosts_Base_NonRef(_) => {
                "CraftingRecipeCosts_Base_NonRef".to_string()
            }
            CraftingRecipeCosts_BasePtr::Unknown { struct_index, .. } => datacore
                .db()
                .struct_name(*struct_index as usize)
                .map(|n| format!("Unknown({})", n))
                .unwrap_or_else(|| format!("Unknown(struct#{})", struct_index)),
        };
        *c.research_costs_variant_by_struct.entry(label).or_default() += 1;
    }
}

fn recipe_has_select(bp: &sc_extract::generated::CraftingBlueprint, pools: &DataPools) -> bool {
    for t in &bp.tiers {
        let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = t else {
            continue;
        };
        let Some(tier) = th.get(pools) else { continue };
        let Some(CraftingRecipe_BasePtr::CraftingRecipe(rh)) = &tier.recipe else {
            continue;
        };
        let Some(r) = rh.get(pools) else { continue };
        let Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) = &r.costs else {
            continue;
        };
        let Some(costs) = ch.get(pools) else { continue };
        if matches!(
            &costs.mandatory_cost,
            Some(CraftingCost_BasePtr::CraftingCost_Select(_))
        ) {
            return true;
        }
    }
    false
}

fn dump_blueprint(
    bp: &sc_extract::generated::CraftingBlueprint,
    pools: &DataPools,
    items: &Items,
    locale: &sc_extract::LocaleMap,
) {
    println!("  tiers: {}", bp.tiers.len());
    if let Some(p) = &bp.process_specific_data {
        match p {
            CraftingProcess_BasePtr::CraftingProcess_Creation(h) => {
                let entity = h.get(pools).and_then(|c| c.entity_class);
                let name = entity
                    .and_then(|g| items.name_key(&g))
                    .and_then(|k| locale.resolve(k));
                println!(
                    "  process: Creation → entity_class={:?} ({:?})",
                    entity, name
                );
            }
            other => println!("  process: {:?}", std::mem::discriminant(other)),
        }
    }
    for (i, t) in bp.tiers.iter().enumerate() {
        let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = t else {
            continue;
        };
        let Some(tier) = th.get(pools) else { continue };
        println!("  tier[{}]:", i);
        if let Some(CraftingRecipe_BasePtr::CraftingRecipe(rh)) = &tier.recipe
            && let Some(recipe) = rh.get(pools)
        {
            if let Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) = &recipe.costs
                && let Some(costs) = ch.get(pools)
            {
                if let Some(mc) = &costs.mandatory_cost {
                    print!("    mandatory: ");
                    dump_cost(mc, pools, items, locale, 0);
                }
                if !costs.optional_costs.is_empty() {
                    println!("    optional: {} entries", costs.optional_costs.len());
                }
            }
            match &recipe.results {
                Some(CraftingRecipeResults_BasePtr::CraftingRecipeResults(rh)) => {
                    if let Some(rs) = rh.get(pools) {
                        println!(
                            "    results: {} entries (typed: see census, dormant=Unknown)",
                            rs.results.len()
                        );
                    }
                }
                Some(other) => {
                    let name = match other {
                        CraftingRecipeResults_BasePtr::Unknown { struct_index, .. } => {
                            format!("Unknown(struct#{})", struct_index)
                        }
                        _ => format!("{:?}", std::mem::discriminant(other)),
                    };
                    println!("    results: ptr variant {}", name);
                }
                None => println!("    results: (none)"),
            }
        }
        if tier.research.is_some() {
            println!("    research: present");
        }
    }
}

fn dump_cost(
    ptr: &CraftingCost_BasePtr,
    pools: &DataPools,
    items: &Items,
    locale: &sc_extract::LocaleMap,
    depth: usize,
) {
    let pad = "  ".repeat(depth);
    match ptr {
        CraftingCost_BasePtr::CraftingCost_Item(h) => {
            if let Some(i) = h.get(pools) {
                let nm = i
                    .entity_class
                    .and_then(|g| items.name_key(&g))
                    .and_then(|k| locale.resolve(k))
                    .unwrap_or("?");
                println!(
                    "{}Item   qty={} minQ={} → {} ({:?})",
                    pad, i.quantity, i.min_quality, nm, i.entity_class
                );
            }
        }
        CraftingCost_BasePtr::CraftingCost_Resource(h) => {
            if let Some(r) = h.get(pools) {
                println!(
                    "{}Res    minQ={} resource={:?}",
                    pad, r.min_quality, r.resource
                );
            }
        }
        CraftingCost_BasePtr::CraftingCost_Select(h) => {
            if let Some(sel) = h.get(pools) {
                println!(
                    "{}Select count={} options={}",
                    pad,
                    sel.count,
                    sel.options.len()
                );
                for o in &sel.options {
                    print!("{}  → ", pad);
                    dump_cost(o, pools, items, locale, depth + 1);
                }
            }
        }
        other => println!("{}{:?}", pad, std::mem::discriminant(other)),
    }
}
