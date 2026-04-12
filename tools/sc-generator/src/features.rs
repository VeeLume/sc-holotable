//! Auto-feature classification for generated types.
//!
//! Walks the DCB record path tree top-down and splits nodes into features
//! based on **compile cost** — the sum of field counts across all struct
//! types under a path prefix. This directly correlates with the actual
//! compiler work (serde derives, Extract impls, LLVM passes) per feature.
//!
//! # Algorithm
//!
//! 1. Start at root (`libs/foundry/records/`)
//! 2. At each node, compute compile cost = Σ field_count(type) for all
//!    unique struct types whose records live under this path prefix
//! 3. If cost > threshold → split into children, recurse
//! 4. If cost ≤ threshold → this node is a leaf feature
//! 5. Feature name = the path (always)
//! 6. Any node that was split → parent feature (Cargo alias enabling children)
//!
//! # Compile cost model
//!
//! Each struct field generates approximately:
//! - 1 match arm in `serde::Deserialize` visitor (`visit_map`)
//! - 1 serialization call in `serde::Serialize`
//! - 1 decode line in `Extract::extract`
//! - 1 field in `Debug` / `Clone` derives
//!
//! So compile cost ≈ total_fields × constant. A struct with 50 fields
//! costs ~10x a struct with 5 fields. The split threshold operates on
//! total fields to produce roughly equal-cost features.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use svarog_datacore::structs::DataCorePropertyDefinition;
use svarog_datacore::{DataCoreDatabase, DataType};

// ── Public types ──────────────────────────────────────────────────────────

/// Configuration for the splitting algorithm.
#[derive(Debug, Clone)]
pub struct FeatureRules {
    /// Maximum total fields (compile cost) before a node is split.
    /// With ~29,000 total fields across 1,935 types, a threshold of
    /// 1500 produces ~20 features of roughly equal compile cost.
    pub max_fields_per_feature: usize,

    /// Minimum total fields for a node to be worth its own feature.
    /// Below this, the compile savings are negligible.
    pub min_fields_per_feature: usize,

    /// Maximum path depth to recurse. Safety limit.
    pub max_depth: usize,

    /// Types needed by this many features or more are promoted to `core`.
    pub core_promotion_threshold: usize,
}

impl Default for FeatureRules {
    fn default() -> Self {
        Self {
            max_fields_per_feature: 500,
            min_fields_per_feature: 50,
            max_depth: 10,
            core_promotion_threshold: 5,
        }
    }
}

/// The result of feature classification.
#[derive(Debug, Clone)]
pub struct FeatureMap {
    /// Feature assignment for each struct index.
    pub struct_feature: Vec<Option<FeatureAssignment>>,

    /// All leaf feature names (these contain actual types).
    pub feature_names: Vec<String>,

    /// Leaf feature name → path prefixes that seeded it.
    pub feature_seeds: BTreeMap<String, Vec<String>>,

    /// Parent feature name → child feature names it enables.
    pub parent_features: BTreeMap<String, Vec<String>>,

    /// Compile cost (total fields) per leaf feature.
    pub feature_costs: BTreeMap<String, usize>,

    /// Total compile cost across all types.
    pub total_cost: usize,
}

/// A type's feature assignment.
#[derive(Debug, Clone)]
pub enum FeatureAssignment {
    /// Always compiled — no cfg attribute needed.
    Core,
    /// Belongs to exactly one feature.
    Single(String),
    /// Needed by multiple features (2-4).
    Multi(Vec<String>),
}

impl FeatureAssignment {
    /// Generate the cfg attribute string. `None` for Core.
    pub fn cfg_attribute(&self) -> Option<String> {
        match self {
            Self::Core => None,
            Self::Single(f) => Some(format!("#[cfg(feature = \"{f}\")]")),
            Self::Multi(fs) => {
                let parts: Vec<String> = fs.iter().map(|f| format!("feature = \"{f}\"")).collect();
                Some(format!("#[cfg(any({}))]", parts.join(", ")))
            }
        }
    }
}

