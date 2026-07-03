//! sc-cargo-viewer — 3D visual validation of the cargo-grid geometry pipeline.
//!
//! Renders, per ship: cargo-grid boxes (wireframe), interior wall boxes
//! (wireframe, distance-filtered), and each grid face's cells **colored by
//! measured distance-to-wall** (red = wall flush, yellow = walkway gap,
//! green = open). Lets you fly around any vehicle and compare the pipeline's
//! output against the layout you know in-game.
//!
//! ```bash
//! cargo run -p sc-cargo-viewer --release
//! ```
//!
//! Controls: drag = orbit · right-drag = pan · scroll = zoom · presets top-left.
//! Data pipeline: see `docs/ship-cargo-grids.md` (§Tier C) — this is the same
//! validated chain as `cargo_grid_walls.rs`.

mod pipeline;

use eframe::egui;
use pipeline::{Holo, MAX_PROBE_M, ShipScene};

fn main() -> eframe::Result<()> {
    // Blocking load before the window opens (progress on the console).
    let holo = match Holo::load() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("failed to load game data: {e}");
            std::process::exit(1);
        }
    };
    eprintln!("building vehicle census…");
    let ships = holo.list_ships();
    eprintln!("{} vehicle entity classes.", ships.len());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 950.0]),
        ..Default::default()
    };
    let app = App::new(holo, ships);
    eframe::run_native(
        "sc-cargo-viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

// ── camera ───────────────────────────────────────────────────────────────────

struct Cam {
    yaw: f32,
    pitch: f32,
    dist: f32,
    target: [f32; 3],
}

impl Cam {
    fn basis(&self) -> ([f32; 3], [f32; 3], [f32; 3], [f32; 3]) {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        let fwd = [sy * cp, cy * cp, -sp];
        let eye = [
            self.target[0] - fwd[0] * self.dist,
            self.target[1] - fwd[1] * self.dist,
            self.target[2] - fwd[2] * self.dist,
        ];
        // right = fwd × up(0,0,1), normalized
        let mut right = [fwd[1], -fwd[0], 0.0];
        let rl = (right[0] * right[0] + right[1] * right[1]).sqrt().max(1e-6);
        right = [right[0] / rl, right[1] / rl, 0.0];
        // up' = right × fwd
        let up = [
            right[1] * fwd[2] - right[2] * fwd[1],
            right[2] * fwd[0] - right[0] * fwd[2],
            right[0] * fwd[1] - right[1] * fwd[0],
        ];
        (eye, right, up, fwd)
    }
}

const NEAR: f32 = 0.05;

/// world point → (camera-space [x,y,z]) with the given basis.
fn to_cam(p: [f32; 3], eye: [f32; 3], right: [f32; 3], up: [f32; 3], fwd: [f32; 3]) -> [f32; 3] {
    let rel = [p[0] - eye[0], p[1] - eye[1], p[2] - eye[2]];
    let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    [dot(rel, right), dot(rel, up), dot(rel, fwd)]
}

fn cam_to_screen(c: [f32; 3], rect: egui::Rect, scale: f32) -> egui::Pos2 {
    egui::pos2(
        rect.center().x + c[0] * scale / c[2],
        rect.center().y - c[1] * scale / c[2],
    )
}

// ── app ──────────────────────────────────────────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
enum WallMode {
    Off,
    Wire,
    Solid,
}

/// (depth, screen corners, fill, stroke) — one filled quad for the sorted pass.
type Quad = (f32, [egui::Pos2; 4], egui::Color32, egui::Stroke);
type Edge = ((usize, usize, usize), (usize, usize, usize));

struct App {
    holo: Holo,
    ships: Vec<String>,
    search: String,
    scene: Option<ShipScene>,
    /// (wall index → gap to nearest grid), precomputed per scene.
    wall_gap: Vec<f32>,
    /// scene bounds (grids ∪ near walls), for the front arrow + fit.
    bounds: Option<([f32; 3], [f32; 3])>,
    pending: Option<String>,
    pending_delay: u8,
    status: String,
    cam: Cam,
    show_grids: bool,
    show_cells: bool,
    cull_cells: bool,
    wall_mode: WallMode,
    wall_range: f32,
    wall_min_size: f32,
    /// faint always-on silhouette of the whole ship (large wall boxes,
    /// ignoring the range filter) — orientation context.
    show_outline: bool,
    selected: Option<String>,
}

impl App {
    fn new(holo: Holo, ships: Vec<String>) -> Self {
        Self {
            holo,
            ships,
            search: String::new(),
            scene: None,
            wall_gap: Vec::new(),
            bounds: None,
            pending: Some("MISC_Freelancer".into()),
            pending_delay: 1,
            status: "select a ship".into(),
            cam: Cam {
                yaw: 0.7,
                pitch: 0.5,
                dist: 40.0,
                target: [0.0; 3],
            },
            show_grids: true,
            show_cells: true,
            cull_cells: true,
            wall_mode: WallMode::Solid,
            wall_range: 3.0,
            wall_min_size: 1.0,
            show_outline: true,
            selected: None,
        }
    }

    fn load_ship(&mut self, name: &str) {
        match self.holo.build_scene(name) {
            Ok(scene) => {
                self.status = format!("{} — {}", scene.entity, scene.info);
                // wall→grid gap precompute (for the range filter)
                self.wall_gap = scene
                    .walls
                    .iter()
                    .map(|w| {
                        scene
                            .grids
                            .iter()
                            .map(|g| box_gap((w.min, w.max), (g.min, g.max)))
                            .fold(f32::INFINITY, f32::min)
                    })
                    .collect();
                self.fit_camera(&scene);
                self.scene = Some(scene);
                self.selected = Some(name.to_string());
            }
            Err(e) => {
                self.status = format!("{name}: {e}");
                self.scene = None;
                self.wall_gap.clear();
                self.selected = Some(name.to_string());
            }
        }
    }

    fn fit_camera(&mut self, scene: &ShipScene) {
        let mut mn = [f32::INFINITY; 3];
        let mut mx = [f32::NEG_INFINITY; 3];
        let mut any = false;
        for g in &scene.grids {
            any = true;
            for a in 0..3 {
                mn[a] = mn[a].min(g.min[a]);
                mx[a] = mx[a].max(g.max[a]);
            }
        }
        if !any {
            for w in scene.walls.iter().take(500) {
                for a in 0..3 {
                    mn[a] = mn[a].min(w.min[a]);
                    mx[a] = mx[a].max(w.max[a]);
                }
            }
        }
        if mn[0].is_finite() {
            self.bounds = Some((mn, mx));
            self.cam.target = [
                (mn[0] + mx[0]) / 2.0,
                (mn[1] + mx[1]) / 2.0,
                (mn[2] + mx[2]) / 2.0,
            ];
            let r = (0..3)
                .map(|a| mx[a] - mn[a])
                .fold(0.0f32, f32::max)
                .max(4.0);
            self.cam.dist = r * 2.0;
        } else {
            self.bounds = None;
        }
    }
}

/// The 6 faces of an AABB as (axis, dir, 4 corners).
fn box_faces(mn: [f32; 3], mx: [f32; 3]) -> [(usize, i32, [[f32; 3]; 4]); 6] {
    let mut out = [(0usize, 0i32, [[0.0f32; 3]; 4]); 6];
    let mut k = 0;
    for axis in 0..3 {
        let (u, v) = match axis {
            0 => (1, 2),
            1 => (0, 2),
            _ => (0, 1),
        };
        for dir in [-1i32, 1] {
            let f = if dir < 0 { mn[axis] } else { mx[axis] };
            let mut corners = [[0.0f32; 3]; 4];
            for (i, (cu, cv)) in [
                (mn[u], mn[v]),
                (mx[u], mn[v]),
                (mx[u], mx[v]),
                (mn[u], mx[v]),
            ]
            .iter()
            .enumerate()
            {
                corners[i][axis] = f;
                corners[i][u] = *cu;
                corners[i][v] = *cv;
            }
            out[k] = (axis, dir, corners);
            k += 1;
        }
    }
    out
}

/// Euclidean gap between two AABBs (0 when overlapping).
fn box_gap(a: ([f32; 3], [f32; 3]), b: ([f32; 3], [f32; 3])) -> f32 {
    let mut s = 0.0f32;
    for ax in 0..3 {
        let d = (b.0[ax] - a.1[ax]).max(a.0[ax] - b.1[ax]).max(0.0);
        s += d * d;
    }
    s.sqrt()
}

fn dist_color(d: f32, alpha: u8) -> egui::Color32 {
    // red (flush) → yellow (~1 cell walkway) → green (open)
    let t = (d / MAX_PROBE_M).clamp(0.0, 1.0);
    let (r, g) = if t < 0.25 {
        let k = t / 0.25;
        (255.0, 60.0 + 180.0 * k) // red → yellow
    } else {
        let k = (t - 0.25) / 0.75;
        (255.0 - 195.0 * k, 240.0 - 40.0 * k) // yellow → green
    };
    egui::Color32::from_rgba_unmultiplied(r as u8, g as u8, 60, alpha)
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // deferred ship load (paint one "loading" frame first)
        if self.pending.is_some() {
            if self.pending_delay > 0 {
                self.pending_delay -= 1;
                ctx.request_repaint();
            } else if let Some(name) = self.pending.take() {
                self.load_ship(&name);
            }
        }

        egui::SidePanel::left("ships")
            .min_width(240.0)
            .show(ctx, |ui| {
                ui.heading("ships");
                ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("filter…"));
                let filter = self.search.to_lowercase();
                let filtered: Vec<&String> = self
                    .ships
                    .iter()
                    .filter(|s| filter.is_empty() || s.to_lowercase().contains(&filter))
                    .collect();
                ui.label(format!("{} / {}", filtered.len(), self.ships.len()));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for s in filtered {
                        let sel = self.selected.as_deref() == Some(s.as_str());
                        if ui.selectable_label(sel, s).clicked() {
                            self.pending = Some(s.clone());
                            self.pending_delay = 1;
                            self.status = format!("loading {s}…");
                        }
                    }
                });
            });

        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_grids, "grids");
                ui.checkbox(&mut self.show_cells, "face cells");
                ui.checkbox(&mut self.cull_cells, "inside view")
                    .on_hover_text("show only the far side of each grid (look INTO the box) — each face visible exactly once");
                ui.separator();
                ui.label("walls:");
                ui.radio_value(&mut self.wall_mode, WallMode::Off, "off");
                ui.radio_value(&mut self.wall_mode, WallMode::Wire, "wire");
                ui.radio_value(&mut self.wall_mode, WallMode::Solid, "solid");
                ui.add(
                    egui::Slider::new(&mut self.wall_range, 0.0..=20.0)
                        .text("range (m)"),
                );
                ui.add(
                    egui::Slider::new(&mut self.wall_min_size, 0.0..=5.0)
                        .text("min size (m)"),
                )
                .on_hover_text("hide wall boxes whose largest dimension is below this (decor: lights, pipes, screens)");
                ui.checkbox(&mut self.show_outline, "ship outline")
                    .on_hover_text("faint silhouette of ALL large wall boxes (ignores range) — shape + orientation context");
                ui.separator();
                if ui.button("top").clicked() {
                    self.cam.yaw = 0.0;
                    self.cam.pitch = 1.45;
                }
                if ui.button("front").clicked() {
                    self.cam.yaw = std::f32::consts::PI;
                    self.cam.pitch = 0.05;
                }
                if ui.button("side").clicked() {
                    self.cam.yaw = std::f32::consts::FRAC_PI_2;
                    self.cam.pitch = 0.05;
                }
                if ui.button("iso").clicked() {
                    self.cam.yaw = 0.7;
                    self.cam.pitch = 0.5;
                }
                if ui.button("fit").clicked()
                    && let Some(scene) = self.scene.take()
                {
                    self.fit_camera(&scene);
                    self.scene = Some(scene);
                }
                ui.separator();
                ui.label("cells:");
                ui.colored_label(egui::Color32::from_rgb(255, 70, 60), "wall");
                ui.colored_label(egui::Color32::from_rgb(255, 220, 60), "walkway");
                ui.colored_label(egui::Color32::from_rgb(80, 210, 70), "open");
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(&self.status);
            if let Some(scene) = &self.scene {
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!("Σ {} SCU —", scene.total_scu));
                    for g in &scene.grids {
                        let short = g
                            .port
                            .trim_start_matches("hardpoint_")
                            .trim_start_matches("cargogrid_");
                        ui.label(
                            egui::RichText::new(format!("{short}: {} SCU", g.scu))
                                .color(egui::Color32::from_rgb(90, 200, 255)),
                        );
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::drag());
            let rect = response.rect;
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(18, 20, 26));

            // controls
            if response.dragged_by(egui::PointerButton::Primary) {
                let d = response.drag_delta();
                self.cam.yaw += d.x * 0.008;
                self.cam.pitch = (self.cam.pitch + d.y * 0.008).clamp(-1.54, 1.54);
            }
            if response.dragged_by(egui::PointerButton::Secondary) {
                let d = response.drag_delta();
                let (_, right, up, _) = self.cam.basis();
                let k = self.cam.dist * 0.0015;
                for a in 0..3 {
                    self.cam.target[a] -= right[a] * d.x * k;
                    self.cam.target[a] += up[a] * d.y * k;
                }
            }
            if response.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll.abs() > 0.0 {
                    self.cam.dist = (self.cam.dist * (-scroll * 0.002).exp()).clamp(1.0, 2000.0);
                }
            }

            let Some(scene) = &self.scene else {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "select a ship",
                    egui::FontId::proportional(18.0),
                    egui::Color32::GRAY,
                );
                return;
            };

            let (eye, right, up, fwd) = self.cam.basis();
            let scale = 0.5 * rect.height() / (30.0f32.to_radians()).tan();
            let proj = |p: [f32; 3]| -> [f32; 3] { to_cam(p, eye, right, up, fwd) };

            // face-visibility test: outward normal (axis,dir) vs view direction.
            // "front" = the face's outside points at the camera.
            let face_front = |axis: usize, dir: i32, center: [f32; 3]| -> bool {
                let mut n = [0.0f32; 3];
                n[axis] = dir as f32;
                let v = [center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]];
                n[0] * v[0] + n[1] * v[1] + n[2] * v[2] < 0.0
            };
            let quad_center = |c: &[[f32; 3]; 4]| -> [f32; 3] {
                [
                    (c[0][0] + c[2][0]) / 2.0,
                    (c[0][1] + c[2][1]) / 2.0,
                    (c[0][2] + c[2][2]) / 2.0,
                ]
            };

            // 1. filled quads (face cells + solid walls), one depth-sorted pass
            let mut quads: Vec<Quad> = Vec::new();
            let push_quad = |quads: &mut Vec<Quad>,
                             corners: &[[f32; 3]; 4],
                             color: egui::Color32,
                             stroke: egui::Stroke| {
                let cams: Vec<[f32; 3]> = corners.iter().map(|&c| proj(c)).collect();
                if cams.iter().any(|c| c[2] <= NEAR) {
                    return;
                }
                let pts = [
                    cam_to_screen(cams[0], rect, scale),
                    cam_to_screen(cams[1], rect, scale),
                    cam_to_screen(cams[2], rect, scale),
                    cam_to_screen(cams[3], rect, scale),
                ];
                let depth = (cams[0][2] + cams[1][2] + cams[2][2] + cams[3][2]) / 4.0;
                quads.push((depth, pts, color, stroke));
            };

            if self.show_cells {
                let cell_stroke =
                    egui::Stroke::new(0.5, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60));
                for g in &scene.grids {
                    for cell in &g.cells {
                        // inside view: draw only faces whose outside points AWAY
                        // from the camera — you look into the box, each face
                        // visible exactly once.
                        if self.cull_cells
                            && face_front(cell.axis, cell.dir, quad_center(&cell.corners))
                        {
                            continue;
                        }
                        let alpha = if self.cull_cells { 165 } else { 100 };
                        push_quad(
                            &mut quads,
                            &cell.corners,
                            dist_color(cell.dist, alpha),
                            cell_stroke,
                        );
                    }
                }
            }

            let wall_visible = |i: usize, w: &pipeline::WallView| -> bool {
                if self.wall_gap.get(i).copied().unwrap_or(0.0) > self.wall_range {
                    return false;
                }
                let ext = (0..3).map(|a| w.max[a] - w.min[a]).fold(0.0f32, f32::max);
                ext >= self.wall_min_size
            };

            if self.wall_mode == WallMode::Solid {
                let fill = egui::Color32::from_rgba_unmultiplied(150, 160, 178, 40);
                let stroke = egui::Stroke::new(
                    0.6,
                    egui::Color32::from_rgba_unmultiplied(170, 180, 195, 90),
                );
                for (i, w) in scene.walls.iter().enumerate() {
                    if !wall_visible(i, w) {
                        continue;
                    }
                    // walls are solid objects: draw camera-facing faces only
                    for (axis, dir, corners) in box_faces(w.min, w.max) {
                        if face_front(axis, dir, quad_center(&corners)) {
                            push_quad(&mut quads, &corners, fill, stroke);
                        }
                    }
                }
            }

            quads.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            for (_, pts, color, stroke) in quads {
                painter.add(egui::Shape::convex_polygon(pts.to_vec(), color, stroke));
            }

            // 2. wire walls (when selected)
            if self.wall_mode == WallMode::Wire {
                let stroke = egui::Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(150, 160, 175, 120),
                );
                for (i, w) in scene.walls.iter().enumerate() {
                    if !wall_visible(i, w) {
                        continue;
                    }
                    draw_box_edges(&painter, rect, scale, &proj, w.min, w.max, stroke);
                }
            }

            // 3. ship outline: faint wire of ALL large wall boxes (range-independent)
            if self.show_outline {
                let stroke = egui::Stroke::new(
                    0.7,
                    egui::Color32::from_rgba_unmultiplied(120, 130, 150, 34),
                );
                for w in &scene.walls {
                    let ext = (0..3).map(|a| w.max[a] - w.min[a]).fold(0.0f32, f32::max);
                    if ext >= 3.0 {
                        draw_box_edges(&painter, rect, scale, &proj, w.min, w.max, stroke);
                    }
                }
            }

            // 4. grid wireframes on top
            if self.show_grids {
                let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(90, 200, 255));
                for g in &scene.grids {
                    draw_box_edges(&painter, rect, scale, &proj, g.min, g.max, stroke);
                }
            }

            // 5. FRONT arrow (+Y is a ship's forward axis) anchored to the bounds
            if let Some((mn, mx)) = self.bounds {
                let cx = (mn[0] + mx[0]) / 2.0;
                let zm = (mn[2] + mx[2]) / 2.0;
                let len = ((mx[1] - mn[1]) * 0.15).clamp(3.0, 25.0);
                let base = [cx, mx[1] + 1.0, zm];
                let tip = [cx, mx[1] + 1.0 + len, zm];
                let col = egui::Color32::from_rgb(255, 210, 80);
                let stroke = egui::Stroke::new(2.5, col);
                draw_line3(&painter, rect, scale, &proj, base, tip, stroke);
                let head = len * 0.25;
                for side in [-1.0f32, 1.0] {
                    let wing = [cx + side * head * 0.6, tip[1] - head, zm];
                    draw_line3(&painter, rect, scale, &proj, tip, wing, stroke);
                }
                let tc = proj(tip);
                if tc[2] > NEAR {
                    painter.text(
                        cam_to_screen(tc, rect, scale) + egui::vec2(0.0, -14.0),
                        egui::Align2::CENTER_BOTTOM,
                        "FRONT",
                        egui::FontId::proportional(14.0),
                        col,
                    );
                }
            }

            // little axes gizmo at the target (x red, y green = fwd, z blue)
            let t = self.cam.target;
            for (axis, color) in [
                ([1.0, 0.0, 0.0], egui::Color32::RED),
                ([0.0, 1.0, 0.0], egui::Color32::GREEN),
                ([0.0, 0.0, 1.0], egui::Color32::from_rgb(90, 90, 255)),
            ] {
                let e = [
                    t[0] + axis[0] * 2.0,
                    t[1] + axis[1] * 2.0,
                    t[2] + axis[2] * 2.0,
                ];
                draw_line3(
                    &painter,
                    rect,
                    scale,
                    &proj,
                    t,
                    e,
                    egui::Stroke::new(1.5, color),
                );
            }
        });
    }
}

