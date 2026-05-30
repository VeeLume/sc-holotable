//! Open-question probes for the sc-crafting future-proofing pass.
//! Resolves the design questions captured in `docs/sc-crafting.md`:
//!
//! 1. `TimeValue` polymorphic variants — what shapes craft_time in?
//! 2. `SBaseCargoUnit` — empty marker in generated, but only variant in
//!    its poly enum. Check the raw schema for hidden subtypes.
//! 3. `ResourceTypeProperties` polymorphic variants — what does
//!    `ResourceType.properties` actually carry?
//! 4. `ResourceTypeDensityType` — same shape check as SBaseCargoUnit.
//! 5. `DefaultBlueprintSelection_Whitelist` — what does the whitelist
//!    list, and how big is it?
//! 6. `CraftingDisplayTransformation_*` polymorphic variants on
//!    `CraftingGameplayPropertyDef.display_transformation`.
//! 7. `CraftingPropertyNameOverride` (the `name_overrides` Class array)
//!    — does anything populate it, and what fields?
//! 8. Three quality types — `Distribution`, `LocationOverride`,
//!    `Quantization` — walk each record type to its concrete leaf
//!    (Uniform/Normal/etc.) and sample values.
//! 9. The big one: confirm `ResourceType.properties` carries
//!    `ResourceTypeCraftingData` per resource (which transitively
//!    encodes per-resource quality distribution + location override +
//!    quantization).
//!
//! ```bash
//! cargo run -p sc-crafting --release --example open_questions
//! ```
#![allow(non_snake_case)]

use std::collections::BTreeMap;

use sc_extract::generated::{
    CraftingDisplayTransformation_BasePtr, CraftingQualityDistributionNormal,
    CraftingQualityDistribution_Base_NonRefPtr,
    CraftingQualityLocationOverride_Base_NonRefPtr,
    CraftingQualityQuantization_Base_NonRefPtr, CraftingRecipeCosts_BasePtr,
    CraftingRecipe_BasePtr, DataPools, DefaultBlueprintSelection_BasePtr, ResourceType,
    ResourceTypePropertiesPtr, TimeValue_BasePtr,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, RecordPaths};