// ── Main entry point ──────────────────────────────────────────────────────

/// Classify all emitted struct types into features based on compile cost.
pub fn classify_features(
    db: &DataCoreDatabase,
    reachable_indices: &HashSet<usize>,
    rules: &FeatureRules,
) -> FeatureMap {
    let n_structs = db.struct_definitions().len();

    // Precompute field count (compile cost) for each struct.
    let field_counts: Vec<usize> = (0..n_structs)
        .map(|idx| {
            if reachable_indices.contains(&idx) {
                collect_full_properties(db, idx as u32).len()
            } else {
                0
            }
        })
        .collect();

    let total_cost: usize = field_counts.iter().sum();

    // Collect record → (path, struct_index) mapping.
    let mut records: Vec<(String, usize)> = Vec::new();
    for record in db.all_records() {
        if let Some(file_name) = record.file_name() {
            let idx = record.struct_index() as usize;
            if reachable_indices.contains(&idx) {
                records.push((file_name.to_string(), idx));
            }
        }
    }

    // Step 1: Walk the tree and determine leaf features.
    let mut leaf_groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut parent_features: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let reachable_set: HashSet<usize> = reachable_indices.iter().copied().collect();
    let root = "libs/foundry/records";
    walk_tree(
        &records,
        &field_counts,
        db,
        &reachable_set,
        root,
        4, // start depth (first level below root)
        rules,
        &mut leaf_groups,
        &mut parent_features,
    );

    // Step 2: Compute type closures per feature and assign types.
    let mut feature_closures: BTreeMap<String, HashSet<usize>> = BTreeMap::new();
    for (feature_name, prefixes) in &leaf_groups {
        // Seed types: unique struct indices under this feature's paths.
        let seeds: HashSet<usize> = records
            .iter()
            .filter(|(path, _)| prefixes.iter().any(|p| path.starts_with(p.as_str())))
            .map(|(_, idx)| *idx)
            .collect();

        let closure = compute_type_closure(db, &seeds, reachable_indices);
        feature_closures.insert(feature_name.clone(), closure);
    }

    // Step 3: Assign each type to features based on closure membership.
    let mut struct_to_features: HashMap<usize, Vec<String>> = HashMap::new();
    for (feature_name, closure) in &feature_closures {
        for &struct_idx in closure {
            struct_to_features
                .entry(struct_idx)
                .or_default()
                .push(feature_name.clone());
        }
    }

    let mut struct_feature: Vec<Option<FeatureAssignment>> = vec![None; n_structs];
    for &struct_idx in reachable_indices {
        let assignment = match struct_to_features.get(&struct_idx) {
            None => FeatureAssignment::Core,
            Some(fs) => {
                let mut sorted = fs.clone();
                sorted.sort();
                sorted.dedup();
                if sorted.len() >= rules.core_promotion_threshold {
                    FeatureAssignment::Core
                } else if sorted.len() == 1 {
                    FeatureAssignment::Single(sorted.into_iter().next().unwrap())
                } else {
                    FeatureAssignment::Multi(sorted)
                }
            }
        };
        struct_feature[struct_idx] = Some(assignment);
    }

    // Step 4: Prune features with 0 exclusive types.
    let mut exclusive_count: HashMap<String, usize> = HashMap::new();
    for assignment in struct_feature.iter().flatten() {
        if let FeatureAssignment::Single(f) = assignment {
            *exclusive_count.entry(f.clone()).or_default() += 1;
        }
    }

    let non_empty: HashSet<String> = exclusive_count
        .iter()
        .filter(|(_, count)| **count > 0)
        .map(|(name, _)| name.clone())
        .collect();

    // Promote types from pruned features to core.
    for assignment in struct_feature.iter_mut().flatten() {
        let should_promote = match assignment {
            FeatureAssignment::Single(f) => !non_empty.contains(f),
            FeatureAssignment::Multi(fs) => {
                fs.retain(|f| non_empty.contains(f));
                fs.is_empty()
            }
            FeatureAssignment::Core => false,
        };
        if should_promote {
            *assignment = FeatureAssignment::Core;
        }
    }

    // Compute compile cost per leaf feature.
    let mut feature_costs: BTreeMap<String, usize> = BTreeMap::new();
    for (struct_idx, assignment) in struct_feature.iter().enumerate().filter_map(|(i, a)| a.as_ref().map(|a| (i, a))) {
        let cost = field_counts[struct_idx];
        match assignment {
            FeatureAssignment::Single(f) => {
                *feature_costs.entry(f.clone()).or_default() += cost;
            }
            FeatureAssignment::Core => {
                *feature_costs.entry("core".to_string()).or_default() += cost;
            }
            FeatureAssignment::Multi(fs) => {
                // Attribute cost to first feature (arbitrary but consistent).
                if let Some(f) = fs.first() {
                    *feature_costs.entry(f.clone()).or_default() += cost;
                }
            }
        }
    }

    // Final feature list (only non-empty leaves).
    let mut feature_names: Vec<String> = non_empty.into_iter().collect();
    feature_names.sort();

    // Filter parent features to only reference existing children.
    let parent_features: BTreeMap<String, Vec<String>> = parent_features
        .into_iter()
        .map(|(parent, children)| {
            let valid: Vec<String> = children.into_iter().filter(|c| feature_names.contains(c) || feature_costs.contains_key(c)).collect();
            (parent, valid)
        })
        .filter(|(_, children)| children.len() > 1)
        .collect();

    let feature_seeds: BTreeMap<String, Vec<String>> = leaf_groups
        .into_iter()
        .filter(|(name, _)| feature_names.contains(name))
        .collect();

    FeatureMap {
        struct_feature,
        feature_names,
        feature_seeds,
        parent_features,
        feature_costs,
        total_cost,
    }
}

