// camera.rs
use raylib::prelude::*;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vector3,
    pub look_at: Vector3,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vector3, look_at: Vector3, fov: f32) -> Self {
        Camera { position, look_at, fov }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let forward = (self.look_at - self.position).normalized();
        let right = Vector3::new(0.0, 1.0, 0.0).cross(forward).normalized();
        let up = forward.cross(right);

        let direction = (forward + right * u + up * v).normalized();
        Ray::new(self.position, direction)
    }
}