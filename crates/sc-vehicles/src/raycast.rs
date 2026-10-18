//! Ray casts against a hull's meshes: where a shot lands and on which damage
//! part.

use crate::PartMesh;

/// A bounding-volume hierarchy over every triangle of a set of part meshes
/// (usually [`crate::HullGeometry::collision`]), built once per hull.
pub struct Raycaster {
    triangles: Vec<Triangle>,
    nodes: Vec<Node>,
}

struct Triangle {
    v: [[f32; 3]; 3],
    part: usize,
}

struct Node {
    min: [f32; 3],
    max: [f32; 3],
    /// Leaf: `[first, count]` triangles; inner: `[left, right]` children
    /// with `count == u32::MAX`.
    a: u32,
    b: u32,
    leaf: bool,
}

/// Where a ray meets the hull.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayHit {
    /// Index into the part meshes the caster was built from.
    pub part: usize,
    /// Distance along the ray, m.
    pub distance: f32,
    pub point: [f32; 3],
    /// The triangle's unit normal, facing the ray.
    pub normal: [f32; 3],
}

const LEAF: usize = 8;

impl Raycaster {
    pub fn new(parts: &[PartMesh]) -> Self {
        let mut triangles = Vec::new();
        for (part, mesh) in parts.iter().enumerate() {
            for t in mesh.indices.as_chunks::<3>().0 {
                triangles.push(Triangle {
                    v: t.map(|i| mesh.positions[i as usize]),
                    part,
                });
            }
        }
        let mut r = Self {
            triangles,
            nodes: Vec::new(),
        };
        if !r.triangles.is_empty() {
            let n = r.triangles.len();
            r.build(0, n);
        }
        r
    }

    /// Builds the node over `triangles[first..first + count]`; returns its
    /// index.
    fn build(&mut self, first: usize, count: usize) -> u32 {
        let (min, max) = bounds(&self.triangles[first..first + count]);
        let index = self.nodes.len();
        self.nodes.push(Node {
            min,
            max,
            a: first as u32,
            b: count as u32,
            leaf: true,
        });
        if count <= LEAF {
            return index as u32;
        }
        // Split at the median centroid along the longest axis.
        let extent: [f32; 3] = std::array::from_fn(|k| max[k] - min[k]);
        let axis = (0..3)
            .max_by(|&a, &b| extent[a].total_cmp(&extent[b]))
            .unwrap_or(0);
        let slice = &mut self.triangles[first..first + count];
        let mid = count / 2;
        slice.select_nth_unstable_by(mid, |x, y| centroid(x)[axis].total_cmp(&centroid(y)[axis]));
        let left = self.build(first, mid);
        let right = self.build(first + mid, count - mid);
        let node = &mut self.nodes[index];
        node.a = left;
        node.b = right;
        node.leaf = false;
        index as u32
    }

    /// The nearest hull triangle `origin + t × direction` meets (`t > 0`).
    pub fn cast(&self, origin: [f32; 3], direction: [f32; 3]) -> Option<RayHit> {
        if self.nodes.is_empty() {
            return None;
        }
        let inv: [f32; 3] = std::array::from_fn(|k| 1.0 / direction[k]);
        let mut best: Option<(f32, usize)> = None;
        let mut stack = vec![0u32];
        while let Some(i) = stack.pop() {
            let node = &self.nodes[i as usize];
            let limit = best.map_or(f32::INFINITY, |b| b.0);
            if !slab(node, origin, inv, limit) {
                continue;
            }
            if node.leaf {
                for t in node.a as usize..(node.a + node.b) as usize {
                    if let Some(d) = intersect(&self.triangles[t].v, origin, direction)
                        && d < best.map_or(f32::INFINITY, |b| b.0)
                    {
                        best = Some((d, t));
                    }
                }
            } else {
                stack.push(node.a);
                stack.push(node.b);
            }
        }
        let (distance, t) = best?;
        let tri = &self.triangles[t];
        let mut normal = normalize(cross(sub(tri.v[1], tri.v[0]), sub(tri.v[2], tri.v[0])));
        if dot(normal, direction) > 0.0 {
            normal = normal.map(|c| -c);
        }
        Some(RayHit {
            part: tri.part,
            distance,
            point: std::array::from_fn(|k| origin[k] + direction[k] * distance),
            normal,
        })
    }
}

fn centroid(t: &Triangle) -> [f32; 3] {
    std::array::from_fn(|k| (t.v[0][k] + t.v[1][k] + t.v[2][k]) / 3.0)
}

fn bounds(ts: &[Triangle]) -> ([f32; 3], [f32; 3]) {
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for t in ts {
        for v in &t.v {
            for k in 0..3 {
                lo[k] = lo[k].min(v[k]);
                hi[k] = hi[k].max(v[k]);
            }
        }
    }
    (lo, hi)
}

/// Whether the ray meets the node's box before `limit`.
fn slab(n: &Node, o: [f32; 3], inv: [f32; 3], limit: f32) -> bool {
    let (mut t0, mut t1) = (0.0f32, limit);
    for k in 0..3 {
        let (a, b) = ((n.min[k] - o[k]) * inv[k], (n.max[k] - o[k]) * inv[k]);
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        t0 = t0.max(lo);
        t1 = t1.min(hi);
        if t0 > t1 {
            return false;
        }
    }
    true
}

/// Möller–Trumbore: distance to the triangle, both faces.
fn intersect(v: &[[f32; 3]; 3], o: [f32; 3], d: [f32; 3]) -> Option<f32> {
    let (e1, e2) = (sub(v[1], v[0]), sub(v[2], v[0]));
    let p = cross(d, e2);
    let det = dot(e1, p);
    if det.abs() < 1e-9 {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(o, v[0]);
    let u = dot(s, p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross(s, e1);
    let w = dot(d, q) * inv;
    if w < 0.0 || u + w > 1.0 {
        return None;
    }
    let t = dot(e2, q) * inv;
    (t > 1e-6).then_some(t)
}

pub(crate) fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(crate) fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub(crate) fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = dot(v, v).sqrt();
    if l > 0.0 { v.map(|c| c / l) } else { v }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_the_nearest_face() {
        // Two parallel squares, x = 1 (part 0) and x = 2 (part 1).
        let quad = |x: f32| PartMesh {
            part: format!("x{x}"),
            positions: vec![
                [x, -1.0, -1.0],
                [x, 1.0, -1.0],
                [x, 1.0, 1.0],
                [x, -1.0, 1.0],
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        };
        let r = Raycaster::new(&[quad(2.0), quad(1.0)]);
        let hit = r.cast([0.0, 0.2, 0.3], [1.0, 0.0, 0.0]).expect("a hit");
        assert_eq!(hit.part, 1);
        assert!((hit.distance - 1.0).abs() < 1e-5);
        assert_eq!(hit.normal, [-1.0, 0.0, 0.0]);
        assert!(r.cast([0.0, 3.0, 0.0], [1.0, 0.0, 0.0]).is_none());
    }
}
