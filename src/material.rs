// material.rs
use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Diffuse,
    Reflective,
    Refractive,
    Emissive,
}

#[derive(Clone)]
pub struct Material {
    pub material_type: MaterialType,
    pub albedo: Color,           // Base color 
    pub specular: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub emission: Option<Color>,
    pub texture_name: Option<String>,
    pub animated: bool,   
}

impl Material {
    pub fn new(
        material_type: MaterialType,
        albedo: Color,
        specular: f32,
        reflectivity: f32,
        transparency: f32,
        refractive_index: f32,
        emission: Option<Color>,
        texture_name: Option<String>,
        animated: bool,
    ) -> Self {
        Material {
            material_type,
            albedo,
            specular,
            reflectivity,
            transparency,
            refractive_index,
            emission,
            texture_name,
            animated,
        }
    }

    // materiales predefinidos

    pub fn grass() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(1.0, 1.0, 1.0),
            0.0,
            0.0,
            0.0,
            1.0,
            None,
            Some("grass".to_string()),
            false,
        )
    }

    pub fn wood() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(1.0, 1.0, 1.0),
            0.1,
            0.0,
            0.0,
            1.0,
            None,
            Some("wood".to_string()),
            false,
        )
    }

    pub fn water() -> Self {
        Material::new(
            MaterialType::Reflective,
            Color::new(0.9, 0.95, 1.0),
            0.8,
            0.6,
            0.3,
            1.33,
            None,
            Some("water".to_string()),
            true,
        )
    }

    pub fn glass() -> Self {
        Material::new(
            MaterialType::Refractive,
            Color::new(0.95, 0.95, 1.0),
            0.9,
            0.4,
            0.8,
            1.5,
            None,
            Some("glass".to_string()),
            false,
        )
    }

    pub fn stone() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(1.0, 1.0, 1.0),
            0.2,
            0.0,
            0.0,
            1.0,
            None,
            Some("stone".to_string()),
            false,
        )
    }

    pub fn leaves() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(0.8, 1.0, 0.8),
            0.0,
            0.0,
            0.0,
            1.0,
            None,
            Some("grass".to_string()),
            false,
        )
    }

    pub fn brick() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(1.0, 1.0, 1.0),
            0.1,
            0.0,
            0.0,
            1.0,
            None,
            Some("brick".to_string()),
            false,
        )
    }

    pub fn woodhouse() -> Self {
        Material::new(
            MaterialType::Diffuse,
            Color::new(1.0, 1.0, 1.0),
            0.1,
            0.0,
            0.0,
            1.0,
            None,
            Some("woodhouse".to_string()),
            false,
        )
    }

    pub fn fire() -> Self {
        Material::new(
            MaterialType::Emissive,
            Color::new(1.0, 0.8, 0.2),
            0.0,
            0.0,
            0.0,
            1.0,
            Some(Color::new(1.0, 0.6, 0.1) * 8.0),
            Some("fire".to_string()),
            true,
        )
    }

    pub fn torch() -> Self {
        Material::new(
            MaterialType::Emissive,
            Color::new(1.0, 0.6, 0.2),
            0.0,
            0.0,
            0.0,
            1.0,
            Some(Color::new(1.0, 0.5, 0.1) * 3.0), 
            None,
            false,
        )
    }

    pub fn lamp() -> Self {
        Material::new(
            MaterialType::Emissive,
            Color::new(1.0, 1.0, 0.8),
            0.0,
            0.0,
            0.0,
            1.0,
            Some(Color::new(1.0, 1.0, 0.8) * 2.0),
            None,
            false,
        )
    }
}