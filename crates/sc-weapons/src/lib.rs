//! Canonical weapon and damage model for Star Citizen.
//!
//! Every consumer that cares about weapons — regardless of whether they need
//! two fields or two hundred — reads them through this crate. There is one
//! shared weapon type; consumers pick the fields they care about from it.
//!
//! # Driving consumer
//!
//! `bulkhead` is the forcing function for correctness. Its damage simulator
//! needs complete, accurate weapon data (projectile ballistics, falloff,
//! heat curves, ammo references, ...), so this crate is designed around its
//! needs first. `sc-langpatch` consumes a much smaller subset (display
//! names, class / size / grade for localization enrichment) and simply reads
//! fewer fields off the same canonical type.
//!
//! This inversion matters: the crate is driven by the *most demanding*
//! consumer, not the easiest one. Consumers that need less adapt downward;
//! they do not get their own narrower type.
//!
//! # Correctness note
//!
//! `sc-langpatch`'s current in-app weapon parser is incomplete and partially
//! incorrect. Switching it to `sc-weapons` is the path to fixing it.
//!
//! **Do not** port sc-langpatch's current assumptions into this crate
//! without verifying them against real DCB data first. See the "go slow"
//! principle in the workspace README: it is better to model nothing than to
//! bake a wrong assumption into a type that three apps depend on.
