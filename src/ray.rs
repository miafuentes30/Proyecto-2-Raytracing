use raylib::prelude::*;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction: direction.normalized() }
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}
