//! Which jump point leads to which, across systems.
//!
//! Each jump point's object container
//! (`objectcontainers/pu/system/<system>/jumppoints/*.socpak`) holds the
//! jump point entity, whose `SJumpPointParams.jumpGateSuperGuid` is a path
//! of four `EntityCryGUID`s, dot-joined. The second is the target system's
//! star placement, the third the target jump point's placement in that
//! system's container, which names the target's `StarMapObject`. So a link
//! joins two `StarMapObject`s by GUID alone.
//!
//! On LIVE 4.10, Stanton–Pyro and Pyro–Nyx link both ways, and the jump
//! points named Stanton–Magnus and Nyx–Castra link to each other (the
//! temporary Stanton–Nyx route). Whether a link is open is not in the data:
//! callers decide.
//!
//! ```no_run
//! use sc_extract::AssetSource;
//! use sc_locations::{JumpLinks, RecordCollection};
//!
//! let install = sc_discovery::discover_primary()?;
//! let assets = AssetSource::from_install(&install)?;
//! for link in JumpLinks::build(&assets)?.values() {
//!     println!("{} -> {}", link.entry, link.exit);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;

use sc_extract::{AssetSource, Guid, RecordCollection, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::object_containers::ObjectContainers;

/// One jump point and the jump point it leads to, both `StarMapObject`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct JumpLink {
    /// The jump point you enter.
    pub entry: Guid,
    /// The jump point you leave from, in the other system.
    pub exit: Guid,
}

/// Every jump link, by entry jump point.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JumpLinks {
    by_entry: HashMap<Guid, JumpLink>,
}

impl JumpLinks {
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the systems' object containers and the jump points one level
    /// down. A link whose target placement can't be resolved is skipped
    /// with a warning. Empty for a snapshot-backed [`AssetSource`].
    pub fn build(assets: &AssetSource) -> Result<Self> {
        let (graph, found) = ObjectContainers::systems_with(assets, 1, |_, _, root| {
            root.descendants()
                .filter_map(|n| n.attr("jumpGateSuperGuid"))
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })?;
        let mut out = Self::new();
        for (socpak, supers) in found {
            let entries: Vec<Guid> = graph
                .nesting(&socpak)
                .iter()
                .filter(|&&id| graph.get(id).socpak().ends_with("system.socpak"))
                .filter_map(|&id| graph.get(id).starmap_guid())
                .collect();
            for super_guid in supers {
                let Some(exit) = exit_of(&graph, &super_guid) else {
                    warn!(%socpak, %super_guid, "jump gate target not resolved");
                    continue;
                };
                for &entry in &entries {
                    out.insert(JumpLink { entry, exit });
                }
            }
        }
        info!(links = out.len(), "built jump links");
        Ok(out)
    }

    /// Insert or replace a link.
    pub fn insert(&mut self, link: JumpLink) {
        self.by_entry.insert(link.entry, link);
    }
}

/// The target jump point of a `jumpGateSuperGuid`: the placement named by
/// its third part, in the system container that holds the star its second
/// part names.
fn exit_of(graph: &ObjectContainers, super_guid: &str) -> Option<Guid> {
    let parts: Vec<&str> = super_guid.split('.').collect();
    let [_, star, target, _] = parts.as_slice() else {
        return None;
    };
    let systems: Vec<&str> = graph
        .by_cry(star)
        .map(|id| graph.get(id).socpak())
        .filter(|s| s.ends_with("system.socpak"))
        .collect();
    graph
        .by_cry(target)
        .map(|id| graph.get(id))
        .find(|p| systems.contains(&p.socpak()))?
        .starmap_guid()
}

impl RecordCollection for JumpLinks {
    type Item = JumpLink;

    fn get(&self, guid: &Guid) -> Option<&JumpLink> {
        self.by_entry.get(guid)
    }

    fn len(&self) -> usize {
        self.by_entry.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &JumpLink)> + '_ {
        self.by_entry.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_round_trip() {
        let link = JumpLink {
            entry: Guid::from_bytes([1; 16]),
            exit: Guid::from_bytes([2; 16]),
        };
        let mut links = JumpLinks::new();
        links.insert(link);
        let back: JumpLinks =
            serde_json::from_str(&serde_json::to_string(&links).unwrap()).unwrap();
        assert_eq!(back.get(&link.entry), Some(&link));
        assert!(back.get(&link.exit).is_none());
    }
}
