//! Tint palettes (`TintPaletteTree` records): the three colour entries a
//! LayerBlend layer's `PaletteTint` 1 / 2 / 3 picks from, plus glass and
//! decal colours.

use std::collections::HashMap;

use sc_extract::generated::{Handle, SRGB8, TintEntry, TintPalette};
use sc_extract::{DataPools, Datacore, Guid, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::PaletteRef;

/// One `TintPalette` node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    /// Entries A, B, C (`PaletteTint` 1, 2, 3).
    pub entries: [PaletteEntry; 3],
    /// `glassColor`, sRGB bytes.
    pub glass: Option<[u8; 3]>,
    /// `decalTexture`; empty when the palette has none.
    pub decal_texture: String,
    /// `decalColorR` / `G` / `B`, sRGB bytes.
    pub decal_colors: [Option<[u8; 3]>; 3],
    /// Named children a [`PaletteRef::child`] selects.
    pub children: Vec<Palette>,
}

/// A `TintEntry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PaletteEntry {
    /// `tintColor`, sRGB bytes; multiplies the layer colour.
    pub tint: [u8; 3],
    /// `specColor`, sRGB bytes; replaces the F0 of a layer whose `TintMode`
    /// tints the specular (0 or 2).
    pub spec: [u8; 3],
    /// 0..=255; multiplies the layer's gloss (as `glossiness / 255`).
    pub glossiness: u8,
}

/// Every `TintPaletteTree` record's root palette, keyed by record GUID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Palettes {
    by_guid: HashMap<Guid, Palette>,
}

impl Palettes {
    /// The empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let by_guid = store
            .records
            .multi_feature
            .tint_palette_tree
            .iter()
            .filter_map(|(&guid, &h)| {
                let tree = h.get(pools)?;
                Some((guid, palette(tree.root?, pools)?))
            })
            .collect();
        Self { by_guid }
    }

    /// The palette a [`PaletteRef`] points at: the record's root, or its
    /// child named [`PaletteRef::child`] (case-insensitive).
    pub fn resolve(&self, r: &PaletteRef) -> Option<&Palette> {
        let root = self.by_guid.get(&r.record)?;
        if r.child.is_empty() {
            return Some(root);
        }
        root.children
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(&r.child))
    }
}

impl RecordCollection for Palettes {
    type Item = Palette;

    fn get(&self, guid: &Guid) -> Option<&Palette> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Palette)> + '_ {
        self.by_guid.iter()
    }
}

fn srgb(h: Option<Handle<SRGB8>>, pools: &DataPools) -> Option<[u8; 3]> {
    let c = h?.get(pools)?;
    Some([c.r, c.g, c.b].map(|v| v.min(255) as u8))
}

fn entry(h: Option<Handle<TintEntry>>, pools: &DataPools) -> PaletteEntry {
    let Some(e) = h.and_then(|h| h.get(pools)) else {
        return PaletteEntry::default();
    };
    PaletteEntry {
        tint: srgb(e.tint_color, pools).unwrap_or_default(),
        spec: srgb(e.spec_color, pools).unwrap_or_default(),
        glossiness: e.glossiness.min(255) as u8,
    }
}

fn palette(h: Handle<TintPalette>, pools: &DataPools) -> Option<Palette> {
    let p = h.get(pools)?;
    Some(Palette {
        name: p.name.clone(),
        entries: [
            entry(p.entry_a, pools),
            entry(p.entry_b, pools),
            entry(p.entry_c, pools),
        ],
        glass: srgb(p.glass_color, pools),
        decal_texture: p.decal_texture.clone(),
        decal_colors: [
            srgb(p.decal_color_r, pools),
            srgb(p.decal_color_g, pools),
            srgb(p.decal_color_b, pools),
        ],
        children: p
            .children
            .iter()
            .filter_map(|c| palette(*c, pools))
            .collect(),
    })
}
