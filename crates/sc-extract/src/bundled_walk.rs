//! Bundled record walk — build several record indices in a single pass over
//! the DataCore, instead of each index re-iterating independently.
//!
//! # Why
//!
//! Each cooked index ([`crate::RecordPaths`], `sc-items`' `ItemCache`,
//! `sc-tags`' `TagTree`, …) is normally built by its own `X::build` call. When
//! a consumer needs several at once — and especially when one of them is an
//! all-records walker (paths, the reference graph) — running them separately
//! repeats work. A *bundled walk* fuses them: one `db.all_records()` pass
//! feeds every visitor, each accumulating into its own output.
//!
//! Each builder stays in its owning crate and keeps its standalone `X::build`;
//! the bundle is an **opt-in** fusion, not a mandatory god-function.
//!
//! # Interest-directed dispatch
//!
//! A visitor declares an [`Interest`] up front: a fixed set of struct types,
//! or every record. The driver resolves type names to `struct_index`es once,
//! then for each record calls only the visitors whose interest matches its
//! `struct_index`. Type-specific visitors thus pay only for the records they
//! asked for (plus the shared scan).
//!
//! # Full-pass only (for now)
//!
//! This driver always does a full `db.all_records()` scan. That is optimal
//! when any visitor wants [`Interest::AllRecords`] (one scan amortizes
//! everything). For a bundle of *only* small disjoint type-readers a full scan
//! is actually more record touches than the separate `X::build`s (which
//! iterate their pre-partitioned per-type maps directly) — so for that case
//! keep calling `X::build`. A "minimal pass" that visits only the needed
//! per-type maps would need generator support (the typed maps can't be
//! iterated generically, and svarog's `records_by_type` is itself a full
//! scan); it is deferred until profiling a real bundle shows the scan hurts.
//!
//! # Example
//!
//! ```ignore
//! let (items, tags, paths) = BundledWalk::new(&datacore)
//!     .run((ItemCacheBuilder::default(), TagTreeBuilder::default(), RecordPathsBuilder::default()));
//! ```

use crate::svarog_datacore::{DataCoreDatabase, Record};
use crate::{Datacore, Guid, RecordStore};

/// What records a [`RecordVisitor`] wants to see.
#[derive(Debug, Clone)]
pub enum Interest {
    /// Every record in the DataCore. Such a visitor runs only in a full pass
    /// (where [`VisitItem::raw`] is `Some`) and may read `raw`.
    AllRecords,
    /// Only records whose DCB struct type name is in this list. Resolved to a
    /// set of `struct_index`es once at the start of the walk. A `Types`
    /// visitor must NOT depend on [`VisitItem::raw`] (a future minimal pass
    /// would leave it `None`); it reads its typed struct via `store` instead.
    Types(&'static [&'static str]),
}

/// One record handed to a [`RecordVisitor`] during a walk.
pub struct VisitItem<'a> {
    /// The record's GUID.
    pub guid: Guid,
    /// The record's DCB struct type index.
    pub struct_index: u32,
    /// The materialized record store. A pool-reading visitor fetches its own
    /// typed struct from here via
    /// `RecordLookup::lookup(&store.records, &guid)` →
    /// `Handle::get(&store.pools)`.
    pub store: &'a RecordStore,
    /// The raw svarog record (carries `file_name` / `name` / `type_name` and
    /// the instance). `Some` in a full pass; reserved `None` for a future
    /// minimal pass. Only [`Interest::AllRecords`] visitors should read it.
    pub raw: Option<Record<'a>>,
}

/// A unit that accumulates over a record walk and produces one index.
///
/// Implemented by each builder (e.g. `ItemCacheBuilder`) in its owning crate.
/// Not object-safe (associated `Output` + by-value `finish`); use the tuple
/// [`VisitorSet`] form for static bundles.
pub trait RecordVisitor {
    /// The index this visitor produces.
    type Output;

