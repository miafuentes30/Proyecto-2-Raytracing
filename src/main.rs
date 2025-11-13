// main.rs 
mod camera;
mod ray;
mod material;
mod cube;
mod light;
mod color;
mod skybox;
mod texture;
mod mesh;

use raylib::prelude::*;
use rayon::prelude::*;
use camera::Camera;
use ray::Ray as CustomRay;
use cube::Cube;
use light::Light;
use color::Color as CustomColor;
use material::Material;
use skybox::Skybox;
use texture::TextureManager;
use mesh::Mesh;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Diorama Raytracer - Threaded")
        .build();

    rl.set_target_fps(60);

    let mut camera = Camera::new(
        Vector3::new(0.0, 5.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        70.0
    );

    let texture_manager = TextureManager::new();
    let cubes = create_scene();
    let meshes = load_meshes();
    
    let mut sun = Light::new(
        Vector3::new(5.0, 10.0, 5.0),
        CustomColor::new(1.0, 1.0, 0.9),
        2.5
    );

    let skybox = Skybox::new(&mut rl, &thread);

    let mut time: f32 = 0.0;
    let mut world_angle: f32 = 0.0; // rotación del diorama
    let mut camera_angle: f32 = 0.0;
    let mut camera_pitch: f32 = 0.2; // inclinación hacia abajo
    let mut camera_distance: f32 = 10.0;

    let mut image = Image::gen_image_color(WIDTH, HEIGHT, Color::BLACK);

    println!("Raytracer inicia bn!");
    println!("Controles:");
    println!("  IZQUIERDA/DERECHA - Rotar mundo");
    println!("  ARRIBA/ABAJO - Mirar arriba/abajo (pitch)");
    println!("  Q/E - Acercar/Alejar");
    println!("  CLIC DERECHO - Orbitar cámara");
    println!("  RUEDA MOUSE - Zoom");
    println!("  ESC - Salir");

    while !rl.window_should_close() {
        let frame_time = rl.get_frame_time();
        time += frame_time;
        // world_angle += 0.4 * frame_time; 

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            world_angle -= 0.8 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            world_angle += 0.8 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            camera_pitch += 1.0 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            camera_pitch -= 1.0 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            camera_distance -= 5.0 * frame_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            camera_distance += 5.0 * frame_time;
        }
        camera_pitch = camera_pitch.clamp(-1.4, 1.4);

        // controles de mouse: arrastrar derecho para orbitar, rueda para zoom
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let md = rl.get_mouse_delta();
            let sensitivity = 0.0035f32;
            camera_angle -= md.x * sensitivity;
            camera_pitch = (camera_pitch - md.y * sensitivity).clamp(-1.2, 1.2);
        }
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 { camera_distance -= wheel * 1.0; }

        camera_distance = camera_distance.clamp(3.0, 25.0);

        // orbitar cámara alrededor del objetivo
        let target = Vector3::new(0.0, 1.5, 0.0);
        let x = camera_distance * camera_angle.cos() * camera_pitch.cos();
        let y = camera_distance * camera_pitch.sin();
        let z = camera_distance * camera_angle.sin() * camera_pitch.cos();
        camera.position = target + Vector3::new(x, y, z);
        camera.look_at = target;

        let day_progress = (time * 0.2).sin();
        sun.position = Vector3::new(
            15.0 * (time * 0.2).cos(),
            15.0 * day_progress.abs(),
            15.0 * (time * 0.2).sin()
        );
        
        sun.color = if day_progress > 0.3 {
            CustomColor::new(1.0, 1.0, 0.9)
        } else if day_progress > -0.3 {
            CustomColor::new(1.0, 0.6, 0.3)
        } else {
            CustomColor::new(0.2, 0.2, 0.4)
        };

        sun.intensity = (day_progress.max(0.3) * 2.5).max(0.8);

        // luces: sol + luces de relleno + cubos emisivos
        let mut lights: Vec<Light> = vec![Light::new(sun.position, sun.color, sun.intensity)];
        lights.push(Light::new(Vector3::new(-5.0, 8.0, 0.0), CustomColor::new(0.8, 0.9, 1.0), 1.2));
        lights.push(Light::new(Vector3::new(5.0, 6.0, -5.0), CustomColor::new(1.0, 0.95, 0.85), 1.0));
        for cube in &cubes {
            if let Some(em) = cube.material.emission {
                let inten = (em.r + em.g + em.b) / 3.0;
                lights.push(Light::new(cube.center, em, inten.max(1.5)));
            }
        }

        render_threaded(&camera, &cubes, &meshes, &lights, &skybox, &texture_manager, &mut image, time, world_angle);

        let texture = rl.load_texture_from_image(&thread, &image).unwrap();
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_fps(10, 10);
        d.draw_text(&format!("Time: {:.1}s", time), 10, 30, 20, Color::WHITE);
        d.draw_text(&format!("Day: {:.0}%", (day_progress + 1.0) * 50.0), 10, 50, 20, Color::WHITE);
    }
}

