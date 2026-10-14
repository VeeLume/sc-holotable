//! `.mtl` material files: the submaterials a geometry file's submeshes index
//! into ([`crate::Submesh::material`]).
//!
//! A `.mtl` is CryXmlB (or plain XML): a root `<Material>` whose
//! `<SubMaterials>` children are the submaterials, in index order. A file
//! without `<SubMaterials>` is a single material (layer files under
//! `materials/layers/` look like that). Each submaterial names its shader
//! (`Illum`, `LayerBlend_V2`, `MeshDecal`, `HardSurface`, `GlassPBR`, …), the
//! shader features compiled in (`StringGenMask`, `%NORMAL_MAP%BLEND_MAP`),
//! its textures by slot (`<Texture Map="TexSlot1" File="…"/>`), scalar
//! attributes, and — for `LayerBlend_V2` — the material layers it blends
//! (`<MatLayers><Layer Path="materials/layers/…mtl" TintColor=… />`).
//!
//! This reader is structural: which slot means what depends on the shader
//! and is the renderer's business. Texture files are named `.tif` / `.dds`
//! as authored; [`crate::Texture::read`] resolves either to the `.dds` in
//! the archive.
//!
//! ```no_run
//! let install = sc_discovery::discover_primary()?;
//! let assets = sc_extract::AssetSource::from_install(&install)?;
//! let mtl = sc_extract::Material::read(
//!     &assets,
//!     "objects/buildingsets/human/universal/prop/dressing/can/can_drink_3_a.mtl",
//! )?;
//! for s in &mtl.submaterials {
//!     println!("{} {} {:?}", s.name, s.shader, s.features);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde::{Deserialize, Serialize};

use crate::object_container::{self, XmlNode};
use crate::{AssetSource, Error, Result};

/// A `.mtl` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    /// In submesh-index order.
    pub submaterials: Vec<SubMaterial>,
}

/// One material of a `.mtl` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubMaterial {
    pub name: String,
    pub shader: String,
    /// `StringGenMask` features without their `%` (e.g. `NORMAL_MAP`).
    pub features: Vec<String>,
    /// `MtlFlags` bits.
    pub flags: u32,
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub emissive: [f32; 3],
    pub opacity: f32,
    /// 0..255.
    pub shininess: f32,
    /// Alpha-test threshold; 0 = off.
    pub alpha_test: f32,
    pub textures: Vec<MaterialTexture>,
    /// `LayerBlend` layers, in file order (`BaseLayer1`, `WearLayer1`, …).
    pub layers: Vec<MaterialLayer>,
    /// `<PublicParams>` attributes, as written.
    pub params: Vec<(String, String)>,
}

/// A texture bound to a shader slot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialTexture {
    /// `TexSlot1` … `TexSlot17`.
    pub slot: String,
    /// Archive path as authored (`.tif` or `.dds`, no `Data/` prefix).
    pub file: String,
    /// UV tiling from `<TexMod>`; `[1, 1]` when absent.
    pub tiling: [f32; 2],
}

/// A `LayerBlend` layer: a reusable layer material plus its tint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayer {
    pub name: String,
    /// The layer's own `.mtl` (`materials/layers/…`).
    pub path: String,
    /// Submaterial of the layer file to use; empty = the first.
    pub submaterial: String,
    pub tint: [f32; 3],
    pub wear_tint: [f32; 3],
    pub gloss_mult: f32,
    pub wear_gloss: f32,
    pub uv_tiling: f32,
    pub height_bias: f32,
    pub height_scale: f32,
    /// Tint-palette channel the layer takes its colour from; 0 = none.
    pub palette_tint: u8,
}

impl Material {
    /// Read and parse `path` (with or without a `Data/` prefix).
    pub fn read(assets: &AssetSource, path: &str) -> Result<Self> {
        let bytes = crate::mesh::read_data(assets, path)?;
        Self::parse(&bytes)
    }

    /// Parse `.mtl` bytes (CryXmlB or plain XML).
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let root = object_container::decode(bytes)?
            .ok_or_else(|| Error::Material("no XML document".into()))?;
        let root = root
            .find_all("Material")
            .next()
            .ok_or_else(|| Error::Material("no <Material> root".into()))?;
        let submaterials = match root.children_named("SubMaterials").next() {
            Some(subs) => subs
                .children_named("Material")
                .map(SubMaterial::from_xml)
                .collect(),
            None => vec![SubMaterial::from_xml(root)],
        };
        Ok(Self { submaterials })
    }

    /// The submaterial named `name`.
    pub fn by_name(&self, name: &str) -> Option<&SubMaterial> {
        self.submaterials.iter().find(|s| s.name == name)
    }
}

