//! Star Citizen contract / mission data.
//!
//! Star Citizen contains roughly 2400 player-visible contracts, generated at
//! runtime by a few hundred **contract generators**. Generators reference
//! data all over the DCB via tags, GUIDs, and nested references, which makes
//! the system substantially more complex than other domains and is the
//! primary reason this crate exists.
//!
//! # Two-layer design
//!
//! This crate exposes the contract system in two distinct layers. The split
//! is deliberate: it lets the crate ship useful value even before the
//! higher-level understanding is complete.
//!
//! ## Raw layer
//!
//! Parsed-but-uninterpreted. Generators, objectives, rewards, and
//! references expressed close to their DCB shape (after cross-reference
//! resolution from `sc-extract`). This layer is deliberately thin and
//! opinion-free, so it can be kept correct even while the higher-level
//! understanding is still evolving.
//!
//! The raw layer is the safe fallback. Consumers that need something the
//! model layer does not yet cover should reach into the raw layer for that
//! slice rather than pressuring the model layer to grow half-understood
//! types.
//!
//! ## Model layer
//!
//! Clean domain types — what a "contract" *means* after generator
//! expansion, objective resolution, and reward normalization. This layer is
//! allowed to be incomplete and to evolve. Every type added here should
//! correspond to real, verified understanding of how the generator system
//! behaves, not a guess from inspecting one example contract.
//!
//! # Driving consumer
//!
//! `sc-langpatch` is the primary driver. Its contract-annotation work
//! motivates this crate's existence — the complexity encountered while
//! parsing contracts for localization enrichment is the concrete problem
//! the model layer is meant to solve.
//!
//! # Status
//!
//! Both layers are stubs. The raw layer should land first and stabilize
//! before any model-layer types are introduced. Until the raw layer is
//! solid, consumers can reach through `sc_extract::svarog` for ad-hoc
//! access — that is what the escape hatch is for.
