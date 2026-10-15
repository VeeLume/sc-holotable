//! An item assembled for drawing: its own geometry and, recursively, what is
//! attached to it, each part with the geometry variant, material and palette
//! the game would pick for it there.
//!
//! [`Visuals::scene`] walks the recipes: a `.cdf` assembly becomes its parts,
//! a port puts the attached item on a bone or helper (or, for a skinned port,
//! on this item's skeleton), and the variant tags follow the game's rules:
//!
//! - an item on its own is the stored prop (`<Body> inventoryStoredEntity`);
//! - an item worn on a body, directly or through a parent, is the
//!   body-fitted variant (`<Body>` only);
//! - the parent port's `Flags` are variant tags too (a player head's
//!   `Eyes_ItemPort` says `Protos_Head`, picking the eyes' head variant).
//!
//! Reading `.cdf` files is the caller's (a closure over
//! `sc_extract::CharacterDefinition::read`), so this stays free of I/O;
//! what cannot be resolved is reported as [`SceneNote`]s, not errors.

use sc_extract::{AttachmentKind, CharacterDefinition, Guid, RecordCollection};
use serde::{Deserialize, Serialize};

use crate::{LoadoutEntry, Palette, Palettes, Visual, Visuals};

/// The body an item is shown for. Armor and clothing pick their geometry
/// variant by it; the game binds it at runtime from the player's character,
/// and tags only select variants that ask for them, so other items are
/// unaffected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Body {
    #[default]
    Female,
    Male,
}

impl Body {
    /// The variant tag (`Female` / `Male`).
    pub fn tag(self) -> &'static str {
        match self {
            Body::Female => "Female",
            Body::Male => "Male",
        }
    }
}

/// An item actually attached at a port (an inventory's), replacing the
/// default loadout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attached {
    /// The parent's port, by name (case-insensitive, as the game matches).
    pub port: String,
    pub item: Guid,
    /// What is attached to it in turn; `None` takes its default loadout.
    pub children: Option<Vec<Attached>>,
}

/// An assembled item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub root: Part,
    /// What could not be resolved; the scene draws without it.
    pub notes: Vec<SceneNote>,
}

impl Scene {
    /// A character: a bare `.chr` skeleton with nothing of its own to draw
    /// (the player actor), its body skinned on.
    pub fn is_character(&self) -> bool {
        self.root.pieces.is_empty()
            && self
                .root
                .skeleton
                .as_deref()
                .is_some_and(|s| s.to_ascii_lowercase().ends_with(".chr"))
    }
}

/// One item (or one part of a `.cdf` assembly) and what hangs on it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    /// The item record; `None` for a `.cdf` part that is no item.
    pub item: Option<Guid>,
    /// For a `.cdf` part: its attachment name (`AName`).
    pub cdf_attachment: Option<String>,
    /// Geometry drawn at the part's origin.
    pub pieces: Vec<Piece>,
    /// The `.chr` whose bones the attachments name (inherited by skinned
    /// children that have none); without one they name geometry nodes.
    pub skeleton: Option<String>,
    pub attachments: Vec<Attachment>,
    /// The tint palette its materials' `PaletteTint` layers read.
    pub palette: Option<Palette>,
}

/// A geometry file with the material to draw it with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    /// `.cgf` / `.cga` / `.skin` path in `Data.p4k`.
    pub geometry: String,
    /// `.mtl`; `None` keeps the geometry's own `MtlName`.
    pub material: Option<String>,
}

/// Where a child part sits on its parent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    /// The parent's bone or geometry node; `None` when the child is skinned
    /// to the parent's skeleton and so sits at its origin in bind pose (a
    /// body's head, a head's hair, worn armor).
    pub bone: Option<String>,
    /// Offset from the bone.
    pub offset: Offset,
    /// The child's own helper (bone or node) that meets the bone instead of
    /// its origin (`ItemOffsetHelperName`).
    pub item_helper: Option<String>,
    pub part: Part,
}

