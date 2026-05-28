//! Generic typed handle into [`DataPools`].
//!
//! Replaces the per-type `{Name}Id(pub u32)` newtypes that an earlier draft
//! of the flat-pool refactor emitted. A single generic [`Handle<T>`] — one
//! declaration, one set of blanket impls — does the same job as ~6,500
//! per-type newtypes, with two important wins:
//!
//! 1. **Compile-time cost.** Eliminating ~6,500 struct definitions, ~6,500
//!    `impl Index<TId>` blocks, and ~6,500 inherent `impl TId { get }`
//!    blocks drops rustc front-end work by roughly half. At the scale of
//!    `sc-extract-generated` that's the difference between "fits in the
//!    dev profile budget" and "link-time symbol storms".
//! 2. **Type safety stays.** `Handle<T>` carries `T` in a `PhantomData`,
//!    so a `Handle<Weapon>` cannot accidentally be passed where a
//!    `Handle<Component>` is expected. Getting the value back still
//!    goes through [`Handle::get`] → [`Pooled::pool`], which is only
//!    implemented once per concrete type.
//!
//! Handles are `Copy` and erase their type parameter at runtime — the
//! stored `u32` is just the pool slot index.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use svarog_common::CigGuid;

use crate::{DataPools, RecordIndex};

/// Describes how to find a type's pool inside [`DataPools`].
///
/// One generated impl per emitted struct — two one-liners each. Replaces
/// the old per-type `impl Index<TId>` + inherent `get` triplet with a
/// single trait the blanket impls can dispatch through.
pub trait Pooled: Sized {
    /// Borrow the slice of `Vec<Option<Self>>` inside `pools` that holds
    /// values of this type.
    fn pool(pools: &DataPools) -> &Vec<Option<Self>>;

    /// Mutably borrow the same slice.
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>>;
}

/// Resolve a record's `CigGuid` to its typed [`Handle`] via the
/// [`RecordIndex`].
///
/// One generated impl per *seeded record type* — the ~600 top-level DCB
/// record types that get a `CigGuid -> Handle<T>` map in [`RecordIndex`].
/// This is the typed counterpart to `db.record(guid)`: it turns a bare
/// `Reference` GUID (emitted as `Option<CigGuid>` on generated structs)
/// back into a pool handle, so consumers can stay on the typed surface
/// across cross-record reference hops instead of dropping to the raw
/// svarog layer.
///
/// Non-record struct types (reached through their parent's [`Handle`]) and
/// record types the generator never seeded do **not** implement this — by
/// design, those are the genuine raw-only frontier. The
/// `T: RecordLookup` bound at a call site is therefore a compile-time
/// guarantee that `T` is GUID-addressable.
pub trait RecordLookup: Pooled {
    /// The handle for the record of this type carrying `guid`, or `None`
    /// if no such record exists.
    fn lookup(index: &RecordIndex, guid: &CigGuid) -> Option<Handle<Self>>;
}

/// A typed slot index into [`DataPools`].
///
/// Phantom-typed in `T` so `Handle<Weapon>` and `Handle<Component>` are
/// distinct at the type level. The stored `u32` is the index into the
/// corresponding `Vec<Option<T>>` pool.
pub struct Handle<T>(u32, PhantomData<fn() -> T>);

impl<T> Handle<T> {
    /// Wrap a raw `u32` slot index.
    #[inline]
    pub const fn new(slot: u32) -> Self {
        Handle(slot, PhantomData)
    }

    /// The underlying slot index (useful for diagnostics or direct `Vec`
    /// access; prefer [`Handle::get`] for typed access).
    #[inline]
    pub const fn slot(self) -> u32 {
        self.0
    }
}

impl<T: Pooled> Handle<T> {
    /// Read the referenced `T` from `pools`.
    ///
    /// Returns `None` if the slot was reserved but never filled (which
    /// shouldn't happen for a cleanly-finished [`crate::Builder`] run but
    /// is still preferable to a panic — malformed snapshots deserve a
    /// graceful miss).
    #[inline]
    pub fn get(self, pools: &DataPools) -> Option<&T> {
        T::pool(pools).get(self.0 as usize).and_then(|o| o.as_ref())
    }
}

// ── Per-T blanket impls that would otherwise be per-type newtype impls ───

impl<T: Pooled> std::ops::Index<Handle<T>> for DataPools {
    type Output = Option<T>;
    #[inline]
    fn index(&self, id: Handle<T>) -> &Self::Output {
        &T::pool(self)[id.0 as usize]
    }
}

// ── Manual trait impls (can't #[derive] with a PhantomData<fn() -> T>) ──

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Handle<T> {
    #[inline]
    fn default() -> Self {
        Handle(0, PhantomData)
    }
}

impl<T> PartialEq for Handle<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Include the type name so trace logs disambiguate different
        // `Handle<_>` instances.
        write!(
            f,
            "Handle<{}>({})",
            std::any::type_name::<T>()
                .rsplit("::")
                .next()
                .unwrap_or("?"),
            self.0
        )
    }
}
