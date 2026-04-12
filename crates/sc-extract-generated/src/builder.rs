//! The [`Builder`] — iterative materialiser for the flat-pool `DataPools`.
//!
//! Replaces the old recursive `FromInstance` drive: instead of letting the
//! call stack reflect DCB nesting depth, the builder owns a [`VecDeque`] of
//! pending instances and a dedup map keyed by native pool identity. Nested
//! allocations from inside [`Extract::extract`] calls reserve a slot, push
//! onto the queue, and return the slot index immediately — stack depth
//! stays O(1).
//!
//! See [`crate::extract`] for the trait side of this design and
//! [`crate::handle`] for the generic typed [`Handle<T>`] that each
//! `alloc_nested` call hands back.

use std::collections::{HashMap, VecDeque};

use svarog_common::CigGuid;
use svarog_datacore::{DataCoreDatabase, Instance};

use crate::extract::{Extract, reserve_pool_slot, store_in_pool};
use crate::handle::{Handle, Pooled};
use crate::{DataPools, RecordIndex, RecordStore};

/// A single deferred instance-materialisation job.
///
/// Stored in [`Builder::pending`] between when a nested instance is first
/// encountered and when [`Builder::drain`] gets around to processing it.
///
/// The `process` field is a monomorphised function pointer that encodes
/// the concrete type to extract. Each call to [`Builder::alloc_nested`]
/// is instantiated with a specific `T: Extract + Pooled`, and rustc
/// produces a distinct `process` function per instantiation — so we get
/// type dispatch without a giant generated match statement.
struct PendingSlot<'a> {
    slot: u32,
    inst: Instance<'a>,
    process: fn(&mut Builder<'a>, u32, Instance<'a>),
}

/// Iterative materialiser for the flat-pool record store.
///
/// Lifetime `'a` is the `&DataCoreDatabase` lifetime — every [`Instance`]
/// stored in the pending queue borrows through the same db, so the
/// builder can hold them without copying bytes out of the svarog-owned
/// buffer.
///
/// Typical usage:
///
/// ```ignore
/// let db = DataCoreDatabase::parse(&dcb_bytes)?;
/// let store = Builder::new(&db).consume_database().finish();
/// ```
pub struct Builder<'a> {
    /// The database every queued `Instance<'a>` borrows from.
    ///
    /// Exposed so that generated `Extract` impls can reach through to
    /// `db.instance(...)` when a field needs to materialise an inline
    /// Class element inside an array iterator.
    pub db: &'a DataCoreDatabase,

    /// Per-type `Vec<Option<T>>` storage. Freshly-reserved slots are
    /// `None` until the drain loop fills them.
    ///
    /// Exposed so that `Pooled::pool_mut` can reach every field.
    pub pools: DataPools,

    /// Per-record-type `HashMap<CigGuid, Handle<T>>` lookups, built
    /// alongside `pools` as we visit records from `db.all_records()`.
    pub records: RecordIndex,

    /// Dedup map for pool-backed instances. Key is `(struct_index,
    /// instance_index)` of the native svarog pool slot; value is the
    /// `u32` slot index we hand out to every caller that sees that
    /// identity.
    ///
    /// Only populated for `ClassRef` / `StrongPointer` / `WeakPointer`
    /// (i.e. sharable pool instances). Inline `Value::Class { .. }`
    /// elements are never deduped — each occurrence is a fresh byte
    /// range in the parent's data and has no stable identity.
    dedup: HashMap<(u32, u32), u32>,

    /// FIFO work queue of deferred instance materialisations.
    pending: VecDeque<PendingSlot<'a>>,
}

impl<'a> Builder<'a> {
    /// Create an empty builder bound to a database.
    pub fn new(db: &'a DataCoreDatabase) -> Self {
        Self {
            db,
            pools: DataPools::default(),
            records: RecordIndex::default(),
            dedup: HashMap::new(),
            pending: VecDeque::new(),
        }
    }