/// Draw the 12 edges of an AABB with near-plane clipping.
#[allow(clippy::too_many_arguments)]
fn draw_box_edges(
    painter: &egui::Painter,
    rect: egui::Rect,
    scale: f32,
    proj: &impl Fn([f32; 3]) -> [f32; 3],
    mn: [f32; 3],
    mx: [f32; 3],
    stroke: egui::Stroke,
) {
    let c = |x: usize, y: usize, z: usize| {
        [
            if x == 0 { mn[0] } else { mx[0] },
            if y == 0 { mn[1] } else { mx[1] },
            if z == 0 { mn[2] } else { mx[2] },
        ]
    };
    const EDGES: [Edge; 12] = [
        ((0, 0, 0), (1, 0, 0)),
        ((0, 1, 0), (1, 1, 0)),
        ((0, 0, 1), (1, 0, 1)),
        ((0, 1, 1), (1, 1, 1)),
        ((0, 0, 0), (0, 1, 0)),
        ((1, 0, 0), (1, 1, 0)),
        ((0, 0, 1), (0, 1, 1)),
        ((1, 0, 1), (1, 1, 1)),
        ((0, 0, 0), (0, 0, 1)),
        ((1, 0, 0), (1, 0, 1)),
        ((0, 1, 0), (0, 1, 1)),
        ((1, 1, 0), (1, 1, 1)),
    ];
    for (a, b) in EDGES {
        draw_line3(
            painter,
            rect,
            scale,
            proj,
            c(a.0, a.1, a.2),
            c(b.0, b.1, b.2),
            stroke,
        );
    }
}

fn draw_line3(
    painter: &egui::Painter,
    rect: egui::Rect,
    scale: f32,
    proj: &impl Fn([f32; 3]) -> [f32; 3],
    a: [f32; 3],
    b: [f32; 3],
    stroke: egui::Stroke,
) {
    let mut ca = proj(a);
    let mut cb = proj(b);
    if ca[2] <= NEAR && cb[2] <= NEAR {
        return;
    }
    // clip the behind endpoint to the near plane
    if ca[2] <= NEAR {
        let t = (NEAR - ca[2]) / (cb[2] - ca[2]);
        for k in 0..3 {
            ca[k] += (cb[k] - ca[k]) * t;
        }
    } else if cb[2] <= NEAR {
        let t = (NEAR - cb[2]) / (ca[2] - cb[2]);
        for k in 0..3 {
            cb[k] += (ca[k] - cb[k]) * t;
        }
    }
    painter.line_segment(
        [
            cam_to_screen(ca, rect, scale),
            cam_to_screen(cb, rect, scale),
        ],
        stroke,
    );
}
