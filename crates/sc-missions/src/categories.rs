//! Mission-type (category) registry — `MissionType` GUID → display name + icon.
//!
//! A contract's category (SCMDB's "Mission Type" column — Bounty Hunter,
//! Hauling, Mercenary, Salvage, …) lives on its template:
//! `ContractTemplate.contractDisplayInfo → ContractDisplayInfo.type
//! (Reference) → MissionType`. The `type` reference is a bare GUID; this
//! registry resolves it to a localizable name + icon paths.
//!
//! `LocaleMap`-free like the other registries: stores the [`LocaleKey`],
//! consumer resolves at the call site. `MissionType` is reachable under the
//! `contracts` sc-extract feature (no extra gate needed).

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, LocaleKey};

/// One resolved mission category.
#[derive(Debug, Clone)]
pub struct MissionTypeInfo {
    /// `MissionType.LocalisedTypeName` — resolve against a `LocaleMap` for the
    /// player-facing category name.
    pub name_key: LocaleKey,
    /// `MissionType.IconName` — UI icon identifier (may be empty).
    pub icon_name: String,
    /// `MissionType.svgIconPath` — vector icon asset path (may be empty).
    pub svg_icon_path: String,
}

/// `MissionType` GUID → [`MissionTypeInfo`].
#[derive(Debug, Clone, Default)]
pub struct MissionTypes {
    by_guid: HashMap<Guid, MissionTypeInfo>,
}

impl MissionTypes {
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let mut by_guid = HashMap::new();
        for (guid, handle) in &datacore.records().records.multi_feature.mission_type {
            if let Some(mt) = handle.get(pools) {
                by_guid.insert(
                    *guid,
                    MissionTypeInfo {
                        name_key: mt.localised_type_name.clone(),
                        icon_name: mt.icon_name.clone(),
                        svg_icon_path: mt.svg_icon_path.clone(),
                    },
                );
            }
        }
        Self { by_guid }
    }

    pub fn get(&self, guid: &Guid) -> Option<&MissionTypeInfo> {
        self.by_guid.get(guid)
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}
