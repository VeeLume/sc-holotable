//! Generic Star Citizen object-container access.
//!
//! This is a **format tool**, the object-container sibling of [`crate::Datacore`]
//! (the DCB reader). It knows the *containers* — `.socpak` (a ZIP), `.soc`/`.pla`
//! (a `CrCh` chunk file embedding a CryXmlB entity tree), and `pivot.entxml`
//! (plain XML or raw CryXmlB) — and decodes them into a generic [`XmlNode`] tree.
//!
//! It deliberately knows **nothing** about what the entities/components *mean*.
//! Interpreting a specific component (e.g. finding a `HarvestableProviderComponent`
//! and reading its `preset` GUID) is a domain concern and lives in the consuming
//! crate, which walks the [`XmlNode`] tree this module hands back.
//!
//! # Container shapes
//!
//! ```text
//! body .socpak (ZIP)  ──member──▶  pivot.entxml         (plain XML | CryXmlB)
//!                     └─member──▶  *.soc / *.pla        (CrCh chunk file)
//!                                     └─ CRYXMLB chunk (0x0004)  ──▶  CryXmlB
//! ```
//!
//! # Example
//!
//! ```no_run
//! use sc_extract::object_container::{self, Socpak};
//!
//! let mut pak = Socpak::open(std::fs::read("body.socpak")?)?;
//! for i in 0..pak.len() {
//!     let Some(name) = pak.name(i) else { continue };
//!     if !(name.ends_with(".soc") || name.ends_with(".entxml")) {
//!         continue;
//!     }
//!     if let Some(root) = object_container::decode(&pak.read(i)?)? {
//!         // domain logic lives in the caller, not here:
//!         for comp in root.find_all("HarvestableProviderComponent") {
//!             if let Some(preset) = comp.attr("preset") {
//!                 println!("{name}: {preset}");
//!             }
//!         }
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::io::{Cursor, Read};

use svarog_cryxml::CryXml;

use crate::error::{Error, Result};

/// CrCh chunk-file magic (first 4 bytes; a 5th byte `F` follows in the version
/// field). A `.soc`/`.pla` is a CrCh container.
const CRCH_MAGIC: &[u8; 4] = b"CrCh";
/// CryXmlB document magic.
const CRYXML_MAGIC: &[u8; 8] = b"CryXmlB\0";

/// A decoded XML node — tag, attributes, children. Produced uniformly from
/// plain XML, raw CryXmlB, or the CryXmlB chunk inside a CrCh `.soc`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    /// Element tag name (e.g. `Entity`, `HarvestableProviderComponent`).
    pub tag: String,
    /// Attributes in document order (`key`, `value`).
    pub attrs: Vec<(String, String)>,
    /// Child elements in document order.
    pub children: Vec<XmlNode>,
}

impl XmlNode {
    /// First attribute value for `key`, if present.
    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// Direct children with the given tag.
    pub fn children_named<'a>(&'a self, tag: &'a str) -> impl Iterator<Item = &'a XmlNode> + 'a {
        self.children.iter().filter(move |c| c.tag == tag)
    }

    /// Pre-order traversal over `self` and every descendant.
    pub fn descendants(&self) -> impl Iterator<Item = &XmlNode> {
        let mut stack = vec![self];
        std::iter::from_fn(move || {
            let node = stack.pop()?;
            // Push children in reverse so traversal yields document order.
            stack.extend(node.children.iter().rev());
            Some(node)
        })
    }

    /// Every descendant (including `self`) whose tag matches — the common way a
    /// consumer locates components anywhere in an entity tree.
    pub fn find_all<'a>(&'a self, tag: &'a str) -> impl Iterator<Item = &'a XmlNode> + 'a {
        self.descendants().filter(move |n| n.tag == tag)
    }
}

