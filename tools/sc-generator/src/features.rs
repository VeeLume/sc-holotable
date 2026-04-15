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

use svarog_common::CigGuid;
use svarog_datacore::{DataCoreDatabase, DataType};

use crate::closure::{walk_closure, GuidLookup};
use crate::emit::PropertyCache;

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
}

impl Default for FeatureRules {
    fn default() -> Self {
        Self {
            max_fields_per_feature: 500,
            min_fields_per_feature: 50,
            max_depth: 10,
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

    /// Parent feature name → child feature names it enables.
    pub parent_features: BTreeMap<String, Vec<String>>,
}

/// A type's feature assignment.
#[derive(Debug, Clone)]
pub enum FeatureAssignment {
    /// Always compiled — no cfg attribute. Reserved for role-based
    /// promotion (empty polymorphic bases) that have no natural home
    /// in any feature closure but are referenced unconditionally from
    /// poly enums.
    Core,
    /// Schema-reachable but not observed in any record instance graph.
    /// Lives in the `core` module file but only compiled when the
    /// `dormant` Cargo feature is enabled. See `docs/feature-gating-v2.md`
    /// Decision 5 for the rationale.
    Dormant,
    /// Belongs to exactly one feature.
    Single(String),
    /// Reachable from multiple features' data-driven closures. Lives in
    /// the `core` module file gated by `#[cfg(any(feature = "…"))]`.
    Multi(Vec<String>),
}

// ── Main entry point ──────────────────────────────────────────────────────

/// Classify all emitted struct types into features based on compile cost.
///
/// `guid_lookup` is a pre-built GUID → (struct_index, instance_index) map
/// used by the data-driven closure walker to follow `Reference` edges.
/// The same lookup is shared across every feature's walk, so build it once
/// at the start of the generator run and pass it through.
///
/// `promoted` is the set of struct indices that should be emitted
/// unconditionally in `core` regardless of closure membership — typically
/// the empty polymorphic bases computed in Phase 4 of v2 (see
/// `docs/feature-gating-v2.md` Decision 4). An empty set disables the
/// promotion pathway.
pub fn classify_features(
    db: &DataCoreDatabase,
    reachable_indices: &HashSet<usize>,
    guid_lookup: &GuidLookup,
    promoted: &HashSet<u32>,
    rules: &FeatureRules,
    cache: &PropertyCache<'_>,
    dangling: &mut HashSet<CigGuid>,
) -> FeatureMap {
    let n_structs = db.struct_definitions().len();

    // Precompute field count (compile cost) for each struct. Reads the
    // shared inherited-property cache so no inheritance walks happen here.
    let field_counts: Vec<usize> = (0..n_structs)
        .map(|idx| {
            if reachable_indices.contains(&idx) {
                cache[idx].len()
            } else {
                0
            }
        })
        .collect();

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
        cache,
        &mut leaf_groups,
        &mut parent_features,
    );

    // Step 2: Compute data-driven type closures per feature.
    //
    // Each closure is the set of struct indices *observed at runtime* when
    // walking the record instance graph from this feature's seed records
    // through Class / StrongPointer / WeakPointer / Reference edges. This
    // reflects what the DCB actually stores, not what the schema declares
    // as theoretically-possible subclasses — see `docs/feature-gating-v2.md`
    // Decision 1 for the rationale.
    let mut feature_closures: BTreeMap<String, HashSet<usize>> = BTreeMap::new();
    for (feature_name, prefixes) in &leaf_groups {
        let observed_u32 = walk_closure(db, prefixes, guid_lookup, cache, dangling);
        let closure: HashSet<usize> = observed_u32
            .into_iter()
            .map(|i| i as usize)
            .filter(|i| reachable_indices.contains(i))
            .collect();
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
    // Assign every type with closure membership:
    // - role-based promoted → Core (unconditional, no cfg)
    // - observed in exactly one feature's data-driven closure → Single(f)
    // - observed in ≥ 2 features' closures → Multi(list), gated via
    //   `#[cfg(any(feature = "…"))]` at emit time
    // - observed in zero closures → Dormant (schema-reachable but not
    //   populated in real DCB data), gated on `#[cfg(feature = "dormant")]`
    for &struct_idx in reachable_indices {
        let assignment = if promoted.contains(&(struct_idx as u32)) {
            FeatureAssignment::Core
        } else {
            match struct_to_features.get(&struct_idx) {
                Some(fs) => {
                    let mut sorted = fs.clone();
                    sorted.sort();
                    sorted.dedup();
                    if sorted.len() == 1 {
                        FeatureAssignment::Single(sorted.into_iter().next().unwrap())
                    } else {
                        FeatureAssignment::Multi(sorted)
                    }
                }
                None => FeatureAssignment::Dormant,
            }
        };
        struct_feature[struct_idx] = Some(assignment);
    }

    // Step 4: Prune features that nothing uses.
    //
    // A feature is "used" if **any** assignment names it — Single OR a
    // member of a Multi list. The previous implementation only counted
    // Single occurrences, which was fine when core-promotion ate every
    // Multi assignment but is wrong now that we emit Multi as real cfg
    // gates: a feature whose types are all Multi-assigned would have
    // been dropped here and its types would be incorrectly promoted to
    // unconditional Core at the bottom of this block.
    let mut used_features: HashSet<String> = HashSet::new();
    for assignment in struct_feature.iter().flatten() {
        match assignment {
            FeatureAssignment::Single(f) => {
                used_features.insert(f.clone());
            }
            FeatureAssignment::Multi(fs) => {
                for f in fs {
                    used_features.insert(f.clone());
                }
            }
            FeatureAssignment::Core | FeatureAssignment::Dormant => {}
        }
    }
    let non_empty = used_features;

    // Fix up orphaned single/multi assignments whose feature was pruned.
    // Rare and should never fire once the data-driven closure is stable,
    // but kept as a safety net: orphans become Dormant so they don't
    // contribute to the default compile surface.
    for assignment in struct_feature.iter_mut().flatten() {
        let should_promote = match assignment {
            FeatureAssignment::Single(f) => !non_empty.contains(f),
            FeatureAssignment::Multi(fs) => {
                fs.retain(|f| non_empty.contains(f));
                fs.is_empty()
            }
            FeatureAssignment::Core | FeatureAssignment::Dormant => false,
        };
        if should_promote {
            *assignment = FeatureAssignment::Dormant;
        }
    }

    // Final feature list (only non-empty leaves).
    let mut feature_names: Vec<String> = non_empty.into_iter().collect();
    feature_names.sort();
    let feature_name_set: HashSet<&String> = feature_names.iter().collect();

    // Filter parent features to only reference existing children.
    let parent_features: BTreeMap<String, Vec<String>> = parent_features
        .into_iter()
        .map(|(parent, children)| {
            let valid: Vec<String> = children
                .into_iter()
                .filter(|c| feature_name_set.contains(c))
                .collect();
            (parent, valid)
        })
        .filter(|(_, children)| children.len() > 1)
        .collect();

    FeatureMap {
        struct_feature,
        feature_names,
        parent_features,
    }
}

// ── Tree walking ──────────────────────────────────────────────────────────

/// Recursively walk the path tree, splitting on compile cost.
///
/// Cost is computed from the **BFS type closure** (not just seed types)
/// so the split decision reflects actual compile-time impact.
#[allow(clippy::too_many_arguments)]
fn walk_tree(
    records: &[(String, usize)],
    field_counts: &[usize],
    db: &DataCoreDatabase,
    reachable: &HashSet<usize>,
    parent_prefix: &str,
    depth: usize,
    rules: &FeatureRules,
    cache: &PropertyCache<'_>,
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
        // Compute compile cost from the MONOMORPHIC (base) closure
        // — no descendant expansion. Descendant-aware closures make
        // every subtree look equally expensive and force splits all
        // the way down to max_depth, which is why this path walks
        // `compute_base_closure` instead of `compute_type_closure`.
        let closure = compute_base_closure(db, child_struct_indices, reachable, cache);
        let cost: usize = closure.iter().map(|&idx| field_counts[idx]).sum();

        if cost > rules.max_fields_per_feature && depth < rules.max_depth {
            // Too expensive — try splitting deeper.
            let deeper = children_at_depth(records, child_prefix, depth + 1);
            if deeper.len() > 1 {
                // Can split — recurse.
                walk_tree(records, field_counts, db, reachable, child_prefix, depth + 1, rules, cache, leaves, parents);
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
        .replace(['.', ' '], "_")
        .to_lowercase();
    // Remove any character that's not alphanumeric, `-`, or `_`.
    clean
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

// ── BFS type closure ──────────────────────────────────────────────────────

/// BFS through struct fields to find all types transitively reachable.
/// Compute the transitive closure of types referenced from `seeds` through
/// declared Class / Pointer field targets only.
///
/// This is the "monomorphic" closure: it does NOT follow pointer
/// descendants. Used by `walk_tree` to decide when to split a path node
/// into child features. Using the monomorphic cost here keeps the split
/// decisions grounded in the DCB's native field graph without letting
/// polymorphic subclass fan-out inflate every subtree's cost to the
/// point where everything splits to max depth.
fn compute_base_closure(
    db: &DataCoreDatabase,
    seeds: &HashSet<usize>,
    reachable: &HashSet<usize>,
    cache: &PropertyCache<'_>,
) -> HashSet<usize> {
    let mut closure: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    for &seed in seeds {
        if closure.insert(seed) {
            queue.push_back(seed);
        }
    }

    while let Some(struct_idx) = queue.pop_front() {
        for prop in &cache[struct_idx] {
            if matches!(
                prop.get_data_type(),
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
