//! Name-keyed model of the bindings: what one game build contributes, and
//! the union of several builds that the emitter writes out.
//!
//! The per-build analysis (reachability, data-driven feature closures,
//! polymorphic bases) runs on one `DataCoreDatabase` and is keyed by that
//! build's struct indices. [`lower`] turns its result into a [`BuildModel`]
//! keyed by DCB type name, so builds can be compared; [`merge`] unions them
//! into the [`Model`] the emitter reads.
//!
//! # The window
//!
//! Bindings cover a sliding window of builds, oldest first, newest last:
//! normally the current LIVE and the incoming PTU. The union keeps every
//! type, field, polymorphic variant and enum value seen in any build, so
//! one library release reads both. Each item remembers the builds it came
//! from; the emitter turns that into markers (`#[deprecated]` on what the
//! newest build dropped, a doc line on what only it has).
//!
//! # Merge rules for a field present in several builds
//!
//! - Same kind and target: one field.
//! - Pointer whose declared target moved up or down one hierarchy: the
//!   ancestor wins. Its polymorphic enum already has the other as a variant.
//! - Inline `Class` whose target changed, or a pointer moved to an unrelated
//!   type: a *shape* field, an enum with one variant per target type,
//!   dispatched on the runtime type name.
//! - Anything else (a number becoming a string, a scalar becoming an array):
//!   the generator stops and lists the conflicts. Those need a decision.
//!
//! With one build the model keeps that build's struct indices as ids, so
//! the emitted output is identical to a single-build run.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use svarog_datacore::{DataCoreDatabase, DataType};

use crate::emit::{DescendantMap, EmittedStructNames, PropertyCache};
use crate::features::{FeatureAssignment, FeatureMap};
use crate::naming::sanitize_struct_name;
use crate::pipeline::Metadata;

/// Bit set over [`Model::builds`]: bit `i` is build `i`.
pub type BuildSet = u32;

/// What a struct-kind field (`Class`, `StrongPointer`, `WeakPointer`) or an
/// enum field points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldTarget {
    /// Scalars, strings, GUIDs, references.
    None,
    /// `EnumChoice`: the sanitized Rust enum name, `None` if unresolvable.
    Enum(Option<String>),
    /// Declared struct target by DCB name; `None` if the target is not
    /// emitted (the field degrades to a GUID / `None`, as before).
    Struct(Option<String>),
    /// Target types differ between builds: one variant per DCB type name.
    Shape(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub orig_name: String,
    pub data_type: DataType,
    pub is_array: bool,
    pub target: FieldTarget,
    pub builds: BuildSet,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub raw_name: String,
    /// DCB parent type name, for the `Inherits from` doc line.
    pub parent: Option<String>,
    /// Full inherited field list in DCB order.
    pub fields: Vec<FieldDef>,
    pub assignment: FeatureAssignment,
    /// Builds whose schema emits this type.
    pub builds: BuildSet,
    /// Builds whose records actually use it (not dormant there).
    pub observed: BuildSet,
    /// Builds with top-level records of this type.
    pub records: BuildSet,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub raw_name: String,
    /// Raw DCB values in order, each with the builds that have it.
    pub values: Vec<(String, BuildSet)>,
}

/// One build after analysis, keyed by DCB name.
pub struct BuildModel {
    pub label: String,
    pub metadata: Metadata,
    /// Total struct definitions in this build's schema.
    pub struct_count: u32,
    /// Emitted structs in struct-index order.
    pub structs: Vec<(u32, StructDef)>,
    /// Emitted descendants of each emitted struct, by name.
    pub descendants: BTreeMap<String, BTreeSet<String>>,
    pub enums: Vec<EnumDef>,
    pub feature_map_parents: BTreeMap<String, Vec<String>>,
}

/// The union the emitter writes.
pub struct Model {
    /// Build labels, oldest first, newest last.
    pub builds: Vec<String>,
    /// Struct id → definition. Ids are the newest build's struct indices;
    /// types the newest build lacks get ids after its last index.
    pub structs: BTreeMap<u32, StructDef>,
    /// DCB type name → struct id.
    pub ids: HashMap<String, u32>,
    pub descendants: DescendantMap,
    pub enums: Vec<EnumDef>,
    pub feature_names: Vec<String>,
    pub parent_features: BTreeMap<String, Vec<String>>,
    /// Metadata of the newest build.
    pub metadata: Metadata,
}

