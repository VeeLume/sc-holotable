//! The [`Extract`] trait — converts a svarog `Instance` into an owned,
//! typed value via a [`Builder`].
//!
//! # Design
//!
//! Every generated struct in this crate implements [`Extract`] **and**
//! [`crate::Pooled`]. [`Extract`] says "how to decode one of me from an
//! `Instance`", and [`crate::Pooled`] says "where to put me in
//! [`crate::DataPools`]". Splitting them keeps the generated per-type
//! code minimal: `Extract::extract` is the only body rustc has to
//! type-check per struct — the allocation, slot-tracking, and dedup
//! logic all live on the generic [`Builder`] side, behind [`Pooled`].
//!
//! Nested Class / StrongPointer / WeakPointer fields call
//! [`Builder::alloc_nested`] from inside an `extract` body. The builder
//! reserves a slot in the target type's pool, enqueues the nested
//! instance for later processing, and returns a typed [`Handle`]
//! immediately. The drain loop walks the queue iteratively, so stack
//! depth stays O(1) regardless of DCB nesting depth.
//!
//! # Patch resilience
//!
//! [`Extract::TYPE_NAME`] is the **DCB schema name** of this type —
//! `Builder::seed_database` uses it to resolve the runtime `struct_index`
//! of every known record type by name against the live DCB. A game
//! patch that adds or reorders struct types does not silently drop
//! records: existing types keep their dispatch even when their runtime
//! index shifts.

use svarog_datacore::Instance;

use crate::builder::Builder;
use crate::handle::Pooled;

/// Convert a svarog [`Instance`] into an owned, typed value via a [`Builder`].
///
/// # Implementor contract
///
/// - Missing primitive fields should produce default values
///   (`unwrap_or_default`) rather than failing. The intent is that
///   partial data in the DCB (new fields CIG added in a patch we haven't
///   regenerated for) never causes the whole snapshot to fail.
/// - For every nested Class / StrongPointer / WeakPointer field, call
///   [`Builder::alloc_nested`] with the nested [`Instance`] and store
///   the returned [`Handle<Self>`](crate::Handle) in the outer struct's
///   corresponding field.
/// - Every `impl Extract for T` requires a matching `impl Pooled for T`
///   (enforced by the `Self: Pooled` bound). The generator emits both in
///   the same bucket file so the two stay in sync.
pub trait Extract<'a>: Sized + Pooled {
    /// The DCB schema name of this type — e.g. `"EntityClassDefinition"`
    /// or `"SCItemWeaponComponentParams"`. Used by
    /// `Builder::seed_database` to resolve the runtime `struct_index`
    /// **by name** against the live DCB instead of baking schema
    /// positions at codegen time.
    const TYPE_NAME: &'static str;

    /// Build a value of `Self` from a svarog instance view.
    ///
    /// Nested struct allocations go through `b`; primitive fields are
    /// read directly from `inst`.
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self;
}

/// Reserve a fresh slot in `T`'s pool inside `pools` and return its
/// index. The slot starts as `None` and will be overwritten by
/// [`store_in_pool`] once the drain loop processes the queued instance.
#[inline]
pub(crate) fn reserve_pool_slot<T: Pooled>(pools: &mut crate::DataPools) -> u32 {
    let pool = T::pool_mut(pools);
    let slot = pool.len() as u32;
    pool.push(None);
    slot
}

/// Overwrite a previously-reserved slot in `T`'s pool with a concrete
/// value. The inverse of [`reserve_pool_slot`].
#[inline]
pub(crate) fn store_in_pool<T: Pooled>(pools: &mut crate::DataPools, slot: u32, value: T) {
    T::pool_mut(pools)[slot as usize] = Some(value);
}