/// A rigid offset: position, then rotation as a quaternion `(x, y, z, w)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Something [`Visuals::scene`] left out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SceneNote {
    /// The item has no geometry for the requested tags.
    NoGeometry { item: Guid },
    /// The `.cdf` could not be read.
    CdfUnreadable { path: String, reason: String },
    /// A `.cdf` attachment of a kind not assembled (`CA_FACE`, `CA_PROX`, a
    /// bone attachment without a bone).
    CdfAttachmentSkipped { path: String, attachment: String },
    /// The parent has no port by that name, or it names no helper.
    NoPort { item: Guid, port: String },
    /// Attachments nested deeper than [`MAX_DEPTH`] were dropped.
    TooDeep { item: Guid },
}

/// Nesting limit for attached items (player → body → armor core → backpack
/// → rifle → scope is five).
pub const MAX_DEPTH: usize = 10;

/// How a part is shown: for which body, whether it is worn, and the parent
/// port's flags.
#[derive(Debug, Clone, Copy)]
struct Fit<'a> {
    body: Body,
    worn: bool,
    port_flags: &'a str,
}

impl<'a> Fit<'a> {
    /// The variant tags: the stored prop on its own, the body-fitted
    /// variant when worn, plus the port's flags.
    fn tags(self) -> Vec<&'a str> {
        let mut tags = vec![self.body.tag()];
        if !self.worn {
            tags.push("inventoryStoredEntity");
        }
        tags.push(self.port_flags);
        tags
    }
}

impl Visuals {
    /// The item `item` assembled for drawing. `attached` replaces its
    /// default loadout (an inventory's real attachments, recursively);
    /// `None` keeps the default. `read_cdf` reads a `.cdf` assembly by path.
    /// `None` when the item has no geometry of its own.
    ///
    /// ```no_run
    /// use sc_extract::{AssetConfig, AssetData, AssetSource, CharacterDefinition, Datacore, Guid};
    /// use sc_items::Items;
    /// use sc_items_visuals::{Body, Palettes, Visuals};
    ///
    /// let install = sc_discovery::discover_primary()?;
    /// let assets = AssetSource::from_install(&install)?;
    /// let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    /// let datacore = Datacore::parse(&assets, &data)?;
    /// let items = Items::build(datacore.records());
    /// let visuals = Visuals::build(&datacore, &items);
    /// let palettes = Palettes::build(&datacore);
    /// let guid: Guid = "ab9a5e4a-0000-0000-0000-000000000000".parse()?;
    /// let scene = visuals.scene(&palettes, &guid, None, Body::Female, |path| {
    ///     CharacterDefinition::read(&assets, path).map_err(|e| e.to_string())
    /// });
    /// if let Some(scene) = scene {
    ///     println!("{} attachments, {} notes", scene.root.attachments.len(), scene.notes.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn scene(
        &self,
        palettes: &Palettes,
        item: &Guid,
        attached: Option<&[Attached]>,
        body: Body,
        mut read_cdf: impl FnMut(&str) -> Result<CharacterDefinition, String>,
    ) -> Option<Scene> {
        let mut notes = Vec::new();
        let fit = Fit {
            body,
            worn: false,
            port_flags: "",
        };
        let mut b = Assembly {
            visuals: self,
            palettes,
            read_cdf: &mut read_cdf,
            notes: &mut notes,
        };
        let mut root = b.part(item, attached, fit, 0)?;
        share_skeleton(&mut root, None);
        Some(Scene { root, notes })
    }
}

struct Assembly<'a, F> {
    visuals: &'a Visuals,
    palettes: &'a Palettes,
    read_cdf: &'a mut F,
    notes: &'a mut Vec<SceneNote>,
}

