// cube.rs
use raylib::prelude::*;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct Cube {
    pub center: Vector3,
    pub size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        Cube { center, size, material }
    }

    pub fn intersect_with_uv(&self, ray: &Ray) -> Option<(f32, f32, f32)> {
        let half_size = self.size / 2.0;
        let min = self.center - Vector3::new(half_size, half_size, half_size);
        let max = self.center + Vector3::new(half_size, half_size, half_size);

        let mut tmin = (min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if tmin > tymax || tymin > tmax {
            return None;
        }

        tmin = tmin.max(tymin);
        tmax = tmax.min(tymax);

        let mut tzmin = (min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return None;
        }

        tmin = tmin.max(tzmin);

        if tmin <= 0.0 {
            return None;
        }

        // calcular UV
        let hit_point = ray.at(tmin);
        let local = hit_point - self.center;
        
        // determinar qué cara fue golpeada y calcular UVs
        let (u, v) = self.calculate_uv(local);

        Some((tmin, u, v))
    }

    fn calculate_uv(&self, local: Vector3) -> (f32, f32) {
        let half_size = self.size / 2.0;

        let dx = (local.x.abs() - half_size).abs();
        let dy = (local.y.abs() - half_size).abs();
        let dz = (local.z.abs() - half_size).abs();

        let min_dist = dx.min(dy).min(dz);
        
        if min_dist == dx {
            // X 
            ((local.z + half_size) / self.size, (local.y + half_size) / self.size)
        } else if min_dist == dy {
            // Y 
            ((local.x + half_size) / self.size, (local.z + half_size) / self.size)
        } else {
            // Z 
            ((local.x + half_size) / self.size, (local.y + half_size) / self.size)
        }
    }

    pub fn normal_at(&self, point: Vector3) -> Vector3 {
        let local = point - self.center;
        let half_size = self.size / 2.0;
        
        let dx = (local.x - half_size).abs().min((local.x + half_size).abs());
        let dy = (local.y - half_size).abs().min((local.y + half_size).abs());
        let dz = (local.z - half_size).abs().min((local.z + half_size).abs());

        if dx < dy && dx < dz {
            Vector3::new(local.x.signum(), 0.0, 0.0)
        } else if dy < dz {
            Vector3::new(0.0, local.y.signum(), 0.0)
        } else {
            Vector3::new(0.0, 0.0, local.z.signum())
        }
    }
}