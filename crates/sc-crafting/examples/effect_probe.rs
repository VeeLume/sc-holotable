//! Throwaway probe: does live DCB populate the per-ingredient
//! `context` (gameplay-property modifiers) + `CraftingCost_Select.name_info`
//! (slot labels) that the Hearth crafting UI renders? sc-crafting's typed
//! `Cost` model drops both today — this measures what we'd be dropping.
//!
//! ```bash
//! cargo run -p sc-crafting --release --example effect_probe
//! ```
#![allow(non_snake_case)]

use std::collections::BTreeMap;

use sc_extract::generated::{
    CraftingBlueprint_Base_NonRefPtr, CraftingBlueprintTier_BasePtr, CraftingCost_BasePtr,
    CraftingCostContext_BasePtr, CraftingGameplayPropertyModifier_BasePtr,
    CraftingGameplayPropertyModifierValueRange_BasePtr, CraftingGameplayPropertyModifiers_BasePtr,
    CraftingRecipe_BasePtr, CraftingRecipeCosts_BasePtr, DataPools,
};
use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, LocaleMap};
use sc_items::Items;

#[derive(Default)]
struct Probe {
    costs_seen: usize,
    costs_with_context: usize,
    context_variant: BTreeMap<&'static str, usize>,
    selects_seen: usize,
    selects_with_name_info: usize,
    slot_labels: BTreeMap<String, usize>,
    modifier_lists: usize,
    modifiers_common: usize,
    value_ranges: BTreeMap<&'static str, usize>,
    // sample lines: "slot → property : Q a-b ×s-e"
    samples: Vec<String>,
    prop_names: std::collections::HashMap<Guid, String>,
    prop_usage: std::collections::HashMap<Guid, usize>,
}