    /// Seed the queue from every top-level record in the database and
    /// then drain it to completion.
    pub fn consume_database(mut self) -> Self {
        // `seed_database` is emitted by the generator as an inherent
        // method — it's a name-based dispatcher that builds a per-run
        // `Vec<Option<fn>>` table by resolving each known record type's
        // runtime `struct_index` via `Extract::TYPE_NAME`.
        self.seed_database();
        self.drain();
        self
    }

    /// Finalize: assert the queue drained cleanly and return a populated
    /// [`RecordStore`].
    pub fn finish(self) -> RecordStore {
        debug_assert!(
            self.pending.is_empty(),
            "Builder::finish called with {} pending slots still unprocessed",
            self.pending.len()
        );
        RecordStore {
            pools: self.pools,
            records: self.records,
        }
    }

    /// Allocate a slot for a nested (non-record) instance, deduping
    /// pool instances by their native `(struct_index, instance_index)`
    /// identity, and return a typed [`Handle<T>`].
    ///
    /// `from_pool` must be `true` when the caller obtained `inst` from a
    /// `ClassRef` / `StrongPointer` / `WeakPointer` (dedupable), and
    /// `false` when it came from an inline `Value::Class { .. }` (each
    /// occurrence is unique).
    pub fn alloc_nested<T>(&mut self, inst: Instance<'a>, from_pool: bool) -> Handle<T>
    where
        T: Extract<'a> + Pooled + 'static,
    {
        if from_pool {
            let key = (inst.struct_index(), inst.instance_index());
            if let Some(&slot) = self.dedup.get(&key) {
                return Handle::new(slot);
            }
            let slot = reserve_pool_slot::<T>(&mut self.pools);
            self.dedup.insert(key, slot);
            self.pending.push_back(PendingSlot {
                slot,
                inst,
                process: Self::process_one::<T>,
            });
            Handle::new(slot)
        } else {
            let slot = reserve_pool_slot::<T>(&mut self.pools);
            self.pending.push_back(PendingSlot {
                slot,
                inst,
                process: Self::process_one::<T>,
            });
            Handle::new(slot)
        }
    }

    /// Allocate a slot for a top-level record, register it in the
    /// dedup map so later StrongPointer references share the same
    /// slot, and return its raw index.
    ///
    /// Called from the generated `seed_database` dispatcher, once per
    /// record in `db.all_records()`. Returns a raw `u32` instead of
    /// `Handle<T>` so the dispatcher's per-record adapter can wrap it
    /// in the right `Handle<_>` locally.
    pub fn alloc_record<T>(&mut self, inst: Instance<'a>, _guid: CigGuid) -> u32
    where
        T: Extract<'a> + Pooled + 'static,
    {
        let slot = reserve_pool_slot::<T>(&mut self.pools);
        let key = (inst.struct_index(), inst.instance_index());
        self.dedup.insert(key, slot);
        self.pending.push_back(PendingSlot {
            slot,
            inst,
            process: Self::process_one::<T>,
        });
        slot
    }

    /// Drain the pending queue.
    ///
    /// Each pop runs one deferred `T::extract` call, which may push
    /// further pending slots onto the queue as it encounters nested
    /// struct fields. Loops until the queue is empty.
    pub fn drain(&mut self) {
        while let Some(pending) = self.pending.pop_front() {
            (pending.process)(self, pending.slot, pending.inst);
        }
    }

    /// Monomorphised per-`T` processor. The `fn` pointer stored in
    /// `PendingSlot::process` is one of these, specialised to the
    /// concrete `T` that was passed at alloc time.
    fn process_one<T>(b: &mut Builder<'a>, slot: u32, inst: Instance<'a>)
    where
        T: Extract<'a> + Pooled,
    {
        let value = T::extract(&inst, b);
        store_in_pool::<T>(&mut b.pools, slot, value);
    }
}
