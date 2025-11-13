use raylib::prelude::*;
use crate::material::Material;
use crate::ray::Ray;

pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub uv0: Option<Vector2>,
    pub uv1: Option<Vector2>,
    pub uv2: Option<Vector2>,
    pub n: Vector3,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub position: Vector3,
    pub scale: f32,
    pub material: Material,
}

impl Mesh {
    // parsear OBJ y retornar mesh
    pub fn from_obj(path: &str, position: Vector3, scale: f32, material: Material) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut positions: Vec<Vector3> = Vec::new();
        let mut uvs: Vec<Vector2> = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    positions.push(Vector3::new(x, y, z));
                }
            } else if line.starts_with("vt ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let u: f32 = parts[1].parse().unwrap_or(0.0);
                    let v: f32 = parts[2].parse().unwrap_or(0.0);
                    uvs.push(Vector2::new(u, v));
                }
            } else if line.starts_with("f ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 { continue; }
                // triangulación 
                let mut idx: Vec<(usize, Option<usize>)> = Vec::new();
                for i in 1..parts.len() {
                    let token = parts[i];
                    let mut v_index: Option<usize> = None;
                    let mut vt_index: Option<usize> = None;
                    let splits: Vec<&str> = token.split('/').collect();
                    if !splits[0].is_empty() {
                        let vi: i32 = splits[0].parse().unwrap_or(0);
                        if vi != 0 {
                            v_index = Some((if vi > 0 { (vi - 1) as usize } else { (positions.len() as i32 + vi) as usize }));
                        }
                    }
                    if splits.len() > 1 && !splits[1].is_empty() {
                        let ti: i32 = splits[1].parse().unwrap_or(0);
                        if ti != 0 {
                            vt_index = Some((if ti > 0 { (ti - 1) as usize } else { (uvs.len() as i32 + ti) as usize }));
                        }
                    }
                    if let Some(vi) = v_index { idx.push((vi, vt_index)); }
                }
                if idx.len() >= 3 {
                    for k in 1..(idx.len() - 1) {
                        let (i0, t0) = idx[0];
                        let (i1, t1) = idx[k];
                        let (i2, t2) = idx[k + 1];
                        let p0 = positions[i0];
                        let p1 = positions[i1];
                        let p2 = positions[i2];
                        let n = (p1 - p0).cross(p2 - p0).normalized();
                        let uv0 = t0.map(|ti| uvs[ti]);
                        let uv1 = t1.map(|ti| uvs[ti]);
                        let uv2 = t2.map(|ti| uvs[ti]);
                        triangles.push(Triangle { v0: p0, v1: p1, v2: p2, uv0, uv1, uv2, n });
                    }
                }
            }
        }

        Ok(Mesh { triangles, position, scale, material })
    }

    // intersección rayo-mesh con UVs y normal
    pub fn intersect_with_uv_normal(&self, ray: &Ray) -> Option<(f32, f32, f32, Vector3)> {
        // transformar a espacio local
        let ro = (ray.origin - self.position) / self.scale;
        let rd = ray.direction;
        let mut best_t_local = f32::INFINITY;
        let mut best_uv: (f32, f32) = (0.0, 0.0);
        let mut best_n = Vector3::zero();

        for tri in &self.triangles {
            if let Some((t, u, v)) = intersect_triangle(ro, rd, tri) {
                if t > 0.001 && t < best_t_local {
                    best_t_local = t;
                    if let (Some(uv0), Some(uv1), Some(uv2)) = (tri.uv0, tri.uv1, tri.uv2) {
                        let w = 1.0 - u - v;
                        let uv = uv0 * w + uv1 * u + uv2 * v;
                        best_uv = (uv.x, 1.0 - uv.y);
                    } else {
                        best_uv = (u, v);
                    }
                    best_n = tri.n;
                }
            }
        }

        if best_t_local.is_finite() && best_t_local < f32::INFINITY {
            Some((best_t_local * self.scale, best_uv.0, best_uv.1, best_n))
        } else {
            None
        }
    }
}

fn intersect_triangle(ro: Vector3, rd: Vector3, tri: &Triangle) -> Option<(f32, f32, f32)> {
    // Möller-Trumbore
    let v0v1 = tri.v1 - tri.v0;
    let v0v2 = tri.v2 - tri.v0;
    let pvec = rd.cross(v0v2);
    let det = v0v1.dot(pvec);
    if det.abs() < 1e-8 { return None; }
    let inv_det = 1.0 / det;
    let tvec = ro - tri.v0;
    let u = tvec.dot(pvec) * inv_det;
    if u < 0.0 || u > 1.0 { return None; }
    let qvec = tvec.cross(v0v1);
    let v = rd.dot(qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 { return None; }
    let t = v0v2.dot(qvec) * inv_det;
    if t <= 0.0 { return None; }
    Some((t, u, v))
}
