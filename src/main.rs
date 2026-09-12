mod bmp;
mod camera;
mod cube;
mod cylinder;
mod framebuffer;
mod light;
mod material;
mod ray;
mod scene;
mod sphere;
mod vector;

use camera::Camera;
use framebuffer::{Framebuffer, rgb};
use light::Light;
use minifb::{Key, Scale, Window, WindowOptions};
use ray::Ray;
use scene::{SceneObject, build_scene};
use vector::Vec3;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const ROTATION_SPEED: f32 = 0.05;
const SHADOW_BIAS: f32 = 0.001;
const REFLECTION_BIAS: f32 = 0.001;
const MAX_DEPTH: u32 = 2;

fn cast_shadow(point: Vec3, normal: Vec3, light: &Light, objects: &[SceneObject]) -> bool {
    let shadow_origin = point + normal * SHADOW_BIAS;

    let to_light = light.position - shadow_origin;
    let light_distance = to_light.length();
    let light_direction = to_light.normalize();

    let shadow_ray = Ray::new(shadow_origin, light_direction);

    for object in objects {
        if let Some(t) = object.intersect(&shadow_ray) {
            if t > 0.0 && t < light_distance {
                return true;
            }
        }
    }

    false
}

fn trace_ray(
    ray: &Ray,
    camera: &Camera,
    objects: &[SceneObject],
    light: &Light,
    depth: u32,
) -> u32 {
    let background = rgb(15, 18, 24);

    let mut closest_t = f32::INFINITY;
    let mut closest_object: Option<&SceneObject> = None;

    for object in objects {
        if let Some(t) = object.intersect(ray) {
            if t < closest_t {
                closest_t = t;
                closest_object = Some(object);
            }
        }
    }

    let Some(object) = closest_object else {
        return background;
    };

    let hit_point = ray.at(closest_t);
    let normal = object.normal_at(hit_point);

    let in_shadow = cast_shadow(hit_point, normal, light, objects);

    let diffuse_light = if in_shadow {
        0.0
    } else {
        light.illuminate(hit_point, normal)
    };

    let specular_light = if in_shadow {
        0.0
    } else {
        let light_direction = (light.position - hit_point).normalize();

        let view_direction = (camera.eye - hit_point).normalize();

        let reflected = (light_direction * -1.0).reflect(&normal);

        view_direction
            .dot(&reflected)
            .max(0.0)
            .powf(object.material.shininess)
            * object.material.specular
            * light.intensity
    };

    let brightness = (object.material.ambient + diffuse_light * object.material.diffuse).min(1.0);

    let color = object.material.color;

    let base_r = ((color >> 16) & 255) as f32;
    let base_g = ((color >> 8) & 255) as f32;
    let base_b = (color & 255) as f32;

    let local_r = (base_r * brightness + 255.0 * specular_light).min(255.0);

    let local_g = (base_g * brightness + 255.0 * specular_light).min(255.0);

    let local_b = (base_b * brightness + 255.0 * specular_light).min(255.0);

    let reflectivity = object.material.reflectivity;

    if depth >= MAX_DEPTH || reflectivity <= 0.0 {
        return rgb(local_r as u32, local_g as u32, local_b as u32);
    }

    let reflection_direction = ray.direction.reflect(&normal).normalize();

    let reflection_origin = hit_point + normal * REFLECTION_BIAS;

    let reflection_ray = Ray::new(reflection_origin, reflection_direction);

    let reflected_color = trace_ray(&reflection_ray, camera, objects, light, depth + 1);

    let reflected_r = ((reflected_color >> 16) & 255) as f32;

    let reflected_g = ((reflected_color >> 8) & 255) as f32;

    let reflected_b = (reflected_color & 255) as f32;

    let final_r = local_r * (1.0 - reflectivity) + reflected_r * reflectivity;

    let final_g = local_g * (1.0 - reflectivity) + reflected_g * reflectivity;

    let final_b = local_b * (1.0 - reflectivity) + reflected_b * reflectivity;

    rgb(
        final_r.min(255.0) as u32,
        final_g.min(255.0) as u32,
        final_b.min(255.0) as u32,
    )
}

fn render(framebuffer: &mut Framebuffer, camera: &Camera, objects: &[SceneObject], light: &Light) {
    framebuffer.clear(rgb(15, 18, 24));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let color = trace_ray(&ray, camera, objects, light, 0);

            framebuffer.set_pixel(x, y, color);
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let objects = build_scene();

    let light = Light::new(Vec3::new(-5.0, 4.5, 1.5), 1.25);

    let mut camera = Camera::new(
        Vec3::new(7.5, 4.5, 7.0),
        Vec3::new(0.0, 0.4, -3.5),
        Vec3::new(0.0, 1.0, 0.0),
        60.0_f32.to_radians(),
    );

    let mut window = Window::new(
        "TRACE//404: The Last Debug | Flechas: camara orbital | ESC: salir",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    let mut needs_render = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit_controls = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit_controls {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);

                needs_render = true;
            }
        }

        if needs_render {
            render(&mut framebuffer, &camera, &objects, &light);

            needs_render = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
