// texture.rs
use image::{GenericImageView, Rgba};
use std::collections::HashMap;
use crate::color::Color;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Rgba<u8>>,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, image::ImageError> {
        // adivinar formato desde el contenido
        let img = image::io::Reader::open(path)?
            .with_guessed_format()?
            .decode()?;
        let (width, height) = img.dimensions();
        
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                pixels.push(img.get_pixel(x, y));
            }
        }
        
        Ok(Texture { width, height, pixels })
    }

    pub fn get_pixel(&self, u: f32, v: f32) -> Color {
        // envolver UVs
        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);
        
        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);
        
        let idx = (y * self.width + x) as usize;
        let pixel = self.pixels[idx];
        
        Color::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }

    // texturas awa y fuego
    pub fn get_pixel_animated(&self, u: f32, v: f32, time: f32) -> Color {
        // animacian de ondas
        let u_offset = (time * 0.5).sin() * 0.1;
        let v_offset = (time * 0.3).cos() * 0.1;
        
        self.get_pixel(u + u_offset, v + v_offset)
    }
}

pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut manager = TextureManager {
            textures: HashMap::new(),
        };
        
        // cargar texturas
        manager.load_texture("grass", "assets/textures/grass.png");
        manager.load_texture("wood", "assets/textures/wood.png");
        manager.load_texture("water", "assets/textures/water.png");
        manager.load_texture("glass", "assets/textures/glass.png");
        manager.load_texture("stone", "assets/textures/stone.png");
        manager.load_texture("steve", "assets/textures/steve.png");
        manager.load_texture("brick", "assets/textures/brick.png");
        manager.load_texture("woodhouse", "assets/textures/woodhouse.png");
        manager.load_texture("fire", "assets/textures/fire.png");
        
        manager
    }

    fn load_texture(&mut self, name: &str, path: &str) {
        match Texture::load(path) {
            Ok(texture) => {
                println!("cargaron texturas: {}", name);
                self.textures.insert(name.to_string(), texture);
            }
            Err(e) => {
                eprintln!("No cargaron texturas {}: {}", path, e);
                // textura de respaldo
                self.textures.insert(name.to_string(), Self::create_fallback_texture(name));
            }
        }
    }

    fn create_fallback_texture(name: &str) -> Texture {
        // textura de respaldo 2x2
        let color = match name {
            "grass" => Rgba([50, 200, 50, 255]),
            "wood" => Rgba([150, 100, 50, 255]),
            "water" => Rgba([50, 100, 200, 255]),
            "glass" => Rgba([200, 220, 255, 255]),
            "stone" => Rgba([120, 120, 120, 255]),
            "brick" => Rgba([150, 80, 60, 255]),
            "woodhouse" => Rgba([120, 80, 40, 255]),
            "fire" => Rgba([255, 200, 0, 255]),
            _ => Rgba([255, 0, 255, 255]), 
        };
        
        Texture {
            width: 2,
            height: 2,
            pixels: vec![color; 4],
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn get_color(&self, name: &str, u: f32, v: f32, time: f32, animated: bool) -> Color {
        if let Some(texture) = self.get_texture(name) {
            if animated {
                texture.get_pixel_animated(u, v, time)
            } else {
                texture.get_pixel(u, v)
            }
        } else {
            // respaldo
            Color::new(1.0, 0.0, 1.0)
        }
    }
}