impl<F: FnMut(&str) -> Result<CharacterDefinition, String>> Assembly<'_, F> {
    fn part(
        &mut self,
        guid: &Guid,
        attached: Option<&[Attached]>,
        fit: Fit,
        depth: usize,
    ) -> Option<Part> {
        let visual = self.visuals.get(guid)?;
        let Some(look) = visual.look(&fit.tags()) else {
            self.notes.push(SceneNote::NoGeometry { item: *guid });
            return None;
        };
        let mut part = Part {
            item: Some(*guid),
            cdf_attachment: None,
            pieces: Vec::new(),
            skeleton: None,
            attachments: Vec::new(),
            palette: look
                .palette
                .as_ref()
                .and_then(|p| self.palettes.resolve(p))
                .cloned(),
        };
        let lower = look.geometry.to_ascii_lowercase();
        if lower.ends_with(".cdf") {
            self.assembly(&mut part, &look.geometry, look.material.as_deref());
        } else if lower.ends_with(".chr") {
            // A bare skeleton (the player actor): nothing to draw itself;
            // its skinned children (the body) take its bones.
            part.skeleton = Some(look.geometry.clone());
        } else {
            part.pieces.push(Piece {
                geometry: look.geometry.clone(),
                material: look.material.clone(),
            });
        }

        if depth >= MAX_DEPTH {
            self.notes.push(SceneNote::TooDeep { item: *guid });
            return Some(part);
        }
        let defaults;
        let children = match attached {
            Some(c) => c,
            None => {
                defaults = default_attached(&visual.default_loadout);
                &defaults
            }
        };
        for c in children {
            // Items without geometry (sensors, controllers) are not drawn.
            if self.visuals.get(&c.item).is_none() {
                continue;
            }
            let Some((port, bone)) = attach_point(visual, &c.port) else {
                self.notes.push(SceneNote::NoPort {
                    item: *guid,
                    port: c.port.clone(),
                });
                continue;
            };
            let child_fit = Fit {
                worn: fit.worn || port.skinned,
                port_flags: &port.flags,
                ..fit
            };
            if let Some(p) = self.part(&c.item, c.children.as_deref(), child_fit, depth + 1) {
                part.attachments.push(Attachment {
                    bone,
                    offset: Offset {
                        position: port.offset.position,
                        rotation: ang3_quaternion(port.offset.rotation),
                    },
                    item_helper: port.item_offset_helper.clone(),
                    part: p,
                });
            }
        }
        Some(part)
    }

    /// A `.cdf` assembly's model and parts, into `part`.
    fn assembly(&mut self, part: &mut Part, path: &str, skin_material: Option<&str>) {
        let cdf = match (self.read_cdf)(path) {
            Ok(c) => c,
            Err(reason) => {
                self.notes.push(SceneNote::CdfUnreadable {
                    path: path.to_string(),
                    reason,
                });
                return;
            }
        };
        // A skin's material replaces the assembly's own parts' materials
        // (those from the `.cdf`'s folder); a part from elsewhere (a
        // cartridge from `ammo/`) keeps its own, as does a part without one
        // (its geometry's MtlName). Verified on the P4-AR "Star Kitten".
        let folder = parent_folder(path);
        let material = |m: &Option<String>| match (skin_material, m) {
            (Some(skin), Some(m)) if parent_folder(m) == folder => Some(skin.to_string()),
            _ => m.clone(),
        };
        if cdf.model.to_ascii_lowercase().ends_with(".chr") {
            part.skeleton = Some(cdf.model.clone());
        } else {
            part.pieces.push(Piece {
                geometry: cdf.model.clone(),
                material: skin_material.map(str::to_string),
            });
        }
        for a in &cdf.attachments {
            let Some(binding) = &a.binding else { continue };
            let piece = Piece {
                geometry: binding.clone(),
                material: material(&a.material),
            };
            match (&a.kind, &a.bone) {
                (AttachmentKind::Skin, _) => part.pieces.push(piece),
                (AttachmentKind::Bone, Some(bone)) => part.attachments.push(Attachment {
                    bone: Some(bone.clone()),
                    offset: Offset {
                        position: a.position,
                        // `RelRotation` is (w, x, y, z).
                        rotation: [a.rotation[1], a.rotation[2], a.rotation[3], a.rotation[0]],
                    },
                    item_helper: None,
                    part: Part {
                        item: None,
                        cdf_attachment: Some(a.name.clone()),
                        pieces: vec![piece],
                        skeleton: None,
                        attachments: Vec::new(),
                        palette: part.palette.clone(),
                    },
                }),
                _ => self.notes.push(SceneNote::CdfAttachmentSkipped {
                    path: path.to_string(),
                    attachment: a.name.clone(),
                }),
            }
        }
    }
}

/// The port named `name` and the bone its child goes on: `None` for a
/// skinned port; a non-skinned port without a helper has no attach point.
fn attach_point<'v>(visual: &'v Visual, name: &str) -> Option<(&'v crate::Port, Option<String>)> {
    let port = visual.port(name)?;
    if port.skinned {
        return Some((port, None));
    }
    Some((port, Some(port.helper.clone()?)))
}