/// Decode object-container file bytes into a node tree.
///
/// Handles all three container shapes by sniffing the leading bytes:
/// - **CrCh** (`.soc`/`.pla`): peel the chunk table, decode every embedded
///   CryXmlB chunk. Returns `Ok(None)` when the container has no CryXmlB chunk
///   (an include-only container — just a reference list, no entity tree), so
///   callers can skip it cleanly rather than treating it as an error.
/// - **raw CryXmlB**: decode directly.
/// - otherwise: treat the bytes as plain UTF-8 XML.
///
/// When a CrCh container holds more than one CryXmlB chunk (rare), the chunk
/// roots are wrapped under a synthetic `ObjectContainerChunks` node so the
/// return is always a single root; [`XmlNode::find_all`] sees through it.
pub fn decode(bytes: &[u8]) -> Result<Option<XmlNode>> {
    if bytes.len() >= 4 && &bytes[..4] == CRCH_MAGIC {
        let mut roots = Vec::new();
        for chunk in cryxml_chunks(bytes)? {
            roots.push(decode_cryxml(chunk)?);
        }
        Ok(match roots.len() {
            0 => None,
            1 => roots.pop(),
            _ => Some(XmlNode {
                tag: "ObjectContainerChunks".to_string(),
                attrs: Vec::new(),
                children: roots,
            }),
        })
    } else if CryXml::is_cryxml(bytes) {
        Ok(Some(decode_cryxml(bytes)?))
    } else {
        let text = std::str::from_utf8(bytes).map_err(|_| {
            Error::ObjectContainerFormat("not CrCh, not CryXmlB, not UTF-8 XML".to_string())
        })?;
        Ok(Some(parse_xml(text)?))
    }
}

/// Decode a CryXmlB byte slice (a whole file or a `.soc` chunk) into a tree.
fn decode_cryxml(bytes: &[u8]) -> Result<XmlNode> {
    let doc = CryXml::parse(bytes).map_err(|e| Error::CryXml(e.to_string()))?;
    let xml = doc
        .to_xml_string()
        .map_err(|e| Error::CryXml(e.to_string()))?;
    parse_xml(&xml)
}

/// Walk a CrCh chunk table and return every chunk whose payload begins with the
/// CryXmlB magic. Layout (little-endian, verified empirically — chunk
/// offsets+sizes land exactly on EOF and the entity chunk starts with
/// `CryXmlB\0`):
///
/// ```text
/// 0x00  magic "CrCh"        [4]   (a 5th byte 'F' follows in the version field)
/// 0x04  version             u32
/// 0x08  chunk_count         u32
/// 0x0C  chunk_table_offset  u32
/// table entry (16 bytes):  type:u16  version:u16  id:u32  size:u32  offset:u32
/// ```
///
/// Chunks are matched by **payload magic**, not by trusting `type == 0x0004`,
/// so a chunk-type-id variant across game versions can't cause a misparse.
fn cryxml_chunks(bytes: &[u8]) -> Result<Vec<&[u8]>> {
    Ok(crch_chunks(bytes)?
        .into_iter()
        .map(|(_, payload)| payload)
        .filter(|p| p.len() >= CRYXML_MAGIC.len() && &p[..CRYXML_MAGIC.len()] == CRYXML_MAGIC)
        .collect())
}

