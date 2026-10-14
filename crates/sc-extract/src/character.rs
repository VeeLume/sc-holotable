//! `.cdf` character definitions: how an item's model is assembled from a base
//! model and its attachments. FPS weapons use them: the base is a `.chr`
//! skeleton (or a `.cga` whose nodes serve as attach points), and the visible
//! parts are attachments.
//!
//! A `.cdf` is CryXmlB: `<CharacterDefinition>` with `<Model File=…>` and an
//! `<AttachmentList>` of `<Attachment>` elements. `Type` is `CA_SKIN` (a
//! skinned part, `.skin`, its vertices already in model space for the bind
//! pose) or `CA_BONE` (a rigid part on `BoneName`, offset by `RelPosition` /
//! `RelRotation`). An attachment without `Binding` is an empty socket (the
//! `bullet` slot of a weapon whose round is not modelled).
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let cdf = sc_extract::CharacterDefinition::read(
//!     &assets,
//!     "objects/fps_weapons/weapons_v7/behr/sniper/p6lr/bsnp_fps_behr_p6lr.cdf",
//! )?;
//! println!("{}", cdf.model);
//! for a in &cdf.attachments {
//!     println!("{:?} {} {:?} on {:?}", a.kind, a.name, a.binding, a.bone);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde::{Deserialize, Serialize};

use crate::object_container::{self, XmlNode};
use crate::{AssetSource, Error, Result};

/// A `.cdf` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterDefinition {
    /// `Model File`: the skeleton (`.chr`) or base geometry (`.cga`).
    pub model: String,
    pub attachments: Vec<CharacterAttachment>,
}

/// One `<Attachment>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterAttachment {
    pub kind: AttachmentKind,
    /// `AName`.
    pub name: String,
    /// `Binding`: the part's geometry; `None` for an empty socket.
    pub binding: Option<String>,
    /// `Material`: overrides the geometry's own material chunk.
    pub material: Option<String>,
    /// `BoneName` (`CA_BONE`).
    pub bone: Option<String>,
    /// `RelPosition` (`CA_BONE`), relative to the bone.
    pub position: [f32; 3],
    /// `RelRotation` (`CA_BONE`), quaternion `(w, x, y, z)` as authored.
    pub rotation: [f32; 4],
}

/// `Attachment Type`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttachmentKind {
    /// `CA_SKIN`.
    Skin,
    /// `CA_BONE`.
    Bone,
    /// Any other type, as authored (`CA_FACE`, `CA_PROX`, …).
    Other(String),
}

impl CharacterDefinition {
    /// Read and parse `path` (with or without a `Data/` prefix).
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        Self::parse(&crate::mesh::read_data(assets, path)?)
    }

    /// Parse `.cdf` bytes (CryXmlB or plain XML).
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let root = object_container::decode(bytes)?.ok_or_else(|| bad("no XML document"))?;
        let root = root
            .find_all("CharacterDefinition")
            .next()
            .ok_or_else(|| bad("no <CharacterDefinition> root"))?;
        let model = root
            .children_named("Model")
            .next()
            .and_then(|m| m.attr("File"))
            .ok_or_else(|| bad("no <Model File>"))?
            .to_string();
        let attachments = root
            .children_named("AttachmentList")
            .flat_map(|l| l.children_named("Attachment"))
            .map(CharacterAttachment::from_xml)
            .collect();
        Ok(Self { model, attachments })
    }
}

impl CharacterAttachment {
    fn from_xml(n: &XmlNode) -> Self {
        let text = |k: &str| n.attr(k).filter(|v| !v.is_empty()).map(str::to_string);
        Self {
            kind: match n.attr("Type").unwrap_or_default() {
                "CA_SKIN" => AttachmentKind::Skin,
                "CA_BONE" => AttachmentKind::Bone,
                other => AttachmentKind::Other(other.to_string()),
            },
            name: n.attr("AName").unwrap_or_default().to_string(),
            binding: text("Binding"),
            material: text("Material"),
            bone: text("BoneName"),
            position: floats(n.attr("RelPosition")).unwrap_or([0.0; 3]),
            rotation: floats(n.attr("RelRotation")).unwrap_or([1.0, 0.0, 0.0, 0.0]),
        }
    }
}

fn floats<const N: usize>(s: Option<&str>) -> Option<[f32; N]> {
    let v: Vec<f32> = s?
        .split(',')
        .map(|x| x.trim().parse().ok())
        .collect::<Option<_>>()?;
    v.try_into().ok()
}

fn bad(what: &str) -> Error {
    Error::Character(what.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_xml_cdf() {
        let xml = br#"<CharacterDefinition>
  <Model File="objects/w.chr"/>
  <AttachmentList>
    <Attachment Type="CA_BONE" AName="body" Binding="objects/w_body.cgf" Material="objects/w.mtl" RelRotation="1,0,0,0" RelPosition="0,0.5,0" BoneName="root"/>
    <Attachment Type="CA_SKIN" AName="parts" Binding="objects/w_parts.skin"/>
    <Attachment Type="CA_BONE" AName="bullet" BoneName="bullet"/>
  </AttachmentList>
</CharacterDefinition>"#;
        let cdf = CharacterDefinition::parse(xml).unwrap();
        assert_eq!(cdf.model, "objects/w.chr");
        assert_eq!(cdf.attachments.len(), 3);
        let body = &cdf.attachments[0];
        assert_eq!(body.kind, AttachmentKind::Bone);
        assert_eq!(body.bone.as_deref(), Some("root"));
        assert_eq!(body.position, [0.0, 0.5, 0.0]);
        assert_eq!(cdf.attachments[1].kind, AttachmentKind::Skin);
        assert_eq!(cdf.attachments[1].material, None);
        assert_eq!(cdf.attachments[2].binding, None);
    }
}