    /// Declared once, before the walk. Drives which records reach [`Self::visit`].
    fn interest(&self) -> Interest;

    /// Called once per matching record. Accumulate into `&mut self`.
    fn visit(&mut self, item: VisitItem<'_>);

    /// Consume the visitor and produce its output.
    fn finish(self) -> Self::Output;
}

/// Resolved form of an [`Interest`]: a predicate over `struct_index`.
enum Matcher {
    All,
    Types(Vec<u32>),
}

impl Matcher {
    fn resolve(interest: Interest, db: &DataCoreDatabase) -> Self {
        match interest {
            Interest::AllRecords => Matcher::All,
            Interest::Types(names) => {
                let mut idxs = Vec::with_capacity(names.len());
                for i in 0..db.struct_definitions().len() {
                    if let Some(name) = db.struct_name(i)
                        && names.contains(&name)
                    {
                        idxs.push(i as u32);
                    }
                }
                Matcher::Types(idxs)
            }
        }
    }

    #[inline]
    fn matches(&self, struct_index: u32) -> bool {
        match self {
            Matcher::All => true,
            // Type lists are tiny (1–3 entries); linear scan beats a HashSet.
            Matcher::Types(v) => v.contains(&struct_index),
        }
    }
}

/// Driver for a bundled record walk over a [`Datacore`].
pub struct BundledWalk<'a> {
    datacore: &'a Datacore,
}

impl<'a> BundledWalk<'a> {
    /// Bind a walk to a parsed datacore.
    pub fn new(datacore: &'a Datacore) -> Self {
        Self { datacore }
    }

    /// Run every visitor in `set` in one pass and return their outputs as a
    /// tuple matching the input tuple's arity and element types.
    pub fn run<S: VisitorSet>(self, set: S) -> S::Output {
        set.run(self.datacore)
    }
}

/// A fixed, heterogeneous set of [`RecordVisitor`]s that can be run as one
/// bundled walk, returning a tuple of their outputs. Implemented for tuples up
/// to arity 8 by a macro — each element keeps its concrete type, so the output
/// tuple is fully typed with no boxing or downcasts.
pub trait VisitorSet {
    /// Tuple of the elements' [`RecordVisitor::Output`]s.
    type Output;

    /// Run the bundle over `datacore` and produce the output tuple.
    fn run(self, datacore: &Datacore) -> Self::Output;
}

macro_rules! impl_visitor_set {
    ($($V:ident $v:ident $m:ident),+) => {
        impl<$($V: RecordVisitor),+> VisitorSet for ($($V,)+) {
            type Output = ($($V::Output,)+);

            fn run(self, datacore: &Datacore) -> Self::Output {
                let ($(mut $v,)+) = self;
                let db = datacore.db();
                let store = datacore.records();
                // Resolve each visitor's interest to a struct_index matcher once.
                $(let $m = Matcher::resolve($v.interest(), db);)+
                // One full pass; dispatch each record to matching visitors.
                for record in db.all_records() {
                    let struct_index = record.struct_index();
                    let guid = record.id();
                    $(
                        if $m.matches(struct_index) {
                            $v.visit(VisitItem {
                                guid,
                                struct_index,
                                store,
                                raw: Some(record),
                            });
                        }
                    )+
                }
                ($($v.finish(),)+)
            }
        }
    };
}

impl_visitor_set!(V0 v0 m0);
impl_visitor_set!(V0 v0 m0, V1 v1 m1);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2, V3 v3 m3);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2, V3 v3 m3, V4 v4 m4);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2, V3 v3 m3, V4 v4 m4, V5 v5 m5);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2, V3 v3 m3, V4 v4 m4, V5 v5 m5, V6 v6 m6);
impl_visitor_set!(V0 v0 m0, V1 v1 m1, V2 v2 m2, V3 v3 m3, V4 v4 m4, V5 v5 m5, V6 v6 m6, V7 v7 m7);
