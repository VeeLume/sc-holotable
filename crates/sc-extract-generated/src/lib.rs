//! Generated DataCore schema bindings for `sc-extract`.
//!
//! This crate is **workspace-internal**. Public consumers should depend on
//! `sc-extract`, which re-exports everything here. We keep the generated
//! output in its own crate so that:
//!
//! 1. Cold release builds only have to LLVM-optimize the generated code
//!    once and cache the result. Edits to the hand-written `sc-extract`
//!    modules (graph, tags, etc.) no longer invalidate the generated
//!    object files.
//! 2. The workspace Cargo.toml can apply a per-crate profile override —
//!    we build this crate at `opt-level = 1` because the expensive LLVM
//!    passes on the `impl Extract` match trees dominate build time at
//!    `opt-level = 3` for virtually no runtime benefit on the
//!    load-once-walk-everything hot path.
//!
//! # Flat-pool architecture (summary)
//!
//! Nested `Class` / `StrongPointer` / `WeakPointer` fields are **not**
//! stored inline as `Option<Box<T>>` — they are stored as
//! [`Handle<T>`](Handle) values (type-safe `u32` indices) into per-type
//! `Vec<Option<T>>` pools living in [`DataPools`]. Top-level records
//! live in the same pools; a parallel [`RecordIndex`] maps
//! `CigGuid -> Handle<T>` for them.
//!
//! This matches the way DCB actually stores struct pools and lets
//! materialisation run iteratively via [`Builder`] instead of recursing
//! through the call stack.
//!
//! Only DCB struct types **reachable from records** (transitively,
//! through Class/StrongPointer/WeakPointer fields) are emitted. Struct
//! definitions that no record transitively references are pruned at
//! generation time — see `tools/sc-generator/src/emit.rs`.

#![allow(clippy::too_many_arguments)]

mod builder;
mod extract;
pub mod generated;
mod handle;

pub use builder::Builder;
pub use extract::Extract;
pub use handle::{Handle, Pooled};

// Flat re-exports of every generated type, so `sc_extract_generated::Foo`
// works the same as `sc_extract_generated::generated::Foo`.
pub use generated::*;