use sc_resources::Resources;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let _resources = Resources::build(datacore.records());
    let paths = RecordPaths::build(&datacore);
    let pools = &datacore.records().pools;
    let records = &datacore.records().records;
    let db = datacore.db();

    // ------- 1. TimeValue variants -------
    println!("\n=== Q1: TimeValue variants on craft_time ===");
    let mut tv_variants: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut tv_unknown_struct: BTreeMap<String, usize> = BTreeMap::new();
    let mut sample_partitioned = None;
    let mut sample_long_seconds = None;
    let mut craft_time_present = 0usize;
    let mut craft_time_none = 0usize;
    for handle in records.multi_feature.crafting_blueprint_record.values() {
        let Some(bp_rec) = handle.get(pools) else { continue };
        let Some(bp_ptr) = &bp_rec.blueprint else { continue };
        use sc_extract::generated::CraftingBlueprint_Base_NonRefPtr;
        let CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(bh) = bp_ptr else { continue };
        let Some(bp) = bh.get(pools) else { continue };
        for tier_ptr in &bp.tiers {
            use sc_extract::generated::CraftingBlueprintTier_BasePtr as T;
            let T::CraftingBlueprintTier(th) = tier_ptr else { continue };
            let Some(tier) = th.get(pools) else { continue };
            let Some(CraftingRecipe_BasePtr::CraftingRecipe(rh)) = &tier.recipe else { continue };
            let Some(recipe) = rh.get(pools) else { continue };
            let Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) = &recipe.costs else {
                continue;
            };
            let Some(costs) = ch.get(pools) else { continue };
            match &costs.craft_time {
                None => craft_time_none += 1,
                Some(tv) => {
                    craft_time_present += 1;
                    match tv {
                        TimeValue_BasePtr::TimeValue_Base(_) => {
                            *tv_variants.entry("TimeValue_Base (empty)").or_default() += 1;
                        }
                        TimeValue_BasePtr::Unknown { struct_index, .. } => {
                            let n = db.struct_name(*struct_index as usize).unwrap_or("?").to_string();
                            *tv_unknown_struct.entry(n).or_default() += 1;
                            *tv_variants.entry("Unknown").or_default() += 1;
                        }
                        // The remaining variants (LongSeconds, Partitioned) are cfg-gated:
                        // LongSeconds is dormant; Partitioned is multi_feature behind crafting.
                        #[allow(unreachable_patterns)]
                        _ => {
                            *tv_variants.entry("(typed variant, see direct match)").or_default() += 1;
                        }
                    }
                    // Direct concrete-typed inspection for Partitioned (reachable under crafting).
                    if let TimeValue_BasePtr::Unknown { struct_index, instance_index } = tv {
                        let name = db.struct_name(*struct_index as usize).unwrap_or("?");
                        if name == "TimeValue_Partitioned" && sample_partitioned.is_none() {
                            // Read raw bytes via the typed pool — TimeValue_Partitioned is multi_feature/crafting-gated.
                            // We can't typed-deref from Unknown; sample via raw instance.
                            let inst = db.instance(*struct_index, *instance_index);
                            sample_partitioned = Some(format!("{inst:?}"));
                        } else if name == "TimeValue_LongSeconds" && sample_long_seconds.is_none() {
                            let inst = db.instance(*struct_index, *instance_index);
                            sample_long_seconds = Some(format!("{inst:?}"));
                        }
                    }
                }
            }
        }
    }
    // Try via typed pool: read first 3 TimeValue_Partitioned values directly (multi_feature, reachable).
    let parts = &pools.multi_feature.time_value_partitioned;
    let part_count = parts.iter().filter(|o| o.is_some()).count();
    println!("  craft_time present : {craft_time_present} / none {craft_time_none}");
    println!("  enum-dispatch variant tally:");
    for (k, v) in &tv_variants {
        println!("    {k:<40} {v}");
    }
    if !tv_unknown_struct.is_empty() {
        println!("  Unknown-by-struct-name:");
        for (k, v) in &tv_unknown_struct {
            println!("    {k:<40} {v}");
        }
    }
    println!("  TimeValue_Partitioned pool population: {part_count}");
    println!("  TimeValue_LongSeconds pool population: (dormant — not reachable typed)");
    if let Some(sample) = parts.iter().flatten().take(3).next() {
        println!(
            "  sample Partitioned: days={} hours={} minutes={} seconds={}",
            sample.days, sample.hours, sample.minutes, sample.seconds
        );
    }

    // ------- 2. SBaseCargoUnit schema check -------
    println!("\n=== Q2: SBaseCargoUnit (empty in generated, only variant in poly enum) ===");
    schema_dump(db, "SBaseCargoUnit");
    let n_su = db.records_by_type("SBaseCargoUnit").count();
    println!("  records of SBaseCargoUnit (none expected — it's a polymorphic base): {n_su}");
    // Check for subtype struct names with the same prefix.
    let related: Vec<String> = db
        .struct_definitions()
        .iter()
        .enumerate()
        .filter_map(|(i, _)| db.struct_name(i))
        .filter(|n| n.starts_with("SBaseCargoUnit") || n.contains("CargoUnit"))
        .map(|s| s.to_string())
        .collect();
    println!("  related struct names: {related:?}");

    // ------- 3. ResourceTypeProperties variants -------
    println!("\n=== Q3: ResourceType.properties variant distribution ===");
    let mut prop_variants: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut sample_crafting_data: Option<Guid> = None;
    let mut sample_volatility: Option<Guid> = None;
    let mut sample_properties_marker: Option<Guid> = None;
    let mut total_property_ptrs = 0usize;
    for (&guid, &handle) in &records.multi_feature.resource_type {
        let Some(rt) = handle.get(pools) else { continue };
        for p in &rt.properties {
            total_property_ptrs += 1;
            match p {
                ResourceTypePropertiesPtr::ResourceTypeProperties(_) => {
                    *prop_variants.entry("ResourceTypeProperties (marker name only)").or_default() += 1;
                    sample_properties_marker.get_or_insert(guid);
                }
                ResourceTypePropertiesPtr::ResourceTypeCraftingData(_) => {
                    *prop_variants.entry("ResourceTypeCraftingData").or_default() += 1;
                    sample_crafting_data.get_or_insert(guid);
                }
                ResourceTypePropertiesPtr::ResourceTypeVolatility(_) => {
                    *prop_variants.entry("ResourceTypeVolatility").or_default() += 1;
                    sample_volatility.get_or_insert(guid);
                }
                ResourceTypePropertiesPtr::Unknown { struct_index, .. } => {
                    let n = db.struct_name(*struct_index as usize).unwrap_or("?");
                    println!("  Unknown property variant: {n}");
                }
            }
        }
    }
    println!("  total property ptrs across all ResourceTypes: {total_property_ptrs}");
    for (k, v) in &prop_variants {
        println!("    {k:<46} {v}");
    }
    if let Some(g) = sample_crafting_data {
        println!("  → sample resource with CraftingData: {g}");
        if let Some(rt) = lookup_resource(records, pools, g) {
            for p in &rt.properties {
                if let ResourceTypePropertiesPtr::ResourceTypeCraftingData(h) = p
                    && let Some(cd) = h.get(pools)
                {
                    println!(
                        "    CraftingData {{ name={:?}, quality_distribution={}, quality_location_override={}, quality_quantization={} }}",
                        cd.name,
                        cd.quality_distribution.is_some(),
                        cd.quality_location_override.is_some(),
                        cd.quality_quantization.is_some(),
                    );
                }
            }
        }
    }
    if let Some(g) = sample_volatility {
        println!("  → sample resource with Volatility: {g}");
        if let Some(rt) = lookup_resource(records, pools, g) {
            for p in &rt.properties {
                if let ResourceTypePropertiesPtr::ResourceTypeVolatility(h) = p
                    && let Some(v) = h.get(pools)
                {
                    println!(
                        "    Volatility {{ name={:?}, volatility={}, health_decay_per_second={} }}",
                        v.name, v.volatility, v.health_decay_per_second
                    );
                }
            }
        }
    }

    // ------- 4. ResourceTypeDensityType schema check -------
    println!("\n=== Q4: ResourceTypeDensityType (empty in generated) ===");
    schema_dump(db, "ResourceTypeDensityType");
    // Tally how many ResourceTypes set density_type at all.
    let mut density_some = 0usize;
    let mut density_none = 0usize;
    let mut density_unknown_struct: BTreeMap<String, usize> = BTreeMap::new();
    for handle in records.multi_feature.resource_type.values() {
        let Some(rt) = handle.get(pools) else { continue };
        match &rt.density_type {
            Some(d) => {
                density_some += 1;
                use sc_extract::generated::ResourceTypeDensityTypePtr as D;
                let label = match d {
                    D::ResourceTypeDensityType(_) => "ResourceTypeDensityType (Base)".to_string(),
                    D::ResourceTypeDensity(_) => "ResourceTypeDensity (concrete)".to_string(),
                    D::Unknown { struct_index, .. } => {
                        let n = db.struct_name(*struct_index as usize).unwrap_or("?");
                        format!("Unknown({n})")
                    }
                };
                *density_unknown_struct.entry(label).or_default() += 1;
            }
            None => density_none += 1,
        }
    }
    println!("  density_type present : {density_some} / none {density_none}");
    println!("  variant distribution :");
    for (k, v) in &density_unknown_struct {
        println!("    {k:<40} {v}");
    }

    // ------- 5. DefaultBlueprintSelection_Whitelist -------
    println!("\n=== Q5: DefaultBlueprintSelection_Whitelist contents ===");
    // GlobalParams.default_blueprint_selection → poly ptr → Whitelist
    let gp_pool = &pools.crafting.crafting_global_params;
    let Some(gp) = gp_pool.iter().flatten().next() else {
        println!("  (no CraftingGlobalParams record found)");
        return Ok(());
    };
    println!(
        "  GlobalParams: refining_multiplier={}, default_composition_quality={}, dismantle_blacklist_resources_count={}, dismantle_blacklist_entity_classes_count={}",
        gp.refining_quality_unit_multiplier,
        gp.default_composition_quality,
        gp.dismantle_blacklist_resources.len(),
        gp.dismantle_blacklist_entity_classes.len(),
    );
    match &gp.default_blueprint_selection {
        None => println!("  default_blueprint_selection: None"),
        Some(DefaultBlueprintSelection_BasePtr::DefaultBlueprintSelection_Whitelist(h)) => {
            if let Some(wl) = h.get(pools) {
                println!(
                    "  Whitelist: blueprint_records.len() = {}  (a list of blueprint GUIDs to default-unlock?)",
                    wl.blueprint_records.len()
                );
                for g in wl.blueprint_records.iter().take(5) {
                    let path = paths.get(g).map(|rp| rp.path.as_str()).unwrap_or("?");
                    println!("    {g} → {path}");
                }
            }
        }
        Some(other) => println!("  default_blueprint_selection: other variant {other:?}"),
    }

    // ------- 6. CraftingDisplayTransformation_BasePtr variants -------
    println!("\n=== Q6: CraftingDisplayTransformation variants ===");
    let mut display_variants: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut display_unknown_struct: BTreeMap<String, usize> = BTreeMap::new();
    let mut display_none = 0usize;
    for handle in records.multi_feature.crafting_gameplay_property_def.values() {
        let Some(d) = handle.get(pools) else { continue };
        match &d.display_transformation {
            None => display_none += 1,
            Some(dt) => {
                use CraftingDisplayTransformation_BasePtr as D;
                let label: &'static str = match dt {
                    D::CraftingDisplayTransformation_Base(_) => "Base",
                    D::CraftingDisplayTransformation_Scale(_) => "Scale",
                    D::CraftingDisplayTransformation_ConvertFactorToPercentChange(_) =>
                        "ConvertFactorToPercentChange",
                    D::CraftingDisplayTransformation_ConvertFactorToNegatedPercentChange(_) =>
                        "ConvertFactorToNegatedPercentChange",
                    D::CraftingDisplayTransformation_ConvertValueToFactorOfBaseValue(_) =>
                        "ConvertValueToFactorOfBaseValue",
                    D::CraftingDisplayTransformation_Sequence(_) => "Sequence",
                    D::Unknown { struct_index, .. } => {
                        let n = db.struct_name(*struct_index as usize).unwrap_or("?");
                        *display_unknown_struct.entry(n.to_string()).or_default() += 1;
                        "Unknown"
                    }
                };
                *display_variants.entry(label).or_default() += 1;
            }
        }
    }
    println!("  display_transformation present : {} / none {display_none}",
        display_variants.values().sum::<usize>());
    for (k, v) in &display_variants {
        println!("    {k:<46} {v}");
    }
    if !display_unknown_struct.is_empty() {
        println!("  Unknown-by-struct-name:");
        for (k, v) in &display_unknown_struct {
            println!("    {k:<40} {v}");
        }
    }

    // ------- 7. CraftingPropertyNameOverride (the inline Class array) -------
    println!("\n=== Q7: CraftingPropertyNameOverride population ===");
    schema_dump(db, "CraftingPropertyNameOverride");
    let mut total_overrides = 0usize;
    let mut props_with_overrides = 0usize;
    for handle in records.multi_feature.crafting_gameplay_property_def.values() {
        let Some(d) = handle.get(pools) else { continue };
        total_overrides += d.name_overrides.len();
        if !d.name_overrides.is_empty() {
            props_with_overrides += 1;
        }
    }
    println!(
        "  GameplayPropertyDefs with non-empty name_overrides : {props_with_overrides}; total overrides: {total_overrides}"
    );

    // ------- 8. Three Quality types — concrete-leaf shapes -------
    println!("\n=== Q8: Quality{{Distribution,LocationOverride,Quantization}} concrete leaves ===");

    println!("  --- Distribution ---");
    let dist_pool = &records.multi_feature.crafting_quality_distribution_record;
    println!("  Records: {}", dist_pool.len());
    let mut dist_leaves: BTreeMap<&'static str, usize> = BTreeMap::new();
    for handle in dist_pool.values() {
        let Some(rec) = handle.get(pools) else { continue };
        let Some(d) = &rec.quality_distribution else {
            *dist_leaves.entry("(empty distribution)").or_default() += 1;
            continue;
        };
        let label = label_dist_nonref(d);
        *dist_leaves.entry(label).or_default() += 1;
    }
    let n_uniform_dormant = db.records_by_type("CraftingQualityDistributionUniform").count();
    let sample_normal = pools
        .multi_feature
        .crafting_quality_distribution_normal
        .iter()
        .flatten()
        .next()
        .map(|n: &CraftingQualityDistributionNormal| {
            format!(
                "Normal {{ min={}, max={}, mean={}, stddev={} }}",
                n.min, n.max, n.mean, n.stddev
            )
        });
    for (k, v) in &dist_leaves {
        println!("    {k:<46} {v}");
    }
    println!("    CraftingQualityDistributionUniform records in raw db (dormant — typed unreachable): {n_uniform_dormant}");
    if let Some(s) = &sample_normal  { println!("    sample {s}"); }

    println!("  --- LocationOverride ---");
    let loc_pool = &records.multi_feature.crafting_quality_location_override_record;
    println!("  Records: {}", loc_pool.len());
    let mut loc_leaves: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut total_entries = 0usize;
    let mut sample_loc_entry: Option<String> = None;
    for handle in loc_pool.values() {
        let Some(rec) = handle.get(pools) else { continue };
        let Some(o) = &rec.location_override else {
            *loc_leaves.entry("(empty location_override)").or_default() += 1;
            continue;
        };
        let label = label_loc_nonref(o);
        *loc_leaves.entry(label).or_default() += 1;
        if let CraftingQualityLocationOverride_Base_NonRefPtr::CraftingQualityLocationOverride(h) = o
            && let Some(co) = h.get(pools)
        {
            for entry_h in &co.location_override_list {
                if let Some(entry) = entry_h.get(pools) {
                    total_entries += 1;
                    if sample_loc_entry.is_none() {
                        sample_loc_entry = Some(format!(
                            "entry {{ location={:?}, has_distribution={} }}",
                            entry.location,
                            entry.quality_distribution.is_some()
                        ));
                    }
                }
            }
        }
    }
    for (k, v) in &loc_leaves {
        println!("    {k:<46} {v}");
    }
    println!("    total override entries: {total_entries}");
    if let Some(s) = sample_loc_entry { println!("    {s}"); }

    println!("  --- Quantization ---");
    let qz_pool = &records.multi_feature.crafting_quality_quantization_record;
    println!("  Records: {}", qz_pool.len());
    let mut qz_leaves: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut total_bands = 0usize;
    let mut sample_band: Option<String> = None;
    for handle in qz_pool.values() {
        let Some(rec) = handle.get(pools) else { continue };
        let Some(q) = &rec.quality_quantization else {
            *qz_leaves.entry("(empty quantization)").or_default() += 1;
            continue;
        };
        let label = label_qz_nonref(q);
        *qz_leaves.entry(label).or_default() += 1;
        if let CraftingQualityQuantization_Base_NonRefPtr::CraftingQualityQuantization(h) = q
            && let Some(qq) = h.get(pools)
        {
            for band_h in &qq.bands {
                if let Some(band) = band_h.get(pools) {
                    total_bands += 1;
                    if sample_band.is_none() {
                        sample_band = Some(format!(
                            "Band {{ start={}, end={}, mapped_value={} }}",
                            band.start, band.end, band.mapped_value
                        ));
                    }
                }
            }
        }
    }
    for (k, v) in &qz_leaves {
        println!("    {k:<46} {v}");
    }
    println!("    total bands: {total_bands}");
    if let Some(s) = sample_band { println!("    sample {s}"); }

    // ------- 9. Confirm per-resource crafting data wiring -------
    println!("\n=== Q9: per-resource CraftingData wiring ===");
    let mut resources_with_crafting_data = 0usize;
    let mut resources_with_quality_dist = 0usize;
    let mut resources_with_location_override = 0usize;
    let mut resources_with_quantization = 0usize;
    for handle in records.multi_feature.resource_type.values() {
        let Some(rt) = handle.get(pools) else { continue };
        let mut has_cd = false;
        for p in &rt.properties {
            if let ResourceTypePropertiesPtr::ResourceTypeCraftingData(h) = p
                && let Some(cd) = h.get(pools)
            {
                has_cd = true;
                if cd.quality_distribution.is_some() {
                    resources_with_quality_dist += 1;
                }
                if cd.quality_location_override.is_some() {
                    resources_with_location_override += 1;
                }
                if cd.quality_quantization.is_some() {
                    resources_with_quantization += 1;
                }
            }
        }
        if has_cd {
            resources_with_crafting_data += 1;
        }
    }
    println!(
        "  resources with ResourceTypeCraftingData : {resources_with_crafting_data}\n  ... with quality_distribution      : {resources_with_quality_dist}\n  ... with quality_location_override : {resources_with_location_override}\n  ... with quality_quantization      : {resources_with_quantization}"
    );

    Ok(())
}