// ── Tree walking ──────────────────────────────────────────────────────────

/// Recursively walk the path tree, splitting on compile cost.
///
/// Cost is computed from the **BFS type closure** (not just seed types)
/// so the split decision reflects actual compile-time impact.
fn walk_tree(
    records: &[(String, usize)],
    field_counts: &[usize],
    db: &DataCoreDatabase,
    reachable: &HashSet<usize>,
    parent_prefix: &str,
    depth: usize,
    rules: &FeatureRules,
    leaves: &mut BTreeMap<String, Vec<String>>,
    parents: &mut BTreeMap<String, Vec<String>>,
) {
    // Get children at this depth level.
    let children = children_at_depth(records, parent_prefix, depth);

    if children.is_empty() {
        // Leaf in the file tree — make this a feature regardless of size.
        let name = path_to_feature_name(parent_prefix);
        leaves.insert(name, vec![parent_prefix.to_string()]);
        return;
    }

    let mut child_feature_names: Vec<String> = Vec::new();

    for (child_prefix, child_struct_indices) in &children {
        // Compute compile cost from the full BFS closure, not just seeds.
        let closure = compute_type_closure(db, child_struct_indices, reachable);
        let cost: usize = closure.iter().map(|&idx| field_counts[idx]).sum();

        if cost > rules.max_fields_per_feature && depth < rules.max_depth {
            // Too expensive — try splitting deeper.
            let deeper = children_at_depth(records, child_prefix, depth + 1);
            if deeper.len() > 1 {
                // Can split — recurse.
                walk_tree(records, field_counts, db, reachable, child_prefix, depth + 1, rules, leaves, parents);
                let child_name = path_to_feature_name(child_prefix);
                child_feature_names.push(child_name);
            } else {
                // Can't split (single child or file-level) — leaf.
                let name = path_to_feature_name(child_prefix);
                leaves.insert(name.clone(), vec![child_prefix.clone()]);
                child_feature_names.push(name);
            }
        } else if cost < rules.min_fields_per_feature {
            // Small but still a leaf feature — keeps core small.
            let name = path_to_feature_name(child_prefix);
            leaves.insert(name.clone(), vec![child_prefix.clone()]);
            child_feature_names.push(name);
        } else {
            // Right-sized — leaf feature.
            let name = path_to_feature_name(child_prefix);
            leaves.insert(name.clone(), vec![child_prefix.clone()]);
            child_feature_names.push(name);
        }
    }

    // Register parent feature if this node was split into multiple children.
    if child_feature_names.len() > 1 {
        let parent_name = path_to_feature_name(parent_prefix);
        // Collect all descendant leaves.
        let all_leaves = collect_all_leaves(&child_feature_names, leaves, parents);
        if !all_leaves.is_empty() {
            parents.insert(parent_name, all_leaves);
        }
    }
}