impl SubMaterial {
    /// Has `StringGenMask` feature `feature` (without `%`).
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }

    /// The texture bound to `slot` (`TexSlot1` …).
    pub fn texture(&self, slot: &str) -> Option<&MaterialTexture> {
        self.textures.iter().find(|t| t.slot == slot)
    }

    /// The `<PublicParams>` value of `key`, parsed as a number.
    pub fn param(&self, key: &str) -> Option<f32> {
        self.params
            .iter()
            .find(|(k, _)| k == key)
            .and_then(|(_, v)| v.trim().parse().ok())
    }

    fn from_xml(n: &XmlNode) -> Self {
        let textures = n
            .children_named("Textures")
            .flat_map(|t| t.children_named("Texture"))
            .filter_map(|t| {
                let file = t.attr("File")?.trim();
                if file.is_empty() {
                    return None;
                }
                let tiling = t
                    .children_named("TexMod")
                    .next()
                    .map(|m| [num(m.attr("TileU"), 1.0), num(m.attr("TileV"), 1.0)])
                    .unwrap_or([1.0, 1.0]);
                Some(MaterialTexture {
                    slot: t.attr("Map").unwrap_or_default().to_string(),
                    file: file.replace('\\', "/"),
                    tiling,
                })
            })
            .collect();
        let layers = n
            .children_named("MatLayers")
            .flat_map(|l| l.children_named("Layer"))
            .map(|l| MaterialLayer {
                name: l.attr("Name").unwrap_or_default().to_string(),
                path: l.attr("Path").unwrap_or_default().replace('\\', "/"),
                submaterial: l.attr("Submtl").unwrap_or_default().to_string(),
                tint: rgb(l.attr("TintColor"), [1.0; 3]),
                wear_tint: rgb(l.attr("WearTint"), [1.0; 3]),
                gloss_mult: num(l.attr("GlossMult"), 1.0),
                wear_gloss: num(l.attr("WearGloss"), 1.0),
                uv_tiling: num(l.attr("UVTiling"), 1.0),
                height_bias: num(l.attr("HeightBias"), 0.0),
                height_scale: num(l.attr("HeightScale"), 1.0),
                palette_tint: num(l.attr("PaletteTint"), 0.0) as u8,
            })
            .collect();
        let params = n
            .children_named("PublicParams")
            .flat_map(|p| p.attrs.iter().cloned())
            .collect();
        Self {
            name: n.attr("Name").unwrap_or_default().to_string(),
            shader: n.attr("Shader").unwrap_or_default().to_string(),
            features: n
                .attr("StringGenMask")
                .unwrap_or_default()
                .split('%')
                .filter(|f| !f.is_empty())
                .map(str::to_string)
                .collect(),
            flags: n.attr("MtlFlags").and_then(|v| v.parse().ok()).unwrap_or(0),
            diffuse: rgb(n.attr("Diffuse"), [1.0; 3]),
            specular: rgb(n.attr("Specular"), [0.04; 3]),
            emissive: rgb(n.attr("Emissive"), [0.0; 3]),
            opacity: num(n.attr("Opacity"), 1.0),
            shininess: num(n.attr("Shininess"), 0.0),
            alpha_test: num(n.attr("AlphaTest"), 0.0),
            textures,
            layers,
            params,
        }
    }
}

fn num(v: Option<&str>, default: f32) -> f32 {
    v.and_then(|v| v.trim().parse().ok()).unwrap_or(default)
}

fn rgb(v: Option<&str>, default: [f32; 3]) -> [f32; 3] {
    let Some(v) = v else { return default };
    let c: Vec<f32> = v.split(',').filter_map(|c| c.trim().parse().ok()).collect();
    match c.as_slice() {
        [r, g, b, ..] => [*r, *g, *b],
        _ => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAN: &str = r#"<Material MtlFlags="256">
      <SubMaterials>
        <Material Name="$proxy" Shader="NoDraw" StringGenMask="" />
        <Material Name="main" MtlFlags="50331776" Shader="Illum" StringGenMask="%NORMAL_MAP"
                  Diffuse="1,1,1" Specular="0.31,0.4,0.06" Opacity="1" Shininess="174">
          <Textures>
            <Texture Map="TexSlot1" File="Objects/can/textures/can_diff.tif" />
            <Texture Map="TexSlot2" File="Objects/shared/gloss_ddna.tif" />
          </Textures>
          <MatLayers>
            <Layer Name="BaseLayer1" Path="materials/layers/synthetic/painted_metal_04.mtl"
                   TintColor="0.5,0.6,0.7" GlossMult="0.8" UVTiling="24" PaletteTint="2" />
          </MatLayers>
          <PublicParams AlphaGlowCutoff="0" BlendMapTiling="1" />
        </Material>
      </SubMaterials>
    </Material>"#;

    #[test]
    fn parses_submaterials_textures_and_layers() {
        let m = Material::parse(CAN.as_bytes()).unwrap();
        assert_eq!(m.submaterials.len(), 2);
        let main = m.by_name("main").unwrap();
        assert_eq!(main.shader, "Illum");
        assert!(main.has_feature("NORMAL_MAP"));
        assert_eq!(main.flags, 50331776);
        assert_eq!(main.shininess, 174.0);
        assert_eq!(
            main.texture("TexSlot2").unwrap().file,
            "Objects/shared/gloss_ddna.tif"
        );
        let layer = &main.layers[0];
        assert_eq!(layer.tint, [0.5, 0.6, 0.7]);
        assert_eq!(layer.palette_tint, 2);
        assert_eq!(layer.uv_tiling, 24.0);
        assert_eq!(main.param("BlendMapTiling"), Some(1.0));
    }

    #[test]
    fn single_material_file_is_one_submaterial() {
        let m = Material::parse(br#"<Material Name="layer" Shader="Illum" />"#).unwrap();
        assert_eq!(m.submaterials.len(), 1);
        assert_eq!(m.submaterials[0].shader, "Illum");
    }
}