fn schema_dump(db: &svarog_datacore::DataCoreDatabase, type_name: &str) {
    let mut idx = None;
    for (i, _) in db.struct_definitions().iter().enumerate() {
        if db.struct_name(i) == Some(type_name) {
            idx = Some(i);
            break;
        }
    }
    let Some(i) = idx else {
        println!("  schema: {type_name} not found");
        return;
    };
    let props = db.get_struct_properties(i);
    println!("  schema {type_name}: {} properties", props.len());
    for p in &props {
        let off = { p.name_offset };
        let n = db.get_string2(&off).unwrap_or("?");
        println!("    {n}");
    }
}

fn label_dist_nonref(d: &CraftingQualityDistribution_Base_NonRefPtr) -> &'static str {
    use CraftingQualityDistribution_Base_NonRefPtr::*;
    match d {
        CraftingQualityDistribution_Base_NonRef(_) => "Base_NonRef",
        CraftingQualityDistributionNormal(_) => "Normal",
        Unknown { .. } => "Unknown (likely Uniform - dormant)",
    }
}

fn label_loc_nonref(o: &CraftingQualityLocationOverride_Base_NonRefPtr) -> &'static str {
    use CraftingQualityLocationOverride_Base_NonRefPtr::*;
    match o {
        CraftingQualityLocationOverride_Base_NonRef(_) => "Base_NonRef",
        CraftingQualityLocationOverride(_) => "LocationOverride",
        Unknown { .. } => "Unknown",
    }
}

fn label_qz_nonref(q: &CraftingQualityQuantization_Base_NonRefPtr) -> &'static str {
    use CraftingQualityQuantization_Base_NonRefPtr::*;
    match q {
        CraftingQualityQuantization_Base_NonRef(_) => "Base_NonRef",
        CraftingQualityQuantization(_) => "Quantization",
        Unknown { .. } => "Unknown",
    }
}

type Guid = sc_extract::Guid;

fn lookup_resource<'a>(
    records: &sc_extract::generated::RecordIndex,
    pools: &'a DataPools,
    guid: Guid,
) -> Option<&'a ResourceType> {
    records
        .multi_feature
        .resource_type
        .get(&guid)
        .copied()
        .and_then(|h| h.get(pools))
}

// re-export under simpler path for the schema_dump fn signature
use sc_extract::svarog_datacore;
