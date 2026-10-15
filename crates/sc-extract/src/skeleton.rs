//! `.chr` skeletons: the bones a character definition's parts and a weapon's
//! item ports attach to (`magAttach`, `sight_attachment`, …).
//!
//! The `CompiledBones` chunk comes in two layouts (StarBreaker's reader,
//! MIT): `0xC2011111` — bone count, then 68-byte entries (controller id,
//! limb id, parent `i32`, parent-relative and model-space `QuatT`), then the
//! names; `0xC201973C` — bone count, string-table size, two flag words, 32
//! bytes padding, 16-byte entries (parent `i16` at +8), the names, then all
//! parent-relative `QuatT`s and all model-space `QuatT`s. A `QuatT` is a
//! quaternion `(x, y, z, w)` then a translation, seven `f32`s.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let s = sc_extract::Skeleton::read(
//!     &assets,
//!     "objects/fps_weapons/weapons_v7/behr/rifle/p4ar/brfl_fps_behr_p4ar.chr",
//! )?;
//! println!("{:?}", s.bone("magAttach").map(|b| b.position));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::mesh::{chunk, read_data};
use crate::{AssetSource, Error, Result};

const BONES_V900: u32 = 0xC201_1111;
const BONES_V901: u32 = 0xC201_973C;

/// A skeleton's bones, in file order.
#[derive(Debug, Clone, PartialEq)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
}

/// One bone, placed in the model's space.
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    pub name: String,
    /// Index into [`Skeleton::bones`].
    pub parent: Option<usize>,
    /// Model-space position.
    pub position: [f32; 3],
    /// Model-space rotation, quaternion `(x, y, z, w)`.
    pub rotation: [f32; 4],
}

impl Skeleton {
    /// Read a `.chr` file.
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        Self::parse(&read_data(assets, path)?)
    }

    /// Parse `.chr` bytes.
    pub fn parse(file: &[u8]) -> Result<Self> {
        if let Ok(d) = chunk(file, BONES_V900) {
            return parse_v900(d);
        }
        if let Ok(d) = chunk(file, BONES_V901) {
            return parse_v901(d);
        }
        Err(bad("no CompiledBones chunk"))
    }

    /// The bone named `name` (case-insensitive).
    pub fn bone(&self, name: &str) -> Option<&Bone> {
        self.bones
            .iter()
            .find(|b| b.name.eq_ignore_ascii_case(name))
    }
}

fn bad(what: &str) -> Error {
    Error::Mesh(format!("skeleton: {what}"))
}

fn u32_at(d: &[u8], at: usize) -> Result<u32> {
    d.get(at..at + 4)
        .map(|b| u32::from_le_bytes(b.try_into().expect("4 bytes")))
        .ok_or_else(|| bad("truncated"))
}

/// `QuatT` at `at`: rotation `(x, y, z, w)`, translation.
fn quat_t(d: &[u8], at: usize) -> Result<([f32; 4], [f32; 3])> {
    let f = |i: usize| -> Result<f32> { Ok(f32::from_bits(u32_at(d, at + i * 4)?)) };
    Ok(([f(0)?, f(1)?, f(2)?, f(3)?], [f(4)?, f(5)?, f(6)?]))
}

fn names(b: &[u8], n: usize) -> Result<Vec<String>> {
    let out: Vec<String> = b
        .split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .take(n)
        .map(|s| String::from_utf8_lossy(s).into_owned())
        .collect();
    if out.len() < n {
        return Err(bad("fewer names than bones"));
    }
    Ok(out)
}

fn parent(p: i64, n: usize) -> Option<usize> {
    (p >= 0 && (p as usize) < n).then_some(p as usize)
}

fn parse_v900(d: &[u8]) -> Result<Skeleton> {
    let n = u32_at(d, 0)? as usize;
    if n == 0 || n > 4096 {
        return Err(bad("implausible bone count"));
    }
    let names_at = 4 + n * 68;
    let names = names(d.get(names_at..).ok_or_else(|| bad("truncated"))?, n)?;
    let bones = (0..n)
        .zip(names)
        .map(|(i, name)| {
            let e = 4 + i * 68;
            let (rotation, position) = quat_t(d, e + 12 + 28)?;
            Ok(Bone {
                name,
                parent: parent(u32_at(d, e + 8)? as i32 as i64, n),
                position,
                rotation,
            })
        })
        .collect::<Result<_>>()?;
    Ok(Skeleton { bones })
}

fn parse_v901(d: &[u8]) -> Result<Skeleton> {
    let n = u32_at(d, 0)? as usize;
    let table = u32_at(d, 4)? as usize;
    if n == 0 || n > 4096 {
        return Err(bad("implausible bone count"));
    }
    let entries = 48;
    let names_at = entries + n * 16;
    let names = names(
        d.get(names_at..names_at + table)
            .ok_or_else(|| bad("truncated"))?,
        n,
    )?;
    let world_at = names_at + table + n * 28;
    let bones = (0..n)
        .zip(names)
        .map(|(i, name)| {
            let e = entries + i * 16;
            let p = i16::from_le_bytes(
                d.get(e + 8..e + 10)
                    .ok_or_else(|| bad("truncated"))?
                    .try_into()
                    .expect("2 bytes"),
            );
            let (rotation, position) = quat_t(d, world_at + i * 28)?;
            Ok(Bone {
                name,
                parent: parent(p as i64, n),
                position,
                rotation,
            })
        })
        .collect::<Result<_>>()?;
    Ok(Skeleton { bones })
}
