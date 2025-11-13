use raylib::prelude::*;
use crate::color::Color;
use crate::texture::Texture;

pub struct Skybox {
    // gradiente de respaldo
    day_color: Color,
    night_color: Color,
    horizon_color: Color,
    // cubemap
    right: Option<Texture>,
    left: Option<Texture>,
    top: Option<Texture>,
    bottom: Option<Texture>,
    front: Option<Texture>,
    back: Option<Texture>,
}

impl Skybox {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        let right = Texture::load("assets/skybox/right.png").ok();
        let left = Texture::load("assets/skybox/left.png").ok();
        let top = Texture::load("assets/skybox/top.png").ok();
        let bottom = Texture::load("assets/skybox/bottom.png").ok();
        let front = Texture::load("assets/skybox/front.png").ok();
        let back = Texture::load("assets/skybox/back.png").ok();

        if right.is_some() && left.is_some() && top.is_some() && bottom.is_some() && front.is_some() && back.is_some() {
            println!(" Skybox texturas cargaron bn");
        } else {
            eprintln!("Skybox texturas no cargaron");
        }

        Skybox {
            day_color: Color::new(0.5, 0.7, 1.0),
            night_color: Color::new(0.05, 0.05, 0.15),
            horizon_color: Color::new(0.8, 0.6, 0.4),
            right,
            left,
            top,
            bottom,
            front,
            back,
        }
    }

    pub fn get_color(&self, direction: &Vector3) -> Color {
        if let (Some(right), Some(left), Some(top), Some(bottom), Some(front), Some(back)) = (
            &self.right, &self.left, &self.top, &self.bottom, &self.front, &self.back
        ) {
            // muestrear cubemap
            let x = direction.x;
            let y = direction.y;
            let z = direction.z;
            let ax = x.abs();
            let ay = y.abs();
            let az = z.abs();

            // elegir cara
            if ax >= ay && ax >= az {
                // cara X
                if x > 0.0 {
                    // +X derecha
                    let u = (-z / ax + 1.0) * 0.5;
                    let v = (-y / ax + 1.0) * 0.5;
                    return right.get_pixel(u, v);
                } else {
                    // -X izquierda
                    let u = (z / ax + 1.0) * 0.5;
                    let v = (-y / ax + 1.0) * 0.5;
                    return left.get_pixel(u, v);
                }
            } else if ay >= ax && ay >= az {
                // cara Y
                if y > 0.0 {
                    // +Y arriba
                    let u = (x / ay + 1.0) * 0.5;
                    let v = (z / ay + 1.0) * 0.5;
                    return top.get_pixel(u, v);
                } else {
                    // -Y abajo
                    let u = (x / ay + 1.0) * 0.5;
                    let v = (-z / ay + 1.0) * 0.5;
                    return bottom.get_pixel(u, v);
                }
            } else {
                // cara Z
                if z > 0.0 {
                    // +Z frente
                    let u = (x / az + 1.0) * 0.5;
                    let v = (-y / az + 1.0) * 0.5;
                    return front.get_pixel(u, v);
                } else {
                    // -Z atrás
                    let u = (-x / az + 1.0) * 0.5;
                    let v = (-y / az + 1.0) * 0.5;
                    return back.get_pixel(u, v);
                }
            }
        }

        // gradiente de respaldo
        let t = (direction.y + 1.0) * 0.5;
        if t > 0.5 {
            self.day_color * t + self.horizon_color * (1.0 - t)
        } else {
            self.horizon_color * t + self.night_color * (1.0 - t)
        }
    }
}