impl Model {
    pub fn newest_bit(&self) -> BuildSet {
        1 << (self.builds.len() - 1)
    }

    pub fn all_bits(&self) -> BuildSet {
        (1u32 << self.builds.len()) - 1
    }

    pub fn is_multi_build(&self) -> bool {
        self.builds.len() > 1
    }

    pub fn newest_label(&self) -> &str {
        self.builds.last().map(String::as_str).unwrap_or("")
    }

    /// Struct id of a DCB type name.
    pub fn id(&self, raw_name: &str) -> Option<u32> {
        self.ids.get(raw_name).copied()
    }

    /// `#[deprecated]` note for a type the newest build dropped or no longer
    /// uses; `None` on a single-build model.
    pub fn struct_deprecation(&self, s: &StructDef) -> Option<String> {
        if !self.is_multi_build() {
            return None;
        }
        let newest = self.newest_label();
        if s.builds & self.newest_bit() == 0 {
            Some(format!("absent from {newest}"))
        } else if s.observed != 0 && s.observed & self.newest_bit() == 0 {
            Some(format!("no record in {newest} uses this type"))
        } else {
            None
        }
    }

    /// Doc line for an item only the newest build has; `None` otherwise.
    pub fn new_only_doc(&self, builds: BuildSet) -> Option<String> {
        (self.is_multi_build() && builds == self.newest_bit()).then(|| {
            format!(
                "New in {}; absent from {}.",
                self.newest_label(),
                self.labels(self.all_bits() & !self.newest_bit())
            )
        })
    }