/// Every chunk of a CrCh container as `(type, payload)` (layout above).
fn crch_chunks(bytes: &[u8]) -> Result<Vec<(u16, &[u8])>> {
    let rd_u32 = |off: usize| -> Result<usize> {
        bytes
            .get(off..off + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
            .ok_or_else(|| Error::ChunkContainer(format!("header truncated at {off:#x}")))
    };

    let count = rd_u32(0x08)?;
    let table = rd_u32(0x0C)?;
    const ENTRY: usize = 16;

    let span = count
        .checked_mul(ENTRY)
        .and_then(|n| table.checked_add(n))
        .ok_or_else(|| Error::ChunkContainer("chunk table size overflow".to_string()))?;
    if span > bytes.len() {
        return Err(Error::ChunkContainer(format!(
            "chunk table [{table:#x}..{span:#x}) past EOF {:#x}",
            bytes.len()
        )));
    }

    let mut out = Vec::new();
    for i in 0..count {
        let base = table + i * ENTRY;
        // type:u16 @+0, version:u16 @+2, id:u32 @+4, size:u32 @+8, offset:u32 @+12
        let ty = u16::from_le_bytes(bytes[base..base + 2].try_into().unwrap());
        let size = u32::from_le_bytes(bytes[base + 8..base + 12].try_into().unwrap()) as usize;
        let offset = u32::from_le_bytes(bytes[base + 12..base + 16].try_into().unwrap()) as usize;
        let end = offset
            .checked_add(size)
            .ok_or_else(|| Error::ChunkContainer(format!("chunk {i} range overflow")))?;
        let payload = bytes.get(offset..end).ok_or_else(|| {
            Error::ChunkContainer(format!(
                "chunk {i} range [{offset:#x}..{end:#x}) past EOF {:#x}",
                bytes.len()
            ))
        })?;
        out.push((ty, payload));
    }
    Ok(out)
}

/// CrCh chunk type of the static geometry an object container places.
const INCLUDED_OBJECTS: u16 = 0x0010;

/// The static geometry (brushes) one `.soc` places: geometry files with a
/// transform each, from its `IncludedObjects` chunks (type `0x0010`).
///
/// Layout (StarBreaker's reader, MIT, after scdatatools; checked on the
/// Constellation Mk V interiors by `sc-vehicles`' `interior_probe`): 4 bytes
/// padding, `u32` geometry count and that many 256-byte NUL-padded paths,
/// `u16` material and `u16` palette counts and their 256-byte paths, 28
/// bytes, then a `u32` byte length of the object records. A record starts
/// with its `u32` type: type 1 is a placement (168 bytes, 184 when the `u64`
/// at +160 is 0) with the geometry index `u16` at +60 and a row-major 3 × 4
/// `f64` transform at +64 (container frame), its tint-palette index the
/// `u16` at +172 of the long form (`0xFFFF`: none); type 7 is 152 bytes and
/// type `0x10` 136 (meaning not read); zero words between records are
/// padding; an unknown type is skipped 4 bytes at a time.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IncludedObjects {
    /// Geometry paths (`.cgf` / `.cga`), as written.
    pub geometry: Vec<String>,
    /// Material paths.
    pub materials: Vec<String>,
    /// Tint palette paths.
    pub palettes: Vec<String>,
    pub objects: Vec<IncludedObject>,
}

/// One placed geometry of [`IncludedObjects`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IncludedObject {
    /// Index into [`IncludedObjects::geometry`].
    pub geometry: usize,
    /// Placement in the container's frame, 3 × 4 row-major.
    pub transform: [[f32; 4]; 3],
    /// Index into [`IncludedObjects::palettes`].
    pub palette: Option<usize>,
}

/// The `IncludedObjects` of a `.soc` (CrCh) file, one per chunk; empty for
/// a container without static geometry.
pub fn included_objects(soc: &[u8]) -> Result<Vec<IncludedObjects>> {
    if soc.get(..4) != Some(CRCH_MAGIC.as_slice()) {
        return Ok(Vec::new());
    }
    crch_chunks(soc)?
        .into_iter()
        .filter(|(ty, _)| *ty == INCLUDED_OBJECTS)
        .map(|(_, payload)| IncludedObjects::parse(payload))
        .collect()
}

