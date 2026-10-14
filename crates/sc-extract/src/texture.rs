//! DDS textures as the archive stores them: split across sibling files.
//!
//! `x.dds` holds the header and the smallest mips; `x.dds.1` … `x.dds.8`
//! hold one larger mip each, the highest number being mip 0. A normal map
//! with gloss (`_ddna`) keeps its gloss as a second texture (BC4) split the
//! same way: `x.dds.a` (header + smallest mips; usually without the `DDS `
//! magic) and `x.dds.1a` … `x.dds.8a`.
//! [`Texture::read`] reassembles both into ordinary DDS files — header, then
//! every mip largest first — that any DDS decoder reads. Decoding the block
//! formats (BC1 … BC7) is the consumer's business.
//!
//! Material files name textures `.tif`; the archive has the `.dds`.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let t = sc_extract::Texture::read(
//!     &assets,
//!     "objects/buildingsets/human/universal/prop/dressing/can/textures/can_drink_3_a_diff.tif",
//! )?;
//! println!("{}x{} {:?}", t.width(), t.height(), t.format());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::{AssetSource, Error, Result};

const MAGIC: &[u8; 4] = b"DDS ";

/// A reassembled DDS texture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Texture {
    /// Complete DDS file, every mip.
    pub dds: Vec<u8>,
    /// The separately stored alpha / gloss texture (`.a`), complete; `None`
    /// when the texture has none.
    pub alpha: Option<Vec<u8>>,
}

/// A DDS pixel format, as the header names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// Legacy `FourCC` (`DXT1`, `DXT5`, `ATI2`, …).
    FourCc([u8; 4]),
    /// `DX10` extended header: the `DXGI_FORMAT` number.
    Dxgi(u32),
    /// Uncompressed, with its bit count.
    Uncompressed(u32),
}

impl Texture {
    /// Read the texture an authored path names (`.tif`, `.dds` or none),
    /// with its split siblings.
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        let base = dds_path(path);
        let main = crate::mesh::read_data(assets, &base)?;
        let dds = merge(&main, |n| {
            crate::mesh::read_data(assets, &format!("{base}.{n}")).ok()
        })?;
        let alpha = match crate::mesh::read_data(assets, &format!("{base}.a")) {
            Ok(a) => {
                // The `.a` base usually omits the `DDS ` magic and starts
                // straight at the 124-byte header.
                let a = if a.starts_with(MAGIC) {
                    a
                } else if a.starts_with(&124u32.to_le_bytes()) {
                    [MAGIC.as_slice(), &a].concat()
                } else {
                    return Ok(Self { dds, alpha: None });
                };
                Some(merge(&a, |n| {
                    crate::mesh::read_data(assets, &format!("{base}.{n}a")).ok()
                })?)
            }
            Err(_) => None,
        };
        Ok(Self { dds, alpha })
    }

    pub fn width(&self) -> u32 {
        u32_at(&self.dds, 16)
    }

    pub fn height(&self) -> u32 {
        u32_at(&self.dds, 12)
    }

    pub fn mip_count(&self) -> u32 {
        u32_at(&self.dds, 28).max(1)
    }

    pub fn format(&self) -> TextureFormat {
        let four_cc: [u8; 4] = self.dds[84..88].try_into().expect("checked header");
        if &four_cc == b"DX10" {
            TextureFormat::Dxgi(u32_at(&self.dds, 128))
        } else if four_cc == [0; 4] {
            TextureFormat::Uncompressed(u32_at(&self.dds, 88))
        } else {
            TextureFormat::FourCc(four_cc)
        }
    }
}

/// The archive path of the `.dds` an authored texture path means.
pub(crate) fn dds_path(path: &str) -> String {
    let path = path.replace('\\', "/");
    let stem = match path.rsplit_once('.') {
        Some((stem, ext)) if !ext.contains('/') => stem.to_string(),
        _ => path,
    };
    format!("{stem}.dds")
}

/// Header + sibling mips (`.8` … `.1`, largest first) + the base file's
/// remaining small mips.
fn merge(base: &[u8], sibling: impl Fn(u32) -> Option<Vec<u8>>) -> Result<Vec<u8>> {
    if base.len() < 128 || !base.starts_with(MAGIC) {
        return Err(Error::Texture("not a DDS file".into()));
    }
    let header = if &base[84..88] == b"DX10" { 148 } else { 128 };
    if base.len() < header {
        return Err(Error::Texture("truncated header".into()));
    }
    let mut out = base[..header].to_vec();
    for n in (1..=8).rev() {
        if let Some(mip) = sibling(n) {
            out.extend_from_slice(&mip);
        }
    }
    out.extend_from_slice(&base[header..]);
    Ok(out)
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    b.get(at..at + 4)
        .map(|s| u32::from_le_bytes(s.try_into().expect("4 bytes")))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(four_cc: &[u8; 4]) -> Vec<u8> {
        let mut h = vec![0u8; 128];
        h[..4].copy_from_slice(MAGIC);
        h[12..16].copy_from_slice(&64u32.to_le_bytes());
        h[16..20].copy_from_slice(&32u32.to_le_bytes());
        h[84..88].copy_from_slice(four_cc);
        h
    }

    #[test]
    fn tif_names_resolve_to_dds() {
        assert_eq!(dds_path("Objects/a/b_diff.tif"), "Objects/a/b_diff.dds");
        assert_eq!(
            dds_path("Textures\\defaults\\white.dds"),
            "Textures/defaults/white.dds"
        );
        assert_eq!(dds_path("objects/no_ext"), "objects/no_ext.dds");
    }

    #[test]
    fn siblings_come_before_the_base_mips_largest_first() {
        let mut base = header(b"DXT1");
        base.extend_from_slice(b"small");
        let merged = merge(&base, |n| match n {
            2 => Some(b"MIP0".to_vec()),
            1 => Some(b"MIP1".to_vec()),
            _ => None,
        })
        .unwrap();
        assert_eq!(&merged[128..], b"MIP0MIP1small");
        let t = Texture {
            dds: merged,
            alpha: None,
        };
        assert_eq!((t.width(), t.height()), (32, 64));
        assert_eq!(t.format(), TextureFormat::FourCc(*b"DXT1"));
    }

    #[test]
    fn rejects_non_dds() {
        assert!(merge(b"nope", |_| None).is_err());
    }
}