fn transform_kind(
    ptr: &sc_extract::generated::CraftingDisplayTransformation_BasePtr,
    pools: &DataPools,
) -> String {
    use sc_extract::generated::CraftingDisplayTransformation_BasePtr as D;
    match ptr {
        D::CraftingDisplayTransformation_Scale(h) => h
            .get(pools)
            .map(|s| format!("Scale(×{})", s.scale))
            .unwrap_or_else(|| "Scale(?)".into()),
        D::CraftingDisplayTransformation_ConvertFactorToPercentChange(_) => {
            "FactorToPercent".into()
        }
        D::CraftingDisplayTransformation_ConvertFactorToNegatedPercentChange(_) => {
            "FactorToNegatedPercent".into()
        }
        D::CraftingDisplayTransformation_ConvertValueToFactorOfBaseValue(_) => {
            "ValueToFactorOfBase".into()
        }
        D::CraftingDisplayTransformation_Sequence(_) => "Sequence".into(),
        _ => "(base/unknown)".into(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("install : {} v{}", install.channel, install.short_version());

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let items = Items::build(datacore.records());
    let locale = &asset_data.locale;

    let records = &datacore.records().records;
    let pools = &datacore.records().pools;

    let mut p = Probe::default();

    // Build gameplay-property GUID -> resolved name map first.
    for (&guid, &handle) in &records.multi_feature.crafting_gameplay_property_def {
        if let Some(rec) = handle.get(pools) {
            let name = locale
                .resolve(&rec.property_name)
                .unwrap_or_else(|| rec.property_name.as_ref())
                .to_string();
            p.prop_names.insert(guid, name);
        }
    }
    println!("gameplay property defs : {}", p.prop_names.len());

    for handle in records.multi_feature.crafting_blueprint_record.values() {
        let Some(bp_record) = handle.get(pools) else {
            continue;
        };
        let Some(bp_ptr) = &bp_record.blueprint else {
            continue;
        };
        let CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(bh) = bp_ptr else {
            continue;
        };
        let Some(bp) = bh.get(pools) else { continue };

        let crafted_name = match &bp.process_specific_data {
            Some(sc_extract::generated::CraftingProcess_BasePtr::CraftingProcess_Creation(h)) => h
                .get(pools)
                .and_then(|c| c.entity_class)
                .and_then(|g| items.name_key(&g))
                .and_then(|k| locale.resolve(k))
                .unwrap_or("?")
                .to_string(),
            _ => "?".to_string(),
        };

        for tier_ptr in &bp.tiers {
            let CraftingBlueprintTier_BasePtr::CraftingBlueprintTier(th) = tier_ptr else {
                continue;
            };
            let Some(tier) = th.get(pools) else { continue };
            let Some(CraftingRecipe_BasePtr::CraftingRecipe(rh)) = &tier.recipe else {
                continue;
            };
            let Some(recipe) = rh.get(pools) else {
                continue;
            };
            let Some(CraftingRecipeCosts_BasePtr::CraftingRecipeCosts(ch)) = &recipe.costs else {
                continue;
            };
            let Some(costs) = ch.get(pools) else { continue };
            if let Some(mc) = &costs.mandatory_cost {
                walk_cost(mc, pools, locale, &crafted_name, "(top)", &mut p);
            }
        }
    }

    println!("\n=== cost nodes ===");
    println!("  total cost nodes walked     : {}", p.costs_seen);
    println!("  with non-empty context[]    : {}", p.costs_with_context);
    println!("  context variant distribution:");
    for (k, v) in &p.context_variant {
        println!("    {:<52} : {}", k, v);
    }

    println!("\n=== Select slot labels (CraftingCost_Select.name_info) ===");
    println!("  Select nodes seen           : {}", p.selects_seen);
    println!(
        "  with name_info              : {}",
        p.selects_with_name_info
    );
    println!("  distinct slot labels:");
    for (k, v) in &p.slot_labels {
        println!("    {:<32} : {}", k, v);
    }

    println!("\n=== gameplay property modifier chain ===");
    println!("  modifier lists reached      : {}", p.modifier_lists);
    println!(
        "  CraftingGameplayPropertyModifierCommon : {}",
        p.modifiers_common
    );
    println!("  value-range variant distribution:");
    for (k, v) in &p.value_ranges {
        println!("    {:<52} : {}", k, v);
    }

    println!(
        "\n=== sample effect lines (first {}) ===",
        p.samples.len().min(40)
    );
    for s in p.samples.iter().take(40) {
        println!("  {}", s);
    }

    // ── gameplay property def table: record name vs display name vs unit ──
    println!(
        "\n=== CraftingGameplayPropertyDef table (record name | display | unit | transform | uses) ==="
    );
    let db = datacore.db();
    let mut rows: Vec<(String, String, String, String, usize)> = Vec::new();
    for (&guid, &handle) in &records.multi_feature.crafting_gameplay_property_def {
        let Some(rec) = handle.get(pools) else {
            continue;
        };
        let record_name = db
            .record(&guid)
            .and_then(|r| r.name())
            .unwrap_or("?")
            .to_string();
        let display = locale
            .resolve(&rec.property_name)
            .unwrap_or_else(|| rec.property_name.as_ref())
            .to_string();
        let unit = if rec.unit_format.as_ref().is_empty() {
            "(none)".to_string()
        } else {
            locale
                .resolve(&rec.unit_format)
                .map(|s| if s.is_empty() { "(empty)" } else { s })
                .unwrap_or_else(|| rec.unit_format.as_ref())
                .to_string()
        };
        let transform = rec
            .display_transformation
            .as_ref()
            .map(|t| transform_kind(t, pools))
            .unwrap_or_else(|| "(none)".into());
        let uses = p.prop_usage.get(&guid).copied().unwrap_or(0);
        rows.push((record_name, display, unit, transform, uses));
    }
    rows.sort_by(|a, b| b.4.cmp(&a.4).then(a.0.cmp(&b.0)));
    for (rn, disp, unit, tf, uses) in &rows {
        println!(
            "  {:<42} | {:<26} | {:<14} | {:<24} | {}",
            rn, disp, unit, tf, uses
        );
    }

    Ok(())
}

fn walk_cost(
    ptr: &CraftingCost_BasePtr,
    pools: &DataPools,
    locale: &LocaleMap,
    crafted: &str,
    slot: &str,
    p: &mut Probe,
) {
    p.costs_seen += 1;
    match ptr {
        CraftingCost_BasePtr::CraftingCost_Select(h) => {
            let Some(sel) = h.get(pools) else { return };
            p.selects_seen += 1;
            let mut this_slot = slot.to_string();
            if let Some(nih) = &sel.name_info
                && let Some(ni) = nih.get(pools)
            {
                p.selects_with_name_info += 1;
                let label = locale
                    .resolve(&ni.display_name)
                    .map(str::to_string)
                    .unwrap_or_else(|| {
                        if ni.display_name.as_ref().is_empty() {
                            ni.debug_name.clone()
                        } else {
                            ni.display_name.as_ref().to_string()
                        }
                    });
                if !label.is_empty() {
                    *p.slot_labels.entry(label.clone()).or_default() += 1;
                    this_slot = label;
                }
            }
            walk_context(&sel.context, pools, locale, crafted, &this_slot, p);
            for o in &sel.options {
                walk_cost(o, pools, locale, crafted, &this_slot, p);
            }
        }
        CraftingCost_BasePtr::CraftingCost_Resource(h) => {
            if let Some(r) = h.get(pools) {
                walk_context(&r.context, pools, locale, crafted, slot, p);
            }
        }
        CraftingCost_BasePtr::CraftingCost_Item(h) => {
            if let Some(i) = h.get(pools) {
                walk_context(&i.context, pools, locale, crafted, slot, p);
            }
        }
        _ => {}
    }
}

fn walk_context(
    ctx: &[CraftingCostContext_BasePtr],
    pools: &DataPools,
    locale: &LocaleMap,
    crafted: &str,
    slot: &str,
    p: &mut Probe,
) {
    if !ctx.is_empty() {
        p.costs_with_context += 1;
    }
    for c in ctx {
        match c {
            CraftingCostContext_BasePtr::CraftingCostContext_ResultGameplayPropertyModifiers(h) => {
                *p.context_variant
                    .entry("ResultGameplayPropertyModifiers")
                    .or_default() += 1;
                if let Some(rec) = h.get(pools)
                    && let Some(mods) = &rec.gameplay_property_modifiers
                {
                    walk_modifiers(mods, pools, locale, crafted, slot, p);
                }
            }
            CraftingCostContext_BasePtr::CraftingCostContext_ResultCompositionInclusion(_) => {
                *p.context_variant
                    .entry("ResultCompositionInclusion")
                    .or_default() += 1;
            }
            CraftingCostContext_BasePtr::CraftingCostContext_QuantityMultiplier(_) => {
                *p.context_variant.entry("QuantityMultiplier").or_default() += 1;
            }
            other => {
                let _ = other;
                *p.context_variant.entry("(other/base/unknown)").or_default() += 1;
            }
        }
    }
}

fn walk_modifiers(
    mods: &CraftingGameplayPropertyModifiers_BasePtr,
    pools: &DataPools,
    locale: &LocaleMap,
    crafted: &str,
    slot: &str,
    p: &mut Probe,
) {
    let CraftingGameplayPropertyModifiers_BasePtr::CraftingGameplayPropertyModifiers_List(h) = mods
    else {
        return;
    };
    let Some(list) = h.get(pools) else { return };
    p.modifier_lists += 1;
    for m in &list.gameplay_property_modifiers {
        let CraftingGameplayPropertyModifier_BasePtr::CraftingGameplayPropertyModifierCommon(mh) =
            m
        else {
            continue;
        };
        let Some(common) = mh.get(pools) else {
            continue;
        };
        p.modifiers_common += 1;
        if let Some(g) = common.gameplay_property_record {
            *p.prop_usage.entry(g).or_default() += 1;
        }
        let prop = common
            .gameplay_property_record
            .and_then(|g| p.prop_names.get(&g).cloned())
            .unwrap_or_else(|| "?".to_string());
        let _ = locale;
        for vr in &common.value_ranges {
            match vr {
                CraftingGameplayPropertyModifierValueRange_BasePtr::CraftingGameplayPropertyModifierValueRange_Linear(vh) => {
                    *p.value_ranges.entry("Linear").or_default() += 1;
                    if let Some(v) = vh.get(pools) && p.samples.len() < 200 {
                        p.samples.push(format!(
                            "{crafted} | {slot} → {prop} : Q {}-{} ×{:.3}-{:.3}",
                            v.start_quality, v.end_quality, v.modifier_at_start, v.modifier_at_end
                        ));
                    }
                }
                CraftingGameplayPropertyModifierValueRange_BasePtr::CraftingGameplayPropertyModifierValueRange_LinearIntegerAdditive(vh) => {
                    *p.value_ranges.entry("LinearIntegerAdditive").or_default() += 1;
                    if let Some(v) = vh.get(pools) && p.samples.len() < 200 {
                        p.samples.push(format!(
                            "{crafted} | {slot} → {prop} : Q {}-{} +{}..+{}",
                            v.start_quality, v.end_quality, v.additive_modifier_at_start, v.additive_modifier_at_end
                        ));
                    }
                }
                _ => {
                    *p.value_ranges.entry("(other/base)").or_default() += 1;
                }
            }
        }
    }
}