impl IncludedObjects {
    /// Parse one `IncludedObjects` chunk payload.
    pub fn parse(d: &[u8]) -> Result<Self> {
        const PATH: usize = 256;
        let bad = |what: &str| Error::ChunkContainer(format!("IncludedObjects: {what}"));
        let u16_at = |p: usize| -> Result<u16> {
            d.get(p..p + 2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .ok_or_else(|| bad("truncated"))
        };
        let u32_at = |p: usize| -> Result<u32> {
            d.get(p..p + 4)
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .ok_or_else(|| bad("truncated"))
        };
        let u64_at = |p: usize| -> Result<u64> {
            d.get(p..p + 8)
                .map(|b| u64::from_le_bytes(b.try_into().expect("8 bytes")))
                .ok_or_else(|| bad("truncated"))
        };
        let f64_at = |p: usize| -> Result<f64> { u64_at(p).map(f64::from_bits) };
        let paths = |at: &mut usize, n: usize| -> Result<Vec<String>> {
            (0..n)
                .map(|_| {
                    let b = d
                        .get(*at..*at + PATH)
                        .ok_or_else(|| bad("truncated path"))?;
                    *at += PATH;
                    let end = b.iter().position(|&c| c == 0).unwrap_or(PATH);
                    Ok(String::from_utf8_lossy(&b[..end]).into_owned())
                })
                .collect()
        };

        let mut at = 4;
        let n_geometry = u32_at(at)? as usize;
        at += 4;
        let geometry = paths(&mut at, n_geometry)?;
        let n_materials = u16_at(at)? as usize;
        let n_palettes = u16_at(at + 2)? as usize;
        at += 4;
        let materials = paths(&mut at, n_materials)?;
        let palettes = paths(&mut at, n_palettes)?;
        at += 28;
        let len = u32_at(at)? as usize;
        at += 4;
        let end = at.checked_add(len).ok_or_else(|| bad("overflow"))?;
        if end > d.len() {
            return Err(bad("objects past chunk end"));
        }

        let mut objects = Vec::new();
        while at + 4 <= end {
            match u32_at(at)? {
                1 => {
                    let short = u64_at(at + 160)? != 0;
                    let size = if short { 168 } else { 184 };
                    if at + size > end {
                        break;
                    }
                    let mut transform = [[0.0f32; 4]; 3];
                    for (r, row) in transform.iter_mut().enumerate() {
                        for (c, v) in row.iter_mut().enumerate() {
                            *v = f64_at(at + 64 + (r * 4 + c) * 8)? as f32;
                        }
                    }
                    let palette = if short {
                        None
                    } else {
                        Some(u16_at(at + 172)?).filter(|&i| i != 0xFFFF)
                    };
                    objects.push(IncludedObject {
                        geometry: u16_at(at + 60)? as usize,
                        transform,
                        palette: palette.map(usize::from),
                    });
                    at += size;
                }
                7 => at += 152,
                0x10 => at += 136,
                _ => at += 4,
            }
        }
        Ok(Self {
            geometry,
            materials,
            palettes,
            objects,
        })
    }
}

/// Parse XML text into an [`XmlNode`] tree (quick-xml event reader + a stack).
fn parse_xml(text: &str) -> Result<XmlNode> {
    use quick_xml::events::Event;

    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<XmlNode> = Vec::new();
    let mut root: Option<XmlNode> = None;

    let xml_err = |e: quick_xml::Error| Error::ObjectContainerXml(e.to_string());

    loop {
        match reader.read_event().map_err(xml_err)? {
            Event::Start(e) => stack.push(node_from(&e)?),
            Event::Empty(e) => {
                let node = node_from(&e)?;
                attach(&mut stack, &mut root, node);
            }
            Event::End(_) => {
                let node = stack
                    .pop()
                    .ok_or_else(|| Error::ObjectContainerXml("unbalanced end tag".to_string()))?;
                attach(&mut stack, &mut root, node);
            }
            Event::Eof => break,
            _ => {}
        }
    }

    root.ok_or_else(|| Error::ObjectContainerXml("document has no root element".to_string()))
}

/// Attach a finished node to its parent (top of stack), or set it as the root.
fn attach(stack: &mut [XmlNode], root: &mut Option<XmlNode>, node: XmlNode) {
    // `stack` is only ever shrunk by the caller; use the last element as parent.
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
    } else {
        *root = Some(node);
    }
}

fn node_from(e: &quick_xml::events::BytesStart) -> Result<XmlNode> {
    let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut attrs = Vec::new();
    for a in e.attributes() {
        let a = a.map_err(|err| Error::ObjectContainerXml(err.to_string()))?;
        let key = String::from_utf8_lossy(a.key.as_ref()).into_owned();
        let val = a
            .unescape_value()
            .map_err(|err| Error::ObjectContainerXml(err.to_string()))?
            .into_owned();
        attrs.push((key, val));
    }
    Ok(XmlNode {
        tag,
        attrs,
        children: Vec::new(),
    })
}