    /// Labels of the builds in `set`, joined for a doc line.
    pub fn labels(&self, set: BuildSet) -> String {
        self.builds
            .iter()
            .enumerate()
            .filter(|(i, _)| set & (1 << i) != 0)
            .map(|(_, l)| l.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

// ── Lowering ──────────────────────────────────────────────────────────────

/// Lower one analysed build into a name-keyed [`BuildModel`].
#[allow(clippy::too_many_arguments)]
pub fn lower(
    db: &DataCoreDatabase,
    label: String,
    metadata: Metadata,
    emitted_names: &EmittedStructNames,
    descendants: &DescendantMap,
    feature_map: &FeatureMap,
    cache: &PropertyCache<'_>,
) -> BuildModel {
    let record_types: HashSet<u32> = db
        .records()
        .iter()
        .filter(|r| r.struct_index >= 0)
        .map(|r| r.struct_index as u32)
        .collect();

    let mut structs = Vec::with_capacity(emitted_names.len());
    for &idx in emitted_names.keys() {
        let raw_name = db
            .struct_name(idx as usize)
            .expect("emitted structs have names")
            .to_string();
        let def = &db.struct_definitions()[idx as usize];
        let parent = (def.parent_type_index >= 0)
            .then(|| db.struct_name(def.parent_type_index as usize))
            .flatten()
            .map(str::to_string);

        let mut fields = Vec::new();
        for prop in &cache[idx as usize] {
            let Some(orig_name) = db.property_name(prop) else {
                continue;
            };
            if orig_name.is_empty() {
                continue;
            }
            let Some(data_type) = prop.get_data_type() else {
                panic!(
                    "sc-generator: unknown DataType on property `{orig_name}` of `{raw_name}`. \
                     svarog's DataType enum may have grown a new variant."
                );
            };
            let target = match data_type {
                DataType::Class | DataType::StrongPointer | DataType::WeakPointer => {
                    let t = prop.struct_index as u32;
                    FieldTarget::Struct(
                        emitted_names
                            .contains_key(&t)
                            .then(|| db.struct_name(t as usize).map(str::to_string))
                            .flatten(),
                    )
                }
                DataType::EnumChoice => FieldTarget::Enum(
                    db.enum_name(prop.struct_index as usize)
                        .map(sanitize_struct_name)
                        .filter(|s| !s.is_empty()),
                ),
                _ => FieldTarget::None,
            };
            fields.push(FieldDef {
                orig_name: orig_name.to_string(),
                data_type,
                is_array: prop.is_array(),
                target,
                builds: 1,
            });
        }

        let assignment = feature_map
            .struct_feature
            .get(idx as usize)
            .cloned()
            .flatten()
            .unwrap_or(FeatureAssignment::Core);
        let observed = u32::from(!matches!(assignment, FeatureAssignment::Dormant));
        structs.push((
            idx,
            StructDef {
                raw_name,
                parent,
                fields,
                assignment,
                builds: 1,
                observed,
                records: u32::from(record_types.contains(&idx)),
            },
        ));
    }

    let mut desc_by_name: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (&parent, kids) in descendants {
        if !emitted_names.contains_key(&parent) {
            continue;
        }
        let Some(pname) = db.struct_name(parent as usize) else {
            continue;
        };
        let names: BTreeSet<String> = kids
            .iter()
            .filter(|k| emitted_names.contains_key(k))
            .filter_map(|&k| db.struct_name(k as usize).map(str::to_string))
            .collect();
        if !names.is_empty() {
            desc_by_name.insert(pname.to_string(), names);
        }
    }

    let enums = (0..db.enum_definitions().len())
        .filter_map(|i| {
            let name = db.enum_name(i)?;
            let def = &db.enum_definitions()[i];
            Some(EnumDef {
                raw_name: name.to_string(),
                values: db
                    .enum_options(def)
                    .iter()
                    .map(|v| (v.to_string(), 1))
                    .collect(),
            })
        })
        .collect();

    BuildModel {
        label,
        metadata,
        struct_count: db.struct_definitions().len() as u32,
        structs,
        descendants: desc_by_name,
        enums,
        feature_map_parents: feature_map.parent_features.clone(),
    }
}

// ── Merging ───────────────────────────────────────────────────────────────

/// A field the merge cannot absorb.
#[derive(Debug)]
pub struct Conflict {
    pub struct_name: String,
    pub field: String,
    pub shapes: Vec<String>,
}

/// Union `builds` (oldest first) into one [`Model`]. Fails with the list of
/// fields whose kind changed incompatibly.
pub fn merge(mut builds: Vec<BuildModel>) -> Result<Model, Vec<Conflict>> {
    assert!(!builds.is_empty(), "merge needs at least one build");
    assert!(builds.len() <= 32, "BuildSet holds at most 32 builds");
    let n = builds.len();
    let labels: Vec<String> = builds.iter().map(|b| b.label.clone()).collect();

    // Rebase every build-local bit (always 1 after lowering) onto its slot.
    for (i, b) in builds.iter_mut().enumerate() {
        let bit = 1u32 << i;
        for (_, s) in &mut b.structs {
            s.builds = bit;
            s.observed *= bit;
            s.records *= bit;
            for f in &mut s.fields {
                f.builds = bit;
            }
        }
        for e in &mut b.enums {
            for v in &mut e.values {
                v.1 = bit;
            }
        }
    }

    // Ids: the newest build keeps its indices; older-only types follow.
    let newest = &builds[n - 1];
    let mut ids: HashMap<String, u32> = HashMap::new();
    for (idx, s) in &newest.structs {
        ids.insert(s.raw_name.clone(), *idx);
    }
    let mut next_id = newest.struct_count;
    for b in builds[..n - 1].iter().rev() {
        for (_, s) in &b.structs {
            if !ids.contains_key(&s.raw_name) {
                ids.insert(s.raw_name.clone(), next_id);
                next_id += 1;
            }
        }
    }

    // Parent chains of every build, for the ancestor test.
    let parents: Vec<HashMap<String, String>> = builds
        .iter()
        .map(|b| {
            b.structs
                .iter()
                .filter_map(|(_, s)| s.parent.clone().map(|p| (s.raw_name.clone(), p)))
                .collect()
        })
        .collect();
    let is_ancestor = |anc: &str, of: &str| {
        parents.iter().any(|chain| {
            let mut cur = chain.get(of);
            let mut guard = 0;
            while let Some(p) = cur {
                if p == anc {
                    return true;
                }
                guard += 1;
                if guard > 256 {
                    return false;
                }
                cur = chain.get(p);
            }
            false
        })
    };

    // Merge structs, newest build first so it sets the base shape.
    let mut merged: HashMap<String, StructDef> = HashMap::new();
    let mut conflicts = Vec::new();
    for b in builds.iter().rev() {
        for (_, s) in &b.structs {
            match merged.get_mut(&s.raw_name) {
                None => {
                    merged.insert(s.raw_name.clone(), s.clone());
                }
                Some(m) => {
                    m.builds |= s.builds;
                    m.observed |= s.observed;
                    m.records |= s.records;
                    if m.parent.is_none() {
                        m.parent = s.parent.clone();
                    }
                    m.assignment = merge_assignment(&m.assignment, &s.assignment);
                    for f in &s.fields {
                        match m.fields.iter_mut().find(|mf| mf.orig_name == f.orig_name) {
                            None => m.fields.push(f.clone()),
                            Some(mf) => {
                                mf.builds |= f.builds;
                                if let Err(shapes) = merge_field(mf, f, &is_ancestor) {
                                    conflicts.push(Conflict {
                                        struct_name: s.raw_name.clone(),
                                        field: f.orig_name.clone(),
                                        shapes,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !conflicts.is_empty() {
        return Err(conflicts);
    }

    let mut structs: BTreeMap<u32, StructDef> = merged
        .into_values()
        .map(|s| (ids[&s.raw_name], s))
        .collect();

    propagate_features(&mut structs, &ids);

    let mut descendants: DescendantMap = BTreeMap::new();
    for b in &builds {
        for (parent, kids) in &b.descendants {
            let Some(&pid) = ids.get(parent) else {
                continue;
            };
            let entry = descendants.entry(pid).or_default();
            entry.extend(kids.iter().filter_map(|k| ids.get(k).copied()));
        }
    }
    for kids in descendants.values_mut() {
        kids.sort_unstable();
        kids.dedup();
    }

    let enums = merge_enums(&builds);

    // Feature names: every feature some type is assigned to.
    let mut used: BTreeSet<String> = BTreeSet::new();
    for s in structs.values() {
        match &s.assignment {
            FeatureAssignment::Single(f) => {
                used.insert(f.clone());
            }
            FeatureAssignment::Multi(fs) => used.extend(fs.iter().cloned()),
            FeatureAssignment::Core | FeatureAssignment::Dormant => {}
        }
    }
    let mut parent_features: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for b in &builds {
        for (parent, kids) in &b.feature_map_parents {
            parent_features
                .entry(parent.clone())
                .or_default()
                .extend(kids.iter().cloned());
        }
    }
    let parent_features = parent_features
        .into_iter()
        .map(|(p, mut kids)| {
            kids.sort();
            kids.dedup();
            kids.retain(|k| used.contains(k));
            (p, kids)
        })
        .filter(|(_, kids)| kids.len() > 1)
        .collect();

    let metadata = builds.pop().expect("non-empty").metadata;
    Ok(Model {
        builds: labels,
        structs,
        ids,
        descendants,
        enums,
        feature_names: used.into_iter().collect(),
        parent_features,
        metadata,
    })
}

fn merge_assignment(a: &FeatureAssignment, b: &FeatureAssignment) -> FeatureAssignment {
    use FeatureAssignment as A;
    let features = |x: &A| -> Vec<String> {
        match x {
            A::Single(f) => vec![f.clone()],
            A::Multi(fs) => fs.clone(),
            A::Core | A::Dormant => Vec::new(),
        }
    };
    if matches!(a, A::Core) || matches!(b, A::Core) {
        return A::Core;
    }
    let mut fs = features(a);
    fs.extend(features(b));
    fs.sort();
    fs.dedup();
    match fs.len() {
        0 => A::Dormant,
        1 => A::Single(fs.remove(0)),
        _ => A::Multi(fs),
    }
}

fn is_struct_kind(t: DataType) -> bool {
    matches!(
        t,
        DataType::Class | DataType::StrongPointer | DataType::WeakPointer
    )
}

fn is_pointer(t: DataType) -> bool {
    matches!(t, DataType::StrongPointer | DataType::WeakPointer)
}

/// Fold `other` into `base`. `Err` carries the two shapes when the field
/// cannot be represented by one type.
fn merge_field(
    base: &mut FieldDef,
    other: &FieldDef,
    is_ancestor: &dyn Fn(&str, &str) -> bool,
) -> Result<(), Vec<String>> {
    let describe = |f: &FieldDef| {
        format!(
            "{:?}{} -> {:?}",
            f.data_type,
            if f.is_array { "[]" } else { "" },
            f.target
        )
    };
    let same_kind = base.data_type == other.data_type
        || (is_pointer(base.data_type) && is_pointer(other.data_type));
    if !same_kind || base.is_array != other.is_array {
        return Err(vec![describe(base), describe(other)]);
    }
    if base.target == other.target {
        return Ok(());
    }
    if !is_struct_kind(base.data_type) {
        return Err(vec![describe(base), describe(other)]);
    }

    let targets = |t: &FieldTarget| -> Vec<String> {
        match t {
            FieldTarget::Struct(Some(n)) => vec![n.clone()],
            FieldTarget::Shape(ns) => ns.clone(),
            _ => Vec::new(),
        }
    };
    let (a, b) = (targets(&base.target), targets(&other.target));
    if b.is_empty() {
        return Ok(());
    }
    if a.is_empty() {
        base.target = other.target.clone();
        return Ok(());
    }
    // A pointer that moved within one hierarchy keeps the ancestor.
    if is_pointer(base.data_type)
        && let ([a1], [b1]) = (a.as_slice(), b.as_slice())
    {
        if is_ancestor(a1, b1) {
            return Ok(());
        }
        if is_ancestor(b1, a1) {
            base.target = other.target.clone();
            return Ok(());
        }
    }
    let mut all = a;
    for t in b {
        if !all.contains(&t) {
            all.push(t);
        }
    }
    base.target = FieldTarget::Shape(all);
    Ok(())
}

/// A type is compiled when any of its features is on, so every type it
/// names in a field must be compiled for at least those features too. Per
/// build the closure walk guarantees that; a field kept from another build
/// can break it, so close the union again.
fn propagate_features(structs: &mut BTreeMap<u32, StructDef>, ids: &HashMap<String, u32>) {
    use FeatureAssignment as A;
    loop {
        let mut additions: Vec<(u32, Vec<String>)> = Vec::new();
        for s in structs.values() {
            let fs: Vec<String> = match &s.assignment {
                A::Single(f) => vec![f.clone()],
                A::Multi(fs) => fs.clone(),
                A::Core | A::Dormant => continue,
            };
            for f in &s.fields {
                let names: Vec<&String> = match &f.target {
                    FieldTarget::Struct(Some(n)) => vec![n],
                    FieldTarget::Shape(ns) => ns.iter().collect(),
                    _ => continue,
                };
                for n in names {
                    let Some(&tid) = ids.get(n) else {
                        continue;
                    };
                    let target = &structs[&tid].assignment;
                    let missing: Vec<String> = match target {
                        A::Core => continue,
                        A::Dormant => fs.clone(),
                        A::Single(t) => fs.iter().filter(|x| *x != t).cloned().collect(),
                        A::Multi(ts) => fs.iter().filter(|x| !ts.contains(x)).cloned().collect(),
                    };
                    if !missing.is_empty() {
                        additions.push((tid, missing));
                    }
                }
            }
        }
        if additions.is_empty() {
            return;
        }
        for (tid, missing) in additions {
            let s = structs.get_mut(&tid).expect("target exists");
            let extra = match missing.as_slice() {
                [one] => A::Single(one.clone()),
                _ => A::Multi(missing),
            };
            s.assignment = merge_assignment(&s.assignment, &extra);
        }
    }
}

fn merge_enums(builds: &[BuildModel]) -> Vec<EnumDef> {
    let mut order: Vec<String> = Vec::new();
    let mut by_name: HashMap<String, EnumDef> = HashMap::new();
    for b in builds.iter().rev() {
        for e in &b.enums {
            match by_name.get_mut(&e.raw_name) {
                None => {
                    order.push(e.raw_name.clone());
                    by_name.insert(e.raw_name.clone(), e.clone());
                }
                Some(m) => {
                    for (v, bits) in &e.values {
                        match m.values.iter_mut().find(|(mv, _)| mv == v) {
                            Some(slot) => slot.1 |= bits,
                            None => m.values.push((v.clone(), *bits)),
                        }
                    }
                }
            }
        }
    }
    order
        .into_iter()
        .map(|n| by_name.remove(&n).expect("collected"))
        .collect()
}