/// Give every skinned part its parent's skeleton when it has none of its
/// own: a worn armor core's ports name the body's bones
/// (`magazine_attach_1_override`), and the body takes the player actor's.
fn share_skeleton(part: &mut Part, inherited: Option<&str>) {
    if part.skeleton.is_none() {
        part.skeleton = inherited.map(str::to_string);
    }
    let skeleton = part.skeleton.clone();
    for a in &mut part.attachments {
        let pass = if a.bone.is_none() {
            skeleton.as_deref()
        } else {
            None
        };
        share_skeleton(&mut a.part, pass);
    }
}

fn default_attached(loadout: &[LoadoutEntry]) -> Vec<Attached> {
    loadout
        .iter()
        .filter_map(|e| {
            Some(Attached {
                port: e.port.clone(),
                item: e.item?,
                children: (!e.children.is_empty()).then(|| default_attached(&e.children)),
            })
        })
        .collect()
}

/// CryEngine `Ang3` degrees (rotation = Rz · Ry · Rx) as a quaternion
/// `(x, y, z, w)`.
pub fn ang3_quaternion([x, y, z]: [f32; 3]) -> [f32; 4] {
    let (sx, cx) = (x.to_radians() / 2.0).sin_cos();
    let (sy, cy) = (y.to_radians() / 2.0).sin_cos();
    let (sz, cz) = (z.to_radians() / 2.0).sin_cos();
    [
        sx * cy * cz - cx * sy * sz,
        cx * sy * cz + sx * cy * sz,
        cx * cy * sz - sx * sy * cz,
        cx * cy * cz + sx * sy * sz,
    ]
}

/// A p4k path's folder, lower-cased, `/`-separated, without a `Data/`
/// prefix.
fn parent_folder(path: &str) -> String {
    let p = path.replace('\\', "/").to_ascii_lowercase();
    let p = p.strip_prefix("data/").unwrap_or(&p);
    p.rsplit_once('/')
        .map_or(String::new(), |(dir, _)| dir.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_items_ask_for_the_prop_worn_ones_for_the_body_variant() {
        let fit = Fit {
            body: Body::Male,
            worn: false,
            port_flags: "",
        };
        assert_eq!(fit.tags(), ["Male", "inventoryStoredEntity", ""]);
        let worn = Fit {
            worn: true,
            port_flags: "Protos_Head",
            ..fit
        };
        assert_eq!(worn.tags(), ["Male", "Protos_Head"]);
    }

    #[test]
    fn ang3_turns_about_z_then_y_then_x() {
        let q = ang3_quaternion([0.0, 0.0, 90.0]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        for (a, b) in q.iter().zip([0.0, 0.0, h, h]) {
            assert!((a - b).abs() < 1e-6, "{q:?}");
        }
        assert_eq!(ang3_quaternion([0.0; 3]), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn folders_compare_without_data_prefix_or_case() {
        assert_eq!(
            parent_folder("Data\\Objects\\Weapons\\P4AR.cdf"),
            parent_folder("objects/weapons/p4ar_mat.mtl")
        );
        assert_eq!(parent_folder("file.mtl"), "");
    }

    fn part(skeleton: Option<&str>, attachments: Vec<Attachment>) -> Part {
        Part {
            item: None,
            cdf_attachment: None,
            pieces: Vec::new(),
            skeleton: skeleton.map(Into::into),
            attachments,
            palette: None,
        }
    }

    fn attach(bone: Option<&str>, part: Part) -> Attachment {
        Attachment {
            bone: bone.map(Into::into),
            offset: Offset::default(),
            item_helper: None,
            part,
        }
    }

    #[test]
    fn skinned_children_inherit_the_skeleton_bone_children_do_not() {
        let mut root = part(
            Some("actor.chr"),
            vec![
                attach(None, part(None, vec![attach(None, part(None, vec![]))])),
                attach(Some("hand"), part(None, vec![])),
            ],
        );
        share_skeleton(&mut root, None);
        let body = &root.attachments[0].part;
        assert_eq!(body.skeleton.as_deref(), Some("actor.chr"));
        assert_eq!(
            body.attachments[0].part.skeleton.as_deref(),
            Some("actor.chr")
        );
        assert_eq!(root.attachments[1].part.skeleton, None);
    }
}