fn render_threaded(
    camera: &Camera,
    cubes: &Vec<Cube>,
    meshes: &Vec<Mesh>,
    lights: &Vec<Light>,
    skybox: &Skybox,
    textures: &TextureManager,
    image: &mut Image,
    time: f32,
    world_angle: f32,
) {
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let width = WIDTH as usize;
    let height = HEIGHT as usize;

    let pixels: Vec<(usize, usize, Color)> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width)
                .into_par_iter()
                .map(|x| {
                    let u = (x as f32 / WIDTH as f32) * 2.0 - 1.0;
                    let v = -((y as f32 / HEIGHT as f32) * 2.0 - 1.0);

                    let ray = camera.get_ray(u * aspect_ratio, v);
                        let color = cast_ray(&ray, cubes, meshes, lights, skybox, textures, 0, time, world_angle);
                    
                    let r = (color.r.clamp(0.0, 1.0) * 255.0) as u8;
                    let g = (color.g.clamp(0.0, 1.0) * 255.0) as u8;
                    let b = (color.b.clamp(0.0, 1.0) * 255.0) as u8;
                    
                    (x, y, Color::new(r, g, b, 255))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for (x, y, color) in pixels {
        image.draw_pixel(x as i32, y as i32, color);
    }
}

fn cast_ray(
    ray: &CustomRay,
    cubes: &Vec<Cube>,
        meshes: &Vec<Mesh>,
    lights: &Vec<Light>,
    skybox: &Skybox,
    textures: &TextureManager,
    depth: u32,
    time: f32,
    world_angle: f32,
) -> CustomColor {
    if depth > 2 {
        return CustomColor::new(0.0, 0.0, 0.0);
    }

    // rotar rayo al espacio del objeto
    let rr = rotate_ray_y(ray, -world_angle);

    let mut closest_t = f32::INFINITY;
    let mut hit_cube: Option<(&Cube, f32, f32)> = None;
    let mut hit_mesh: Option<(&Mesh, f32, f32, Vector3)> = None;

    for cube in cubes {
        if let Some((t, u, v)) = cube.intersect_with_uv(&rr) {
            if t < closest_t && t > 0.001 {
                closest_t = t;
                hit_cube = Some((cube, u, v));
                hit_mesh = None;
            }
        }
    }

    // probar intersecciones con meshes
    for m in meshes {
        if let Some((t, u, v, n)) = m.intersect_with_uv_normal(&rr) {
            if t < closest_t && t > 0.001 {
                closest_t = t;
                hit_mesh = Some((m, u, v, n));
                hit_cube = None;
            }
        }
    }

    if let Some((cube, u, v)) = hit_cube {
        // calcular punto de impacto y normal
        let hit_point_obj = rr.at(closest_t);
        let normal_obj = cube.normal_at(hit_point_obj);
        let hit_point = rotate_vec3_y(hit_point_obj, world_angle);
        let normal = rotate_vec3_y(normal_obj, world_angle).normalized();
        
        let texture_color = if let Some(ref tex_name) = cube.material.texture_name {
            textures.get_color(tex_name, u, v, time, cube.material.animated)
        } else {
            CustomColor::white()
        };

        let base_color = texture_color * cube.material.albedo;

        if let Some(emission) = cube.material.emission {
            // emisivo: textura * emisión
            return texture_color * emission;
        }

        let view_dir = (ray.origin - hit_point).normalized();
        let mut color = base_color * 0.35; // ambiental 

        // luz
        for l in lights {
            let to_light = l.position - hit_point;
            let dist = to_light.length();
            let light_dir = to_light / dist;

            // verificar sombras
            let shadow_ray = CustomRay::new(hit_point + normal * 0.002, light_dir);
            let mut in_shadow = false;
            for oc in cubes {
                if let Some((t, _, _)) = oc.intersect_with_uv(&shadow_ray) {
                    if t > 0.001 && t < dist - 0.001 {
                        in_shadow = true;
                        break;
                    }
                }
            }
            if in_shadow { continue; }

            let ndotl = normal.dot(light_dir).max(0.0);
            let half_dir = (light_dir + view_dir).normalized();
            let specular = normal.dot(half_dir).max(0.0).powf(32.0) * cube.material.specular;

            // Distancia
            let attenuation = 1.0 / (1.0 + 0.15 * dist + 0.05 * dist * dist);
            let contrib = (base_color * ndotl + CustomColor::white() * specular) * l.intensity * attenuation;
            color = color + contrib * l.color;
        }

        // refleccion
        if cube.material.reflectivity > 0.0 {
            let reflect_dir = ray.direction - normal * 2.0 * ray.direction.dot(normal);
            let reflect_ray = CustomRay::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_color = cast_ray(&reflect_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);
            color = color * (1.0 - cube.material.reflectivity) + reflect_color * cube.material.reflectivity;
        }

        // refracción
        if cube.material.transparency > 0.0 {
            let mut n = normal;
            let mut eta = 1.0 / cube.material.refractive_index.max(1e-3);
            let cosi = (-ray.direction).dot(n).clamp(-1.0, 1.0);
            let entering = cosi > 0.0;
            let cosi_abs = cosi.abs();
            if !entering {
                n = -n;
                eta = 1.0 / eta; 
            }

            let k = 1.0 - eta * eta * (1.0 - cosi_abs * cosi_abs);
            let mut refract_color = CustomColor::black();
            let reflect_dir = ray.direction - normal * 2.0 * ray.direction.dot(normal);
            let reflect_ray = CustomRay::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_col = cast_ray(&reflect_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);

            let r0 = ((1.0 - cube.material.refractive_index) / (1.0 + cube.material.refractive_index)).powi(2);
            let fresnel = r0 + (1.0 - r0) * (1.0 - cosi_abs).powi(5);

            if k >= 0.0 {
                let refract_dir = ray.direction * eta + n * (eta * cosi_abs - k.sqrt());
                let refr_ray = CustomRay::new(hit_point - n * 0.001, refract_dir.normalized());
                refract_color = cast_ray(&refr_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);
                color = color * (1.0 - cube.material.transparency)
                    + (reflect_col * fresnel + refract_color * (1.0 - fresnel)) * cube.material.transparency;
            } else {
                // Reflexion interna total
                color = color * (1.0 - cube.material.transparency) + reflect_col * cube.material.transparency;
            }
        }

        color
    } else if let Some((mesh, u, v, n_obj)) = hit_mesh {
        let hit_point_obj = rr.at(closest_t);
        let normal = rotate_vec3_y(n_obj, world_angle).normalized();
        let hit_point = rotate_vec3_y(hit_point_obj, world_angle);

        let texture_color = if let Some(ref tex_name) = mesh.material.texture_name {
            textures.get_color(tex_name, u, v, time, mesh.material.animated)
        } else {
            CustomColor::white()
        };
        let base_color = texture_color * mesh.material.albedo;

        if let Some(emission) = mesh.material.emission { 
            return texture_color * emission; 
        }

        let view_dir = (ray.origin - hit_point).normalized();
        let mut color = base_color * 0.35; // ambiental 
        for l in lights {
            let to_light = l.position - hit_point;
            let dist = to_light.length();
            let light_dir = to_light / dist;
            let shadow_ray = CustomRay::new(hit_point + normal * 0.002, light_dir);
            let mut in_shadow = false;
            for oc in cubes {
                if let Some((t, _, _)) = oc.intersect_with_uv(&shadow_ray) {
                    if t > 0.001 && t < dist - 0.001 { in_shadow = true; break; }
                }
            }
            if !in_shadow {
                for om in meshes {
                    if let Some((t, _, _, _)) = om.intersect_with_uv_normal(&shadow_ray) {
                        if t > 0.001 && t < dist - 0.001 { in_shadow = true; break; }
                    }
                }
            }
            if in_shadow { continue; }
            let ndotl = normal.dot(light_dir).max(0.0);
            let half_dir = (light_dir + view_dir).normalized();
            let specular = normal.dot(half_dir).max(0.0).powf(32.0) * mesh.material.specular;
            let attenuation = 1.0 / (1.0 + 0.15 * dist + 0.05 * dist * dist);
            let contrib = (base_color * ndotl + CustomColor::white() * specular) * l.intensity * attenuation;
            color = color + contrib * l.color;
        }

        // Reflection
        if mesh.material.reflectivity > 0.0 {
            let reflect_dir = ray.direction - normal * 2.0 * ray.direction.dot(normal);
            let reflect_ray = CustomRay::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_color = cast_ray(&reflect_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);
            color = color * (1.0 - mesh.material.reflectivity) + reflect_color * mesh.material.reflectivity;
        }

        // Refraccon
        if mesh.material.transparency > 0.0 {
            let mut n = normal;
            let mut eta = 1.0 / mesh.material.refractive_index.max(1e-3);
            let cosi = (-ray.direction).dot(n).clamp(-1.0, 1.0);
            let entering = cosi > 0.0;
            let cosi_abs = cosi.abs();
            if !entering { n = -n; eta = 1.0 / eta; }
            let k = 1.0 - eta * eta * (1.0 - cosi_abs * cosi_abs);
            let reflect_dir = ray.direction - normal * 2.0 * ray.direction.dot(normal);
            let reflect_ray = CustomRay::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_col = cast_ray(&reflect_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);
            let r0 = ((1.0 - mesh.material.refractive_index) / (1.0 + mesh.material.refractive_index)).powi(2);
            let fresnel = r0 + (1.0 - r0) * (1.0 - cosi_abs).powi(5);
            if k >= 0.0 {
                let refract_dir = ray.direction * eta + n * (eta * cosi_abs - k.sqrt());
                let refr_ray = CustomRay::new(hit_point - n * 0.001, refract_dir.normalized());
                let refr_col = cast_ray(&refr_ray, cubes, meshes, lights, skybox, textures, depth + 1, time, world_angle);
                color = color * (1.0 - mesh.material.transparency)
                    + (reflect_col * fresnel + refr_col * (1.0 - fresnel)) * mesh.material.transparency;
            } else {
                color = color * (1.0 - mesh.material.transparency) + reflect_col * mesh.material.transparency;
            }
        }

        color
    } else {
        skybox.get_color(&ray.direction)
    }
}

fn load_meshes() -> Vec<Mesh> {
    let mut result = Vec::new();
    // Cargar modelo de Steve
    if let Ok(mesh) = Mesh::from_obj("assets/models/Steve.obj", Vector3::new(0.0, 0.5, -3.5), 1.2, material::Material::new(
        material::MaterialType::Diffuse,
        CustomColor::new(1.0, 1.0, 1.0),
        0.2, 0.0, 0.0, 1.0,
        None,
        Some("steve".to_string()),
        false,
    )) { result.push(mesh); }

    // esfera de vidrio (igual la borre xq no la uso)
    if let Ok(mesh) = Mesh::from_obj("assets/models/sphere.obj", Vector3::new(-4.0, 0.0, -2.0), 1.0, material::Material::glass()) {
        result.push(mesh);
    }
    result
}

// funciones de rotación en eje Y
fn rotate_vec3_y(v: Vector3, angle: f32) -> Vector3 {
    let (s, c) = angle.sin_cos();
    Vector3::new(c * v.x + s * v.z, v.y, -s * v.x + c * v.z)
}

fn rotate_ray_y(ray: &CustomRay, angle: f32) -> CustomRay {
    CustomRay::new(rotate_vec3_y(ray.origin, angle), rotate_vec3_y(ray.direction, angle).normalized())
}

fn create_scene() -> Vec<Cube> {
    let mut cubes = Vec::new();

    // suelo
    for x in -6..6 {
        for z in -6..6 {
            cubes.push(Cube::new(
                Vector3::new(x as f32, -0.5, z as f32),
                1.0,
                Material::grass()
            ));
        }
    }

    // paredes de la casa
    for y in 0..3 {
        for x in -2..3 {
            if !(y == 0 && x == 0) {
                cubes.push(Cube::new(
                    Vector3::new(x as f32, y as f32, -2.0),
                    1.0,
                    Material::brick()
                ));
            }
            cubes.push(Cube::new(
                Vector3::new(x as f32, y as f32, 2.0),
                1.0,
                Material::brick()
            ));
        }
        
        for z in -1..2 {
            cubes.push(Cube::new(
                Vector3::new(-2.0, y as f32, z as f32),
                1.0,
                Material::brick()
            ));
            cubes.push(Cube::new(
                Vector3::new(2.0, y as f32, z as f32),
                1.0,
                Material::brick()
            ));
        }
    }

    // ventanas
    cubes.push(Cube::new(
        Vector3::new(1.0, 1.0, -2.0),
        1.0,
        Material::glass()
    ));
    cubes.push(Cube::new(
        Vector3::new(-1.0, 1.0, -2.0),
        1.0,
        Material::glass()
    ));

    // techo
    for x in -2..3 {
        for z in -2..3 {
            cubes.push(Cube::new(
                Vector3::new(x as f32, 3.0, z as f32),
                1.0,
                Material::woodhouse()
            ));
        }
    }

    // tronco del arbol
    for y in 0..3 {
        cubes.push(Cube::new(
            Vector3::new(4.0, y as f32, 3.0),
            1.0,
            Material::wood()
        ));
    }

    // hojas
    for x in -1..2 {
        for z in -1..2 {
            for y in 3..5 {
                cubes.push(Cube::new(
                    Vector3::new(4.0 + x as f32, y as f32, 3.0 + z as f32),
                    1.0,
                    Material::leaves()
                ));
            }
        }
    }

    // agua
    for x in -5..-2 {
        for z in 3..6 {
            cubes.push(Cube::new(
                Vector3::new(x as f32, -0.3, z as f32),
                1.0,
                Material::water()
            ));
        }
    }

    // fogata
    cubes.push(Cube::new(
        Vector3::new(-3.0, 1.0, -3.0),
        0.5,
        Material::fire()
    ));

    // rocas 
    cubes.push(Cube::new(Vector3::new(-4.0, -0.3, -4.0), 0.6, Material::stone()));
    cubes.push(Cube::new(Vector3::new(5.0, -0.4, -3.0), 0.5, Material::stone()));
    cubes.push(Cube::new(Vector3::new(-3.0, -0.35, 4.5), 0.55, Material::stone()));
    cubes.push(Cube::new(Vector3::new(3.0, -0.3, 5.0), 0.7, Material::stone()));
    cubes.push(Cube::new(Vector3::new(-5.0, -0.4, 1.0), 0.5, Material::stone()));
    cubes.push(Cube::new(Vector3::new(1.5, -0.35, -5.0), 0.6, Material::stone()));

    println!("Scene created with {} cubes", cubes.len());
    cubes
}