/// Get child path groups at a given depth under a parent prefix.
/// Returns: child_prefix → set of unique struct indices under it.
fn children_at_depth(
    records: &[(String, usize)],
    parent: &str,
    depth: usize,
) -> BTreeMap<String, HashSet<usize>> {
    let mut children: BTreeMap<String, HashSet<usize>> = BTreeMap::new();
    for (path, struct_idx) in records {
        if !path.starts_with(parent) {
            continue;
        }
        let prefix: String = path.split('/').take(depth).collect::<Vec<_>>().join("/");
        if prefix.len() > parent.len() {
            children.entry(prefix).or_default().insert(*struct_idx);
        }
    }
    children
}

/// Collect all leaf feature names reachable from a list of children
/// (traversing parent→child relationships).
fn collect_all_leaves(
    children: &[String],
    leaves: &BTreeMap<String, Vec<String>>,
    parents: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut result = Vec::new();
    for child in children {
        if leaves.contains_key(child) {
            result.push(child.clone());
        }
        if let Some(grandchildren) = parents.get(child) {
            result.extend(grandchildren.iter().cloned());
        }
    }
    result.sort();
    result.dedup();
    result
}

// ── Naming ────────────────────────────────────────────────────────────────

/// Convert a path prefix to a feature name.
/// Strips `libs/foundry/records/`, replaces `/` with `-`.
fn path_to_feature_name(prefix: &str) -> String {
    let stripped = prefix
        .strip_prefix("libs/foundry/records/")
        .unwrap_or(prefix);
    let clean = stripped
        .replace(".xml", "")
        .replace('/', "-")
        .replace('.', "_")
        .replace(' ', "_")
        .to_lowercase();
    // Remove any character that's not alphanumeric, `-`, or `_`.
    clean
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

// ── BFS type closure ──────────────────────────────────────────────────────

/// BFS through struct fields to find all types transitively reachable.
fn compute_type_closure(
    db: &DataCoreDatabase,
    seeds: &HashSet<usize>,
    reachable: &HashSet<usize>,
) -> HashSet<usize> {
    let mut closure: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    for &seed in seeds {
        if closure.insert(seed) {
            queue.push_back(seed);
        }
    }

    while let Some(struct_idx) = queue.pop_front() {
        let props = collect_full_properties(db, struct_idx as u32);
        for prop in &props {
            let data_type = prop.get_data_type();
            if matches!(
                data_type,
                Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
            ) {
                let target = prop.struct_index as usize;
                if target < db.struct_definitions().len()
                    && reachable.contains(&target)
                    && closure.insert(target)
                {
                    queue.push_back(target);
                }
            }
        }
    }

    closure
}

fn collect_full_properties(db: &DataCoreDatabase, struct_idx: u32) -> Vec<&DataCorePropertyDefinition> {
    let mut out = Vec::new();
    collect_props_recursive(db, struct_idx, &mut out);
    out
}

fn collect_props_recursive<'a>(
    db: &'a DataCoreDatabase,
    struct_idx: u32,
    out: &mut Vec<&'a DataCorePropertyDefinition>,
) {
    let Some(def) = db.struct_definitions().get(struct_idx as usize) else {
        return;
    };
    if def.parent_type_index >= 0 {
        collect_props_recursive(db, def.parent_type_index as u32, out);
    }
    for prop in db.get_struct_properties(struct_idx as usize) {
        out.push(prop);
    }
}