/// A `.socpak` opened as a ZIP archive. Members are read by index on demand.
pub struct Socpak {
    inner: zip::ZipArchive<Cursor<Vec<u8>>>,
}

impl Socpak {
    /// Open socpak bytes (the raw `.socpak` entry from a P4K, or a file) as ZIP.
    pub fn open(bytes: Vec<u8>) -> Result<Self> {
        let inner = zip::ZipArchive::new(Cursor::new(bytes)).map_err(Error::Socpak)?;
        Ok(Self { inner })
    }

    /// Number of members.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the archive has no members.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Member name at `index`, if any.
    pub fn name(&self, index: usize) -> Option<String> {
        self.inner.name_for_index(index).map(str::to_string)
    }

    /// Read the bytes of the member at `index`.
    pub fn read(&mut self, index: usize) -> Result<Vec<u8>> {
        let mut file = self.inner.by_index(index).map_err(Error::Socpak)?;
        let mut buf = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buf).map_err(Error::SocpakRead)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_entity_xml_and_finds_nested_component() {
        let xml = r#"<?xml version="1.0"?>
            <Entity Name="b" EntityClass="ProceduralEntity">
              <PropertiesDataCore>
                <HarvestableProviderComponent __type="HarvestableProviderParams" preset="abc-123"/>
              </PropertiesDataCore>
            </Entity>"#;
        let root = decode(xml.as_bytes()).unwrap().unwrap();
        assert_eq!(root.tag, "Entity");
        assert_eq!(root.attr("Name"), Some("b"));
        let presets: Vec<_> = root
            .find_all("HarvestableProviderComponent")
            .filter_map(|n| n.attr("preset"))
            .collect();
        assert_eq!(presets, ["abc-123"]);
    }

    /// One geometry path, no materials or palettes, one long-form placement
    /// (a 90 degree turn about z, moved to (5, 10, 15), palette 4) and a
    /// zero padding word.
    #[test]
    fn reads_an_included_objects_chunk() {
        let mut d = vec![0u8; 4];
        d.extend(1u32.to_le_bytes());
        let mut path = b"objects/test.cgf".to_vec();
        path.resize(256, 0);
        d.extend(path);
        d.extend([0u8; 4]);
        d.extend([0u8; 28]);
        let mut obj = Vec::new();
        obj.extend(1u32.to_le_bytes());
        obj.extend([0u8; 56]);
        obj.extend(0u16.to_le_bytes());
        obj.extend(0u16.to_le_bytes());
        for v in [
            0.0, -1.0, 0.0, 5.0, 1.0, 0.0, 0.0, 10.0, 0.0, 0.0, 1.0, 15.0f64,
        ] {
            obj.extend(v.to_le_bytes());
        }
        obj.extend(0u64.to_le_bytes());
        let mut tail = [0u8; 16];
        tail[4..6].copy_from_slice(&4u16.to_le_bytes());
        obj.extend(tail);
        obj.extend([0u8; 4]);
        d.extend((obj.len() as u32).to_le_bytes());
        d.extend(obj);
        let io = IncludedObjects::parse(&d).unwrap();
        assert_eq!(io.geometry, ["objects/test.cgf"]);
        assert_eq!(io.objects.len(), 1);
        let o = io.objects[0];
        assert_eq!((o.geometry, o.palette), (0, Some(4)));
        assert_eq!(o.transform[0], [0.0, -1.0, 0.0, 5.0]);
        assert_eq!(o.transform[2][3], 15.0);
    }

    #[test]
    fn non_xml_bytes_are_a_format_error() {
        let err = decode(&[0xff, 0xfe, 0x00, 0x01, 0x02]).unwrap_err();
        assert!(matches!(err, Error::ObjectContainerFormat(_)));
    }

    #[test]
    fn descendants_yield_document_order() {
        let xml = r#"<r><a/><b><c/></b></r>"#;
        let root = decode(xml.as_bytes()).unwrap().unwrap();
        let tags: Vec<_> = root.descendants().map(|n| n.tag.as_str()).collect();
        assert_eq!(tags, ["r", "a", "b", "c"]);
    }
}
